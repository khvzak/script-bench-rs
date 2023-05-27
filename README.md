# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [mlua](https://crates.io/crates/mlua) (Lua 5.4 and Luau)
- [rlua](https://crates.io/crates/rlua) (Lua 5.4)
- [rhai](https://crates.io/crates/rhai)
- [rune](https://crates.io/crates/rune)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run the `bench.py` script to generate images. It requires `cargo criterion` and `python3-matplotlib` package installed.

## Environment

|       |                               |
|-------|-------------------------------|
| OS    | macOS 13.3, Core i9-9880H     |
| mlua  | v0.9.0-beta.2                 |
| rlua  | v0.19.4                       |
| rhai  | v1.14.0                       |
| rune  | v0.12.3                       |
| rustc | v1.69.0                       |

## Results

![Sorting userdata objects](Sort%20userdata.png)
