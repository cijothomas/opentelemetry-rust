use std::{
    env, fmt,
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, Weak,
    },
    thread,
    time::{Duration, Instant},
};

use opentelemetry::metrics::{MetricsError, Result};
use opentelemetry::{otel_debug, otel_error, otel_info, otel_warn};

use crate::{
    metrics::{exporter::PushMetricsExporter, reader::SdkProducer},
    Resource,
};

use super::{
    data::{ResourceMetrics, Temporality},
    instrument::InstrumentKind,
    reader::{MetricReader, TemporalitySelector},
    Pipeline,
};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_INTERVAL: Duration = Duration::from_secs(60);

const METRIC_EXPORT_INTERVAL_NAME: &str = "OTEL_METRIC_EXPORT_INTERVAL";
const METRIC_EXPORT_TIMEOUT_NAME: &str = "OTEL_METRIC_EXPORT_TIMEOUT";

/// Configuration options for [PeriodicReader].
///
/// A periodic reader is a [MetricReader] that collects and exports metric data
/// to the exporter at a defined interval.
///
/// By default, the returned [MetricReader] will collect and export data every
/// 60 seconds. The export time is not counted towards the interval between
/// attempts. PeriodicReader itself does not enforce timeouts. Instead timeout
/// is passed on to the exporter for each export attempt.
///
/// The [collect] method of the returned [MetricReader] continues to gather and
/// return metric data to the user. It will not automatically send that data to
/// the exporter outside of the predefined interval.
///
/// [collect]: MetricReader::collect
#[derive(Debug)]
pub struct PeriodicReaderBuilder<E> {
    interval: Duration,
    timeout: Duration,
    exporter: E,
}

impl<E> PeriodicReaderBuilder<E>
where
    E: PushMetricsExporter,
{
    fn new(exporter: E) -> Self {
        let interval = env::var(METRIC_EXPORT_INTERVAL_NAME)
            .ok()
            .and_then(|v| v.parse().map(Duration::from_millis).ok())
            .unwrap_or(DEFAULT_INTERVAL);
        let timeout = env::var(METRIC_EXPORT_TIMEOUT_NAME)
            .ok()
            .and_then(|v| v.parse().map(Duration::from_millis).ok())
            .unwrap_or(DEFAULT_TIMEOUT);

        PeriodicReaderBuilder {
            interval,
            timeout,
            exporter,
        }
    }

    /// Configures the intervening time between exports for a [PeriodicReader].
    ///
    /// This option overrides any value set for the `OTEL_METRIC_EXPORT_INTERVAL`
    /// environment variable.
    ///
    /// If this option is not used or `interval` is equal to zero, 60 seconds is
    /// used as the default.
    pub fn with_interval(mut self, interval: Duration) -> Self {
        if !interval.is_zero() {
            self.interval = interval;
        }
        self
    }

    /// Configures the timeout for an export to complete. PeriodicReader itself
    /// does not enforce timeouts. Instead timeout is passed on to the exporter
    /// for each export attempt.
    ///
    /// This option overrides any value set for the `OTEL_METRIC_EXPORT_TIMEOUT`
    /// environment variable.
    ///
    /// If this option is not used or `timeout` is equal to zero, 30 seconds is used
    /// as the default.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        if !timeout.is_zero() {
            self.timeout = timeout;
        }
        self
    }

    /// Create a [PeriodicReader] with the given config.
    pub fn build(self) -> PeriodicReader {
        PeriodicReader::new(self.exporter, self.interval, self.timeout)
    }
}

/// A [MetricReader] that continuously collects and exports metric data at a set
/// interval.
///
/// By default, PeriodicReader will collect and export data every
/// 60 seconds. The export time is not counted towards the interval between
/// attempts. PeriodicReader itself does not enforce timeouts. Instead timeout
/// is passed on to the exporter for each export attempt.
///
/// The [collect] method of the returned continues to gather and
/// return metric data to the user. It will not automatically send that data to
/// the exporter outside of the predefined interval.
///
/// The exporter can be any exporter that implements [PushMetricsExporter] such
/// as [opentelemetry-otlp].
///
/// [collect]: MetricReader::collect
/// [opentelemetry-otlp]: https://docs.rs/opentelemetry-otlp/latest/opentelemetry_otlp/
///
/// # Example
///
/// ```no_run
/// use opentelemetry_sdk::metrics::PeriodicReader;
/// # fn example<E>(get_exporter: impl Fn() -> E)
/// # where
/// #     E: opentelemetry_sdk::metrics::exporter::PushMetricsExporter,
/// # {
///
/// let exporter = get_exporter(); // set up a push exporter like OTLP
///
/// let reader = PeriodicReader::builder(exporter).build();
/// # drop(reader);
/// # }
/// ```
#[derive(Clone)]
pub struct PeriodicReader {
    inner: Arc<PeriodicReaderInner>,
}

