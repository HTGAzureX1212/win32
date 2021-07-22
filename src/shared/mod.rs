// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/shared/mod.rs
 *
 * Headers (modules) shared between kernel and user modes.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 HTG-YT
 */

#[cfg(feature = "shared-basetsd")]
pub mod basetsd;

#[cfg(feature = "shared-minwindef")]
pub mod minwindef;

#[cfg(feature = "shared-ntdef")]
pub mod ntdef;