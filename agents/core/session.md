# Session Core

## What Exists Today

Session support is present but still evolving; several items are marked `FIXME` in `crates/fefix/src/session/mod.rs`.

Main files:

- `crates/fefix/src/session/mod.rs`
- `crates/fefix/src/session/event_loop.rs`
- `crates/fefix/src/session/config.rs`
- `crates/fefix/src/session/seq_numbers.rs`
- `crates/fefix/src/session/environment.rs`
- `crates/fefix/src/session/heartbeat_rule.rs`

## Core Runtime Loop

`LlEventLoop` in `event_loop.rs` multiplexes:

- inbound bytes from `AsyncRead`,
- heartbeat deadline,
- soft tolerance (`TestRequest`),
- hard tolerance (`Logout`).

It wraps a `DecoderStreaming<Vec<u8>>` and yields `LlEvent`.

## Sequence Number Semantics

`SeqNumbers` (`seq_numbers.rs`) tracks:

- `next_inbound`,
- `next_outbound`,
- inbound validation (`Equal` OK, `Less` TooLow, `Greater` Recover).

When changing this logic, verify behavior around recoverable gaps vs hard failures.

## Backend Contract

`Backend` trait in `session/mod.rs` is the agent-facing integration point for:

- inbound application message handling,
- outbound post-processing,
- resend request handling,
- handshake completion hooks,
- fetch/pending message retrieval.

Treat trait signature changes as breaking API changes.
