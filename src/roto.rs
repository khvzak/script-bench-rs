use std::{cell::RefCell, rc::Rc, sync::Arc};

use rand::Rng;
use roto::{library, Runtime, Val};

#[derive(Clone, Default)]
pub struct RustData(pub Arc<str>);

#[derive(Clone)]
pub struct List(pub Rc<RefCell<Vec<Val<RustData>>>>);

#[derive(Clone)]
struct Array(Arc<Vec<Arc<str>>>);

macro_rules! generate_charset {
    ($($c:expr),+) => {
        vec![
            $(
                Arc::from($c),
            )+
        ]
    };
}

pub fn sort_userdata(
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(Val<List>),
) -> anyhow::Result<()> {
    let lib = library! {
        fn rand(n: i64) -> i64 {
            rand::rng().random_range(0..n)
        }

        fn to_f64(n: i64) -> f64 {
            n as f64
        }

        fn to_i64(n: f64) -> i64 {
            n.floor() as i64
        }

        #[clone] type RustData = Val<RustData>;

        impl Val<RustData> {
            fn new(s: Arc<str>) -> Val<RustData> {
                Val(RustData(s))
            }

            fn lt(this: Val<RustData>, rhs: Val<RustData>) -> bool {
                this.0.0 < rhs.0.0
            }
        }

        const CHARSET: Val<Array> = Val(Array(Arc::new(generate_charset!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"])));

        #[clone] type Array = Val<Array>;

        impl Val<Array> {
            fn get(this: Val<Array>, idx: i64) -> Arc<str> {
                this.0.0.get(idx as usize).cloned().expect("valid array idx")
            }

            fn len(this: Val<Array>) -> i64 {
                this.0.0.len() as i64
            }
        }

        #[clone] type List = Val<List>;

        impl Val<List> {
            fn new() -> Val<List> {
                Val(List(Rc::new(RefCell::new(Vec::new()))))
            }

            fn push(this: Val<List>, rd: Val<RustData>) {
                this.0.0.borrow_mut().push(rd);
            }

            fn get(this: Val<List>, idx: i64) -> Val<RustData> {
                this.0.0.borrow().get(idx as usize).cloned().expect("get valid list idx")
            }

            fn set(this: Val<List>, idx: i64, val: Val<RustData>) {
                *this.0.0.borrow_mut().get_mut(idx as usize).expect("set valid list idx") = val;
            }

            fn len(this: Val<List>) -> i64 {
                this.0.0.borrow().len() as i64
            }
        }
    };

    let runtime = Runtime::from_lib(lib).map_err(|e| anyhow::anyhow!(e))?;
    let mut compiled = runtime.compile("scripts/sort_userdata.roto")?;

    let func = compiled
        .get_function::<(), fn() -> Val<List>>("bench")
        .map_err(|e| anyhow::anyhow!(e))?;

    validate(func.call(&mut ()));
    run(&mut || {
        func.call(&mut ());
    });

    Ok(())
}
