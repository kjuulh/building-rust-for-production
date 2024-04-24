#!/usr/bin/env zsh

set -e

pushd cross-compile-ci
echo "building cross-compile"
cargo build
popd

echo "executing ci against cross-compile"
./cross-compile-ci/target/debug/cross-compile
