#!/usr/bin/env bash

set -euxo pipefail

rt='cargo run -- --template you-rust'

export RUSTFLAGS="$($rt rust-flags)"

$rt rustfmt
$rt clippy
$rt test-generic .