use criterion::{criterion_group, criterion_main, Criterion};

use script_bench::rhai::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::rhai::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |array| {
            // Validate that the results are sorted
            assert_eq!(array.len(), 10000);
            let mut prev = RustData::default();
            for next in array {
                let next = next.cast::<RustData>();
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
