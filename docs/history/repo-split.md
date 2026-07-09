# Repository Split Blueprint: Public BIL, Private BAINK

Historical background only: this blueprint captures an earlier public/private
split design. It is not the current repo-topology source of truth. For the
current repository shape, support tiers, and LOC baselines, see
[`docs/repo-assessment.md`](../repo-assessment.md).

This document is the authoritative blueprint for splitting the current
monolithic repository into a public open-source repository named `bil` and a
private commercial repository named `bankabil`.

The split proves the core architecture claim:

> BIL is the open thin waist. BAINK is the commercial banking product built on
> top.

`docs/bil-thin-waist.md` and `docs/open-core-boundary.md` are draft inputs.
They should be finalized after this split blueprint exists and the move map is
settled.

## 1. Public `bil/` Workspace

The public repository contains domain-neutral evidence primitives, canonical
commitment machinery, receipt artifacts, structural verification semantics,
conformance vectors, and generic developer tooling.

```text
bil/
├─ Cargo.toml
├─ README.md
├─ LICENSE-APACHE
├─ LICENSE-MIT
├─ rust-toolchain.toml
├─ .gitignore
│
├─ crates/
│  ├─ bil-core/         # Generic ids, refs, event vocabulary, status types
│  ├─ bil-mir/          # Domain-neutral graph/IR and replay state
│  ├─ bil-canonical/    # Canonical value model, deterministic encoding, hashes
│  ├─ bil-ink/          # INK receipt envelope, commitment, Merkle primitives
│  ├─ bil-signers/      # Signer/verifier traits and L0 software reference signer
│  ├─ bil-verify/       # Structural verifier and report semantics
│  ├─ bil-sdk/          # Generic issue/verify/explain convenience layer
│  ├─ bil-cli/          # Generic mock/issue/verify/explain CLI
│  └─ bil-conformance/  # Public conformance vectors and checks
│
├─ examples/
│  └─ generic/
│     └─ human_override/
│        ├─ trace.json
│        ├─ issued_receipt.json
│        ├─ verification_report.json
│        └─ explainer_memo.md
│
├─ docs/
│  ├─ repo-split.md
│  ├─ bil-thin-waist.md
│  ├─ verification-semantics.md
│  ├─ mir.md
│  ├─ receipt-envelope.md
│  ├─ canonicalization.md
│  └─ conformance.md
│
└─ tests/
   ├─ issue_verify_roundtrip.rs
   ├─ canonicalization_vectors.rs
   ├─ merkle_inclusion.rs
   └─ verification_semantics.rs
```

Public workspace members:

- `crates/bil-core`
- `crates/bil-mir`
- `crates/bil-canonical`
- `crates/bil-ink`
- `crates/bil-signers`
- `crates/bil-verify`
- `crates/bil-sdk`
- `crates/bil-cli`
- `crates/bil-conformance`

Removed or folded from public workspace:

- `crates/bil-mock`: replaced by generic examples and any future generic
  synthetic profile trait.
- `crates/bil-replay`: fold `ReplayState` and replay status primitives into
  `bil-mir`; structural replay checks live in `bil-verify`.
- `crates/bil-explain`: fold generic explanation into `bil-sdk::explain`.
- `python`: move the current `bankabil` Python package to the private repo.

Public metadata changes:

- Rename meaning from "Bank Institutional Liability" to "Base Institutional
  Language".
- Change repository/homepage metadata to the public `bil` repository.
- Keep license as `MIT OR Apache-2.0`.

## 2. Private `bankabil/` Workspace

The private repository contains banking profiles, vendor adapters, commercial
services, hosted registry code, HSM/KMS checkpointing, BAINK verification
profiles, Python bindings, demos, and business collateral.

