/* OS: Ubuntu 22.04.4 LTS (5.15.153.1-microsoft-standard-WSL2)
Hardware: Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz, 16vCPUs,
RAM: 64.0 GB
| Test                           | Average time|
|--------------------------------|-------------|
| CreateOTelKey_Static           |      1.2 ns |
| CreateOTelKey_Owned            |     12.6 ns |
| CreateOTelKey_Arc              |    23.35 ns |
| CreateOTelKeyValue             |     3.24 ns |
| CreateTupleKeyValue            |      671 ps |
| CreateOtelKeyValueArray        |     18.4 ns |
| CreateTupleKeyValueArray       |     2.73 ns |
*/

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opentelemetry::{Key, KeyValue};
use std::sync::Arc;

// Run this benchmark with:
// cargo bench --bench attributes

fn criterion_benchmark(c: &mut Criterion) {
    attributes_creation(c);
}

fn attributes_creation(c: &mut Criterion) {
    c.bench_function("CreateOTelKey_Static", |b| {
        b.iter(|| {
            let _v1 = black_box(Key::new("attribute1"));
        });
    });

    c.bench_function("CreateOTelKey_Owned", |b| {
        b.iter(|| {
            let _v1 = black_box(Key::new(String::from("attribute1")));
        });
    });

    c.bench_function("CreateOTelKey_Arc", |b| {
        b.iter(|| {
            let _v1 = black_box(Key::new(Arc::from("attribute1")));
        });
    });

    c.bench_function("CreateOTelKeyValue", |b| {
        b.iter(|| {
            let _v1 = black_box(KeyValue::new("attribute1", "value1"));
        });
    });

    c.bench_function("CreateTupleKeyValue", |b| {
        b.iter(|| {
            let _v1 = black_box(("attribute1", "value1"));
        });
    });

    #[allow(clippy::useless_vec)]
    c.bench_function("CreateOtelKeyValueArray", |b| {
        b.iter(|| {
            let _v1 = black_box([
                KeyValue::new("attribute1", "value1"),
                KeyValue::new("attribute2", "value2"),
                KeyValue::new("attribute3", "value3"),
                KeyValue::new("attribute4", "value4"),
            ]);
        });
    });

    #[allow(clippy::useless_vec)]
    c.bench_function("CreateTupleKeyValueArray", |b| {
        b.iter(|| {
            let _v1 = black_box([
                ("attribute1", "value1"),
                ("attribute2", "value2"),
                ("attribute3", "value3"),
                ("attribute4", "value4"),
            ]);
        });
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
