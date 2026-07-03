# SBA Express Evidence Demo

This demo is a non-production, synthetic scenario showing how `bil-sdk` can
issue and verify INK receipts for a multi-vendor small-business lending
workflow.

The scenario is designed for a Live Oak-style design-partner conversation, but
it does not use real Live Oak data, customer data, vendor secrets, production
credentials, or endorsement language.

## Demo Thesis

Vendors execute the workflow. BIL / INK preserves the evidence trail.

## What This Demo Proves

- A synthetic multi-vendor lending trace can be normalized into BIL MIR.
- The MIR graph can be hashed.
- An INK receipt can be issued.
- The receipt can be signed using Ed25519.
- The verifier can check canonical commitment and signature validity.
- A markdown explanation can summarize verification findings.

## Run The Demo

```bash
cargo run -p bil-cli -- mock sba-express-evidence \
  --out demos/sba-express-evidence/fixtures/live_oak_sba_express_trace.json

cargo run -p bil-cli -- issue \
  --input demos/sba-express-evidence/fixtures/live_oak_sba_express_trace.json \
  --out demos/sba-express-evidence/expected/issued_receipt.json \
  --capability assurance-receipt

cargo run -p bil-cli -- verify \
  --receipt demos/sba-express-evidence/expected/issued_receipt.json \
  --out demos/sba-express-evidence/expected/verification_report.json \
  --pretty

cargo run -p bil-cli -- explain \
  --receipt demos/sba-express-evidence/expected/issued_receipt.json \
  --report demos/sba-express-evidence/expected/verification_report.json \
  --out demos/sba-express-evidence/expected/audit_replay_bundle.md
```

The scripts in `scripts/` run the issue, verify, and explain steps.

## Current Limitations

- This is not a production compliance system.
- Replay transition hashing is still mocked.
- The L0 software signer is not suitable for production.
- Full Merkle inclusion proof generation and verification are not claimed here.
- The fixtures are synthetic and intentionally not vendor-authentic payloads.
