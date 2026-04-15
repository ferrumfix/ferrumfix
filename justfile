# Justfile docs: <https://just.systems/man/en/chapter_22.html>

export QUICKCHECK_TESTS := "2500"

# List available recipes
help:
	@just --list

# Install the necessary tools to build and test FerrumFIX
install-tools: && _install-tools-after-binstall
	cargo install cargo-binstall

_install-tools-after-binstall:
	cargo binstall cargo-nextest --locked --secure --no-confirm
	cargo binstall cargo-hack --locked --secure --no-confirm
	cargo binstall cargo-udeps --locked --secure --no-confirm
	cargo binstall zepter --locked --secure --no-confirm
	rustup install nightly
	rustup component add rustfmt --toolchain nightly
	rustup install stable

# Run all linting steps
lint:
	cargo +nightly fmt --check
	RUSTFLAGS=-Awarnings cargo check --all-targets --all-features
	zepter
	# TODO: clippy
	# TODO: RUSTDOCFLAGS="--cfg doc_cfg" cargo +nightly doc --all-features

# Run all tests
test:
	cargo nextest run --workspace --all-features
	# nextest does not support doctests yet: <https://github.com/nextest-rs/nextest/issues/16>.
	cargo test --workspace --doc --all-features 

# Run cargo-hack to test all different Cargo feature combinations
check-features:
	RUSTFLAGS=-Awarnings cargo hack check --all --all-targets --each-feature

# Find unused dependencies
udeps:
	RUSTFLAGS=-Awarnings cargo +nightly hack udeps --package fefix --feature-powerset --all-targets --depth 2

_init-submodules:
	git submodule init
	git submodule update

_make-quickcheck:
	#!/usr/bin/env bash
	set -euxo pipefail

	mkdir -p lib/quickfix/config
	cd lib/quickfix/config
	cmake ..
	make -j

# Build QuickFIX C++ for isolated acceptance harness use
quickfix-cpp-setup:
	./acceptance/quickfix-cpp/scripts/setup.sh

# Run isolated QuickFIX C++ acceptance harness flow
quickfix-cpp-acceptance:
	./acceptance/quickfix-cpp/scripts/run.sh

# Run QuickFIX C++ acceptance definitions one-by-one with per-definition timeout/progress
quickfix-cpp-acceptance-defs:
	./acceptance/quickfix-cpp/scripts/run-defs.sh
