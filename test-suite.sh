#!/usr/bin/env bash

set -euxo pipefail

# Increase number of iteration for QuickCheck.
export QUICKCHECK_TESTS="2500"

# TODO: Is there an xtask for running a test suite against an array of feature sets
cargo test
cargo test --no-default-features
cargo test --no-default-features --features "fix40"
cargo test --no-default-features --features "fix42"
cargo test --no-default-features --features "fixt11"
# TODO: What happened to fixs
# cargo test --no-default-features --features "fixs"
cargo test --no-default-features --features "utils-bytes, utils-rust-decimal"
cargo test --no-default-features --features "derive, fix43"
cargo test --no-default-features --features "full"

RUSTDOCFLAGS="--cfg doc_cfg" cargo +nightly doc --all-features
