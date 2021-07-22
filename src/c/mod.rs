// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/c/mod.rs
 *
 * C types and things for crossing the FFI boundary.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 HTG-YT
 */

pub enum void { }

pub type char = i8;
pub type unsigned_char = u8;

pub type float = f32;

pub type short = i16;
pub type unsigned_short = u16;

pub type int = i32;
pub type unsigned_int = u32;

pub type long = i32;
pub type unsigned_long = u32;

pub type __int64 = i64;
pub type __uint64 = u64;
