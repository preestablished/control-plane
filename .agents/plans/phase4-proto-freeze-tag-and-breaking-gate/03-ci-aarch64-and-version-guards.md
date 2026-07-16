# CI, aarch64, and Version Guards

Workstream W2.

## W2.1: Restructure Rust CI into an architecture matrix

Update `.github/workflows/ci.yaml` so the Rust job covers x86_64 and aarch64.
Preferred matrix:

```yaml
strategy:
  fail-fast: false
  matrix:
    include:
      - name: x86_64
        runner: ubuntu-latest
      - name: aarch64
        runner: ubuntu-24.04-arm
runs-on: ${{ matrix.runner }}
```

Steps for both lanes:

- `actions/checkout@v4`
- `dtolnay/rust-toolchain@stable`
- `cargo build --workspace --all-features`
- `cargo test --workspace --all-features`

Run `cargo fmt --all -- --check` at least once. It can run only on x86_64 to
avoid duplicate work.

If `ubuntu-24.04-arm` is unavailable for this repository, use self-hosted
arm64 or QEMU/cross as an interim and record the chosen mechanism in
`04-resolution.md`. The final target remains native build and test green on
aarch64.

## W2.2: Add manifest and `PROTO_VERSION` drift check

Replace the hard-coded `PROTO_VERSION` test in
`crates/determinism-proto/src/lib.rs` with a crate-version-derived assertion:

```rust
#[test]
fn exposes_proto_tag_matching_crate_version() {
    assert_eq!(crate::PROTO_VERSION, concat!("proto-v", env!("CARGO_PKG_VERSION")));
}
```

Update the root workspace package version to `0.2.0` unless there is a
documented reason not to. The request explicitly calls out the current
workspace `0.1.0` as drift.

Add `scripts/check-proto-version.sh`:

- Parse `crates/determinism-proto/Cargo.toml` package version.
- Parse root `Cargo.toml` workspace package version, or check for an
  explicit exemption file/comment if the implementer chooses not to align it.
- Parse `PROTO_VERSION` from `crates/determinism-proto/src/lib.rs`.
- Assert `PROTO_VERSION == "proto-v${crate_version}"`.
- Assert workspace package version matches crate version unless exempted.
- On tag-triggered CI runs where `GITHUB_REF_TYPE=tag`, assert
  `GITHUB_REF_NAME == PROTO_VERSION`.

Use structured tooling where practical. `cargo metadata` plus a small shell
or Rust helper is preferable to fragile grep, but a short script is acceptable
if it is easy to read and tested in CI.

## W2.3: Add CI triggers for tag verification

Ensure the workflow runs on `proto-v*` tags:

```yaml
on:
  pull_request:
  push:
    branches:
      - main
    tags:
      - "proto-v*"
```

Run `scripts/check-proto-version.sh` on every workflow run. The tag-specific
assertion activates only for tag refs.

## W2.4: Keep CI output useful

The final workflow should make failures easy to classify:

- `proto` job: Buf install, lint, breaking, fixture self-test, version script.
- `rust` job: x86_64/aarch64 build and test.

Do not hide the aarch64 lane behind `continue-on-error`. If the lane is
interim and non-blocking for some reason, that is a deviation to record in
the resolution before tagging.
