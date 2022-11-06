#!/bin/bash

# (c) Copyright 2022 Christian Saide
# SPDX-License-Identifier: GPL-3.0-or-later

BUILD="${1}"
TARGET="${2}"
DOCKER="${3}"

BUILD_ARGS=""
if [[ $BUILD = "release" ]]; then
    BUILD_ARGS="--release"
fi

mkdir -p ./output/${BUILD}

if [[ ${DOCKER} != "true" ]]; then
    # Build Client
    cargo build ${BUILD_ARGS} --target ${TARGET} --bin rift --no-default-features -F ui --target-dir ./target/ui
fi

# Build Server
RUSTFLAGS="-Ctarget-feature=+crt-static" cargo build ${BUILD_ARGS} --target ${TARGET} --bin riftd --no-default-features -F server --target-dir ./target/server