#!/usr/bin/env bash
set -euo pipefail

crate_version="$(awk '
  /^\[package\]$/ { in_section = 1; next }
  /^\[/ { in_section = 0 }
  in_section && /^version[[:space:]]*=/ {
    gsub(/"/, "", $3);
    print $3;
    exit
  }
' crates/determinism-proto/Cargo.toml)"
workspace_version="$(awk '
  /^\[workspace\.package\]$/ { in_section = 1; next }
  /^\[/ { in_section = 0 }
  in_section && /^version[[:space:]]*=/ {
    gsub(/"/, "", $3);
    print $3;
    exit
  }
' Cargo.toml)"
proto_version="$(sed -n 's/^pub const PROTO_VERSION: &str = "\(proto-v[0-9][^"]*\)";$/\1/p' \
  crates/determinism-proto/src/lib.rs)"

if [[ -z "${crate_version}" || "${crate_version}" == "null" ]]; then
  echo "error: failed to read determinism-proto crate version" >&2
  exit 1
fi

if [[ -z "${workspace_version}" || "${workspace_version}" == "null" ]]; then
  echo "error: failed to read workspace package version" >&2
  exit 1
fi

if [[ -z "${proto_version}" ]]; then
  echo "error: failed to read PROTO_VERSION" >&2
  exit 1
fi

expected_proto_version="proto-v${crate_version}"
if [[ "${proto_version}" != "${expected_proto_version}" ]]; then
  echo "error: PROTO_VERSION ${proto_version} does not match ${expected_proto_version}" >&2
  exit 1
fi

if [[ "${workspace_version}" != "${crate_version}" ]]; then
  echo "error: workspace version ${workspace_version} does not match crate version ${crate_version}" >&2
  exit 1
fi

if [[ "${GITHUB_REF_TYPE:-}" == "tag" && "${GITHUB_REF_NAME:-}" != "${proto_version}" ]]; then
  echo "error: tag ${GITHUB_REF_NAME:-<unset>} does not match ${proto_version}" >&2
  exit 1
fi

echo "proto version check passed: ${proto_version}"
