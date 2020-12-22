#!/usr/bin/env bash

set -euxo pipefail

export RUST_BACKTRACE=1
export RUSTFLAGS="$(cargo run -- rust-flags)"

cargo run -- rustfmt
cargo run -- clippy
cargo run -- test-generic .