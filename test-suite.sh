#!/usr/bin/env bash

git submodule init
git submodule update

mkdir lib/quickfix/config
cd lib/quickfix/config
cmake ..
make
cd ../../..

# Increase number of iteration for QuickCheck.
QUICKCHECK_TESTS="2500"

# Test problematic feature combinations.
cargo test
cargo test --features ""
cargo test --features "fix42"
cargo test --features "fixt11"
cargo test --features "fixs"
cargo test --features "fixs, fixs-utils-openssl, fix40"
