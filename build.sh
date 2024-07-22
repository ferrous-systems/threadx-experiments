#!/bin/bash

# Builds the various ThreadX demo apps.

# SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -euo pipefail

pushd demo-app
cargo build --release
popd
