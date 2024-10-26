# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [boa](https://boajs.dev)
- [mlua](https://crates.io/crates/mlua) (Lua 5.4 and Luau)
- [rhai](https://crates.io/crates/rhai)
- [rquickjs](https://crates.io/crates/rquickjs)
- [rune](https://crates.io/crates/rune)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run the `bench.py` script to generate images. It requires `cargo criterion` and `python3-matplotlib` package installed.

## Environment

|          |                               |
|----------|-------------------------------|
| OS       | Ubuntu 23.04, r6i.8xlarge     |
| boa      | v0.19.1                       |
| mlua     | v0.10.0                       |
| rhai     | v1.19.0                       |
| rquickjs | v0.6.2                        |
| rune     | v0.13.4                       |
| rustc    | v1.80.1                       |

## Results

![Sort Rust objects](Sort%20Rust%20objects.png)
