use std::sync::Arc;

use rand::Rng;
#[cfg(feature = "wasmi")]
use wasmi::*;
#[cfg(feature = "wasmtime")]
use wasmtime::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Arc<str>);

type HostState = Vec<RustData>;

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> anyhow::Result<()> {
    let engine = Engine::default();
    let wasm = include_bytes!("../scripts/sort_userdata.wasm");
    let module = Module::new(&engine, wasm)?;

    let mut store = Store::new(&engine, Vec::new());
    let rustdata_new = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, HostState>, off: u32, len: u32| -> u32 {
            let buffer = {
                let mut buffer = vec![0; len as usize];
                let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                memory.read(&caller, off as usize, &mut buffer).unwrap();
                buffer
            };
            caller
                .data_mut()
                .push(RustData(std::str::from_utf8(&buffer).unwrap().into()));
            caller.data().len() as u32 - 1
        },
    );
    let rustdata_lt = Func::wrap(
        &mut store,
        |caller: Caller<'_, HostState>, i: u32, j: u32| -> u32 {
            let data = caller.data();
            (data[i as usize] < data[j as usize]) as u32
        },
    );

    let mut linker = <Linker<HostState>>::new(&engine);
    linker.func_wrap("RustData", "rand", |n: u32| {
        rand::thread_rng().gen_range(0..n)
    })?;
    #[cfg(feature = "wasmtime")]
    {
        linker
            .define(&store, "RustData", "rustdata_new", rustdata_new)?
            .define(&store, "RustData", "rustdata_lt", rustdata_lt)?;
    }
    #[cfg(feature = "wasmi")]
    {
        linker
            .define("RustData", "rustdata_new", rustdata_new)?
            .define("RustData", "rustdata_lt", rustdata_lt)?;
    }

    let instance = linker.instantiate(&mut store, &module)?;
    #[cfg(feature = "wasmi")]
    let instance = instance.start(&mut store)?;
    let bench = instance.get_typed_func::<(), ()>(&mut store, "bench")?;

    run(&mut || {
        bench.call(&mut store, ()).unwrap();
        store.data_mut().clear();
    });

    Ok(())
}
