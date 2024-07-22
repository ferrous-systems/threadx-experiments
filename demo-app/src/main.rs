//! Rust Demo for the nRF52840, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_main]
#![no_std]

use byte_strings::c;
use cortex_m_rt::entry;
use defmt_rtt as _;
use nrf52840_hal::prelude::OutputPin;
use panic_probe as _;
use static_cell::StaticCell;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: usize = 8192;
const DEMO_POOL_SIZE: usize = (DEMO_STACK_SIZE * 2) + 16384;

const SYSTEM_CLOCK: u32 = 64_000_000;
const SYSTICK_CYCLES: u32 = (SYSTEM_CLOCK / 100) - 1;

#[no_mangle]
extern "C" fn tx_application_define(_first_unused_memory: *mut core::ffi::c_void) {
    defmt::println!("In tx_application_define()...");

    // ThreadX requires a non-const pointer to char for the names, which it
    // wil hold on to in the object, so it must have static lifetime. So we
    // cast-away-const on a static string slice to appease the API.

    let byte_pool = {
        static BYTE_POOL: StaticCell<threadx_sys::TX_BYTE_POOL> = StaticCell::new();
        static BYTE_POOL_STORAGE: StaticCell<[u8; DEMO_POOL_SIZE]> = StaticCell::new();
        let byte_pool = BYTE_POOL.uninit();
        let byte_pool_storage = BYTE_POOL_STORAGE.uninit();
        unsafe {
            threadx_sys::_tx_byte_pool_create(
                byte_pool.as_mut_ptr(),
                c!("byte-pool0").as_ptr() as *mut threadx_sys::CHAR,
                byte_pool_storage.as_mut_ptr() as *mut _,
                DEMO_POOL_SIZE as u32,
            );
            byte_pool.assume_init_mut()
        }
    };

    let entry = 0x12345678;
    let thread0 = {
        let mut stack_pointer = core::ptr::null_mut();
        unsafe {
            threadx_sys::_tx_byte_allocate(
                byte_pool,
                &mut stack_pointer,
                DEMO_STACK_SIZE as _,
                threadx_sys::TX_NO_WAIT,
            );
        }
        defmt::println!("Stack allocated @ {}", stack_pointer);
        if stack_pointer.is_null() {
            panic!("No space for stack");
        }

        static THREAD_STORAGE: StaticCell<threadx_sys::TX_THREAD> = StaticCell::new();
        let thread = THREAD_STORAGE.uninit();
        unsafe {
            let res = threadx_sys::_tx_thread_create(
                thread.as_mut_ptr(),
                c!("thread0").as_ptr() as *mut threadx_sys::CHAR,
                Some(my_thread),
                entry,
                stack_pointer,
                DEMO_STACK_SIZE as _,
                1,
                1,
                threadx_sys::TX_NO_TIME_SLICE,
                threadx_sys::TX_AUTO_START,
            );
            if res != threadx_sys::TX_SUCCESS {
                panic!("Failed to create thread: {}", res);
            }
            thread.assume_init_mut()
        }
    };
    defmt::println!(
        "Thread spawned (entry={:08x}) @ {}",
        entry,
        thread0 as *const _
    );

    let entry = 0xAABBCCDD;
    let thread1 = {
        let mut stack_pointer = core::ptr::null_mut();
        unsafe {
            threadx_sys::_tx_byte_allocate(
                byte_pool,
                &mut stack_pointer,
                DEMO_STACK_SIZE as _,
                threadx_sys::TX_NO_WAIT,
            );
        }
        defmt::println!("Stack allocated @ {:08x}", stack_pointer);
        if stack_pointer.is_null() {
            panic!("No space for stack");
        }

        static THREAD_STORAGE: StaticCell<threadx_sys::TX_THREAD> = StaticCell::new();
        let thread = THREAD_STORAGE.uninit();
        unsafe {
            let res = threadx_sys::_tx_thread_create(
                thread.as_mut_ptr(),
                c!("thread1").as_ptr() as *mut threadx_sys::CHAR,
                Some(my_thread),
                entry,
                stack_pointer,
                DEMO_STACK_SIZE as _,
                1,
                1,
                threadx_sys::TX_NO_TIME_SLICE,
                threadx_sys::TX_AUTO_START,
            );
            if res != threadx_sys::TX_SUCCESS {
                panic!("Failed to create thread: {}", res);
            }
            thread.assume_init_mut()
        }
    };
    defmt::println!(
        "Thread spawned (entry={:08x}) @ {}",
        entry,
        thread1 as *const _
    );
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
    let mut cp = cortex_m::Peripherals::take().unwrap();

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

    // Enable cycle counter
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    // Enable the systick
    cp.SYST.set_reload(SYSTICK_CYCLES);
    cp.SYST.clear_current();
    cp.SYST.enable_interrupt();
    cp.SYST
        .set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    cp.SYST.enable_counter();

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

#[no_mangle]
unsafe extern "C" fn __tx_SysTickHandler() {
    extern "C" {
        fn _tx_timer_interrupt();
    }
    // Call into OS function (not in public API)
    _tx_timer_interrupt();
    // Can do any extra work here
}
