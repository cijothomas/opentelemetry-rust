use opentelemetry::{InstrumentationScope, KeyValue};
use std::{borrow::Cow, fmt::Debug, time::SystemTime};

use crate::{error::OTelSdkResult, metrics::Temporality};

/// Metric exporter
pub trait MetricExporter: Send + Sync + Debug + 'static {
    /// Export serializes and transmits metric data to a receiver.
    ///
    /// All retry logic must be contained in this function. The SDK does not
    /// implement any retry logic. All errors returned by this function are
    /// considered unrecoverable and will be logged.
    fn export(&self, metrics: &[Metric])
        -> impl std::future::Future<Output = OTelSdkResult> + Send;

    /// Flushes any metric data held by an exporter.
    fn force_flush(&self) -> OTelSdkResult;

    /// Releases any held computational resources.
    ///
    /// After Shutdown is called, calls to Export will perform no operation and
    /// instead will return an error indicating the shutdown state.
    fn shutdown(&self) -> OTelSdkResult;

    /// Access the [Temporality] of the MetricExporter.
    fn temporality(&self) -> Temporality;
}

/// A collection of Metrics. Each Metric will contain collection of DataPoints.
#[derive(Debug)]
pub struct Metric {
    /// The scope of the instrument that created this data.
    pub scope: InstrumentationScope,
    /// The name of the instrument that created this data.
    pub name: Cow<'static, str>,
    /// The description of the instrument, which can be used in documentation.
    pub description: Cow<'static, str>,
    /// The unit in which the instrument reports.
    pub unit: Cow<'static, str>,
    /// Aggregated data.
    pub data: AggregatedMetrics,
}

/// Aggregated Metrics.
#[derive(Debug)]
pub enum AggregatedMetrics {
    /// GaugeF64
    GaugeF64(Gauge<f64>),
    /// GaugeI64
    GaugeI64(Gauge<i64>),
    /// SumF64
    SumF64(Sum<f64>),
    /// SumI64
    SumI64(Sum<i64>),
    /// HistogramF64
    HistogramF64(Histogram<f64>),
    /// HistogramI64
    HistogramI64(Histogram<i64>),
}

/// Gauge metric
#[derive(Debug)]
pub struct Gauge<T> {
    /// Represents individual aggregated measurements with unique attributes.
    pub data_points: Vec<GaugeDataPoint<T>>,
    /// The time when the time series was started.
    pub start_time: Option<SystemTime>,
    /// The time when the time series was recorded.
    pub time: SystemTime,
}

/// DataPoint is a single data point in a time series.
#[derive(Debug, PartialEq)]
pub struct GaugeDataPoint<T> {
    /// Attributes is the set of key value pairs that uniquely identify the
    /// time series.
    pub attributes: Vec<KeyValue>,
    /// The value of this data point.
    pub value: T,
}

impl<T: Copy> Clone for GaugeDataPoint<T> {
    fn clone(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
            value: self.value,
        }
    }
}

/// Sum metric
#[derive(Debug)]
pub struct Sum<T> {
    /// Represents individual aggregated measurements with unique attributes.
    pub data_points: Vec<SumDataPoint<T>>,
    /// The time when the time series was started.
    pub start_time: SystemTime,
    /// The time when the time series was recorded.
    pub time: SystemTime,
    /// Describes if the aggregation is reported as the change from the last report
    /// time, or the cumulative changes since a fixed start time.
    pub temporality: Temporality,
    /// Whether this aggregation only increases or decreases.
    pub is_monotonic: bool,
}

/// DataPoint is a single data point in a time series.
#[derive(Debug, PartialEq)]
pub struct SumDataPoint<T> {
    /// Attributes is the set of key value pairs that uniquely identify the
    /// time series.
    pub attributes: Vec<KeyValue>,
    /// The value of this data point.
    pub value: T,
}

impl<T: Copy> Clone for SumDataPoint<T> {
    fn clone(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
            value: self.value,
        }
    }
}

/// Histogram metric
#[derive(Debug)]
pub struct Histogram<T> {
    /// Individual aggregated measurements with unique attributes.
    pub data_points: Vec<HistogramDataPoint<T>>,
    /// The time when the time series was started.
    pub start_time: SystemTime,
    /// The time when the time series was recorded.
    pub time: SystemTime,
    /// Describes if the aggregation is reported as the change from the last report
    /// time, or the cumulative changes since a fixed start time.
    pub temporality: Temporality,
}

/// A single histogram data point in a time series.
#[derive(Debug, PartialEq)]
pub struct HistogramDataPoint<T> {
    /// The set of key value pairs that uniquely identify the time series.
    pub attributes: Vec<KeyValue>,
    /// The number of updates this histogram has been calculated with.
    pub count: u64,
    /// The upper bounds of the buckets of the histogram.
    ///
    /// Because the last boundary is +infinity this one is implied.
    pub bounds: Vec<f64>,
    /// The count of each of the buckets.
    pub bucket_counts: Vec<u64>,

    /// The minimum value recorded.
    pub min: Option<T>,
    /// The maximum value recorded.
    pub max: Option<T>,
    /// The sum of the values recorded.
    pub sum: T,
}

impl<T: Copy> Clone for HistogramDataPoint<T> {
    fn clone(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
            count: self.count,
            bounds: self.bounds.clone(),
            bucket_counts: self.bucket_counts.clone(),
            min: self.min,
            max: self.max,
            sum: self.sum,
        }
    }
}
