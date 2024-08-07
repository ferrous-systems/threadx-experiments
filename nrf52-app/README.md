# Rust on ThreadX on nRF52 Example Binary

This is a Rust application which uses ThreadX and runs on an nRF52840-DK board.

You can compile it with Rust, or with Ferrocene. You will need
`arm-none-eabi-gcc` installed to compile ThreadX, which this application does
automatically. You will also need `probe-rs` from <https://probe.rs>.

```console
$ cargo run --release
   Compiling nrf52-app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/threadx-experiments/nrf52-app)
    Finished `dev` profile [optimized + debuginfo] target(s) in 4.81s
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabi/debug/nrf52-app`
      Erasing ✔ [00:00:00] [####################################] 12.00 KiB/12.00 KiB @ 33.03 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [####################################] 12.00 KiB/12.00 KiB @ 42.84 KiB/s (eta 0s )    Finished in 0.661s
<lvl> Hello, this is version unknown!
└─ nrf52_app::__cortex_m_rt_main @ src/main.rs:151
<lvl> Entering ThreadX kernel...
└─ nrf52_app::__cortex_m_rt_main @ src/main.rs:186
<lvl> In tx_application_define()...
└─ nrf52_app::tx_application_define @ src/main.rs:26
<lvl> Stack allocated @ 0x20037444
└─ nrf52_app::tx_application_define @ src/main.rs:59
<lvl> Thread spawned (entry=12345678) @ 0x2003f440
└─ nrf52_app::tx_application_define @ src/main.rs:85
<lvl> Stack allocated @ 0x2003944c
└─ nrf52_app::tx_application_define @ src/main.rs:102
<lvl> Thread spawned (entry=aabbccdd) @ 0x2003f4f8
└─ nrf52_app::tx_application_define @ src/main.rs:128
<lvl> I am my_thread(12345678)
└─ nrf52_app::my_thread @ src/main.rs:136
<lvl> I am my_thread(aabbccdd)
└─ nrf52_app::my_thread @ src/main.rs:136
<lvl> I am my_thread(12345678), count = 1
└─ nrf52_app::my_thread @ src/main.rs:145
<lvl> I am my_thread(aabbccdd), count = 1
└─ nrf52_app::my_thread @ src/main.rs:145
<lvl> I am my_thread(12345678), count = 2
└─ nrf52_app::my_thread @ src/main.rs:145
<lvl> I am my_thread(aabbccdd), count = 2
└─ nrf52_app::my_thread @ src/main.rs:145
<lvl> I am my_thread(12345678), count = 3
└─ nrf52_app::my_thread @ src/main.rs:145
...
```

## Licence

* Copyright (c) 2024 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
