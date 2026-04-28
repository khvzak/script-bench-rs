# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [boa](https://boajs.dev)
- [koto](https://crates.io/crates/koto)
- [mlua](https://crates.io/crates/mlua) (Lua 5.5, LuaJIT, Luau and Luau JIT)
- [rhai](https://crates.io/crates/rhai)
- [roto](https://crates.io/crates/roto)
- [rquickjs](https://crates.io/crates/rquickjs)
- [wasmi](https://crates.io/crates/wasmi)
- [wasmtime](https://crates.io/crates/wasmtime)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run `uv run bench.py` to generate images. It requires `cargo criterion` and [uv](https://docs.astral.sh/uv/) installed (dependencies are declared inline in the script).

You also must have `wasm32-unknown-unknown` target installed for webassembly benchmarks.

## Environment

| Component | Version |
|-----------|---------|
| OS | macOS 27.0, Apple M5 Max |
| rustc | v1.99.0 |
| boa | v0.22.0 |
| koto | v0.16.1 |
| mlua | v0.12.2 |
| rhai | v1.26.1 |
| roto | v0.12.0 |
| rquickjs | v0.14.0 |
| wasmi | v2.0.0 |
| wasmtime | v49.0.2 |

## Results

![Sort Rust objects](Sort%20Rust%20objects.png)

Rev 1791068096
