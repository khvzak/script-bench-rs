use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Any, Context, ContextError, Diagnostics, Module, Source, Sources, Vm};

#[derive(Any, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Arc<String>);

impl RustData {
    pub fn new(s: String) -> Self {
        RustData(Arc::new(s))
    }
}

const PROG: &'static str = r#"
pub fn main() {
    let charset = ["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"];
    let generate_string = |len| {
        let data = Vec::new();
        for i in 0..len {
            data.push(charset[rand(charset.len())]);
        }
        return concat(data);
    };

    let array = Vec::new();
    for i in 0..100000 {
        array.push(RustData::new(generate_string(rand(16) + 1)));
    }

    array.sort_by(|a, b| a.cmp(b));
}"#;

pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();

    module.ty::<RustData>()?;
    module.function(&["RustData", "new"], RustData::new)?;
    module.inst_fn("cmp", RustData::cmp)?;

    module.function(&["rand"], |n: i64| rand::random::<u32>() as i64 % n)?;
    module.function(&["concat"], |items: Vec<String>| -> String {
        items.join("")
    })?;

    Ok(module)
}

pub fn benchmark(c: &mut Criterion) {
    let mut sources = Sources::new();
    sources.insert(Source::new("sort_userdata.rn", PROG));

    let mut context = Context::with_default_modules().unwrap();
    context.install(&module().unwrap()).unwrap();

    let runtime = Arc::new(context.runtime());

    let mut diag = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diag)
        .build();

    if !diag.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diag.emit(&mut writer, &sources).unwrap();
    }

    let unit = result.unwrap();

    let mut vm = Vm::new(runtime, Arc::new(unit));
    c.bench_function("Sort userdata", |b| {
        b.iter(|| {
            let _ = vm.call(&["main"], ()).unwrap();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);
