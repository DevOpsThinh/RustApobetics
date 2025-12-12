/*
Copyright (©) 2025 Thinh Truong Nguyen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without limitation in the rights to use, copy, modify, merge,
publish, and/ or distribute copies of the Software in an educational or
personal context, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

Permission is granted to sell and/ or distribute copies of the Software in
a commercial context, subject to the following conditions:
Substantial changes: adding, removing, or modifying large parts, shall be
developed in the Software. Reorganizing logic in the software does not warrant
a substantial change.

This project and source code may use libraries or frameworks that are
released under various Open-Source licenses. Use of those libraries and
frameworks are governed by their own individual licenses.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

Created At: 22:47 - 06/12/2025
*/
use crate::game_util::constants::{PHYSIC_HEIGHT, PHYSIC_WIDTH};

pub const fn width_factor() -> f32 {
    match PHYSIC_WIDTH {
        // Common PC - Laptop screen resolutions
        960 => 6.0,
        1366 => 8.5375,
        1280 => 8.0,
        1600 => 10.0,
        1920 => 12.0,
        2560 => 16.0,
        2880 => 18.0,
        3840 => 24.0,
        _ => 12.0,
    }
}

pub const fn size_factor() -> f32 {
    match PHYSIC_HEIGHT {
        // Common PC - Laptop screen resolutions
        540 => 1.0,
        768 => 1.42,
        800 => 1.48,
        900 => 1.67,
        1080 => 2.0,
        1440 => 2.67,
        1864 => 3.45,
        2160 => 4.0,
        _ => 2.0,
    }
}

