// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/shared/minwindef.rs
 *
 * Basic Windows Type Definitions for minwin partition
 *
 * Author: HTG-YT
 * Copyright (c) 2021 HTG-YT
 */

use crate::c::{
    char,
    unsigned_char,
    unsigned_long,
    unsigned_short
};

pub type PSZ = *mut char;

pub type UCHAR = unsigned_char;
pub type PUCHAR = *mut UCHAR;

pub type ULONG = unsigned_long;
pub type PULONG = *mut ULONG;

pub type USHORT = u16;
pub type PUSHORT = *mut unsigned_short;