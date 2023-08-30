use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server,
};
use once_cell::sync::Lazy;
use opentelemetry::{
    metrics::{Counter, Histogram, MeterProvider as _, Unit},
    KeyValue,
};
use opentelemetry_sdk::metrics::MeterProvider;
use prometheus::{Encoder, Registry, TextEncoder};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::SystemTime;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

static HANDLER_ALL: Lazy<[KeyValue; 1]> = Lazy::new(|| [KeyValue::new("handler", "all")]);

async fn serve_req(
    req: Request<Body>,
    state: Arc<AppState>,
) -> Result<Response<Body>, hyper::Error> {
    println!("Receiving request at path {}", req.uri());
    let request_start = SystemTime::now();

    state.http_counter.add(1, HANDLER_ALL.as_ref());

    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = state.registry.gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            state
                .http_response_size
                .record(buffer.len() as u64, HANDLER_ALL.as_ref());

            Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap()
        }
        (&Method::GET, "/") => {
            state.http_req_histogram.record(
                request_start.elapsed().map_or(0.0, |d| d.as_secs_f64()),
                [KeyValue::new("status_code", "200")].as_ref(),
            );
            Response::builder()
                .status(200)
                .body(Body::from("Hello World"))
                .unwrap()
        }
        _ => {
            state.http_req_histogram.record(
                request_start.elapsed().map_or(0.0, |d| d.as_secs_f64()),
                [KeyValue::new("status_code", "404")].as_ref(),
            );

            Response::builder()
                .status(404)
                .body(Body::from("Missing Page"))
                .unwrap()
        }
    };

    Ok(response)
}

struct AppState {
    registry: Registry,
    http_counter: Counter<u64>,
    http_response_size: Histogram<u64>,
    http_req_histogram: Histogram<f64>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()?;
    let provider = MeterProvider::builder().with_reader(exporter).build();

    let mut sys = System::new_all();
    sys.refresh_all();
    let meter = provider.meter("hyper-example");
    let gauge_system_memory = meter
        .u64_observable_gauge("system_memory")
        .with_description("System memory")
        .with_unit(Unit::new("by"))
        .init();

    meter.register_callback(&[gauge_system_memory.as_any()], move |observer| {
        observer.observe_u64(
            &gauge_system_memory,
            sys.total_memory(),
            [
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ]
            .as_ref(),
        )
    })?;

    let state = Arc::new(AppState {
        registry,
        http_counter: meter
            .u64_counter("http_requests_total")
            .with_description("Total number of HTTP requests made.")
            .init(),
        http_response_size: meter
            .u64_histogram("http_response_size")
            .with_unit(Unit::new("By"))
            .with_description("The metrics HTTP response sizes in bytes.")
            .init(),
        http_req_histogram: meter
            .f64_histogram("http_request_duration")
            .with_unit(Unit::new("ms"))
            .with_description("The HTTP request latencies in milliseconds.")
            .init(),
    });

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(move |_conn| {
        let state = state.clone();
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async move { Ok::<_, Infallible>(service_fn(move |req| serve_req(req, state.clone()))) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{addr}");

    server.await?;

    Ok(())
}
