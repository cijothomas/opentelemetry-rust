use std::sync::Arc;

use opentelemetry::{metrics::{Counter, InstrumentBuilder, InstrumentProvider}, InstrumentationScope, Key, KeyValue};

use crate::{metrics::Temporality, metrics_new::export::AggregatedMetrics};

use super::{export::{Metric, Sum, SumDataPoint}, noop};

/// Implements Meter API
#[derive(Debug)]
pub(crate) struct SdkMeter {
    scope: InstrumentationScope,
    // must store all Instruments it ever handed out.
    // The struct should be Arc cloned, so one
    // clone is kept here and another is handed over
    // to the Instrument.
    // When collect() triggers, the Instrument
    // will be snapshotted and the data will be
    // returned.
}

impl SdkMeter {
    /// Create a new meter core.
    pub(crate) fn new(scope: InstrumentationScope) -> Self {
        SdkMeter { scope }
    }

    pub(crate) fn collect(&self) -> Vec<Metric> {
        let sum_datapoint_1 = SumDataPoint::<i64> {
            attributes: vec![
                KeyValue::new(Key::new("key1"), "value1"),
                KeyValue::new(Key::new("key2"), "value2"),
            ],
            value: 1,
        };
        let sum_datapoint_2 = SumDataPoint::<i64> {
            attributes: vec![
                KeyValue::new(Key::new("key3"), "value3"),
                KeyValue::new(Key::new("key4"), "value4"),
            ],
            value: 12,
        };
        let sum_aggregation = Sum::<i64> {
            start_time: std::time::SystemTime::now(),
            time: std::time::SystemTime::now(),
            is_monotonic: true,
            temporality: Temporality::Cumulative,
            data_points: vec![sum_datapoint_1, sum_datapoint_2],
        };
        let m1 = Metric {
            scope: self.scope.clone(),
            name: "metric1".into(),
            description: "metric1 desc".into(),
            unit: "ms".into(),
            data: AggregatedMetrics::SumI64(sum_aggregation),
        };
        vec![m1]
    }
}

#[doc(hidden)]
impl InstrumentProvider for SdkMeter {
    /// creates an instrument for recording increasing values.
    fn u64_counter(&self, _builder: InstrumentBuilder<'_, Counter<u64>>) -> Counter<u64> {
        Counter::new(Arc::new(noop::NoopSyncInstrument::new()))
    }       

    // /// creates an instrument for recording increasing values.
    // fn f64_counter(&self, _builder: InstrumentBuilder<'_, Counter<f64>>) -> Counter<f64> {
    //     Counter::new(Arc::new(noop::NoopSyncInstrument::new()))
    // }
}
