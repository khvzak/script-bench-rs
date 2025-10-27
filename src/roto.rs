use std::{cell::RefCell, rc::Rc, sync::Arc};

use rand::Rng;
use roto::{library, Runtime, Val};

#[derive(Clone, Default)]
pub struct RustData(pub Arc<str>);

#[derive(Clone)]
pub struct List(pub Rc<RefCell<Vec<Val<RustData>>>>);

pub fn sort_userdata(
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(Val<List>),
) -> anyhow::Result<()> {
    let lib = library! {
        fn rand(n: i64) -> i64 {
            rand::rng().random_range(0..n)
        }

        fn string_get(charset: Arc<str>, idx: i64) -> Arc<str> {
            charset[idx as usize..idx as usize + 1].into()
        }

        fn string_len(s: Arc<str>) -> i64 {
            s.len() as i64
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

    let runtime = Runtime::from_lib(lib)?;
    let mut compiled = runtime.compile("scripts/sort_userdata.roto")?;

    let func = compiled.get_function::<(), fn() -> Val<List>>("bench")?;

    validate(func.call(&mut ()));
    run(&mut || {
        func.call(&mut ());
    });

    Ok(())
}
