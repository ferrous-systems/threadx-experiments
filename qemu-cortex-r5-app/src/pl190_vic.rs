//! Code for the Arm PL190 Vector Interrupt Controller

// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

/// A driver for a virtual PL190 Vector Interrupt Controller
///
/// It might skip some important initialisation, but it works on QEMU.
pub struct Interrupt<const ADDR: usize>();

impl Interrupt<0x10140000> {
    /// Create an interrupt controller driver
    ///
    /// # Safety
    ///
    /// Only construct one object per Interrupt Controller at any given time.
    pub unsafe fn new() -> Interrupt<0x10140000> {
        Interrupt()
    }
}

impl<const ADDR: usize> Interrupt<ADDR> {
    const BASE_PTR: *mut u32 = ADDR as *mut u32;

    // These are in 32-bit word offsets (so * 4 to get byte offsets)

    const IRQ_STATUS_OFFSET: usize = 0x00 >> 2;
    const INT_SELECT_OFFSET: usize = 0x0C >> 2;
    const INT_EN_OFFSET: usize = 0x10 >> 2;
    const DEF_VECT_ADDR_OFFSET: usize = 0x34 >> 2;
    const VECT_CTRL_N_START_OFFSET: usize = 0x200 >> 2;

    const CNTL_ENABLE: u32 = 1 << 5;

    const NUM_PRIOS: u8 = 16;
    const NUM_IRQS: u8 = 32;

    /// Get the address of the control register for a particular interrupt vector.
    const fn prio_control_addr(prio: u8) -> *mut u32 {
        if prio >= Self::NUM_PRIOS {
            panic!("bad prio");
        }
        unsafe { Self::BASE_PTR.add(Self::VECT_CTRL_N_START_OFFSET + (prio as usize)) }
    }

    /// Initialise the interrupt controller by setting up all the vectors
    pub fn init(&mut self) {
        unsafe {
            // Set the first 16 vectors to point to the first 16 sources
            for i in 0..Self::NUM_PRIOS {
                Self::prio_control_addr(i).write_volatile(Self::CNTL_ENABLE | u32::from(i));
            }
            // Setup default vector - points at IRQ handler in vector table
            Self::BASE_PTR
                .add(Self::DEF_VECT_ADDR_OFFSET)
                .write_volatile(0x18);
            // Every interrupt is an IRQ not an FIQ
            Self::BASE_PTR
                .add(Self::INT_SELECT_OFFSET)
                .write_volatile(0x0000_0000);
        }
    }

    pub fn enable_interrupt(&mut self, interrupt: u8) {
        if interrupt > Self::NUM_IRQS {
            panic!("Bad IRQ");
        }

        unsafe {
            Self::BASE_PTR
                .add(Self::INT_EN_OFFSET)
                .write_volatile(1 << interrupt);
        }
    }

    pub fn read_interrupt_status() -> u32 {
        unsafe { Self::BASE_PTR.add(Self::IRQ_STATUS_OFFSET).read_volatile() }
    }
}