```text
bankabil/
├─ Cargo.toml
├─ README.md
├─ .gitignore
│
├─ baink/
│  ├─ banking-profiles/
│  │  ├─ sba-express/
│  │  │  ├─ schema/
│  │  │  ├─ synthetic/
│  │  │  ├─ policy-map/
│  │  │  ├─ fixtures/
│  │  │  └─ README.md
│  │  ├─ cfpb-adverse-action/
│  │  ├─ sponsor-bank-oversight/
│  │  └─ ai-assurance/
│  │
│  ├─ vendor-adapters/
│  │  ├─ casca/
│  │  ├─ alloy/
│  │  ├─ apiture/
│  │  ├─ infinant/
│  │  ├─ orca/
│  │  └─ README.md
│  │
│  ├─ registry-service/
│  ├─ hsm-checkpoints/
│  ├─ replay-bundles/
│  └─ compliance-memos/
│
├─ ink-commercial/
│  ├─ receipt-service/
│  ├─ assurance-levels/
│  ├─ artifact-pricing/
│  └─ verification-api/
│
├─ python/
│  └─ bankabil/
│
├─ demos/
│  └─ live-oak/
│     ├─ fixtures/
│     ├─ expected/
│     ├─ memos/
│     ├─ pitch/
│     └─ README.md
│
├─ collateral/
│  ├─ OUTREACH_MEMO.md
│  ├─ RECEIPT_TAXONOMY.md
│  ├─ LIVE_OAK_DESIGN_PARTNER.md
│  ├─ PRICING_THESIS.md
│  └─ INVESTOR_PACKET.md
│
└─ docs/
   ├─ baink-registry-architecture.md
   ├─ live-oak-design-partner-sprint.md
   ├─ sba-express-profile.md
   ├─ cfpb-adverse-action-profile.md
   └─ vendor-adapter-boundaries.md
```

Decision: the SBA Express profile logic moves to
`bankabil/baink/banking-profiles/sba-express/`; the Live Oak design-partner
demo package moves to `bankabil/demos/live-oak/`.

## 3. Cargo Dependency Model

### Public `bil/Cargo.toml`

```toml
[workspace]
resolver = "2"

members = [
    "crates/bil-core",
    "crates/bil-mir",
    "crates/bil-canonical",
    "crates/bil-ink",
    "crates/bil-signers",
    "crates/bil-verify",
    "crates/bil-sdk",
    "crates/bil-cli",
    "crates/bil-conformance",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["BIL Contributors"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/<org>/bil"
homepage = "https://github.com/<org>/bil"
readme = "README.md"
rust-version = "1.80"
description = "Base Institutional Language: an open evidence grammar and receipt layer"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"
sha2 = "0.10"
ed25519-dalek = "2"
rand = "0.8"
uuid = { version = "1", features = ["v4"] }
hex = "0.4"
base64 = "0.22"
ciborium = "0.2"
clap = { version = "4", features = ["derive"] }

bil-core = { path = "crates/bil-core" }
bil-mir = { path = "crates/bil-mir" }
bil-canonical = { path = "crates/bil-canonical" }
bil-ink = { path = "crates/bil-ink" }
bil-signers = { path = "crates/bil-signers" }
bil-verify = { path = "crates/bil-verify" }
bil-sdk = { path = "crates/bil-sdk" }
```

### Private `bankabil/Cargo.toml`

```toml
[workspace]
resolver = "2"

members = [
    "baink/registry-service",
    "baink/hsm-checkpoints",
    "baink/replay-bundles",
    "ink-commercial/receipt-service",
    "ink-commercial/verification-api",
    "python/bankabil",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "UNLICENSED"
repository = "https://github.com/<org>/bankabil"
rust-version = "1.80"

[workspace.dependencies]
bil-core = { git = "https://github.com/<org>/bil", package = "bil-core" }
bil-mir = { git = "https://github.com/<org>/bil", package = "bil-mir" }
bil-canonical = { git = "https://github.com/<org>/bil", package = "bil-canonical" }
bil-ink = { git = "https://github.com/<org>/bil", package = "bil-ink" }
bil-signers = { git = "https://github.com/<org>/bil", package = "bil-signers" }
bil-verify = { git = "https://github.com/<org>/bil", package = "bil-verify" }
bil-sdk = { git = "https://github.com/<org>/bil", package = "bil-sdk" }

serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"
sha2 = "0.10"
```

For local development before publishing the public repo:

```toml
[patch."https://github.com/<org>/bil"]
bil-core = { path = "../bil/crates/bil-core" }
bil-mir = { path = "../bil/crates/bil-mir" }
bil-canonical = { path = "../bil/crates/bil-canonical" }
bil-ink = { path = "../bil/crates/bil-ink" }
bil-signers = { path = "../bil/crates/bil-signers" }
bil-verify = { path = "../bil/crates/bil-verify" }
bil-sdk = { path = "../bil/crates/bil-sdk" }
```

