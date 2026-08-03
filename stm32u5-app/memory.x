/*
 * Linker script for running ThreadX/Rust on STM32U5 in Nonsecure State
 *
 * SPDX-FileCopyrightText: Copyright (c) 2026 Ferrous Systems
 * SPDX-License-Identifier: MIT OR Apache-2.0
*/

MEMORY {
  /* Nonecure Code uses the first flash bank, nonsecure alias */
  FLASH (rx): ORIGIN = 0x08000000, LENGTH = 2M
  /* Nonecure Code uses SRAM3, nonsecure alias */
  RAM (rwx) : ORIGIN = 0x200D0000, LENGTH = 832K
}

/* Link to the raw assembly function, avoiding a Rust function prologue/epilogue */
PROVIDE(PendSV = PendSV_Handler);
