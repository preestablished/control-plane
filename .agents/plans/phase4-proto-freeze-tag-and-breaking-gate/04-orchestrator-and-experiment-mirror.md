# Orchestrator Confirmation and `ExperimentSpec` Mirror

Workstreams W3 and W4.

## W3.1: Confirm orchestrator upstream state

The real orchestrator proto is already present in this checkout. Verify:

```bash
cmp -s proto/determinism/orchestrator/v1/orchestrator.proto crates/determinism-proto/proto/determinism/orchestrator/v1/orchestrator.proto
cargo test --workspace --all-features
```

Also inspect:

- `crates/determinism-proto/build.rs` includes
  `CARGO_FEATURE_ORCHESTRATOR`, the orchestrator proto path, and packaged
  copy staleness checking.
- `crates/determinism-proto/Cargo.toml` has the optional prost/tonic deps
  needed by the generated orchestrator code.
- `crates/determinism-proto/src/lib.rs` exposes
  `tonic::include_proto!("determinism.orchestrator.v1")` under
  `orchestrator::v1`.
- Existing orchestrator facade tests exercise `ExperimentConfig`,
  `ProgressEvent`, and client type generation.

If any of these are missing on the implementation branch, restore the pattern
from the current local commit `9cb1a0c`. Do not change the orchestrator wire
shape for lint unless the phases track explicitly signs off.

## W3.2: Rework `controlplane/v1/resources.proto`

Replace the old `Budgets`, `BurstParams`, and `ExperimentSpec` stub in
`proto/determinism/controlplane/v1/resources.proto` with the field-for-field
mirror of `determinism.orchestrator.v1.ExperimentConfig`.

Expected root message:

```proto
message ExperimentSpec {
  uint32 version = 1;
  uint64 seed = 2;
  string workload_image_ref = 3;
  string feature_map_ref = 4;
  string scoring_program_ref = 5;
  string synth_config_ref = 6;
  repeated string macro_pack_refs = 7;
  Budgets budgets = 8;
  SelectionConfig selection = 9;
  BurstConfig burst = 10;
  PlateauConfig plateau = 11;
  SchedulingConfig scheduling = 12;
  CheckpointConfig checkpoint = 13;
  PruneAction prune_action = 14;
  OnGoal on_goal = 15;
  repeated string decoded_features = 16;
}
```

Then duplicate the transitive closure from orchestrator with matching field
numbers and types:

- `PruneAction`
- `OnGoal`
- `PolicyKind`
- `SchedMode`
- `Budgets`
- `SelectionConfig`
- `StagedConfig`
- `BurstConfig`
- `PlateauConfig`
- `LadderConfig`
- `SchedulingConfig`
- `CheckpointConfig`

The old `BurstParams` shape is not part of the mirror. Removing it before the
first tag is acceptable. If another file imports it on the implementation
branch, stop and decide whether to update that consumer or preserve a
deprecated compatibility facade outside the proto contract.

## W3.3: Update the handwritten Rust facade

Update `crates/determinism-proto/src/lib.rs` under
`#[cfg(feature = "controlplane")]` to mirror the new proto shape.

Keep the local handwritten style unless the implementer deliberately converts
`controlplane` to generated code. Recommended:

- Define Rust structs for the message closure listed in W3.2.
- Define Rust enums for `PruneAction`, `OnGoal`, `PolicyKind`, and
  `SchedMode` with stable integer discriminants matching proto values.
- Keep `ExperimentSpec { seed: 1, ..Default::default() }` compiling if
  possible; this preserves the current reference-workload smoke shape.
- Add facade tests for defaults and representative nested config
  construction.

If using generated controlplane code instead, revisit feature dependencies
carefully. Avoid a `controlplane` <-> `orchestrator` feature cycle.

## W3.4: Add descriptor-equality test

Add `crates/determinism-proto/tests/experiment_spec_mirror.rs`.

Test algorithm:

1. Locate the repository root from `CARGO_MANIFEST_DIR`.
2. Run vendored `protoc` with `--include_imports` and
   `--descriptor_set_out` against:
   - `proto/determinism/controlplane/v1/resources.proto`
   - `proto/determinism/orchestrator/v1/orchestrator.proto`
3. Decode the descriptor set with `prost_types::FileDescriptorSet`.
4. Find `determinism.controlplane.v1.ExperimentSpec` and
   `determinism.orchestrator.v1.ExperimentConfig`.
5. Recursively compare the D4 closure:
   - field name
   - field number
   - label
   - scalar type
   - enum/message reference after package/name mapping
   - oneof membership if any is introduced later
6. Fail on any missing or extra message/enum in the mirror scope.

Add dev-dependencies as needed, likely:

```toml
[dev-dependencies]
prost = "0.14.4"
prost-types = "0.14"
protoc-bin-vendored = "3.2.0"
tempfile = "3"
```

Do not compare raw text. Do not rely on comments.

## W4.1: Downstream and companion checks

After W3 lands locally, run:

```bash
cargo test --workspace --all-features
```

Then, from the sibling reference-workload checkout if present:

```bash
cargo test -p m0-proto-client
```

If `m0-proto-client` needs a source update because the facade changed, make
that update in the reference-workload repo and record the commit or required
patch in the resolution. The request expects this fallout to be handled, not
ignored.

For exploration-orchestrator, verify it still builds against this
control-plane checkout after the final proto and feature shape:

```bash
cargo test --workspace --all-features
```

If that repo is not available, record the skipped verification and why.

## W4.2: Deviation rule

If the implementer concludes strict field-for-field equality is wrong, stop.
That is a request-level deviation requiring phases-track sign-off and must be
recorded in the resolution before the tag. Do not quietly weaken the check.
