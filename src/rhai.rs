use std::rc::Rc;

use itertools::Itertools;
use rand::Rng;
use rhai::{Array, Engine};

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustData(Rc<str>);

pub fn sort_userdata(
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(Array),
) -> anyhow::Result<()> {
    let mut engine = Engine::new();
    engine.set_max_call_levels(1000);
    engine.set_max_expr_depths(0, 0);

    engine
        .register_type_with_name::<RustData>("RustData")
        .register_fn("RustData_new", |s: &str| RustData(s.into()))
        .register_fn("to_string", |this: &mut RustData| this.0.to_string())
        .register_fn("<", |l: &mut RustData, r: RustData| *l < r)
        .register_fn("rand", |n: i64| rand::rng().random_range(0..n))
        .register_fn("concat", |items: Array| {
            items
                .into_iter()
                .map(|x| x.into_immutable_string().unwrap())
                .join("")
        });

    let ast = engine.compile(include_str!("../scripts/sort_userdata.rhai"))?;

    validate(engine.eval_ast::<Array>(&ast).unwrap());
    run(&mut || {
        engine.eval_ast::<Array>(&ast).unwrap();
    });

    Ok(())
}
