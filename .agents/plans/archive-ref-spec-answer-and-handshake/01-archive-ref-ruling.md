# 01 — The archive_ref Ruling

Deliverable: state-scorer spec observation 1 answered on the record, as an
addendum in their request dir, complete enough that state-scorer can act
without follow-up questions.

## Step 1 — Re-verify the sources

Read, in order (paths absolute; line refs as of 2026-07-16, re-locate if drifted):

1. `/Users/punk1290/git/preestablished/state-scorer/.agents/requests/phase4-m1-m4-first-boss-scoring/04-resolution.md`
   lines 101–108 — the question.
2. `/Users/punk1290/git/preestablished/state-scorer/.agents/requests/phase4-m1-m4-first-boss-scoring/05-refwork-spec-ratification.md`
   — the assignment ("control-plane's to answer") and the ratification-stamp
   precedent for items 2–4.
3. `/Users/punk1290/.agents/projects/determinism/docs/state-scorer/API.md` §5
   (lines ~643–687) — the contract: MANIFEST.json fields (incl.
   `created_unix_ms`, line ~671) and the hash rule (lines ~684–685:
   BLAKE3-256 over `MANIFEST.json` bytes ‖ each file's bytes in `files[]`
   order, verified on restore).
4. `/Users/punk1290/git/preestablished/control-plane/proto/determinism/scorer/v1/scorer.proto`
   lines ~159–187 — confirm `created_unix_ms` still absent from the proto.
5. Precedents: `/Users/punk1290/.agents/projects/determinism/docs/MAP.md`
   lines ~110–111 (snapshot ref = BLAKE3 of manifest canonical bytes, nothing
   else) and ownership table (~143–155);
   `/Users/punk1290/.agents/projects/determinism/docs/snapshot-store/README.md`
   line ~104 and `ARCHITECTURE.md` line ~103 (logical counters, wall-clock-free
   hashed manifest).
6. Optional but cheap: grep `created_unix_ms` in
   `/Users/punk1290/git/preestablished/state-scorer/` to confirm the
   implemented manifest matches API.md §5 and that tests inject a fixed Clock.

## Step 2 — The ruling

**Recommended ruling (grounded at plan time): `created_unix_ms` comes OUT of
the hashed prefix.** `archive_ref` must be computable from archive content and
caller-supplied identity alone — no wall-clock input. Rationale, each point
cite-backed:

1. **Project convention.** Content addresses hash canonical content only:
   snapshot refs are "the BLAKE3-256 of the manifest's canonical bytes —
   nothing else" (MAP.md ~110), and `feature_map_hash`/`program_hash` are
   hashes of exact canonical artifact bytes (state-scorer API.md ~440). A
   wall-clock timestamp inside a content address is the one nondeterministic
   input in an otherwise deterministic identity scheme.
2. **Sibling precedent.** snapshot-store — the platform's other
   checkpoint-shaped store — deliberately keeps wall time out of hashed
   manifests: its hashed layout carries `created_epoch`, a *logical* counter
   "independent of wall clock". state-scorer already has the logical analog in
   the manifest: `archive_seq`. The wall-clock field is redundant as identity
   input.
3. **Idempotency / retry.** `CheckpointArchive(experiment_id, checkpoint_id)`
   re-issued over identical archive state should return the same
   `archive_ref`. With wall time inside the hash, a retried or repeated
   checkpoint of byte-identical state mints a new ref — exactly the anomaly
   observation 1 reports ("different ref while file bytes are identical").
4. **Test/production divergence.** The archive-determinism CI test only passes
   because tests inject a fixed Clock (04-resolution.md, observation 1);
   production refs are therefore unreproducible in a way CI never exercises.
   Ruling the field out makes the tested property and the production property
   the same property.

**Counterpoints, addressed in the addendum (do not omit them):**

- *"archive_ref is a blob-integrity address, not a state-identity address —
  restore only verifies bytes."* True today (RestoreArchive verifies
  blake3(blob) == ref), and integrity still holds under the ruling: the hash
  still covers every byte that is hashed; the manifest simply no longer
  contains a nondeterministic field. Nothing consumes wall time from the ref.
- *"`experiment_id`/`checkpoint_id` are in the manifest anyway, so refs were
  never pure content addresses."* Those are deterministic, caller-assigned
  identity inputs — same call, same ref. `created_unix_ms` is the only input
  the caller cannot reproduce.

**Mechanism (the addendum offers both; state-scorer picks, both compliant):**

