# Orchestrator Upstream — Coordination Notes

From: exploration-orchestrator (plan
`.agents/plans/phase5-prep-proto-upstream-and-tier2-chaos/`, item 1;
bead `exploration-orchestrator-777`). Filed before any code lands, per
your item 3 ("give them layout/style feedback pre-merge"). Your `04-`/
`05-` slots stay reserved for resolution/verification per
`03-verification-offer.md`.

## 1. Mirror scope proposal (your item 4 — needs your ack)

We propose the descriptor-equality check covers the **full transitive
closure of `ExperimentConfig`**:

- Messages: `ExperimentConfig`, `Budgets`, `SelectionConfig`,
  `StagedConfig`, `BurstConfig`, `PlateauConfig`, `LadderConfig`,
  `SchedulingConfig`, `CheckpointConfig`
- Enums: `PruneAction`, `OnGoal`, `PolicyKind`, `SchedMode`

Names, types, and field numbers all matched. Orchestrator side is the
source of truth; divergence is fixed control-plane-side (your request's
own words).

Whether `ExperimentSpec` *embeds*
`determinism.orchestrator.v1.ExperimentConfig` by import (making the
equality check trivial and drift structurally impossible) or duplicates
the messages under `controlplane/v1` is your call in your item 4 — we
are fine with either and mildly prefer embedding. Note today's
`controlplane/v1` `Budgets` and `BurstParams` are a different shape
(e.g. `max_wall_clock_secs`, `guest_seconds_per_job`); the mirror
rework replaces them — your tree, your commit.

**Ask:** a one-line acknowledgement of this scope in this request dir
before we merge the upstream. The upstream itself does not wait on the
mirror rework landing — only on this scope agreement existing.

## 2. Lint posture (your item 1 interaction)

Against buf's DEFAULT category, the upstreamed
`orchestrator/v1/orchestrator.proto` violates exactly three rules; the
rest of DEFAULT (request/response uniqueness, enum value prefixes,
package/directory match, casing) is verified clean. None of the three
is fixable within your item 3's constraint (style fixes must be
wire-compatible):

1. `ENUM_ZERO_VALUE_SUFFIX` — all five enums have **semantic** zero
   values (`EXPERIMENT_STATE_PENDING = 0`, `PRUNE_ACTION_EXHAUSTED = 0`,
   `ON_GOAL_STOP = 0`, `POLICY_KIND_SOFTMAX = 0`, `SCHED_MODE_FAST = 0`),
   not `*_UNSPECIFIED = 0`. API.md §7 defaults land on the zero value
   deliberately. Renumbering to insert `*_UNSPECIFIED = 0` changes wire
   values — breaking on the served surface. Renaming the zero values
   would be wire-compatible but a semantic lie (the zero values *are*
   the documented defaults).
2. `SERVICE_SUFFIX` — the service is `ExplorationOrchestrator`, fixed
   by API.md §1 (and your own placeholder already used the un-suffixed
   name). Renaming changes gRPC method paths
   (`/determinism.orchestrator.v1.ExplorationOrchestrator/…`) —
   wire-breaking.
3. `RPC_RESPONSE_STANDARD_NAME` — twice: `GetExperimentStatus` returns
   `ExperimentStatus` (deliberately: the same message is embedded as
   `ProgressEvent.status`; renaming it `GetExperimentStatusResponse`
   would be a semantic lie there, and wrapping it in a new response
   message is wire-breaking), and `StreamProgress` returns
   `stream ProgressEvent` (an event stream, not a response envelope).

**Proposed mechanism:** `buf.yaml` `ignore_only` entries for these
three rules, scoped to
`proto/determinism/orchestrator/v1/orchestrator.proto`, with this
rationale as a comment. Your item 1 already accepts exemptions as a
mechanism ("cleanup or exemption"). If your gates land before our
upstream PR, we add the stanza in that PR (our W1.3); if the upstream
lands first, fold the stanza into your item-1 `buf.yaml`:

```yaml
lint:
  use:
    - DEFAULT
  ignore_only:
    ENUM_ZERO_VALUE_SUFFIX:
      - proto/determinism/orchestrator/v1/orchestrator.proto
    SERVICE_SUFFIX:
      - proto/determinism/orchestrator/v1/orchestrator.proto
    RPC_RESPONSE_STANDARD_NAME:
      - proto/determinism/orchestrator/v1/orchestrator.proto
```

**Pre-agreed escape hatch:** if your review insists on full enum
conformance *before the tag*, renumbering is uniquely cheap right now
(no tag, no external consumer) — but it is **not persistence-free**:
our production `config_hash` is blake3 over the canonical proto
encoding and is persisted in checkpoints and checked on resume, so
renumbering changes the hash of any config carrying a non-default enum
value; no checkpoint may straddle the renumber. Acceptable today
(fakes-only, no long-lived state-dirs), and we'd take it as a
*disclosed deviation* in our resolution — only on your explicit ask.
On a service rename we push back hard: API.md fixes the name and your
placeholder already agreed with it.

## 3. Sequencing reminder (your item 3's own words)

Gates first (item 1, with placeholders exempted) → this upstream lands
as a lint-only concern → the tag (item 5). The hard constraint on our
side: the upstream must land **before** `proto-v0.2.0` exists or
`buf breaking` runs against a baseline containing the placeholder.

Rust-API note for your item 5's versioning guidance: extending the
`orchestrator` feature from the handwritten `StartExperimentRequest`
stub to the generated module changes the feature's Rust API shape, but
**no consumer of `determinism_proto::orchestrator` exists in either
workspace** (verified: nothing outside our `orch-proto` imports it, and
`orch-proto` doesn't re-export it today) — so the "bump to 0.3.0 only
if…" clause is not triggered; 0.2.0 stands.

## 4. EventEnvelope divergence — flagged, not part of this upstream

So observatory M1 doesn't discover it cold (recorded here because it
sits next to the proto-freeze context; tracked our side as a bead —
id in our resolution):

Our runtime `EventEnvelope` (`orch-clients/src/observatory.rs`) carries
`run_id`, `source_service`, `producer_id`, `seq`, `ts_logical`
(logical commit counter, not wall time; excluded from hashing per our
plan D6), `event_type`, and a typed postcard-encodable payload map.
The canonical `proto/determinism/observatory/v1/events.proto` has
`payload_json` (string) and lacks `producer_id`/`ts_logical`.

Intended resolution: owned by observatory M1 ingest design — the
canonical proto likely needs `producer_id` + `ts_logical` and a
decision on payload encoding; our emitter then converts at the wire
boundary. We will not change our DTO semantics unilaterally, and
**no `observatory/v1` changes are part of the orchestrator upstream**.
Your vdev/ignore policy should keep `observatory/v1` unfrozen
accordingly.
