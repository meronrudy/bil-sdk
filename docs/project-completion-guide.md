# Project Completion Guide

This guide translates `docs/repo-split.md` into an operational completion
path for the full public/private split:

- public `bil`: the open thin waist for institutional evidence
- private `bankabil` / BAINK: the commercial banking evidence registry built on
  top of public BIL and INK receipts

`docs/repo-split.md` remains the boundary source of truth. This file is the
milestone and acceptance source of truth.

## Phase 0: Current State And Immediate Blockers

Current public repo state:

- The root workspace already targets the public `bil` crate set.
- `crates/bil-mock`, `crates/bil-replay`, `crates/bil-explain`, and `python`
  have been removed from workspace membership.
- Some remaining public crates still reference removed path dependencies.
- `artifacts/private_staging/bankabil/` is the current staging source for
  private BAINK materials.

Immediate blocker:

```text
cargo test
```

currently fails because `crates/bil-mir` still depends on the removed
`crates/bil-replay` path. Related cleanup is also needed for remaining
`bil-mock` and `bil-explain` references in `bil-cli` and `bil-sdk`.

Exit gate:

- Every missing path dependency is either restored intentionally or removed by
  folding its public-safe functionality into an active public crate.

## Phase 1: Restore Public BIL Build Health

Goal: make the public `bil` workspace compile and test before doing broader
genericization.

Required work:

- Fold `ReplayState`, replay status primitives, and any structural replay
  metadata into `bil-mir` or `bil-verify`; remove the `bil-replay` dependency.
- Fold generic explanation output into `bil-sdk::explain`; remove the
  `bil-explain` dependency.
- Remove public `bil-mock` dependencies from `bil-sdk` and `bil-cli`.
- Temporarily replace bank-shaped demo/mock entrypoints with a minimal generic
  trace generator if needed to keep CLI tests passing.
- Confirm `Cargo.toml` only lists public workspace members from
  `docs/repo-split.md`.

Exit gates:

```bash
cargo test
cargo fmt --check
```

Both must pass in the public `bil` workspace.

## Phase 2: Genericize Public BIL

Goal: remove banking/vendor-specific semantics from public code and examples
while preserving the developer proof path.

Required work:

- Replace `BankBranchIntakeStarted` with a generic MIR event kind such as
  `WorkflowStarted`.
- Replace bank profile names such as `bank_branch`, `loan_decision`, and
  `ai_assurance` with generic example profile names.
- Replace public examples with `examples/generic/human_override/`.
- Replace `bil-cli mock bank-branch` and `bil-cli mock sba-express-evidence`
  with:

```bash
cargo run -p bil-cli -- mock generic \
  --profile human-override \
  --out examples/generic/human_override/trace.json
```

- Keep public `issue`, `verify`, and `explain` commands generic:

```bash
cargo run -p bil-cli -- issue \
  --input examples/generic/human_override/trace.json \
  --out examples/generic/human_override/issued_receipt.json

cargo run -p bil-cli -- verify \
  --receipt examples/generic/human_override/issued_receipt.json \
  --out examples/generic/human_override/verification_report.json

cargo run -p bil-cli -- explain \
  --receipt examples/generic/human_override/issued_receipt.json \
  --report examples/generic/human_override/verification_report.json \
  --out examples/generic/human_override/explainer_memo.md
```

Exit gates:

- Public CLI smoke passes: `mock generic -> issue -> verify -> explain`.
- Public examples contain no banking/vendor scenario material.
- The public banned-term gate passes, except in explicit boundary docs:

```bash
rg -n "Live Oak|SBA|CFPB|Casca|Alloy|Apiture|Infinant|BankBranch|bank_branch|loan_decision|ai_assurance" crates examples demos Cargo.toml
```

## Phase 3: Finish Structural Receipt, Verification, And Conformance

Goal: make public BIL credible as a structural evidence and receipt layer.

Required work:

- Keep receipt issuance/signature verification over explicit preimage bytes.
- Ensure `bil-verify` checks structural validity only:
  - canonical encoding
  - commitment hash
  - signature
  - receipt envelope
  - required references
  - Merkle root/proof structure
  - generic profile declaration
