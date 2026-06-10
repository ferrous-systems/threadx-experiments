//! Rust Demo for the nRF52840, running ThreadX

// SPDX-FileCopyrightText: Copyright (c) 2026 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception};
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use nrf52840_hal::gpio::{Output, Pin, PushPull};
use static_cell::StaticCell;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: usize = 8192;
const DEMO_POOL_SIZE: usize = (DEMO_STACK_SIZE * 2) + 16384;

const SYSTEM_CLOCK: u32 = 64_000_000;
const SYSTICK_CYCLES: u32 = (SYSTEM_CLOCK / 100) - 1;

static LED_PIN: critical_section::Mutex<core::cell::RefCell<Option<Pin<Output<PushPull>>>>> =
    critical_section::Mutex::new(core::cell::RefCell::new(None));

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

        // try setting a breakpoint here
        if value == 0x12345678 {
            // blink the LED
            critical_section::with(|cs| {
                let mut led_pin = LED_PIN.borrow_ref_mut(cs);
                if let Some(led_pin) = led_pin.as_mut() {
                    if thread_counter & 1 == 0 {
                        led_pin.set_high().unwrap();
                    } else {
                        led_pin.set_low().unwrap();
                    }
                }
            });
        }

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
    critical_section::with(|cs| {
        let mut led_pin = LED_PIN.borrow_ref_mut(cs);
        *led_pin = Some(led);
    });

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

// End of file
