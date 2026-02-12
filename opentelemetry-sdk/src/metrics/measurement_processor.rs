//! MeasurementProcessor for observing measurements in the metrics pipeline.
//!
//! This module provides the [`MeasurementProcessor`] trait which allows observing
//! measurements as they are recorded, without modifying them. This is useful for
//! scenarios like exporting raw measurements to external systems (e.g., ETW).
//!
//! # Example
//!
//! ```
//! use opentelemetry_sdk::metrics::{
//!     MeasurementProcessor, MeasurementValue, SdkMeterProvider,
//! };
//! use opentelemetry_sdk::metrics::Instrument;
//! use opentelemetry::KeyValue;
//!
//! struct MyProcessor;
//!
//! impl MeasurementProcessor for MyProcessor {
//!     fn process(&self, instrument: &Instrument, value: MeasurementValue, attrs: &[KeyValue]) {
//!         println!("Measurement: {} = {:?}", instrument.name(), value);
//!     }
//! }
//!
//! let provider = SdkMeterProvider::builder()
//!     .with_measurement_processor(MyProcessor)
//!     .build();
//! ```

use opentelemetry::KeyValue;

use super::Instrument;

/// Represents a measurement value that can be u64, i64, or f64.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementValue {
    /// An unsigned 64-bit integer measurement.
    U64(u64),
    /// A signed 64-bit integer measurement.
    I64(i64),
    /// A 64-bit floating point measurement.
    F64(f64),
}

impl From<u64> for MeasurementValue {
    fn from(value: u64) -> Self {
        MeasurementValue::U64(value)
    }
}

impl From<i64> for MeasurementValue {
    fn from(value: i64) -> Self {
        MeasurementValue::I64(value)
    }
}

impl From<f64> for MeasurementValue {
    fn from(value: f64) -> Self {
        MeasurementValue::F64(value)
    }
}

/// A processor that observes measurements without modifying them.
///
/// `MeasurementProcessor` implementations are invoked synchronously in the hot path
/// when measurements are recorded. Implementations **must** be fast and non-blocking
/// to avoid impacting application performance.
///
/// Unlike the modification-capable processors discussed in the OpenTelemetry
/// specification, this trait provides read-only access to measurements. It cannot
/// modify values, attributes, or drop measurements.
///
/// # Thread Safety
///
/// Implementations must be thread-safe (`Send + Sync`) as they may be called
/// concurrently from multiple threads.
///
/// # Example
///
/// ```
/// use opentelemetry_sdk::metrics::{MeasurementProcessor, MeasurementValue, Instrument};
/// use opentelemetry::KeyValue;
///
/// struct LoggingProcessor;
///
/// impl MeasurementProcessor for LoggingProcessor {
///     fn process(&self, instrument: &Instrument, value: MeasurementValue, attrs: &[KeyValue]) {
///         // Log or export the measurement
///         println!("{}: {:?}", instrument.name(), value);
///     }
/// }
/// ```
pub trait MeasurementProcessor: Send + Sync + 'static {
    /// Called when a measurement is recorded.
    ///
    /// This method receives read-only access to the measurement data.
    /// It cannot modify or drop measurements.
    ///
    /// # Arguments
    ///
    /// * `instrument` - Metadata about the instrument (name, kind, unit, description)
    /// * `value` - The measurement value
    /// * `attrs` - The attributes associated with this measurement
    fn process(&self, instrument: &Instrument, value: MeasurementValue, attrs: &[KeyValue]);
}

impl<T: MeasurementProcessor + ?Sized> MeasurementProcessor for std::sync::Arc<T> {
    fn process(&self, instrument: &Instrument, value: MeasurementValue, attrs: &[KeyValue]) {
        (**self).process(instrument, value, attrs)
    }
}
