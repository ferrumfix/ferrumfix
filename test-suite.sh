#!/usr/bin/env bash

set -euxo pipefail

git submodule init
git submodule update

mkdir -p lib/quickfix/config
cd lib/quickfix/config
cmake ..
make
cd ../../..

# Increase number of iteration for QuickCheck.
QUICKCHECK_TESTS="2500"

# Default features
cargo test
# Test multiple feature combinations.
cargo test --no-default-features
cargo test --no-default-features --features "fix42"
cargo test --no-default-features --features "fixt11"
cargo test --no-default-features --features "fixs"
cargo test --no-default-features --features "utils-bytes, utils-rust-decimal"
cargo test --no-default-features --features "fixs, utils-openssl, fix40"
cargo test --no-default-features --features "derive, fix43"
cargo test --no-default-features --features "full"

RUSTDOCFLAGS="--cfg doc_cfg" cargo +nightly doc --all-features
