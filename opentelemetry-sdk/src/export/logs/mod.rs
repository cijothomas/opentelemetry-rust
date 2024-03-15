//! Log exporters
use async_trait::async_trait;
#[cfg(feature = "logs_level_enabled")]
use opentelemetry::logs::Severity;
use opentelemetry::{
    logs::{LogError, LogRecord, LogResult},
    InstrumentationLibrary,
};
use std::fmt::Debug;

/// `LogExporter` defines the interface that log exporters should implement.
#[async_trait]
pub trait LogExporter: Send + Sync + Debug {
    /// Exports a batch of [`LogData`].
    async fn export(&mut self, batch: Vec<LogData>) -> LogResult<()>;
    /// Shuts down the exporter.
    fn shutdown(&mut self) {}
    /// Set the resource associated with the provider. If the exporter intends
    /// to use the resource, it should store it during this call. Resource will
    /// not be passed as part of LogData. Resource is immutable and can be
    /// stored as is.
    fn set_resource(&mut self, _resource: crate::Resource) {}
    #[cfg(feature = "logs_level_enabled")]
    /// Check if logs are enabled.
    fn event_enabled(&self, _level: Severity, _target: &str, _name: &str) -> bool {
        true
    }
}

/// `LogData` associates a [`LogRecord`] with a [`Resource`] and
/// [`InstrumentationLibrary`].
#[derive(Clone, Debug)]
pub struct LogData {
    /// Log record
    pub record: LogRecord,
    /// Instrumentation details for the emitter who produced this `LogData`.
    pub instrumentation: InstrumentationLibrary,
}

/// Describes the result of an export.
pub type ExportResult = Result<(), LogError>;
