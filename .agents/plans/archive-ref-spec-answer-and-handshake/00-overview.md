# Plan: archive_ref Spec Answer and Playbook-Ready Handshake

Small, two-deliverable plan. Both items are open control-plane obligations that
are dischargeable today, with no dependency on the Phase 5 exit:

1. **Answer state-scorer spec observation 1 on the record** — whether
   `created_unix_ms` belongs inside the `archive_ref` hash prefix. The question
   was explicitly assigned to control-plane
   (`state-scorer/.agents/requests/phase4-m1-m4-first-boss-scoring/05-refwork-spec-ratification.md`,
   lines 15–16: "Item 1 (`created_unix_ms` inside the archive_ref hash) is
   control-plane's to answer") and has never been answered.
2. **Deliver control-plane's playbook-ready signal** into snapshot-store's
   request dir. `.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/04-playbook-resolution.md`
   records the vdev promotion playbook as ready (2026-07-11, commit `5a3b4f9`)
   but also records "No snapstore consumer unpark signal was sent" — the
   reciprocal handshake ("whichever side is ready first ... leaves the
   ready-signal in the other's request dir",
   `snapshot-store/.agents/requests/phase2-closeout-m8-joint-fork-integrity/02-requested-work.md`
   lines 77–84) was left half-done.

Plan files:

| File | Outcome |
|---|---|
| `01-archive-ref-ruling.md` | Grounded ruling written as an addendum into state-scorer's request dir; control-plane-owned docs checked |
| `02-handshake-signal-and-closeout.md` | Playbook-ready signal in snapshot-store's request dir (only if absent); resolution note; session close |

## Grounding notes (verified 2026-07-16 at plan-authoring time)

Every citation below was read from disk while authoring this plan. The
implementer must re-verify each before acting — line numbers can drift.

| Claim | Source | Verified content |
|---|---|---|
| Observation 1 text | `/Users/punk1290/git/preestablished/state-scorer/.agents/requests/phase4-m1-m4-first-boss-scoring/04-resolution.md` lines 101–108 | "`created_unix_ms` sits inside the `archive_ref` hash (API.md §5): a re-checkpoint of identical archive state at a different wall time yields a different ref while file bytes are identical. Tests inject a fixed Clock; production uses wall time. If refs were meant to be wall-time-independent, the field belongs outside the hashed prefix — control-plane/spec question." |
| Assignment to control-plane | same dir, `05-refwork-spec-ratification.md` lines 15–16 | "Item 1 ... is control-plane's to answer." Items 2–4 were ratified by reference-workload (the owner) by editing its own API.md with "ratified 2026-07-12" stamps — the precedent this plan mirrors. |
| The archive_ref contract | `/Users/punk1290/.agents/projects/determinism/docs/state-scorer/API.md` §5 (lines 661–687) | `MANIFEST.json` contains `"created_unix_ms": 1781049600000` (line 671). Hash rule (lines 684–685): "`archive_ref` / `archive_hash` = BLAKE3-256 over: `MANIFEST.json` bytes ‖ each file's bytes in `files[]` order. Verified on restore before anything is deserialized." So the "hashed prefix" is the manifest bytes; the wall-clock field is inside them. |
| Proto is untouched by any ruling | `/Users/punk1290/git/preestablished/control-plane/proto/determinism/scorer/v1/scorer.proto` lines 159–187 | `CheckpointArchiveRequest`, `CheckpointArchiveResponse` (`archive_ref` field 1, `archive_hash` field 2), `RestoreArchiveRequest` (`archive_ref` field 3). **`created_unix_ms` does not appear anywhere in the proto** — it exists only in the on-disk MANIFEST.json spec. Either ruling is a doc/format matter, not a proto change. |
| Content-address convention | `/Users/punk1290/.agents/projects/determinism/docs/MAP.md` lines 110–111 | Snapshot reference = "the BLAKE3-256 of the manifest's canonical bytes — nothing else." |
| Wall-clock-free precedent | `/Users/punk1290/.agents/projects/determinism/docs/snapshot-store/README.md` line 104; `ARCHITECTURE.md` line 103 | snapshot-store's logical counter is "Monotonic `u64` ... independent of wall clock"; its hashed manifest layout carries `created_epoch u64` — a **logical** counter at creation, never wall time. |
| Doc ownership | `/Users/punk1290/.agents/projects/determinism/docs/MAP.md` ownership table (lines 143–155) | "novelty archives + checkpointing" → **state-scorer** (API.md §5 is their owner doc); "proto repo layout" → control-plane. Control-plane answers the identity-semantics question; state-scorer stamps its own doc. |
| No-re-serialization convention | state-scorer API.md line 440 | `feature_map_hash`/`program_hash` = "BLAKE3-256 of the artifact's canonical bytes (the exact bytes loaded, no re-serialization)". |
| Handshake protocol | `/Users/punk1290/git/preestablished/snapshot-store/.agents/requests/phase2-closeout-m8-joint-fork-integrity/02-requested-work.md` lines 77–84 | "Reciprocal handshake, mirrored here so both sides' texts agree: whichever side is ready first (their playbook, or this repo's authored schema) leaves the ready-signal in the other's request dir." |
| Signal never sent | `/Users/punk1290/git/preestablished/control-plane/.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/04-playbook-resolution.md` lines 64–69 | "No snapstore consumer unpark signal was sent and no real `proto-v*` tag was created. File the successor only after snapshot-store sends an owner-authored stable-schema ready signal." |
| No signal file exists yet | grep of `/Users/punk1290/git/preestablished/snapshot-store/.agents/requests/` (2026-07-16) | Only hit for "ready signal" language is the protocol text itself in `02-requested-work.md`. The three request dirs (`phase2-closeout-m8-joint-fork-integrity` 00–06, `phase3-m7-gc-exit-gate` 00–05, `phase5-readiness-gc-benchmark-and-transport-revalidation` 00–04) contain no signal file. |

### Unconfirmed at plan time (implementer must resolve)

- **Scorer source behavior**: the claim that production hashes wall time is
  grounded on API.md §5 plus state-scorer's own observation text, not on a read
  of the Rust in `/Users/punk1290/git/preestablished/state-scorer/` (crate
  `scorer-archive`). Optionally confirm before finalizing the ruling; the
  observation is state-scorer's own report of their implementation, so this is
  low risk.
- **Sibling plan**: the parallel snapshot-store plan
  `snapstore-v1-stable-schema-and-ready-signal` now exists under
  `/Users/punk1290/git/preestablished/snapshot-store/.agents/plans/`. It covers
  only the opposite direction (snapshot-store's schema-ready signal into
  control-plane's request dir) and declares no convention for this plan's
  control-plane→snapshot-store signal, so the default
  `07-controlplane-playbook-ready-signal.md` stands. Re-check for drift before
  writing (see `02-*`).
- **Whether `~/.agents/projects/determinism/docs/` is itself version-controlled**
  — not checked. The plan does not require editing it (the API.md §5 edit is
  delivered as proposed text for state-scorer to stamp), so this only matters
  if the implementer chooses to apply the stamp directly; verify first if so.

## Scope fence

- **NO M1–M4 work.** Those are Phase 6, gated on the Phase 5 exit. Nothing in
  this plan touches control-plane milestone code.
- **NO proto changes.** Verified above: `created_unix_ms` is not a proto field,
  so the ruling lands as spec-doc text only. If the implementer's re-grounding
  finds a proto change is somehow required, STOP — that lands per the frozen
  proto process (`docs/proto-freeze-policy.md`) as a follow-up request, not in
  this plan.
- **NO filing of the snapstore promotion successor** (`phase?-snapstore-v1-promotion-execution/`)
  — per `04-playbook-resolution.md` that waits for snapshot-store's
  owner-authored stable-schema ready signal, which is the *other* direction of
  the handshake and is the sibling plan's job.
- **NO code or bead changes inside state-scorer or snapshot-store.** This plan
  writes markdown into their `.agents/requests/` dirs only (established
  cross-repo pattern; refwork's `05-refwork-spec-ratification.md` is the
  precedent).

## Tracking and session close

- control-plane has **no `.beads/`** (verified 2026-07-16) and prior plan dirs
  track via plan files + request-dir resolutions; mirror that. Do not `bd init`
  as a side effect of this plan.
- Three repos may be touched: control-plane (this plan dir + resolution note),
  state-scorer (ruling addendum), snapshot-store (signal file). Commit and push
  **each repo separately**; before every commit run `pwd` + `git remote -v` to
  confirm repo context (worktree-drift guard). state-scorer and snapshot-store
  CLAUDE.md both mandate push-to-remote before a session is complete.
- Docs-only change set: a light review pass (one reviewer sanity-checking the
  ruling's citations and the signal's protocol conformance) satisfies the
  review gate; no build/test surface exists.
