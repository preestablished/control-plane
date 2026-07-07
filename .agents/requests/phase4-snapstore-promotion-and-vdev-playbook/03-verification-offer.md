# Choreography And Handback

## With snapshot-store

- Their round-2 request (`phase2-closeout-m8-joint-fork-integrity/` —
  their Phase-2 close-out) keeps `8qx` parked and names this request
  as the receiving side. When their schema authorship day comes
  (possibly M9/Phase-8-era — see the split in `02-`), the PR flows:
  they author → you review per the playbook → joint green → `8qx`
  unparked. Whichever side finishes its half first leaves the
  ready-signal in the other's request dir — and the phases track has
  mirrored this protocol into *their* request dir so both sides'
  texts state the same handshake.
- We verify item 3's re-pin by checking their CI config post-change.

## Phases-Track Verification

On your resolution we will:

1. execute the playbook's dry-run steps ourselves on a scratch branch
   from the doc alone (the cold-agent test — if we need context the
   doc doesn't give, the playbook fails its own bar);
2. run the scratch-branch field-deletion demonstration in the promoted
   family and confirm red CI;
3. build a downstream consumer (reference-workload's
   `m0-proto-client` pattern) against the tag with the `snapstore`
   feature and confirm it compiles.

## Handback Shape

Playbook-only progress (entry condition 1 met, condition 2 pending)
may resolve early as `04-playbook-resolution.md`; the snapstore
execution follows as its own numbered resolution. We respond with
verification files per the usual convention.

## Contact / Tracking

- Downstream bead: `snapstore-8qx`; their round-2 request dir.
- Predecessor: round-1 (`phase4-proto-freeze-tag-and-breaking-gate/`).
- Program-gap escalation carried alongside: scorer/synth repo
  bootstrap (work-order note; operator decision).
