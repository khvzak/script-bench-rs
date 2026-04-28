# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [boa](https://boajs.dev)
- [koto](https://crates.io/crates/koto)
- [mlua](https://crates.io/crates/mlua) (Lua 5.4, LuaJIT, Luau and Luau JIT)
- [rhai](https://crates.io/crates/rhai)
- [roto](https://crates.io/crates/roto)
- [rquickjs](https://crates.io/crates/rquickjs)
- [wasmi](https://crates.io/crates/wasmi)
- [wasmtime](https://crates.io/crates/wasmtime)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run the `bench.py` script to generate images. It requires `cargo criterion` and `python3-matplotlib` package installed.

You also must have `wasm32-unknown-unknown` target installed for webassembly benchmarks.

## Environment

|          |                               |
|----------|-------------------------------|
| OS       | Arch Linux x86_64             |
| rustc    | v1.95.0                       |
| boa      | v0.21.1                       |
| koto     | v0.16.1                       |
| mlua     | v0.11.6                       |
| rhai     | v1.24.0                       |
| roto     | v0.10.0                       |
| rquickjs | v0.11.0                       |
| wasmi    | v1.0.9                        |
| wasmtime | v44.0.0                       |

## Results

![Sort Rust objects](Sort%20Rust%20objects.png)

Rev 1777374104
