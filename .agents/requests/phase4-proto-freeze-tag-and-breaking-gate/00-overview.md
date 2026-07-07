# Request: Freeze The Phase-4 Contracts — Tag, Breaking Gate, aarch64 CI

## Who Is Asking

The phases track, on behalf of the Phase 4 services about to build against
this repo's contracts (`state-scorer`, `input-synthesizer` — repos not yet
instantiated) and `exploration-orchestrator`, which is upstreaming its
proto surface here under a companion request. Filed 2026-07-07.

## Why control-plane, Why Now

This repo is correctly at M0-only — M1–M4 (server, registry, queue, run
lifecycle) are gated to Phase 6 and nothing here starts them. But M0's own
acceptance criteria are not fully met, and the gap becomes dangerous
exactly now:

- **The Phase-4 contracts just landed** (`261141b`, `scorer/v1` 250 lines +
  `inputsynth/v1` 230 lines, real generated tonic code) and are about to
  get their first real consumers: state-scorer M1–M4 and input-synthesizer
  M1–M3 carry Phase 4's critical path (alongside reference-workload M6,
  `phase-4-scoring-and-inputs.md`), and
  `reference-workload/crates/m0-proto-client` already pulls the `scorer`
  feature.
- **Nothing protects those contracts.** CI runs `cargo fmt/build/test`
  only — no `buf lint`, no `buf breaking`. The repo's own
  IMPLEMENTATION-PLAN puts the breaking-change gate in M0's CI acceptance
  and requires a standing self-test for it (its "Proto CI" testing row),
  and the phases standing rule ("released fields are never broken") has
  no enforcement mechanism today.
- **Sister repos consume by path, mostly unpinned.** All five local repos
  reference `path = "../control-plane/crates/determinism-proto"`; the one
  exception proves the point — snapshot-store's CI pins control-plane at
  the Phase 0 skeleton commit (`ca9ee90…`), i.e. *before the Phase-4
  contracts existed*, because there has never been a tag to pin to. The
  crate claims `PROTO_VERSION = "proto-v0.2.0"` (`src/lib.rs:6`) but no
  git tag exists at all — the M0 acceptance item "tag-based consumption"
  (written as `proto-v0.1.0`, never tagged; satisfied late at 0.2.0) is
  unmet.
- **No aarch64 CI**, despite the plan mandating both architectures from
  M0 (control-plane and the scorer deploy on the DGX Spark, aarch64).

One quiet-window week of CI/tag work now buys Phase 4 a frozen, enforced
contract surface; skipping it prices proto churn into the scorer and
synthesizer bring-up instead.

## The Ask In One Paragraph

Bring M0 to its own written acceptance bar, in this order: add `buf lint`
+ `buf breaking` to CI **with the plan's `vdev` exemption for the eight
placeholder families** (so the gate protects real contracts without
freezing stubs their owners haven't upstreamed yet); add an aarch64 lane;
receive the orchestrator's `determinism.orchestrator.v1` upstream
(companion request in that repo — it replaces an incompatible placeholder
file, so it must land before the tag); rework `controlplane/v1`
`ExperimentSpec` into the field-for-field mirror of `ExperimentConfig`
the plan mandates and land the descriptor-equality check; and only then
publish the `proto-v0.2.0` tag the crate already claims, born protected.

## Files In This Request

| File | Contents |
|---|---|
| `01-current-state.md` | Evidence: M0 state, the acceptance gaps, consumer inventory |
| `02-requested-work.md` | The ask, sequencing, acceptance criteria, out of scope |
| `03-verification-offer.md` | Cross-repo verification with exploration-orchestrator |
