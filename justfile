# Justfile docs: <https://just.systems/man/en/chapter_22.html>

export QUICKCHECK_TESTS := "2500"

# List available recipes
help:
	@just --list

# Install the necessary tools to build and test FerrumFIX
install-tools: && _install-tools-no-binstall
	cargo install cargo-binstall

_install-tools-no-binstall:
	cargo binstall cargo-nextest --locked --secure --no-confirm
	cargo binstall cargo-hack --locked --secure --no-confirm
	cargo binstall cargo-udeps --locked --secure --no-confirm
	cargo binstall zepter --locked --secure --no-confirm
	rustup component add rustfmt --toolchain nightly
	rustup update stable

# Find unused dependencies
udeps:
	RUSTFLAGS=-Awarnings cargo +nightly hack udeps --package fefix --feature-powerset --all-targets --depth 2

# Run all linting steps
lint:
	cargo +nightly fmt --check
	RUSTFLAGS=-Awarnings cargo check --all-targets --all-features
	zepter
	# TODO: clippy
	# TODO: RUSTDOCFLAGS="--cfg doc_cfg" cargo +nightly doc --all-features

# Run cargo-hack to test all different Cargo feature combinations
check-feature-combinations:
	cargo hack check --all --feature-powerset --all-targets --depth 2

# Run all tests
test:
	cargo nextest run --workspace --all-features

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
