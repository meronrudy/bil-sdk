# Python Bindings

Use PyO3 and maturin.

## Python rule

Python never owns:

* canonical encoding;
* hash commitments;
* signature creation;
* signature verification;
* Merkle proof verification;
* replay semantics;
* authority binding checks.

Python only owns:

* orchestration;
* notebook display;
* fixture loading;
* Markdown/HTML rendering;
* developer ergonomics.
