# Architecture

BIL is a Rust workspace for domain-neutral evidence commitments, INK receipts,
and structural verification.

The root Cargo workspace is intentionally small; the repository is broader:

- `bil-core`: shared ids, refs, status, and assurance primitives
- `bil-mir`: the domain-neutral MIR graph and replay state shape
- `bil-canonical`: canonical value model, deterministic CBOR, and hash helpers
- `bil-ink`: receipt envelopes and Merkle primitives
- `bil-signers`: signer traits and the L0 software reference signer
- `bil-verify`: structural receipt verification
- `bil-sdk`: ergonomic issue, verify, explain, and generic example helpers
- `bil-cli`: local developer commands over the public SDK
- `bil-conformance`: public structural conformance checks

Outside the workspace, this repository also carries:

- `Insurance/AiSSURANCE`: the supported local downstream
- `integrations/bil-aissurance-bridge`: the feature-gated mapping layer
- `experiments/axiom-tui`: a live standalone experiment
- `archive/insurance-experiments`: historical reference code

The public proof path is:

```text
generic MIR graph -> canonical MIR commitment -> receipt preimage
-> canonical receipt commitment -> software signature -> structural report
```

The thin waist is the workspace boundary, not the full repository contents. For
the repo-wide LOC breakdown and support tiers, see
[repo-assessment.md](repo-assessment.md).
