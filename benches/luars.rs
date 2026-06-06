use criterion::{criterion_group, criterion_main, Criterion};
use luars::Value;

use script_bench::luars::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::luars::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |table| {
            let values = table.sequence_values::<RustData>().unwrap();
            assert_eq!(values.len(), 10000);

            let mut prev = RustData::default();
            for next in values {
                assert!(prev <= next);
                prev = next;
            }
        },
    )
    .unwrap();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);