## 4. Current Surface Move Map

### Move to private `bankabil`

| Current surface | Destination | Reason |
| --- | --- | --- |
| `demos/sba-express-evidence/` | `bankabil/demos/live-oak/` | Live Oak/SBA design-partner material |
| `demos/sba-express-evidence/fixtures/live_oak_sba_express_trace.json` | `bankabil/demos/live-oak/fixtures/` | Live Oak-style trace |
| SBA Express fixture/policy content | `bankabil/baink/banking-profiles/sba-express/fixtures/` | Banking profile data |
| `OUTREACH_MEMO.md`, `RECEIPT_TAXONOMY.md`, Live Oak notes | `bankabil/collateral/` | Commercial/product material |
| `crates/bil-cli` SBA trace schema and normalizer | private `baink-cli` or `baink/banking-profiles/sba-express/synthetic/` | Banking/vendor-specific ingestion |
| `SbaExpressEvidenceTrace`, `default_sba_express_trace`, `trace_to_mir` | `bankabil/baink/banking-profiles/sba-express/synthetic/` | SBA profile generator |
| Casca, Alloy, Apiture, Infinant, Orca mappings | `bankabil/baink/vendor-adapters/` | Vendor-specific adapters |
| `crates/bil-mock` bank profile generator | `bankabil/baink/banking-profiles/*/synthetic/` | Bank-shaped mock data |
| `BankBranchSyntheticConfig` | private banking synthetic profile config | Banking-specific generator |
| `generate_bank_branch_mock` | private banking synthetic profile generator | Banking-specific generator |
| `SyntheticProfile::{BankBranch, LoanDecision, AdverseAction, AiAssurance, ThirdPartyVendor}` | private BAINK profile enum | Banking/product profile names |
| `DemoProfile::{BankBranch, LoanDecision, AiAssurance}` | private demo/profile layer | Banking demo surface |
| `examples/bank_branch/` | `bankabil/baink/banking-profiles/bank-branch/fixtures/` | Bank fixture |
| `examples/adverse_action/` | `bankabil/baink/banking-profiles/cfpb-adverse-action/fixtures/` | CFPB-oriented fixture |
| `examples/loan_decision/` | `bankabil/baink/banking-profiles/sba-express/fixtures/` | Banking fixture |
| `examples/ai_assurance/` | `bankabil/baink/banking-profiles/ai-assurance/fixtures/` | Banking/product profile fixture |
| `examples/third_party_vendor/` | `bankabil/baink/vendor-adapters/fixtures/` | Vendor-facing fixture |
| `python/bankabil/`, `python/Cargo.toml`, `python/pyproject.toml` | `bankabil/python/bankabil/` | Private/commercial Python package |
| `docs/verification.md` bank/auditor wording | private docs or public-neutral rewrite | Public docs must be domain-neutral |
| `docs/architecture.md` bank-facing claim language | private docs or public-neutral rewrite | Product positioning |
| `docs/conformance.md` profile-bank-* entries | private profile conformance docs | Banking profile checks |

### Keep or genericize in public `bil`

| Current surface | Public action | Reason |
| --- | --- | --- |
| `bil-core` ids and refs | keep | Thin-waist primitives |
| `AssuranceLevel` | keep generic levels only | Receipt trust primitive |
| `BilStatus` | keep | Verification report primitive |
| `BilMirGraph` | keep | Generic IR |
| `EvidenceRefNode` | keep | Generic evidence reference |
| `ReplayState` | fold into `bil-mir` | Generic replay metadata |
| `bil-canonical` | keep | Domain-neutral commitment path |
| `bil-ink` receipt and Merkle primitives | keep | Generic signed artifact |
| `bil-signers` traits and L0 software signer | keep | Open signer interface and reference signer |
| `bil-verify` report and structural engine | keep | Open verification semantics |
| `bil-conformance` | keep and genericize vectors | Standard credibility |
| `bil-explain` | fold into `bil-sdk::explain` | Generic explanation helper |
| `bil-cli issue/verify/explain` | keep generic | Developer adoption |
| `bil-cli mock bank-branch` | replace with `mock generic --profile human-override` | Domain-neutral public demo |
| `BankBranchIntakeStarted` | rename to `WorkflowStarted` | Remove bank-specific event vocabulary |
| `bank_branch`, `loan_decision`, `ai_assurance` CLI/profile names | replace with generic profile names | Public neutrality |

