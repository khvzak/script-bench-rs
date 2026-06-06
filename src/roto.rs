use rand::Rng;
use roto::{library, List, RotoString, Runtime, Val};

#[derive(Clone, Default, PartialEq)]
pub struct RustData(pub RotoString);

pub fn sort_userdata(
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(List<Val<RustData>>),
) -> anyhow::Result<()> {
    let lib = library! {
        fn rand(n: u64) -> u64 {
            rand::rng().random_range(0..n)
        }

        impl RotoString {
            fn get(self, idx: u64) -> RotoString {
                self[idx as usize..idx as usize + 1].into()
            }

            fn len(self) -> u64 {
                self.bytes().len() as u64
            }
        }

        #[clone] type RustData = Val<RustData>;

        impl Val<RustData> {
            fn new(s: RotoString) -> Val<RustData> {
                Val(RustData(s))
            }

            fn lt(this: Val<RustData>, rhs: Val<RustData>) -> bool {
                &*this.0.0 < &*rhs.0.0
            }
        }
    };

    let runtime = Runtime::from_lib(lib)?;
    let mut compiled = runtime.compile("scripts/sort_userdata.roto")?;

    let func = compiled.get_function::<fn() -> List<Val<RustData>>>("bench")?;

    validate(func.call());
    run(&mut || {
        func.call();
    });

    Ok(())
}
