/*
 * Linker script for running ThreadX/Rust on STM32U5 in Nonsecure State
 *
 * SPDX-FileCopyrightText: Copyright (c) 2026 Ferrous Systems
 * SPDX-License-Identifier: MIT OR Apache-2.0
*/

MEMORY {
  /* Uses the first flash bank, nonsecure alias */
  FLASH (rx): ORIGIN = 0x08000000, LENGTH = 2M
  /* Uses SRAM1, nonsecure alias */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 768K
}

/* Link to the raw assembly function, avoiding a Rust function prologue/epilogue */
PROVIDE(PendSV = PendSV_Handler);
