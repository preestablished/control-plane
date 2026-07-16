# snapstore/v1 owner-ready signal (stable schema)

Delivered by snapshot-store, 2026-07-16, per the reciprocal handshake
(`snapshot-store/.agents/requests/phase2-closeout-m8-joint-fork-integrity/02-requested-work.md:77-83`:
"whichever side is ready first (their playbook, or this repo's authored
schema) leaves the ready-signal in the other's request dir"). This is the
trigger named in `04-playbook-resolution.md:69` — the
owner-authored stable-schema ready signal control-plane is waiting on before
filing `phase?-snapstore-v1-promotion-execution/`.

```text
signal type:               owner-authored stable-schema ready signal
                           (the trigger named in 04-playbook-resolution.md:69)
family/package:            determinism.snapstore.v1
owner repository:          snapshot-store
                           (git@github.com:preestablished/snapshot-store.git)
owner commit (SHA):        a582bee5abfd0f1bd078e645f2eaa9576e3f966f
                           (pushed to origin/main; verified equal to
                           origin/main at signal time — immutable)
owner canonical proto root/file:  proto/ , snapshot_store.proto (455 lines)
control-plane candidate commit:   not landed — successor copies from owner SHA
                                  (default signal-only path; playbook Release A
                                  step 1 "documented owner approval" clause)
descriptor comparator:     deferred to the successor's Release A — no
                           control-plane copy exists yet (proto/determinism/
                           snapstore/v1/snapshot_store.proto is still the
                           12-line placeholder; comparing against it would
                           rightly fail)
owner-ready signal location:      control-plane/.agents/requests/
                                  phase4-snapstore-promotion-and-vdev-playbook/
                                  05-snapstore-owner-ready-signal.md (this file)
owner v1-freeze approval location: this file, "v1 stability approval" field
                                  below
v1 stability approval:     snapshot-store approves freezing this schema as
                           determinism.snapstore.v1; post-freeze evolution is
                           additive-only per docs/proto-freeze-policy.md.
known consumers:           snapstore-client, snapstore-server (in-repo,
                           build.rs codegen of the vendored copy, both at the
                           owner SHA above). determinism-hypervisor consumes
                           via path deps on snapstore-client/-server/
                           -manifest/-types (checked at
                           b4358a77068dc6534bd08ee5bcf0a1c91a5d82a1) — it does
                           NOT vendor the proto file itself. exploration-
                           orchestrator has no snapstore dependency (checked
                           at ffe93f27636183e18f42ff078130751ff3454494).
                           API.md §5's "hypervisor and orchestrator vendor the
                           file" is not realized as of these SHAs — re-verify
                           pins live at promotion time, do not trust this
                           snapshot.
bead:                      legacy snapstore-8qx absent from live DB (phase-2
                           prefix teardown; survives only in documents) —
                           closure recorded in snapshot-store-98o (closed on
                           this delivery); successor bead snapshot-store-bxg
                           tracks the post-T_freeze re-pin
requested next step:       control-plane files phase?-snapstore-v1-promotion-
                           execution/ per the playbook (criteria 2-3 of
                           02-requested-work.md); two-release staging/freeze
                           sequence mandatory.
```

## Owner-side verification performed (2026-07-16)

- All 21 RPCs of the as-built `proto/snapshot_store.proto` (5 pages/snapshots,
  2 input logs, 6 tree, 3 metadata KV, 5 lifecycle) mapped against
  `crates/snapstore-server/src/service.rs` handlers — every one implemented,
  streaming directions match (PutPages client-stream; ResolvePages and
  QueryNodes server-stream; rest unary). TriggerGc is implemented (M7
  shipped); the vendored file's `// UNIMPLEMENTED until M7` comment is stale
  doc-only drift, tracked in snapshot-store-wz8 and refreshable in the
  successor's landing commit (comments do not affect the descriptor).
- API.md §1 has drifted from the as-built proto in 14 message shapes; code
  wins for v1. Doc-drift bead snapshot-store-wz8 filed owner-side. The RPC
  set itself agrees between doc and code.
- Enum zero values are `*_UNSPECIFIED` (buf STANDARD compliant:
  `NodeStatus`, `QueryOrder`).
- `git log --follow -p proto/snapshot_store.proto` shows no field was ever
  released then removed — no `reserved` statements needed.
- Deliberate absences confirmed as owner intent for v1: no `ReleaseSnapshot`,
  no `ResolveArtifact`, no `ListNodes` (QueryNodes covers listing).
- Error-detail messages (`MissingPages`, `MissingNodes`, `CurrentGeneration`)
  are part of the contract and freeze with the file.
