//! Read-only measurement processor for metrics.
//!
//! This module provides a mechanism to observe raw measurements as they flow
//! through the metrics SDK, without the ability to modify or drop them.
//!
//! # Overview
//!
//! The [`MeasurementProcessor`] trait allows you to implement custom observers
//! that receive every measurement recorded by instruments. This is useful for:
//!
//! - Exporting raw measurements to external systems (e.g., ETW, custom exporters)
//! - Debugging and logging measurement flow
//! - Real-time monitoring and alerting on individual measurements
//!
//! # Important Notes
//!
//! - Processors receive **read-only** access to measurements
//! - Processors **cannot** modify or drop measurements
//! - Implementations should be fast and non-blocking to avoid impacting throughput
//! - This feature is experimental and behind the `experimental_metrics_measurement_processor` feature flag
//!
//! # Example
//!
//! ```ignore
//! use opentelemetry_sdk::metrics::{MeasurementProcessor, MeasurementValue, Instrument};
//! use opentelemetry::KeyValue;
//! use std::fmt::Debug;
//!
//! #[derive(Debug)]
//! struct LoggingProcessor;
//!
//! impl MeasurementProcessor for LoggingProcessor {
//!     fn on_measurement(
//!         &self,
//!         instrument: &Instrument,
//!         value: MeasurementValue,
//!         attributes: &[KeyValue],
//!     ) {
//!         println!(
//!             "Measurement: instrument={}, value={:?}, attrs={:?}",
//!             instrument.name(),
//!             value,
//!             attributes
//!         );
//!     }
//! }
//! ```

use crate::error::OTelSdkResult;
use opentelemetry::KeyValue;
use std::fmt::Debug;

use super::Instrument;

/// Sealed trait for converting numeric types to MeasurementValue.
/// This is used internally by the SDK.
pub(crate) trait IntoMeasurementValue: Copy + Send + Sync + 'static {
    fn into_measurement_value(self) -> MeasurementValue;
}

impl IntoMeasurementValue for u64 {
    fn into_measurement_value(self) -> MeasurementValue {
        MeasurementValue::U64(self)
    }
}

impl IntoMeasurementValue for i64 {
    fn into_measurement_value(self) -> MeasurementValue {
        MeasurementValue::I64(self)
    }
}

impl IntoMeasurementValue for f64 {
    fn into_measurement_value(self) -> MeasurementValue {
        MeasurementValue::F64(self)
    }
}

/// A value recorded by a measurement.
///
/// This enum represents the different numeric types that can be recorded
/// by OpenTelemetry instruments.
#[derive(Debug, Clone, Copy)]
pub enum MeasurementValue {
    /// An unsigned 64-bit integer value (used by u64 Counter, Histogram, etc.)
    U64(u64),
    /// A signed 64-bit integer value (used by i64 UpDownCounter, etc.)
    I64(i64),
    /// A 64-bit floating point value (used by f64 Histogram, Gauge, etc.)
    F64(f64),
}

/// A read-only processor that observes measurements as they flow through the SDK.
///
/// Unlike [`SpanProcessor`] or [`LogProcessor`], this trait provides **read-only**
/// access to measurements. Implementations cannot modify or drop measurements;
/// they can only observe them.
///
/// # Performance Considerations
///
/// The `on_measurement` method is called synchronously on the hot path for every
/// measurement. Implementations should:
///
/// - Avoid blocking operations (I/O, locks, etc.)
/// - Keep processing time minimal
/// - Consider buffering/batching if export is needed
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` as they may be called from multiple threads
/// concurrently.
///
/// [`SpanProcessor`]: crate::trace::SpanProcessor
/// [`LogProcessor`]: crate::logs::LogProcessor
pub trait MeasurementProcessor: Send + Sync + Debug {
    /// Called when a measurement is recorded.
    ///
    /// This method is invoked for every measurement recorded by any instrument
    /// in the [`MeterProvider`] that this processor is registered with.
    ///
    /// # Arguments
    ///
    /// * `instrument` - Information about the instrument recording the measurement
    /// * `value` - The recorded value
    /// * `attributes` - The attributes associated with this measurement
    ///
    /// # Note
    ///
    /// This method is called on the hot path. Keep implementations fast and
    /// non-blocking.
    ///
    /// [`MeterProvider`]: crate::metrics::SdkMeterProvider
    fn on_measurement(&self, instrument: &Instrument, value: MeasurementValue, attributes: &[KeyValue]);

    /// Flushes any buffered data.
    ///
    /// This is called when [`MeterProvider::force_flush`] is invoked.
    /// Implementations that buffer measurements should export them here.
    ///
    /// The default implementation does nothing.
    ///
    /// [`MeterProvider::force_flush`]: crate::metrics::SdkMeterProvider::force_flush
    fn force_flush(&self) -> OTelSdkResult {
        Ok(())
    }

    /// Shuts down the processor.
    ///
    /// This is called when the [`MeterProvider`] is shut down.
    /// Implementations should release any resources here.
    ///
    /// The default implementation does nothing.
    ///
    /// [`MeterProvider`]: crate::metrics::SdkMeterProvider
    fn shutdown(&self) -> OTelSdkResult {
        Ok(())
    }
}

impl<T: MeasurementProcessor + ?Sized> MeasurementProcessor for std::sync::Arc<T> {
    fn on_measurement(
        &self,
        instrument: &Instrument,
        value: MeasurementValue,
        attributes: &[KeyValue],
    ) {
        (**self).on_measurement(instrument, value, attributes)
    }

    fn force_flush(&self) -> OTelSdkResult {
        (**self).force_flush()
    }

    fn shutdown(&self) -> OTelSdkResult {
        (**self).shutdown()
    }
}
