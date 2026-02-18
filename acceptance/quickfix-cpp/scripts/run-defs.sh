#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
HARNESS_MANIFEST="${ROOT_DIR}/acceptance/quickfix-cpp/harness/Cargo.toml"
HARNESS_BIN="${ROOT_DIR}/acceptance/quickfix-cpp/harness/target/debug/ferrumfix-quickfix-cpp-harness"
LOG_DIR="${ROOT_DIR}/acceptance/quickfix-cpp/logs"
PER_DEF_LOG_DIR="${LOG_DIR}/per-def"
QUICKFIX_ROOT="${QUICKFIX_ROOT:-${ROOT_DIR}/lib/quickfix}"
DEFS_GLOB="${FFX_QF_DEFS_GLOB:-definitions/server/fix44/*.def}"
PER_DEF_TIMEOUT_SECS="${FFX_QF_PER_DEF_TIMEOUT_SECS:-30}"
MAX_DEFS="${FFX_QF_MAX_DEFS:-0}"

mkdir -p "${PER_DEF_LOG_DIR}"

if [[ ! -d "${QUICKFIX_ROOT}/test" ]]; then
  echo "[run-defs] QUICKFIX test dir missing: ${QUICKFIX_ROOT}/test" >&2
  exit 2
fi

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

cargo build --offline --manifest-path "${HARNESS_MANIFEST}" >/dev/null

defs=()
while IFS= read -r def; do
  defs+=("${def}")
done < <(
  cd "${QUICKFIX_ROOT}/test"
  shopt -s nullglob
  files=( ${DEFS_GLOB} )
  printf '%s\n' "${files[@]}" | sort
)

if [[ "${#defs[@]}" -eq 0 ]]; then
  echo "[run-defs] no definitions matched: ${DEFS_GLOB}" >&2
  exit 2
fi

if [[ "${MAX_DEFS}" -gt 0 && "${MAX_DEFS}" -lt "${#defs[@]}" ]]; then
  defs=("${defs[@]:0:${MAX_DEFS}}")
fi

total="${#defs[@]}"
passed=0
failed=0
timeouts=0

summary_file="${LOG_DIR}/summary.txt"
: > "${summary_file}"

run_one() {
  local def="$1"
  local idx="$2"
  local safe_name
  safe_name="$(echo "${def}" | tr '/ ' '__')"
  local acceptance_log="${PER_DEF_LOG_DIR}/${safe_name}.acceptance.log"
  local harness_log="${PER_DEF_LOG_DIR}/${safe_name}.harness.log"

  "${HARNESS_BIN}" >"${harness_log}" 2>&1 &
  local harness_pid="$!"

  cleanup_one() {
    if kill -0 "${harness_pid}" >/dev/null 2>&1; then
      kill "${harness_pid}" >/dev/null 2>&1 || true
      wait "${harness_pid}" >/dev/null 2>&1 || true
    fi
  }

  trap cleanup_one RETURN

  sleep 0.2
  if ! kill -0 "${harness_pid}" >/dev/null 2>&1; then
    echo "[${idx}/${total}] FAIL ${def} :: harness exited early"
    echo "FAIL ${def} :: harness exited early" >> "${summary_file}"
    failed=$((failed + 1))
    return
  fi

  set +e
  (
    cd "${QUICKFIX_ROOT}/test"
    if [[ "${PER_DEF_TIMEOUT_SECS}" -gt 0 ]]; then
      perl -e 'alarm shift @ARGV; exec @ARGV' \
        "${PER_DEF_TIMEOUT_SECS}" \
        ruby -I. Runner.rb "${FFX_QF_BIND_HOST}" "${FFX_QF_BIND_PORT}" "${def}"
    else
      ruby -I. Runner.rb "${FFX_QF_BIND_HOST}" "${FFX_QF_BIND_PORT}" "${def}"
    fi
  ) >"${acceptance_log}" 2>&1
  local status="$?"
  set -e

  local result
  local reason
  result="$(rg -o "result='[a-z]+'" "${acceptance_log}" | head -n1 || true)"
  reason="$(awk '/<message>/{getline; gsub(/^[ \t]+|[ \t]+$/, "", $0); print; exit}' "${acceptance_log}" || true)"
  if [[ -z "${reason}" ]]; then
    reason="$(rg "Connection|Value in field|Expected field|Number of fields" "${acceptance_log}" | head -n1 | sed 's/^\s*//' || true)"
  fi

  if [[ "${status}" -eq 142 ]]; then
    echo "[${idx}/${total}] TIMEOUT ${def}"
    echo "TIMEOUT ${def}" >> "${summary_file}"
    timeouts=$((timeouts + 1))
    return
  fi

  if [[ "${status}" -eq 0 && "${result}" == "result='success'" ]]; then
    echo "[${idx}/${total}] PASS ${def}"
    echo "PASS ${def}" >> "${summary_file}"
    passed=$((passed + 1))
    return
  fi

  if [[ -n "${reason}" ]]; then
    echo "[${idx}/${total}] FAIL ${def} :: ${reason}"
    echo "FAIL ${def} :: ${reason}" >> "${summary_file}"
  else
    echo "[${idx}/${total}] FAIL ${def}"
    echo "FAIL ${def}" >> "${summary_file}"
  fi
  failed=$((failed + 1))
}

idx=0
for def in "${defs[@]}"; do
  idx=$((idx + 1))
  run_one "${def}" "${idx}"
done

echo
printf '[run-defs] summary: total=%d pass=%d fail=%d timeout=%d\n' "${total}" "${passed}" "${failed}" "${timeouts}"
printf 'TOTAL %d\nPASS %d\nFAIL %d\nTIMEOUT %d\n' "${total}" "${passed}" "${failed}" "${timeouts}" >> "${summary_file}"
printf '[run-defs] summary file: %s\n' "${summary_file}"
