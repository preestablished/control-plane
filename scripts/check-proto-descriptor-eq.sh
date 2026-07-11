#!/usr/bin/env bash
set -euo pipefail

fixture_root="ci/vdev-promotion-fixtures"
shared_source="${fixture_root}/control/shared"

compare() {
  cargo run --quiet -p proto-descriptor-eq -- "$@"
}

compare \
  --owner-root "${fixture_root}/owner" \
  --owner-file scratch_owner.proto \
  --control-root "${fixture_root}/control" \
  --control-file determinism/scratch/v1/scratch.proto

for mismatch in mismatch-field mismatch-import mismatch-option; do
  temporary_root="$(mktemp -d)"
  trap 'rm -rf "${temporary_root}"' EXIT
  mkdir -p "${temporary_root}/shared" "${temporary_root}/determinism/scratch/v1"
  cp -f "${shared_source}/options.proto" "${temporary_root}/shared/options.proto"
  cp -f "${fixture_root}/${mismatch}/determinism/scratch/v1/scratch.proto" \
    "${temporary_root}/determinism/scratch/v1/scratch.proto"

  if compare \
    --owner-root "${fixture_root}/owner" \
    --owner-file scratch_owner.proto \
    --control-root "${temporary_root}" \
    --control-file determinism/scratch/v1/scratch.proto >/dev/null 2>&1; then
    echo "error: descriptor comparator accepted ${mismatch}" >&2
    exit 1
  fi
  rm -rf "${temporary_root}"
  trap - EXIT
  echo "descriptor comparator rejected ${mismatch} as expected"
done
