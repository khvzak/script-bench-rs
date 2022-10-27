use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rhai::{Array, Engine};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Arc<String>);

impl RustData {
    pub fn new(s: String) -> Self {
        RustData(Arc::new(s))
    }
}

const PROG: &'static str = r#"
    let charset = ["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"];
    let generate_string = |len| {
        let data = [];
        for i in 0..len {
            data.push(charset[rand(charset.len)]);
        }
        return concat(data);
    };

    let array = [];
    for i in 0..100000 {
        array.push(RustData_new(generate_string.call(rand(16) + 1)));
    }
"#;

fn benchmark(c: &mut Criterion) {
    let mut engine = Engine::new();

    engine
        .register_type_with_name::<RustData>("RustData")
        .register_fn("RustData_new", RustData::new)
        .register_fn("<", |l: &mut RustData, r: RustData| *l < r)
        .register_fn("rand", |n: i64| rand::random::<u32>() as i64 % n)
        .register_fn("concat", |items: Array| {
            items
                .into_iter()
                .map(|x| x.into_immutable_string().unwrap())
                .join("")
        });

    let ast = engine.compile(PROG).unwrap();

    c.bench_function("Sort userdata", |b| {
        b.iter(|| {
            engine.eval_ast::<()>(&ast).unwrap();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);
