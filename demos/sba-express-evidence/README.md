# SBA Express Evidence Demo: Cross-Platform Evidence Registry for AI-Assisted Small Business Lending

This demo is a non-production, synthetic scenario showing how `bil-sdk`
can issue and verify INK receipts for a multi-vendor small-business lending workflow.

This scenario is designed for a Live Oak-style small business lending design-partner conversation.

The scenario is inspired by the type of event chain that can exist across:

- digital banking / consent capture
- identity and KYB checks
- AI-native loan origination
- document extraction
- credit analysis
- human underwriting review
- final loan decisioning

No real customer data, bank data, vendor secrets, or production credentials are used.

## Demo Thesis

Vendors execute the workflow. BIL / INK preserves the evidence trail.

## What this demo proves

- A synthetic multi-vendor lending trace can be normalized into BIL MIR.
- The MIR graph can be hashed.
- An INK receipt can be issued.
- The receipt can be signed using Ed25519.
- The verifier can detect signature validity and missing assurance metadata.
- A markdown explanation can summarize the evidence trail.

## Current limitations

- Evidence Merkle root is not yet populated.
- Replay transition hashing is mocked.
- Not all verification checks are implemented.
- CLI commands are under active development.
- This is not a production compliance system.
