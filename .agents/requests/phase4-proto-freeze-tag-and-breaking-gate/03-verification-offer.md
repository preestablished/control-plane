# Cross-Repo Verification

## With exploration-orchestrator (items 3–4)

Their companion request
(`../exploration-orchestrator/.agents/requests/phase5-prep-proto-upstream-and-tier2-chaos/`)
carries the other half of the upstream. The joint verification is:

- both repos' CI green with the canonical `orchestrator/v1` in this tree;
- the scratch-branch `buf breaking` demonstration run by whichever repo
  lands second, recorded in both request dirs;
- the descriptor-equality (or documented-relationship) check green.

## Phases-Track Check

We will verify from a clean checkout that:

1. a fresh clone of any sister repo builds against the tagged
   `determinism-proto` (spot check: reference-workload's
   `m0-proto-client` with `["controlplane","scorer"]`);
2. the tag, `PROTO_VERSION`, and `Cargo.toml` agree;
3. deleting a released field on a scratch branch fails CI.

## Handback Shape

This repo had no `.agents/` directory before this request, so this
directory establishes the convention here (same as the other repos):
append `04-resolution.md` with git SHAs, the tag name, CI run links, the
breaking-gate demonstration, and the notification list; we respond with
`05-verification.md`.

## Contact / Tracking

- Companion request: exploration-orchestrator
  `phase5-prep-proto-upstream-and-tier2-chaos` (filed the same day).
- Known downstream waiters: `snapstore-8qx` (vendored-proto swap — waits
  on the tag *and* on a future upstream of the real snapstore schema over
  its placeholder, which the vdev policy must leave unfrozen), future
  state-scorer / input-synthesizer repos (consume `scorer/v1` /
  `inputsynth/v1` from day one), observatory (consumes the orchestrator
  event stream at its M1 — note the recorded EventEnvelope divergence
  flagged in the orchestrator's request).