- **(a) Preferred: remove `created_unix_ms` from `MANIFEST.json`.** Provenance
  wall time moves outside the hashed blob — e.g. the orchestrator's checkpoint
  record, or an unhashed sidecar file that is *not* listed in `files[]`. Keeps
  the existing hash rule ("MANIFEST.json bytes ‖ file bytes, verbatim")
  untouched.
- **(b) Rejected variant, named so state-scorer doesn't reinvent it:** keep the
  field but strip/zero it during hashing. This forces re-serialization or
  byte-surgery before hashing on both checkpoint and restore, violating the
  "exact bytes, no re-serialization" convention (API.md ~440) and complicating
  "verified on restore before anything is deserialized".
- Format note: MANIFEST.json is versioned (`format_version: 1`). M4 is
  development-complete but pre-Phase-5-exit and single-consumer; state-scorer
  chooses whether this is an in-place v1 amendment or a v2 bump — the ruling
  constrains the *identity semantics*, not the migration path.

**Escape hatch:** if Step 1's re-grounding surfaces a real consumer that needs
wall time inside the ref (none is known at plan time), the deliverable is
unchanged in shape: a documented decision with rationale. Rule KEEP, write down
why, and state that refs are defined as wall-time-*dependent* blob addresses so
state-scorer can delete the "if refs were meant to be wall-time-independent"
ambiguity either way. An unanswered question is the only unacceptable outcome.

## Step 3 — Deliver the addendum

Write
`/Users/punk1290/git/preestablished/state-scorer/.agents/requests/phase4-m1-m4-first-boss-scoring/06-archive-ref-ruling.md`
— `06-` follows their numbering (dir currently ends at `05-`; re-check before
writing and take the next free number). Content outline:

1. Header: "control-plane → state-scorer: archive_ref ruling (spec observation
   1)", dated, replying to `04-resolution.md` observation 1 per the assignment
   in `05-refwork-spec-ratification.md`.
2. **The ruling in one sentence**, then the normative rule text, e.g.:
   "`archive_ref`/`archive_hash` are wall-time-independent: every byte covered
   by the hash MUST be reproducible from archive content and caller-supplied
   identity. `created_unix_ms` is removed from `MANIFEST.json`; creation wall
   time is provenance metadata and lives outside the hashed blob."
3. Rationale + addressed counterpoints (from Step 2).
4. **Exact proposed edit text for API.md §5** (the MANIFEST.json example minus
   `created_unix_ms`, plus one bullet stating the wall-time-independence rule).
   API.md §5 is state-scorer's owner doc (MAP.md ownership: "novelty archives +
   checkpointing" → state-scorer), so mirror the refwork precedent: control-plane
   supplies the ruling and the exact text; state-scorer applies the edit and
   stamps it ("ruled by control-plane 2026-07-XX").
5. Explicit statement: **no proto change** — `created_unix_ms` appears in no
   proto message; `CheckpointArchiveResponse`/`RestoreArchiveRequest` are
   unaffected. (Pre-empts a frozen-proto scare.)
6. Mechanism options (a)/(b) with (a) recommended, format_version note, and a
   pointer that the archive-determinism CI test should then pass with the
   production clock path, closing the test/prod divergence.
7. Backward compatibility, stated explicitly: existing v1 checkpoints (whose
   manifests contain `created_unix_ms`) remain restorable — restore verifies
   the ref against the actual on-disk bytes, so old blobs stay
   self-consistent; only newly minted refs change format. (This is the first
   question a fresh state-scorer session would ask; answer it in the
   addendum, not in follow-up.)

## Step 4 — Control-plane-owned docs

Check whether any control-plane-owned doc must record the ruling:
`/Users/punk1290/git/preestablished/control-plane/docs/` holds only
`proto-freeze-policy.md`, `vdev-promotion-playbook.md`,
`vdev-promotion-dry-run.md` (verified 2026-07-16) — none defines archive_ref
semantics, and the proto is untouched. Expected outcome: **none required**;
record that finding (one line) in the addendum so the "update any
control-plane-owned doc" box is explicitly closed, not silently skipped.

## Acceptance

- Addendum exists in state-scorer's request dir, follows their numbering, and
  contains: ruling, normative text, rationale with counterpoints, exact
  proposed API.md §5 edit, mechanism options, explicit no-proto-change
  statement.
- Every citation in the addendum re-verified against the file on disk at
  execution time.
- A fresh state-scorer session can act on it with zero follow-up questions.
