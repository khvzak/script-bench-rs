use std::rc::Rc;

use koto::{derive::*, prelude::*, Result};

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
    fn display(&self, ctx: &mut DisplayContext) -> Result<()> {
        ctx.append(self.0.to_string());
        Ok(())
    }

    fn less(&self, rhs: &KValue) -> Result<bool> {
        if let KValue::Object(kobj) = rhs {
            let rhs_dc = kobj.cast::<RustData>()?;
            Ok(*self < *rhs_dc)
        } else {
            type_error("RustData object", rhs)
        }
    }
}

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> Result<()> {
    let mut engine = Koto::default();
    
    engine.prelude().add_fn("RustData_new", |ctx| match ctx.args() {
        [KValue::Str(input)] => {
            Ok(RustData::new_koto_object(input.as_str()).into())
        }
        unexpected => type_error_with_slice("a string", unexpected),
    });

    engine.prelude().add_fn("rand", |ctx| match ctx.args() {
        [KValue::Number(n)] => {
            let res = rand::random::<u32>() as i64 % (n.as_i64());
            Ok(KNumber::from(res).into())
        }
        unexpected => type_error_with_slice("a number", unexpected),
    });

    let script = include_str!("../scripts/sort_userdata.koto");
    let _chunk = engine.compile(script)?;

    run(&mut || {
        engine.run().unwrap();
    });

    Ok(())
}
