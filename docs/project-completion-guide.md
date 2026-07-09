# Project Completion Guide

This file is no longer the active repository-topology blueprint.

The current source of truth for repo shape, support tiers, and LOC-based
reality is [repo-assessment.md](repo-assessment.md). The older split blueprint
now lives as historical background in [history/repo-split.md](history/repo-split.md).

## Current State

- The root Cargo workspace is the supported public BIL core.
- `Insurance/AiSSURANCE` plus `integrations/bil-aissurance-bridge` form the
  supported local downstream lane.
- `experiments/axiom-tui` is a live standalone experiment outside root
  workspace guarantees.
- `archive/insurance-experiments` and `docs/history` are reference-only.

## Current Working Agreement

- Use the root README for public-core onboarding.
- Use [aissurance-local-integration.md](aissurance-local-integration.md) for
  the supported downstream path.
- Use [repo-assessment.md](repo-assessment.md) for repo-wide claims, support
  tiers, and dual-baseline LOC numbers.
- Treat historical split material as archival context, not as an active
  implementation plan.
- Public banned-term gate passes.
- Private demo and collateral use non-production language.
- Claim gates are reflected in README and docs.
