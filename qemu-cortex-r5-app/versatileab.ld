/*
Memory configuration for the Arm Versatile Peripheral Board.

See https://github.com/qemu/qemu/blob/master/hw/arm/versatilepb.c

SPDX-FileCopyrightText: Copyright (c) 2025 Ferrous Systems
SPDX-License-Identifier: MIT OR Apache-2.0

*/

MEMORY {
    SDRAM : ORIGIN = 0, LENGTH = 128M
}

REGION_ALIAS("CODE", SDRAM);
REGION_ALIAS("DATA", SDRAM);

PROVIDE(_end = __euninit);
