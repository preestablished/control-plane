# Requested Work

## Entry Conditions (Hard Gate)

1. **Round-1 resolved**: buf lint/breaking CI live with the committed
   vdev policy, aarch64 lane, orchestrator upstream received,
   `proto-v0.2.0` tagged. Verify the resolution actually shipped the
   vdev exemption leaving `snapstore/v1` unfrozen — round-1 promises
   exactly that; if its execution drifted, fix that first.
2. **snapshot-store ready to author**: their `snapstore/v1` schema
   stable enough to freeze (their call — the M8/M9-era vendored schema
   is the source; do not pressure a freeze before their milestones
   firm it up).

**Scope ruling (post-review):** this request's resolvable deliverable
is the **playbook half** — items 1 and the dry-run — closable via
`04-playbook-resolution.md` once round-1 lands. The snapstore
*execution* (items 2–3) may be M9/Phase-8-distant ("their call" is
real); rather than idling half-open across phases, the execution half
is a **named successor**: when snapshot-store's ready-signal lands,
file `phase?-snapstore-v1-promotion-execution/` citing the playbook.
Items 2–3 below are that successor's spec, written now so the
playbook is designed against its first customer.

## What We Need (Behavioral)

1. **The promotion playbook.** A committed doc (this repo) with the
   mechanical steps: owner authors real schema in the vdev family
   (owner-authored, control-plane-reviewed PR); lint clean; descriptor
   review vs the vendored source-of-truth, **with an abort/rollback
   path when that review finds divergence mid-promotion**; `buf.yaml`
   ignore-path / vdev flip into breaking-gate coverage, **settling the
   tag-interaction question** (a newly covered family diffed against a
   pre-promotion tag baseline is a foreseeable footgun — the playbook
   states the order, not just the question); the **vdev-policy ledger
   update** (round-1's committed which-families-are-frozen doc must
   change with every promotion); `build.rs` + Cargo feature extended
   to real codegen; facade removal or supersession marking;
   version/tag decision rule; consumer notification list + re-pin
   instructions. Dry-run it against a scratch family on a branch —
   and the scratch setup must include a **fake vendored source file
   and a minimal in-repo consumer of the scratch feature**, so the two
   riskiest steps (descriptor review, consumers-keep-compiling) are
   actually exercised; any step the scratch genuinely can't reach is
   enumerated in the transcript so "dry-run passed" isn't overread.
2. **Execute for `snapstore/v1`** when condition 2 holds: receive
   their authored schema, run the playbook end-to-end, replace the
   12-line placeholder, extend the `snapstore` feature to real
   codegen (existing feature consumers keep compiling — CI proves it),
   flip the family into breaking coverage, tag per the decision rule,
   and hand `snapstore-8qx` its unpark signal (tag name + family path
   + re-pin instructions, noted on the bead and in their request dir).
3. **Correct the playbook** with whatever the first real execution
   taught; the doc's changelog records the deltas. The next family
   (hypervisor's, replay's, observatory's) should be executable by a
   cold agent from the doc alone.

## Acceptance Criteria

1. Playbook committed with a recorded dry-run transcript.
2. `snapstore/v1` frozen: placeholder gone, real codegen behind the
   feature, both repos' CI green, family under `buf breaking`
   (demonstrated: a scratch-branch field deletion in the promoted
   family goes red).
3. `snapstore-8qx` unparked: the signal (tag name + family path +
   re-pin instructions, verified accurate against their current CI
   config) recorded on the bead and in their request dir. The re-pin
   *landing* is theirs; the phases track verifies it when it occurs —
   it is not this executor's acceptance gate.
4. Playbook changelog shows the post-execution corrections (or
   records "none needed" with the dry-run/real diff as evidence).

## Out Of Scope For This Request

- Round-1's scope — predecessor, unmodified.
- Authoring the snapstore schema — snapshot-store's, reviewed here.
- Promoting the other placeholder families — the playbook enables
  them; each waits for its owner.
- The scorer/synth repo bootstrap — program-level gap, escalated in
  the work-order note, not adopted here.
- Anything M1+ (Phase 6).
