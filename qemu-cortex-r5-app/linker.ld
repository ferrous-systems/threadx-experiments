/*
 * Linker script for running ThreadX/Rust on QEMU's Versatile AB with a Cortex-R5
 *
 * SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
 * SPDX-License-Identifier: MIT OR Apache-2.0
*/

MEMORY {
    RAM : ORIGIN = 0, LENGTH = 0x1000000
}

ENTRY(_start)
SECTIONS {
    .startup ORIGIN(RAM) : {
        *(.text.startup)
    } > RAM
    .text : { *(.text .text*) } > RAM
    .rodata : { *(.rodata .rodata*) } > RAM
    .data : { *(.data .data*) } > RAM
    .bss : { *(.bss .bss* COMMON) } > RAM
    /DISCARD/ : {
        *(.note .note*)
    }

    . = ALIGN(16);
    .stack : {
        _stack_bottom = ABSOLUTE(.) ;
        /* Allocate room for stack. This must be big enough for the IRQ, FIQ, and 
        SYS stack if nested interrupts are enabled.  */
        .   =  ALIGN(8) ;
        .   += 0x100000;
        _sp =  . - 16 ;
        _stack_top = ABSOLUTE(.) ;
    } > RAM

  _end = .; __end__ = . ;
}
