# AiSSURANCE Release 2 (Public Preview) Technical Plan

## 1. API Service Architecture

**Crate Structure:**
- Create a new crate named `api_server` (or `service`) in the workspace.
- This crate will depend on `control_plane`, `risk_layer`, `contracts`, and `shared`.

**Framework:**
- Use **Axum** as the web framework. It is built on top of Tokio and Hyper, providing excellent performance, ergonomics, and ecosystem compatibility.
- Use `tokio` as the async runtime.
- Use `serde` and `serde_json` for request/response serialization.

**Authentication:**
- For the Public Preview, implement a simple **Bearer Token** authentication mechanism.
- Use `axum::middleware` or `tower-http::auth::RequireAuthorizationLayer` to protect the endpoints.
- The valid token(s) can be loaded from an environment variable (e.g., `AISSURANCE_API_TOKEN`) for simplicity in the preview release.

**Endpoints:**
- `POST /api/v1/jobs`: Submit a new batch job.
  - Request Body: JSON representation of `RiskLayerInput`.
  - Action: Calls `ControlPlane::submit_batch`.
  - Response: JSON representation of `BatchJobRecord` (includes `job_id`).
- `GET /api/v1/jobs/:job_id`: Get the status of a job.
  - Action: Calls `ControlPlane::job_status`.
  - Response: JSON representation of `BatchJobRecord`.
- `GET /api/v1/jobs/:job_id/report`: Retrieve the report for a completed job.
  - Action: Calls `ControlPlane::report`.
  - Response: JSON representation of `RiskLayerReport`.

## 2. Python SDK and Bindings

**Approach:**
- Use **PyO3** to create native Python bindings for the Rust engine.
- Use **Maturin** as the build backend to compile the Rust code into Python wheels.

**Implementation Steps:**
- Create a new crate `python_bindings` (or configure an existing one like `actuarial` with `crate-type = ["cdylib"]`).
- Implement PyO3 wrapper classes (`#[pyclass]`) that implement or wrap the traits defined in `actuarial/src/pyo3_interfaces.rs` (e.g., `FrequencyModel`, `SeverityModel`, `ExplainabilityEngine`).
- Expose the `ControlPlane` functionality to Python so users can submit jobs and retrieve reports directly from Python scripts or Jupyter notebooks.
- Update `pyproject.toml` to configure Maturin:
  ```toml
  [build-system]
  requires = ["maturin>=1.0,<2.0"]
  build-backend = "maturin"
  ```
- Create a Python package structure (e.g., `aissurance/`) that imports the compiled Rust extension and provides a more Pythonic API (type hints, docstrings).

## 3. Deployment Flow

**Containerization (Docker):**
- Create a `Dockerfile` for the `api_server`.
- Use a **multi-stage build** to keep the final image size small:
  - **Builder Stage:** Use an official Rust image (e.g., `rust:1.75-slim-bookworm`). Copy the source code, build the workspace in release mode (`cargo build --release -p api_server`).
  - **Runtime Stage:** Use a minimal base image like `debian:bookworm-slim` or `gcr.io/distroless/cc-debian12`. Copy the compiled binary from the builder stage.
- Expose the API port (e.g., 8080).
- Configure environment variables (e.g., `AISSURANCE_API_TOKEN`, `RUST_LOG`).

**Docker Compose:**
- Update the existing `docker-compose.yml` to include the `api_server` service.
- This allows users to easily spin up the entire platform locally for preview using `docker-compose up`.

**Release Automation (CI/CD):**
- Set up GitHub Actions workflows:
  - **Rust CI:** Run `cargo test`, `cargo clippy`, and `cargo fmt`.
  - **Docker Build & Push:** Build the Docker image and push it to a container registry (e.g., GHCR or Docker Hub) on tagged releases.
  - **Python Wheels:** Use `PyO3/maturin-action` to build Python wheels for multiple platforms (Linux, macOS, Windows) and publish them to PyPI on tagged releases.

## 4. Documentation and Examples

- **API Documentation:** Generate OpenAPI/Swagger documentation for the Axum service (e.g., using `utoipa`).
- **Python SDK Docs:** Use Sphinx or MkDocs to generate documentation from Python docstrings.
- **Examples:** Provide Jupyter notebooks in the `notebooks/` directory demonstrating how to use the Python SDK for actuarial modeling and job submission.
