# Integrations

Integration crates map downstream domain artifacts into BIL without moving
domain-specific semantics into the public core crates.

The current supported bridge is `bil-aissurance-bridge`, which connects the
vendored AiSSURANCE downstream to feature-gated `bil-cli` flows while keeping
the default root workspace domain-neutral.

For repo-wide support tiers and LOC reality, see
[../docs/repo-assessment.md](../docs/repo-assessment.md).
