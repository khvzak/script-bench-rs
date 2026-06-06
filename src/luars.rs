use std::rc::Rc;

use luars::{lua_methods, Function, Lua, LuaResult, LuaUserData, SafeOption, Stdlib, Table};
use rand::Rng;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, LuaUserData)]
#[lua_impl(Display, PartialEq, PartialOrd)]
pub struct RustData(Rc<str>);

#[lua_methods]
impl RustData {
    pub fn new(s: &str) -> Self {
        Self(Rc::from(s))
    }
}

impl std::fmt::Display for RustData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn sort_userdata(
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(Table),
) -> LuaResult<()> {
    let mut lua = Lua::new(SafeOption::default());
    lua.open_stdlib(Stdlib::All)?;
    lua.register_type_of::<RustData>("RustData")?;
    lua.register_function("rand", |n: u32| rand::rng().random_range(0..n))?;

    lua.load(include_str!("../scripts/sort_userdata.lua"))
        .exec()?;

    let func = lua.globals().get::<Function>("bench")?;

    validate(func.call(())?);
    run(&mut || func.call::<(), ()>(()).unwrap());

    Ok(())
}
