# MechAssure Architecture Plan for the MechAssure SDK.

## Project Structure

The project will be a Rust workspace named `mechassure` with the following structure:

```text
mechassure/
├── Cargo.toml (Workspace root)
├── crates/
│   ├── mechassure-core/         # Core traits, artifact bundles, deterministic reducers
│   ├── mechassure-evidence/     # Evidence schemas, signing/hashing
│   ├── mechassure-rssl/         # Risk Sufficient Statistics Layer definitions
│   ├── mechassure-actuarial/    # Actuarial export, pricing models
│   ├── mechassure-underwriting/ # Universal underwriting file schema
│   ├── mechassure-verify/       # Validation, SDK doctor
│   ├── mechassure-report/       # Report generation
│   ├── mechassure-cli/          # CLI tool (demo, init, reduce, export, validate)
│   └── mechassure-py/           # Python bindings
└── domain-packs/
    ├── construction/            # Construction domain pack
    ├── banking/                 # Banking domain pack
    ├── healthcare/              # Healthcare domain pack
    ├── cyber/                   # Cyber domain pack
    └── customer-support/        # Customer support domain pack
```

## Core Components

### 1. Core Traits (`mechassure-core`)
Define the universal interface that all domain packs must implement:
- `DomainPack` trait
- `ExposureSchema`
- `FailureMode`
- `RiskReducer` trait
- `UnderwritingRule`
- `FeatureDictionary`

### 2. Universal Underwriting File (`mechassure-underwriting`)
Define the standard JSON schema for the AI Underwriting File:
- `artifact_type`
- `schema_version`
- `domain`
- `insured_system`
- `period`
- `exposure`
- `risk_statistics`
- `control_evidence`
- `evidence_quality`
- `underwriting_flags`
- `actuarial_export`
- `audit_manifest`

### 3. Construction Domain Pack (`domain-packs/construction`)
Implement the `DomainPack` trait for the construction domain, serving as the reference implementation:
- Exposure: autonomous machine hours, human-machine interaction hours
- Failure events: near misses, emergency stops, etc.
- Severity signals: kinetic energy, proximity, etc.
- Controls: geofencing, human spotter, etc.
- Evidence quality: sensor uptime, localization uncertainty, etc.

### 4. CLI (`mechassure-cli`)
Implement the command-line interface with the following commands:
- `mechassure demo <domain>`
- `mechassure init --domain <domain>`
- `mechassure reduce <logs_dir> --domain <domain> --out <out_dir>`
- `mechassure export <underwriting_file_dir> --format <format>`
- `mechassure validate <underwriting_file_dir>`

## Implementation Steps

1. **Initialize Workspace**: Create the root directory and workspace `Cargo.toml`.
2. **Create Crates**: Generate the boilerplate for all crates and domain packs using `cargo new --lib` (or `--bin` for CLI).
3. **Define Core Traits**: Implement the foundational traits and structs in `mechassure-core`.
4. **Define Underwriting Schema**: Implement the universal underwriting file structure in `mechassure-underwriting`.
5. **Implement Construction Pack**: Build the reference implementation in `domain-packs/construction`.
6. **Build CLI**: Implement the CLI commands in `mechassure-cli` using `clap`.
7. **Create Demos**: Implement the `mechassure demo construction` command to generate a sample underwriting file.