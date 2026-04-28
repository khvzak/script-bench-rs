use criterion::{criterion_group, criterion_main, Criterion};

use script_bench::mlua::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::mlua::sort_userdata(
        |func| {
            // Benchmark the function
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |table| {
            // Validate that the results are sorted
            let mut count = 0;
            let mut prev = RustData::default();
            table
                .for_each_value(|next: mlua::UserDataRef<RustData>| {
                    assert!(prev <= *next);
                    prev = next.clone();
                    count += 1;
                    Ok(())
                })
                .unwrap();
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
