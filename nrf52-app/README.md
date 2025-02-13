# Rust on Eclipse ThreadX Demo for the nRF52840-DK

This example program shows how to compile a Rust application which runs on the [Eclipse
ThreadX](https://projects.eclipse.org/projects/iot.threadx) RTOS.

This application is for the [nRF52840-DK], which includes an Arm Cortex-M4
processor. This board was chosen because it is used in Ferrous System's Rust
Trainings, is inexpensive, and includes a SEGGER J-Link on-board.

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
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabi/debug/nrf52-app`
      Erasing ✔ [00:00:00] [####################################] 12.00 KiB/12.00 KiB @ 33.03 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [####################################] 12.00 KiB/12.00 KiB @ 42.84 KiB/s (eta 0s )    Finished in 0.661s
Hello, this is version unknown!
└─ nrf52_app::__cortex_m_rt_main @ src/main.rs:151
Entering ThreadX kernel...
└─ nrf52_app::__cortex_m_rt_main @ src/main.rs:186
In tx_application_define()...
└─ nrf52_app::tx_application_define @ src/main.rs:26
Stack allocated @ 0x20037444
└─ nrf52_app::tx_application_define @ src/main.rs:59
Thread spawned (entry=12345678) @ 0x2003f440
└─ nrf52_app::tx_application_define @ src/main.rs:85
Stack allocated @ 0x2003944c
└─ nrf52_app::tx_application_define @ src/main.rs:102
Thread spawned (entry=aabbccdd) @ 0x2003f4f8
└─ nrf52_app::tx_application_define @ src/main.rs:128
I am my_thread(12345678)
└─ nrf52_app::my_thread @ src/main.rs:136
I am my_thread(aabbccdd)
└─ nrf52_app::my_thread @ src/main.rs:136
I am my_thread(12345678), count = 1
└─ nrf52_app::my_thread @ src/main.rs:145
I am my_thread(aabbccdd), count = 1
└─ nrf52_app::my_thread @ src/main.rs:145
I am my_thread(12345678), count = 2
└─ nrf52_app::my_thread @ src/main.rs:145
I am my_thread(aabbccdd), count = 2
└─ nrf52_app::my_thread @ src/main.rs:145
I am my_thread(12345678), count = 3
└─ nrf52_app::my_thread @ src/main.rs:145
...
```

Console output appears through `defmt`, Ferrous System's *deferred formatting*
logging mechanism, which is natively understood by `probe-rs`.

ThreadX is automatically compiled from source thanks to the
[`build.rs`](./build.rs) script that this package includes. Refer to that file
if you wish to adjust which ThreadX components are compiled in.

[nRF52840-DK]: https://www.nordicsemi.com/Products/Development-hardware/nRF52840-DK

## Licence

* Copyright (c) 2025 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
