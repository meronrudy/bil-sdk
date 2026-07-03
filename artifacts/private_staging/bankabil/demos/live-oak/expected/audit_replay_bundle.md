# Verification Passed with Warnings

Found 1 issues during verification.

## Findings

* **[P2]** Receipt is signed with L0SoftwareDev key. Not suitable for production.

## Remediation Steps

1. Use a production-grade signer (e.g., KMS or HSM) for production receipts.