impl PeriodicReader {
    /// Configuration options for a periodic reader
    pub fn builder<E>(exporter: E) -> PeriodicReaderBuilder<E>
    where
        E: PushMetricsExporter,
    {
        PeriodicReaderBuilder::new(exporter)
    }

    fn new<E>(exporter: E, interval: Duration, timeout: Duration) -> Self
    where
        E: PushMetricsExporter,
    {
        let (message_sender, message_receiver): (Sender<Message>, Receiver<Message>) =
            mpsc::channel();
        let reader = PeriodicReader {
            inner: Arc::new(PeriodicReaderInner {
                message_sender,
                is_shutdown: AtomicBool::new(false),
                producer: Mutex::new(None),
                exporter: Arc::new(exporter),
            }),
        };
        let cloned_reader = reader.clone();

        let result_thread_creation = thread::Builder::new()
            .name("OpenTelemetry.Metrics.PeriodicReader".to_string())
            .spawn(move || {
                let mut interval_start = Instant::now();
                let mut remaining_interval = interval;
                otel_info!(
                    name: "PeriodReaderThreadStarted",
                    interval = interval.as_secs(),
                    timeout = timeout.as_secs()
                );
                loop {
                    otel_debug!(
                        name: "PeriodReaderThreadLoopAlive", message = "Next export will happen after interval, unless flush or shutdown is triggered.", interval = remaining_interval.as_secs()
                    );
                    match message_receiver.recv_timeout(remaining_interval) {
                        Ok(Message::Flush(response_sender)) => {
                            otel_debug!(
                                name: "PeriodReaderThreadExportingDueToFlush"
                            );
                            if let Err(_e) = cloned_reader.collect_and_export(timeout) {
                                response_sender.send(false).unwrap();
                            } else {
                                response_sender.send(true).unwrap();
                            }

                            // Adjust the remaining interval after the flush
                            let elapsed = interval_start.elapsed();
                            if elapsed < interval {
                                remaining_interval = interval - elapsed;
                                otel_debug!(
                                    name: "PeriodReaderThreadAdjustingRemainingIntervalAfterFlush",
                                    event_name = "PeriodReaderThreadAdjustingRemainingIntervalAfterFlush",
                                    remaining_interval = remaining_interval.as_secs()
                                );
                            } else {
                                otel_debug!(
                                    name: "PeriodReaderThreadAdjustingExportAfterFlush",
                                    event_name = "PeriodReaderThreadAdjustingExportAfterFlush",
                                );
                                // Reset the interval if the flush finishes after the expected export time
                                // effectively missing the normal export.
                                // Should we attempt to do the missed export immediately?
                                // Or do the next export at the next interval?
                                // Currently this attempts the next export immediately.
                                // i.e calling Flush can affect the regularity.
                                interval_start = Instant::now();
                                remaining_interval = Duration::ZERO;
                            }
                        }
                        Ok(Message::Shutdown(response_sender)) => {
                            // Perform final export and break out of loop and exit the thread
                            otel_debug!(
                                name: "PeriodReaderThreadExportingDueToShutdown", event_name =  "PeriodReaderThreadExportingDueToShutdown"
                            );
                            if let Err(_e) = cloned_reader.collect_and_export(timeout) {
                                response_sender.send(false).unwrap();
                            } else {
                                response_sender.send(true).unwrap();
                            }
                            break;
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            let export_start = Instant::now();
                            otel_debug!(
                                name: "PeriodReaderThreadExportingDueToTimer",
                                event_name = "PeriodReaderThreadExportingDueToTimer"
                            );
                            cloned_reader.collect_and_export(timeout).unwrap();

                            let time_taken_for_export = export_start.elapsed();
                            if time_taken_for_export > interval {
                                otel_debug!(
                                    name: "PeriodReaderThreadExportTookLongerThanInterval",
                                    event_name = "PeriodReaderThreadExportTookLongerThanInterval"
                                );
                                // if export took longer than interval, do the
                                // next export immediately.
                                // Alternatively, we could skip the next export
                                // and wait for the next interval.
                                // Or enforce that export timeout is less than interval.
                                // What is the desired behavior?
                                interval_start = Instant::now();
                                remaining_interval = Duration::ZERO;
                            } else {
                                remaining_interval = interval - time_taken_for_export;
                                interval_start = Instant::now();
                            }
                        }
                        Err(_) => {
                            // Some other error. Break out and exit the thread.
                            break;
                        }
                    }
                }
                otel_info!(
                    name: "PeriodReaderThreadStopped",
                    event_name = "PeriodReaderThreadStopped"
                );
            });

        // TODO: Should we fail-fast here and bubble up the error to user?
        #[allow(unused_variables)]
        if let Err(e) = result_thread_creation {
            otel_error!(
                name: "PeriodReaderThreadStartError",
                event_name =  "PeriodReaderThreadStartError",
                error = format!("{:?}", e)
            );
        }
        reader
    }

