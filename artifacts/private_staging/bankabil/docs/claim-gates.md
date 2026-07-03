# Claim Gates

| Claim                        | Required implementation                                                       |
| ---------------------------- | ----------------------------------------------------------------------------- |
| **BIL MIR**                  | schema + validator                                                            |
| **canonical commitment**     | dCBOR/canonical encoder + test vectors                                        |
| **INK Receipt**              | receipt envelope + canonical commitment                                       |
| **signed receipt**           | signer trait + signature verification                                         |
| **Merkle proof**             | Merkle tree + proof verifier                                                  |
| **verified receipt**         | verification engine checks signature, commitment, schema, authority, evidence |
| **deterministic replay**     | replay state + transition commitments                                         |
| **explainable verification** | diagnostic engine with remediation                                            |
| **HSM/TPM/KMS-backed**       | actual signer adapter + tests                                                 |
| **Python SDK**               | PyO3 bindings calling Rust trust core                                         |
