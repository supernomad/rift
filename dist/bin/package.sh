#!/bin/bash

# (c) Copyright 2022 Christian Saide
# SPDX-License-Identifier: GPL-3.0-or-later

BUILD="${1}"
TARGET="${2}"
SUFFIX="${3}"

for bin in $(find target/ui/${TARGET}/${BUILD}/* -maxdepth 0 -type f -executable); do
    name="$(basename ${bin})"
    rm -f ./output/${BUILD}/${name}_${SUFFIX}
    cp ${bin} ./output/${BUILD}/${name}_${SUFFIX}
done

for bin in $(find target/server/${TARGET}/${BUILD}/* -maxdepth 0 -type f -executable); do
    name="$(basename ${bin})"
    rm -f ./output/${BUILD}/${name}_${SUFFIX}
    cp ${bin} ./output/${BUILD}/${name}_${SUFFIX}
done