use std::collections::HashMap;

use opentelemetry_api::{KeyValue, logs::Logger};
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{
    logs::{Config, LogProcessor, LoggerProvider},
    Resource,
};
use opentelemetry_user_events_logs::{ExporterConfig, ReentrantLogProcessor};
use tracing::error;
use tracing_subscriber::{prelude::*, Layer};

mod throughput;

struct NoopEventVisitor;

impl tracing::field::Visit for NoopEventVisitor {
    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {}
}

struct NoOpLogLayer;
impl<S> Layer<S> for NoOpLogLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = NoopEventVisitor;
        event.record(&mut visitor);
    }

    fn event_enabled(
        &self,
        _event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct NoOpLogProcessor;

impl LogProcessor for NoOpLogProcessor {
    fn emit(&self, _data: opentelemetry_sdk::export::logs::LogData) {}

    fn force_flush(&self) -> opentelemetry_api::logs::LogResult<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> opentelemetry_api::logs::LogResult<()> {
        Ok(())
    }

    fn event_enabled(
        &self,
        _level: opentelemetry_api::logs::Severity,
        _target: &str,
        _name: &str,
    ) -> bool {
        true
    }
}

fn init_logger_user_events_exporter() -> LoggerProvider {
    let exporter_config = ExporterConfig {
        default_keyword: 1,
        keywords_map: HashMap::new(),
    };
    let reenterant_processor = ReentrantLogProcessor::new("test", None, exporter_config);
    LoggerProvider::builder()
        .with_log_processor(reenterant_processor)
        .build()
}

fn init_logger_no_op_processor() -> LoggerProvider {
    // LoggerProvider with a no-op processor.
    let provider: LoggerProvider = LoggerProvider::builder()
        .with_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "log-appender-tracing-example",
            )])),
        )
        .with_log_processor(NoOpLogProcessor {})
        .build();
    provider
}

fn main() {
    // LoggerProvider with a no-op processor.
    let provider = init_logger_no_op_processor();
    // let provider = init_logger_user_events_exporter();

    // Use the OpenTelemetryTracingBridge to test the throughput of the appender-tracing.
    let layer = layer::OpenTelemetryTracingBridge::new(&provider);
    tracing_subscriber::registry().with(layer).init();

    // Use a "Do-Nothing" layer to test the throughput of the tracing system without
    // OpenTelemetry overhead. This helps measure the OpenTelemetry overhead.
    // let noop_layer = NoOpLogLayer;
    // tracing_subscriber::registry().with(noop_layer).init();

    throughput::test_throughput(test_log);
}

fn test_log() {
    error!(target: "my-system", event_id = 20, event_name = "my-event_name", user_name = "otel", user_email = "otel@opentelemetry.io");
}
