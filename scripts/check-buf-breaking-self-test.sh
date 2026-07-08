#!/usr/bin/env bash
set -euo pipefail

output_file="$(mktemp)"
trap 'rm -f "${output_file}"' EXIT

if buf breaking ci/buf-breaking-fixtures/broken \
  --against ci/buf-breaking-fixtures/baseline >"${output_file}" 2>&1; then
  cat "${output_file}"
  echo "error: buf breaking self-test unexpectedly passed" >&2
  exit 1
fi

cat "${output_file}"
echo "buf breaking self-test produced the expected failure"
