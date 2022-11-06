// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::path::PathBuf;

const PROTO_DIR: &str = "./proto/";

fn compile_server() -> tonic_build::Builder {
    tonic_build::configure().build_server(true)
}

fn compile_client() -> tonic_build::Builder {
    tonic_build::configure().build_client(true)
}

fn compile_both() -> tonic_build::Builder {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let files = std::fs::read_dir(PROTO_DIR).expect("failed to list proto files.");
    for file in files {
        let file = file.expect("failed to read file path");
        let file = file.path();
        let file_name = String::from(
            file.file_name()
                .expect("failed to determine filename of current path")
                .to_str()
                .expect("failed to convert &OsStr to &str"),
        );

        let descriptor_name = file_name.replace(".proto", "_descriptor.bin");

        let builder = if cfg!(all(feature = "ui", feature = "server")) {
            compile_both()
        } else if cfg!(feature = "ui") {
            compile_client()
        } else if cfg!(feature = "server") {
            compile_server()
        } else {
            panic!("invalid feature selected");
        };

        builder
            .file_descriptor_set_path(out_dir.join(descriptor_name))
            .compile(&[file], &[PROTO_DIR])?;
    }

    Ok(())
}