- Rename or consolidate public check kinds around structural semantics.
- Add or update conformance vectors for:
  - canonicalization
  - receipt roundtrip
  - signature verification
  - Merkle roots and inclusion proofs
  - verification semantics
- Keep profile-specific checks out of public `bil-verify`.

Exit gates:

- `cargo test` passes.
- Conformance groups pass for canonicalization, receipts, Merkle, signatures,
  and verification semantics.
- Public docs do not claim full replay determinism, production HSM/KMS, or
  bank-grade compliance.

## Phase 4: Public BIL Release Readiness

Goal: make public `bil` ready to publish or open as the generic thin waist.

Required work:

- Add top-level public `README.md`.
- Update workspace metadata from "Bank Institutional Liability" to "Base
  Institutional Language".
- Finalize, in order:
  - `docs/open-core-boundary.md`
  - `docs/bil-thin-waist.md`
  - `docs/verification-semantics.md`
  - `docs/conformance.md`
- Add license files if absent:
  - `LICENSE-APACHE`
  - `LICENSE-MIT`
- Add public release checklist covering tests, conformance, examples, and claim
  gates.

Exit gates:

- Public repo can be cloned, tested, and run through the generic CLI flow
  without private assets.
- Public docs clearly state what BIL provides and does not provide.
- BIL may claim: open thin-waist evidence grammar, canonical commitments,
  software-signed INK receipts, and structural verification.

## Phase 5: Private BAINK Repo Completion

Goal: make private `bankabil` the commercial banking layer built on public BIL.

Bootstrap source:

```text
artifacts/private_staging/bankabil/
```

Required work:

- Create or update the private `bankabil` workspace using the layout in
  `docs/repo-split.md`.
- Depend on public BIL crates by Git, with local `[patch]` entries for
  development.
- Move the Live Oak/SBA demo to private `demos/live-oak/`.
- Move SBA/Live Oak/vendor trace normalizer out of public `bil-cli` and into
  private `baink-cli` or `baink/banking-profiles/sba-express/synthetic/`.
- Add BAINK profile verification layered on top of public `bil-verify`.
- Restore the private demo flow:

```bash
cargo run -p baink-cli -- mock sba-express \
  --scenario live-oak \
  --out demos/live-oak/fixtures/live_oak_sba_express_trace.json

cargo run -p baink-cli -- issue \
  --profile sba-express \
  --input demos/live-oak/fixtures/live_oak_sba_express_trace.json \
  --out demos/live-oak/expected/issued_receipt.json

cargo run -p baink-cli -- verify \
  --profile sba-express \
  --receipt demos/live-oak/expected/issued_receipt.json \
  --out demos/live-oak/expected/verification_report.json

cargo run -p baink-cli -- profile-verify \
  --profile sba-express \
  --receipt demos/live-oak/expected/issued_receipt.json \
  --out demos/live-oak/expected/profile_report.json
```

Exit gates:

- Private `cargo test` passes.
- Live Oak-style demo runs end to end from private assets.
- BAINK profile verifier can report banking checks separately from public
  structural checks.
- Private docs and collateral do not imply endorsement, certification, or
  production compliance unless explicitly implemented and approved.

## Phase 6: Final Claim Gates And Launch Checklist

Public BIL may claim:

- Base Institutional Language is an open thin waist for institutional evidence.
- BIL provides a domain-neutral MIR graph, canonicalization, receipt envelope,
  signer traits, structural verifier, conformance vectors, and generic CLI.
- INK receipts can be software-signed and structurally verified.

Public BIL may not claim:

- SBA or CFPB compliance mapping
- Live Oak or other bank-specific readiness
- vendor adapters
- hosted registry service
- production HSM/KMS
- full deterministic replay
- bank-grade compliance verification

Private BAINK may claim banking registry capability only after:

- private profile verification passes
- Live Oak-style demo passes
- public BIL structural report and private BAINK profile report are both
  generated
- banking/profile claims are backed by implemented checks

Do not claim production HSM/KMS, full replay determinism, or bank-grade
compliance until implemented, tested, and documented.

Final launch checklist:

- Public `bil` build passes.
- Private `bankabil` build passes.
- Public generic CLI smoke passes.
- Private BAINK CLI smoke passes.
- Public banned-term gate passes.
- Private demo and collateral use non-production language.
- Claim gates are reflected in README and docs.
