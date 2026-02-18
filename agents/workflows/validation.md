# Validation Playbook

## Quick Checks (Preferred First)

- Targeted regression decode test:
  - `cargo test -p test_regression_tainted_decoder_fix44`
- Codegen smoke test:
  - `cargo test -p codegen_fix44`
- Core crate tests:
  - `cargo test -p fefix --lib`

## Broader Checks

- Workspace tests (all features):
  - `cargo test --workspace --all-features`
- If `nextest` is installed:
  - `cargo nextest run --workspace --all-features`

## Lint / Policy

From `justfile`:

- `just lint`
- `just check-features`

Use these for pre-merge confidence, not as first-pass feedback loops.

## Where To Add Tests

- Parser/decoder behavior:
  - `tests/test_regression_tainted_decoder_fix44/src/lib.rs`
  - unit tests in `crates/fefix/src/tagvalue/*`
- Session behavior:
  - unit tests near `crates/fefix/src/session/*`
- Codegen/dictionary behavior:
  - `tests/codegen_fix44/src/lib.rs`
  - unit tests under `crates/fefix-dictionary/src/*` or `crates/fefix-codegen/src/*`
