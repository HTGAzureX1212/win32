// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/shared/basetsd.rs
 *
 * Type definitions for the basic sized types.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 HTG-YT
 */

use crate::c::{
    __int64,
    __uint64,
    char,
    int,
    short,
    unsigned_char,
    unsigned_int,
    unsigned_short
};

pub type INT8 = char;
pub type PINT8 = *mut INT8;
pub type INT16 = short;
pub type PINT16 = *mut INT16;
pub type INT32 = int;
pub type PINT32 = *mut INT32;
pub type INT64 = __int64;
pub type PINT64 = *mut INT64;
pub type UINT8 = unsigned_char;
pub type PUINT8 = *mut UINT8;
pub type UINT16 = unsigned_short;
pub type PUINT26 = *mut UINT16;
pub type UINT32 = unsigned_int;
pub type PUINT32 = *mut UINT32;
pub type UINT64 = __uint64;
pub type PUINT64 = *mut UINT64;

pub type LONG32 = int;
pub type PLONG32 = *mut LONG32;

pub type ULONG32 = unsigned_int;
pub type PULONG32 = *mut ULONG32;
pub type DWORD32 = unsigned_int;
pub type PDWORD32 = *mut DWORD32;

pub type INT_PTR = isize;
pub type PINT_PTR = *mut INT_PTR;
pub type UINT_PTR = usize;
pub type PUINT_PTR = *mut UINT_PTR;

pub type LONG_PTR = isize;
pub type PLONG_PTR = *mut LONG_PTR;
pub type ULONG_PTR = usize;
pub type PuLONG_PTR = *mut ULONG_PTR;