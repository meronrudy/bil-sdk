# AiSSURANCE

AiSSURANCE is an operating and risk intelligence stack for mixed-autonomy
construction fleets. The current repository is a **design-partner alpha**
platform skeleton: the risk layer is production-shaped, while safety and VLA
now exist as deterministic, compileable seams that support replay, orchestration,
and end-to-end platform demos.

## Current Product Surface

### What is real today

- **`risk_layer`**: batch telemetry-to-risk facade with contracts, modeling,
  premium calculation, explainability, and filing artifact generation.
- **`control_plane`**: internal job API with file-backed artifact persistence
  for alpha batch execution and report retrieval.
- **`safety_layer`**: deterministic replay guardrail that evaluates VLA
  proposals, emits safety decisions/events, and enforces an alpha latency budget.
- **`vla_layer`**: deterministic planner seam that produces explicit proposals
  and fallback statuses for bounded alpha workflows.
- **`cli`**: `demo` and `platform-demo` entry points for risk-only and full
  alpha-platform runs.

### What is not true yet

- This is **not** public GA.
- The current VLA implementation is **skeletal/deterministic**, not a production
  autonomy stack.
- The safety layer is **simulation/replay-grade**, not an edge-integrated field runtime.
- The control plane is an **internal alpha API**, not a hardened multi-tenant service.

## Alpha Architecture

- **Edge seam**
  - `vla_layer` produces `ActionProposal`
  - `safety_layer` converts proposals into `SafetyDecision` and `SafetyEvent`
- **Backend seam**
  - `control_plane` submits batch risk jobs and persists artifacts
  - `risk_layer` turns telemetry and claims into risk events, feature bundles,
    premiums, and explainability outputs
- **Contracts**
  - `contracts` freezes the path toward `TelemetryFrame`, `ClaimRecord`,
    `RiskEvent`, `RiskFeatureBundle`, `ActionProposal`, `SafetyDecision`,
    and `SafetyEvent`

## Quick Start

Build the full alpha workspace:

```bash
cargo check --workspace
cargo test --workspace
```

Run the risk-only demo:

```bash
cargo run -p cli -- demo --json
```

Run the full alpha platform demo:

```bash
cargo run -p cli -- platform-demo --json
```

Run the Docker-based alpha demo shell:

```bash
docker compose up platform_alpha
```

## Workspace Map

- `contracts` - versioned cross-layer telemetry, claim, proposal, and safety contracts
- `actuarial` - normalization, risk detection, feature aggregation, GLM-style modeling, pricing, filing
- `risk_layer` - public risk facade used by the control plane
- `control_plane` - internal alpha job API with persisted artifacts and job status
- `safety_layer` - replayable guardrail runtime and controllers
- `vla_layer` - deterministic planner seam for bounded alpha flows
- `synthetic` - deterministic fleet, telemetry, and claims generation
- `cli` - alpha demo workflows
- `shared` - shared sim/runtime data structures
- `tests` - golden, stability, and end-to-end alpha platform tests

## Roadmap

The repository roadmap now follows:

1. **MVP / Design Partner Alpha** - current target in this repo
2. **Public Preview**
3. **Stable Public / GA**
4. **Enterprise Grade**

See [docs/roadmap.md](docs/roadmap.md) for the staged release plan.
