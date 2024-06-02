# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [boa](https://boajs.dev)
- [mlua](https://crates.io/crates/mlua) (Lua 5.4 and Luau)
- [rhai](https://crates.io/crates/rhai)
- [rquickjs](https://crates.io/crates/rquickjs)
- [rune](https://crates.io/crates/rune)
- [koto](https://crates.io/crates/koto)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run the `bench.py` script to generate images. It requires `cargo criterion` and `python3-matplotlib` package installed.

## Environment

|          |                               |
|----------|-------------------------------|
| OS       | Pop!_OS 22.04 x86_64          |
| boa      | v0.18.0                       |
| mlua     | v0.9.6                        |
| rhai     | v1.17.1                       |
| rquickjs | v0.6.2                        |
| rune     | v0.13.2                       |
| koto     | v0.14.0                       |
| rustc    | v1.78.0                       |

## Results

![Sort Rust objects](Sort%20Rust%20objects.png)
