use anyhow::{bail, Result};
use koto::{derive::*, prelude::*, runtime};
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, KotoCopy, KotoType)]
struct RustData(Rc<str>);

#[koto_impl]
impl RustData {
    fn new_koto_object(s: &str) -> KObject {
        let me = Self(s.into());
        KObject::from(me)
    }
}

impl KotoObject for RustData {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.0.to_string());
        Ok(())
    }

    fn less(&self, rhs: &KValue) -> runtime::Result<bool> {
        if let KValue::Object(kobj) = rhs {
            let rhs_dc = kobj.cast::<RustData>()?;
            Ok(*self < *rhs_dc)
        } else {
            unexpected_type("RustData object", rhs)
        }
    }
}

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> Result<()> {
    let mut engine = Koto::default();
    let prelude = engine.prelude();

    prelude.add_fn("RustData_new", |ctx| match ctx.args() {
        [KValue::Str(input)] => Ok(RustData::new_koto_object(input.as_str()).into()),
        unexpected => unexpected_args("a string", unexpected),
    });

    prelude.add_fn("rand", |ctx| match ctx.args() {
        [KValue::Number(n)] => {
            let res = rand::random::<u32>() as i64 % i64::from(n);
            Ok(res.into())
        }
        unexpected => unexpected_args("a number", unexpected),
    });

    engine.compile_and_run(include_str!("../scripts/sort_userdata.koto"))?;
    let Some(bench) = engine.exports().get("bench") else {
        bail!("Missing bench function");
    };

    run(&mut || {
        engine.call_function(bench.clone(), &[]).unwrap();
    });

    Ok(())
}
