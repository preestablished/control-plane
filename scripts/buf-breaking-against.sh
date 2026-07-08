#!/usr/bin/env bash
set -euo pipefail

if [[ -n "${GITHUB_ACTIONS:-}" ]]; then
  git fetch --force --tags origin
fi

tag="$(git tag --list 'proto-v[0-9]*' --sort=-v:refname | awk 'NR == 1 { print }')"
if [[ -n "${tag}" ]]; then
  echo "buf breaking baseline: ${tag}"
  exec buf breaking proto --against ".git#tag=${tag},subdir=proto"
fi

if ! git rev-parse --verify --quiet origin/main >/dev/null; then
  echo "error: no proto-v* tag exists and origin/main is unavailable" >&2
  exit 1
fi

merge_base="$(git merge-base HEAD origin/main)"
if [[ -z "${merge_base}" ]]; then
  echo "error: failed to determine merge-base with origin/main" >&2
  exit 1
fi

echo "buf breaking baseline: origin/main merge-base ${merge_base}"
exec buf breaking proto --against ".git#ref=${merge_base},subdir=proto"
