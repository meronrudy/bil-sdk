# Plan: Python SDK Layer — `python/cios`

## Overview

A pure-Python package that wraps the native `_cios_core` extension module
(from `cios-pyo3`) with a Pythonic, typed, ergonomic API. Provides dataclass
models, pandas/polars interop, async helpers, and a high-level `Pipeline`
class. Published to PyPI as `cios`.

> **Key concept:** The SDK's primary developer on-ramp is the **canonical flat
> file** — a single self-contained Python file covering an entire use case from
> config to filing. See [canonical-flat-files.md](canonical-flat-files.md) for
> the full specification of this pattern.

## Design Principles

1. **Type-annotated** — Full `typing` annotations, compatible with mypy strict mode
2. **Dataclass models** — Python-side dataclasses mirror Rust structs for IDE autocomplete
3. **DataFrame interop** — Accept/return `pandas.DataFrame` and `polars.DataFrame`
4. **Lazy native calls** — Import the native module lazily so the SDK is importable
   even when the native extension is not installed (for type-checking, docs, stubs)
5. **Exception hierarchy** — Pythonic exceptions wrapping Rust errors

## Package Layout

```
python/
├── pyproject.toml              # Package metadata, build config
├── cios/
│   ├── __init__.py             # Top-level exports
│   ├── _native.py              # Lazy loader for _cios_core
│   ├── exceptions.py           # CiosError, ModelError, PricingError, etc.
│   ├── types/
│   │   ├── __init__.py
│   │   ├── ids.py              # MachineId, SiteId, PolicyId, etc.
│   │   ├── policy.py           # Policy, PolicyStatus dataclasses
│   │   ├── claim.py            # Claim, ClaimStatus dataclasses
│   │   ├── exposure.py         # ExposureBundle dataclass
│   │   ├── risk.py             # RiskEvent, RiskEventType
│   │   └── results.py         # PremiumResult, ChainLadderResult, etc.
│   ├── model/
│   │   ├── __init__.py
│   │   ├── glm.py              # FittedGlm wrapper, fit_poisson, fit_gamma
│   │   └── diagnostics.py      # Model diagnostic helpers
│   ├── pricing/
│   │   ├── __init__.py
│   │   ├── expense.py          # ExpenseAssumptions, apply_expense_loading
│   │   ├── credibility.py      # CredibilityParams, compute_experience_modifier
│   │   └── premium.py          # calculate_final_premium
│   ├── reserving/
│   │   ├── __init__.py
│   │   ├── triangle.py         # LossTriangle with DataFrame interop
│   │   └── chain_ladder.py     # chain_ladder function
│   ├── explain/
│   │   ├── __init__.py
│   │   └── attribution.py      # explain_prediction, FeatureContribution
│   ├── governance/
│   │   ├── __init__.py
│   │   └── filing.py           # generate_filing, RateFilingArtifact
│   ├── pipeline/
│   │   ├── __init__.py
│   │   └── runner.py           # Pipeline class — high-level orchestration
│   ├── io/
│   │   ├── __init__.py
│   │   ├── csv.py              # CSV ↔ domain type converters
│   │   └── json.py             # JSON ↔ domain type converters
│   └── sim/
│       ├── __init__.py
│       └── generators.py       # Python wrappers for cios-sim generators
├── tests/
│   ├── conftest.py             # Fixtures
│   ├── test_model.py
│   ├── test_pricing.py
│   ├── test_reserving.py
│   ├── test_explain.py
│   ├── test_governance.py
│   ├── test_pipeline.py
│   ├── test_io.py
│   └── test_types.py
├── examples/
│   ├── cios_fleet_90d.py           # Primary canonical flat file (v0.3.0)
│   ├── cios_single_crane.py        # Crane-specific use case
│   ├── cios_highway_civil.py       # Highway construction
│   ├── cios_urban_excavation.py    # Dense urban site
│   ├── cios_autonomous_fleet.py    # Autonomous fleet comparison
│   ├── cios_renewal_pricing.py     # Renewal with experience data
│   └── cios_catastrophe_scenario.py # Catastrophe year simulation
├── stubs/
│   └── cios/                   # .pyi stub files for _cios_core
│       └── _native.pyi
└── docs/
    ├── quickstart.md
    ├── api_reference.md
    └── flat-file-workflow.md
```

## Python API Design

### Types — Dataclasses

```python
@dataclass(frozen=True)
class Policy:
    id: str
    effective_date: date
    expiration_date: date
    coverage: str
    written_premium: float
    written_exposure: float
    status: PolicyStatus

class PolicyStatus(str, Enum):
    QUOTED = "Quoted"
    BOUND = "Bound"
    IN_FORCE = "InForce"
    EXPIRED = "Expired"
    CANCELLED = "Cancelled"
```

### Model Fitting

```python
from cios.model import fit_poisson, fit_gamma, FittedGlm

# From numpy arrays
model: FittedGlm = fit_poisson(
    X=feature_matrix,      # np.ndarray (n, p)
    y=claim_counts,        # np.ndarray (n,)
    exposure=exposure,     # np.ndarray (n,)  — offset
    feature_names=["harsh_decel_rate", "proximity_rate", ...],
)

# Predictions
mu = model.predict(new_features)              # single observation
mus = model.predict_batch(feature_df)         # DataFrame → Series

# Diagnostics
model.summary()          # statsmodels-style text summary
model.significant(0.05)  # list of significant features
model.aic                # AIC value
```

