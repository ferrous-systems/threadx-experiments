/*
SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
SPDX-License-Identifier: MIT OR Apache-2.0
*/

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

/* Link to the raw assembly function, avoiding a Rust function prologue/epilogue */
PROVIDE(PendSV = __tx_PendSVHandler);
