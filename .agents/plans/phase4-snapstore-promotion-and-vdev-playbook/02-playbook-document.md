# Production Playbook Document

Workstream W1.

Create `docs/vdev-promotion-playbook.md` with the following executable
sections.

## 1. Preconditions and evidence manifest

Require all of the following before touching a schema:

- owner-ready signal naming the canonical source commit and schema path;
- owner statement that the schema is stable enough to freeze as v1;
- current latest `proto-v*` tag and green CI evidence;
- family is listed vdev in `docs/proto-freeze-policy.md` and ignored at the
  expected path in `buf.yaml`;
- list of root/packaged proto copies, Cargo feature/module paths, and all
  known consumers;
- clean worktrees and recorded SHAs for control-plane and owner repository.

Provide a copyable evidence-manifest template containing those values.

## 2. Staging branch: receive and validate

Give exact repository-relative operations, with the family path parameterized:

1. Copy the owner-authored schema into root `proto/` through an owner-authored
   or owner-approved change. Preserve provenance in the PR description.
2. Add/update the packaged crate copy and extend the copy-staleness check.
3. Extend the existing feature in `Cargo.toml`, `build.rs`, and `src/lib.rs`
   to compile and expose generated types at the same module path.
4. Remove or explicitly deprecate/supersede the handwritten facade. Avoid two
   competing definitions of the same contract.
5. Add focused symbol, streaming-shape, and round-trip tests; add compile
   checks for feature-only and no-default-feature configurations.
6. Run the descriptor comparator against the exact owner commit recorded in
   the evidence manifest.
7. Run Buf lint, full Rust verification, and all identified consumers.

Keep the Buf ignore and vdev ledger entry intact throughout this branch.

## 3. Staging release

Document the repository's required version bump across root workspace,
`determinism-proto`, `PROTO_VERSION`, and lockfile. Land only after PR CI and
owner review are green. Create/push `T_stage` only on the green main commit,
using the tag kind required by the repository's discovered convention. Wait
for the proto and both x86_64/aarch64 Rust jobs, including the tag-context
version guard, and record SHA/run URLs. Label it explicitly as a promotion
baseline, not an adoption signal.

## 4. Freeze branch and release

Branch from the staging tag/commit. Assert the family descriptors are byte- or
semantically identical to `T_stage`. Then:

- remove exactly the family's `breaking.ignore` path;
- move every package covered by that path from pre-release to frozen in the
  ledger (important for multi-package directories such as replay);
- update the playbook changelog/promotion table;
- bump versions for `T_freeze` without editing the schema;
- run all CI and version guards against `T_stage`;
- land, tag, and wait for green tag CI.

If any contract edit is discovered here, return to staging and mint a new
staging release. Do not combine it with activation.

Before both merges and tags, refetch tags and apply D2's serialization,
ancestry, descriptor-identity, and printed-baseline assertions.

## 5. Prove coverage

From `T_freeze`, create a scratch branch that deletes or renumbers a field in
the newly frozen family. Run/push the normal breaking gate, capture the
nonzero output or failing CI URL, then discard the branch. Also show that an
equivalent scratch break in a still-vdev family remains outside protection;
label that result as policy confirmation, not a successful safety test.

## 6. Notify and verify consumers

Provide a handback template with:

- `T_freeze`, target SHA, family/package path, Cargo feature/module path;
- staging and freeze CI URLs plus negative breaking demonstration;
- exact consumer dependency/ref edit derived from its current configuration;
- vendored proto/build-script/re-export changes the consumer owns;
- known source-compatibility edits and commands already run;
- bead/request locations where the signal was recorded.

The promotion is complete when the signal is recorded accurately. Consumer
landing remains consumer-owned unless the successor explicitly includes it.

## 7. Abort matrix and changelog

Include the D4 cases as a compact table and keep a dated changelog. After each
real promotion, record lessons or explicitly state that no correction was
needed, citing the dry-run/real-execution comparison.
