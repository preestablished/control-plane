# Acceptance Checklist

Use this as the closeout checklist before writing `04-resolution.md`.

## Buf gates

- [ ] Root `buf.yaml` exists and is committed.
- [ ] CI installs a pinned Buf CLI version.
- [ ] CI runs `buf lint`.
- [ ] CI runs `buf breaking` against the latest `proto-v*` tag when one
      exists.
- [ ] CI falls back to merge-base with `origin/main` before the first tag.
- [ ] Checkout uses full history or otherwise fetches the required refs/tags.
- [ ] The standing fixture self-test proves a known breaking change fails.
- [ ] The one-time scratch branch demonstration is recorded.

## vdev policy

- [ ] A committed ledger documents frozen packages.
- [ ] The ledger documents pre-release/vdev packages.
- [ ] `buf breaking` ignores only the documented vdev paths.
- [ ] Frozen packages are not in `breaking.ignore`.
- [ ] Snapstore remains unfrozen for the later promotion request.

## aarch64

- [ ] Rust build runs on x86_64.
- [ ] Rust tests run on x86_64.
- [ ] Rust build runs on aarch64.
- [ ] Rust tests run on aarch64.
- [ ] The runner mechanism is recorded in the resolution.

## Orchestrator

- [ ] `determinism.orchestrator.v1` is the real upstream surface, not the
      empty placeholder.
- [ ] Root and packaged orchestrator proto copies match.
- [ ] `orchestrator` feature generates tonic code successfully.
- [ ] Buf lint exceptions for orchestrator are scoped and documented.
- [ ] exploration-orchestrator still builds against this repo, or the skip is
      recorded with a concrete reason.

## `ExperimentSpec` mirror

- [ ] `controlplane/v1` `ExperimentSpec` mirrors orchestrator
      `ExperimentConfig` fields 1 through 16.
- [ ] The transitive message and enum closure mirrors the orchestrator shape.
- [ ] Rust controlplane facade is updated.
- [ ] Descriptor-equality test runs in normal cargo test.
- [ ] reference-workload `m0-proto-client` builds/tests, or required fallout is
      recorded.

## Version and tag

- [ ] `crates/determinism-proto/Cargo.toml` package version is `0.2.0`.
- [ ] Workspace version is `0.2.0` or explicitly exempted.
- [ ] `PROTO_VERSION` is `proto-v0.2.0`.
- [ ] CI checks manifest/constant drift on every run.
- [ ] Tag-triggered CI checks tag name equals `PROTO_VERSION`.
- [ ] `proto-v0.2.0` exists and points at the final green commit.
- [ ] `04-resolution.md` records commits, CI runs, tag, scratch demo, and
      notifications.
