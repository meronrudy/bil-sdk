# Verification Semantics

The `bil-verify` crate provides structural verification of INK receipts. It ensures that the receipt is cryptographically sound and structurally complete according to the BIL grammar.

## Structural Checks

The public verifier performs the following checks:

- `SchemaValid`: The receipt JSON/CBOR matches the expected schema.
- `CanonicalEncodingValid`: The preimage can be deterministically encoded.
- `CommitmentHashMatches`: The `canonical_commitment` matches the hash of the encoded preimage.
- `SignatureValid`: The signature is valid for the canonical receipt preimage and the declared signer.
- `ChainLinkValid`: (If applicable) The receipt correctly links to a previous state.
- `MerkleInclusionValid`: The `evidence_root` correctly commits to the provided evidence leaves.
- `ReceiptEnvelopeValid`: The overall envelope structure is correct.
- `RequiredReferencePresent`: All required references (issuer, subject, etc.) are present.
- `ProfileDeclared`: A profile is declared (though the public verifier does not validate the profile's domain rules).
- `ExtensionRecognized`: Any extensions used are recognized by the verifier.

## Domain-Specific Verification

The public verifier **does not** perform domain-specific checks. For example, it does not check whether a declared domain workflow has all profile-specific evidence required by that domain.

Domain-specific verification is the responsibility of higher-level profile verifiers that build on top of the structural verification provided by `bil-verify`.
