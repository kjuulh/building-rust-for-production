#!/usr/bin/env zsh

set -e

pushd cross-compile-musl-ci
echo "building cross-compile (musl)"
cargo build
popd

echo "executing ci against cross-compile"
./cross-compile-musl-ci/target/debug/cross-compile
