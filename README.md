# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [boa](https://boajs.dev)
- [koto](https://crates.io/crates/koto)
- [mlua](https://crates.io/crates/mlua) (Lua 5.4 and Luau)
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

|          |                               |
|----------|-------------------------------|
| OS       | MacOS 26.5 M5 Max             |
| rustc    | v1.96.0                       |
| boa      | v0.21.1                       |
| koto     | v0.16.1                       |
| mlua     | v0.12-rc.2                    |
| rhai     | v1.25.1                       |
| roto     | v0.11.0                       |
| rquickjs | v0.12.0                       |
| wasmi    | v1.0.9                        |
| wasmtime | v45.0.1                       |

## Results

![Sort Rust objects](Sort%20Rust%20objects.png)

Rev 1761581592
