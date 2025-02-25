use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use opentelemetry::{
    metrics::{Meter, MeterProvider},
    otel_debug, otel_info, InstrumentationScope,
};

use crate::{
    error::{OTelSdkError, OTelSdkResult},
    metrics_new::{noop::NoopMeter, reader::MetricReader},
    Resource,
};

use super::{export::Metric, meter::SdkMeter};

/// Handles the creation and coordination of Meters.
#[derive(Debug, Clone)]
pub struct SdkMeterProvider {
    inner: Arc<SdkMeterProviderInner>,
}

impl SdkMeterProvider {
    /// Creates a new `MeterProviderBuilder` instance.
    pub fn builder() -> MeterProviderBuilder {
        MeterProviderBuilder::new()
    }

    /// Forces a flush of all metrics.
    pub fn force_flush(&self) -> OTelSdkResult {
        self.inner.force_flush()
    }

    fn register_provider_on_reader(&self) {
        let weak_ref = Arc::downgrade(&self.inner);
        for reader in &self.inner.readers {
            reader.register_provider(weak_ref.clone());
        }
    }
}

impl MeterProvider for SdkMeterProvider {
    fn meter(&self, name: &'static str) -> Meter {
        self.meter_with_scope(InstrumentationScope::builder(name).build())
    }

    fn meter_with_scope(&self, scope: InstrumentationScope) -> Meter {
        self.inner.meter_with_scope(scope)
    }
}

#[derive(Debug)]
pub(crate) struct SdkMeterProviderInner {
    meters: Mutex<HashMap<InstrumentationScope, Arc<SdkMeter>>>,
    shutdown_invoked: AtomicBool,
    resource: Resource,
    readers: Vec<Box<dyn MetricReader>>,
}

impl SdkMeterProviderInner {
    fn collect(&self) -> Result<Vec<Metric>, OTelSdkError> {
        let meters = self.meters.lock().unwrap();
        let mut metrics = vec![];
        for meter in meters.values() {
            metrics.extend(meter.collect());
        }
        Ok(metrics)
    }

    fn force_flush(&self) -> OTelSdkResult {
        for reader in &self.readers {
            reader.force_flush()?;
        }
        Ok(())
    }

    fn meter_with_scope(&self, scope: InstrumentationScope) -> Meter {
        if self.shutdown_invoked.load(Ordering::Relaxed) {
            otel_debug!(
                name: "MeterProvider.NoOpMeterReturned",
                meter_name = scope.name(),
            );
            return Meter::new(Arc::new(NoopMeter::new()));
        }

        if scope.name().is_empty() {
            otel_info!(name: "MeterNameEmpty", message = "Meter name is empty; consider providing a meaningful name. Meter will function normally and the provided name will be used as-is.");
        };

        if let Ok(mut meters) = self.meters.lock() {
            if let Some(existing_meter) = meters.get(&scope) {
                otel_debug!(
                    name: "MeterProvider.ExistingMeterReturned",
                    meter_name = scope.name(),
                );
                Meter::new(existing_meter.clone())
            } else {
                let new_meter = Arc::new(SdkMeter::new(scope.clone()));
                meters.insert(scope.clone(), new_meter.clone());
                otel_debug!(
                    name: "MeterProvider.NewMeterCreated",
                    meter_name = scope.name(),
                );
                Meter::new(new_meter)
            }
        } else {
            otel_debug!(
                name: "MeterProvider.NoOpMeterReturned",
                meter_name = scope.name(),
            );
            Meter::new(Arc::new(NoopMeter::new()))
        }
    }
}

/// Builder for `SdkMeterProvider`.
#[derive(Debug)]
pub struct MeterProviderBuilder {
    resource: Option<Resource>,
    readers: Vec<Box<dyn MetricReader>>,
}

impl MeterProviderBuilder {
    pub(crate) fn new() -> Self {
        Self {
            resource: None,
            readers: vec![],
        }
    }

    /// Add Resource to the MeterProvider.
    /// If invoked more than once, they will be merged.
    pub fn with_resource(mut self, resource: Resource) -> Self {
        self.resource = match self.resource {
            Some(existing) => Some(existing.merge(&resource)),
            None => Some(resource),
        };

        self
    }

    /// Add a reader to the MeterProvider.
    /// Can be invoked multiple times to add more readers.
    pub fn with_reader<T: MetricReader>(mut self, reader: T) -> Self {
        self.readers.push(Box::new(reader));
        self
    }

    /// Build the `SdkMeterProvider`.
    pub fn build(self) -> SdkMeterProvider {
        let provider_inner = SdkMeterProviderInner {
            meters: Mutex::new(HashMap::new()),
            shutdown_invoked: AtomicBool::new(false),
            resource: self.resource.unwrap_or(Resource::builder().build()),
            readers: self.readers,
        };

        let provider = SdkMeterProvider {
            inner: Arc::new(provider_inner),
        };
        provider.register_provider_on_reader();
        provider
    }
}

/// MetricProducer trait for collecting metrics.
pub trait MetricProducer: Send + Sync {
    /// Collect metrics.
    fn collect(&self) -> Result<Vec<Metric>, OTelSdkError>;
}

impl MetricProducer for SdkMeterProviderInner {
    fn collect(&self) -> Result<Vec<Metric>, OTelSdkError> {
        self.collect()
    }
}
