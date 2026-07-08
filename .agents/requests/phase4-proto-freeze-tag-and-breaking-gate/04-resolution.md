# Resolution: Phase-4 proto freeze, breaking gate, and tag

Status: complete. The implementation landed on `main`, required local and
remote gates passed, the scratch breaking demonstration produced the expected
red Buf gate, and `proto-v0.2.0` was tagged and verified.

## Workstream commits

| Workstream | Commit | Notes |
|---|---:|---|
| W1 Buf gates and vdev policy | `51d74ca` | Added `buf.yaml`, vdev ledger, breaking baseline script, and standing negative fixture self-test. |
| W2 CI/aarch64/version guards | `51d74ca` | Split CI into `proto` and Rust x86_64/aarch64 matrix jobs; added tag trigger and version drift script. |
| W3 ExperimentSpec mirror | `51d74ca` | Replaced the old controlplane `ExperimentSpec` stub with the orchestrator `ExperimentConfig` mirror. |
| W4 Descriptor/downstream checks | `51d74ca` | Added descriptor-equality test and verified sibling smoke tests locally. |
| W5 Tag handoff | `1a9fb94` / `proto-v0.2.0` | Main CI is green, scratch breaking evidence is captured, and the release tag is published. |

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
- Main CI URL:
  https://github.com/preestablished/control-plane/actions/runs/28914073266
- Tag CI URL:
  https://github.com/preestablished/control-plane/actions/runs/28914115796

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
- `scripts/buf-breaking-against.sh`
- `scripts/check-buf-breaking-self-test.sh`
- `scripts/check-proto-version.sh`
- Root/package proto copy checks for scorer, inputsynth, and orchestrator

Current post-tag breaking baseline:

- `scripts/buf-breaking-against.sh` compares against `proto-v0.2.0`, which is
  now the latest `proto-v*` tag, and the local gate passes.

Passed locally from sibling checkouts:

- `../reference-workload`: `cargo test -p refwork-m0-proto-client`
- `../exploration-orchestrator`: `cargo test --workspace --all-features`

## Scratch breaking demonstration

Captured before tag:

- Scratch PR: https://github.com/preestablished/control-plane/pull/3
- Scratch branch name: `scratch/proto-breaking-scorer-return-decoded`
- Frozen field changed:
  `proto/determinism/scorer/v1/scorer.proto`
  `ScoreBatchRequest.return_decoded = 5` was deleted.
- Failing CI URL:
  https://github.com/preestablished/control-plane/actions/runs/28913986693/job/85776976986
- Buf failure message:
  previously present field `5` with name `return_decoded` on message
  `ScoreBatchRequest` was deleted.
- Cleanup: PR #3 was closed and the remote scratch branch was deleted after
  evidence was captured.

## Tag

- Tag name: `proto-v0.2.0`
- Target SHA: `1a9fb946b48f6bf5b328823a5e2004aa075ff79c`
- Tag-triggered CI URL:
  https://github.com/preestablished/control-plane/actions/runs/28914115796

## Downstream notifications

Notify downstream consumers:

- snapshot-store, because its CI pin predates Phase-4 contracts
- reference-workload
- exploration-orchestrator
- future state-scorer/input-synthesizer bootstrap owners
