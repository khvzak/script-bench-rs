use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    script_bench::rustscript::sort_userdata(
        false,
        |func| {
            c.bench_function("Sort Rust objects", |b| b.iter(|| func()));
        },
        script_bench::rustscript::validate_sorted_handles,
    )
    .unwrap();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);
