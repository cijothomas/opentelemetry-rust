use tracing::span::Attributes;

use crate::metrics_new::export::AggregatedMetrics;

use super::export::Metric;

/// StdOutExporter
#[derive(Debug)]
pub struct StdOutExporter;
impl StdOutExporter {
    /// Creates a new `StdOutExporter` instance.
    pub fn new() -> Self {
        Self
    }
}

impl super::export::MetricExporter for StdOutExporter {
    async fn export(&self, metrics: &[Metric]) -> crate::error::OTelSdkResult {
        for metric in metrics {
            println!("Metric Name {}", metric.name);
            println!("Metric Description {}", metric.description);
            println!("Metric Unit {}", metric.unit);
            println!("Metric Scope - Name {}", metric.scope.name());
            match &metric.data {
                AggregatedMetrics::GaugeF64(gauge) => {
                    println!("Metric Data - GaugeF64");
                    for data_point in &gauge.data_points {
                        println!("Attributes = {:?}", data_point.attributes);
                        println!("Value = {}", data_point.value);
                    }
                }
                AggregatedMetrics::GaugeI64(gauge) => {
                    println!("Metric Data - GaugeI64");
                    for data_point in &gauge.data_points {
                        println!("Attributes = {:?}", data_point.attributes);
                        println!("Value = {}", data_point.value);
                    }
                }
                AggregatedMetrics::SumF64(sum) => {
                    println!("Metric Data - SumF64");
                    for data_point in &sum.data_points {
                        println!("Attributes = {:?}", data_point.attributes);
                        println!("Value = {}", data_point.value);
                    }
                }
                AggregatedMetrics::SumI64(sum) => {
                    println!("Metric Data - SumI64");
                    for data_point in &sum.data_points {
                        println!("Attributes = {:?}", data_point.attributes);
                        println!("Value = {}", data_point.value);
                    }
                }
                AggregatedMetrics::HistogramF64(histogram) => {
                    println!("Metric Data - HistogramF64");
                    for data_point in &histogram.data_points {
                        println!("Attributes = {:?}", data_point.attributes);
                        if let Some(min) = data_point.min {
                            println!("Min = {}", min);
                        }
                        if let Some(max) = data_point.max {
                            println!("Max = {}", max);
                        }
                        println!("Sum = {}", data_point.sum);
                        println!("Count = {}", data_point.count);
                    }
                }
                AggregatedMetrics::HistogramI64(histogram) => {
                    println!("Metric Data - HistogramI64");
                    for data_point in &histogram.data_points {
                        println!("Attributes = {:?}", data_point.attributes);
                        if let Some(min) = data_point.min {
                            println!("Min = {}", min);
                        }
                        if let Some(max) = data_point.max {
                            println!("Max = {}", max);
                        }
                        println!("Sum = {}", data_point.sum);
                        println!("Count = {}", data_point.count);
                    }
                }
            };
        }
        Ok(())
    }
    fn force_flush(&self) -> crate::error::OTelSdkResult {
        Ok(())
    }
    fn shutdown(&self) -> crate::error::OTelSdkResult {
        Ok(())
    }
    fn temporality(&self) -> crate::metrics::Temporality {
        crate::metrics::Temporality::Cumulative
    }
}
