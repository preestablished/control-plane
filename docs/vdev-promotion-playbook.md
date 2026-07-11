# Vdev family promotion playbook

This document promotes an owner-authored protobuf family from the pre-release
set in `docs/proto-freeze-policy.md` to a frozen v1 contract. It is deliberately
mechanical: substitute the values in the evidence manifest, preserve the
ordering, and stop at any failed assertion.

## Responsibilities

- The owning repository stabilizes and authors the schema, identifies its
  canonical commit/path and consumers, and approves any semantic difference.
- Control-plane reviews descriptors, integrates generated code, changes the
  freeze policy, publishes tags, proves the breaking gate, and records the
  handback.
- Each consumer adopts the freeze tag and removes its vendored generation.
  Consumer landing is not part of promotion unless its request says so.

## Evidence manifest and hard preconditions

Copy this block into the promotion request or PR and fill every value:

```text
family/package:
control-plane proto path:
control-plane packaged proto path:
Cargo feature and public Rust module:
owner repository URL/path:
owner commit (immutable SHA):
owner canonical proto root and file:
owner-ready signal location:
owner v1-freeze approval location:
current control-plane SHA:
current latest proto-v tag and target SHA:
current green proto/x86_64/aarch64 CI URLs:
current breaking.ignore path:
known consumers and their checked SHAs:
```

Before editing, verify clean worktrees, fetch all tags, and verify that the
family is both pre-release in `docs/proto-freeze-policy.md` and ignored at the
same path in `buf.yaml`. Inspect the live owner and consumers: do not reuse old
dependency pins, CI checkout refs, build scripts, re-export seams, or bead
status from a request snapshot.

No promotion starts without an owner-ready signal and explicit v1 stability
approval. For snapstore, that means snapshot-store authors/approves its
canonical `determinism.snapstore.v1` schema first.

## Why promotion uses two releases

The prior release contains a placeholder and excludes it with
`breaking.ignore`. Replacing that placeholder and removing the ignore in one
change exposes the intentional vdev break against the prior tag, making CI
red. Do not weaken or bypass Buf. Instead:

1. `T_stage` contains the real schema and codegen but remains ignored/vdev.
2. `T_freeze` contains the identical schema, removes the ignore, and marks the
   family frozen.

`T_stage` is never an adoption signal. Only `T_freeze` is published to
consumers.

Serialize proto promotions. Immediately before each merge and tag, fetch
tags, inspect `git tag --list 'proto-v*' --sort=-v:refname`, and capture the
baseline printed by `scripts/buf-breaking-against.sh`. The freeze baseline
must be `T_stage`, or a newer ancestor containing an equivalent family
descriptor. If another tag wins the version sort, rebase and repeat the
checks. Discover whether the repository uses annotated tags from current
release history; never move a published tag.

## Release A: stage the owner schema

Create a branch from current main and keep the family in both the vdev ledger
and `breaking.ignore`.

1. Bring the schema from the exact owner SHA into the root `proto/` path
   through an owner-authored commit or documented owner approval.
2. Run the semantic comparator:

   ```bash
   cargo run -p proto-descriptor-eq -- \
     --owner-root OWNER_PROTO_ROOT --owner-file OWNER_RELATIVE_FILE \
     --control-root proto --control-file CONTROL_RELATIVE_FILE
   ```

   It compares the target and import closure, discarding only source locations
   and mapping the explicitly named root file. A mismatch aborts promotion.
3. Copy the schema and required imports to
   `crates/determinism-proto/proto/`. Extend `build.rs` feature environment
   tracking, proto selection, and root/packaged-copy matching.
4. Keep the existing Cargo feature and public module path. Add the optional
   prost/tonic dependencies to that feature and expose generated types with
   `tonic::include_proto!`. Remove the handwritten contract definitions, or
   mark a non-conflicting compatibility layer as superseded.
