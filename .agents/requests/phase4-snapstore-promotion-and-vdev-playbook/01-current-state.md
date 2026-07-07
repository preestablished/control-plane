# Current State (Evidence-Based)

Repo `main` at `5376e63` (the round-1 filing commit on `261141b`),
clean tree, assessed 2026-07-07. `git tag` empty — no `proto-v*`
exists; `PROTO_VERSION = "proto-v0.2.0"` still unmatched by any tag.

## Round-1: Filed, Zero Progress

No `buf.yaml`, no buf CI, no aarch64 lane; the orchestrator placeholder
(`proto/determinism/orchestrator/v1/orchestrator.proto`, empty service)
untouched; `controlplane/v1` `ExperimentSpec` still the 21-line stub;
no resolution file. The companion orchestrator request is likewise
unexecuted (their real schema sits ready in `orch-proto`).

## The snapstore Situation

- `snapshot-store/Cargo.toml` path-deps this crate, and their CI pins
  this repo at the Phase-0 skeleton commit `ca9ee90…` — pre-dating the
  Phase-4 contracts — because there has never been a tag to pin.
- `snapstore-8qx` ("adopt-snapstore-proto-v1", P2): parked on (a) a
  `proto-v*` tag existing and (b) snapshot-store authoring its real
  `snapstore/v1` schema over this repo's 12-line placeholder. Their GC
  work ships proto messages marked "canonical until `snapstore-8qx` —
  mirror to control-plane at adoption" — the real schema exists in
  vendored form; upstreaming is authorship + review, not invention.
- snapshot-store's round-2 request (their Phase-2 M8 close-out,
  `phase2-closeout-m8-joint-fork-integrity/`) explicitly keeps `8qx`
  parked and points at this request for the receiving side. Note
  their request also out-scopes the proto upstream from M8 — the
  authorship day may be M9/Phase-8-era.
- One adjacent item deliberately *not* picked as round-2: the
  recorded EventEnvelope divergence (orchestrator runtime struct vs
  `observatory/v1` proto). It resolves through this same playbook one
  day (an observatory-family promotion) — out of scope until that
  repo activates.

## Scope-Honesty Checks (So This Request Can't Wander)

- **Phase-4 protos are complete.** `scorer/v1` covers ScoreBatch +
  LatencyHistogram (exit gate 1's budget shape), the archive
  checkpoint/restore/replay surface with binding hashes (the
  archive-determinism test), `state_hash`/`duplicate`/NoveltyDetail
  (gate 2); `inputsynth/v1` covers seeded ProposeBursts with
  seed/config_fingerprint/synth_version echo (gate 4's golden-seed
  shape), macro packs, mutation ops. No proto gap for Phase 4.
- **Repo bootstrap for scorer/synth is not control-plane's.** The
  matrix assigns state-scorer P0 = `wksp` and input-synthesizer
  P0 = M0 — their own Phase-0 deliverables. Flagged as a program gap,
  not adopted as scope.
- **M1 pre-work stays closed.** Phase 6 gates M1–M4;
  the design-early-implement-late posture (M1–M4 gated to Phase 6) leaves no pre-work window.

## What The Promotion Path Must Handle (Learned From Round-1's Design)

- vdev families are exempt from `buf breaking`; promotion flips a
  family into coverage — the playbook must specify the exact
  `buf.yaml`/ignore-path change and the tag interaction (promote
  before or after which tag?).
- Codegen: the crate's per-family features currently serve facades for
  placeholder families; promotion extends `build.rs` + features to
  real tonic codegen without breaking existing feature consumers
  (the orchestrator receive in round-1 item 3 is the template —
  its lessons feed the playbook).
- Consumers: snapshot-store's stale CI pin is the worked example of
  why notification + re-pin instructions belong in the playbook.
