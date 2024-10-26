use std::rc::Rc;

use boa_engine::class::{Class, ClassBuilder};
use boa_engine::property::Attribute;
use boa_engine::{
    js_string, Context, Finalize, JsData, JsResult, JsString, JsValue, NativeFunction, Source,
    Trace,
};
use boa_runtime::Console;
use rand::Rng;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Trace, Finalize, JsData)]
struct RustData(Rc<str>);

impl Class for RustData {
    const NAME: &'static str = "RustData";
    const LENGTH: usize = 1;

    fn data_constructor(
        _new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<Self> {
        let kind = args[0].to_string(context)?;
        let s = kind.to_std_string().unwrap();
        Ok(RustData(s.into()))
    }

    fn init(class: &mut ClassBuilder) -> JsResult<()> {
        class.method(
            js_string!("toString"),
            0,
            NativeFunction::from_fn_ptr(|this, _args, _ctx| {
                let this = this.as_object().unwrap();
                let this = this.downcast_ref::<Self>().unwrap();
                Ok(JsString::from(&*this.0).into())
            }),
        );

        class.method(
            js_string!("lt"),
            1,
            NativeFunction::from_fn_ptr(|this, args, _ctx| {
                let this = this.as_object().unwrap();
                let this = this.downcast_ref::<Self>().unwrap();
                let other = args[0].as_object().unwrap();
                let other = other.downcast_ref::<Self>().unwrap();
                Ok((*this < *other).into())
            }),
        );

        Ok(())
    }
}

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> JsResult<()> {
    let mut context = Context::default();

    let console = Console::init(&mut context);
    context.register_global_property(js_string!(Console::NAME), console, Attribute::all())?;

    context.register_global_class::<RustData>()?;
    context.register_global_builtin_callable(
        js_string!("rand"),
        1,
        NativeFunction::from_fn_ptr(|_this, args, ctx| {
            let n = args[0].to_u32(ctx)?;
            Ok(rand::thread_rng().gen_range(0..n).into())
        }),
    )?;

    let source = Source::from_bytes(include_str!("../scripts/sort_userdata.js"));
    context.eval(source)?;

    let globals = context.global_object();
    let bench_val = globals.get(js_string!("bench"), &mut context)?;
    let bench_fn = bench_val.as_callable().unwrap();

    run(&mut || {
        bench_fn.call(&bench_val, &[], &mut context).unwrap();
    });

    Ok(())
}
