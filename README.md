# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [boa](https://boajs.dev)
- [koto](https://crates.io/crates/koto)
- [mlua](https://crates.io/crates/mlua) (Lua 5.4 and Luau)
- [rhai](https://crates.io/crates/rhai)
- [rquickjs](https://crates.io/crates/rquickjs)
- [rune](https://crates.io/crates/rune)
- [wasmi](https://crates.io/crates/wasmi)
- [wasmtime](https://crates.io/crates/wasmtime)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run the `bench.py` script to generate images. It requires `cargo criterion` and `python3-matplotlib` package installed.

You also must have `wasm32-unknown-unknown` target installed for webassembly benchmarks.

## Environment

|          |                               |
|----------|-------------------------------|
| OS       | Ubuntu 22.04, m6i.16xlarge    |
| rustc    | v1.86.0                       |
| boa      | v0.20.0                       |
| koto     | v0.15.3                       |
| mlua     | v0.10.5                       |
| rhai     | v1.22.2                       |
| rquickjs | v0.9.0                        |
| rune     | v0.13.4                       |
| wasmi    | v0.47.0                       |
| wasmtime | v33.0.0                       |

## Results

![Sort Rust objects](Sort%20Rust%20objects.png)

Rev 1744324713
