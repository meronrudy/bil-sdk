# Plan: Integration Tests Across the Full Pipeline

## Overview

End-to-end integration tests that exercise every layer boundary in the system:
telemetry ingestion → risk detection → exposure accumulation → GLM fitting →
pricing → reserving → explainability → governance filing. Tests live in a
dedicated `tests/` workspace crate and use `cios-sim` for deterministic data.

## Test Architecture

```mermaid
graph LR
    SIM[cios-sim] -->|generates| DATA[Synthetic Data]
    DATA --> T1[Ingest Tests]
    T1 --> T2[Risk Tests]
    T2 --> T3[Exposure Tests]
    T3 --> T4[Model Tests]
    T4 --> T5[Pricing Tests]
    T5 --> T6[Reserving Tests]
    T6 --> T7[Explain Tests]
    T7 --> T8[Filing Tests]
    T8 --> T9[Full Pipeline]
    T9 --> ASSERT[Assertions + Snapshots]
```

## Directory Layout

```
tests/
├── integration/
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs              # Shared test harness + helpers
│   ├── tests/
│   │   ├── test_ingest_to_risk.rs
│   │   ├── test_risk_to_exposure.rs
│   │   ├── test_exposure_to_model.rs
│   │   ├── test_model_to_pricing.rs
│   │   ├── test_pricing_to_filing.rs
│   │   ├── test_reserving_pipeline.rs
│   │   ├── test_explain_pipeline.rs
│   │   ├── test_full_pipeline.rs
│   │   ├── test_determinism.rs
│   │   ├── test_edge_cases.rs
│   │   └── test_serde_roundtrip.rs
│   └── snapshots/              # Expected output snapshots
│       ├── filing_ca_2026.json
│       └── premium_result_baseline.json
```

## Test Categories

### 1. Layer Boundary Tests

Each test validates that the output of one layer is valid input for the next.

#### `test_ingest_to_risk.rs`
- Generate synthetic telemetry via `cios-sim::telemetry`
- Feed observations into risk detectors
- Assert: risk events are produced with valid types, timestamps, severity in [0,1]
- Assert: harsh decel events have `decel_mps2 < 0`
- Assert: worker proximity events have `distance > 0`

#### `test_risk_to_exposure.rs`
- Generate risk events via `cios-sim::risk_events`
- Feed into `ExposureAccumulator`
- Assert: `machine_hours > 0`, `autonomous_fraction` in [0,1]
- Assert: totals are self-consistent (`auto_hours + manual_hours ≈ total`)

#### `test_exposure_to_model.rs`
- Generate feature matrix via `cios-sim::features` with known true β
- Fit Poisson GLM
- Assert: model converges
- Assert: recovered coefficients are within 2 standard errors of true β
- Assert: AIC is finite

#### `test_model_to_pricing.rs`
- Use fitted model to predict frequency
- Apply expense loading with default assumptions
- Compute experience modifier and final premium
- Assert: `gross_premium > pure_premium`
- Assert: `final_premium >= minimum_premium`
- Assert: `implied_loss_ratio` in (0, 1)

#### `test_pricing_to_filing.rs`
- Generate a rate filing from fitted model
- Assert: filing has non-empty content hash
- Assert: all significant factors have `p_value < 0.05`
- Assert: filing JSON is valid and re-parseable
- Assert: hash is deterministic (same inputs → same hash)

#### `test_reserving_pipeline.rs`
- Generate loss triangle via `cios-sim::triangles`
- Run chain-ladder
- Assert: LDFs > 1.0 (development is forward)
- Assert: `ultimate >= total_incurred`
- Assert: `ibnr >= 0`
- Assert: `ibnr_as_pct` is reasonable (not > 200%)

#### `test_explain_pipeline.rs`
- Fit a model, then explain a prediction
- Assert: contributions sum to linear predictor (within tolerance)
- Assert: contributions are sorted by absolute magnitude
- Assert: directions are consistent with coefficient signs

### 2. Full Pipeline Tests

#### `test_full_pipeline.rs`
- Use `cios-sim::scenarios::WellManagedFleet` to generate everything
- Run telemetry → risk → exposure → model fit → pricing → filing
- Assert: the pipeline completes without error
- Assert: final premium is positive and finite
- Assert: filing artifact is valid JSON with hash
- Snapshot: compare filing output to baseline snapshot

#### `test_determinism.rs`
- Run full pipeline twice with same seed
- Assert: every intermediate and final output is bit-identical
- This validates `cios-sim` seeding and pure-functional pipeline

### 3. Edge Case Tests

#### `test_edge_cases.rs`
- **Empty data**: zero observations, zero claims → graceful errors
- **Single observation**: minimal data through pipeline
- **Extreme values**: very large/small premiums, zero exposure hours
- **Degenerate triangle**: 1×1 triangle, all-zero row
- **Expense overload**: assumptions that sum to > 100% → `ActuarialError`
- **Non-convergence**: pathological data that prevents GLM convergence
- **All claims closed**: $0 case reserve portfolio

### 4. Serialization Round-trip Tests

#### `test_serde_roundtrip.rs`
- For every key type: serialize → JSON → deserialize → assert equality
- Types tested: `Policy`, `Claim`, `FittedGlm`, `LossTriangle`,
  `ChainLadderResult`, `PremiumResult`, `RateFilingArtifact`,
  `ExposureBundle`, `RiskEvent`, `FeatureContribution`
- Validates that no field is lost through serde boundaries

## Shared Test Harness — `src/lib.rs`

```rust
// Helpers that every integration test uses
pub fn default_rng(seed: u64) -> StdRng;
pub fn assert_approx(left: f64, right: f64, tol: f64);
pub fn load_snapshot(name: &str) -> serde_json::Value;
pub fn assert_snapshot_match(actual: &serde_json::Value, name: &str);
pub fn generate_standard_scenario() -> ScenarioBundle;
```

## Cargo.toml Skeleton

```toml
[package]
name = "cios-integration-tests"
version.workspace = true
edition.workspace = true
publish = false

[dependencies]
engine = { workspace = true }
cios-sim = { path = "../../crates/cios-sim" }
serde = { workspace = true }
serde_json = { workspace = true }
approx = { workspace = true }
rand = "0.8"
```

## Implementation Steps

1. Create `tests/integration/` directory and `Cargo.toml`
2. Add to workspace members
3. Implement shared test harness in `src/lib.rs`
4. Implement `test_ingest_to_risk.rs`
5. Implement `test_risk_to_exposure.rs`
6. Implement `test_exposure_to_model.rs`
7. Implement `test_model_to_pricing.rs`
8. Implement `test_pricing_to_filing.rs`
9. Implement `test_reserving_pipeline.rs`
10. Implement `test_explain_pipeline.rs`
11. Implement `test_full_pipeline.rs`
12. Implement `test_determinism.rs`
13. Implement `test_edge_cases.rs`
14. Implement `test_serde_roundtrip.rs`
15. Create baseline snapshots for snapshot tests
16. Verify all tests pass with `cargo test -p cios-integration-tests`
