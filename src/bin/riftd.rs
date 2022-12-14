// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::OsString;

#[tokio::main]
async fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();
    let code = librift::riftd::run(args).await;
    std::process::exit(code)
}
