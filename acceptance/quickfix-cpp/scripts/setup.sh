#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
QUICKFIX_ROOT="${QUICKFIX_ROOT:-${ROOT_DIR}/lib/quickfix}"
QUICKFIX_BUILD_DIR="${QUICKFIX_BUILD_DIR:-${ROOT_DIR}/acceptance/quickfix-cpp/.build/quickfix}"

if [[ ! -f "${QUICKFIX_ROOT}/CMakeLists.txt" ]]; then
  echo "[setup] quickfix sources not present at ${QUICKFIX_ROOT}" >&2
  echo "[setup] initializing submodule lib/quickfix" >&2
  git -C "${ROOT_DIR}" submodule update --init --recursive lib/quickfix
fi

mkdir -p "${QUICKFIX_BUILD_DIR}"

cmake \
  -S "${QUICKFIX_ROOT}" \
  -B "${QUICKFIX_BUILD_DIR}" \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_POLICY_VERSION_MINIMUM=3.5 \
  -DCMAKE_CXX_FLAGS="-D__TOS_AIX__"
cmake --build "${QUICKFIX_BUILD_DIR}" --parallel

echo "[setup] quickfix build completed"
if [[ -f "${QUICKFIX_ROOT}/test/Runner.rb" ]]; then
  echo "[setup] detected acceptance runner script: ${QUICKFIX_ROOT}/test/Runner.rb"
elif [[ -x "${QUICKFIX_ROOT}/runat" ]]; then
  echo "[setup] detected acceptance runner: ${QUICKFIX_ROOT}/runat"
else
  echo "[setup] no default acceptance runner detected; set QUICKFIX_CPP_ACCEPTANCE_CMD explicitly when running acceptance"
fi
