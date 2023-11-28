//! Rust Demo for the nRF52840, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_main]
#![no_std]

use core::{ffi::CStr, mem::MaybeUninit};

use cortex_m_rt::entry;
use defmt_rtt as _;
use nrf52840_hal::prelude::OutputPin;
use panic_probe as _;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: usize = 1024;

#[no_mangle]
extern "C" fn tx_application_define(_first_unused_memory: *mut core::ffi::c_void) {
    static mut THREAD_0: MaybeUninit<threadx_sys::TX_THREAD> = MaybeUninit::uninit();
    static mut THREAD_1: MaybeUninit<threadx_sys::TX_THREAD> = MaybeUninit::uninit();
    static mut BYTE_POOL: MaybeUninit<threadx_sys::TX_BYTE_POOL> = MaybeUninit::uninit();
    static mut BYTE_POOL_STORAGE: MaybeUninit<[u8; 32768]> = MaybeUninit::uninit();

    defmt::println!("In tx_application_define()...");
    // ThreadX requires a non-const pointer to char for the names, which it
    // wil hold on to in the object, so it must have static lifetime. So we
    // cast-away-const on a static string slice to appease the API.
    unsafe {
        let pool_name = CStr::from_bytes_with_nul(b"byte-pool0\0").unwrap();
        threadx_sys::_tx_byte_pool_create(
            BYTE_POOL.as_mut_ptr(),
            pool_name.as_ptr() as *mut threadx_sys::CHAR,
            BYTE_POOL_STORAGE.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&BYTE_POOL_STORAGE) as u32,
        );
        let mut pointer = core::ptr::null_mut();
        threadx_sys::_tx_byte_allocate(
            BYTE_POOL.as_mut_ptr(),
            &mut pointer,
            DEMO_STACK_SIZE as _,
            threadx_sys::TX_NO_WAIT,
        );
        defmt::println!("Stack allocated @ {:08x}", pointer);
        let thread_name = CStr::from_bytes_with_nul(b"thread0\0").unwrap();
        threadx_sys::_tx_thread_create(
            THREAD_0.as_mut_ptr(),
            thread_name.as_ptr() as *mut threadx_sys::CHAR,
            Some(my_thread),
            0x12345678,
            pointer,
            DEMO_STACK_SIZE as _,
            1,
            1,
            threadx_sys::TX_NO_TIME_SLICE,
            threadx_sys::TX_AUTO_START,
        );
        defmt::println!("Thread spawned ({:08x})", 0x12345678);

        let mut pointer = core::ptr::null_mut();
        threadx_sys::_tx_byte_allocate(
            BYTE_POOL.as_mut_ptr(),
            &mut pointer,
            DEMO_STACK_SIZE as _,
            threadx_sys::TX_NO_WAIT,
        );
        defmt::println!("Stack allocated @ {:08x}", pointer);
        let thread_name = CStr::from_bytes_with_nul(b"thread1\0").unwrap();
        threadx_sys::_tx_thread_create(
            THREAD_1.as_mut_ptr(),
            thread_name.as_ptr() as *mut threadx_sys::CHAR,
            Some(my_thread),
            0xAABBCCDD,
            pointer,
            DEMO_STACK_SIZE as _,
            1,
            1,
            threadx_sys::TX_NO_TIME_SLICE,
            threadx_sys::TX_AUTO_START,
        );
        defmt::println!("Thread spawned ({:08x})", 0xAABBCCDD);
    }
}

extern "C" fn my_thread(value: u32) {
    defmt::println!("I am my_thread({:08x})", value);
    let mut thread_counter = 0;
    loop {
        thread_counter += 1;

        unsafe {
            threadx_sys::_tx_thread_sleep(100);
        }

        defmt::println!("I am my_thread({:08x}), count = {}", value, thread_counter);
    }
}

#[entry]
fn main() -> ! {
    defmt::println!(
        "Hello, this is version {}!",
        BUILD_SLUG.unwrap_or("unknown")
    );

    let pp = nrf52840_hal::pac::Peripherals::take().unwrap();

    let clocks = nrf52840_hal::Clocks::new(pp.CLOCK);
    let clocks = clocks.enable_ext_hfosc();
    let clocks =
        clocks.set_lfclk_src_external(nrf52840_hal::clocks::LfOscConfiguration::NoExternalNoBypass);
    let clocks = clocks.start_lfclk();
    let _clocks = clocks.enable_ext_hfosc();

    let pins = nrf52840_hal::gpio::p0::Parts::new(pp.P0);
    let mut led = pins
        .p0_13
        .degrade()
        .into_push_pull_output(nrf52840_hal::gpio::Level::High);

    let _ = led.set_low();

    defmt::println!("Entering ThreadX kernel...");
    unsafe {
        threadx_sys::_tx_initialize_kernel_enter();
    }

    panic!("Kernel exited");
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
