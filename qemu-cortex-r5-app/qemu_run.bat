@echo off

rem SPDX-FileCopyrightText: Copyright (c) 2025 Ferrous Systems
rem SPDX-License-Identifier: CC0-1.0

rem This requires you to previously run `cargo install defmt-print`

set ELF_BINARY=%1
set MACHINE=-cpu cortex-r5f -machine versatileab
set QEMU_PATH=%ProgramFiles%\qemu
echo ELF_BINARY=%ELF_BINARY%
echo Running on '%MACHINE%'...
echo ------------------------------------------------------------------------
"%QEMU_PATH%\qemu-system-arm" %MACHINE% -semihosting-config enable=on -nographic -kernel %* | defmt-print -e %ELF_BINARY%
echo "------------------------------------------------------------------------"
