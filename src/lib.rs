// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg_attr(coverage_nightly, feature(no_coverage))]

#[macro_use]
extern crate slog;

pub mod base;
pub mod logging;
#[cfg(feature = "ui")]
pub mod rift;
#[cfg(feature = "server")]
pub mod riftd;
pub mod rpc;
