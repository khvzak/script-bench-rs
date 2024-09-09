// use std::rc::Rc;

// use piccolo::{
//     Callback, CallbackReturn, Closure, Context, Executor, Function, Lua, MetaMethod, String, Table,
//     UserData,
// };
// use rand::Rng;

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
// struct RustData(Rc<str>);

// fn register_userdata(ctx: Context<'_>) {
//     let rust_data_proxy = UserData::new_static(&ctx, RustData("".into()));
//     let metatable = {
//         let metatable = Table::new(&ctx);

//         let index = Table::new(&ctx);
//         index.set_field(
//             ctx,
//             "new",
//             Callback::from_fn(&ctx, |ctx, _, mut stack| {
//                 let s = stack.consume::<String>(ctx).unwrap();
//                 let metatable = ctx.get_global("RustData__mt").unwrap();
//                 let ud = UserData::new_static(&ctx, RustData(s.to_str().unwrap().into()));
//                 ud.set_metatable(&ctx, metatable);
//                 stack.into_back(ctx, ud);
//                 Ok(CallbackReturn::Return)
//             }),
//         );
//         metatable.set(ctx, MetaMethod::Index, index).unwrap();

//         metatable
//             .set(
//                 ctx,
//                 MetaMethod::Lt,
//                 Callback::from_fn(&ctx, |ctx, _, mut stack| {
//                     let (this, rhs) = stack.consume::<(UserData, UserData)>(ctx).unwrap();
//                     let this = this.downcast_static::<RustData>().unwrap();
//                     let rhs = rhs.downcast_static::<RustData>().unwrap();
//                     stack.into_back(ctx, this < rhs);
//                     Ok(CallbackReturn::Return)
//                 }),
//             )
//             .unwrap();

//         metatable
//             .set(
//                 ctx,
//                 MetaMethod::ToString,
//                 Callback::from_fn(&ctx, |ctx, _, mut stack| {
//                     let this = stack.consume::<UserData>(ctx).unwrap();
//                     let this = this.downcast_static::<RustData>().unwrap();
//                     stack.into_back(ctx, this.0.to_string());
//                     Ok(CallbackReturn::Return)
//                 }),
//             )
//             .unwrap();

//         metatable
//     };
//     rust_data_proxy.set_metatable(&ctx, Some(metatable));
//     ctx.set_global("RustData", rust_data_proxy);
//     ctx.set_global("RustData__mt", metatable);
// }

fn main() {
    // let mut lua = Lua::full();

    // lua.enter(|ctx| {
    //     let rand = Callback::from_fn(&ctx, |ctx, _, mut stack| {
    //         let n: u32 = stack.consume(ctx).unwrap();
    //         let r = rand::thread_rng().gen_range(0..n);
    //         stack.into_back(ctx, r);
    //         Ok(CallbackReturn::Return)
    //     });
    //     ctx.set_global("rand", rand);

    //     register_userdata(ctx);
    // });

    // let executor = lua.enter(|ctx| {
    //     let closure = Closure::load(
    //         ctx,
    //         Some("sort_userdata.lua"),
    //         include_str!("../scripts/sort_userdata.lua").as_bytes(),
    //     )
    //     .unwrap();
    //     ctx.stash(Executor::start(ctx, closure.into(), ()))
    // });
    // lua.execute::<()>(&executor).unwrap();

    // let executor = lua.enter(|ctx| {
    //     let bench = ctx.get_global::<Function>("bench").unwrap();
    //     ctx.stash(Executor::start(ctx, bench, ()))
    // });
    // lua.execute::<()>(&executor).unwrap();
}
