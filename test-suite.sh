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

# Default features.
cargo test
# No features at all.
cargo test --features ""
# Only FIX 4.2.
cargo test --features "fix42"
# Only FIXT 1.1.
cargo test --features "fixt11"
# FIXS without OpenSSL.
cargo test --features "fixs"
# OpenSSL without FIXS.
cargo test --features "expose_openssl"
