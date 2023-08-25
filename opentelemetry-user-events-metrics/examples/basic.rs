//! run with `$ cargo run --example basic --all-features
use std::{thread::sleep, time::Duration};

use opentelemetry::{
    metrics::{MeterProvider as _, Unit},
    Key, KeyValue,
};
use opentelemetry_sdk::{
    metrics::{MeterProvider, PeriodicReader},
    runtime, Resource,
};
use opentelemetry_user_events_metrics::MetricsExporter;

fn init_metrics(exporter: MetricsExporter) -> MeterProvider {
    let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();
    MeterProvider::builder()
        .with_resource(Resource::new(vec![
            KeyValue::new("service.name", "metric-demo"),
            KeyValue::new("_microsoft_metrics_account", "exemplardemo"),
            KeyValue::new("_microsoft_metrics_namespace", "cithomaslinux"),
        ]))
        .with_reader(reader)
        .build()
}

#[tokio::main]
#[allow(unused_must_use)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exporter = opentelemetry_user_events_metrics::MetricsExporter::new();
    let meter_provider = init_metrics(exporter);

    let meter = meter_provider.versioned_meter(
        "user-event-test",
        Some("test-version"),
        Some("test_url"),
        Some(vec![KeyValue::new("key", "value")]),
    );
    let c = meter
        .f64_counter("MyFruitCounter_Rust")
        .with_description("test_decription")
        .with_unit(Unit::new("test_unit"))
        .init();

    loop
    {
        c.add(1.0, [ KeyValue::new("name", "apple"), KeyValue::new("color", "red"),].as_ref(),);
        c.add(2.0, [ KeyValue::new("name", "lemon"), KeyValue::new("color", "yellow"),].as_ref(),);
        c.add(1.0, [ KeyValue::new("name", "lemon"), KeyValue::new("color", "yellow"),].as_ref(),);
        c.add(2.0, [ KeyValue::new("name", "apple"), KeyValue::new("color", "green"),].as_ref(),);
        c.add(5.0, [ KeyValue::new("name", "apple"), KeyValue::new("color", "red"),].as_ref(),);
        c.add(4.0, [ KeyValue::new("name", "lemon"), KeyValue::new("color", "yellow"),].as_ref(),);
        
        sleep(Duration::from_secs(1));
        println!("Done!");
    }
    

    meter_provider.shutdown()?;

    println!("Done!");
    Ok(())
}
