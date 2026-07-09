#!/usr/bin/env python3

from __future__ import annotations

import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
IGNORED_DIRS = {
    ".git",
    ".hg",
    ".svn",
    ".venv",
    "__pycache__",
    "node_modules",
    "target",
    "venv",
}


@dataclass(frozen=True)
class Lane:
    label: str
    path: str


@dataclass(frozen=True)
class Tier:
    label: str
    files: tuple[str, ...]


HEAD_RUST_LANES = (
    Lane("Public BIL core (`crates/`)", "crates"),
    Lane("AiSSURANCE bridge (`integrations/`)", "integrations/bil-aissurance-bridge"),
    Lane("AiSSURANCE downstream (`Insurance/AiSSURANCE`)", "Insurance/AiSSURANCE"),
    Lane("AXIOM experiment (`experiments/axiom-tui`)", "experiments/axiom-tui"),
)

POST_REORG_RUST_LANES = HEAD_RUST_LANES + (
    Lane(
        "Archived insurance experiments (`archive/insurance-experiments`)",
        "archive/insurance-experiments",
    ),
)

DOC_TIERS = (
    Tier(
        "Active public docs",
        (
            "README.md",
            "docs/aissurance-local-integration.md",
            "docs/architecture.md",
            "docs/bil-thin-waist.md",
            "docs/canonical_encoding.md",
            "docs/conformance.md",
            "docs/ink_receipts.md",
            "docs/open-core-boundary.md",
            "docs/project-completion-guide.md",
            "docs/signer_ladder.md",
            "docs/verification-semantics.md",
            "docs/verification.md",
            "examples/generic/human_override/README.md",
        ),
    ),
    Tier(
        "AiSSURANCE docs",
        (
            "Insurance/AiSSURANCE/README.md",
            "Insurance/AiSSURANCE/docs/README.md",
            "Insurance/AiSSURANCE/docs/api/risk_layer.md",
            "Insurance/AiSSURANCE/docs/installation.md",
            "Insurance/AiSSURANCE/docs/roadmap.md",
            "Insurance/AiSSURANCE/docs/user-guide.md",
            "Insurance/AiSSURANCE/plans/release_2_plan.md",
        ),
    ),
    Tier("Historical docs", ("docs/history/repo-split.md",)),
    Tier(
        "Archive docs",
        (
            "archive/insurance-experiments/construction-insurance-os/README.md",
            "archive/insurance-experiments/mechassure/mechassure-architecture.md",
        ),
    ),
)


def fail(message: str) -> "NoReturn":
    raise SystemExit(message)


def git_stdout(*args: str) -> str:
    return subprocess.check_output(
        ["git", *args],
        cwd=ROOT,
        text=True,
    )


def count_text_lines(contents: str) -> int:
    return len(contents.splitlines())


def count_worktree_lines(path: Path) -> int:
    with path.open("r", encoding="utf-8", errors="ignore") as handle:
        return sum(1 for _ in handle)


def walk_files(base: Path, suffix: str) -> list[Path]:
    if not base.exists():
        fail(f"expected lane path is missing: {base}")
    if not base.is_dir():
        fail(f"expected lane path is not a directory: {base}")

    results: list[Path] = []
    for path in base.rglob(f"*{suffix}"):
        if any(part in IGNORED_DIRS for part in path.parts):
            continue
        if path.is_file():
            results.append(path)

    if not results:
        fail(f"no matching {suffix} files found under: {base}")

    return sorted(results)


def head_rust_counts() -> list[tuple[str, int]]:
    tracked = git_stdout("ls-tree", "-r", "--name-only", "HEAD").splitlines()
    results: list[tuple[str, int]] = []
    for lane in HEAD_RUST_LANES:
        prefix = f"{lane.path}/"
        paths = [path for path in tracked if path.startswith(prefix) and path.endswith(".rs")]
        if not paths:
            fail(f"no HEAD-tracked Rust files found for lane: {lane.path}")
        total = 0
        for path in paths:
            total += count_text_lines(git_stdout("show", f"HEAD:{path}"))
        results.append((lane.label, total))
    return results


def post_reorg_rust_counts() -> list[tuple[str, int]]:
    results: list[tuple[str, int]] = []
    for lane in POST_REORG_RUST_LANES:
        files = walk_files(ROOT / lane.path, ".rs")
        total = sum(count_worktree_lines(path) for path in files)
        results.append((lane.label, total))
    return results


def doc_counts() -> list[tuple[str, int]]:
    results: list[tuple[str, int]] = []
    for tier in DOC_TIERS:
        total = 0
        for rel_path in tier.files:
            path = ROOT / rel_path
            if not path.exists():
                fail(f"expected doc file is missing: {rel_path}")
            total += count_worktree_lines(path)
        results.append((tier.label, total))
    return results


def print_rust_table(title: str, rows: list[tuple[str, int]]) -> None:
    total = sum(value for _, value in rows)
    print(f"## {title}")
    print()
    print("| Lane | Rust LOC | Share |")
    print("| --- | ---: | ---: |")
    for label, value in rows:
        share = (value / total) * 100 if total else 0.0
        print(f"| {label} | {value:,} | {share:.1f}% |")
    print(f"| Total | {total:,} | 100.0% |")
    print()


def print_doc_table(rows: list[tuple[str, int]]) -> None:
    print("## Markdown Inventory")
    print()
    print("| Tier | Markdown lines |")
    print("| --- | ---: |")
    for label, value in rows:
        print(f"| {label} | {value:,} |")
    print()


def main() -> int:
    print("# Repo LOC Report")
    print()
    print("Generated by `python3 scripts/repo_loc_report.py`.")
    print()
    print_rust_table("HEAD tracked Rust LOC", head_rust_counts())
    print_rust_table("Post-reorg target Rust LOC", post_reorg_rust_counts())
    print_doc_table(doc_counts())
    return 0


if __name__ == "__main__":
    sys.exit(main())
