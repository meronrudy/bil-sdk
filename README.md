# BIL

BIL, the Base Institutional Language, is the public thin-waist Rust workspace
inside this repository.

BIL does not execute workflows, make decisions, run compliance programs, or
replace domain systems. It defines a small common evidence layer that lets
heterogeneous systems emit canonical proof objects and lets independent
verifiers test those objects.

This repository is broader than that workspace. By tracked Rust LOC in `HEAD`,
the public BIL core is `2,315 / 13,688` lines (16.9%). Most tracked Rust LOC
currently lives in the vendored AiSSURANCE downstream and the live AXIOM
experiment, and the intended post-reorg repo shape also carries archived
insurance experiments. The root workspace remains the supported public core.

The core pattern is:

```text
many systems execute
BIL normalizes evidence
many parties verify
```

## Repo Reality

The root Cargo workspace is the public thin waist. The repository also contains
one supported local downstream, one live standalone experiment, and historical
archive material.

| Support tier | Paths | Status |
| --- | --- | --- |
| Public supported core | `crates/` | Default workspace, default CI, domain-neutral contract |
| Supported local downstream | `Insurance/AiSSURANCE`, `integrations/bil-aissurance-bridge` | Feature-gated local integration path |
| Live standalone experiment | `experiments/axiom-tui` | Not part of root workspace guarantees |
| Historical/archive | `archive/insurance-experiments`, `docs/history` | Reference-only, not default CI |

For the dual-baseline Rust LOC tables and doc inventory methodology, see
[docs/repo-assessment.md](docs/repo-assessment.md).

## What BIL Provides

- Core evidence reference types
- MIR graph representation
- Deterministic canonical encoding
- Commitment hashing
- INK receipt envelopes
- Software signer traits and reference signer
- Structural verification reports
- Domain-neutral CLI tooling
- Public conformance scaffolding

## What BIL Does Not Provide

- Domain-specific policy profiles
- Vendor adapters
- Hosted registry services
- HSM-backed checkpointing
- Commercial assurance packets
- Customer dashboards

Domain-specific products can build on BIL without changing the public thin
waist.

## Public Smoke Flow

```bash
cargo run -p bil-cli -- mock generic --profile human-override \
  --out /tmp/human_override.json

cargo run -p bil-cli -- issue \
  --input /tmp/human_override.json \
  --out /tmp/receipt.json

cargo run -p bil-cli -- verify \
  --receipt /tmp/receipt.json \
  --pretty

cargo run -p bil-cli -- explain \
  --receipt /tmp/receipt.json
```

## Development

```bash
cargo metadata --no-deps --format-version 1
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -p bil-cli -- conformance all
```

Generated artifacts should stay out of the public repo unless they are explicit
fixtures under `examples/` or conformance vectors.
