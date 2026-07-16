# Tag, Handback, and Verification

Workstream W5.

## W5.1: Local final verification before merge/tag

From control-plane:

```bash
cargo fmt --all -- --check
cargo build --workspace --all-features
cargo test --workspace --all-features
buf lint
scripts/buf-breaking-against.sh
scripts/check-buf-breaking-self-test.sh
scripts/check-proto-version.sh
```

From `../reference-workload`, if present:

```bash
cargo test -p m0-proto-client
```

From `../exploration-orchestrator`, if present and practical:

```bash
cargo test --workspace --all-features
```

Record any unavailable sibling checkout in the resolution with the exact
reason.

## W5.2: CI green requirement

Do not tag until GitHub Actions is green on the final main commit for:

- Buf lint
- Buf breaking
- Buf fixture self-test
- version drift check
- Rust x86_64 build/test
- Rust aarch64 build/test
- descriptor-equality test

If a lane is intentionally interim, record the deviation and get sign-off
before the tag.

## W5.3: Scratch breaking demonstration

Before creating `proto-v0.2.0`, prove the gate on a scratch branch from the
final main commit:

1. Create a scratch branch.
2. Delete or renumber a released field in a frozen package, preferably
   `proto/determinism/scorer/v1/scorer.proto` or
   `proto/determinism/orchestrator/v1/orchestrator.proto`.
3. Push the branch or open a PR so CI runs.
4. Confirm the Buf breaking job fails.
5. Save the CI run URL and the field changed.
6. Delete the scratch branch after evidence is captured.

The no-tag bootstrap path should compare against the merge-base with main, so
this demonstration can happen before the first tag. If it cannot, explain why
and run the demonstration immediately after the tag using `proto-v0.2.0` as
the baseline.

## W5.4: Create and push the tag

On the final green commit:

```bash
git tag -a proto-v0.2.0 -m "proto-v0.2.0"
git push origin proto-v0.2.0
```

Then confirm the tag-triggered workflow passes, including
`scripts/check-proto-version.sh` asserting the tag name matches
`PROTO_VERSION`.

Do not move the tag after pushing. If a fatal issue is found, create a new
versioned tag only after phases-track sign-off.

## W5.5: Resolution file

Append
`.agents/requests/phase4-proto-freeze-tag-and-breaking-gate/04-resolution.md`.

Required contents:

- Commit table for each workstream.
- Pinned Buf CLI version and Buf lint category spelling used.
- The vdev ledger summary: frozen packages and pre-release ignored packages.
- aarch64 lane mechanism: native runner, self-hosted, or interim fallback.
- Descriptor-equality decision: duplicate mirror or signed-off alternative.
- Scratch breaking demonstration: branch name, changed field, failing CI URL.
- Final CI run URLs for main and tag.
- Tag name and target SHA.
- Downstream notification list, explicitly including:
  - snapshot-store, because its CI pin predates Phase-4 contracts
  - reference-workload
  - exploration-orchestrator
  - future state-scorer/input-synthesizer bootstrap owners if known

Leave `05-verification.md` for the verifier/phases-track response unless the
local convention changes.
