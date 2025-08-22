//! Common code for the ThreadX/Rust on Cortex-R5 demo

// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]

pub mod pl011_uart;
pub mod pl190_vic;
pub mod sp804_timer;

// Ensure we pick up the defmt-semihosting transport
use defmt_semihosting as _;

// Ensure we pick up the critical-section impl
use cortex_ar as _;

core::arch::global_asm!(
    r#"

.section .text.startup
.global _start
.global _vectors
.code 32
.align 0
// Work around https://github.com/rust-lang/rust/issues/127269
.fpu vfp3-d16

_vectors:
    LDR     pc, STARTUP                     @ Reset goes to startup function 0x00
    LDR     pc, UNDEFINED                   @ Undefined handler              0x04
    LDR     pc, SWI                         @ Software interrupt handler     0x08
    LDR     pc, PREFETCH                    @ Prefetch exception handler     0x0C
    LDR     pc, ABORT                       @ Abort exception handler        0x10
    LDR     pc, RESERVED                    @ Reserved exception handler     0x14
    LDR     pc, IRQ                         @ IRQ interrupt handler          0x18
    LDR     pc, FIQ                         @ FIQ interrupt handler          0x1C

STARTUP:
    .word  _start                           @ Reset goes to C startup function
UNDEFINED:
    .word  __tx_undefined                   @ Undefined handler
SWI:
    .word  __tx_swi_interrupt               @ Software interrupt handler
PREFETCH:
    .word  __tx_prefetch_handler            @ Prefetch exception handler
ABORT:                             
    .word  __tx_abort_handler               @ Abort exception handler
RESERVED:                            
    .word  __tx_reserved_handler            @ Reserved exception handler
IRQ:                                  
    .word  __tx_irq_handler                 @ IRQ interrupt handler
FIQ:
    .word  __tx_fiq_handler                 @ FIQ interrupt handler

_start:
    // Set stack pointer
    ldr sp, =_stack_top

    // Allow VFP coprocessor access
    mrc p15, 0, r0, c1, c0, 2
    orr r0, r0, #0xF00000
    mcr p15, 0, r0, c1, c0, 2

    // Enable VFP
    mov r0, #0x40000000
    vmsr fpexc, r0

    // Jump to application
    bl kmain

    // In case the application returns, loop forever
    b .

"#
);
