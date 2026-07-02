# Verification

The verifier (`bil-verify`) is the heart of the bank/auditor trust story.

Minimum checks:

* SchemaValid
* CanonicalEncodingValid
* CommitmentHashValid
* SignatureValid
* SignerKnown
* EvidenceRefsPresent
* MerkleRootValid
* MerkleProofsValid
* AuthorityRefsPresent
* AuthorityBindingValid
* PolicyRefsPresent
* ReplayDeterministic
* TimestampValid
* AssuranceLevelValid
* ProfileChecksPassed

Status rules:

```text
P0 finding → FAIL
P1 finding → WARN unless profile says fail
P2 finding → WARN or PASS_WITH_WARNINGS
```
