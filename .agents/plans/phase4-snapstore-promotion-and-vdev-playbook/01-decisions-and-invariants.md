# Decisions and Invariants

## D1: The playbook is the product of this request

Place the operational document at `docs/vdev-promotion-playbook.md`. It must
be usable by a cold agent without reading this planning packet. Link it from
`docs/proto-freeze-policy.md`, but do not change any family's frozen/vdev
classification in this request.

The playbook must clearly separate:

- owner responsibilities: stabilize and author the schema, identify the
  canonical vendored source and affected consumers, and approve semantic
  divergence;
- control-plane responsibilities: descriptor review, codegen integration,
  Buf policy transition, release tags, CI evidence, and notification;
- consumer responsibilities: re-pin/adopt and remove vendored generation.

## D2: Promotion uses two releases, not one

Removing a family's `breaking.ignore` while replacing its placeholder makes
Buf compare the real schema to the placeholder in the prior release. That is
the exact intentional vdev break, but it would make the promotion PR red.
Do not bypass or weaken the global breaking gate.

Use this sequence:

1. **Staging release:** receive the owner-authored schema, prove descriptor
   equivalence, add packaged copy/codegen/tests, and keep the family in the
   vdev ledger and Buf ignore list. Bump crate/workspace/`PROTO_VERSION`, land
   green, then publish tag `T_stage`. This tag is explicitly not the consumer
   unpark signal.
2. **Freeze release:** from `T_stage`, make no schema/codegen changes. Remove
   only the family's breaking ignore, move it from pre-release to frozen in
   the ledger, update version constants to the next release, and update
   documentation. CI now compares the unchanged real schema to `T_stage` and
   passes. Land green and publish `T_freeze`.
3. Demonstrate on a scratch branch from `T_freeze` that deleting a field in
   the promoted family makes `scripts/buf-breaking-against.sh` fail.
4. Only `T_freeze` plus the family path/codegen feature is the consumer
   unpark signal.

Choose exact versions at execution time by inspecting all current `proto-v*`
tags and the repository's versioning policy. Never predict tag names in the
playbook. Both tag commits must satisfy `scripts/check-proto-version.sh`, and
both tag workflows must be green. Do not move a published tag.

Serialize proto promotions. Immediately before each merge and tag, fetch and
reinspect tags, require the selected breaking baseline to be an ancestor and
to contain the expected descriptor, and capture the baseline printed by
`scripts/buf-breaking-against.sh`. For the freeze it must be `T_stage`, or a
newer ancestor whose promoted-family descriptor is identical. If an unrelated
tag wins the version sort, stop and rebase/re-plan.

## D3: Descriptor review is semantic and fail-closed

Compare canonical compiled `FileDescriptorSet` data, not text. Compile both
sides with recorded include roots and select the target file/package plus its
dependency closure. Remove only `source_code_info`; map only the explicitly
configured owner root filename to its control-plane filename and rewrite its
dependency references consistently. Preserve all other descriptor semantics,
including fully-qualified types, imports, syntax/edition, JSON names,
defaults, proto3 optional/oneofs, options, map metadata, reservations,
extensions, services, methods, and streaming flags. Fail on duplicate or
unresolved packages. Prefer recursive equality of every remaining descriptor
field over a partial allowlist.

The tool must print an actionable diff and exit nonzero. Any unexplained
difference aborts before the staging release. Fix the owner source or the
control-plane copy through owner review; never edit both independently merely
to make the check green.

## D4: Abort and rollback points

- Before merge/tag: abandon or amend the promotion branch. The family remains
  vdev and consumers remain on their old source.
- After `T_stage`, before `T_freeze`: do not freeze. Correct the schema in a
  new staging release/tag, then use that newest corrected tag as the freeze
  baseline. Never retag or delete a tag to conceal the correction.
- After `T_freeze`: normal protobuf compatibility rules apply. Fix forward in
  a new release; do not restore the ignore path as an emergency escape hatch.
- Consumer adoption happens only after `T_freeze`; therefore an aborted
  staging release must not require downstream rollback.

Record the reason, last green tag/commit, and required next action whenever a
promotion aborts.

## D5: Existing feature consumers must keep their import seam

Promotion changes the existing family feature from handwritten facade to
generated prost/tonic types without renaming the public module path. Update
feature dependency edges, `build.rs`, the packaged proto copy, public module,
and tests together. Compile with:

- no default features;
- the promoted feature alone;
- all features;
- the named minimal consumer; and
- each real consumer identified at preflight.

Source compatibility beyond the documented seam is assessed, not assumed.
Generated prost types can differ from handwritten structs even when wire
descriptors match; report required consumer edits before tagging. Compatibility
claims apply only to the explicitly documented stable seam, not every trait,
default, or field detail of an old handwritten facade.

## D6: Handback data is verified at execution time

The successor must inspect the consumer's current checkout/ref configuration,
dependency declaration, vendored build scripts, re-export modules, and bead
state immediately before notification. The present snapshot-store CI has no
explicit control-plane `ref`, contrary to older request text. The playbook
must teach re-inspection rather than hard-code stale re-pin instructions.
