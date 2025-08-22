//! Rust Demo for a QEMU Cortex-R machine, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2025 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![no_main]

use qemu_cortex_r5_app::{
    pl011_uart::Uart,
    pl190_vic,
    sp804_timer::{self, Timer0},
};
use static_cell::StaticCell;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: usize = 16384;
const DEMO_POOL_SIZE: usize = (DEMO_STACK_SIZE * 2) + 16384;

/// Initialise our application.
///
/// ThreadX calls this function during scheduler start-up. We use it to create
/// some threads.
#[no_mangle]
extern "C" fn tx_application_define(_first_unused_memory: *mut core::ffi::c_void) {
    defmt::info!("In tx_application_define()...");

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
                c"byte-pool0".as_ptr() as *mut threadx_sys::CHAR,
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
        defmt::debug!("Stack allocated @ 0x{=usize:08x}", stack_pointer as usize);
        if stack_pointer.is_null() {
            panic!("No space for stack");
        }

        static THREAD_STORAGE: StaticCell<threadx_sys::TX_THREAD> = StaticCell::new();
        let thread = THREAD_STORAGE.uninit();
        unsafe {
            let res = threadx_sys::_tx_thread_create(
                thread.as_mut_ptr(),
                c"thread0".as_ptr() as *mut threadx_sys::CHAR,
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
    defmt::debug!(
        "Thread spawned (entry={=u32:08x}) @ 0x{=usize:08x}",
        entry,
        thread0 as *const _ as usize
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
        defmt::debug!("Stack allocated @ 0x{=usize:08x}", stack_pointer as usize);
        if stack_pointer.is_null() {
            panic!("No space for stack");
        }

        static THREAD_STORAGE2: StaticCell<threadx_sys::TX_THREAD> = StaticCell::new();
        let thread = THREAD_STORAGE2.uninit();
        unsafe {
            let res = threadx_sys::_tx_thread_create(
                thread.as_mut_ptr(),
                c"thread1".as_ptr() as *mut threadx_sys::CHAR,
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
    defmt::debug!(
        "Thread spawned entry={=u32:08x} @ 0x{=usize:08x}",
        entry,
        thread1 as *const _ as usize
    );
}

/// A function we execute in its own thread.
extern "C" fn my_thread(value: u32) {
    defmt::info!("I am my_thread({=u32:08x})", value);
    let mut thread_counter: u64 = 0;
    loop {
        thread_counter += 1;

        unsafe {
            threadx_sys::_tx_thread_sleep(100);
        }

        defmt::info!(
            "I am my_thread({=u32:08x}), count = {=u64}",
            value,
            thread_counter
        );
    }
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `lib.rs`.
#[no_mangle]
pub extern "C" fn kmain() {
    defmt::info!(
        "Hello, this is version {}!",
        BUILD_SLUG.unwrap_or("unknown")
    );

    // Create a UART
    let _uart0 = unsafe { Uart::new_uart0() };

    // Create a timer
    let mut timer0 = unsafe { Timer0::new_timer0() };
    timer0.init(
        10_000,
        sp804_timer::Mode::AutoReload,
        sp804_timer::Interrupts::Enabled,
    );

    // Now we need to enable the Timer0 interrupt and connect it to IRQ on this core
    // It's on PIC interrupt 4.
    let mut vic = unsafe { pl190_vic::Interrupt::new() };
    vic.init();
    vic.enable_interrupt(4);

    timer0.start();

    unsafe {
        threadx_sys::_tx_initialize_kernel_enter();
    }

    panic!("Kernel exited");
}

/// Called from the main interrupt handler
#[no_mangle]
unsafe extern "C" fn handle_interrupt() {
    extern "C" {
        fn _tx_timer_interrupt();
    }

    if Timer0::is_pending() {
        unsafe {
            _tx_timer_interrupt();
        }
        Timer0::clear_interrupt();
    }
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::info!("PANIC: {:?}", defmt::Debug2Format(info));
    semihosting::process::exit(1);
}

// End of file
