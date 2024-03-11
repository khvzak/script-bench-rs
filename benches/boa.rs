use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    script_bench::boa::sort_userdata(|func| {
        c.bench_function("Sort Rust objects", |b| {
            b.iter(|| func());
        });
    })
    .unwrap();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);
