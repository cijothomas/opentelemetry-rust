use opentelemetry::metrics::MeterProvider;
use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics_new::manual_reader::ManualReader;
use opentelemetry_sdk::metrics_new::meter_provider::SdkMeterProvider;
use opentelemetry_sdk::metrics_new::stdout_exporter::StdOutExporter;
use opentelemetry_sdk::Resource;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resource = Resource::builder()
        .with_attribute(KeyValue::new("service.name", "my-service"))
        .build();
    let stdout_exporter = StdOutExporter::new();
    let reader = ManualReader::new(stdout_exporter);
    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_reader(reader)
        .build();

    let _meter = meter_provider.meter("my-meter");

    meter_provider.force_flush();

    Ok(())
}
