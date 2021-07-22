// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/shared/minwindef.rs
 *
 * Basic Windows Type Definitions for minwin partition
 *
 * Author: HTG-YT
 * Copyright (c) 2021 HTG-YT
 */

#[doc = include_str!("../documentation/shared/minwindef/ULONG.md")]
pub type ULONG = u32;

#[doc = include_str!("../documentation/shared/minwindef/PULONG.md")]
pub type PULONG = *mut ULONG;