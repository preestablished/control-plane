# Acceptance Checklist

## Scope protection

- [ ] `snapstore/v1` placeholder and handwritten facade are unchanged.
- [ ] Snapstore remains in the vdev ledger and Buf ignore list.
- [ ] No real `proto-v*` tag or remote scratch branch was created.
- [ ] No snapstore unpark signal was sent.

## Playbook

- [ ] `docs/vdev-promotion-playbook.md` is executable by a cold agent.
- [ ] Owner/control-plane/consumer responsibilities are distinct.
- [ ] Descriptor comparison is semantic, reusable, and fail-closed.
- [ ] The two-release staging/freeze ordering settles the old-tag footgun.
- [ ] Abort behavior is defined before staging, after staging, and after freeze.
- [ ] Ledger, Buf ignore, codegen, facade, tests, versioning, and notifications
      are all covered.
- [ ] `docs/proto-freeze-policy.md` links to the playbook without changing
      family classifications.

## Scratch dry run

- [ ] Fake owner-vendored and placeholder schemas are committed as fixtures.
- [ ] A minimal consumer compiles before and after generated-code promotion.
- [ ] Comparator success and intentional failure are both proven.
- [ ] Field/RPC, import/type, and option mismatch cases all fail closed.
- [ ] Staging while ignored succeeds.
- [ ] Freeze against the schema-identical staging tag succeeds.
- [ ] A post-freeze field deletion fails Buf breaking.
- [ ] Harness cleanup proves the caller's tags and policy files are unchanged.
- [ ] A second full harness run proves repeatability.
- [ ] Transcript records tools, commands, results, and honest limitations.

## Verification and closeout

- [ ] Existing Rust build/tests, Buf lint/breaking, fixture self-test, and
      version guard pass.
- [ ] Comparator self-tests are a standing CI step; the full harness is also
      CI-wired or its measured-runtime exemption is recorded.
- [ ] Playbook changelog reflects dry-run lessons or evidence-backed “none.”
- [ ] `04-playbook-resolution.md` maps commits/evidence to this checklist.
- [ ] Resolution names the gated snapstore execution successor and does not
      falsely claim the full request's real-promotion criteria are complete.
