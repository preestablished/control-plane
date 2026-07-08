# Resolution: Phase-4 proto freeze, breaking gate, and tag

Status: local implementation and verification are complete through the code,
proto, Buf, and downstream smoke-test workstreams. Remote CI, the scratch
breaking demonstration, and the release tag remain pending and must be filled
before this request is closed.

## Workstream commits

| Workstream | Commit | Notes |
|---|---:|---|
| W1 Buf gates and vdev policy | `51d74ca` | Added `buf.yaml`, vdev ledger, breaking baseline script, and standing negative fixture self-test. |
| W2 CI/aarch64/version guards | `51d74ca` | Split CI into `proto` and Rust x86_64/aarch64 matrix jobs; added tag trigger and version drift script. |
| W3 ExperimentSpec mirror | `51d74ca` | Replaced the old controlplane `ExperimentSpec` stub with the orchestrator `ExperimentConfig` mirror. |
| W4 Descriptor/downstream checks | `51d74ca` | Added descriptor-equality test and verified sibling smoke tests locally. |
| W5 Tag handoff | pending | Requires remote CI green, scratch breaking demo evidence, and `proto-v0.2.0` tag creation. |

## Buf gate

- Pinned Buf CLI version: `1.71.0`.
- Lint category spelling: `STANDARD`.
- `buf lint` passes locally with scoped `ignore_only` entries for
  contract-sensitive frozen service/enum names and vdev placeholder layout.
- Standing negative fixture:
  `scripts/check-buf-breaking-self-test.sh` fails on the expected deleted field
  and exits successfully.

## vdev ledger

Frozen by `proto-v0.2.0`:

- `determinism.common.v1`
- `determinism.controlplane.v1`
- `determinism.orchestrator.v1`
- `determinism.scorer.v1`
- `determinism.inputsynth.v1`

Pre-release/vdev and ignored by `buf breaking`:

- `determinism.hypervisor.v1`
- `determinism.snapstore.v1`
- `determinism.policy.v1`
- `determinism.replay.v1`
- `determinism.replay.agent.v1`
- `determinism.observatory.v1`

## CI and architecture

- x86_64 mechanism: GitHub-hosted `ubuntu-latest`.
- aarch64 mechanism: native GitHub-hosted `ubuntu-24.04-arm`.
- Neither lane is marked `continue-on-error`.
- Remote CI URLs: pending.

## Descriptor mirror

- Decision: duplicate
  `determinism.orchestrator.v1.ExperimentConfig` and its transitive closure
  inside `determinism.controlplane.v1`, with the root renamed to
  `ExperimentSpec`.
- Local descriptor-equality test:
  `crates/determinism-proto/tests/experiment_spec_mirror.rs`.

## Local verification

Passed locally from `control-plane`:

- `cargo fmt --all -- --check`
- `cargo build --workspace --all-features`
- `cargo test --workspace --all-features`
- `buf lint` with Buf `1.71.0`
- `scripts/check-buf-breaking-self-test.sh`
- `scripts/check-proto-version.sh`
- Root/package proto copy checks for scorer, inputsynth, and orchestrator

Known pre-tag bootstrap state:

- `scripts/buf-breaking-against.sh` currently compares against merge-base
  `261141b3bbaa4371a7dd4147ac6626e0f4918e53` because no `proto-v*` tag exists
  and `origin/main` has not advanced to the final freeze commit. That correctly
  reports the intentional pre-tag controlplane/orchestrator contract changes.
  Re-run this gate after the final commit is on `main`, or after
  `proto-v0.2.0` exists as the baseline.

Passed locally from sibling checkouts:

- `../reference-workload`: `cargo test -p refwork-m0-proto-client`
- `../exploration-orchestrator`: `cargo test --workspace --all-features`

## Scratch breaking demonstration

Pending. Required before tag:

- Scratch branch name: pending
- Frozen field changed: pending
- Failing CI URL: pending

## Tag

- Tag name: `proto-v0.2.0`
- Target SHA: pending
- Tag-triggered CI URL: pending

## Downstream notifications

Notify after final tag:

- snapshot-store, because its CI pin predates Phase-4 contracts
- reference-workload
- exploration-orchestrator
- future state-scorer/input-synthesizer bootstrap owners
