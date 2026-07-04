# Conformance

`bil-conformance` provides executable structural checks for the public BIL
workspace. The current groups are intentionally domain-neutral:

```text
all
canonical-encoding
receipt-roundtrip
signature-verification
merkle-proof
merkle-root
verification-semantics
```

Run all public conformance checks through the CLI:

```bash
cargo run -p bil-cli -- conformance all
```

Conformance currently covers:

- canonical map ordering
- receipt JSON roundtrip stability
- receipt commitment over explicit preimage bytes
- software signature verification
- evidence Merkle root mutation behavior
- generated Merkle proof verification
- structural verifier pass/fail semantics

Profile-specific conformance belongs outside public BIL.
