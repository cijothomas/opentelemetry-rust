use criterion::{criterion_group, criterion_main, Criterion};
use opentelemetry::KeyValue;
use opentelemetry_sdk::AttributeSet;

// Run this benchmark with:
// cargo bench --bench metric_counter

fn criterion_benchmark(c: &mut Criterion) {
    attribute_set(c);
}

fn attribute_set(c: &mut Criterion) {
    c.bench_function("AttributeSet_without_duplicates_sorted", |b| {
        b.iter(|| {
            let attributes: &[KeyValue] = &[
                KeyValue::new("attribute1", "value1"),
                KeyValue::new("attribute2", "value2"),
                KeyValue::new("attribute3", "value3"),
                KeyValue::new("attribute4", "value4"),
            ];
            let _attribute_set: AttributeSet = attributes.into();
        });
    });

    c.bench_function("AttributeSet_without_duplicates_unsorted", |b| {
        b.iter(|| {
            let attributes: &[KeyValue] = &[                                
                KeyValue::new("attribute3", "value3"),
                KeyValue::new("attribute2", "value2"),
                KeyValue::new("attribute1", "value1"),
                KeyValue::new("attribute4", "value4"),
            ];
            let _attribute_set: AttributeSet = attributes.into();
        });
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
