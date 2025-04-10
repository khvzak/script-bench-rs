use criterion::{criterion_group, criterion_main, Criterion};

use boa_engine::object::builtins::JsArray;
use boa_engine::value::TryFromJs;
use script_bench::boa::RustData;

fn benchmark(c: &mut Criterion) {
    script_bench::boa::sort_userdata(
        |func| {
            c.bench_function("Sort Rust objects", |b| {
                b.iter(|| func());
            });
        },
        |value, ctx| {
            // Validate that the results are sorted
            let array: JsArray = TryFromJs::try_from_js(&value, ctx).unwrap();
            let len = array.length(ctx).unwrap();
            assert_eq!(len, 10000);
            let mut prev = RustData::default();
            for i in 0..len {
                let next = array.get(i, ctx).unwrap();
                let next = next.as_object().unwrap();
                let next = next.downcast_ref::<RustData>().unwrap();
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
