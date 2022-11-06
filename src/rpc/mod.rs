// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

mod interceptor;
#[cfg(feature = "server")]
mod server;

pub use interceptor::{LogExt, LogInterceptor};
#[cfg(feature = "server")]
pub use server::serve;
