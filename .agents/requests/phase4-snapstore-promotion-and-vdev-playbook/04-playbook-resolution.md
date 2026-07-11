# Playbook resolution

Resolved the currently actionable playbook half on 2026-07-11.

## Implementation

| Commit | Contents |
|---|---|
| `5a3b4f9` | Operational playbook, policy-ledger link, descriptor comparator, pass/fail fixtures, isolated two-release harness, dry-run transcript, and standing CI comparator check |

The operational entry point is `docs/vdev-promotion-playbook.md`. The recorded
rehearsal is `docs/vdev-promotion-dry-run.md`.

## Evidence

The following passed locally on the implementation commit:

```text
cargo fmt --all -- --check
cargo build --workspace --all-features
cargo test --workspace --all-features
buf lint
scripts/buf-breaking-against.sh
scripts/check-buf-breaking-self-test.sh
scripts/check-proto-descriptor-eq.sh
scripts/check-proto-version.sh
scripts/dry-run-vdev-promotion.sh
```

The workspace tests passed 23 Rust tests plus doc tests. Buf breaking selected
`proto-v0.2.0`. The generic comparator accepted the canonical fixture and
rejected field/type, import/type, and option mismatches. The promotion harness
passed twice, including pre/post consumer compiles, local tag-context version
checks, schema-identical freeze, and a failing post-freeze deletion of field 2
(`PromoteRequest.note`). It verified that caller tag refs, `buf.yaml`, and the
freeze ledger were unchanged.

The proto CI job now runs the fast descriptor comparator self-test on every
PR. The full harness is kept as a required manual promotion rehearsal because
its first isolated Cargo build is materially slower. Hosted CI URLs are not
available until this local commit is published; no remote mutation was part of
this implementation request.

## Playbook corrections from the dry run

- Imported generated packages must be wired into their matching Rust module
  hierarchy.
- Stable-seam compatibility and generated-only prost/tonic assertions must be
  separate checks.
- The entire isolated harness runs twice to prove cleanup and repeatability.

These corrections are recorded in the playbook changelog.

## Scope and acceptance mapping

This resolution satisfies original acceptance criterion 1: the playbook and
recorded dry-run transcript exist. It also completes the dry-run portion of
criterion 4.

Criteria 2 and 3 remain wholly owned by the future
`phase?-snapstore-v1-promotion-execution/` successor. Criterion 4's
post-real-execution comparison/correction also remains successor-owned.

`determinism.snapstore.v1` is unchanged: its placeholder and handwritten
facade remain, its path remains ignored by Buf breaking, and it remains in the
pre-release ledger. No snapstore consumer unpark signal was sent and no real
`proto-v*` tag was created. File the successor only after snapshot-store sends
an owner-authored stable-schema ready signal; that successor must use the
two-release staging/freeze sequence in the playbook.
