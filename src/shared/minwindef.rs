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
    float,
    int,
    long,
    unsigned_char,
    unsigned_int,
    unsigned_long,
    unsigned_short,
    void
};

pub type PSZ = *mut char;
pub type UCHAR = unsigned_char;
pub type PUCHAR = *mut UCHAR;
pub type ULONG = unsigned_long;
pub type PULONG = *mut ULONG;
pub type USHORT = u16;
pub type PUSHORT = *mut unsigned_short;

pub const MAX_PATH: int = 260;

pub const NULL: *mut void = 0 as *mut void;

pub const FALSE: int = 0;
pub const TRUE: int = 1;

pub type DWORD = unsigned_long;
pub type BOOL = int;
pub type BYTE = unsigned_char;
pub type WORD = unsigned_short;
pub type FLOAT = float;
pub type PFLOAT = *mut FLOAT;
pub type PBOOL = *mut BOOL;
pub type LPBOOL = *mut BOOL;
pub type PBYTE = *mut BYTE;
pub type LPBYTE = *mut BYTE;
pub type PINT = *mut PINT;
pub type LPINT = *mut PINT;
pub type PWORD = *mut WORD;
pub type LPWORD = *mut WORD;
pub type LPLONG = *mut long;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type LPVOID = *mut void;
pub type LPCVOID = *const void;

pub type INT = int;
pub type UINT = unsigned_int;
pub type PUINT = *mut UINT;