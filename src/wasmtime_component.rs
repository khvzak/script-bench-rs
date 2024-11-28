use std::sync::Arc;

use rand::Rng;
use wasmtime::component::{bindgen, Component, Resource, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({with: {"component:bench/types/rust-data": RustData}});

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustData(Arc<str>);

struct HostState(ResourceTable, WasiCtx);

impl BenchImports for HostState {
    fn rand(&mut self, n: u32) -> u32 {
        rand::thread_rng().gen_range(0..n)
    }
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.1
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.0
    }
}

impl component::bench::types::Host for HostState {}

impl component::bench::types::HostRustData for HostState {
    fn new(&mut self, s: String) -> Resource<RustData> {
        self.0.push(RustData(s.into())).unwrap()
    }
    fn lt(&mut self, lhs: Resource<RustData>, rhs: Resource<RustData>) -> bool {
        self.0.get(&lhs).unwrap() < self.0.get(&rhs).unwrap()
    }
    fn drop(&mut self, rd: Resource<RustData>) -> wasmtime::Result<()> {
        self.0.delete(rd)?;
        Ok(())
    }
}

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> anyhow::Result<()> {
    let engine = Engine::default();
    let wasm = include_bytes!("../bench/target/wasm32-wasip2/release/bench.wasm");
    let component = Component::new(&engine, wasm)?;
    let mut linker = wasmtime::component::Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let mut builder = WasiCtxBuilder::new();
    let mut store = Store::new(&engine, HostState(ResourceTable::new(), builder.build()));
    Bench::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    let bindings = Bench::instantiate(&mut store, &component, &linker)?;

    run(&mut || bindings.call_bench(&mut store).unwrap());

    Ok(())
}
