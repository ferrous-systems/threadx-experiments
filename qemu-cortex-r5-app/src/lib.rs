//! Common code for the ThreadX/Rust on Cortex-R5 demo

// SPDX-FileCopyrightText: Copyright (c) 2025 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]

pub mod pl011_uart;
pub mod pl190_vic;
pub mod sp804_timer;

use cortex_r_rt as _;

core::arch::global_asm!(
    r#"
.global kmain

SVC_MODE        =       0xD3                    @ Disable IRQ/FIQ SVC mode

kmain:
@
@    /* Switch to SVC mode, which is what ThreadX expects us to be in (cortex-m-rt leaves us in SYS, not SVC) */
@
    MOV     r0, #SVC_MODE                       @ Build SVC mode CPSR
    MSR     CPSR, r0                            @ Enter SVC mode
    MOV     r1, sp                              @ Get pointer to stack area
@
@    /* Save the system stack pointer.  */
@    _tx_thread_system_stack_ptr = (VOID_PTR) (sp);
@
    LDR     r2, =_tx_thread_system_stack_ptr    @ Pickup stack pointer
    STR     r1, [r2]                            @ Save the system stack
@
@    /* Save the first available memory address.  */
@    _tx_initialize_unused_memory =  (VOID_PTR) _end;
@
    LDR     r1, =_end                           @ Get end of non-initialized RAM area
    LDR     r2, =_tx_initialize_unused_memory   @ Pickup unused memory ptr address
    ADD     r1, r1, #8                          @ Increment to next free word
    STR     r1, [r2]                            @ Save first free memory address
@
    bl      rust_main
    B       .

.global _tx_initialize_low_level

_tx_initialize_low_level:
    bx      lr

    "#
);
