# Request: The First Real Promotion — snapstore/v1 Through The vdev Rails (Gated)

> **CURRENT STATUS (2026-07-10):** The playbook half is now ready; real
> snapstore promotion remains gated. Read `04-current-status-2026-07-10.md`.

## Who Is Asking

The phases track, round 2 (2026-07-07), on behalf of snapshot-store —
whose `snapstore-8qx` (vendored-proto swap) is the first live downstream
waiter on this repo's promotion machinery — and every placeholder-family
owner who will follow the same path (hypervisor, replay, observatory).

## Standing Relative To Round 1 — Read This First

Round-1 (`phase4-proto-freeze-tag-and-breaking-gate/`) is unexecuted and
is strictly first: it creates the things this request uses (the buf
gates, the vdev placeholder policy, the `proto-v0.2.0` tag). Do not open
this request before round-1's resolution exists. It is filed now so
round-1's executor designs the vdev policy knowing its first real
customer: a policy that can't promote snapstore cleanly is wrong on
day one.

## Why control-plane, Why Now (Well, Why Next)

- Round-1 establishes *policy* — the placeholder families exempted
  from `buf breaking` so owners can upstream real schemas without
  landing in a red gate (of the eight placeholders, round-1 itself
  freezes `common`, `controlplane`, and `orchestrator` post-upstream —
  the vdev set at tag time is the remaining ~5: hypervisor, snapstore,
  policy, replay, observatory). Policy unexercised is policy untested. The
  **promotion path** — vdev → frozen v1, codegen feature extended,
  breaking-gate coverage switched on for the newly frozen family — is
  the machinery every future upstream repeats, and it should be built
  against a real case, not hypothetically.
- **snapstore is the first real case with a filed waiter.**
  `snapstore-8qx` is doubly parked today (no tag; no authored schema),
  and snapshot-store currently treats its GC proto messages as
  "canonical until `snapstore-8qx` — mirror to control-plane at
  adoption." Their round-2 request (their Phase-2 M8 close-out) keeps `8qx` parked but
  the M9-era handoff makes the schema authorship inevitable; when they
  author, this repo must be ready to receive without improvising.
- Verified for scope honesty: the Phase-4 service contracts
  (`scorer/v1`, `inputsynth/v1`) were diffed against every Phase-4
  exit-gate item and are **complete** — there is no independent
  Phase-4 proto work to pull forward. And the M0/M1 boundary stays
  closed: the plan's design-early-implement-late posture (M1–M4 live in Phase 6 per the matrix) authorizes
  no M1 server pre-work. The promotion path is the only in-charter,
  next-in-line chunk.

## One Flag That Belongs To Nobody's Repo (Not Scope — Escalation)

Phase 4's critical path needs `state-scorer` (P0 = `wksp`) and
`input-synthesizer` (P0 = M0) to *exist as repos*. They don't, locally.
That bootstrap is each repo's own Phase-0 deliverable per the matrix —
i.e., a phases-track/operator gap, not control-plane scope. Recorded
here and in the round-2 work-order note
(`~/git/preestablished/REQUEST-WORK-ORDER-2026-07-07.md`) so it stops
being invisible.

## The Ask In One Paragraph

After round-1 resolves: write the promotion playbook (the exact
mechanical steps from "owner authors real schema in a vdev family" to
"frozen v1 under `buf breaking`, codegen feature live, consumers
notified"), then execute it for `snapstore/v1` when snapshot-store
authors their schema — owner-authored, control-plane-reviewed, the
12-line placeholder replaced, the `snapstore` crate feature switched to
real codegen without breaking existing feature consumers, the family
moved from vdev exemption into breaking-gate coverage, and
`snapstore-8qx` handed its unpark signal (tag + published family) —
with the playbook itself corrected by whatever the first execution
teaches.

## Files In This Request

| File | Contents |
|---|---|
| `01-current-state.md` | Evidence: round-1's unexecuted state, 8qx's double park, scope-honesty checks |
| `02-requested-work.md` | Entry conditions, the ask, acceptance criteria, out of scope |
| `03-verification-offer.md` | Choreography with snapshot-store; handback |
