#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT"

cargo run -p bil-cli -- explain \
  --receipt demos/sba-express-evidence/expected/issued_receipt.json \
  --report demos/sba-express-evidence/expected/verification_report.json \
  --out demos/sba-express-evidence/expected/audit_replay_bundle.md
