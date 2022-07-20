use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use mlua::{Lua, MetaMethod, UserData, UserDataMethods};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Arc<String>);

impl UserData for RustData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |_, s: String| Ok(RustData(Arc::new(s))));
        methods.add_meta_method(MetaMethod::Lt, |_, this, rhs: Self| Ok(this < &rhs));
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.0.to_string()));
    }
}

fn benchmark(c: &mut Criterion) {
    let lua = Lua::new();

    let globals = lua.globals();
    globals
        .set("RustData", lua.create_proxy::<RustData>().unwrap())
        .unwrap();
    globals
        .set(
            "rand",
            lua.create_function(|_, n: u32| Ok(rand::random::<u32>() % n))
                .unwrap(),
        )
        .unwrap();

    #[cfg(feature = "mlua_luau")]
    lua.sandbox(true).unwrap();

    let f = lua
        .load(include_str!("sort_userdata.lua"))
        .into_function()
        .unwrap();

    c.bench_function("Sort userdata", |b| {
        b.iter(|| {
            f.call::<_, ()>(()).unwrap();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}

criterion_main!(benches);
