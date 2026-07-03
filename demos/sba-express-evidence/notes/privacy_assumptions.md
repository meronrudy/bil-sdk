# Privacy Assumptions

The fixture uses synthetic references and payload hashes only.

Assumptions:

- No real borrower, business, owner, bank, or vendor data is present.
- `hmac_demo_*` identifiers are illustrative placeholders.
- Raw application payloads, financial documents, KYB attributes, model prompts,
  model outputs, and underwriting notes are not stored in the fixture.
- The evidence trace demonstrates structure, not data minimization policy.

Any production design would need explicit retention, redaction, encryption,
access-control, and audit logging decisions.
