# User Guide: Alpha Platform Flow

AiSSURANCE currently ships a **design-partner alpha** workflow that connects a
deterministic planner seam, a replayable safety guardrail, and the risk/control
plane backend.

## Alpha flow

1. `vla_layer` turns a bounded planner request into an `ActionProposal`
2. `safety_layer` evaluates that proposal against safety constraints
3. `control_plane` submits a batch job to `risk_layer`
4. `risk_layer` produces risk events, feature bundles, premiums, and filing artifacts

## CLI entry points

### Risk-only facade

```bash
cargo run -p cli -- demo --json
```

### End-to-end alpha platform demo

```bash
cargo run -p cli -- platform-demo --json
```

## Contracts on the path to v1

The current repo has explicit alpha contracts for:

- `TelemetryFrame`
- `ClaimRecord`
- `RiskEvent`
- `RiskFeatureBundle`
- `ActionProposal`
- `SafetyDecision`
- `SafetyEvent`

These are versioned in `contracts` and are the intended path toward a stable
public interface later in the roadmap.

## Current release posture

- **Risk** is the strongest subsystem today.
- **Safety** is validated through deterministic replay, not edge hardware integration.
- **VLA** is deterministic and bounded, intended to keep integration seams honest.
- **Control plane** is file-backed and internal-only.

## Artifacts and persistence

The alpha control plane stores:

- input payloads
- risk reports
- filing artifacts
- job metadata

This is intentionally simple, but it gives the platform a real persistence
boundary instead of in-memory-only demos.
