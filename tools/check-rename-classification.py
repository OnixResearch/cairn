#!/usr/bin/env python3
"""Classify remaining legacy `ironspec` name occurrences after the Cairn rename."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKIP_PARTS = {".git", "target", "artifacts", ".direnv"}
TEXT_SUFFIXES = {
    ".md",
    ".rs",
    ".toml",
    ".nix",
    ".json",
    ".ncl",
    ".py",
    ".lock",
    ".txt",
    ".styx",
}
LEGACY = "ironspec"


@dataclass(order=True)
class Occurrence:
    path: str
    line: int
    bucket: str
    text: str


def is_text_file(path: Path) -> bool:
    return path.name in {"Cargo.lock", "README.md"} or path.suffix in TEXT_SUFFIXES


def iter_files() -> list[Path]:
    files: list[Path] = []
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        rel = path.relative_to(ROOT)
        if any(part in SKIP_PARTS for part in rel.parts):
            continue
        if rel.as_posix().endswith("evidence/rename-classification.json"):
            continue
        if is_text_file(path):
            files.append(path)
    return sorted(files)


def classify(path: str, line: str) -> str:
    text = line.lower()
    path_text = path.lower()
    if "/archive/" in path_text or "openspec/changes/archive/" in path_text:
        return "historical-archive"
    if path_text.startswith("openspec/specs/"):
        return "requirement-id-history"
    if "rename-ironspec-to-cairn" in path_text or path_text == "tools/check-rename-classification.py":
        return "intentional-reference"
    if "ironspec" in path_text and (
        "docs/hermes" in path_text
        or "tool" in path_text
        or "src/ironspec.rs" in path_text
        or "ironspec-supersession" in path_text
    ):
        return "intentional-reference"
    if "r[" in text and "ironspec." in text:
        return "requirement-id-history"
    if "ironspec" in text and "cairn" in text:
        return "intentional-reference"
    compatibility_tokens = [
        "legacy",
        "alias",
        "compat",
        "migration source",
        "migrate ironspec",
        "migrate_ironspec",
        "legacyironspec",
        "build_ironspec_migration_plan",
        "cargo_bin_exe_ironspec",
        "ironspec_archive_date",
        "name = \"ironspec\"",
        "path = \"src/ironspec.rs\"",
        "bin/ironspec",
        "invoked_as.ends_with(\"ironspec\")",
        "source_layout\"] == \"ironspec\"",
        "dry_run[\"source_layout\"], \"ironspec\"",
        "source_layout\", \"ironspec\"",
        "migrate-ironspec",
        "ironspec = {",
        "\"ironspec\" =>",
        "\"ironspec\",",
        "[\"openspec\", \"ironspec\"]",
        "openspec or ironspec",
        "<openspec|ironspec>",
    ]
    if "ironspec" in text and any(token in text for token in compatibility_tokens):
        return "compatibility-alias"
    return "needs-fix"


def scan() -> list[Occurrence]:
    # r[impl cairn.supersede.rename_cairn.classification]
    # r[verify cairn.supersede.rename_cairn.classification]
    occurrences: list[Occurrence] = []
    for path in iter_files():
        rel = path.relative_to(ROOT).as_posix()
        try:
            lines = path.read_text().splitlines()
        except UnicodeDecodeError:
            continue
        for index, line in enumerate(lines, start=1):
            if LEGACY in line.lower():
                occurrences.append(Occurrence(rel, index, classify(rel, line), line.strip()))
        if LEGACY in rel.lower():
            occurrences.append(Occurrence(rel, 0, classify(rel, rel), "<path>"))
    return sorted(occurrences)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, help="optional JSON evidence output")
    args = parser.parse_args()

    occurrences = scan()
    buckets: dict[str, int] = {}
    for occurrence in occurrences:
        buckets[occurrence.bucket] = buckets.get(occurrence.bucket, 0) + 1
    report = {
        "legacy": LEGACY,
        "canonical": "cairn",
        "total": len(occurrences),
        "buckets": dict(sorted(buckets.items())),
        "occurrences": [asdict(occurrence) for occurrence in occurrences],
        "valid": buckets.get("needs-fix", 0) == 0,
    }
    rendered = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(rendered)
    sys.stdout.write(rendered)
    if not report["valid"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
