#!/bin/bash

# Builds the various ThreadX demo apps.

# SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -euo pipefail

pushd nrf52-app
cargo build --release
popd
