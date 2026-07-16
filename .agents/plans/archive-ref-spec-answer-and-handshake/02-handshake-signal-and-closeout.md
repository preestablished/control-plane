# 02 — Playbook-Ready Signal and Closeout

Deliverable: the half-done reciprocal handshake completed from control-plane's
side — a playbook-ready signal in snapshot-store's request dir — plus a
resolution note at home and session close.

## The residue being fixed

`/Users/punk1290/git/preestablished/control-plane/.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/04-playbook-resolution.md`
records the vdev promotion playbook as done (2026-07-11, commit `5a3b4f9`,
entry point `docs/vdev-promotion-playbook.md`, rehearsal
`docs/vdev-promotion-dry-run.md`) and explicitly records that no signal was
sent. The protocol
(`/Users/punk1290/git/preestablished/snapshot-store/.agents/requests/phase2-closeout-m8-joint-fork-integrity/02-requested-work.md`
lines 77–84) says whichever side is ready first leaves the ready-signal in the
other's request dir. Control-plane's side was ready first (2026-07-11) and
never signaled.

## Coordination rule with the sibling snapshot-store plan (READ FIRST)

A parallel snapshot-store plan named
**`snapstore-v1-stable-schema-and-ready-signal`** is being authored (expected
under `/Users/punk1290/git/preestablished/snapshot-store/.agents/plans/`; not
yet on disk 2026-07-16). It covers the **opposite direction**: snapshot-store
sending its owner-authored stable-schema ready signal into *control-plane's*
request dir (`.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/`).
The two signals are distinct files in distinct repos and do not conflict —
but to guarantee no duplicate or contradictory signal:

- **Write control-plane's signal only if absent.** If any file in
  snapshot-store's request dirs already announces control-plane's playbook
  readiness (whoever authored it), do not write a second one — verify its
  content matches `04-playbook-resolution.md` and move to the resolution note.
- If the sibling plan exists on disk by execution time, skim it for a
  signal-file location/naming convention and follow it if one is declared for
  the control-plane→snapshot-store direction; otherwise use the default below.
- Do NOT send, imply, or paraphrase snapshot-store's schema-ready signal —
  that is owner-authored by definition and is the sibling plan's deliverable.
- Do NOT file the promotion successor request
  (`phase?-snapstore-v1-promotion-execution/`) — per `04-playbook-resolution.md`
  it waits for snapshot-store's schema-ready signal to actually arrive.

## Step 1 — Check for an existing signal

```
ls /Users/punk1290/git/preestablished/snapshot-store/.agents/requests/*/
grep -rli "playbook" /Users/punk1290/git/preestablished/snapshot-store/.agents/requests/
ls /Users/punk1290/git/preestablished/snapshot-store/.agents/plans/
```

Plan-time state (2026-07-16): three request dirs
(`phase2-closeout-m8-joint-fork-integrity` files 00–06, `phase3-m7-gc-exit-gate`
00–05, `phase5-readiness-gc-benchmark-and-transport-revalidation` 00–04); no
signal file anywhere; the only "ready signal" text is the protocol paragraph
itself. If that still holds, proceed.

## Step 2 — Write the signal file

Default location (the dir where the handshake protocol is mirrored, next free
number — re-check numbering at execution time):

`/Users/punk1290/git/preestablished/snapshot-store/.agents/requests/phase2-closeout-m8-joint-fork-integrity/07-controlplane-playbook-ready-signal.md`

Content outline (keep it short — it is a signal, not a report):

1. Header: "control-plane → snapshot-store: vdev promotion playbook READY",
   dated, citing the protocol paragraph in `02-requested-work.md` (~77–84) as
   the reason this file exists in this dir.
2. What is ready: promotion playbook + rehearsed dry run, landed 2026-07-11 at
   control-plane commit `5a3b4f9`; entry point
   `control-plane/docs/vdev-promotion-playbook.md`; transcript
   `control-plane/docs/vdev-promotion-dry-run.md`; standing CI descriptor
   comparator on every PR. Resolution of record:
   `control-plane/.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/04-playbook-resolution.md`.
3. What control-plane now awaits: snapshot-store's **owner-authored
   stable-schema ready signal**, left in
   `control-plane/.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/`
   per the same protocol. On arrival, control-plane files the successor
   request (`phase?-snapstore-v1-promotion-execution/`) using the playbook's
   two-release staging/freeze sequence.
4. Status quo reassurance: `determinism.snapstore.v1` remains a placeholder,
   path still ignored by Buf breaking, still in the pre-release ledger; no
   `proto-v*` tag was created (all per `04-playbook-resolution.md`).
5. Coordination line: note that snapshot-store's own plan
   `snapstore-v1-stable-schema-and-ready-signal` covers the reverse-direction
   signal, so neither side double-sends.

## Step 3 — Resolution note at home

Add
`/Users/punk1290/git/preestablished/control-plane/.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/05-handshake-signal-sent.md`
(next free number after `04-current-status-2026-07-10.md` /
`04-playbook-resolution.md`; re-check — the sibling snapshot-store plan has a
declared claim on `05-snapstore-owner-ready-signal.md` in this same dir, so
if that file is present or imminent, PREFER `06-handshake-signal-sent.md`;
never renumber the sibling's file): three or four lines recording that the
signal `04-playbook-resolution.md` left unsent has now been delivered, with the
signal file's absolute path and date, and that observation-1's ruling addendum
(plan file `01-*`) was delivered the same session. This closes the residue on
control-plane's side of the record.

## Step 4 — Session close

Docs-only session; per the repo state (no `.beads/` in control-plane) tracking
stays in these plan/request files. **No `bd` commands in snapshot-store or
state-scorer this session**: no bead state changed, so skipping `bd dolt push`
there is intentional — and it avoids colliding with the sibling snapshot-store
plan's serial bead work (`snapstore-8qx`), which may be mid-flight.

1. Light review pass: one reviewer checks (a) every citation in the ruling
   addendum resolves against the file on disk, (b) the signal file matches the
   protocol text and `04-playbook-resolution.md` facts, (c) no scope-fence
   breach (no proto, no code, no successor filed).
2. Commit + push **per repo**, verifying context first (`pwd`,
   `git remote -v`) — three possible repos: control-plane (plan dir +
   `05-handshake-signal-sent.md`), state-scorer (`06-archive-ref-ruling.md`),
   snapshot-store (`07-controlplane-playbook-ready-signal.md`). state-scorer
   and snapshot-store CLAUDE.md both require `git pull --rebase` then push
   until `git status` shows up to date with origin; do the same for
   control-plane. Run pushes as separate, individually-checked commands.
3. Verification report (honest): list the files that now exist, the push
   confirmation per repo, and anything skipped.

## Acceptance

- Exactly one control-plane playbook-ready signal exists in snapshot-store's
  request dir (written by this plan, or pre-existing and verified — never two).
- The signal names what is ready, where the evidence lives, and what
  control-plane awaits next, per the protocol.
- `05-handshake-signal-sent.md` closes the residue in control-plane's request
  dir.
- All touched repos committed and pushed; no proto, code, bead, or successor
  changes anywhere.
