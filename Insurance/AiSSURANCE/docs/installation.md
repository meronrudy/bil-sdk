# Installation and Alpha Setup

This guide covers the current **design-partner alpha** environment for the
AiSSURANCE platform skeleton.

## Requirements

- Rust 1.70+
- Cargo
- Git
- Optional: Docker for the alpha demo container workflow

## Build and verify

```bash
cargo check --workspace
cargo test --workspace
```

The workspace should compile the following crates:

- `contracts`
- `actuarial`
- `risk_layer`
- `control_plane`
- `safety_layer`
- `vla_layer`
- `synthetic`
- `cli`
- `shared`
- `tests`

## Run the demos

### Risk-only demo

```bash
cargo run -p cli -- demo --json
```

### Full alpha platform demo

```bash
cargo run -p cli -- platform-demo --json --data-dir .aissurance-alpha
```

This demo exercises:

1. deterministic VLA proposal generation
2. safety replay evaluation
3. control-plane job submission
4. persisted risk report and filing artifacts

## Docker alpha shell

The repository includes a `docker-compose.yml` aligned to the current alpha
surface. It runs the platform demo from a Rust container rather than promising
nonexistent production services.

```bash
docker compose up platform_alpha
```

## Output artifacts

The control plane persists alpha artifacts under the configured data directory:

- `jobs/<job-id>/input.json`
- `jobs/<job-id>/report.json`
- `jobs/<job-id>/filing.json`
- `jobs/<job-id>/job.json`
