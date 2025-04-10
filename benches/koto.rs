use criterion::{criterion_group, criterion_main, Criterion};

use koto::runtime::KValue;
use script_bench::koto::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::koto::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |value| {
            // Validate that the results are sorted
            let KValue::List(list) = value else {
                panic!("Expected a list");
            };
            let mut count = 0;
            let mut prev = script_bench::koto::RustData::default();
            for next in list.data().iter() {
                let KValue::Object(obj) = next else {
                    panic!("Expected an object");
                };
                let next = obj.cast::<RustData>().unwrap();
                assert!(prev <= *next);
                prev = next.clone();
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
