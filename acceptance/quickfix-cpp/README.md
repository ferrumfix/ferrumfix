# QuickFIX C++ Acceptance Harness (Isolated)

This directory hosts FerrumFIX integration for QuickFIX C++ acceptance testing.

It is intentionally **outside** `crates/*`, `examples/*`, and `tests/*` so heavy QuickFIX tooling and orchestration stay decoupled from the main workspace build/test path.

## Layout

- `harness/`: standalone Rust binary that runs FerrumFIX session engine as a network counterparty.
- `scripts/setup.sh`: initializes/builds QuickFIX C++ (via `lib/quickfix` submodule path).
- `scripts/run.sh`: builds the harness, starts it, and runs a QuickFIX acceptance command.
- `logs/`: harness and acceptance command output.

## Prerequisites

- CMake toolchain for QuickFIX C++.
- `lib/quickfix` checked out (setup script can initialize it).
- A QuickFIX C++ acceptance command available as `QUICKFIX_CPP_ACCEPTANCE_CMD` (optional override).
  - If unset, the run script defaults to `test/Runner.rb` (FIX.4.4 server definitions).

## Quick Start

```bash
./acceptance/quickfix-cpp/scripts/setup.sh
./acceptance/quickfix-cpp/scripts/run.sh
# deterministic per-definition progress + timeout
./acceptance/quickfix-cpp/scripts/run-defs.sh
```

If your acceptance flow uses a custom wrapper, point `QUICKFIX_CPP_ACCEPTANCE_CMD` to it.

## Harness Environment

The run script exports these defaults (override as needed):

- `FFX_QF_BIND_HOST=127.0.0.1`
- `FFX_QF_BIND_PORT=7001`
- `FFX_QF_ROLE=acceptor` (`acceptor` or `initiator`)
- `FFX_QF_BEGIN_STRING=FIX.4.4`
- `FFX_QF_SENDER_COMP_ID=ISLD`
- `FFX_QF_TARGET_COMP_ID=TW`
- `FFX_QF_HEARTBEAT_SECS=30`
- `FFX_QF_HEARTBEAT_RULE=exact` (`exact` or `any`)
- `FFX_QF_VERIFY_SENDING_TIME=1`
- `FFX_QF_VERIFY_TEST_INDICATOR=1`
- `FFX_QF_ENFORCE_BEGIN_STRING=1`
- `FFX_QF_ENFORCE_COMP_ID=1`
- `FFX_QF_ALLOW_TEST_MESSAGES=1`
- `FFX_QF_MAX_ALLOWED_LATENCY_MS=3000`
- `FFX_QF_MAX_SESSIONS=0` (`0` means unbounded)
- `FFX_QF_CONTINUE_ON_PROTOCOL_VIOLATION=1`
- `FFX_QF_CONTINUE_ON_IO_ERROR=1`
- `FFX_QF_ACCEPTANCE_TIMEOUT_SECS=300` (`0` disables timeout)
- `FFX_QF_DEFS_GLOB=definitions/server/fix44/*.def` (used by `run-defs.sh`)
- `FFX_QF_PER_DEF_TIMEOUT_SECS=30` (used by `run-defs.sh`)
- `FFX_QF_MAX_DEFS=0` (`0` means all matched definitions)

For external wrapper scripts, these passthrough aliases are exported too:

- `FERRUMFIX_HOST`
- `FERRUMFIX_PORT`
- `FERRUMFIX_SENDER_COMP_ID`
- `FERRUMFIX_TARGET_COMP_ID`

## Notes

- This harness currently targets **session-level** behavior (logon/heartbeat/test-request/logout/resend handling).
- It does not attempt to run QuickFIX C++ acceptance tests from CI by default.
- Keep long-running acceptance expansions inside this directory only.
- `run-defs.sh` writes per-definition logs to `logs/per-def/` and a summary to `logs/summary.txt`.
