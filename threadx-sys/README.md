# `threadx-sys` - Rust bindings for ThreadX

This Rust library uses [`bindgen`] to process the ThreadX header files into Rust
source code. You can simply include the library in your Rust project and use the
APIs - the [`build.rs`](./build.rs) file in this library will handle the
conversion from C to Rust automatically.

The `build.rs` feeds [`bindgen`] the file [`wrapper.h`](./wrapper.h). This file
then `#include`s the relevant parts of the ThreadX that we wish to convert. The
`build.rs` file also applies a filter so that only *useful* symbols from the
header file are exported.

Unfortunately [`bindgen`] cannot convert some of the ThreadX constants, which
are provided as `#define` macros rather than `const int` values. We have
therefore manually translated some of those constants and placed them into the
[`lib.rs`](./src/lib.rs) for user convenience.

This library assumes that ThreadX is available at `../threadx`. If you wish to
use this library outside of this example repository you may wish to alter the
code to accept an environment variable that gives the path to the ThreadX source
code. It is a deliberate choice to not include the ThreadX source code as part
of this library.

[`bindgen`]: https://crates.io/crates/bindgen

## Licence

* Copyright (c) 2025 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
