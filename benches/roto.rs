use criterion::{criterion_group, criterion_main, Criterion};

use script_bench::roto::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::roto::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| b.iter(|| func()));
        },
        |list| {
            // Validate that the results are sorted
            let mut count = 0;
            let mut prev = RustData::default();
            let list = &list.0;
            let list = list.0.borrow();
            list.iter().for_each(|next| {
                let next = &next.0;
                assert!(prev.0 <= next.0);
                prev = next.clone();
                count += 1;
            });

            assert_eq!(count, 10000);
        },
    )
    .unwrap();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark
}

criterion_main!(benches);
