# Construction Insurance Operating System

A complete, production-grade actuarial and risk measurement system for
construction fleet insurance. Built in Rust for correctness, auditability,
and regulatory defensibility.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Thin Waist (no_std core)                     │
│  ctw-core │ ctw-geo │ ctw-machine │ ctw-ingest │ ctw-risk     │
│  ctw-exposure │ ctw-context │ ctw-serde                        │
├─────────────────────────────────────────────────────────────────┤
│                    Actuarial Engine (std)                       │
│  actuarial-core │ actuarial-model │ actuarial-pricing          │
│  actuarial-reserving │ actuarial-explain │ actuarial-governance │
├─────────────────────────────────────────────────────────────────┤
│                    Integration (engine)                         │
│  Complete pipeline: telemetry → risk → pricing → filing        │
└─────────────────────────────────────────────────────────────────┘
```

## Build

```bash
cargo build --release
cargo test
cargo clippy
```

## Crate Map

| Crate | Domain | std? | Purpose |
|-------|--------|------|---------|
| ctw-core | Thin waist | no_std | IDs, units, time, confidence, bounded values |
| ctw-geo | Thin waist | no_std | Site geometry, zones, polygons, distance |
| ctw-machine | Thin waist | no_std | Machine capability traits, kinematics |
| ctw-ingest | Thin waist | no_std | Raw frame → canonical observation pipeline |
| ctw-risk | Thin waist | no_std | Risk event detectors, accumulators |
| ctw-exposure | Thin waist | no_std | Exposure counters, duty cycles |
| ctw-context | Thin waist | no_std | Site/weather/task/operator context |
| ctw-serde | Thin waist | optional | Serialization bridges (serde feature-gated) |
| actuarial-core | Actuarial | std | Coverage, policies, claims, loss types |
| actuarial-model | Actuarial | std | GLM fitting (Poisson, Gamma, Tweedie) |
| actuarial-pricing | Actuarial | std | Expense loading, credibility, premium calc |
| actuarial-reserving | Actuarial | std | Loss triangles, chain-ladder, IBNR |
| actuarial-explain | Actuarial | std | Attribution, counterfactuals, reporting |
| actuarial-governance | Actuarial | std | Filing artifacts, audit trails, hashing |
| engine | Integration | std | End-to-end pipeline orchestration |
