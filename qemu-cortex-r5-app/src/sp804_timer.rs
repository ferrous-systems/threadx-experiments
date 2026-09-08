//! Code for the Arm SP804 Timer

// SPDX-FileCopyrightText: Copyright (c) 2026 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Supported timer modes
pub enum Mode {
    AutoReload,
    SingleShot,
}

/// Supported interrupt options
pub enum Interrupts {
    Disabled,
    Enabled,
}

/// Timer0 on an Arm Versatile Application Board.
pub type Timer0 = Timer<0x101e_2000>;

/// A driver for a virtual SP804 Timer
///
/// It probably skips some important initialisation, but it works on QEMU.
pub struct Timer<const ADDR: usize> {
    inner: OwnedRegisters<ADDR>,
}

impl Timer0 {
    /// Create a new Timer object for Timer0
    ///
    /// # Safety
    ///
    /// Only construct one object per Timer at any given time.
    pub const unsafe fn new_timer0() -> Self {
        Timer {
            inner: OwnedRegisters::new(),
        }
    }
}

impl<const ADDR: usize> Timer<ADDR> {
    /// Initialise the timer
    pub fn init(&mut self, load_value: u32, mode: Mode, interrupt: Interrupts) {
        let mut registers = self.inner.borrow_mut();
        registers.write_load(load_value);
        registers.modify_control(|mut w| {
            w.set_enabled(false);
            w.set_int_enable(match interrupt {
                Interrupts::Disabled => false,
                Interrupts::Enabled => true,
            });
            w.set_one_shot(match mode {
                Mode::AutoReload => false,
                Mode::SingleShot => true,
            });
            w.set_size_32bit(true);
            w.set_periodic(true);
            w
        });
    }

    /// Start the timer
    pub fn start(&mut self) {
        let mut registers = self.inner.borrow_mut();
        registers.modify_control(|w| w.with_enabled(true));
    }

    /// Is the interrupt pending?
    pub fn is_pending(&self) -> bool {
        let registers = self.inner.borrow();
        registers.read_masked_int_status().int_en()
    }

    /// Clear a pending interrupt
    pub fn clear_interrupt(&self) {
        let registers = self.inner.borrow();
        let ptr = registers.pointer_to_int_clear();
        // Safety: this is an atomic write-to-clear register
        unsafe {
            ptr.write_volatile(1);
        }
    }

    /// Read the timer value
    ///
    /// We have a count-down timer so this number goes down over time (until it resets).
    pub fn timer_read(&self) -> u32 {
        let registers = self.inner.borrow();
        registers.read_value()
    }

    /// Get the time remaining before reset
    ///
    /// This number goes up.
    pub fn timer_remaining(&self) -> u32 {
        let registers = self.inner.borrow();
        let reset = registers.read_load();
        let current = registers.read_value();
        reset.wrapping_sub(current)
    }
}

#[derive(derive_mmio::Mmio)]
#[repr(C)]
struct Registers {
    #[mmio(PureRead, Write, Modify)]
    load: u32,
    #[mmio(PureRead)]
    value: u32,
    #[mmio(PureRead, Write, Modify)]
    control: Control,
    #[mmio(Write)]
    int_clear: u32,
    #[mmio(PureRead)]
    raw_int_status: IntStatus,
    #[mmio(PureRead)]
    masked_int_status: IntStatus,
    #[mmio(PureRead, Write, Modify)]
    bg_load: u32,
}

use arbitrary_int::u2;

#[bitbybit::bitfield(u32, debug)]
struct Control {
    #[bit(7, rw)]
    enabled: bool,
    #[bit(6, rw)]
    periodic: bool,
    #[bit(5, rw)]
    int_enable: bool,
    #[bits(2..=3, rw)]
    prescale: u2,
    #[bit(1, rw)]
    size_32bit: bool,
    #[bit(0, rw)]
    one_shot: bool,
}

#[bitbybit::bitfield(u32, debug)]
struct IntStatus {
    #[bit(0, rw)]
    int_en: bool,
}

// End of file
