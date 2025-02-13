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
compile ThreadX (which is looks for in `../threadx`).

You will see something like:

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.50s
     Running `qemu-system-arm -machine versatileab -cpu cortex-r5f -semihosting -nographic -kernel target/armv7r-none-eabihf/debug/qemu-cortex-r5-app`
Hello, this is version unknown!
In tx_application_define()...
Stack allocated @ 0xbc48
Thread spawned (entry=12345678) @ 0x17c44
Stack allocated @ 0xfc50
Thread spawned (entry=aabbccdd) @ 0x17cfc
I am my_thread(12345678)
I am my_thread(aabbccdd)
I am my_thread(12345678), count = 1
I am my_thread(aabbccdd), count = 1
I am my_thread(12345678), count = 2
I am my_thread(aabbccdd), count = 2
I am my_thread(12345678), count = 3
I am my_thread(aabbccdd), count = 3
```

Console output appears through a Rust driver for the PL011 UART, which is
included in this binary for simplicity.

You may need to run `killall qemu-system-arm` or use your system Task Manager to
kill QEMU, as the UART console support seems to prevent it from responding to
`Ctrl-C` - at least on macOS. Windows users might try `Ctrl+Break` instead.

If you wish to debug the program, run:

```bash
cargo run -- -s -S
```

The `-s -S` arguments are passed to `qemu-system-arm` and will cause it to start
a GDB server on `localhost:1234` and wait for GDB to connect.

ThreadX is automatically compiled from source thanks to the
[`build.rs`](./build.rs) script that this package includes. Refer to that file
if you wish to adjust which ThreadX components are compiled in.

## Licence

* Copyright (c) 2025 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
