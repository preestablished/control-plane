#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
fixture_root="${repo_root}/ci/vdev-promotion-fixtures"
export CARGO_TARGET_DIR="${repo_root}/target/vdev-promotion-dry-run"
before_tags="$(git -C "${repo_root}" for-each-ref --format='%(refname) %(objectname)' refs/tags)"
before_policy="$(sha256sum "${repo_root}/docs/proto-freeze-policy.md" "${repo_root}/buf.yaml")"

echo "dry-run repository: $(git -C "${repo_root}" rev-parse HEAD)"
echo "rustc: $(rustc --version)"
echo "cargo: $(cargo --version)"
echo "buf: $(buf --version)"
cargo build --quiet -p proto-descriptor-eq --manifest-path "${repo_root}/Cargo.toml"
comparator="${CARGO_TARGET_DIR}/debug/proto-descriptor-eq"

run_once() {
  local run_number="$1"
  local temporary_root
  temporary_root="$(mktemp -d)"
  if [[ -n "${KEEP_VDEV_PROMOTION_TMP:-}" ]]; then
    echo "run ${run_number} temporary repository retained at ${temporary_root}"
  else
    trap 'rm -rf "${temporary_root}"' RETURN
  fi

  echo "run ${run_number}: create isolated placeholder repository"
  mkdir -p \
    "${temporary_root}/crates/determinism-proto/src" \
    "${temporary_root}/crates/determinism-proto/proto/determinism/scratch/v1" \
    "${temporary_root}/consumer/src" \
    "${temporary_root}/proto/determinism/scratch/v1"
  cp -f "${fixture_root}/harness/Cargo.toml" "${temporary_root}/Cargo.toml"
  cp -f "${fixture_root}/harness/buf.yaml" "${temporary_root}/buf.yaml"
  cp -f "${fixture_root}/harness/pre/Cargo.toml" \
    "${temporary_root}/crates/determinism-proto/Cargo.toml"
  cp -f "${fixture_root}/harness/pre/build.rs" \
    "${temporary_root}/crates/determinism-proto/build.rs"
  cp -f "${fixture_root}/harness/pre/lib.rs" \
    "${temporary_root}/crates/determinism-proto/src/lib.rs"
  cp -f "${fixture_root}/harness/placeholder.proto" \
    "${temporary_root}/proto/determinism/scratch/v1/scratch.proto"
  cp -f "${fixture_root}/harness/placeholder.proto" \
    "${temporary_root}/crates/determinism-proto/proto/determinism/scratch/v1/scratch.proto"
  cp -f "${fixture_root}/harness/consumer/Cargo.toml" \
    "${temporary_root}/consumer/Cargo.toml"
  cp -f "${fixture_root}/harness/consumer/main.rs" \
    "${temporary_root}/consumer/src/main.rs"
  cp -f "${repo_root}/scripts/check-proto-version.sh" \
    "${temporary_root}/check-proto-version.sh"
  printf '%s\n' '# Scratch freeze ledger' '' '- pre-release: determinism.scratch.v1' \
    >"${temporary_root}/proto-freeze-policy.md"

  git -C "${temporary_root}" init -q
  git -C "${temporary_root}" config user.name "vdev-promotion-dry-run"
  git -C "${temporary_root}" config user.email "dry-run@example.invalid"
  git -C "${temporary_root}" add .
  git -C "${temporary_root}" commit -qm "placeholder baseline"
  git -C "${temporary_root}" tag -a proto-v0.0.0 -m proto-v0.0.0

  cargo check --quiet --manifest-path "${temporary_root}/Cargo.toml" \
    -p scratch-consumer --bin scratch-consumer
  echo "run ${run_number}: handwritten stable-seam consumer compiled"

  echo "run ${run_number}: stage owner schema while family remains ignored"
  mkdir -p \
    "${temporary_root}/proto/shared" \
    "${temporary_root}/crates/determinism-proto/proto/shared"
  cp -f "${fixture_root}/control/determinism/scratch/v1/scratch.proto" \
    "${temporary_root}/proto/determinism/scratch/v1/scratch.proto"
  cp -f "${fixture_root}/control/shared/options.proto" \
    "${temporary_root}/proto/shared/options.proto"
  cp -f "${fixture_root}/control/determinism/scratch/v1/scratch.proto" \
    "${temporary_root}/crates/determinism-proto/proto/determinism/scratch/v1/scratch.proto"
  cp -f "${fixture_root}/control/shared/options.proto" \
    "${temporary_root}/crates/determinism-proto/proto/shared/options.proto"
  cp -f "${fixture_root}/harness/post/Cargo.toml" \
    "${temporary_root}/crates/determinism-proto/Cargo.toml"
  cp -f "${fixture_root}/harness/post/build.rs" \
    "${temporary_root}/crates/determinism-proto/build.rs"
  cp -f "${fixture_root}/harness/post/lib.rs" \
    "${temporary_root}/crates/determinism-proto/src/lib.rs"
  mkdir -p "${temporary_root}/consumer/src/bin"
  cp -f "${fixture_root}/harness/consumer/generated.rs" \
    "${temporary_root}/consumer/src/bin/generated.rs"
  sed -i 's/version = "0.0.0"/version = "0.0.1"/' "${temporary_root}/Cargo.toml"

  "${comparator}" \
    --owner-root "${fixture_root}/owner" --owner-file scratch_owner.proto \
    --control-root "${temporary_root}/proto" \
    --control-file determinism/scratch/v1/scratch.proto
  "${repo_root}/scripts/check-proto-descriptor-eq.sh"
  buf breaking "${temporary_root}/proto" \
    --against "${temporary_root}/.git#tag=proto-v0.0.0,subdir=proto"
  cargo check --quiet --manifest-path "${temporary_root}/Cargo.toml" \
    -p determinism-proto --no-default-features
  cargo check --quiet --manifest-path "${temporary_root}/Cargo.toml" \
    -p determinism-proto --no-default-features --features scratch
  cargo check --quiet --manifest-path "${temporary_root}/Cargo.toml" \
    -p determinism-proto --all-features
  cargo run --quiet --manifest-path "${temporary_root}/Cargo.toml" \
    -p scratch-consumer --bin scratch-consumer
  cargo run --quiet --manifest-path "${temporary_root}/Cargo.toml" \
    -p scratch-consumer --bin generated
  git -C "${temporary_root}" add .
  git -C "${temporary_root}" commit -qm "stage real scratch schema"
  (cd "${temporary_root}" && GITHUB_REF_TYPE=tag GITHUB_REF_NAME=proto-v0.0.1 \
    bash ./check-proto-version.sh)
  git -C "${temporary_root}" tag -a proto-v0.0.1 -m proto-v0.0.1
  echo "run ${run_number}: staging tag proto-v0.0.1 is not an adoption signal"

  echo "run ${run_number}: freeze schema unchanged against staging tag"
  sed -i '/^  ignore:$/,/proto\/determinism\/scratch\/v1$/d' \
    "${temporary_root}/buf.yaml"
  sed -i 's/pre-release:/frozen:/' "${temporary_root}/proto-freeze-policy.md"
  sed -i 's/version = "0.0.1"/version = "0.0.2"/' \
    "${temporary_root}/Cargo.toml" \
    "${temporary_root}/crates/determinism-proto/Cargo.toml"
  sed -i 's/proto-v0.0.1/proto-v0.0.2/' \
    "${temporary_root}/crates/determinism-proto/src/lib.rs"
  "${comparator}" \
    --owner-root "${fixture_root}/owner" --owner-file scratch_owner.proto \
    --control-root "${temporary_root}/proto" \
    --control-file determinism/scratch/v1/scratch.proto
  buf breaking "${temporary_root}/proto" \
    --against "${temporary_root}/.git#tag=proto-v0.0.1,subdir=proto"
  git -C "${temporary_root}" add .
  git -C "${temporary_root}" commit -qm "freeze scratch schema"
  (cd "${temporary_root}" && GITHUB_REF_TYPE=tag GITHUB_REF_NAME=proto-v0.0.2 \
    bash ./check-proto-version.sh)
  git -C "${temporary_root}" tag -a proto-v0.0.2 -m proto-v0.0.2

  sed -i '/optional string note = 2;/d' \
    "${temporary_root}/proto/determinism/scratch/v1/scratch.proto"
  if buf breaking "${temporary_root}/proto" \
    --against "${temporary_root}/.git#tag=proto-v0.0.2,subdir=proto" \
    >"${temporary_root}/expected-breaking.log" 2>&1; then
    echo "error: post-freeze field deletion unexpectedly passed" >&2
    exit 1
  fi
  grep -E 'FIELD|field|note' "${temporary_root}/expected-breaking.log" | head -5 || \
    sed -n '1,5p' "${temporary_root}/expected-breaking.log"
  echo "run ${run_number}: post-freeze field deletion failed as expected"
}

run_once 1
run_once 2

after_tags="$(git -C "${repo_root}" for-each-ref --format='%(refname) %(objectname)' refs/tags)"
after_policy="$(sha256sum "${repo_root}/docs/proto-freeze-policy.md" "${repo_root}/buf.yaml")"
if [[ "${before_tags}" != "${after_tags}" ]]; then
  echo "error: caller tag refs changed during dry run" >&2
  exit 1
fi
if [[ "${before_policy}" != "${after_policy}" ]]; then
  echo "error: caller freeze policy or Buf configuration changed during dry run" >&2
  exit 1
fi
echo "dry run passed twice; caller tags and policy files are unchanged"
