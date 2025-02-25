use std::{sync::Mutex, sync::Weak};

use crate::error::{OTelSdkError, OTelSdkResult};

use super::{
    export::{Metric, MetricExporter},
    meter_provider::MetricProducer,
    reader::MetricReader,
};

/// Manual reader
pub struct ManualReader<T>
where
    T: MetricExporter,
{
    exporter: T,
    // a closure that accepts nothing and returns Vec<Metric>
    // collect_fn: std::sync::Mutex<Option<Box<dyn Fn() -> Vec<Metric> + Send + Sync>>>,
    provider: Mutex<Option<Weak<dyn MetricProducer>>>,
}

impl<T> std::fmt::Debug for ManualReader<T>
where
    T: MetricExporter,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManualReader")
            .field("exporter", &self.exporter)
            .finish()
    }
}

impl<T> ManualReader<T>
where
    T: MetricExporter,
{
    /// Creates a new `ManualReader` instance.
    pub fn new(exporter: T) -> Self {
        Self {
            exporter,
            provider: Mutex::new(None),
        }
    }
}

impl<T> MetricReader for ManualReader<T>
where
    T: MetricExporter,
{
    fn collect(&self) -> Result<Vec<Metric>, OTelSdkError> {
        let provider = self.provider.lock().unwrap();
        let provider = provider
            .as_ref()
            .ok_or(OTelSdkError::InternalFailure(
                "No provider registered".to_string(),
            ))?
            .upgrade()
            .ok_or(OTelSdkError::InternalFailure(
                "Provider has been dropped".to_string(),
            ))?;
        provider.collect()
    }
    fn shutdown(&self) -> OTelSdkResult {
        self.exporter.shutdown()
    }
    fn force_flush(&self) -> OTelSdkResult {
        let metrics = self.collect()?;
        futures_executor::block_on(self.exporter.export(&metrics))?;
        self.exporter.force_flush()
    }

    fn temporality(&self, _kind: crate::metrics::InstrumentKind) -> crate::metrics::Temporality {
        self.exporter.temporality()
    }

    fn register_provider(&self, provider: Weak<dyn MetricProducer>) {
        let mut self_provider = self.provider.lock().unwrap();
        *self_provider = Some(provider);
    }
}
