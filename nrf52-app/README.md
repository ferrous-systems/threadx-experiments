# Rust on Eclipse ThreadX Demo for the nRF52840-DK

This example program shows how to compile a Rust application which runs on the [Eclipse
ThreadX](https://projects.eclipse.org/projects/iot.threadx) RTOS.

This application is for the [nRF52840-DK], which includes an Arm Cortex-M4
processor. This board was chosen because it is used in Ferrous System's Rust
Trainings, is inexpensive, and includes a SEGGER J-Link on-board.

## Pre-requisites

To build this demo you must:

1. Run `git submodule update --init` to check-out the ThreadX source code
2. Install `probe-rs` (see <https://probe.rs>)
3. Install `arm-none-eabi-gcc` - such as from the [Arm GNU Toolchain](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain) or via `winget install gcc-arm-embedded`
4. Install `libclang`, as specified in [the `bindgen` documentation](https://rust-lang.github.io/rust-bindgen/requirements.html) or via `winget install LLVM.LLVM`
5. Add the `thumb7em-none-eabihf` target with `rustup`:

  ```bash
  rustup target add thumb7em-none-eabihf --toolchain=stable
  ```

This demo was tested on Ferrocene 26.02 and Rust 1.94. It may work with earlier versions.

## Building

To build and run this project, simply run:

```bash
cargo run --release
```

You must have `probe-rs` in your system's PATH. See <https://probe.rs> for more
details and installation instructions. You will also need `arm-none-eabi-gcc` in
your system's PATH, so this project can automatically compile ThreadX (which it
looks for in `../threadx`).

You will see something like:

```console
$ cargo run --release
   Compiling nrf52-app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/threadx-experiments/nrf52-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 4.81s
     Running `probe-rs run --chip nRF52840_xxAA --allow-erase-all --log-format=oneline target/thumbv7em-none-eabihf/debug/nrf52-app`
      Erasing ✔ 100% [####################]  12.00 KiB @  21.46 KiB/s (took 1s)
  Programming ✔ 100% [####################]  12.00 KiB @  15.23 KiB/s (took 1s)
     Finished in 1.46s
[WARN ] Hello, this is version unknown! (nrf52_app nrf52-app/src/main.rs:170)
[INFO ] Entering ThreadX kernel... (nrf52_app nrf52-app/src/main.rs:209)
In tx_application_define()...
[INFO ] Thread spawned (entry=12345678) @ 0x200373fc (nrf52_app nrf52-app/src/main.rs:88)
[INFO ] Thread spawned (entry=aabbccdd) @ 0x200374b4 (nrf52_app nrf52-app/src/main.rs:131)
[INFO ] I am my_thread(12345678) (nrf52_app nrf52-app/src/main.rs:139)
[INFO ] I am my_thread(aabbccdd) (nrf52_app nrf52-app/src/main.rs:139)
[INFO ] I am my_thread(12345678), count = 1 (nrf52_app nrf52-app/src/main.rs:164)
[INFO ] I am my_thread(aabbccdd), count = 1 (nrf52_app nrf52-app/src/main.rs:164)
[INFO ] I am my_thread(12345678), count = 2 (nrf52_app nrf52-app/src/main.rs:164)
...
```

Console output appears through `defmt`, Ferrous System's *deferred formatting*
logging mechanism, which is natively understood by `probe-rs`.

ThreadX is automatically compiled from source thanks to the
[`build.rs`](./build.rs) script that this package includes. Refer to that file
if you wish to adjust which ThreadX components are compiled in.

[nRF52840-DK]: https://www.nordicsemi.com/Products/Development-hardware/nRF52840-DK

## Licence

* Copyright (c) 2026 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
