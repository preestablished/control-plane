# Current Status - 2026-07-10

Round 1 is complete. Control-plane `main` contains its resolution, Buf lint and
breaking gates, committed vdev ledger, hosted aarch64 CI, and published
`proto-v0.2.0` tag. The first hard entry condition in this request is
satisfied.

## Start Now

The playbook and scratch-family dry run described in item 1 are now ungated.
No promotion playbook or dry-run transcript currently exists, so this request
is open rather than resolved.

## Still Gated

The real `snapstore/v1` promotion remains a successor. Snapshot-store still
owns a vendored `proto/snapshot_store.proto`, while control-plane's
`determinism.snapstore.v1` remains in the vdev set recorded by the
`proto-v0.2.0` resolution. Do not freeze a schema until snapshot-store sends an
owner-authored stable schema and ready signal.

The old program-gap statement in this packet is obsolete: `state-scorer` and
`input-synthesizer` both exist, have Phase 0 skeletons, configured GitHub
remotes, and clean `main...origin/main` state.

Close the current request when the playbook half and its required dry run are
complete, as its original scope ruling permits. File the named execution
successor only when snapshot-store is ready.
