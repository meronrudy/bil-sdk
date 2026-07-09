# Repository Assessment

This document is the source of truth for repo-wide claims about what this tree
actually contains.

The root Cargo workspace is the public BIL thin waist. The repository is
broader than that workspace and should be described that way in the README and
boundary docs.

## Measurement Policy

- The headline metric is Rust LOC only.
- Tests count with the lane they live in.
- Primary Rust totals exclude `target/`, `Cargo.lock`, markdown, JSON, TOML,
  and generated artifacts.
- `HEAD tracked Rust LOC` measures committed repo truth from the Git tree.
- `Post-reorg target Rust LOC` measures the intended on-disk repo shape once
  `archive/` and `docs/history/` are committed.
- Markdown lines are secondary documentation inventory only.
- Re-run `python3 scripts/repo_loc_report.py` to regenerate the tables below.

## Rust LOC Baselines

### HEAD tracked Rust LOC

| Lane | Rust LOC | Share |
| --- | ---: | ---: |
| Public BIL core (`crates/`) | 2,315 | 16.9% |
| AiSSURANCE bridge (`integrations/`) | 650 | 4.7% |
| AiSSURANCE downstream (`Insurance/AiSSURANCE`) | 5,650 | 41.3% |
| AXIOM experiment (`experiments/axiom-tui`) | 5,073 | 37.1% |
| Total | 13,688 | 100.0% |

### Post-reorg target Rust LOC

| Lane | Rust LOC | Share |
| --- | ---: | ---: |
| Public BIL core (`crates/`) | 2,315 | 12.9% |
| AiSSURANCE bridge (`integrations/`) | 650 | 3.6% |
| AiSSURANCE downstream (`Insurance/AiSSURANCE`) | 5,650 | 31.5% |
| AXIOM experiment (`experiments/axiom-tui`) | 5,073 | 28.3% |
| Archived insurance experiments (`archive/insurance-experiments`) | 4,235 | 23.6% |
| Total | 17,923 | 100.0% |

The honest read is simple: the public BIL core is the supported default
workspace, but it is not most of the repository by Rust LOC.

## Markdown Inventory

The doc inventory is secondary context for how much narrative material exists in
each lane. These counts are expected to move as the docs are rewritten.

| Tier | Markdown lines |
| --- | ---: |
| Active public docs | 504 |
| AiSSURANCE docs | 466 |
| Historical docs | 465 |
| Archive docs | 128 |

## Support Tiers

| Tier | Paths | Meaning |
| --- | --- | --- |
| Public supported core | `crates/` | Default workspace, default tests, domain-neutral contract |
| Supported local downstream | `Insurance/AiSSURANCE`, `integrations/bil-aissurance-bridge` | Supported local integration path outside the default workspace |
| Live standalone experiment | `experiments/axiom-tui` | Active experiment, not part of root workspace guarantees |
| Historical/archive | `archive/insurance-experiments`, `docs/history` | Reference-only material, not default CI |

## Claim Discipline

- It is accurate to describe BIL as the public thin-waist workspace in this
  repository.
- It is inaccurate to describe the repository itself as only the public thin
  waist.
- Repo-wide claims should distinguish `workspace` from `repository`.
- Downstream, experiment, and archive lanes should be named explicitly when
  discussing repo scope.
