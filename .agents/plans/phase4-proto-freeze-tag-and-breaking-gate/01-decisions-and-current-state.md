# Decisions and Current State

## Current-state traps

- The request's `01-current-state.md` describes `orchestrator/v1` as an
  empty placeholder. That is no longer true in this checkout. Commit
  `9cb1a0c` replaced it with the real upstreamed surface and extended the
  `orchestrator` feature to tonic codegen. Preserve that wire shape unless
  the phases track explicitly signs off on a pre-tag rename/renumber.
- `docs/` and `phases/` are absent in this checkout even though the request
  cites them. Use the request directory as the local authority unless those
  docs appear on the branch being implemented.
- Root proto copies for `scorer`, `inputsynth`, and `orchestrator` currently
  match the packaged copies under `crates/determinism-proto/proto/`. Keep
  that invariant.
- `reference-workload/crates/m0-proto-client` uses the `controlplane` and
  `scorer` features and currently constructs `ExperimentSpec { seed: 1,
  ..Default::default() }`. Re-run that downstream after the mirror rework.

## D1: vdev policy mechanism

Use explicit Buf breaking ignore paths for pre-release placeholder families.
Do not rename their packages to `vdev` for this request.

Reasoning: Buf's package-version lint expects package suffixes shaped like
`v1`, `v1alpha1`, etc. A literal `vdev` package would require either broader
lint exceptions or path/package churn. The request explicitly allows
breaking ignore paths, and that is the least invasive way to avoid freezing
unowned placeholders before their owners upstream real schemas.

## D2: frozen vs pre-release packages at `proto-v0.2.0`

Frozen by the tag after this work:

- `determinism.common.v1`
- `determinism.controlplane.v1` after the `ExperimentSpec` mirror lands
- `determinism.orchestrator.v1`
- `determinism.scorer.v1`
- `determinism.inputsynth.v1`

Pre-release/vdev and ignored by `buf breaking` at this tag:

- `determinism.hypervisor.v1`
- `determinism.snapstore.v1`
- `determinism.policy.v1`
- `determinism.replay.v1`
- `determinism.replay.agent.v1`
- `determinism.observatory.v1`

Record this in a committed ledger, preferably
`docs/proto-freeze-policy.md`. The round-2 snapstore request can later build
the promotion playbook against that ledger.

## D3: Buf lint posture

Use current Buf v2 config. Current Buf docs name the recommended lint set
`STANDARD`; the request and older notes say `DEFAULT`. If the pinned CLI
accepts both, prefer the current documented name and note that it is the
same intended posture. If the pinned CLI only accepts one spelling, use the
one it accepts and record it in the resolution.

Make lint green by this priority order:

1. Fix harmless style issues in proto files before the first tag.
2. For real contract surfaces where a lint-motivated change would alter a
   deliberate wire/gRPC/Rust API shape, add file-scoped `ignore_only` entries
   with comments explaining the contract reason.
3. For pre-release/vdev placeholders, prefer minimal cleanup, but it is
   acceptable to scope lint ignores to those files while they remain
   unfrozen.

The orchestrator file already has a documented exemption request in
`06-orchestrator-upstream-notes.md` for:

- `ENUM_ZERO_VALUE_SUFFIX`
- `SERVICE_SUFFIX`
- `RPC_RESPONSE_STANDARD_NAME`

Do not rename `ExplorationOrchestrator` just to satisfy lint; that changes
gRPC method paths.

## D4: `ExperimentSpec` mirror shape

Implement the mirror by duplicating the `ExperimentConfig` transitive closure
inside `determinism.controlplane.v1`, with `ExperimentSpec` matching
`ExperimentConfig` field-for-field except for the root message name.

Do not embed `determinism.orchestrator.v1.ExperimentConfig` unless the
implementer deliberately revisits this decision. Embedding is allowed by the
coordination note, but duplication is the safer default for this crate
because `controlplane` is currently a handwritten facade feature with no
generated-code dependency. Embedding would likely force feature dependency
changes between `controlplane` and `orchestrator`.

The mirror scope is:

- Messages: `ExperimentSpec`/`ExperimentConfig`, `Budgets`,
  `SelectionConfig`, `StagedConfig`, `BurstConfig`, `PlateauConfig`,
  `LadderConfig`, `SchedulingConfig`, `CheckpointConfig`
- Enums: `PruneAction`, `OnGoal`, `PolicyKind`, `SchedMode`

Field names, numbers, labels, scalar types, enum references, and message
references must match after mapping:

- `determinism.controlplane.v1.ExperimentSpec` ->
  `determinism.orchestrator.v1.ExperimentConfig`
- `determinism.controlplane.v1.*` ->
  `determinism.orchestrator.v1.*`

## D5: Descriptor-equality implementation

Use structured descriptors, not text grep.

Recommended implementation:

- Add a Rust integration test
  `crates/determinism-proto/tests/experiment_spec_mirror.rs`.
- Use `protoc-bin-vendored` to produce a descriptor set from:
  - `proto/determinism/controlplane/v1/resources.proto`
  - `proto/determinism/orchestrator/v1/orchestrator.proto`
- Decode with `prost_types::FileDescriptorSet`.
- Compare the transitive closure listed in D4 after package/name mapping.
- Ignore comments/source locations and JSON names; compare semantic
  descriptor fields that affect the contract.

This test must run in normal `cargo test --workspace --all-features` CI.

## D6: aarch64 CI choice

Prefer a native GitHub-hosted arm64 runner (`ubuntu-24.04-arm` or the current
supported equivalent) for the Rust build/test lane. If that runner is not
available to this repository, use self-hosted arm64 or QEMU/cross as an
interim lane and record the limitation in `04-resolution.md`.

The acceptance target is build and test green on both x86_64 and aarch64.
A build-only arm64 lane is not enough unless explicitly recorded as interim.

## D7: version and tag policy

Keep `0.2.0`.

Do not bump to `0.3.0` merely because pre-tag proto files changed. The request
states that `0.2.0` is the first released tag and is correct unless the work
breaks an existing Rust API consumer in a way that cannot be avoided. The
orchestrator coordination note says there are no current consumers of
`determinism_proto::orchestrator` outside the resolved cutover.

Update the workspace package version to `0.2.0` unless the implementer has a
specific reason to exempt it. The CI drift check must compare:

- `crates/determinism-proto/Cargo.toml` package version
- `Cargo.toml` workspace package version, or an explicit documented
  exemption
- `PROTO_VERSION`
- tag name on `proto-v*` tag-triggered runs
