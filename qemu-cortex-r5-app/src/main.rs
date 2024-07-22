//! Rust Demo for a QEMU Cortex-R machine, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![no_main]

use byte_strings::c;
use core::{cell::RefCell, fmt::Write};
use critical_section::Mutex;
use qemu_cortex_r5_app::virt_uart::Uart;
use static_cell::StaticCell;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: usize = 1024;

static UART: GlobalUart = GlobalUart::new();

struct GlobalUart {
    inner: Mutex<RefCell<Option<Uart<0x101f_1000>>>>,
}

impl GlobalUart {
    /// Create a new, empty, global UART wrapper
    const fn new() -> GlobalUart {
        GlobalUart {
            inner: Mutex::new(RefCell::new(None)),
        }
    }

    /// Store a new UART at run-time
    ///
    /// Gives you back the old one, if any.
    fn store(&self, uart: Uart<0x101f_1000>) -> Option<Uart<0x101f_1000>> {
        critical_section::with(|cs| {
            let mut uart_ref = self.inner.borrow_ref_mut(cs);
            uart_ref.replace(uart)
        })
    }
}

// Note that we're implementing for `&GlobalUart`, so we can write to a shared
// reference instead of requiring an exclusive-mutable reference.
impl core::fmt::Write for &GlobalUart {
    /// Write the string to the inner UART, with a lock held
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        critical_section::with(|cs| {
            let mut maybe_uart = self.inner.borrow_ref_mut(cs);
            let Some(uart) = maybe_uart.as_mut() else {
                return Err(core::fmt::Error);
            };
            uart.write_str(s)
        })
    }
}

#[no_mangle]
extern "C" fn tx_application_define(_first_unused_memory: *mut core::ffi::c_void) {
    _ = writeln!(&UART, "In tx_application_define()...");

    // ThreadX requires a non-const pointer to char for the names, which it
    // wil hold on to in the object, so it must have static lifetime. So we
    // cast-away-const on a static string slice to appease the API.

    let byte_pool = {
        static BYTE_POOL: StaticCell<threadx_sys::TX_BYTE_POOL> = StaticCell::new();
        static BYTE_POOL_STORAGE: StaticCell<[u8; 32768]> = StaticCell::new();
        let byte_pool = BYTE_POOL.uninit();
        let byte_pool_storage = BYTE_POOL_STORAGE.uninit();
        unsafe {
            threadx_sys::_tx_byte_pool_create(
                byte_pool.as_mut_ptr(),
                c!("byte-pool0").as_ptr() as *mut threadx_sys::CHAR,
                byte_pool_storage.as_mut_ptr() as *mut _,
                core::mem::size_of_val(&BYTE_POOL_STORAGE) as u32,
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
    let uart0 = unsafe { Uart::new_uart0() };
    UART.store(uart0);
    _ = writeln!(
        &UART,
        "Hello, this is version {}!",
        BUILD_SLUG.unwrap_or("unknown")
    );
    _ = writeln!(&UART, "Entering ThreadX kernel...");
    unsafe {
        threadx_sys::_tx_initialize_kernel_enter();
    }

    panic!("Kernel exited");
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
