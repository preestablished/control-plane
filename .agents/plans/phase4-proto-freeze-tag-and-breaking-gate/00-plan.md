# Plan: Phase-4 Proto Freeze, Breaking Gate, and Tag

Answers:
`.agents/requests/phase4-proto-freeze-tag-and-breaking-gate/`

Baseline observed while writing this plan:

- Local `main` is at `9cb1a0c` with a clean worktree.
- `origin/main` is still at `261141b`; local `main` has already added
  `06-orchestrator-upstream-notes.md` and upstreamed the real
  `determinism.orchestrator.v1` proto/codegen.
- `git tag --list 'proto-v*'` is empty.
- There is no `buf.yaml`, no Buf CI, and no aarch64 lane.
- `crates/determinism-proto/Cargo.toml` is `0.2.0`,
  `crates/determinism-proto/src/lib.rs` exposes
  `PROTO_VERSION = "proto-v0.2.0"`, and the workspace package version is
  still `0.1.0`.
- `proto/determinism/controlplane/v1/resources.proto` still has the old
  stub `ExperimentSpec`, not the `ExperimentConfig` mirror.

The request snapshot was taken before the orchestrator upstream landed.
Do not replace the orchestrator file blindly. Treat item 3 as "confirm and
finish integrating the already-upstreamed surface into the gates".

## Objective

Bring M0 to the acceptance bar described by the request:

1. Add Buf lint and breaking-change gates, including a standing negative
   fixture test proving the breaking gate fails on a known break.
2. Commit the pre-release/vdev policy so placeholder families are not
   frozen by `proto-v0.2.0`.
3. Add aarch64 CI for `determinism-proto`.
4. Rework `controlplane/v1` `ExperimentSpec` to mirror orchestrator
   `ExperimentConfig`, then add a descriptor-equality check.
5. Add version/tag drift checks and publish `proto-v0.2.0` only after all
   gates are green and the scratch breaking demonstration is recorded.

## Files in this plan

| File | Purpose |
|---|---|
| `01-decisions-and-current-state.md` | Decisions the implementer should follow, plus current-state traps |
| `02-buf-gates-vdev-policy.md` | Buf config, vdev ledger, breaking baseline script, fixture self-test |
| `03-ci-aarch64-and-version-guards.md` | Rust CI matrix, arm64 lane, manifest/constant/tag drift assertions |
| `04-orchestrator-and-experiment-mirror.md` | Orchestrator confirmation, `ExperimentSpec` mirror, descriptor-equality test |
| `05-tag-handback-verification.md` | Final verification, scratch red-gate demo, tag, resolution handback |
| `06-acceptance-checklist.md` | Direct mapping from requested acceptance criteria to implementation evidence |

## Sequencing

Use this order unless a later current-state check proves a step has already
landed:

1. W0: Preflight from a clean tree. Confirm no tag exists, root and
   packaged generated protos match, and the orchestrator upstream commit is
   present.
2. W1: Add `buf.yaml`, the vdev policy ledger, Buf CI scripts, and the
   breaking-gate fixture self-test. Make lint pass with documented scoped
   exemptions only.
3. W2: Add CI version/tag guards and the aarch64 Rust lane.
4. W3: Rework `controlplane/v1` `ExperimentSpec` and its Rust facade.
5. W4: Add the descriptor-equality check and cross-repo compile checks.
6. W5: Land to `main`, run the scratch breaking demonstration, then tag
   `proto-v0.2.0` and record the resolution.

The tag is last. Do not create or push `proto-v0.2.0` while any item above
is missing, red, or only locally verified.

## Out of scope

- M1 and later control-plane work: server, registry, queue, auth, run
  lifecycle, `detctl`, databases, blob storage.
- Real schemas for the remaining placeholder families: snapstore,
  hypervisor, policy, replay, replay.agent, observatory.
- Publishing to a crate registry.
- The round-2 snapstore promotion playbook, except that this plan must leave
  snapstore unfrozen so that request can execute later.
