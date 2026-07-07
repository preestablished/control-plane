# Current State (Evidence-Based)

Repo `main` at `261141b` ("publish phase4 proto contracts"), clean tree,
assessed 2026-07-07. Workspace contains exactly one crate:
`crates/determinism-proto`.

## What M0 Delivered

- **Phase-4 contracts, real and current.**
  `proto/determinism/scorer/v1/scorer.proto` (~250 lines) covers the full
  scorer chain: `ScoreBatch` (progress/novelty/`state_hash`/`goal_hit`/
  `stage`/`duplicate`), `LoadFeatureMap` with
  `ExtractRange {region, layout_version, offset, len}` mirroring the
  Phase 3 extraction-list contract, `LoadScoringProgram` (DSL: components,
  stages, goal_expr), `CheckpointArchive`/`RestoreArchive`/`ReplayCommits`,
  `Stats` with the latency histogram the M4 budget gate needs.
  `proto/determinism/inputsynth/v1/synthesizer.proto` (~230 lines) covers
  pad model, seeded `ProposeBursts`, `LoadMacroPack` + provenance,
  mutation ops, `MineMacros`. Both compile as real tonic code
  (`build.rs`), with facade tests.
- **Eight placeholder families by design** (common, hypervisor, snapstore,
  orchestrator, policy, replay, observatory, controlplane — the crate's
  ten families minus the two generated ones), ~21 thin `.proto` stubs on
  disk, including a real `orchestrator/v1/orchestrator.proto` placeholder
  file with an empty `service ExplorationOrchestrator {}`. Sister repos
  supersede them with their own generated crates (`dh-proto`,
  `orch-proto`, etc.). Acceptable for M0 *only because* nothing enforces
  their shape yet — which is exactly why the tag must not freeze them
  (see the `vdev` policy in `02-`).

## The M0 Acceptance Gaps

Against `docs/control-plane/IMPLEMENTATION-PLAN.md` §M0:

1. **No `buf lint` / `buf breaking` gate.** `.github/workflows/ci.yaml`
   runs `cargo fmt/build/test` only. The plan's M0 CI acceptance mandates
   both, its testing matrix requires a standing fixtures-based self-test
   of the breaking gate, and its risk table's answer to "gate blocks
   evolution" is the pre-1.0 `vdev` package exemption — none of which
   exist; the phases standing rule ("released fields are never broken",
   `phases/README.md`) currently relies on nobody making a mistake.
2. **No `proto-v*` tag.** `git tag` is empty while `src/lib.rs:6`
   declares `PROTO_VERSION = "proto-v0.2.0"` and `Cargo.toml` says
   `0.2.0`. The M0 acceptance item "tag-based consumption" is unmet.
3. **No aarch64 CI.** Matrix is `ubuntu-latest` only; the plan mandates
   x86_64 + aarch64 from M0 (DGX Spark deployment target for both
   control-plane and the scorer).
4. **No descriptor-equality check** between `controlplane/v1`
   `ExperimentSpec` and the orchestrator's `ExperimentConfig` — a check
   the plan names, now becoming live: the orchestrator is upstreaming
   `determinism.orchestrator.v1` here (companion request:
   `../exploration-orchestrator/.agents/requests/phase5-prep-proto-upstream-and-tier2-chaos/`).

## Consumer Inventory (Why The Gap Is Live Risk)

Path dependencies on `crates/determinism-proto`:

- `determinism-hypervisor/Cargo.toml:24`
- `snapshot-store/Cargo.toml:13` — and their CI pins this repo at the
  Phase 0 skeleton commit `ca9ee90…`, pre-dating the Phase-4 contracts
  entirely; their vendored-proto swap bead `snapstore-8qx` waits on a tag
  *and* on upstreaming their real schema over the 12-line snapstore
  placeholder (which the `vdev` policy must not freeze first)
- `guest-sdk/Cargo.toml:31`
- `reference-workload/Cargo.toml:11` — its `m0-proto-client` already
  pulls features `["controlplane","scorer"]`: a Phase-4 scorer-contract
  consumer exists in-tree today
- `exploration-orchestrator/Cargo.toml:12`

The Phase-4 service repos (state-scorer, input-synthesizer) are not yet
instantiated locally; when they appear they consume `scorer/v1` and
`inputsynth/v1` from day one. Freezing before they exist is the whole
point of this request.

## No In-Flight Work To Collide With

No `.agents/` directory existed here before this request, no beads, no
STATUS docs, clean tree. `origin/codex/phase4-proto-contracts` is
content-identical to `main` (superseded by `261141b` — note the branch
tip itself is *not* in main's ancestry, so `--merged` checks will
mislead; diff the trees instead). This repo is idle; the request is the
plan.
