//! Rust Demo for the STM32U5A5 in Nonsecure State, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2026 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![no_main]

use cortex_m::peripheral::scb::SystemHandler;
use cortex_m_rt::{entry, exception};
use defmt_rtt as _;
use static_cell::StaticCell;
use stm32u5 as _;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: usize = 16384;
const DEMO_POOL_SIZE: usize = (DEMO_STACK_SIZE * 2) + 16384;

const SYSTEM_CLOCK: u32 = 4_000_000;
const SYSTICK_CYCLES: u32 = (SYSTEM_CLOCK / 100) - 1;

/// Initialise our application.
///
/// ThreadX calls this function during scheduler start-up. We use it to create
/// some threads.
#[unsafe(no_mangle)]
extern "C" fn tx_application_define(_first_unused_memory: *mut core::ffi::c_void) {
    use threadx_sys::{
        _tx_byte_allocate as tx_byte_allocate, _tx_byte_pool_create as tx_byte_pool_create,
        _tx_thread_create as tx_thread_create, CHAR, TX_AUTO_START, TX_BYTE_POOL, TX_NO_TIME_SLICE,
        TX_NO_WAIT, TX_SUCCESS, TX_THREAD,
    };

    defmt::debug!("In tx_application_define()...");

    // ThreadX requires a non-const pointer to char for the names, which it
    // wil hold on to in the object, so it must have static lifetime. So we
    // cast-away-const on a static string slice to appease the API.

    let byte_pool = {
        static BYTE_POOL: StaticCell<TX_BYTE_POOL> = StaticCell::new();
        static BYTE_POOL_STORAGE: StaticCell<[u8; DEMO_POOL_SIZE]> = StaticCell::new();
        let byte_pool = BYTE_POOL.uninit();
        let byte_pool_storage = BYTE_POOL_STORAGE.uninit();
        unsafe {
            tx_byte_pool_create(
                byte_pool.as_mut_ptr(),
                c"byte-pool0".as_ptr() as *mut CHAR,
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
            tx_byte_allocate(
                byte_pool,
                &mut stack_pointer,
                DEMO_STACK_SIZE as _,
                TX_NO_WAIT,
            );
        }
        defmt::debug!("Stack allocated @ 0x{=usize:08x}", stack_pointer as usize);
        if stack_pointer.is_null() {
            panic!("No space for stack");
        }

        static THREAD_STORAGE: StaticCell<TX_THREAD> = StaticCell::new();
        let thread = THREAD_STORAGE.uninit();
        unsafe {
            let res = tx_thread_create(
                thread.as_mut_ptr(),
                c"thread0".as_ptr() as *mut CHAR,
                Some(my_thread),
                entry,
                stack_pointer,
                DEMO_STACK_SIZE as _,
                1,
                1,
                TX_NO_TIME_SLICE,
                TX_AUTO_START,
            );
            if res != TX_SUCCESS {
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
            tx_byte_allocate(
                byte_pool,
                &mut stack_pointer,
                DEMO_STACK_SIZE as _,
                TX_NO_WAIT,
            );
        }
        defmt::debug!("Stack allocated @ 0x{=usize:08x}", stack_pointer as usize);
        if stack_pointer.is_null() {
            panic!("No space for stack");
        }

        static THREAD_STORAGE2: StaticCell<TX_THREAD> = StaticCell::new();
        let thread = THREAD_STORAGE2.uninit();
        unsafe {
            let res = tx_thread_create(
                thread.as_mut_ptr(),
                c"thread1".as_ptr() as *mut CHAR,
                Some(my_thread),
                entry,
                stack_pointer,
                DEMO_STACK_SIZE as _,
                1,
                1,
                TX_NO_TIME_SLICE,
                TX_AUTO_START,
            );
            if res != TX_SUCCESS {
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
    defmt::info!("Starting my_thread({=u32:08x})", value);
    let sleep_time = if value == 0x12345678 { 100 } else { 200 };
    let mut thread_counter: u64 = 0;
    loop {
        thread_counter += 1;

        defmt::debug!("my_thread({=u32:08x}) is sleeping...", value);

        unsafe {
            threadx_sys::_tx_thread_sleep(sleep_time);
        }

        defmt::info!(
            "my_thread({=u32:08x}) says count = {=u64}",
            value,
            thread_counter
        );
    }
}

/// The entry-point to the Rust application
///
/// Called by the start-up code in cortex-m-rt
#[entry]
fn main() -> ! {
    defmt::info!(
        "Hello, this is version {}!",
        BUILD_SLUG.unwrap_or("unknown")
    );

    let mut cp = cortex_m::Peripherals::take().unwrap();

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

    // set exception priorities as required by ThreadX
    unsafe {
        cp.SCB.set_priority(SystemHandler::MemoryManagement, 0);
        cp.SCB.set_priority(SystemHandler::BusFault, 0);
        cp.SCB.set_priority(SystemHandler::UsageFault, 0);
        cp.SCB.set_priority(SystemHandler::SecureFault, 0);
        cp.SCB.set_priority(SystemHandler::SVCall, 0xFF);
        cp.SCB.set_priority(SystemHandler::DebugMonitor, 0);
        cp.SCB.set_priority(SystemHandler::PendSV, 0xFF);
        cp.SCB.set_priority(SystemHandler::SysTick, 0x40);
    }

    defmt::info!("Entering ThreadX kernel...");
    unsafe {
        threadx_sys::_tx_initialize_kernel_enter();
    }

    panic!("Kernel exited");
}

/// Systick exception handler
#[exception]
fn SysTick() {
    unsafe extern "C" {
        fn _tx_timer_interrupt();
    }

    // Call into OS function (not in public API)
    unsafe {
        _tx_timer_interrupt();
    }
    // Can do any extra work here
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits probe-rs using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    defmt::info!("PANIC: {}", info);
    semihosting::process::exit(1);
}

/// Do ThreadX low-level init
///
/// This is a Rust version of `tx_low_level.S` from the GNU Cortex-M33 example
#[unsafe(no_mangle)]
pub extern "C" fn _tx_initialize_low_level() {
    unsafe extern "C" {
        static __vector_table: u32;
        static __sheap: u32;
        static mut _tx_thread_system_stack_ptr: u32;
        static mut _tx_initialize_unused_memory: u32;
    }
    let vector_table_ptr = &raw const __vector_table;
    let stack_ptr_var = &raw mut _tx_thread_system_stack_ptr;
    let heap_start = &raw const __sheap;
    let unused_mem_ptr = &raw mut _tx_initialize_unused_memory;
    unsafe {
        let init_stack_pointer = vector_table_ptr.read();
        stack_ptr_var.write_volatile(init_stack_pointer);
        unused_mem_ptr.write_volatile(heap_start.offset(1) as u32);
    }
}

// End of file
