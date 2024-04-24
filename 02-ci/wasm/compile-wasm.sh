#!/usr/bin/env zsh

set -e

pushd cross-compile-wasm-ci
echo "building cross-compile (wasm)"
cargo build
popd

echo "executing ci against cross-compile"
./cross-compile-wasm-ci/target/debug/cross-compile
