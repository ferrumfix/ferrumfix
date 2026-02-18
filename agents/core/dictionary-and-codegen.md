# Dictionary And Codegen

## Dictionary Source Of Truth

Dictionary loading/parsing lives in:

- `crates/fefix-dictionary/src/lib.rs`
- `crates/fefix-dictionary/src/quickfix.rs`
- `crates/fefix-dictionary/src/builder.rs`

Built-in FIX version constructors (`fix44`, etc.) load QuickFIX XML specs from:

- `crates/fefix-dictionary/src/resources/quickfix/`

## Import Pipeline

`Dictionary::from_quickfix_spec`:

1. parses XML with `roxmltree`,
2. feeds `QuickFixReader`,
3. imports fields/components/messages/header/trailer into a `DictionaryBuilder`.

When touching import logic, preserve required tags and version parsing behavior.

## Codegen Pipeline

Codegen implementation:

- `crates/fefix-codegen/src/lib.rs`

Integration in core crate:

- `crates/fefix/build.rs` generates version files into `OUT_DIR`.
- `crates/fefix/src/definitions.rs` includes generated files.

## Safety Rules

- Do not commit generated `OUT_DIR` artifacts.
- Change generation behavior in `fefix-codegen` or `build.rs`, not included output.
- If definitions change unexpectedly, inspect dictionary parsing first before patching templates.

## Fast Verification Targets

- `tests/codegen_fix44/src/lib.rs`
- `tests/codegen_fix44/build.rs`
