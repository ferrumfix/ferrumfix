# Next release

- Reduced number of dependencies in the `fesofh` crate.

# v0.7.0 (YYYY-MM-DD)

This release addresses the following issues:

- **New Cargo features with optional dependencies**. We simplified the dependency graph and hid "nice-to-have's" behing `utils-*` features. At the moment, the following utility features are available:
  - `utils-bytes`
  - `utils-chrono`
  - `utils-decimal`
  - `utils-openssl`
  - `utils-rust-decimal`
  - `utils-slog`
  - `utils-tokio`
