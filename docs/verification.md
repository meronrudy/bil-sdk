# Verification

`bil-verify` performs structural verification of INK receipts. It verifies the
receipt envelope and cryptographic consistency, not domain profile semantics.

Current checks include:

- `CanonicalEncodingValid`
- `CommitmentHashMatches`
- `SignatureValid`
- `RequiredReferencePresent`
- `ProfileDeclared`
- `ReceiptEnvelopeValid`
- `SchemaValid`

Status rules:

```text
P0 finding -> Fail
P1 finding -> Warn unless a structural failure already occurred
P2 finding -> Warn or Pass depending on the check
```

The L0 software signer stores its Ed25519 public key as the receipt signer
reference so the public verifier can validate signatures locally.

Domain-specific verification should run after structural verification and
should not be added to `bil-verify`.
