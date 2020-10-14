# About

This repo is part of a larger project called Audio Anywhere (AA). Audio Anywhere a 
framework for work-ing with audio plugins that are compiled once and run anywhere.
At the heart of Audio Anywhere is an audio engine whose Digital Signal Processing (DSP) components are written in Faust and deployed with WebAssembly. 

Details about the project can be found on the [project's homepage](https://muses-dmi.github.io/projects/).

## Introduction

C bindings for [AA Wasmtime](https://github.com/bgaster/aa_wasmtime).

## Building

It is implemented using stable [Rust](https://www.rust-lang.org/).

To install Rust go you need simply to install [Rustup](https://rustup.rs/) and 
if you already have Rust installed, then you can update with the command rustup update.

You can build with the following cargo command:

```bash
cargo build --release
```
This produces:

 * aa_wasmtime_c.h
 * target/release/libaa_wasmtime_c.a

# License
© 2020 [Benedict R. Gaster (cuberoo_)](https://bgaster.github.io/)

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
