#!/bin/bash

# SPDX-FileCopyrightText: Copyright (c) 2025 Ferrous Systems
# SPDX-License-Identifier: CC0-1.0

# This requires you to previously run `cargo install defmt-print`

ELF_BINARY=$1
shift
MACHINE="-cpu cortex-r5f -machine versatileab"
LOG_FORMAT='{[{L}]%bold} {s} {({ff}:{l:1})%dimmed}'
echo "ELF_BINARY=$ELF_BINARY"
echo "Running on '$MACHINE'..."
echo "------------------------------------------------------------------------"
qemu-system-arm $MACHINE -semihosting-config enable=on,target=native -kernel $ELF_BINARY $* | defmt-print -e $ELF_BINARY --log-format="$LOG_FORMAT"
echo "------------------------------------------------------------------------"
