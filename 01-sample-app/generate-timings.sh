#!/usr/bin/env zsh

set -e

cargo clean
cargo build --timings --release

open target/cargo-timings/cargo-timing.html
