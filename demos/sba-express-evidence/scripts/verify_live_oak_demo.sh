#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT"

cargo run -p bil-cli -- verify \
  --receipt demos/sba-express-evidence/expected/issued_receipt.json \
  --out demos/sba-express-evidence/expected/verification_report.json \
  --pretty
