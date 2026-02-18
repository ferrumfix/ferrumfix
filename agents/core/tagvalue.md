# Tag-Value Core

## What This Layer Owns

The classic FIX `tag=value<SOH>` encoding and decoding pipeline.

Main files:

- `crates/fefix/src/tagvalue/mod.rs`
- `crates/fefix/src/tagvalue/config.rs`
- `crates/fefix/src/tagvalue/raw_decoder.rs`
- `crates/fefix/src/tagvalue/decoder.rs`
- `crates/fefix/src/tagvalue/encoder.rs`

## Decoder Stack

- `RawDecoder` / `RawDecoderStreaming`:
  - Minimal framing and integrity checks (`BodyLength <9>`, `CheckSum <10>`).
  - Returns `RawFrame`.
- `Decoder` / `DecoderStreaming`:
  - Builds typed `Message` access, including repeating-group handling.
  - Uses dictionary-driven tag type hints (`Length`, `NumInGroup`).

## Key Behavior To Preserve

- Config defaults in `config.rs`:
  - `separator = SOH`.
  - `verify_checksum = true`.
  - `should_decode_associative = true`.
- In `RawDecoder::decode`:
  - Must reject malformed headers/body length.
  - Checksum verification currently applies only when separator is `SOH`.
- In `Decoder::store_field`:
  - Group state transitions depend on `NumInGroup`.
  - Data-field length handoff depends on `Length` tags.

## Encoder Path

- `Encoder::start_message` writes `8`, reserves `9`, then writes `35`.
- `EncoderHandle::done` backfills body length and appends checksum `10`.
- Avoid breaking zero-copy/append semantics on user-provided buffers.

## Typical Change Targets

- Parsing correctness: `raw_decoder.rs` and `decoder.rs`.
- Performance/layout issues: `decoder.rs` internals and buffer handling.
- Output formatting or separator handling: `encoder.rs` + `config.rs`.
