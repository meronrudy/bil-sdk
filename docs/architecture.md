# Architecture

BIL is a Rust workspace for domain-neutral evidence commitments, INK receipts,
and structural verification.

The public repository is intentionally small:

- `bil-core`: shared ids, refs, status, and assurance primitives
- `bil-mir`: the domain-neutral MIR graph and replay state shape
- `bil-canonical`: canonical value model, deterministic CBOR, and hash helpers
- `bil-ink`: receipt envelopes and Merkle primitives
- `bil-signers`: signer traits and the L0 software reference signer
- `bil-verify`: structural receipt verification
- `bil-sdk`: ergonomic issue, verify, explain, and generic example helpers
- `bil-cli`: local developer commands over the public SDK
- `bil-conformance`: public structural conformance checks

The public proof path is:

```text
generic MIR graph -> canonical MIR commitment -> receipt preimage
-> canonical receipt commitment -> software signature -> structural report
```

Domain profiles, vendor adapters, hosted services, customer workflows, and
commercial assurance packages live outside the public thin waist.
