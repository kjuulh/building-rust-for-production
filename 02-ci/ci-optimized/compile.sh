#!/usr/bin/env zsh

set -e

pushd ci
echo "building ci"
cargo build
popd

echo "executing ci against app"
./ci/target/debug/cross-compile