    fn collect_and_export(&self, timeout: Duration) -> Result<()> {
        self.inner.collect_and_export(timeout)
    }
}

impl fmt::Debug for PeriodicReader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeriodicReader").finish()
    }
}

struct PeriodicReaderInner {
    exporter: Arc<dyn PushMetricsExporter>,
    message_sender: mpsc::Sender<Message>,
    producer: Mutex<Option<Weak<dyn SdkProducer>>>,
    is_shutdown: AtomicBool,
}

impl PeriodicReaderInner {
    fn register_pipeline(&self, producer: Weak<dyn SdkProducer>) {
        let mut inner = self.producer.lock().expect("lock poisoned");
        *inner = Some(producer);
    }

    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.exporter.temporality(kind)
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> Result<()> {
        if self.is_shutdown.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(MetricsError::Other("reader is shut down".into()));
        }

        let producer = self.producer.lock().expect("lock poisoned");
        if let Some(p) = producer.as_ref() {
            p.upgrade()
                .ok_or_else(|| MetricsError::Other("pipeline is dropped".into()))?
                .produce(rm)?;
            Ok(())
        } else {
            Err(MetricsError::Other("pipeline is not registered".into()))
        }
    }

    fn collect_and_export(&self, timeout: Duration) -> Result<()> {
        // TODO: Reuse the internal vectors. Or refactor to avoid needing any
        // owned data structures to be passed to exporters.
        let mut rm = ResourceMetrics {
            resource: Resource::empty(),
            scope_metrics: Vec::new(),
        };

        let collect_result = self.collect(&mut rm);
        #[allow(clippy::question_mark)]
        if let Err(e) = collect_result {
            otel_warn!(
                name: "PeriodReaderCollectError",
                event_name = "PeriodReaderCollectError",
                error = format!("{:?}", e)
            );
            return Err(e);
        }

        if rm.scope_metrics.is_empty() {
            otel_debug!(name: "NoMetricsCollected");
            return Ok(());
        }

        // TODO: substract the time taken for collect from the timeout. collect
        // involves observable callbacks too, which are user defined and can
        // take arbitrary time.
        //
        // Relying on futures executor to execute asyc call. No timeout is
        // enforced here. The exporter is responsible for enforcing the timeout.
        futures_executor::block_on(self.exporter.export(&mut rm, timeout))?;
        Ok(())
    }

    fn force_flush(&self) -> Result<()> {
        if self.is_shutdown.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(MetricsError::Other("reader is shut down".into()));
        }
        let (response_tx, response_rx) = mpsc::channel();
        self.message_sender
            .send(Message::Flush(response_tx))
            .map_err(|e| MetricsError::Other(e.to_string()))?;

        if let Ok(response) = response_rx.recv() {
            if response {
                Ok(())
            } else {
                Err(MetricsError::Other("Failed to flush".into()))
            }
        } else {
            Err(MetricsError::Other("Failed to flush".into()))
        }
    }

    fn shutdown(&self) -> Result<()> {
        if self.is_shutdown.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(MetricsError::Other("reader is already shut down".into()));
        }

        let (response_tx, response_rx) = mpsc::channel();
        self.message_sender
            .send(Message::Shutdown(response_tx))
            .map_err(|e| MetricsError::Other(e.to_string()))?;

        if let Ok(response) = response_rx.recv() {
            self.is_shutdown
                .store(true, std::sync::atomic::Ordering::Relaxed);
            if response {
                Ok(())
            } else {
                Err(MetricsError::Other("Failed to shutdown".into()))
            }
        } else {
            self.is_shutdown
                .store(true, std::sync::atomic::Ordering::Relaxed);
            Err(MetricsError::Other("Failed to shutdown".into()))
        }
    }
}

