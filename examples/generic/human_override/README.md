# Generic Human Override Example

This example is the public BIL smoke path for a domain-neutral workflow where a
document is received, policy is evaluated, a reviewer records a human override,
and a decision is issued.

Generate the MIR fixture locally:

```bash
cargo run -p bil-cli -- mock generic --profile human-override \
  --out examples/generic/human_override/human_override_trace.json
```

Issue and verify a receipt:

```bash
cargo run -p bil-cli -- issue \
  --input examples/generic/human_override/human_override_trace.json \
  --out examples/generic/human_override/issued_receipt.json

cargo run -p bil-cli -- verify \
  --receipt examples/generic/human_override/issued_receipt.json \
  --pretty
```
