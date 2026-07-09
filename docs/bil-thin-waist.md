# BIL: The Open Thin Waist for Institutional Evidence

BIL (Base Institutional Language) is an open-source thin-waist evidence grammar for institutional workflows.

BIL does not execute workflows, make decisions, run compliance programs, or replace domain systems. It defines a minimal common evidence layer that allows heterogeneous systems to emit canonical proof objects and independent verifiers to test them.

In this repository, the thin waist is the root Cargo workspace boundary, not a
claim that every tracked path is part of the public core. Repo-level support
tiers and LOC totals are documented in [repo-assessment.md](repo-assessment.md).

The core architecture is:

1. Many systems execute.
2. BIL normalizes evidence.
3. Many parties verify.

## What BIL provides

- Canonical evidence event types
- Actor, subject, policy, and evidence references
- MIR graph representation
- Deterministic canonicalization
- Commitment hashing
- Signed receipt envelope (INK)
- Verification semantics
- Open conformance vectors
- Local CLI tooling

## What BIL does not provide

- Banking-specific policy profiles
- SBA or CFPB compliance mappings
- Vendor adapters
- Hosted registry services
- HSM-backed checkpointing
- Commercial assurance packets
- Customer dashboards

Commercial or domain-specific products can build on BIL without changing the thin waist.