## 5. Verification Seam

Public `bil-verify` checks structural validity only.

Target public check kind vocabulary:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationCheckKind {
    SchemaValid,
    CanonicalEncodingValid,
    CommitmentHashMatches,
    SignatureValid,
    ChainLinkValid,
    MerkleInclusionValid,
    ReceiptEnvelopeValid,
    RequiredReferencePresent,
    ProfileDeclared,
    ExtensionRecognized,
}
```

Move out of public `bil-verify` or keep out permanently:

- `ProfileBankBranchValid`
- `ProfileLoanDecisionValid`
- `ProfileAiAssuranceValid`
- `SbaEligibilityEvidencePresent`
- `CfpbAdverseActionReasonSpecific`
- `LiveOakSbaExpressTraceValid`
- `CascaDocumentConfidencePresent`
- `AlloyKybCaseStatusPresent`
- `ApitureConsentEventPresent`
- `InfinantSponsorOversightPresent`

Private BAINK profile checks:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BainkBankingCheckKind {
    SbaExpressRequiredFieldsPresent,
    SbaEligibilityPolicyRefsPresent,
    FinancialSpreadingEvidencePresent,
    ManualOverrideReasonPresent,
    CfpbAdverseActionFactorsPresent,
    CfpbSpecificReasonSupported,
    SponsorBankOversightTrailPresent,
    VendorSourceSystemMapped,
    PrivacyRefsOnlyNoRawPii,
}
```

The public verifier answers: "Is this receipt structurally valid?"

The private BAINK verifier answers: "Does this structurally valid receipt
satisfy a banking profile?"

## 6. CLI Boundary

Public `bil-cli` target commands:

```bash
cargo run -p bil-cli -- mock generic \
  --profile human-override \
  --out examples/generic/human_override/trace.json

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

Private `baink-cli` target commands:

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
```

## 7. Migration Sequence

1. Draft repo split: create this `docs/repo-split.md`. No code behavior changes.
2. Update workspace/Cargo structure for the public `bil` target.
3. Split verification check vocabulary into structural public checks and private
   BAINK banking checks.
4. Move bank mocks and banking profile enums into private `bankabil`.
5. Move SBA/Live Oak demo assets and collateral into private `bankabil`.
6. Replace public bank demos with `examples/generic/human_override/`.
7. Replace bank-specific MIR event names with generic event names.
8. Move Python package to private `bankabil/python/bankabil/`.
9. Finalize `docs/open-core-boundary.md`.
10. Finalize `docs/bil-thin-waist.md`.

## 8. Acceptance Checks

Draft-only checks:

```bash
rg -n "BankBranch|bank_branch|LoanDecision|AiAssurance|SBA|sba|CFPB|cfpb|Live Oak|live-oak|live_oak|Casca|Alloy|Apiture|Infinant|Orca|Bankabil|bankabil|BAINK|vendor|adverse|banking" crates docs examples demos python Cargo.toml -g '!target'
```

Every current hit must be mapped in this document before code moves begin.

Later implementation checks:

- Public `bil`: `cargo test`
- Private `bankabil`: `cargo test`
- Public smoke: `mock generic -> issue -> verify -> explain`
- Private smoke: `mock sba-express -> issue -> verify -> profile-verify`
- Public banned-term gate for `Live Oak`, `SBA`, `CFPB`, `Casca`, `Alloy`,
  `Apiture`, `Infinant`, `BankBranch`, `bank_branch`, `loan_decision`, and
  `ai_assurance`, except in explicit boundary docs.

## 9. Final Architecture Sentence

Use this wording consistently:

> BIL is the open-source thin waist for institutional evidence. INK is the
> signed receipt artifact generated from BIL-compatible evidence. BAINK is
> Bankabil's commercial banking registry that applies BIL and INK to
> AI-enabled small-business banking workflows.
