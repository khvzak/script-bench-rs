use std::sync::Arc;

use rand::Rng;
use wasmi::*;
use wat;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Arc<str>);

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> anyhow::Result<()> {
    let engine = Engine::default();
    let wasm = wat::parse_str(include_str!("../scripts/sort_userdata.wat"))?;
    let module = Module::new(&engine, &mut &wasm[..])?;

    type HostState = Vec<RustData>;
    let mut store = Store::new(&engine, Vec::with_capacity(10_000));
    let rustdata_new = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, HostState>, off: i32, len: i32| -> i32 {
            let buffer = {
                let mut buffer = [0u8; 24];
                let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                memory
                    .read(&caller, off as usize, &mut buffer[..len as usize])
                    .unwrap();
                buffer
            };
            let id = caller.data().len() as i32;
            caller.data_mut().push(RustData(
                std::str::from_utf8(&buffer[..len as usize]).unwrap().into(),
            ));
            id
        },
    );
    let rustdata_lt = Func::wrap(
        &mut store,
        |caller: Caller<'_, HostState>, i: i32, j: i32| -> i32 {
            let data = caller.data();
            (data[i as usize] < data[j as usize]) as i32
        },
    );
    let rustdata_clear = Func::wrap(&mut store, |mut caller: Caller<'_, HostState>| {
        caller.data_mut().clear();
    });

    let mut linker = <Linker<HostState>>::new(&engine);
    linker
        .define("RustData", "new", rustdata_new)?
        .define("RustData", "lt", rustdata_lt)?
        .func_wrap("RustData", "rand", |n: i32| {
            rand::thread_rng().gen_range(0..n)
        })?
        .define("RustData", "clear", rustdata_clear)?;

    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let bench = instance.get_typed_func::<(), ()>(&store, "bench")?;

    run(&mut || bench.call(&mut store, ()).unwrap());

    Ok(())
}
