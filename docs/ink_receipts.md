# INK Receipts

This crate (`bil-ink`) owns receipt envelopes.

Receipt outputs:

```text
ink_receipt.v1.cbor   # canonical artifact
ink_receipt.v1.json   # developer/debug view
```

Important:

* JSON receipt must be generated from the canonical model.
* JSON must never be the source of the signed commitment.
* receipt verification must reconstruct canonical bytes before verifying the signature.
