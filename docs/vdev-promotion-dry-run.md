# Vdev promotion dry-run transcript

Date: 2026-07-11 UTC

## Environment

- Control-plane SHA before implementation: `931d5998bf51f00667d19468928988a0785eaa2f`
- Rust: `rustc 1.97.0 (2d8144b78 2026-07-07)`
- Cargo: `cargo 1.97.0 (c980f4866 2026-06-30)`
- Buf: `1.71.0`
- Protoc: supplied by `protoc-bin-vendored 3.2.0`
- Host: Linux x86_64

Command:

```bash
scripts/dry-run-vdev-promotion.sh
```

The harness creates disposable git repositories under `mktemp`, using the
committed fake owner, placeholder, generated-crate, and consumer fixtures in
`ci/vdev-promotion-fixtures/`. It runs twice and compares the caller's tag
names/object IDs plus SHA-256 hashes of `buf.yaml` and the freeze ledger before
and after.

## Results

Both runs completed with exit status 0.

- The handwritten stable-seam consumer compiled before promotion.
- The canonical owner and differently named control-plane schema compiled to
  equivalent descriptor sets.
- Field/type, imported-type, and descriptor-option mutations were each
  rejected by the comparator.
- Staging retained the scratch Buf ignore, compiled no-default,
  scratch-feature-only, and all-feature crate configurations, then ran both
  stable-seam and generated prost/tonic consumers.
- Tag-context version checks passed for local `proto-v0.0.1` (`T_stage`) and
  `proto-v0.0.2` (`T_freeze`). These tags existed only in disposable repos.
- Freeze removed the scratch ignore and updated its temporary ledger without
  changing the schema; Buf passed against `T_stage`.
- Deleting `PromoteRequest.note` after `T_freeze` failed with:

  ```text
  Previously present field "2" with name "note" on message
  "PromoteRequest" was deleted.
  ```

- The second complete run repeated the same pass/fail behavior.
- Final integrity check reported: `dry run passed twice; caller tags and policy
  files are unchanged`.

The first run populated Cargo dependencies and was materially slower than the
fast comparator suite. CI therefore runs `scripts/check-proto-descriptor-eq.sh`
on every PR; the full two-release harness remains a recorded promotion
rehearsal and a required manual pre-promotion check.

## Corrections learned

The initial harness exposed that a generated message referencing an imported
package needs that package included at the matching Rust module path. The
fixture and playbook now require imported-package module wiring. It also
confirmed that one program cannot simultaneously prove a handwritten pre-state
and use generated-only prost/tonic APIs, so those checks are separate.

## Limitations

The dry run does not prove GitHub branch protection, hosted tag workflows,
owner approval/provenance, remote tag publication, actual version selection,
real downstream re-pin/adoption, bead/request updates, snapstore-specific
client/server build removal, or notification delivery. Those require the
owner-ready execution successor. It does prove the local descriptor, codegen,
consumer, version-guard, two-release Buf baseline, negative break, cleanup,
and repeatability mechanics.
