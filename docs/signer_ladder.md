# Signer Ladder

Implement in this order:

| Stage | Signer              | Claim allowed                       |
| ----: | ------------------- | ----------------------------------- |
|     1 | `SoftwareDevSigner` | software-signed development receipt |
|     2 | `SandboxHsmSigner`  | sandbox-signed receipt              |
|     3 | `KmsSigner`         | KMS-backed receipt                  |
|     4 | `Pkcs11HsmSigner`   | HSM-backed receipt                  |
|     5 | `TpmSigner`         | TPM-backed receipt                  |
|     6 | `WitnessSigner`     | multi-party witnessed receipt       |

Do not claim HSM/TPM support until the adapter exists and has conformance tests.
