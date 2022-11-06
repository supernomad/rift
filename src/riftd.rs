// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::OsString;

use clap::{crate_version, Parser};
use exitcode::ExitCode;
use futures::stream::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use tokio::select;

use crate::logging;
use crate::rpc::serve;

const RIFTD: &str = "riftd";

#[derive(Debug, Clone, Parser)]
#[command(author, version)]
#[command(name = RIFTD)]
#[command(about = "Rift server backend.", long_about = None)]
struct RiftdConfig {
    #[command(flatten)]
    logging: logging::Config,
    #[arg(short = 'p', long = "rpc-port", env = "RIFT_RPC_PORT", help = "The port to listen on for incoming gRPC requests.", long_help = None, default_value = "8080")]
    rpc_port: u16,
    #[arg(short = 'a', long = "rpc-host", env = "RIFT_RPC_HOST", help = "The IP address to listen on for incoming gRPC requests.", long_help = None, default_value = "127.0.0.1")]
    rpc_host: String,
}

pub async fn run(args: Vec<OsString>) -> ExitCode {
    let cfg = match crate::base::config::<RiftdConfig>(args) {
        Ok(cfg) => cfg,
        Err((code, msg)) => {
            println!("{}", msg);
            return code;
        }
    };

    let logger = logging::new(&cfg.logging, RIFTD, crate_version!());

    let mut signals = match Signals::new(&[SIGTERM, SIGINT]) {
        Ok(signals) => signals,
        Err(e) => {
            error!(logger, "Failed to register signal handler."; "error" => e.to_string());
            return exitcode::IOERR;
        }
    };
    let shutdown = async move {
        signals.next().await;
    };

    let addr = format!("{}:{}", cfg.rpc_host, cfg.rpc_port)
        .parse()
        .unwrap();
    let srv = serve(addr, &logger, shutdown);

    select! {
        _ = srv => {
            error!(logger, "gRPC server exited.")
        }
    }
    exitcode::IOERR
}
