//! Build Script for threadx-sys
//!
//! Calls out to bindgen to generate a Rust crate from the ThreadX header
//! files.

// SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;
use std::path::PathBuf;

fn main() {
    let threadx_path = PathBuf::from("../threadx");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Point to ThreadX headers
        .clang_arg(format!("-I{}", threadx_path.join("common/inc").display()))
        .clang_arg(format!(
            "-I{}",
            threadx_path.join("ports/cortex_m4/gnu/inc").display()
        ))
        // Some fake local include files
        .clang_arg("-I./include")
        // Disable standard includes (they belong to the host)
        .clang_arg("-nostdinc")
        // Set the target
        .clang_arg("--target=arm")
        .clang_arg("-mthumb")
        .clang_arg("-mcpu=cortex-m4")
        // Use softfp
        .clang_arg("-mfloat-abi=soft")
        // We're no_std
        .use_core()
        // Include only the useful stuff
        .allowlist_function("tx_.*")
        .allowlist_function("_tx_.*")
        .allowlist_type("TX_.*")
        .allowlist_var("TX_.*")
        .allowlist_var("TX_AUTO_START")
        // Format the output
        .formatter(bindgen::Formatter::Rustfmt)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let rust_source = bindings.to_string();

    let bindings_out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    std::fs::write(bindings_out_path, rust_source).expect("Couldn't write updated bindgen output");

    // The user will have to specify the path to the library themselves because
    // they have to compile ThreadX themselves (or use a pre-compiled version).
    // We only generate the API here.
}
