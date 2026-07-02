# BIL SDK Architecture

The right structure is:

> **Rust owns proof. Python owns velocity.**

Build the BIL trust core as a Rust workspace, expose a narrow Python surface over the verified Rust engine, and keep every bank-facing claim gated by implemented cryptographic capabilities.

## Workspace layout

* `crates/`: Rust workspace containing the trust core.
* `python/`: Python bindings over the Rust trust core.
* `conformance/`: Test vectors and compatibility suite.
* `examples/`: Example workflows and use cases.
* `docs/`: Architecture and design documentation.
