# Change Workflow

## Goal

Keep changes small, layer-correct, and easy to validate.

## Procedure

1. Identify layer first:
   - parser/encoder -> `tagvalue/*`
   - session behavior -> `session/*`
   - semantics/schema -> `fefix-dictionary/*`
   - generated definitions -> `fefix-codegen/*` + `crates/fefix/build.rs`
2. Edit the narrowest high-signal file.
3. Run targeted tests before full workspace checks.
4. Expand validation only after targeted checks pass.

## Edit Heuristics

- Prefer preserving public trait signatures unless change is intentional.
- Keep error taxonomies stable (`DecodeError`, session errors) unless required.
- Maintain zero-copy/low-allocation behavior in hot paths.
- Follow existing separator/checksum/body-length behavior unless explicitly changing protocol handling.

## Common Pitfalls

- Mistaking generated definitions for source files.
- Mixing session-layer fixes with parser internals in one patch.
- Running full-feature workspace tests before quick targeted checks.
