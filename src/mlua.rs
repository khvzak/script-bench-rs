use std::rc::Rc;

use mlua::{
    Function, Lua, MetaMethod, Result, String as LuaString, UserData, UserDataMethods, UserDataRef,
};
use rand::Rng;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Rc<str>);

impl UserData for RustData {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, s: LuaString| Ok(RustData((*s.to_str()?).into())));
        methods.add_meta_method(MetaMethod::Lt, |_, this, rhs: UserDataRef<Self>| {
            Ok(this < &rhs)
        });
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.0.to_string()));
    }
}

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> Result<()> {
    let lua = Lua::new();

    let globals = lua.globals();
    globals.set("RustData", lua.create_proxy::<RustData>()?)?;
    globals.set(
        "rand",
        Function::wrap(|n: u32| Ok(rand::thread_rng().gen_range(0..n))),
    )?;

    #[cfg(feature = "mlua_luau")]
    {
        lua.sandbox(true)?;
        lua.set_compiler(mlua::Compiler::new().set_optimization_level(2));
    }

    lua.load(include_str!("../scripts/sort_userdata.lua"))
        .exec()?;

    let func = lua.globals().get::<Function>("bench")?;

    run(&mut || func.call::<()>(()).unwrap());

    Ok(())
}
