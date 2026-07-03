# Receipt Taxonomy

## Receipt Class

`assurance-receipt`

The receipt represents an issued evidence object for a synthetic SBA
Express-style workflow. It commits to the normalized MIR graph and is signed by
the current L0 software development signer.

## Evidence Categories

- Consent capture event
- KYB and fraud screening event
- Application intake event
- AI extraction event
- Credit analysis event
- Human underwriting review event
- Partner route observation
- Loan decision event

## Current Trust Level

The demo uses `L0SoftwareDev`. That is useful for local repeatable demos and
SDK development, but it is not a production signer level.

## Not Claimed

- Bank approval
- Live Oak endorsement
- Production compliance coverage
- HSM, KMS, or TPM backed signing
- Full Merkle inclusion proof verification
