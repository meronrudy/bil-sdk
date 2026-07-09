# Risk Layer API Reference

The `risk_layer` crate is the current public Rust-facing facade for batch risk
processing inside the alpha platform.

## Core types

### `RiskLayerConfig`

```rust
pub struct RiskLayerConfig {
    pub feature_window_days: u32,
    pub feature_version: FeatureVersion,
    pub base_frequency: f32,
    pub base_severity: f32,
    pub expense_loading: f32,
}
```

Defaults:

- `feature_window_days = 30`
- `feature_version = 1.0`
- `base_frequency = 0.10`
- `base_severity = 12000.0`
- `expense_loading = 0.20`

### `RiskLayerInput`

```rust
pub struct RiskLayerInput {
    pub telemetry_frames: Vec<TelemetryFrame>,
    pub claims: Vec<ClaimRecord>,
}
```

### `RiskLayerReport`

```rust
pub struct RiskLayerReport {
    pub frames_ingested: usize,
    pub frames_rejected: usize,
    pub observations: usize,
    pub claims: usize,
    pub risk_events: Vec<RiskEvent>,
    pub feature_bundles: Vec<RiskFeatureBundle>,
    pub model: ModelReport,
    pub premiums: Vec<Premium>,
    pub explainability: ExplainabilityResult,
}
```

### `ModelReport`

```rust
pub struct ModelReport {
    pub used_fallback_model: bool,
    pub expected_frequency: f32,
    pub expected_severity: f32,
    pub frequency_intercept: f32,
    pub severity_intercept: f32,
    pub frequency_converged: bool,
    pub severity_converged: bool,
}
```

## Main API

### `RiskLayer::new`

```rust
let layer = RiskLayer::new(RiskLayerConfig::default());
```

### `RiskLayer::run_batch`

```rust
let input = RiskLayerInput::new(frames, claims);
let report = layer.run_batch(input)?;
```

Behavior:

- normalizes telemetry frames into canonical observations
- skips malformed frames and increments `frames_rejected`
- detects `RiskEvent`s
- aggregates `RiskFeatureBundle`s
- fits frequency/severity models when possible
- falls back to bounded deterministic defaults when data is sparse
- calculates premiums and explainability summaries

## Errors

```rust
pub enum RiskLayerError {
    Aggregation,
    UnsupportedWindow(u32),
    EmptyFeatures,
}
```

## Notes

- The alpha service boundary above this crate lives in `control_plane`
- Streaming/network APIs are not part of the current crate yet
- The current stable path is the Rust batch facade, not a public hosted API
