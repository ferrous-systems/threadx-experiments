//! Rust Demo for a QEMU Cortex-R machine, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2026 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![no_main]

use static_cell::StaticCell;

use qemu_cortex_r5_app::sp804_timer;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");
static PL190: pl190_vic::Pl190Driver =
    unsafe { pl190_vic::Pl190Driver::new_static(qemu_cortex_r5_app::PL190_BASE_ADDR) };
const TIMER_INTERRUPT: pl190_vic::InterruptId = pl190_vic::InterruptId::new(4);
static TIMER0: sp804_timer::Timer0 = unsafe { sp804_timer::Timer0::new_timer0() };

const DEMO_STACK_SIZE: usize = 16384;
const DEMO_POOL_SIZE: usize = (DEMO_STACK_SIZE * 2) + 16384;

/// The SP804 timer runs at 1 MHz, and we want 100 Hz timer ticks
const CLOCKS_PER_TICK: u32 = 1_000_000 / 100;

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

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in lib.rs
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    defmt::info!(
        "Hello, this is version {}!",
        BUILD_SLUG.unwrap_or("unknown")
    );

    // Temporarily create a mutable timer driver so we can configure it...
    let mut timer0 = unsafe { sp804_timer::Timer0::new_timer0() };
    timer0.init(
        CLOCKS_PER_TICK,
        sp804_timer::Mode::AutoReload,
        sp804_timer::Interrupts::Enabled,
    );

    // Now we need to enable the Timer0 interrupt and connect it to IRQ on this core
    // It's on PIC interrupt 4.
    PL190.enable_interrupt(TIMER_INTERRUPT);
    PL190.set_handler(
        TIMER_INTERRUPT,
        pl190_vic::VectorId::new(0),
        Some(handle_timer_interrupt),
    );

    // Start timer after interrupts enabled
    timer0.start();

    // use global TIMER0 from now on
    drop(timer0);

    defmt::info!("Entering ThreadX kernel...");
    unsafe {
        threadx_sys::_tx_initialize_kernel_enter();
    }

    panic!("Kernel exited");
}

/// Call when there's a timer interrupt
fn handle_timer_interrupt() {
    unsafe extern "C" {
        safe fn _tx_timer_interrupt();
    }
    if TIMER0.is_pending() {
        _tx_timer_interrupt();
        TIMER0.clear_interrupt();
    }
}

/// Called from the main interrupt handler in tx_initialize_low_level.S
#[unsafe(no_mangle)]
unsafe extern "C" fn handle_interrupt() {
    PL190.irq_process();
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    aarch32_cpu::interrupt::disable();
    defmt::info!("PANIC: {}", info);
    semihosting::process::exit(1);
}

/// Get the uptime in clock ticks
fn uptime() -> u64 {
    loop {
        // need to check we don't hit a timer interrupt, so read timer ticks twice
        let ticks1 = unsafe { threadx_sys::_tx_time_get() };
        let fraction = TIMER0.timer_remaining();
        let ticks2 = unsafe { threadx_sys::_tx_time_get() };

        // no overflow, so calculate the total uptime
        if ticks1 == ticks2 {
            return (u64::from(ticks1) * u64::from(CLOCKS_PER_TICK)) + u64::from(fraction);
        }
    }
}

defmt::timestamp!("{=u64:tus}", uptime());

// End of file
