/*
 * Copyright (c) 2025 Ben Buzard
 * SPDX-License-Identifier: MPL-2.0
 */

use std::{env, fs};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // Build stage1 bootloader
    Command::new("nasm").args(&["src/stage1.asm", "-f", "bin", "-o", &format!("{}/bootloader-stage1.bin", out_dir)])
        .status().unwrap();
    println!("cargo:rerun-if-changed=src/stage1.asm");

    fs::write(".last-compile-dir", out_dir).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}