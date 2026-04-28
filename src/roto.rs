use std::{cell::RefCell, rc::Rc, sync::Arc};

use rand::RngExt;
use roto::{library, Runtime, Val};

#[derive(Clone, Default)]
pub struct RustData(pub Arc<str>);

#[derive(Clone)]
pub struct RustList(pub Rc<RefCell<Vec<Val<RustData>>>>);

pub fn sort_userdata(
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(Val<RustList>),
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

        #[clone] type RustList = Val<RustList>;

        impl Val<RustList> {
            fn new() -> Val<RustList> {
                Val(RustList(Rc::new(RefCell::new(Vec::new()))))
            }

            fn push(this: Val<RustList>, rd: Val<RustData>) {
                this.0.0.borrow_mut().push(rd);
            }

            fn get(this: Val<RustList>, idx: i64) -> Val<RustData> {
                this.0.0.borrow().get(idx as usize).cloned().expect("get valid list idx")
            }

            fn len(this: Val<RustList>) -> i64 {
                this.0.0.borrow().len() as i64
            }

            fn swap(self, i: i64, j: i64) {
                self.0.0.borrow_mut().swap(i as usize, j as usize)
            }
        }
    };

    let runtime = Runtime::from_lib(lib)?;
    let mut compiled = runtime.compile("scripts/sort_userdata.roto")?;

    let func = compiled.get_function::<fn() -> Val<RustList>>("bench")?;

    validate(func.call());
    run(&mut || {
        func.call();
    });

    Ok(())
}
