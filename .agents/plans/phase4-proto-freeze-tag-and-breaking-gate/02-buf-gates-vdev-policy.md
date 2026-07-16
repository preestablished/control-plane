# Buf Gates and vdev Policy

Workstream W1.

## W1.1: Add the committed policy ledger

Create `docs/proto-freeze-policy.md` with:

- The frozen package list from D2.
- The pre-release/vdev package list from D2.
- The exact `buf.yaml` paths ignored for breaking detection.
- The rule: vdev packages may be linted, but they are not protected by
  breaking detection until a future owner-authored promotion moves them into
  the frozen ledger.
- The rule: promoting a vdev family requires updating this ledger and
  removing its breaking ignore path in the same change that freezes it.
- A pointer to
  `.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/` as the
  first known future consumer of the policy.

Keep the doc short and operational. It is a ledger, not a design essay.

## W1.2: Add `buf.yaml`

Add root `buf.yaml`. Start from this shape and adjust only after running the
pinned Buf CLI:

```yaml
version: v2
modules:
  - path: proto
lint:
  use:
    - STANDARD
  ignore_only:
    ENUM_ZERO_VALUE_SUFFIX:
      - proto/determinism/orchestrator/v1/orchestrator.proto
    SERVICE_SUFFIX:
      - proto/determinism/orchestrator/v1/orchestrator.proto
    RPC_RESPONSE_STANDARD_NAME:
      - proto/determinism/orchestrator/v1/orchestrator.proto
breaking:
  use:
    - FILE
  ignore:
    - proto/determinism/hypervisor/v1
    - proto/determinism/snapstore/v1
    - proto/determinism/policy/v1
    - proto/determinism/replay/v1
    - proto/determinism/observatory/v1
```

Notes:

- If the pinned CLI rejects `STANDARD`, use the equivalent accepted category
  and record the spelling in the resolution.
- The `replay/v1` ignore path intentionally covers both
  `determinism.replay.v1` and the current `reexec_agent.proto` package
  `determinism.replay.agent.v1`; if that package is moved to
  `proto/determinism/replay/agent/v1/`, update the ignore path and ledger.
- Add further `lint.ignore_only` entries only after running `buf lint` and
  deciding whether each finding is harmless to fix or contract-sensitive.
- Do not put frozen packages in `breaking.ignore`.

## W1.3: Make lint green

Run:

```bash
buf lint
```

Triage findings in this order:

1. Frozen packages:
   - Fix style issues that do not change intended wire semantics or gRPC
     paths.
   - For `scorer/v1` and `inputsynth/v1`, be careful with service names and
     enum value names. They are Phase-4 contracts even though they are not
     tagged yet. If changing them would surprise current local consumers or
     contradict request text, prefer scoped `ignore_only` with a rationale.
   - For orchestrator, use the three scoped exemptions from
     `06-orchestrator-upstream-notes.md`.
2. vdev packages:
   - Minimal cleanup is fine.
   - Scoped lint exemptions are fine while they are pre-release.
   - Do not promote a vdev package into frozen coverage in this request.

After every proto edit, re-run:

```bash
cargo test --workspace --all-features
```

For generated-feature protos, also confirm root and packaged copies still
match:

```bash
cmp -s proto/determinism/scorer/v1/scorer.proto crates/determinism-proto/proto/determinism/scorer/v1/scorer.proto
cmp -s proto/determinism/inputsynth/v1/synthesizer.proto crates/determinism-proto/proto/determinism/inputsynth/v1/synthesizer.proto
cmp -s proto/determinism/orchestrator/v1/orchestrator.proto crates/determinism-proto/proto/determinism/orchestrator/v1/orchestrator.proto
```

## W1.4: Add breaking baseline script

Add `scripts/buf-breaking-against.sh` and run it from CI instead of inlining
baseline logic in YAML.

Required behavior:

- Use `set -euo pipefail`.
- Fetch tags in CI; the workflow checkout must use `fetch-depth: 0`.
- If a `proto-v*` tag exists, compare against the highest semver-ish tag:
  `buf breaking proto --against ".git#tag=${tag},subdir=proto"`.
- If no `proto-v*` tag exists, compare against the merge-base with
  `origin/main`:
  `buf breaking proto --against ".git#ref=${merge_base},subdir=proto"`.
- Print the chosen baseline before running Buf.
- Fail closed if the baseline cannot be determined.

The no-tag fallback is for bootstrapping PRs before `proto-v0.2.0`. Once the
tag exists, the tag must be the baseline.

## W1.5: Add the standing negative fixture self-test

Add fixture directories that are not real protos:

```text
ci/buf-breaking-fixtures/
  baseline/determinism/fixture/v1/fixture.proto
  broken/determinism/fixture/v1/fixture.proto
```

The baseline fixture should contain a simple message with at least two fields.
The broken fixture should delete or renumber one released field. The package
can be `determinism.fixture.v1`.

Add `scripts/check-buf-breaking-self-test.sh`:

- Run `buf breaking ci/buf-breaking-fixtures/broken --against
  ci/buf-breaking-fixtures/baseline`.
- Expect that command to fail.
- If it succeeds, print a clear error and exit nonzero.
- If it fails, print the captured Buf output and exit zero.

This proves CI would catch a known breaking change without mutating real
contract files.

## W1.6: Wire Buf into CI

In `.github/workflows/ci.yaml`:

- Use `actions/checkout@v4` with `fetch-depth: 0`.
- Install a pinned Buf CLI with `bufbuild/buf-action@v1` and
  `setup_only: true`.
- Set an explicit `version`; do not use `latest`.
- Run:
  - `buf --version`
  - `buf lint`
  - `scripts/buf-breaking-against.sh`
  - `scripts/check-buf-breaking-self-test.sh`

Keep the Buf gate in a separate job or a clearly named step group so failures
are obvious in GitHub Actions. The Rust matrix can depend on it or run in
parallel; either is fine as long as all required checks are green before the
tag.
