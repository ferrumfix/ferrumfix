# Agents Index

Use this index to jump directly to the smallest set of docs needed for your task.

## Quick Routing

- Parsing or encoding bug:
  - `agents/core/tagvalue.md`
  - `agents/workflows/validation.md`
- Session behavior / heartbeat / sequence numbers:
  - `agents/core/session.md`
  - `agents/workflows/validation.md`
- FIX dictionary semantics or generated constants:
  - `agents/core/dictionary-and-codegen.md`
  - `agents/workflows/change-workflow.md`
- Need broad context first:
  - `agents/core/architecture.md`

## Tree

- `agents/core/architecture.md`
- `agents/core/tagvalue.md`
- `agents/core/session.md`
- `agents/core/dictionary-and-codegen.md`
- `agents/workflows/change-workflow.md`
- `agents/workflows/validation.md`

## Core Entry Files

- `crates/fefix/src/lib.rs`: public modules and traits.
- `crates/fefix/src/tagvalue/mod.rs`: tag-value API surface.
- `crates/fefix/src/session/mod.rs`: session layer exports and traits.
- `crates/fefix-dictionary/src/lib.rs`: dictionary model and FIX version loaders.
- `crates/fefix-codegen/src/lib.rs`: Rust code generation for dictionary-backed definitions.
