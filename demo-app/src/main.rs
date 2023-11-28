//! Rust Demo for the nRF52840, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_main]
#![no_std]

use core::mem::MaybeUninit;

use cortex_m_rt::entry;
use defmt_rtt as _;
use nrf52840_hal::prelude::OutputPin;
use panic_probe as _;

#[repr(C)]
struct TxThread {
    storage: [u32; 128],
}

#[repr(C)]
struct TxBytePool {
    storage: [u32; 128],
}

static mut THREAD_0: MaybeUninit<TxThread> = MaybeUninit::uninit();

static mut THREAD_1: MaybeUninit<TxThread> = MaybeUninit::uninit();

static mut BYTE_POOL: MaybeUninit<TxBytePool> = MaybeUninit::uninit();

static mut BYTE_POOL_STORAGE: MaybeUninit<[u8; 32768]> = MaybeUninit::uninit();

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

type TxThreadFunc = extern "C" fn(entry_input: core::ffi::c_ulong);

extern "C" {
    fn _tx_initialize_kernel_enter();
    // UINT        _tx_thread_create(TX_THREAD *thread_ptr, CHAR *name_ptr,
    //     VOID (*entry_function)(ULONG entry_input), ULONG entry_input,
    //     VOID *stack_start, ULONG stack_size,
    //     UINT priority, UINT preempt_threshold,
    //     ULONG time_slice, UINT auto_start);

    fn _tx_thread_create(
        thread_ptr: *mut TxThread,
        name_ptr: *const core::ffi::c_char,
        entry_function: TxThreadFunc,
        entry_input: core::ffi::c_ulong,
        stack_start: *mut core::ffi::c_void,
        stack_size: core::ffi::c_ulong,
        priority: core::ffi::c_uint,
        preempt_threshold: core::ffi::c_uint,
        time_slice: core::ffi::c_ulong,
        auto_start: core::ffi::c_uint,
    ) -> u32;

    // _tx_byte_allocate(TX_BYTE_POOL *pool_ptr, VOID **memory_ptr, ULONG memory_size,
    //     ULONG wait_option);
    fn _tx_byte_allocate(
        pool_ptr: *mut TxBytePool,
        memory_ptr: *mut *mut core::ffi::c_void,
        memory_size: core::ffi::c_ulong,
        wait_option: core::ffi::c_ulong,
    );

    // UINT        _tx_byte_pool_create(TX_BYTE_POOL *pool_ptr, CHAR *name_ptr, VOID *pool_start,
    //     ULONG pool_size);
    fn _tx_byte_pool_create(
        pool_ptr: *mut TxBytePool,
        name_ptr: *const core::ffi::c_char,
        pool_start: *mut core::ffi::c_void,
        pool_size: core::ffi::c_ulong,
    );
    // UINT        _tx_thread_sleep(ULONG timer_ticks);
    fn _tx_thread_sleep(timer_ticks: core::ffi::c_ulong);
}

const DEMO_STACK_SIZE: core::ffi::c_ulong = 1024;

#[no_mangle]
extern "C" fn tx_application_define(_first_unused_memory: *mut core::ffi::c_void) {
    defmt::println!("In tx_application_define()...");
    unsafe {
        _tx_byte_pool_create(
            BYTE_POOL.as_mut_ptr(),
            "byte pool 0\0".as_ptr() as *const core::ffi::c_char,
            BYTE_POOL_STORAGE.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&BYTE_POOL_STORAGE) as u32,
        );
        let mut pointer = core::ptr::null_mut();
        _tx_byte_allocate(BYTE_POOL.as_mut_ptr(), &mut pointer, DEMO_STACK_SIZE, 0);
        defmt::println!("Stack allocated @ {:08x}", pointer);
        _tx_thread_create(
            THREAD_0.as_mut_ptr(),
            "thread 0\0".as_ptr() as *const core::ffi::c_char,
            my_thread,
            0x12345678,
            pointer,
            DEMO_STACK_SIZE,
            1,
            1,
            0,
            1,
        );
        defmt::println!("Thread spawned ({:08x})", 0x12345678);

        let mut pointer = core::ptr::null_mut();
        _tx_byte_allocate(BYTE_POOL.as_mut_ptr(), &mut pointer, DEMO_STACK_SIZE, 0);
        defmt::println!("Stack allocated @ {:08x}", pointer);
        _tx_thread_create(
            THREAD_1.as_mut_ptr(),
            "thread 1\0".as_ptr() as *const core::ffi::c_char,
            my_thread,
            0xAABBCCDD,
            pointer,
            DEMO_STACK_SIZE,
            1,
            1,
            0,
            1,
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
            _tx_thread_sleep(100);
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
        _tx_initialize_kernel_enter();
    }

    panic!("Kernel exited");
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
