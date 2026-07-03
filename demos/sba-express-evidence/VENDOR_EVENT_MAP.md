# Vendor Event Map

The CLI normalizer maps this synthetic vendor trace into the current MIR event
vocabulary.

| Synthetic event type | Vendor style | MIR event kind |
| --- | --- | --- |
| `CONSENT_CAPTURED` | Apiture-style digital banking | `ConsentCaptured` |
| `KYB_CHECK_COMPLETED` | Alloy-style KYB | `PolicyEvaluated` |
| `APPLICATION_INTAKE` | Casca-style origination | `BankBranchIntakeStarted` |
| `DOCUMENT_UPLOADED` | Casca-style origination | `DocumentReceived` |
| `AI_DOCUMENT_EXTRACTION` | Casca-style AI extraction | `EvidenceExtracted` |
| `CREDIT_ANALYSIS` | Casca-style credit analysis | `PolicyEvaluated` |
| `HUMAN_UNDERWRITING_REVIEW` | Casca-style underwriting | `HumanReviewed` |
| `PARTNER_ROUTE_OBSERVED` | Infinant-style embedded banking | `VendorRouteObserved` |
| `LOAN_DECISION` | Casca-style decisioning | `DecisionIssued` |

The fixture intentionally stores payload hashes and references, not raw PII,
financial records, document images, or proprietary vendor payloads.
