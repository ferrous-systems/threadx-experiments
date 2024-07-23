//! Code for the Arm SP804 Timer

// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Supported timer modes
pub enum Mode {
    AutoReload = 0,
    SingleShot = 1,
}

/// Supported interrupt options
pub enum Interrupts {
    Disabled = 0,
    Enabled = 1 << 5,
}

/// Timer0 on an Arm Versatile Application Board.
pub type Timer0 = Timer<0x101e_2000>;

/// A driver for a virtual SP804 Timer
///
/// It probably skips some important initialisation, but it works on QEMU.
pub struct Timer<const ADDR: usize>();

impl Timer0 {
    /// Create a new Timer object for Timer0
    ///
    /// # Safety
    ///
    /// Only construct one object per Timer at any given time.
    pub unsafe fn new_timer0() -> Self {
        Timer()
    }
}

impl<const ADDR: usize> Timer<ADDR> {
    const BASE_PTR: *mut u32 = ADDR as *mut u32;

    const LOAD_REGISTER: usize = 0x00 >> 2;
    const CTRL_OFFSET: usize = 0x08 >> 2;
    const ICR_OFFSET: usize = 0x0C >> 2;
    const MIS_OFFSET: usize = 0x14 >> 2;

    const CTRL_TIMERSIZE_32: u32 = 1 << 1;
    const CTRL_TIMERMODE: u32 = 1 << 6;
    const CTRL_TIMEREN: u32 = 1 << 7;

    /// Initialise the timer
    pub fn init(&mut self, load_value: u32, mode: Mode, interrupt: Interrupts) {
        unsafe {
            Self::BASE_PTR
                .add(Self::LOAD_REGISTER)
                .write_volatile(load_value);
            let settings =
                Self::CTRL_TIMERSIZE_32 | Self::CTRL_TIMERMODE | mode as u32 | interrupt as u32;
            Self::BASE_PTR
                .add(Self::CTRL_OFFSET)
                .write_volatile(settings);
        }
    }

    /// Start the timer
    pub fn start(&mut self) {
        unsafe {
            let time1_ctrl = Self::BASE_PTR.add(2);
            let mut temp = time1_ctrl.read_volatile();
            temp |= Self::CTRL_TIMEREN;
            time1_ctrl.write_volatile(temp);
        }
    }

    pub fn is_pending() -> bool {
        let value = unsafe { Self::BASE_PTR.add(Self::MIS_OFFSET).read_volatile() };
        (value & 1) != 0
    }

    /// Clear a pending interrupt
    pub fn clear_interrupt() {
        // Write anything here to clear the interrupt
        unsafe {
            Self::BASE_PTR.add(Self::ICR_OFFSET).write_volatile(1);
        }
    }
}

// End of file
