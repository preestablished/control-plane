# Plan: Vdev Promotion Playbook and Snapstore Dry Run

Answers:
`.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/`.

## Scope ruling

Implement only the currently resolvable half of the request:

1. Commit an operational vdev-to-frozen-v1 promotion playbook.
2. Dry-run it with a disposable scratch family, a fake owner-vendored proto,
   and a minimal consumer of the generated feature.
3. Record the transcript, limitations, and any corrections learned.
4. Close this request with `04-playbook-resolution.md`.

Do **not** replace or freeze `determinism.snapstore.v1`. Snapshot-store has
not yet sent the owner-authored stable-schema ready signal. Its real promotion
is a named successor request, to be filed only after that signal arrives.

## Baseline observed while planning

- `main` and `origin/main` are at `931d599`.
- Round 1 is complete; `proto-v0.2.0` points at `1a9fb94`.
- Buf 1.71.0 lint/breaking CI, native x86_64/aarch64 Rust CI, version guards,
  and `docs/proto-freeze-policy.md` are present.
- `determinism.snapstore.v1` remains ignored by `buf breaking`, has a
  12-line placeholder, and the `snapstore` Cargo feature still exposes a
  handwritten facade.
- Generated families are duplicated under root `proto/` and
  `crates/determinism-proto/proto/`; `build.rs` checks those copies.
- Snapshot-store still owns `proto/snapshot_store.proto` and generates client
  and server types from it. Its CI checks out control-plane without an
  explicit ref, so successor planning must re-inspect the live configuration
  rather than repeat the request's now-stale pin claim.

## Deliverables

| Plan file | Implementer outcome |
|---|---|
| `01-decisions-and-invariants.md` | Fixed scope, safety rules, tag ordering, and abort semantics |
| `02-playbook-document.md` | Production promotion playbook and checklist |
| `03-scratch-fixture-and-harness.md` | Fake vendored schema, scratch family, consumer, and repeatable harness |
| `04-dry-run-and-transcript.md` | Executed dry run with honest evidence and limitations |
| `05-resolution-and-successor.md` | Request closeout and gated snapstore successor template |
| `06-acceptance-checklist.md` | Direct acceptance mapping |

## Sequence

1. W0: Reconfirm the baseline and preserve the snapstore vdev state.
2. W1: Write the playbook with the two-release promotion sequence in
   `01-decisions-and-invariants.md`.
3. W2: Add a repeatable scratch harness and fixtures that exercise descriptor
   comparison, generated code, consumer compatibility, and breaking coverage.
4. W3: Run the harness, save a transcript, and correct the playbook wherever
   execution disagrees with prose.
5. W4: Run repository verification and add the playbook-only resolution.

The scratch harness may use temporary git repositories/branches and local
tags, but must not create or push a real `proto-v*` tag or alter the published
freeze ledger.
