# Proto freeze policy

This ledger defines what `proto-v0.2.0` freezes.

## Frozen packages

- `determinism.common.v1`
- `determinism.controlplane.v1`
- `determinism.orchestrator.v1`
- `determinism.scorer.v1`
- `determinism.inputsynth.v1`

## Pre-release packages

These packages remain vdev/pre-release and are ignored by `buf breaking` for
`proto-v0.2.0`:

- `determinism.hypervisor.v1`
- `determinism.snapstore.v1`
- `determinism.policy.v1`
- `determinism.replay.v1`
- `determinism.replay.agent.v1`
- `determinism.observatory.v1`

`buf.yaml` must ignore exactly these pre-release paths for breaking detection:

- `proto/determinism/hypervisor/v1`
- `proto/determinism/snapstore/v1`
- `proto/determinism/policy/v1`
- `proto/determinism/replay/v1`
- `proto/determinism/observatory/v1`

Vdev packages may be linted, but they are not protected by breaking detection
until a future owner-authored promotion moves them into the frozen ledger.
Promoting a vdev family requires updating this ledger and removing its breaking
ignore path in the same change that freezes it.

The first known consumer of this policy is
`.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/`.

Use [`docs/vdev-promotion-playbook.md`](vdev-promotion-playbook.md) for the
required owner handoff, two-release staging/freeze sequence, codegen migration,
breaking-coverage proof, and consumer notification procedure.
