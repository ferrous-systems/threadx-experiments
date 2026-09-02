# Rust on Eclipse ThreadX Demo for the ST Micro NUCLEO-U5A5ZJ-Q, in Nonsecure State

This example program shows how to compile a Rust application which runs on the [Eclipse
ThreadX](https://projects.eclipse.org/projects/iot.threadx) RTOS.

This application is for the [NUCLEO-U5A5ZJ-Q], which includes an Arm Cortex-M33
processor. This board was chosen because it is used in Ferrous System's Rust
Trainings, is inexpensive, and includes a ST-Link on-board.

## Pre-requisites

To build this demo you must:

1. Run `git submodule update --init` to check-out the ThreadX source code
2. Install `probe-rs` (see <https://probe.rs>)
3. Install `arm-none-eabi-gcc` - such as from the [Arm GNU Toolchain](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain) or via `winget install gcc-arm-embedded`
4. Install `libclang`, as specified in [the `bindgen` documentation](https://rust-lang.github.io/rust-bindgen/requirements.html) or via `winget install LLVM.LLVM`

This demo was tested on Ferrocene 26.05 and Rust 1.94. It may work with earlier versions.

## Building

Once you have the pre-requisites, to build and run this project, simply run:

```bash
cargo run --release
```

You must have `probe-rs` in your system's PATH. See <https://probe.rs> for more
details and installation instructions. You will also need `arm-none-eabi-gcc` in
your system's PATH, so this project can automatically compile ThreadX (which it
looks for in `../threadx`).

If you have Ferrocene available, you can also do:

```bash
criticalup install
criticalup run cargo run --release
```

Either way, you will see something like:

```console
$ cargo run --release
   Compiling nrf52-app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/threadx-experiments/stm32u5-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 4.81s
     Running `probe-rs run --chip STM32U5A5ZJ --allow-erase-all --log-format=oneline target/thumbv7em-none-eabihf/debug/stm32u5-app`
      Erasing ✔ 100% [####################]  12.00 KiB @  21.46 KiB/s (took 1s)
  Programming ✔ 100% [####################]  12.00 KiB @  15.23 KiB/s (took 1s)
     Finished in 1.46s
[INFO ] Hello, this is version unknown! (stm32u5_app stm32u5-app/src/main.rs:171)
[INFO ] Entering ThreadX kernel... (stm32u5_app stm32u5-app/src/main.rs:202)
[DEBUG] In tx_application_define()... (stm32u5_app stm32u5-app/src/main.rs:35)
[DEBUG] Stack allocated @ 0x200d01bc (stm32u5_app stm32u5-app/src/main.rs:68)
[DEBUG] Thread spawned (entry=12345678) @ 0x200d003c (stm32u5_app stm32u5-app/src/main.rs:94)
[DEBUG] Stack allocated @ 0x200d41c4 (stm32u5_app stm32u5-app/src/main.rs:111)
[DEBUG] Thread spawned entry=aabbccdd @ 0x200d00f8 (stm32u5_app stm32u5-app/src/main.rs:137)
[INFO ] Starting my_thread(12345678) (stm32u5_app stm32u5-app/src/main.rs:146)
[DEBUG] my_thread(12345678) is sleeping... (stm32u5_app stm32u5-app/src/main.rs:152)
[INFO ] Starting my_thread(aabbccdd) (stm32u5_app stm32u5-app/src/main.rs:146)
[DEBUG] my_thread(aabbccdd) is sleeping... (stm32u5_app stm32u5-app/src/main.rs:152)
[INFO ] my_thread(12345678) says count = 1 (stm32u5_app stm32u5-app/src/main.rs:158)
[DEBUG] my_thread(12345678) is sleeping... (stm32u5_app stm32u5-app/src/main.rs:152)
...
```

Console output appears through `defmt`, Ferrous System's *deferred formatting*
logging mechanism, which is natively understood by `probe-rs`.

ThreadX is automatically compiled from source thanks to the
[`build.rs`](./build.rs) script that this package includes. Refer to that file
if you wish to adjust which ThreadX components are compiled in.

[NUCLEO-U5A5ZJ-Q]: https://www.st.com/en/evaluation-tools/nucleo-u5a5zj-q.html

## Licence

* Copyright (c) 2026 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
