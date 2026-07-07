# Requested Work

## What We Need (Behavioral)

1. **`buf lint` + `buf breaking` in CI, with the plan's `vdev` policy.**
   Create `buf.yaml` (pin the buf version in CI), lint with the plan's
   named rules (package version suffix, enum zero-values, service
   suffix — expect the existing ~21 placeholder stubs to need cleanup or
   exemption), and run breaking-change detection against the most recent
   `proto-v*` tag (bootstrap: until the first tag exists, run against the
   merge-base with `main`; note `buf breaking --against '.git#tag=…'`
   needs full-depth checkout in Actions). **Before the first tag, decide
   the placeholder policy per the plan's risk-table escape hatch:** move
   the eight unowned placeholder families to `vdev` packages (or
   configure explicit `buf breaking` ignore paths for them) so that only
   fully-specified families — `scorer/v1`, `inputsynth/v1`,
   `orchestrator/v1` post-upstream, `common/v1`, and `controlplane/v1`
   post-item-4 — are frozen by the tag. Otherwise every future upstream
   (snapstore's real schema over its 12-line stub, hypervisor's, etc.)
   lands into a red gate by construction.
   Two proofs the gate works: (a) the plan's standing self-test — a
   fixtures-dir breaking change must fail the gate in CI on every run,
   without mutating real protos; (b) a one-time scratch-branch
   demonstration (delete a released field, watch CI go red), recorded in
   the resolution.
2. **aarch64 CI lane.** Build + test `determinism-proto` on aarch64
   (native runner or cross/QEMU — your call; record which). The plan's M0
   bar is build-green on both arches; it wants the full suite on aarch64
   from M1, so if you ship a build-only lane, flag it as interim.
3. **Receive the orchestrator upstream — before the tag.** The companion
   request has exploration-orchestrator replacing the placeholder
   `orchestrator/v1/orchestrator.proto` (empty service, no consumers)
   with their real surface, and extending this crate's `orchestrator`
   feature to real tonic codegen (they author the `build.rs`/Cargo
   changes in the upstream PR; you review). Ordering, explicitly: **CI
   gates (item 1, with placeholders exempted) land first, then the
   upstream lands as a lint-only concern, then the tag.** Give them
   layout/style feedback pre-merge, not as a post-tag break.
4. **`ExperimentSpec` mirror + descriptor check — the real scope.** The
   plan is unambiguous: `ExperimentSpec` sub-messages are a
   field-for-field mirror of the orchestrator's `ExperimentConfig`
   (orchestrator schema is the single source of truth; divergence is
   fixed control-plane-side; names/types/numbers must match or the gate
   fails). Today's `resources.proto` `ExperimentSpec` is a 21-line stub —
   so this item is a **rework of `controlplane/v1` `ExperimentSpec` to
   mirror the upstreamed `ExperimentConfig`**, plus the facade update in
   `lib.rs` and whatever falls out in `reference-workload`'s
   `m0-proto-client` (`controlplane` feature consumer), **sequenced
   before the tag** — after it, the mirror fix would itself be a
   released-field break. Then land the descriptor-equality CI check. If
   you conclude strict field-for-field equality is wrong, that is a
   disclosed deviation from the plan requiring phases-track sign-off in
   the resolution — not a quiet local decision.
5. **Publish `proto-v0.2.0`, last.** Tag once items 1–4 are green, so the
   tag freezes the tree Phase 4 actually builds on. The plan's M0
   acceptance literally names `proto-v0.1.0` — record that 0.1.0 was
   never tagged and 0.2.0 satisfies that item late. Versioning guidance:
   0.2.0 is correct regardless of what lands before it (the first tag
   freezes whatever is in tree; there is no released surface to "break"
   yet). Bump to 0.3.0 only if replacing the orchestrator facade breaks
   the crate's *Rust API* for existing `orchestrator`-feature consumers
   (item 3 says to avoid that). Add the drift assertion: crate-manifest
   version ↔ `PROTO_VERSION` checked on every CI run (note the workspace
   `Cargo.toml` still says 0.1.0 — fold it in or exempt it explicitly);
   tag ↔ `PROTO_VERSION` checked in a tag-triggered job that hard-fails
   on mismatch.

## Suggested Sequencing (Yours To Overrule)

1 (gates + vdev policy) → 2 (pure CI) → 3 + 4 with the orchestrator
(one coordinated window; their request is filed and aligned on this
ordering) → 5.

## Acceptance Criteria

1. CI on `main` runs buf lint + breaking (with the fixtures self-test) +
   aarch64 lanes; the vdev/ignore policy is committed and documented in
   the repo (which families are frozen, which are pre-1.0); the
   scratch-branch demonstration is recorded in the resolution.
2. `determinism.orchestrator.v1` (real surface) builds from this repo in
   both repos' CI; the placeholder is gone.
3. `ExperimentSpec` mirrors `ExperimentConfig` per the plan (or a
   disclosed, signed-off deviation is recorded); the descriptor check
   runs in CI.
4. Tag exists and matches `PROTO_VERSION` and the crate manifest, with CI
   assertions preventing future drift; sister repos notified — including
   snapshot-store explicitly (their CI pin `ca9ee90…` predates the
   Phase-4 contracts; the tag is what lets them re-pin sanely, and their
   `snapstore-8qx` swap additionally waits on a future snapstore-schema
   upstream that your vdev policy must leave room for).

## Out Of Scope For This Request

- **All of M1–M7.** Server, DB, auth, blob, queue, `detctl`, run
  lifecycle — Phase 6+ gated (`phases/README.md` matrix row); nothing
  here opens that door.
- Upstreaming the other placeholder families' real schemas (snapstore,
  hypervisor, replay, observatory...) — each lands on its owner's
  initiative, protected from premature freezing by the vdev policy;
  don't speculate their schemas.
- Publishing to a registry (crates.io / private) — tag-based git
  consumption is the plan's bar; registry publishing is a later
  operability question.