5. Add tests for representative messages, unary/streaming service symbols,
   encode/decode, and the documented stable consumer seam. Generated traits,
   defaults, and field spelling are not automatically source-compatible with
   a handwritten facade; enumerate real consumer edits.
6. Run:

   ```bash
   cargo check -p determinism-proto --no-default-features
   cargo check -p determinism-proto --no-default-features --features FAMILY
   cargo build --workspace --all-features
   cargo test --workspace --all-features
   buf lint
   scripts/buf-breaking-against.sh
   scripts/check-proto-descriptor-eq.sh
   ```

   Also run every consumer named in the manifest at its recorded SHA.
7. Select the next version from current tags and repository policy. Bump the
   root workspace version, `determinism-proto` version, `PROTO_VERSION`, and
   lockfile together. Run `scripts/check-proto-version.sh`.
8. Land only with owner review and green proto, x86_64, and aarch64 jobs. Tag
   the green main commit as `T_stage`; wait for all tag jobs, including the
   tag-context version guard. Record tag object/target SHAs and CI URLs.

## Release B: freeze the staged schema

Branch from `T_stage`. Before any policy edit, rerun the comparator and record
a descriptor hash/diff proving the family schema is unchanged.

1. Remove exactly the family path from `breaking.ignore`.
2. Move every package covered by that path from pre-release to frozen in
   `docs/proto-freeze-policy.md`. A shared directory such as replay may cover
   more than one package; move them together or first split the paths.
3. Make no proto, packaged proto, build, generated-module, or contract-test
   change in this release. Any schema discovery returns the work to Release A
   and requires a new staging tag.
4. Select the following release version and update workspace/crate versions,
   `PROTO_VERSION`, and lockfile.
5. Fetch tags again. Run the full verification set and assert the printed Buf
   baseline is `T_stage` (or an approved newer ancestor with an identical
   descriptor).
6. Land green, tag the main commit as `T_freeze`, and wait for proto,
   x86_64, aarch64, and tag-version checks. Record immutable evidence.

## Prove breaking coverage

From `T_freeze`, create a disposable branch and delete or renumber a released
field in the promoted family. Run `scripts/buf-breaking-against.sh` and, when
practical, push the branch to exercise normal CI. It must fail for that field.
Record the command output or CI URL and discard the branch.

As a policy check, the same experiment in a still-vdev family should remain
outside breaking coverage. That is confirmation of the remaining exemption,
not a successful safety test.

## Abort and recovery

| Point | Required response |
|---|---|
| Before merge/tag | Amend or abandon the branch. The ledger, tags, and consumers remain unchanged. |
| Descriptor mismatch | Stop. Resolve against the owner source with owner review; never edit both copies independently to silence the tool. |
| After `T_stage`, before freeze | Do not freeze or notify. Correct forward in a new staging release and use its tag as the new baseline. Never move/delete the old tag. |
| After `T_freeze` | Compatibility rules are active. Fix forward in a new release; never restore the ignore as an escape hatch. |

Every abort records its reason, last green tag/commit, consumer exposure, and
next action. Because adoption starts only at `T_freeze`, a staging abort
should not require consumer rollback.

## Consumer handback

Record the signal in the owner's bead and request directory using live,
verified instructions:

```text
frozen tag and target SHA:
family/package and proto path:
Cargo feature and public Rust module:
staging/freeze/tag CI URLs:
breaking-deletion proof:
consumer's current dependency/checkout configuration:
exact ref/dependency change:
vendored proto/build-script/re-export removals owned by consumer:
known source edits and commands already verified:
signal locations (bead/request):
```

The re-pin landing is consumer-owned unless explicitly included. Update the
changelog below after the real execution with corrections, or state “none”
and cite the dry-run/real comparison.

## Changelog

- 2026-07-11: Initial playbook. The scratch dry run added explicit imported
  package module wiring, separated stable-seam from generated-only consumer
  assertions, and made two consecutive isolated runs mandatory. Evidence:
  `docs/vdev-promotion-dry-run.md`.