### Pricing

```python
from cios.pricing import (
    ExpenseAssumptions,
    apply_expense_loading,
    compute_experience_modifier,
    calculate_final_premium,
)

assumptions = ExpenseAssumptions()  # sensible defaults
loaded = apply_expense_loading(pure_premium=4200.0, assumptions=assumptions)
print(loaded.gross_premium)       # 6547.62
print(loaded.implied_loss_ratio)  # 0.6416

modifier = compute_experience_modifier(
    exposure_hours=25_000,
    actual_losses=80_000,
    expected_loss_per_1000h=3.5,
    behavioral_score=0.85,
)

result = calculate_final_premium(
    gross_premium=loaded.gross_premium,
    experience_modifier=modifier.final_modifier,
    exposure_hours=25_000,
    minimum_premium=2500.0,
)
print(result.final_premium)
```

### Reserving — DataFrame Interop

```python
from cios.reserving import LossTriangle, chain_ladder
import pandas as pd

# From a pandas DataFrame
df = pd.read_csv("triangle.csv", index_col=0)
triangle = LossTriangle.from_dataframe(df)

result = chain_ladder(triangle)
print(result.ibnr)
print(result.loss_development_factors)

# Back to DataFrame
result.to_dataframe()
```

### Explainability

```python
from cios.explain import explain_prediction

contributions = explain_prediction(model, features=[1.0, 0.5, 0.2, ...])
for c in contributions:
    print(f"{c.feature}: {c.multiplicative_effect:.2f}x ({c.direction})")
```

### Pipeline — High-level Orchestration

```python
from cios.pipeline import Pipeline

pipe = Pipeline(config="pipeline.toml")

# Or programmatic configuration
pipe = Pipeline()
pipe.load_telemetry("fleet_data.csv")
pipe.detect_risk_events()
pipe.accumulate_exposure()
pipe.fit_model(family="poisson", features=[...])
pipe.price(assumptions=ExpenseAssumptions())
pipe.generate_filing(effective_date="2026-07-01", territory="CA")

# Get results at any stage
print(pipe.exposure_bundle)
print(pipe.model.summary())
print(pipe.premium_result)
print(pipe.filing)

# Export everything
pipe.export("output/")  # writes JSON artifacts
```

### Simulation Wrappers

```python
from cios.sim import generators

fleet = generators.fleet(n_machines=50, seed=42)
claims = generators.claims(n_policies=200, frequency=0.05, seed=42)
triangle = generators.triangle(n_origins=10, seed=42)
scenario = generators.scenario("well_managed_fleet", seed=42)
```

## Exception Hierarchy

```python
class CiosError(Exception): ...
class ModelError(CiosError): ...
class ConvergenceError(ModelError): ...
class InsufficientDataError(ModelError): ...
class PricingError(CiosError): ...
class InvalidExpenseLoadingError(PricingError): ...
class DataError(CiosError): ...
class BoundsViolationError(DataError): ...
```

## Dependencies

```toml
[project]
name = "cios"
requires-python = ">=3.10"
dependencies = []  # zero required deps beyond native extension

[project.optional-dependencies]
numpy = ["numpy>=1.24"]
pandas = ["pandas>=2.0"]
polars = ["polars>=0.20"]
all = ["cios[numpy,pandas,polars]"]
dev = ["pytest>=7", "mypy>=1.8", "ruff>=0.3"]
```

## Build Integration

The Python SDK is a pure-Python package. The native extension (`_cios_core`)
is built separately by `maturin` from `crates/cios-pyo3`. The SDK detects
and imports it at runtime:

```python
# cios/_native.py
def _load_native():
    try:
        import _cios_core
        return _cios_core
    except ImportError:
        raise ImportError(
            "cios native extension not found. "
            "Install with: pip install cios[native]"
        )
```

## Implementation Steps

1. Create `python/cios/` package structure
2. Create `python/pyproject.toml`
3. Implement `cios/_native.py` — lazy native module loader
4. Implement `cios/exceptions.py` — exception hierarchy
5. Implement `cios/types/` — all dataclass models
6. Implement `cios/model/glm.py` — FittedGlm wrapper + fit functions
7. Implement `cios/pricing/` — expense, credibility, premium
8. Implement `cios/reserving/` — triangle + chain-ladder with DataFrame interop
9. Implement `cios/explain/` — attribution wrapper
10. Implement `cios/governance/` — filing wrapper
11. Implement `cios/pipeline/runner.py` — Pipeline orchestrator
12. Implement `cios/io/` — CSV/JSON converters
13. Implement `cios/sim/generators.py` — simulation wrappers
14. Implement `cios/cli/unflatten.py` — shared unflatten logic
15. Implement `cios/cli/flatten.py` — SDK → flat file reassembly
16. Create `python/examples/cios_fleet_90d.py` — primary canonical flat file (v0.3.0, Rust-native fallback)
17. Create additional use-case flat files (crane, highway, urban, autonomous, renewal, catastrophe)
18. Write `tests/conftest.py` with shared fixtures
19. Write unit tests for every module
20. Add CI test that executes every canonical flat file and verifies deterministic output
21. Generate `.pyi` stub files
22. Write `docs/quickstart.md`, `docs/api_reference.md`, and `docs/flat-file-workflow.md`
