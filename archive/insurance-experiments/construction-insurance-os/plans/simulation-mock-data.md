# Plan: Simulation & Mock Data Generators — `crates/cios-sim`

## Overview

A library crate that produces realistic, reproducible synthetic data for
every layer of the system: telemetry observations, risk events, exposure
bundles, policies, claims, loss triangles, and feature matrices. All
generators are seeded via `rand` for deterministic reproducibility.

## Design Principles

1. **Domain-faithful** — Generated data must respect real-world invariants
   (e.g., claim dates after policy effective dates, severity > 0, loss
   triangles monotonically increasing along development).
2. **Composable** — Each generator is independent but can be chained
   into a full pipeline scenario.
3. **Seeded** — Every generator accepts an `rng: &mut impl Rng` for
   reproducibility in tests and benchmarks.
4. **Configurable** — `SimConfig` structs control distribution parameters,
   fleet size, time horizons, frequency/severity ratios.

## Crate Layout

```
crates/cios-sim/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Re-exports + SimConfig
│   ├── config.rs           # SimConfig, ScenarioProfile
│   ├── fleet.rs            # Machine fleet generation
│   ├── sites.rs            # Construction site geometry generation
│   ├── telemetry.rs        # Observation stream generation
│   ├── risk_events.rs      # RiskEvent batch generation
│   ├── exposure.rs         # ExposureBundle generation
│   ├── policies.rs         # Policy portfolio generation
│   ├── claims.rs           # Claim generation with frequency/severity
│   ├── triangles.rs        # Loss triangle generation
│   ├── features.rs         # GLM feature matrix generation
│   └── scenarios.rs        # Pre-built scenario bundles
```

## Generator Catalog

### Fleet Generator — `fleet.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_machines` | `usize` | 50 | Number of machines |
| `machine_classes` | `Vec<MachineClass>` | excavator, loader, crane, haul truck | Mix of machine types |
| `class_weights` | `Vec<f64>` | 0.3, 0.25, 0.2, 0.25 | Proportion per class |

Produces: `Vec<Machine>` with IDs, classes, capabilities.

### Site Generator — `sites.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_sites` | `usize` | 5 | Number of construction sites |
| `zones_per_site` | `Range<usize>` | 3..8 | Geofence zones |
| `workers_per_site` | `Range<usize>` | 10..50 | Workers on site |

Produces: `Vec<Site>` with polygons, zones, worker counts.

### Telemetry Generator — `telemetry.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `duration_hours` | `f64` | 2000.0 | Total observation hours |
| `sample_rate_hz` | `f64` | 10.0 | Observations per second |
| `anomaly_rate` | `f64` | 0.02 | Fraction of anomalous obs |

Produces: `Vec<Observation>` — motion, pose, proximity, zone events.

### Risk Event Generator — `risk_events.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_events` | `usize` | 500 | Total risk events |
| `event_type_weights` | `HashMap<RiskEventType, f64>` | Realistic mix | Proportion per type |
| `severity_distribution` | `SeverityDist` | Beta(2,5) | Severity distribution |

Produces: `Vec<RiskEvent>` with realistic timestamps and severity.

### Exposure Generator — `exposure.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `total_hours` | `f64` | 25000.0 | Machine-hours |
| `autonomous_fraction` | `f64` | 0.15 | Fraction in autonomous mode |
| `night_fraction` | `f64` | 0.20 | Fraction during night |

Produces: `ExposureBundle`.

### Policy Generator — `policies.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_policies` | `usize` | 200 | Number of policies |
| `term_months` | `usize` | 12 | Policy term |
| `premium_range` | `Range<f64>` | 5000..50000 | Written premium range |

Produces: `Vec<Policy>`.

### Claim Generator — `claims.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `policies` | `&[Policy]` | required | Policies to attach claims to |
| `frequency` | `f64` | 0.05 | Claims per policy |
| `severity_mean` | `f64` | 15000.0 | Mean claim severity |
| `severity_cv` | `f64` | 1.5 | Coefficient of variation |

Produces: `Vec<Claim>` with realistic development patterns.

### Triangle Generator — `triangles.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_origins` | `usize` | 10 | Accident periods |
| `n_development` | `usize` | 10 | Development periods |
| `base_premium` | `f64` | 1_000_000.0 | Base volume |
| `loss_ratio` | `f64` | 0.65 | Expected loss ratio |
| `development_pattern` | `Vec<f64>` | Industry standard | CDF pattern |

Produces: `LossTriangle` with realistic cumulative development.

### Feature Matrix Generator — `features.rs`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n_observations` | `usize` | 500 | Number of rows |
| `n_features` | `usize` | 6 | Number of features |
| `true_coefficients` | `Vec<f64>` | `[0.5, -0.3, 0.2, ...]` | Ground truth β |
| `family` | `Family` | Poisson | Response distribution |

Produces: Design matrix `X`, response `y`, exposure offsets.

### Pre-built Scenarios — `scenarios.rs`

```rust
pub enum ScenarioProfile {
    /// Low-risk fleet with telematics, few claims
    WellManagedFleet,
    /// High-risk fleet: night ops, proximity events, many claims
    HighRiskUrbanSite,
    /// Mixed fleet for credibility testing
    MixedPortfolio,
    /// Minimal data for edge-case testing
    SparseData,
}

pub fn generate_scenario(profile: ScenarioProfile, seed: u64) -> ScenarioBundle;
```

A `ScenarioBundle` contains all generated data needed to run the full pipeline.

## Cargo.toml Skeleton

```toml
[package]
name = "cios-sim"
version.workspace = true
edition.workspace = true

[dependencies]
engine = { workspace = true }
rand = "0.8"
rand_distr = "0.4"
serde = { workspace = true }
serde_json = { workspace = true }
chrono = { workspace = true }

[dev-dependencies]
approx = { workspace = true }
```

## Implementation Steps

1. Create `crates/cios-sim/` directory and `Cargo.toml`
2. Add `cios-sim` to workspace members and workspace dependencies
3. Implement `config.rs` — `SimConfig` and `ScenarioProfile`
4. Implement `fleet.rs` — machine fleet generation
5. Implement `sites.rs` — site + zone geometry generation
6. Implement `telemetry.rs` — observation stream generation
7. Implement `risk_events.rs` — risk event batch generation
8. Implement `exposure.rs` — exposure bundle generation
9. Implement `policies.rs` — policy portfolio generation
10. Implement `claims.rs` — claim generation with freq/sev model
11. Implement `triangles.rs` — loss triangle generation
12. Implement `features.rs` — GLM feature matrix generation (with known β)
13. Implement `scenarios.rs` — pre-built scenario bundles
14. Unit tests for each generator verifying invariants
