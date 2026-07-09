# AiSSURANCE Local Integration

This repository can integrate `Insurance/AiSSURANCE` as a **local downstream**
of BIL without folding any insurance-specific logic into the public thin waist.

## Boundary

- `bil-core`, `bil-mir`, `bil-canonical`, `bil-ink`, `bil-signers`,
  `bil-verify`, `bil-sdk`, and the default `bil-cli` flow remain domain-neutral.
- `integrations/bil-aissurance-bridge` owns the AiSSURANCE-specific mapping from
  platform-demo artifacts into BIL MIR, receipts, and local manifests.
- The bridge crate is intentionally **excluded** from the root workspace so
  default `cargo test` stays aligned to the public BIL crates.
- `bil-verify` remains structural only. AiSSURANCE semantics live in profile
  IDs, capability codes, authority refs, policy refs, evidence kinds, and the
  local evidence manifests written by the bridge.

## Local Feature

Enable the local integration surface through the `bil-cli` package feature:

```bash
cargo test -p bil-cli --features aissurance-local
```

That feature wires `bil-cli` to the local bridge crate at
`integrations/bil-aissurance-bridge/` and to the standalone AiSSURANCE workspace
under `Insurance/AiSSURANCE/`.

## CLI Flow

Run the integrated demo with:

```bash
cargo run -p bil-cli --features aissurance-local -- \
  aissurance platform-demo \
  --data-dir .aissurance-alpha \
  --out-dir artifacts/aissurance \
  --json
```

The bridge runs the existing AiSSURANCE full `platform-demo` flow and emits:

- a planner receipt
- a safety receipt
- a risk receipt
- an aggregate run receipt
- a final `manifest.json`

Outputs are written under:

```text
artifacts/aissurance/<run-id>/
```

The final manifest includes:

- `run_id`
- `profile`
- `job_id`
- planner, safety, risk, and aggregate receipt paths and IDs
- verification report paths
- underlying AiSSURANCE artifact paths for `input.json`, `report.json`,
  `filing.json`, and `job.json`

## Test Commands

Default public path:

```bash
cargo test
```

AiSSURANCE workspace baseline:

```bash
cd Insurance/AiSSURANCE
cargo test --workspace
```

Bridge-only tests:

```bash
cargo test --manifest-path integrations/bil-aissurance-bridge/Cargo.toml
```

Feature-enabled `bil-cli` integration path:

```bash
cargo test -p bil-cli --features aissurance-local
```
