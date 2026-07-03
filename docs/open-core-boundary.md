# Open Core Boundary

This document defines the boundary between the open-source `bil` repository and private/commercial repositories (like `bankabil`).

## The Rule

**BIL is the open-source thin waist for institutional evidence. INK is the signed receipt artifact generated from BIL-compatible evidence. BAINK is Bankabil’s commercial banking registry that applies BIL and INK to AI-enabled small-business banking workflows.**

## Public Repo (`bil`)

The public repository contains only domain-neutral, structural primitives.

- **Core Grammar:** `bil-core` (events, refs, actors, subjects, policies)
- **Compiler/IR:** `bil-mir` (graph representation)
- **Canonicalization:** `bil-canonical` (deterministic encoding, hashing)
- **Artifacts:** `bil-ink` (receipt envelope, Merkle inclusion)
- **Signatures:** `bil-signers` (traits, reference implementations)
- **Verification:** `bil-verify` (structural checks only)
- **Tooling:** `bil-cli`, `bil-sdk` (generic developer tools)
- **Testing:** `bil-conformance` (standard vectors)

## Private Repositories (e.g., `bankabil`)

Private repositories contain domain-specific logic, commercial profiles, and proprietary integrations.

- **Banking Profiles:** SBA Express, CFPB Adverse Action, Sponsor Bank Oversight
- **Vendor Adapters:** Casca, Alloy, Apiture, Infinant, Orca
- **Commercial Services:** Registry service, HSM checkpoints, Replay bundles
- **Sales Collateral:** Pitch decks, outreach memos, pricing thesis
- **Demos:** Live Oak design partner demo, specific vendor traces

## Verification Seam

The `bil-verify` crate must only perform structural verification:
- Schema validity
- Canonical encoding validity
- Commitment hash matching
- Signature validity
- Merkle inclusion validity
- Required references present

It must **not** perform domain-specific checks (e.g., "SBA Eligibility Evidence Present"). Domain checks belong in the private repository's profile verifier.
