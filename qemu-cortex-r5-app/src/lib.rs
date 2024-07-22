#![no_std]

pub mod critical_section;
pub mod virt_uart;

core::arch::global_asm!(
    r#"

.section .text.startup
.global _start
.global _vectors
.code 32
.align 0
// Work around https://github.com/rust-lang/rust/issues/127269
.fpu vfp3-d16

__vectors:
    LDR     pc, STARTUP                     @ Reset goes to startup function
    LDR     pc, UNDEFINED                   @ Undefined handler
    LDR     pc, SWI                         @ Software interrupt handler
    LDR     pc, PREFETCH                    @ Prefetch exception handler
    LDR     pc, ABORT                       @ Abort exception handler
    LDR     pc, RESERVED                    @ Reserved exception handler
    LDR     pc, IRQ                         @ IRQ interrupt handler
    LDR     pc, FIQ                         @ FIQ interrupt handler

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
