use criterion::{criterion_group, criterion_main, Criterion};

use script_bench::rquickjs::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::rquickjs::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |array| {
            // Validate that the results are sorted
            let mut count = 0;
            let mut prev = RustData::default();
            for next in array.iter::<RustData>() {
                let next = next.unwrap();
                assert!(prev <= next);
                prev = next;
                count += 1;
            }
            assert_eq!(count, 10000);
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
