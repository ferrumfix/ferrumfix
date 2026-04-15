# FerrumFIX Agent Handbook

This file is the root entrypoint for agent-oriented documentation in this repo.

## Table Of Contents

- [Purpose](#purpose)
- [Core Scope](#core-scope)
- [Read Order (Token-Efficient)](#read-order-token-efficient)
- [Documentation Tree](#documentation-tree)

## Purpose

Give all agents a minimal, high-signal map for working on core FerrumFIX logic:

- Where to look first.
- Which files define behavior.
- Where to edit safely.
- How to validate changes.

## Core Scope

For this handbook, "core FerrumFIX logic" means:

- `crates/fefix` (tag-value, session, public API).
- `crates/fefix-dictionary` (dictionary model + QuickFIX XML import).
- `crates/fefix-codegen` (generated field/type definitions).
- `tests/codegen_fix44` and `tests/test_regression_tainted_decoder_fix44`.

Large repository resources (full FIX XML trees, logos, etc.) are out of scope unless directly needed.

## Read Order (Token-Efficient)

1. `agents/README.md`
2. One core doc relevant to the task:
   - `agents/core/tagvalue.md`
   - `agents/core/session.md`
   - `agents/core/dictionary-and-codegen.md`
3. One workflow doc:
   - `agents/workflows/change-workflow.md`
   - `agents/workflows/validation.md`

## Documentation Tree

- `agents/README.md`
- `agents/core/architecture.md`
- `agents/core/tagvalue.md`
- `agents/core/session.md`
- `agents/core/dictionary-and-codegen.md`
- `agents/workflows/change-workflow.md`
- `agents/workflows/validation.md`
