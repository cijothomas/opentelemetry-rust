use std::{fmt, sync::Weak};

use crate::{
    error::{OTelSdkError, OTelSdkResult},
    metrics::{InstrumentKind, Temporality},
};

use super::{export::Metric, meter_provider::MetricProducer};

/// MetricReader. Used to collect metrics from the SDK.
pub trait MetricReader: fmt::Debug + Send + Sync + 'static {
    /// Registers a [MetricReader] with a [Pipeline].
    ///
    /// The pipeline argument allows the `MetricReader` to signal the sdk to collect
    /// and send aggregated metric measurements.
    fn register_provider(&self, provider: Weak<dyn MetricProducer>);

    /// Gathers and returns all metric data related to the [MetricReader] from the
    /// SDK and stores it in the provided [ResourceMetrics] reference.
    ///
    /// An error is returned if this is called after shutdown.
    fn collect(&self) -> Result<Vec<Metric>, OTelSdkError>;

    /// Flushes all metric measurements held in an export pipeline.
    ///
    /// There is no guaranteed that all telemetry be flushed or all resources have
    /// been released on error.
    fn force_flush(&self) -> OTelSdkResult;

    /// Flushes all metric measurements held in an export pipeline and releases any
    /// held computational resources.
    ///
    /// There is no guaranteed that all telemetry be flushed or all resources have
    /// been released on error.
    ///
    /// After `shutdown` is called, calls to `collect` will perform no operation and
    /// instead will return an error indicating the shutdown state.
    fn shutdown(&self) -> OTelSdkResult;

    /// The output temporality, a function of instrument kind.
    /// This SHOULD be obtained from the exporter.
    ///
    /// If not configured, the Cumulative temporality SHOULD be used.
    fn temporality(&self, kind: InstrumentKind) -> Temporality;
}
