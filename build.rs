/*
 * Copyright (c) 2025 Ben Buzard
 * SPDX-License-Identifier: MPL-2.0
 */

use std::{env, fs};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // Build stage1 bootloader
    let output = Command::new("nasm").args(&["src/stage1.asm", "-f", "bin", "-o", &format!("{}/bootloader-stage1.bin", out_dir)])
        .output().expect("Failed to execute nasm, is it installed?");

    if !output.status.success() {
        eprintln!("Failed to assemble stage1.asm with nasm (exit code: {})", output.status);

        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        std::process::exit(1);
    }

    println!("cargo:rerun-if-changed=src/stage1.asm");

    fs::write(".last-compile-dir", out_dir).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}