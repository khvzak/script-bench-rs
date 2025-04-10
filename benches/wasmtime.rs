use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    script_bench::wasm::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |result| {
            // Validate that the results are sorted
            assert_eq!(result.len(), 10000);
            for i in 0..result.len() - 1 {
                assert!(result[i] <= result[i + 1],);
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
