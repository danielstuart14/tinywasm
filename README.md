<div>
    <div>
        <a href=""><img align="left" src="./tinywasm.png" width="100px"></a>
    </div>
    <h1>TinyWasm</h1>
    A tiny WebAssembly Runtime written in Rust
</div>

<br>

[![docs.rs](https://img.shields.io/docsrs/tinywasm?logo=rust)](https://docs.rs/tinywasm) [![Crates.io](https://img.shields.io/crates/v/tinywasm.svg?logo=rust)](https://crates.io/crates/tinywasm) [![Crates.io](https://img.shields.io/crates/l/tinywasm.svg)](./LICENSE-APACHE)

# Status

TinyWasm, starting from upcoming version `0.3.0`, passes all the WebAssembly 1.0 tests in the [WebAssembly Test Suite](https://github.com/WebAssembly/testsuite). The 2.0 tests are in progress (notably `simd` and `bulk-memory-operations` are not implemented yet).

Some APIs to interact with the runtime are not yet exposed, and the existing ones are subject to change, but the core functionality is mostly complete.
Results of the tests can be found [here](https://github.com/explodingcamera/tinywasm/tree/main/crates/tinywasm/tests/generated).

TinyWasm is not designed for performance, but rather for size and portability. However, it is still reasonably fast.
There are a couple of low-hanging fruits on the performance side, but they are not a priority at the moment.

## Supported Proposals

- [**Mutable Globals**](https://github.com/WebAssembly/mutable-global/blob/master/proposals/mutable-global/Overview.md) - **Fully implemented**
- [**Multi-value**](https://github.com/WebAssembly/spec/blob/master/proposals/multi-value/Overview.md) - **Fully implemented**
- [**Sign-extension operators**](https://github.com/WebAssembly/spec/blob/master/proposals/sign-extension-ops/Overview.md) - **Fully implemented**
- [**Reference Types**](https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md) - **_Partially implemented_**
- [**Multiple Memories**](https://github.com/WebAssembly/multi-memory/blob/master/proposals/multi-memory/Overview.md) - **_Partially implemented_** (not tested yet)
- [**Memory64**](https://github.com/WebAssembly/memory64/blob/master/proposals/memory64/Overview.md) - **_Partially implemented_** (only 32-bit addressing is supported at the moment, but larger memories can be created)

## Usage

TinyWasm can be used through the `tinywasm-cli` CLI tool or as a library in your Rust project. Documentation can be found [here](https://docs.rs/tinywasm).

### CLI

```sh
$ cargo install tinywasm-cli
$ tinywasm-cli --help
```

### Library

```sh
$ cargo add tinywasm
```

## Feature Flags

- **`std`**\
  Enables the use of `std` and `std::io` for parsing from files and streams. This is enabled by default.
- **`logging`**\
  Enables logging using the `log` crate. This is enabled by default.
- **`parser`**\
  Enables the `tinywasm-parser` crate. This is enabled by default.

With all these features disabled, TinyWasm only depends on `core`, `alloc` and `libm` and can be used in `no_std` environments.

<!-- # 🎯 Goals

- Self-hosted (can run itself compiled to WebAssembly)
- No unsafe code
- Works on `no_std` (with `alloc` the feature and nightly compiler)
- Fully support WebAssembly MVP (1.0)
- Low Memory Usage (less than 10kb)
- Fast Startup Time
- Preemptive multitasking support -->

## Performance

> Benchmarks are coming soon.

# 📄 License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in TinyWasm by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

**Note:** The GitHub repository contains a Submodule (`crates/tinywasm-parser/data`) which is licensed only under the [Apache License, Version 2.0](https://github.com/WebAssembly/spec/blob/main/test/LICENSE). This data is generated from the [WebAssembly Specification](https://github.com/WebAssembly/spec/tree/main/test) and is only used for testing purposes and not included in the final binary.
