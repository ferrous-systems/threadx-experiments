# Rust on Eclipse ThreadX Demo

This repository shows how to compile a Rust application which runs on the [Eclipse
ThreadX](https://projects.eclipse.org/projects/iot.threadx) RTOS.

Note this project uses Git submodules, so you should first run:

```bash
git submodule update --init
```

## Components

Thus repository contains:

* [`nrf52-app`](./nrf52-app/) - a Rust application for the nRF52 which uses ThreadX as its kernel; it automatically compiles ThreadX to a static library and links to it. You need an nRF52840-DK board to run this binary.
* [`qemu-cortex-r5-app`](./qemu-cortex-r5-app/) - a Rust application for the Arm Versatile Application Baseboard which uses ThreadX as its kernel; it automatically compiles ThreadX to a static library and links to it. You can use `qemu-system-arm` to run this binary in an emulated version of the Arm Versatile Application Board.
* [`threadx-sys`](./threadx-sys/) - a library crate that uses [`bindgen`] to automatically generate bindings to the ThreadX APIs.
* [`threadx`](./threadx) - a git submodule pointing at <https://github.com/eclipse-threadx/threadx.git>, unmodified.
* [`LICENSES`](./LICENSES/) - collection of license texts covering the licences used by every file in this repository (excluding any git submodules), for compliance with [Reuse].

[`bindgen`]: https://crates.io/crates/bindgen
[Reuse]: https://reuse.software

## Licence

* Copyright (c) 2025 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
