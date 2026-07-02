# Canonical Encoding

This is one of the most important crates (`bil-canonical`).

It should own:

* canonical BIL value model;
* deterministic CBOR encoding;
* duplicate-key rejection;
* stable key ordering;
* integer minor units;
* timestamp normalization;
* canonical hash commitments;
* conformance test vectors.

Core doctrine:

> **JSON is a view. CBOR is the artifact. The hash is over canonical bytes.**

Hard rule:

```rust
// Never let serde_json define the proof commitment.
```
