#!/usr/bin/env bash

#
# Copyright (c) 2025 Ben Buzard
# SPDX-License-Identifier: MPL-2.0
#

kernel_path=$1
build_path=$(dirname "$kernel_path")

buildrs_path=$(cat ".last-compile-dir")

# TODO: this assumes file names, probably shouldn't, maybe get an actual build system

cat "$buildrs_path/bootloader-stage1.bin" "$kernel_path" > "$build_path/os-image.img"

qemu-system-i386 -drive file="$build_path/os-image.img",if=ide,format=raw -m 512