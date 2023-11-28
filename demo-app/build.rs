//! Build script for the Rust/ThreadX demo

// SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    // put memory layout (linker script) in the linker search path
    fs::copy("memory.x", out_dir.join("memory.x"))?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");

    // Include our ThreadX static library
    println!(
        "cargo:rustc-link-search={}",
        crate_dir.join("../threadx/build").display()
    );
    println!("cargo:rustc-link-lib=static=threadx");

    // Compile our assembly code
    cc::Build::new()
        .file("src/tx_low_level.S")
        .compile("tx_low_level");
    println!("cargo:rerun-if-changed=src/tx_low_level.S");

    Ok(())
}
