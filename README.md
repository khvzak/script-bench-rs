# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [mlua](https://crates.io/crates/mlua) (Lua 5.4 and Luau)
- [rlua](https://crates.io/crates/rlua) (Lua 5.4)
- [rhai](https://crates.io/crates/rhai)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run the `bench.py` script to generate images. It requires `cargo criterion` and `python3-matplotlib` package installed.

## Environment

|       |                               |
|-------|-------------------------------|
| OS    | macOS 12.4, Core i9-9880H     |
| mlua  | v0.8.1 (git)                  |
| rlua  | v0.19.2                       |
| rhai  | v1.8.0                        |
| rustc | v1.62.1                       |

## Results

![Sorting userdata objects](Sort%20userdata.png)
