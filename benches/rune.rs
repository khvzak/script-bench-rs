use criterion::{criterion_group, criterion_main, Criterion};

use script_bench::rune::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::rune::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |value| {
            // Validate that the results are sorted
            let array = value.into_vec().unwrap();
            let array = array.take().unwrap();
            assert_eq!(array.len(), 10000);
            let mut prev = RustData::default();
            for next in array {
                let next = next.into_any().unwrap();
                let next = next.downcast_into_ref::<RustData>().unwrap();
                assert!(prev <= *next);
                prev = next.clone();
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
