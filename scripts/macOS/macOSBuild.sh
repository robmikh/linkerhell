#!/bin/bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

CMAKE_BUILD_TYPE=Debug
CARGO_BUILD_FLAG=
if [ $1 -e "release" ]; then
    CMAKE_BUILD_TYPE=release
    CARGO_BUILD_FLAG="--release"
fi

REPO_ROOT_PATH="$(dirname $(dirname "$CURRENT_DIR"))"
BUILDPATH="$REPO_ROOT_PATH/_build/macOS"

pushd "$REPO_ROOT_PATH/rust-lib"
CARGO_TARGET_DIR="$BUILDPATH/rust-lib"
env CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo build --features metal $CARGO_BUILD_FLAG
popd

cmake . "-B$BUILDPATH" -GNinja "-DCMAKE_BUILD_TYPE=$CMAKE_BUILD_TYPE" "-DCMAKE_INSTALL_PREFIX=$BUILDPATH" -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++
ninja -C "$BUILDPATH" install
