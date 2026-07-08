#!/usr/bin/env bash
set -euo pipefail

if [[ -n "${GITHUB_ACTIONS:-}" ]]; then
  git fetch --force --tags origin
fi

head_sha="$(git rev-parse HEAD)"
current_tag=""
if [[ "${GITHUB_REF_TYPE:-}" == "tag" ]]; then
  current_tag="${GITHUB_REF_NAME:-}"
fi

tag="$(git tag --list 'proto-v[0-9]*' --sort=-v:refname | awk -v skip="${current_tag}" '$0 != skip { print; exit }')"
if [[ -n "${tag}" ]]; then
  tag_sha="$(git rev-list -n 1 "${tag}")"
  if [[ "${tag_sha}" == "${head_sha}" ]]; then
    echo "error: refusing to run buf breaking against ${tag}; it points at HEAD (${head_sha})" >&2
    exit 1
  fi

  echo "buf breaking baseline: ${tag}"
  exec buf breaking proto --against ".git#tag=${tag},subdir=proto"
fi

if [[ -n "${current_tag}" ]]; then
  if ! git rev-parse --verify --quiet "refs/tags/${current_tag}" >/dev/null; then
    echo "error: current tag ${current_tag} is not available locally" >&2
    exit 1
  fi

  echo "buf breaking baseline: ${current_tag} (first proto tag bootstrap)"
  exec buf breaking proto --against ".git#tag=${current_tag},subdir=proto"
fi

baseline_ref=""
baseline_name=""
if [[ "${GITHUB_EVENT_NAME:-}" == "push" && -n "${GITHUB_EVENT_PATH:-}" ]]; then
  before_sha="$(
    python3 - "${GITHUB_EVENT_PATH}" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    before = json.load(handle).get("before", "")
print(before)
PY
  )"
  if [[ "${before_sha}" =~ ^[0-9a-f]{40}$ && "${before_sha}" != "0000000000000000000000000000000000000000" ]]; then
    baseline_ref="${before_sha}"
    baseline_name="push before ${before_sha}"
  fi
fi

if ! git rev-parse --verify --quiet origin/main >/dev/null; then
  echo "error: no proto-v* tag exists and origin/main is unavailable" >&2
  exit 1
fi

if [[ -z "${baseline_ref}" ]]; then
  baseline_ref="$(git merge-base HEAD origin/main)"
  baseline_name="origin/main merge-base ${baseline_ref}"
fi

if [[ -z "${baseline_ref}" ]]; then
  echo "error: failed to determine breaking baseline" >&2
  exit 1
fi
if [[ "${baseline_ref}" == "${head_sha}" ]]; then
  echo "error: refusing to run buf breaking against HEAD (${head_sha})" >&2
  exit 1
fi

echo "buf breaking baseline: ${baseline_name}"
exec buf breaking proto --against ".git#ref=${baseline_ref},subdir=proto"
