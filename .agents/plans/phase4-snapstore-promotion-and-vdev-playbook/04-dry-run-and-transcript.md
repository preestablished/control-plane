# Dry Run and Transcript

Workstream W3.

Run the scratch harness from a clean control-plane checkout and commit
`docs/vdev-promotion-dry-run.md`.

## Transcript contents

Record:

- date, control-plane SHA, OS/architecture, Rust/Cargo/Buf/protoc versions;
- exact harness command and fixture paths;
- preflight proof that real tags, ledger, and `buf.yaml` were unchanged;
- descriptor equality pass and intentional-divergence failure excerpt;
- placeholder consumer compile and generated consumer compile;
- staging-tag baseline selected for the freeze step;
- freeze step passing with an unchanged schema;
- post-freeze deletion failing Buf breaking, including the field deleted;
- exit status of every phase and final cleanup/integrity checks.
- results of the second complete run proving repeatability.

Keep output excerpts short but include enough error text to prove failures
occurred for the expected reason. If full logs are useful, store a stable
artifact path or CI URL rather than pasting pages into the doc.

## Honest limitations section

Enumerate every production step the scratch cannot prove, including at least:

- GitHub branch protection and hosted tag workflows;
- owner approval and cross-repository commit provenance;
- actual version-number selection and remote tag publication;
- real downstream re-pin/adoption and bead/request updates;
- snapstore-specific source compatibility and its client/server build swap;
- notification delivery.

Do not describe the dry run as end-to-end without this qualification.

## Correct the playbook

Compare observed steps/output to `docs/vdev-promotion-playbook.md`. Amend the
playbook for every discrepancy, then add a changelog entry listing the deltas
or “none needed” with a link to the transcript evidence.

Run at minimum after documentation/harness changes:

```bash
cargo fmt --all -- --check
cargo build --workspace --all-features
cargo test --workspace --all-features
buf lint
scripts/buf-breaking-against.sh
scripts/check-buf-breaking-self-test.sh
scripts/check-proto-version.sh
scripts/dry-run-vdev-promotion.sh
```

Also run any focused tests for the descriptor comparator. If markdown lint or
link checking exists on the implementation branch, run it too.

Wire a standing comparator fixture self-test into the `proto` CI job. Run the
full harness there too if its measured runtime is reasonable; otherwise keep
the full run as recorded one-time evidence and make the fast comparator suite
mandatory on every PR. Record that runtime-based choice.
