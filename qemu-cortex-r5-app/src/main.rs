//! Rust Demo for a QEMU Cortex-R machine, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2025 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![no_main]

use byte_strings::c;
use core::{cell::UnsafeCell, fmt::Write as _, mem::MaybeUninit};
use qemu_cortex_r5_app::{
    pl011_uart::Uart,
    pl190_vic,
    sp804_timer::{self, Timer0},
};
use static_cell::StaticCell;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: usize = 16384;
const DEMO_POOL_SIZE: usize = (DEMO_STACK_SIZE * 2) + 16384;

static UART: GlobalUart = GlobalUart::new();

unsafe impl Sync for GlobalUart {}

/// This is our thread-safe global UART.
struct GlobalUart {
    /// This holds the Rust driver
    inner: UnsafeCell<Option<Uart<0x101f_1000>>>,
    /// This ThreadX mutex is used to guard access to the UART in a thread-safe
    /// way, without using a Rust 'critical-section' (which would disable
    /// interrupts).
    mutex: MaybeUninit<UnsafeCell<threadx_sys::TX_MUTEX_STRUCT>>,
}

impl GlobalUart {
    /// Create a new, empty, global UART wrapper
    const fn new() -> GlobalUart {
        GlobalUart {
            inner: UnsafeCell::new(None),
            mutex: MaybeUninit::uninit(),
        }
    }

    /// Store a new UART at run-time, and initialise the ThreadX mutex that
    /// holds it.
    ///
    /// # Safety
    ///
    /// Only call from init, not when threads are running, and only call it
    /// once.
    unsafe fn store(&self, uart: Uart<0x101f_1000>) {
        // Init the ThreadX mutex
        unsafe {
            // init mutex
            threadx_sys::_tx_mutex_create(
                UnsafeCell::raw_get(self.mutex.as_ptr()),
                "my_mutex\0".as_ptr() as _,
                0,
            );
            // unsafely store UART object
            let ptr = self.inner.get();
            let mut_ret = &mut *ptr;
            *mut_ret = Some(uart);
        }
    }
}

// Note that we're implementing for `&GlobalUart`, so we can write to a shared
// reference instead of requiring an exclusive-mutable reference.
impl core::fmt::Write for &GlobalUart {
    /// Write the string to the inner UART, with a lock held
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Grab ThreadX mutex
        unsafe {
            threadx_sys::_tx_mutex_get(
                UnsafeCell::raw_get(self.mutex.as_ptr()),
                threadx_sys::TX_WAIT_FOREVER,
            );
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Acquire);
        }

        // # Safety
        //
        // We hold the ThreadX Mutex at this point
        let uart_option_ref = unsafe { &mut *self.inner.get() };
        let Some(uart) = uart_option_ref else {
            return Err(core::fmt::Error);
        };

        let result = uart.write_str(s);

        // Drop the UART ref, then the threadX mutex
        let _ = uart;
        unsafe {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
            threadx_sys::_tx_mutex_put(UnsafeCell::raw_get(self.mutex.as_ptr()));
        }

        result
    }
}

/// Initialise our application.
///
/// ThreadX calls this function during scheduler start-up. We use it to create
/// some threads.
#[no_mangle]
extern "C" fn tx_application_define(_first_unused_memory: *mut core::ffi::c_void) {
    _ = writeln!(&UART, "In tx_application_define()...");

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
        _ = writeln!(&UART, "Stack allocated @ {:p}", stack_pointer);
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
    _ = writeln!(
        &UART,
        "Thread spawned (entry={:08x}) @ {:p}",
        entry, thread0 as *const _
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
        _ = writeln!(&UART, "Stack allocated @ {:p}", stack_pointer);
        if stack_pointer.is_null() {
            panic!("No space for stack");
        }

        static THREAD_STORAGE2: StaticCell<threadx_sys::TX_THREAD> = StaticCell::new();
        let thread = THREAD_STORAGE2.uninit();
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
    _ = writeln!(
        &UART,
        "Thread spawned (entry={:08x}) @ {:p}",
        entry, thread1 as *const _
    );
}

extern "C" fn my_thread(value: u32) {
    _ = writeln!(&UART, "I am my_thread({:08x})", value);
    let mut thread_counter = 0;
    loop {
        thread_counter += 1;

        unsafe {
            threadx_sys::_tx_thread_sleep(100);
        }

        _ = writeln!(
            &UART,
            "I am my_thread({:08x}), count = {}",
            value, thread_counter
        );
    }
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `lib.rs`.
#[no_mangle]
pub extern "C" fn kmain() {
    // Create a UART
    let mut uart0 = unsafe { Uart::new_uart0() };
    _ = writeln!(
        uart0,
        "Hello, this is version {}!",
        BUILD_SLUG.unwrap_or("unknown")
    );
    unsafe {
        UART.store(uart0);
    }

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
    const SYS_REPORTEXC: u32 = 0x18;
    let _ = writeln!(&UART, "PANIC: {:?}", info);
    loop {
        // Exit, using semihosting
        unsafe {
            core::arch::asm!(
                "svc 0x123456",
                in("r0") SYS_REPORTEXC,
                in("r1") 0x20026
            )
        }
    }
}

// End of file
