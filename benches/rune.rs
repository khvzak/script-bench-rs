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
        array.push(RustData::new(generate_string(8 + rand(16))));
    }

    fn partition(arr, lo, hi) {
        let pivot = arr[(hi - lo) / 2 + lo];
        let i = lo - 1;
        let j = hi + 1;
        loop {
            loop {
                i = i + 1;
                if !arr[i].lt(pivot) {
                    break;
                }
            }
            loop {
                j = j - 1;
                if arr[j].le(pivot) {
                    break;
                }
            }
            if i >= j {
                return j;
            }
            let t = arr[i];
            arr[i] = arr[j];
            arr[j] = t;
        }
    }

    fn quicksort(arr, lo, hi) {
        while lo >= 0 && hi >= 0 && lo < hi {
            let p = partition(arr, lo, hi);
            quicksort(arr, lo, p);
            // tail recursion
            lo = p + 1;
        }
    }

    quicksort(array, 0, array.len() - 1);
}"#;

pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();

    module.ty::<RustData>()?;
    module.function(&["RustData", "new"], RustData::new)?;
    module.inst_fn("lt", |a: &RustData, b: &RustData| a < b)?;
    module.inst_fn("le", |a: &RustData, b: &RustData| a <= b)?;

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
