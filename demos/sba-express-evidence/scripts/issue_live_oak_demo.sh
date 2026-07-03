#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT"

cargo run -p bil-cli -- issue \
  --input demos/sba-express-evidence/fixtures/live_oak_sba_express_trace.json \
  --out demos/sba-express-evidence/expected/issued_receipt.json \
  --capability assurance-receipt
