# Rust on Eclipse ThreadX Demo for the Arm Versatile Application Board

This example program shows how to compile a Rust application which runs on the [Eclipse
ThreadX](https://projects.eclipse.org/projects/iot.threadx) RTOS.

This application is for the Arm Versatile Application Board, which includes an
Arm Cortex-R5 processor. This board was chosen because it can be emulated by
QEMU.

## Pre-requisites

To build this demo you must:

1. Run `git submodule update --init` to check-out the ThreadX source code
2. Install `qemu-run` with `cargo install qemu-run`
3. Install `qemu-system-arm` - see [the QEMU website](https://www.qemu.org/download/) or via `winget install --id=SoftwareFreedomConservancy.QEMU`
4. Install `arm-none-eabi-gcc` - such as from the [Arm GNU Toolchain](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain) or via `winget install gcc-arm-embedded`
5. Install `libclang`, as specified in [the `bindgen` documentation](https://rust-lang.github.io/rust-bindgen/requirements.html) or via `winget install LLVM.LLVM`
6. Add the `armv7r-none-eabihf` target with `rustup`:

  ```bash
  rustup target add armv7r-none-eabihf --toolchain=stable
  ```

This demo was tested on Ferrocene 26.02 and Rust 1.94. It may work with earlier versions.

## Building

Once you have the pre-requisites, to build and run this project, simply run:

```bash
cargo run --release
```

You must have `qemu-system-arm` in your system's PATH. You will also need
`arm-none-eabi-gcc` in your system's PATH, so this project can automatically
compile ThreadX (which is expected in `../threadx`). Our runner also requires `defmt-print`.

If you have Ferrocene available, you can also do:

```bash
criticalup install
criticalup run cargo run --release
```

You will see something like:

```console
$ cargo run --release
    Finished `release` profile [optimized + debuginfo] target(s) in 0.3s
     Running `qemu-run --machine=versatileab --cpu=cortex-r5f '--log-format={[{L}]%bold} {s} {({c:bold} {fff}:{l:1})%dimmed}' target/armv7r-none-eabihf/debug/qemu-cortex-r5-app`
[INFO ] Hello, this is version unknown! (qemu_cortex_r5_app src/main.rs:169)
[INFO ] In tx_application_define()... (qemu_cortex_r5_app src/main.rs:34)
[DEBUG] Stack allocated @ 0x00007dac (qemu_cortex_r5_app src/main.rs:67)
[DEBUG] Thread spawned (entry=12345678) @ 0x00007c34 (qemu_cortex_r5_app src/main.rs:93)
[DEBUG] Stack allocated @ 0x0000bdb4 (qemu_cortex_r5_app src/main.rs:110)
[DEBUG] Thread spawned entry=aabbccdd @ 0x00007cec (qemu_cortex_r5_app src/main.rs:136)
[INFO ] Starting my_thread(12345678) (qemu_cortex_r5_app src/main.rs:145)
[DEBUG] my_thread(12345678) is sleeping... (qemu_cortex_r5_app src/main.rs:150)
[INFO ] Starting my_thread(aabbccdd) (qemu_cortex_r5_app src/main.rs:145)
[DEBUG] my_thread(aabbccdd) is sleeping... (qemu_cortex_r5_app src/main.rs:150)
[INFO ] my_thread(12345678) says count = 1 (qemu_cortex_r5_app src/main.rs:156)
[DEBUG] my_thread(12345678) is sleeping... (qemu_cortex_r5_app src/main.rs:150)
[INFO ] my_thread(aabbccdd) says count = 1 (qemu_cortex_r5_app src/main.rs:156)
[DEBUG] my_thread(aabbccdd) is sleeping... (qemu_cortex_r5_app src/main.rs:150)
[INFO ] my_thread(12345678) says count = 2 (qemu_cortex_r5_app src/main.rs:156)
[DEBUG] my_thread(12345678) is sleeping... (qemu_cortex_r5_app src/main.rs:150)
[INFO ] my_thread(aabbccdd) says count = 2 (qemu_cortex_r5_app src/main.rs:156)
[DEBUG] my_thread(aabbccdd) is sleeping... (qemu_cortex_r5_app src/main.rs:150)
^Cqemu-system-arm: terminating on signal 2 from pid 56574 (<unknown process>)
------------------------------------------------------------------------
```

Press `Ctrl-C` to quit `qemu-run`.

Console output appears through `defmt` which is transported over QEMU's
semihosting interface into `qemu-run`, which decodes it and prints it out.

ThreadX is automatically compiled from source thanks to the
[`build.rs`](./build.rs) script that this package includes. Refer to that file
if you wish to adjust which ThreadX components are compiled in.

## Licence

* Copyright (c) 2026 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
