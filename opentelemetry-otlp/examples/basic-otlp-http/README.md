# Basic OTLP exporter Example

This example shows how to setup OpenTelemetry OTLP exporter for logs, metrics
and traces to exports them to the [OpenTelemetry
Collector](https://github.com/open-telemetry/opentelemetry-collector) via OTLP over HTTP.
The Collector then sends the data to the appropriate backend, in this case,
the logging Exporter, which displays data to console.

## Usage

### `docker-compose`

By default runs against the `otel/opentelemetry-collector:latest` image, and uses `reqwest-client`
as the http client, using http as the transport.

```shell
docker-compose up
```

In another terminal run the application `cargo run`

The docker-compose terminal will display logs, traces, metrics.

Press Ctrl+C to stop the collector, and then tear it down:

```shell
docker-compose down
```

### Manual

If you don't want to use `docker-compose`, you can manually run the `otel/opentelemetry-collector` container
and inspect the logs to see traces being transferred.

```shell
# From the current directory, run `opentelemetry-collector`
$ docker run --rm -it -p 4318:4318 -v $(pwd):/cfg otel/opentelemetry-collector:latest --config=/cfg/otel-collector-config.yaml

# Run the app which exports logs, metrics and traces via OTLP to the collector.
$ cargo run
```

## View results

You should be able to see something similar below with different time and ID in the same console that docker runs.

```text
WARN[0000] The "OTELCOL_ARGS" variable is not set. Defaulting to a blank string.
[+] Running 2/0
 ✔ Network basic-otlp-http_default             Created                                                                                                                                                                               0.0s
 ✔ Container basic-otlp-http-otel-collector-1  Created                                                                                                                                                                               0.0s
Attaching to basic-otlp-http-otel-collector-1
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.720Z	info	service@v0.91.0/telemetry.go:86	Setting up own telemetry...
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.720Z	info	service@v0.91.0/telemetry.go:203	Serving Prometheus metrics	{"address": ":8888", "level": "Basic"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.720Z	info	exporter@v0.91.0/exporter.go:275	Deprecated component. Will be removed in future releases.	{"kind": "exporter", "data_type": "metrics", "name": "logging"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.720Z	warn	common/factory.go:68	'loglevel' option is deprecated in favor of 'verbosity'. Set 'verbosity' to equivalent value to preserve behavior.	{"kind": "exporter", "data_type": "metrics", "name": "logging", "loglevel": "debug", "equivalent verbosity level": "Detailed"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.720Z	info	exporter@v0.91.0/exporter.go:275	Deprecated component. Will be removed in future releases.	{"kind": "exporter", "data_type": "traces", "name": "logging"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.720Z	info	exporter@v0.91.0/exporter.go:275	Deprecated component. Will be removed in future releases.	{"kind": "exporter", "data_type": "logs", "name": "logging"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.740Z	info	service@v0.91.0/service.go:145	Starting otelcol...	{"Version": "0.91.0", "NumCPU": 16}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.740Z	info	extensions/extensions.go:34	Starting extensions...
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.740Z	warn	internal@v0.91.0/warning.go:40	Using the 0.0.0.0 address exposes this server to every network interface, which may facilitate Denial of Service attacks	{"kind": "receiver", "name": "otlp", "data_type": "metrics", "documentation": "https://github.com/open-telemetry/opentelemetry-collector/blob/main/docs/security-best-practices.md#safeguards-against-denial-of-service-attacks"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.740Z	info	otlpreceiver@v0.91.0/otlp.go:83	Starting GRPC server	{"kind": "receiver", "name": "otlp", "data_type": "metrics", "endpoint": "0.0.0.0:4317"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.740Z	warn	internal@v0.91.0/warning.go:40	Using the 0.0.0.0 address exposes this server to every network interface, which may facilitate Denial of Service attacks	{"kind": "receiver", "name": "otlp", "data_type": "metrics", "documentation": "https://github.com/open-telemetry/opentelemetry-collector/blob/main/docs/security-best-practices.md#safeguards-against-denial-of-service-attacks"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.740Z	info	otlpreceiver@v0.91.0/otlp.go:101	Starting HTTP server	{"kind": "receiver", "name": "otlp", "data_type": "metrics", "endpoint": "0.0.0.0:4318"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:19.740Z	info	service@v0.91.0/service.go:171	Everything is ready. Begin running and processing data.
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.387Z	info	TracesExporter	{"kind": "exporter", "data_type": "traces", "name": "logging", "resource spans": 2, "spans": 2}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.387Z	info	ResourceSpans #0
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeSpans #0
basic-otlp-http-otel-collector-1  | ScopeSpans SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope basic
basic-otlp-http-otel-collector-1  | InstrumentationScope attributes:
basic-otlp-http-otel-collector-1  |      -> scope-key: Str(scope-value)
basic-otlp-http-otel-collector-1  | Span #0
basic-otlp-http-otel-collector-1  |     Trace ID       : bb26ea9055aecc7c5d86a8e8546d66bf
basic-otlp-http-otel-collector-1  |     Parent ID      : c452f8e9e9952dca
basic-otlp-http-otel-collector-1  |     ID             : 19c83cf3eaa5fc71
basic-otlp-http-otel-collector-1  |     Name           : Sub operation...
basic-otlp-http-otel-collector-1  |     Kind           : Internal
basic-otlp-http-otel-collector-1  |     Start time     : 2024-05-14 01:33:28.385854614 +0000 UTC
basic-otlp-http-otel-collector-1  |     End time       : 2024-05-14 01:33:28.385858334 +0000 UTC
basic-otlp-http-otel-collector-1  |     Status code    : Unset
basic-otlp-http-otel-collector-1  |     Status message :
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> another.key: Str(yes)
basic-otlp-http-otel-collector-1  | Events:
basic-otlp-http-otel-collector-1  | SpanEvent #0
basic-otlp-http-otel-collector-1  |      -> Name: Sub span event
basic-otlp-http-otel-collector-1  |      -> Timestamp: 2024-05-14 01:33:28.385856276 +0000 UTC
basic-otlp-http-otel-collector-1  |      -> DroppedAttributesCount: 0
basic-otlp-http-otel-collector-1  | ResourceSpans #1
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeSpans #0
basic-otlp-http-otel-collector-1  | ScopeSpans SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope basic
basic-otlp-http-otel-collector-1  | InstrumentationScope attributes:
basic-otlp-http-otel-collector-1  |      -> scope-key: Str(scope-value)
basic-otlp-http-otel-collector-1  | Span #0
basic-otlp-http-otel-collector-1  |     Trace ID       : bb26ea9055aecc7c5d86a8e8546d66bf
basic-otlp-http-otel-collector-1  |     Parent ID      :
basic-otlp-http-otel-collector-1  |     ID             : c452f8e9e9952dca
basic-otlp-http-otel-collector-1  |     Name           : Main operation
basic-otlp-http-otel-collector-1  |     Kind           : Internal
basic-otlp-http-otel-collector-1  |     Start time     : 2024-05-14 01:33:28.385824073 +0000 UTC
basic-otlp-http-otel-collector-1  |     End time       : 2024-05-14 01:33:28.385861746 +0000 UTC
basic-otlp-http-otel-collector-1  |     Status code    : Unset
basic-otlp-http-otel-collector-1  |     Status message :
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> another.key: Str(yes)
basic-otlp-http-otel-collector-1  | Events:
basic-otlp-http-otel-collector-1  | SpanEvent #0
basic-otlp-http-otel-collector-1  |      -> Name: Nice operation!
basic-otlp-http-otel-collector-1  |      -> Timestamp: 2024-05-14 01:33:28.385829396 +0000 UTC
basic-otlp-http-otel-collector-1  |      -> DroppedAttributesCount: 0
basic-otlp-http-otel-collector-1  |      -> Attributes::
basic-otlp-http-otel-collector-1  |           -> bogons: Int(100)
basic-otlp-http-otel-collector-1  | 	{"kind": "exporter", "data_type": "traces", "name": "logging"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.387Z	info	MetricsExporter	{"kind": "exporter", "data_type": "metrics", "name": "logging", "resource metrics": 1, "metrics": 1, "data points": 1}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.387Z	info	ResourceMetrics #0
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeMetrics #0
basic-otlp-http-otel-collector-1  | ScopeMetrics SchemaURL: schema_url
basic-otlp-http-otel-collector-1  | InstrumentationScope basic v1.0
basic-otlp-http-otel-collector-1  | InstrumentationScope attributes:
basic-otlp-http-otel-collector-1  |      -> scope-key: Str(scope-value)
basic-otlp-http-otel-collector-1  | Metric #0
basic-otlp-http-otel-collector-1  | Descriptor:
basic-otlp-http-otel-collector-1  |      -> Name: test_counter
basic-otlp-http-otel-collector-1  |      -> Description: a simple counter for demo purposes.
basic-otlp-http-otel-collector-1  |      -> Unit: my_unit
basic-otlp-http-otel-collector-1  |      -> DataType: Sum
basic-otlp-http-otel-collector-1  |      -> IsMonotonic: true
basic-otlp-http-otel-collector-1  |      -> AggregationTemporality: Cumulative
basic-otlp-http-otel-collector-1  | NumberDataPoints #0
basic-otlp-http-otel-collector-1  | Data point attributes:
basic-otlp-http-otel-collector-1  |      -> test_key: Str(test_value)
basic-otlp-http-otel-collector-1  | StartTimestamp: 2024-05-14 01:33:28.385773324 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 2024-05-14 01:33:28.386720559 +0000 UTC
basic-otlp-http-otel-collector-1  | Value: 11
basic-otlp-http-otel-collector-1  | 	{"kind": "exporter", "data_type": "metrics", "name": "logging"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.387Z	info	LogsExporter	{"kind": "exporter", "data_type": "logs", "name": "logging", "resource logs": 6, "log records": 6}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.387Z	info	ResourceLog #0
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.385846842 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: INFO
basic-otlp-http-otel-collector-1  | SeverityNumber: Info(9)
basic-otlp-http-otel-collector-1  | Body: Str(hello from banana. My price is 2.99. I am also inside a Span!)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event opentelemetry-otlp/examples/basic-otlp-http/src/main.rs:126)
basic-otlp-http-otel-collector-1  | Trace ID: bb26ea9055aecc7c5d86a8e8546d66bf
basic-otlp-http-otel-collector-1  | Span ID: c452f8e9e9952dca
basic-otlp-http-otel-collector-1  | Flags: 1
basic-otlp-http-otel-collector-1  | ResourceLog #1
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.385865973 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: INFO
basic-otlp-http-otel-collector-1  | SeverityNumber: Info(9)
basic-otlp-http-otel-collector-1  | Body: Str(hello from apple. My price is 1.99)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event opentelemetry-otlp/examples/basic-otlp-http/src/main.rs:135)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #2
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.3861316 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(checkout waiting for idle connection: ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:639)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #3
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386189042 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Http::connect; scheme=Some("http"), host=Some("localhost"), port=Some(Port(4318)))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:278)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #4
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386329931 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(resolving host="localhost")
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/dns.rs:122)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #5
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386558157 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(connecting to 127.0.0.1:4318)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:542)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | 	{"kind": "exporter", "data_type": "logs", "name": "logging"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.389Z	info	LogsExporter	{"kind": "exporter", "data_type": "logs", "name": "logging", "resource logs": 85, "log records": 85}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.389Z	info	ResourceLog #0
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38671525 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(connected to 127.0.0.1:4318)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:545)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #1
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38673263 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(client handshake Http1)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/conn.rs:1007)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #2
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386763713 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(handshake complete, spawning background dispatcher task)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/client.rs:509)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #3
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386808854 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #4
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386824144 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(checkout dropped for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:681)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #5
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386871209 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(checkout waiting for idle connection: ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:639)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #6
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386871399 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(checkout waiting for idle connection: ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:639)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #7
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386899673 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Http::connect; scheme=Some("http"), host=Some("localhost"), port=Some(Port(4318)))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:278)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #8
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386898916 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Http::connect; scheme=Some("http"), host=Some("localhost"), port=Some(Port(4318)))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:278)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #9
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386923104 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Client::encode method=POST, body=Some(Known(441)))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:1110)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #10
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386950105 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(resolving host="localhost")
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/dns.rs:122)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #11
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386964396 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(sized write, len = 441)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/encode.rs:159)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #12
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.386970628 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(buffer.queue)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:595)
basic-otlp-http-otel-collector-1  |      -> self.len: Str(166)
basic-otlp-http-otel-collector-1  |      -> buf.len: Str(441)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #13
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38697825 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(resolving host="localhost")
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/dns.rs:122)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #14
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387000202 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(flushed 607 bytes)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:318)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #15
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387004234 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #16
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38700801 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(connecting to 127.0.0.1:4318)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:542)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #17
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387054361 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(connecting to 127.0.0.1:4318)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:542)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #18
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38711324 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(connected to 127.0.0.1:4318)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:545)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #19
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387130414 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(client handshake Http1)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/conn.rs:1007)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #20
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387135708 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(connected to 127.0.0.1:4318)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/connect/http.rs:545)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #21
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387146518 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(handshake complete, spawning background dispatcher task)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/client.rs:509)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #22
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387149098 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(client handshake Http1)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/conn.rs:1007)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #23
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387161772 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(handshake complete, spawning background dispatcher task)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/client.rs:509)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #24
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387181457 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #25
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387187448 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #26
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387199648 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(checkout dropped for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:681)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #27
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387205124 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(checkout dropped for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:681)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #28
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387271945 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Client::encode method=POST, body=Some(Known(1667)))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:1110)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #29
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387272392 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Client::encode method=POST, body=Some(Known(224)))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:1110)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #30
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387298881 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(sized write, len = 224)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/encode.rs:159)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #31
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387298885 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(sized write, len = 1667)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/encode.rs:159)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #32
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387306555 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(buffer.queue)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:595)
basic-otlp-http-otel-collector-1  |      -> self.len: Str(167)
basic-otlp-http-otel-collector-1  |      -> buf.len: Str(224)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #33
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387308313 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(buffer.queue)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:595)
basic-otlp-http-otel-collector-1  |      -> self.len: Str(165)
basic-otlp-http-otel-collector-1  |      -> buf.len: Str(1667)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #34
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387337037 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(flushed 1832 bytes)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:318)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #35
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387337042 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(flushed 391 bytes)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:318)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #36
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387343195 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #37
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387343192 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #38
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387423796 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Conn::read_head)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:193)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #39
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38743246 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(received 115 bytes)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:267)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #40
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387447387 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Response.parse)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:946)
basic-otlp-http-otel-collector-1  |      -> bytes: Str(115)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #41
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387460126 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Response.parse Complete(113))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:955)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #42
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38749402 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(parsed 3 headers)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:208)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #43
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387498592 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(incoming body is content-length (2 bytes))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:224)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #44
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387517189 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(decode; state=Length(2))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/decode.rs:127)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #45
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387523251 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(incoming body completed)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:300)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #46
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387534016 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(maybe_notify; read_from_io blocked)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:471)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #47
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387548364 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #48
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387555375 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #49
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387582153 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(put; add idle connection for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:333)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #50
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387592716 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(pooling idle connection for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:380)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #51
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387679649 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(client tx closed)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/dispatch.rs:601)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #52
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387680312 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(pool closed, canceling idle interval)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:759)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #53
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387680637 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Conn::read_head)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:193)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #54
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387685473 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Conn::read_head)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:193)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #55
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38768712 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(State::close_read())
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:955)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #56
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38769123 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(State::close_write())
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:961)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #57
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387691989 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(received 115 bytes)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:267)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #58
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387697367 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(received 115 bytes)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:267)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #59
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387699296 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #60
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387717803 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Response.parse)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:946)
basic-otlp-http-otel-collector-1  |      -> bytes: Str(115)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #61
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387718151 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(shut down IO complete)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:738)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #62
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387720808 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Response.parse)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:946)
basic-otlp-http-otel-collector-1  |      -> bytes: Str(115)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #63
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387729027 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Response.parse Complete(113))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:955)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #64
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387731286 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(Response.parse Complete(113))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/role.rs:955)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #65
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387757383 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(parsed 3 headers)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:208)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #66
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387759178 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(parsed 3 headers)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/io.rs:208)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #67
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38776125 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(incoming body is content-length (2 bytes))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:224)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #68
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387766345 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(incoming body is content-length (2 bytes))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:224)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #69
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387774026 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(decode; state=Length(2))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/decode.rs:127)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #70
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387777732 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(incoming body completed)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:300)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #71
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387780848 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(decode; state=Length(2))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/decode.rs:127)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #72
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387784375 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(incoming body completed)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:300)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #73
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387785555 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(maybe_notify; read_from_io blocked)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:471)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #74
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387791698 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(maybe_notify; read_from_io blocked)
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:471)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #75
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38779653 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #76
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.38780108 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #77
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387803635 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #78
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387808251 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #79
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387819232 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(put; add idle connection for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:333)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #80
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387825154 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(put; add idle connection for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:333)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #81
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387825866 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(pooling idle connection for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:380)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #82
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387832711 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: DEBUG
basic-otlp-http-otel-collector-1  | SeverityNumber: Debug(5)
basic-otlp-http-otel-collector-1  | Body: Str(pooling idle connection for ("http", localhost:4318))
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/client/pool.rs:380)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #83
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387879308 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | ResourceLog #84
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeLogs #0
basic-otlp-http-otel-collector-1  | ScopeLogs SchemaURL:
basic-otlp-http-otel-collector-1  | InstrumentationScope opentelemetry-appender-tracing 0.3.0
basic-otlp-http-otel-collector-1  | LogRecord #0
basic-otlp-http-otel-collector-1  | ObservedTimestamp: 2024-05-14 01:33:28.387942933 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 1970-01-01 00:00:00 +0000 UTC
basic-otlp-http-otel-collector-1  | SeverityText: TRACE
basic-otlp-http-otel-collector-1  | SeverityNumber: Trace(1)
basic-otlp-http-otel-collector-1  | Body: Str(flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle })
basic-otlp-http-otel-collector-1  | Attributes:
basic-otlp-http-otel-collector-1  |      -> name: Str(event /home/zhongyang/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyper-0.14.28/src/proto/h1/conn.rs:731)
basic-otlp-http-otel-collector-1  | Trace ID:
basic-otlp-http-otel-collector-1  | Span ID:
basic-otlp-http-otel-collector-1  | Flags: 0
basic-otlp-http-otel-collector-1  | 	{"kind": "exporter", "data_type": "logs", "name": "logging"}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.390Z	info	MetricsExporter	{"kind": "exporter", "data_type": "metrics", "name": "logging", "resource metrics": 1, "metrics": 1, "data points": 1}
basic-otlp-http-otel-collector-1  | 2024-05-14T01:33:28.390Z	info	ResourceMetrics #0
basic-otlp-http-otel-collector-1  | Resource SchemaURL:
basic-otlp-http-otel-collector-1  | Resource attributes:
basic-otlp-http-otel-collector-1  |      -> service.name: Str(basic-otlp-example)
basic-otlp-http-otel-collector-1  | ScopeMetrics #0
basic-otlp-http-otel-collector-1  | ScopeMetrics SchemaURL: schema_url
basic-otlp-http-otel-collector-1  | InstrumentationScope basic v1.0
basic-otlp-http-otel-collector-1  | InstrumentationScope attributes:
basic-otlp-http-otel-collector-1  |      -> scope-key: Str(scope-value)
basic-otlp-http-otel-collector-1  | Metric #0
basic-otlp-http-otel-collector-1  | Descriptor:
basic-otlp-http-otel-collector-1  |      -> Name: test_counter
basic-otlp-http-otel-collector-1  |      -> Description: a simple counter for demo purposes.
basic-otlp-http-otel-collector-1  |      -> Unit: my_unit
basic-otlp-http-otel-collector-1  |      -> DataType: Sum
basic-otlp-http-otel-collector-1  |      -> IsMonotonic: true
basic-otlp-http-otel-collector-1  |      -> AggregationTemporality: Cumulative
basic-otlp-http-otel-collector-1  | NumberDataPoints #0
basic-otlp-http-otel-collector-1  | Data point attributes:
basic-otlp-http-otel-collector-1  |      -> test_key: Str(test_value)
basic-otlp-http-otel-collector-1  | StartTimestamp: 2024-05-14 01:33:28.385773324 +0000 UTC
basic-otlp-http-otel-collector-1  | Timestamp: 2024-05-14 01:33:28.389972564 +0000 UTC
basic-otlp-http-otel-collector-1  | Value: 11
basic-otlp-http-otel-collector-1  | 	{"kind": "exporter", "data_type": "metrics", "name": "logging"}
```