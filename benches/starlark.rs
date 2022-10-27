#[macro_use]
extern crate starlark;

use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

#[starlark_module]
fn functions(builder: &mut GlobalsBuilder) {
    fn rand(n: i32) -> anyhow::Result<i32> {
        Ok(rand::random::<i32>() % n)
    }
}

const PROG: &'static str = r#"
charset = ["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"];
def generate_string(len):
    data = []
    for i in range(len):
        data.append(charset[rand(len)])
    return ' '.join(data)

array = []
for i in range(100000):
    array.append(generate_string(rand(16) + 1));
"#;

fn benchmark(c: &mut Criterion) {
    let module = Module::new();
    let globals = GlobalsBuilder::standard().with(functions).build();

    c.bench_function("Sort userdata", |b| {
        b.iter_batched(
            || {
                let ast =
                    AstModule::parse("sort_userdata.star", PROG.to_owned(), &Dialect::Extended)
                        .expect("parse");
                let eval = Evaluator::new(&module);
                (eval, ast)
            },
            |(mut eval, ast)| {
                eval.eval_module(ast, &globals).expect("eval module");
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);
