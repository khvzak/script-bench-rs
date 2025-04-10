use std::sync::Arc;

use rand::Rng;
use rune::runtime::Function;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Any, Context, Diagnostics, Module, Source, Sources, Value, Vm};

#[derive(Any, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustData(Arc<str>);

impl RustData {
    #[rune::function(path = Self::new)]
    pub fn new(s: &str) -> Self {
        RustData(s.into())
    }

    #[rune::function]
    fn lt(&self, other: &Self) -> bool {
        self < other
    }

    #[rune::function]
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[rune::function]
fn rand(n: u32) -> u32 {
    rand::rng().random_range(0..n)
}

#[rune::function]
fn concat(items: Vec<Value>) -> String {
    let mut output = String::new();
    for s in items {
        let s = s.into_string().unwrap();
        output.push_str(&s.borrow_ref().unwrap());
    }
    output
}

pub fn sort_userdata(
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(Value),
) -> anyhow::Result<()> {
    let mut context = Context::with_default_modules()?;

    let mut module = Module::default();
    module.ty::<RustData>()?;
    module.function_meta(RustData::new)?;
    module.function_meta(RustData::lt)?;
    module.function_meta(RustData::to_string)?;
    module.function_meta(rand)?;
    module.function_meta(concat)?;
    context.install(module)?;

    let runtime = Arc::new(context.runtime()?);

    let mut sources = Sources::new();
    sources.insert(Source::memory(include_str!("../scripts/sort_userdata.rn"))?)?;

    let mut diagnostics = Diagnostics::new();
    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();
    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let mut vm = Vm::new(runtime, Arc::new(result?));
    let output = vm.call(["main"], ())?;
    let bench: Function = rune::from_value(output)?;

    validate(bench.call::<_, Value>(()).unwrap());
    run(&mut || {
        bench.call::<_, Value>(()).unwrap();
    });

    Ok(())
}
