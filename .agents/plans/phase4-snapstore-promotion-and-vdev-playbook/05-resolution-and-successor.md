# Resolution and Gated Successor

Workstream W4.

## Close the current request

Add
`.agents/requests/phase4-snapstore-promotion-and-vdev-playbook/04-playbook-resolution.md`
after the playbook and dry run land.

Record:

- commit(s) containing the playbook, comparator/harness, fixtures, transcript,
  and policy-ledger link;
- CI run URL and local verification commands/results;
- dry-run pass/fail evidence and all enumerated limitations;
- playbook changelog corrections learned from the run;
- explicit statement that snapstore remains vdev, ignored by breaking, on its
  placeholder/facade, and that no consumer unpark signal was sent;
- name of the future execution request and its owner-ready entry condition.
- explicit acceptance mapping: this closeout satisfies original AC1; AC2 and
  AC3 remain successor-owned, while AC4's real-execution correction remains
  successor-owned and only its dry-run changelog portion is complete here.

Leave the request's verifier response file to the phases-track verifier unless
local convention explicitly assigns it to the implementer.

## Successor template (do not file yet)

When snapshot-store sends a ready signal, file a separate request such as
`phase?-snapstore-v1-promotion-execution/` citing:

- `docs/vdev-promotion-playbook.md` and its current changelog;
- the owner-ready signal, snapshot-store SHA, and canonical schema path;
- current control-plane main/tag/CI state;
- current snapshot-store dependency, workflow checkout refs, vendored build
  scripts, re-export seams, and `snapstore-8qx` bead state;
- two-release staging/freeze version choices made from tags that exist then;
- exact cross-repo verification and handback locations.

The successor owns real schema receipt, codegen conversion, staged baseline
tag, freeze tag, negative breaking proof, notification, and post-execution
playbook correction. Snapshot-store owns authoring and its re-pin/adoption
landing unless the successor explicitly expands scope.
