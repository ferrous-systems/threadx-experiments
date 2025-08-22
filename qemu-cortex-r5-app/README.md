# Rust on Eclipse ThreadX Demo for the Arm Versatile Application Board

This example program shows how to compile a Rust application which runs on the [Eclipse
ThreadX](https://projects.eclipse.org/projects/iot.threadx) RTOS.

This application is for the Arm Versatile Application Board, which includes an
Arm Cortex-R5 processor. This board was chosen because it can be emulated by
QEMU.

To build and run this project, simply run:

```bash
cargo run
```

You must have `qemu-system-arm` in your system's PATH. You will also need
`arm-none-eabi-gcc` in your system's PATH, so this project can automatically
compile ThreadX (which is expected in `../threadx`). Our runner also requires `defmt-print`.

You will see something like:

```console
$ DEFMT_LOG=info cargo run
   Compiling defmt-macros v1.0.1
   Compiling defmt v1.0.1
   Compiling defmt v0.3.100
   Compiling defmt-semihosting v0.3.0
   Compiling cortex-ar v0.2.0
   Compiling qemu-cortex-r5-app v0.1.0 (threadx-experiments/qemu-cortex-r5-app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.17s
     Running `threadx-experiments/qemu-cortex-r5-app/./qemu_run.sh target/armv7r-none-eabihf/debug/qemu-cortex-r5-app`
ELF_BINARY=target/armv7r-none-eabihf/debug/qemu-cortex-r5-app
Running on '-cpu cortex-r5f -machine versatileab'...
------------------------------------------------------------------------
[INFO ] Hello, this is version unknown! (src/main.rs:156)
[INFO ] In tx_application_define()... (src/main.rs:27)
[INFO ] I am my_thread(12345678) (src/main.rs:138)
[INFO ] I am my_thread(aabbccdd) (src/main.rs:138)
[INFO ] I am my_thread(12345678), count = 1 (src/main.rs:147)
[INFO ] I am my_thread(aabbccdd), count = 1 (src/main.rs:147)
[INFO ] I am my_thread(12345678), count = 2 (src/main.rs:147)
[INFO ] I am my_thread(aabbccdd), count = 2 (src/main.rs:147)
```

Press `Ctrl-C` to quit QEMU.

Console output appears through `defmt` which is transported over QEMU's
semihosting interface into `defmt-print` on the host.

If you wish to debug the program, run:

```bash
cargo run -- -s -S
```

The `-s -S` arguments are passed to `qemu-system-arm` and will cause it to
start a GDB server on `localhost:1234` and wait for GDB to connect.

ThreadX is automatically compiled from source thanks to the
[`build.rs`](./build.rs) script that this package includes. Refer to that file
if you wish to adjust which ThreadX components are compiled in.

## Licence

* Copyright (c) 2025 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
