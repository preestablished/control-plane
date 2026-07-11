# Scratch Fixture and Harness

Workstream W2.

The dry run must exercise mechanics, not merely walk through the prose.
Prefer a script such as `scripts/dry-run-vdev-promotion.sh` that creates a
temporary git repository/worktree and cleans it through a trap. It must never
mutate real tags, `docs/proto-freeze-policy.md`, or `buf.yaml` in the caller's
checkout.

## Fixture layout

Add committed inputs under `ci/vdev-promotion-fixtures/`:

```text
owner/proto/determinism/scratch/v1/scratch.proto
placeholder/proto/determinism/scratch/v1/scratch.proto
consumer/Cargo.toml
consumer/src/main.rs
consumer/src/generated.rs
patches/handwritten-seam.patch
patches/generated-feature.patch
```

- The placeholder should be a realistic handwritten-facade-era stub.
- The fake owner schema must contain a service, unary and streaming RPCs, an
  enum, optional/repeated fields, and enough message structure to expose
  descriptor comparison mistakes.
- The consumer must depend on a temporary/path copy of `determinism-proto`
  and enable only `scratch`. Its stable-seam program compiles before and after
  promotion using only promised-compatible APIs. A separate post-promotion
  module round-trips a prost message and references a generated client/server
  type; generated-only APIs are not required from the handwritten pre-state.
- Commit explicit before/after patch templates, or equivalently precise and
  asserted transformations, for Cargo features, `build.rs`, copy matching,
  and `src/lib.rs`.

Do not add `scratch` to the production crate, ledger, or default features.
The harness should patch a temporary copy using the same edits a real family
requires.

## Required harness phases

1. Copy the repository to an isolated temp git repo and seed a placeholder
   baseline plus vdev ignore/ledger entry.
2. Compile the placeholder feature/consumer before promotion.
3. Stage the owner schema, packaged copy, generated feature, and facade
   supersession while the ignore remains.
4. Run a reusable descriptor-equivalence checker against the fake vendored
   source; save a passing result.
5. Deliberately test field/RPC, import/referenced-type, and descriptor-option
   mismatches and prove the comparator fails, then restore the valid schema.
6. Run `cargo check` for no defaults, scratch-only, consumer, and all
   production features applicable in the temp copy.
7. Create a local staging tag compatible with the copied version/tag guard;
   simulate tag context and assert the version guard passes.
8. In a schema-identical freeze commit, remove the scratch ignore and update
   the temporary ledger; prove Buf breaking passes against the staging tag.
9. Create a local freeze tag, delete a field, and prove Buf breaking fails.
10. Restore/exit and assert the caller checkout/tag list is unchanged.
11. Run the entire harness a second time to prove repeatability and cleanup.

If reusing `scripts/buf-breaking-against.sh`, isolate environment variables
so it selects the intended local baseline and cannot fetch/push. It is also
acceptable to invoke the exact underlying `buf breaking` command in the temp
repo, provided the transcript calls out that CI baseline-selection logic was
not exercised.

## Reusable comparator

Do not hide descriptor comparison solely inside the demo. Add a production-
usable script/tool parameterized by owner proto root/file and control-plane
root/file. Its normal CI self-test must cover equality plus field/RPC,
import/type, and option mismatches. Apply D3's canonicalization contract, and
give fixture sides different root filenames so path mapping is exercised.

## Safety and repeatability

- Use `mktemp -d` and `trap` cleanup. Prefer `git archive` or a disposable
  worktree over copying `.git` or `target/`.
- Require pinned/known `buf`, Rust, and protoc tooling; print versions.
- Avoid network access except dependency resolution already normal for Cargo.
- Exit nonzero if an expected failure unexpectedly succeeds.
- Record caller tag names and object IDs before/after so moved refs are caught.
- Support retaining the temp directory via an opt-in environment variable for
  debugging, while defaulting to cleanup.

In the temporary crate, explicitly patch optional prost/tonic dependencies,
the `scratch` feature edges, build-script env tracking and proto inclusion,
packaged-copy matching, and `tonic::include_proto!`. Check:

```bash
cargo check -p determinism-proto --no-default-features
cargo check -p determinism-proto --no-default-features --features scratch
cargo check -p determinism-proto --all-features
```
