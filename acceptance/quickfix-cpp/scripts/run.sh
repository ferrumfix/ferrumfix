#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
HARNESS_MANIFEST="${ROOT_DIR}/acceptance/quickfix-cpp/harness/Cargo.toml"
HARNESS_BIN="${ROOT_DIR}/acceptance/quickfix-cpp/harness/target/debug/ferrumfix-quickfix-cpp-harness"
LOG_DIR="${ROOT_DIR}/acceptance/quickfix-cpp/logs"
QUICKFIX_ROOT="${QUICKFIX_ROOT:-${ROOT_DIR}/lib/quickfix}"
ACCEPTANCE_CMD="${QUICKFIX_CPP_ACCEPTANCE_CMD:-}"
ACCEPTANCE_TIMEOUT_SECS="${FFX_QF_ACCEPTANCE_TIMEOUT_SECS:-300}"

mkdir -p "${LOG_DIR}"

if [[ ! -d "${QUICKFIX_ROOT}" ]]; then
  echo "[run] QUICKFIX_ROOT does not exist: ${QUICKFIX_ROOT}" >&2
  exit 2
fi

if [[ -z "${ACCEPTANCE_CMD}" ]]; then
  if [[ -f "${QUICKFIX_ROOT}/test/Runner.rb" ]]; then
    ACCEPTANCE_CMD="cd test && ruby -I. Runner.rb 127.0.0.1 ${FFX_QF_BIND_PORT:-7001} definitions/server/fix44/*.def"
  elif [[ -x "${QUICKFIX_ROOT}/test/runat.sh" ]]; then
    ACCEPTANCE_CMD="cd test && ./runat.sh ${FFX_QF_BIND_PORT:-7001}"
  elif [[ -x "${QUICKFIX_ROOT}/runat" ]]; then
    ACCEPTANCE_CMD="./runat"
  else
    echo "[run] no acceptance command configured" >&2
    echo "[run] set QUICKFIX_CPP_ACCEPTANCE_CMD, for example: QUICKFIX_CPP_ACCEPTANCE_CMD='./runat'" >&2
    exit 2
  fi
fi

# Harness defaults. Override in caller env when needed.
export FFX_QF_BIND_HOST="${FFX_QF_BIND_HOST:-127.0.0.1}"
export FFX_QF_BIND_PORT="${FFX_QF_BIND_PORT:-7001}"
export FFX_QF_ROLE="${FFX_QF_ROLE:-acceptor}"
export FFX_QF_BEGIN_STRING="${FFX_QF_BEGIN_STRING:-FIX.4.4}"
export FFX_QF_SENDER_COMP_ID="${FFX_QF_SENDER_COMP_ID:-ISLD}"
export FFX_QF_TARGET_COMP_ID="${FFX_QF_TARGET_COMP_ID:-TW}"
export FFX_QF_HEARTBEAT_SECS="${FFX_QF_HEARTBEAT_SECS:-30}"
export FFX_QF_HEARTBEAT_RULE="${FFX_QF_HEARTBEAT_RULE:-any}"
export FFX_QF_VERIFY_SENDING_TIME="${FFX_QF_VERIFY_SENDING_TIME:-1}"
export FFX_QF_VERIFY_TEST_INDICATOR="${FFX_QF_VERIFY_TEST_INDICATOR:-1}"
export FFX_QF_ENFORCE_BEGIN_STRING="${FFX_QF_ENFORCE_BEGIN_STRING:-1}"
export FFX_QF_ENFORCE_COMP_ID="${FFX_QF_ENFORCE_COMP_ID:-1}"
export FFX_QF_ALLOW_TEST_MESSAGES="${FFX_QF_ALLOW_TEST_MESSAGES:-1}"
export FFX_QF_MAX_ALLOWED_LATENCY_MS="${FFX_QF_MAX_ALLOWED_LATENCY_MS:-3000}"
export FFX_QF_MAX_SESSIONS="${FFX_QF_MAX_SESSIONS:-0}"
export FFX_QF_CONTINUE_ON_PROTOCOL_VIOLATION="${FFX_QF_CONTINUE_ON_PROTOCOL_VIOLATION:-1}"
export FFX_QF_CONTINUE_ON_IO_ERROR="${FFX_QF_CONTINUE_ON_IO_ERROR:-1}"

# Optional passthrough vars for custom quickfix wrappers.
export FERRUMFIX_HOST="${FFX_QF_BIND_HOST}"
export FERRUMFIX_PORT="${FFX_QF_BIND_PORT}"
export FERRUMFIX_SENDER_COMP_ID="${FFX_QF_SENDER_COMP_ID}"
export FERRUMFIX_TARGET_COMP_ID="${FFX_QF_TARGET_COMP_ID}"

cargo build --offline --manifest-path "${HARNESS_MANIFEST}"

HARNESS_LOG="${LOG_DIR}/harness.log"
ACCEPTANCE_LOG="${LOG_DIR}/acceptance.log"

"${HARNESS_BIN}" >"${HARNESS_LOG}" 2>&1 &
HARNESS_PID="$!"

cleanup() {
  if kill -0 "${HARNESS_PID}" >/dev/null 2>&1; then
    kill "${HARNESS_PID}" >/dev/null 2>&1 || true
    wait "${HARNESS_PID}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

sleep 1
if ! kill -0 "${HARNESS_PID}" >/dev/null 2>&1; then
  echo "[run] harness exited before acceptance command started" >&2
  cat "${HARNESS_LOG}" >&2 || true
  exit 1
fi

echo "[run] quickfix acceptance command: ${ACCEPTANCE_CMD}"
set +e
(
  cd "${QUICKFIX_ROOT}"
  if [[ "${ACCEPTANCE_TIMEOUT_SECS}" -gt 0 ]]; then
    perl -e 'alarm shift @ARGV; exec @ARGV' \
      "${ACCEPTANCE_TIMEOUT_SECS}" \
      bash -lc "${ACCEPTANCE_CMD}"
  else
    bash -lc "${ACCEPTANCE_CMD}"
  fi
) >"${ACCEPTANCE_LOG}" 2>&1
ACCEPTANCE_STATUS="$?"
set -e

if [[ "${ACCEPTANCE_STATUS}" -ne 0 ]]; then
  if [[ "${ACCEPTANCE_TIMEOUT_SECS}" -gt 0 && "${ACCEPTANCE_STATUS}" -eq 142 ]]; then
    echo "[run] acceptance command timed out after ${ACCEPTANCE_TIMEOUT_SECS}s" >&2
  fi
  echo "[run] acceptance command failed (exit ${ACCEPTANCE_STATUS})" >&2
  echo "[run] acceptance log: ${ACCEPTANCE_LOG}" >&2
  echo "[run] harness log: ${HARNESS_LOG}" >&2
  exit "${ACCEPTANCE_STATUS}"
fi

echo "[run] acceptance command completed successfully"
echo "[run] acceptance log: ${ACCEPTANCE_LOG}"
echo "[run] harness log: ${HARNESS_LOG}"