#[derive(Debug)]
enum Message {
    Flush(Sender<bool>),
    Shutdown(Sender<bool>),
}

impl TemporalitySelector for PeriodicReader {
    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.inner.temporality(kind)
    }
}

impl MetricReader for PeriodicReader {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.inner.register_pipeline(pipeline);
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> Result<()> {
        self.inner.collect(rm)
    }

    fn force_flush(&self) -> Result<()> {
        self.inner.force_flush()
    }

    // TODO: Offer an async version of shutdown so users can await the shutdown
    // completion, and avoid blocking the thread. The default shutdown on drop
    // can still use blocking call. If user already explicitly called shutdown,
    // drop won't call shutdown again.
    fn shutdown(&self) -> Result<()> {
        self.inner.shutdown()
    }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use super::PeriodicReader;
    use crate::{
        metrics::{data::ResourceMetrics, reader::MetricReader, SdkMeterProvider},
        testing::metrics::InMemoryMetricsExporter,
        Resource,
    };
    use opentelemetry::metrics::MeterProvider;
    use std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            mpsc, Arc,
        },
        time::Duration,
    };

    // use below command to run all tests
    // cargo test metrics::periodic_reader::tests --features=testing -- --nocapture
    #[test]
    fn collection_triggered_by_interval_multiple() {
        // Arrange
        let interval = std::time::Duration::from_millis(1);
        let exporter = InMemoryMetricsExporter::default();
        let reader = PeriodicReader::builder(exporter.clone())
            .with_interval(interval)
            .build();
        let i = Arc::new(AtomicUsize::new(0));
        let i_clone = i.clone();

        // Act
        let meter_provider = SdkMeterProvider::builder().with_reader(reader).build();
        let meter = meter_provider.meter("test");
        let _counter = meter
            .u64_observable_counter("testcounter")
            .with_callback(move |_| {
                i_clone.fetch_add(1, Ordering::Relaxed);
            })
            .init();

        // Sleep for a duration 5X (plus liberal buffer to account for potential
        // CI slowness) the interval to ensure multiple collection.
        // Not a fan of such tests, but this seems to be the only way to test
        // if periodic reader is doing its job.
        // TODO: Decide if this should be ignored in CI
        std::thread::sleep(interval * 5 * 20);

        // Assert
        assert!(i.load(Ordering::Relaxed) >= 5);
    }

    #[test]
    fn shutdown_repeat() {
        // Arrange
        let interval = std::time::Duration::from_millis(1);
        let exporter = InMemoryMetricsExporter::default();
        let reader = PeriodicReader::builder(exporter.clone())
            .with_interval(interval)
            .build();

        let meter_provider = SdkMeterProvider::builder().with_reader(reader).build();
        let result = meter_provider.shutdown();
        assert!(result.is_ok());

        // calling shutdown again should return Err
        let result = meter_provider.shutdown();
        assert!(result.is_err());

        // calling shutdown again should return Err
        let result = meter_provider.shutdown();
        assert!(result.is_err());
    }

    #[test]
    fn flush_after_shutdown() {
        // Arrange
        let interval = std::time::Duration::from_millis(1);
        let exporter = InMemoryMetricsExporter::default();
        let reader = PeriodicReader::builder(exporter.clone())
            .with_interval(interval)
            .build();

        let meter_provider = SdkMeterProvider::builder().with_reader(reader).build();
        let result = meter_provider.force_flush();
        assert!(result.is_ok());

        let result = meter_provider.shutdown();
        assert!(result.is_ok());

        // calling force_flush after shutdown should return Err
        let result = meter_provider.force_flush();
        assert!(result.is_err());
    }

    #[test]
    fn flush_repeat() {
        // Arrange
        let interval = std::time::Duration::from_millis(1);
        let exporter = InMemoryMetricsExporter::default();
        let reader = PeriodicReader::builder(exporter.clone())
            .with_interval(interval)
            .build();

        let meter_provider = SdkMeterProvider::builder().with_reader(reader).build();
        let result = meter_provider.force_flush();
        assert!(result.is_ok());

        // calling force_flush again should return Ok
        let result = meter_provider.force_flush();
        assert!(result.is_ok());
    }

    #[test]
    fn periodic_reader_without_pipeline() {
        // Arrange
        let interval = std::time::Duration::from_millis(1);
        let exporter = InMemoryMetricsExporter::default();
        let reader = PeriodicReader::builder(exporter.clone())
            .with_interval(interval)
            .build();

        let rm = &mut ResourceMetrics {
            resource: Resource::empty(),
            scope_metrics: Vec::new(),
        };
        // Pipeline is not registered, so collect should return an error
        let result = reader.collect(rm);
        assert!(result.is_err());

        // Pipeline is not registered, so flush should return an error
        let result = reader.force_flush();
        assert!(result.is_err());

        // Adding reader to meter provider should register the pipeline
        // TODO: This part might benefit from a different design.
        let meter_provider = SdkMeterProvider::builder()
            .with_reader(reader.clone())
            .build();

        // Now collect and flush should succeed
        let result = reader.collect(rm);
        assert!(result.is_ok());

        let result = meter_provider.force_flush();
        assert!(result.is_ok());
    }

    #[test]
    fn collection() {
        collection_triggered_by_interval_helper();
        collection_triggered_by_flush_helper();
        collection_triggered_by_shutdown_helper();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn collection_from_tokio_multi_with_one_worker() {
        collection_triggered_by_interval_helper();
        collection_triggered_by_flush_helper();
        collection_triggered_by_shutdown_helper();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn collection_from_tokio_with_two_worker() {
        collection_triggered_by_interval_helper();
        collection_triggered_by_flush_helper();
        collection_triggered_by_shutdown_helper();
    }

    #[tokio::test(flavor = "current_thread")]
    async fn collection_from_tokio_current() {
        collection_triggered_by_interval_helper();
        collection_triggered_by_flush_helper();
        collection_triggered_by_shutdown_helper();
    }

    fn collection_triggered_by_interval_helper() {
        collection_helper(|_| {
            // Sleep for a duration longer than the interval to ensure at least one collection
            // Not a fan of such tests, but this seems to be the only way to test
            // if periodic reader is doing its job.
            // TODO: Decide if this should be ignored in CI
            std::thread::sleep(Duration::from_millis(5000));
        });
    }

    fn collection_triggered_by_flush_helper() {
        collection_helper(|meter_provider| {
            meter_provider.force_flush().expect("flush should succeed");
        });
    }

    fn collection_triggered_by_shutdown_helper() {
        collection_helper(|meter_provider| {
            meter_provider.shutdown().expect("shutdown should succeed");
        });
    }

    fn collection_helper(trigger: fn(&SdkMeterProvider)) {
        // Arrange
        let interval = std::time::Duration::from_millis(1000);
        let exporter = InMemoryMetricsExporter::default();
        let reader = PeriodicReader::builder(exporter.clone())
            .with_interval(interval)
            .build();
        let (sender, receiver) = mpsc::channel();

        let meter_provider = SdkMeterProvider::builder().with_reader(reader).build();
        let meter = meter_provider.meter("test");
        let _counter = meter
            .u64_observable_counter("testcounter")
            .with_callback(move |_| {
                sender.send(()).expect("channel should still be open");
            })
            .init();

        // Act
        trigger(&meter_provider);

        // Assert
        receiver
            .recv_timeout(Duration::ZERO)
            .expect("message should be available in channel, indicating a collection occurred");
    }
}
