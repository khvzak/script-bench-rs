use std::rc::Rc;

use rand::Rng;
#[cfg(feature = "wasmi")]
use wasmi::*;
#[cfg(feature = "wasmtime")]
use wasmtime::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Rc<str>);

#[derive(Default)]
struct HostState {
    store: Vec<Entry>,
    free: Option<u32>,
    memory: Option<Memory>,
}

enum Entry {
    Free { next: Option<u32> },
    Occupied { rd: RustData },
}

impl HostState {
    fn alloc(&mut self, rd: RustData) -> u32 {
        if let Some(id) = self.free {
            let entry = &mut self.store[id as usize];
            let Entry::Free { next } = entry else {
                panic!("corruption");
            };
            self.free = *next;
            *entry = Entry::Occupied { rd };
            return id;
        }
        self.store.push(Entry::Occupied { rd });
        self.store.len() as u32 - 1
    }

    fn get(&self, id: u32) -> &RustData {
        match &self.store[id as usize] {
            Entry::Free { .. } => panic!("use after free"),
            Entry::Occupied { rd } => rd,
        }
    }

    fn dealloc(&mut self, id: u32) {
        let entry = &mut self.store[id as usize];
        let Entry::Occupied { .. } = entry else {
            panic!("double free");
        };
        *entry = Entry::Free { next: self.free };
        self.free = Some(id);
    }
}

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> anyhow::Result<()> {
    let engine = Engine::default();
    let wasm = include_bytes!("../scripts/sort_userdata.wasm");
    let module = Module::new(&engine, wasm)?;

    let mut store = Store::new(&engine, Default::default());
    let rustdata_new = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, HostState>, off: u32, len: u32| -> u32 {
            let buffer =
                &caller.data().memory.unwrap().data(&mut caller)[off as usize..][..len as usize];
            let rd = RustData(std::str::from_utf8(buffer).unwrap().into());
            caller.data_mut().alloc(rd)
        },
    );
    let rustdata_delete = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, HostState>, id: u32| -> () {
            caller.data_mut().dealloc(id);
        },
    );
    let rustdata_lt = Func::wrap(
        &mut store,
        |caller: Caller<'_, HostState>, i: u32, j: u32| -> u32 {
            let data = caller.data();
            (data.get(i) < data.get(j)) as u32
        },
    );

    let mut linker = <Linker<HostState>>::new(&engine);
    linker.func_wrap("RustData", "rand", |n: u32| rand::rng().random_range(0..n))?;
    #[cfg(feature = "wasmtime")]
    {
        linker
            .define(&store, "RustData", "rustdata_new", rustdata_new)?
            .define(&store, "RustData", "rustdata_delete", rustdata_delete)?
            .define(&store, "RustData", "rustdata_lt", rustdata_lt)?;
    }
    #[cfg(feature = "wasmi")]
    {
        linker
            .define("RustData", "rustdata_new", rustdata_new)?
            .define("RustData", "rustdata_delete", rustdata_delete)?
            .define("RustData", "rustdata_lt", rustdata_lt)?;
    }

    let instance = linker.instantiate(&mut store, &module)?;
    #[cfg(feature = "wasmi")]
    let instance = instance.start(&mut store)?;
    store.data_mut().memory = instance.get_memory(&mut store, "memory");
    let bench = instance.get_typed_func::<(), ()>(&mut store, "bench")?;

    run(&mut || bench.call(&mut store, ()).unwrap());

    Ok(())
}
