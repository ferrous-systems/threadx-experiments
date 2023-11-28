#!/bin/bash

# Builds the Cortex-M4 version of ThreadX, then builds a Rust
# application that links to it.

# SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -euo pipefail

pushd threadx
cmake -Bbuild -GNinja -DCMAKE_TOOLCHAIN_FILE=cmake/cortex_m4.cmake .
cmake --build ./build
popd

pushd demo-app
cargo build --release
popd
