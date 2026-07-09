# AiSSURANCE Roadmap

This repository is implementing the staged path below.

## Release 1 — MVP / Design Partner Alpha

Current focus:

- buildable `risk_layer`, `safety_layer`, `vla_layer`, and `control_plane` seams
- replayable safety runtime with measured latency budget
- deterministic VLA proposal boundary
- file-backed alpha job persistence and artifact export
- end-to-end sim path across planner, safety, control plane, and risk

## Release 2 — Public Preview

Planned next:

- preview installation and deployment flow
- preview service/API surface with auth and job status
- Rust SDK polish and Python SDK/bindings path
- rewritten docs/examples and release automation

## Release 3 — Stable Public / GA

Required before GA:

- risk, safety, and VLA all meet production-grade bars
- stable contracts/API `v1`
- operational control plane, upgrade path, backup/restore, observability
- bounded autonomy with explicit safety-enforced operational envelopes

## Release 4 — Enterprise Grade

Planned after GA:

- SSO / SCIM / RBAC
- immutable audit trails and governance workflows
- HA / DR / scale operations
- private deployment and enterprise support posture
