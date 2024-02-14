# Justfile docs: <https://just.systems/man/en/chapter_22.html>

export QUICKCHECK_TESTS := "2500"

# Lists available recipes
help:
	@just --list

# Installs the necessary tools to build and test FerrumFIX (developers and CI only)
install-tools:
	cargo install cargo-binstall
	cargo binstall cargo-nextest --secure --no-confirm
	cargo binstall cargo-hack --secure --no-confirm
	cargo binstall zepter --secure --no-confirm
	rustup update nightly


# Runs all linting steps
lint:
	cargo +nightly fmt --check
	cargo check --all-targets --all-features
	cargo clippy
	zepter
	RUSTDOCFLAGS="--cfg doc_cfg" cargo +nightly doc --all-features

# Runs cargo-hack to test all different Cargo feature combinations
check-feature-combinations:
	cargo hack check --workspace --feature-powerset --all-targets

# Runs all tests
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
