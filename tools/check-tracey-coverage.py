#!/usr/bin/env python3
"""Deterministic bootstrap traceability check for Cairn requirement IDs.

Tracey remains the interactive source of truth. This helper gives Nix a small,
packaging-independent baseline: every accepted `r[...]` requirement marker must
have at least one Rust comment reference using `r[impl ...]` or `r[verify ...]`.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

SPEC_MARKER = re.compile(r"^r\[([a-zA-Z0-9_.-]+)(?:\+\d+)?\]")
REF_MARKER = re.compile(r"r\[(impl|verify|depends|related)\s+([a-zA-Z0-9_.-]+)(?:\+\d+)?\]")


def iter_files(root: Path, patterns: list[str]) -> list[Path]:
    files: list[Path] = []
    for pattern in patterns:
        files.extend(path for path in root.glob(pattern) if path.is_file())
    return sorted(set(files))


def collect_requirements(root: Path) -> dict[str, list[str]]:
    # r[impl cairn.supersede.tracey_coverage.config]
    requirements: dict[str, list[str]] = {}
    for path in iter_files(root, ["openspec/specs/**/*.md", "cairn/specs/**/*.md"]):
        for line_no, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
            match = SPEC_MARKER.match(line.strip())
            if match:
                requirements.setdefault(match.group(1), []).append(f"{path}:{line_no}")
    return requirements


def collect_references(root: Path) -> dict[str, list[dict[str, str]]]:
    references: dict[str, list[dict[str, str]]] = {}
    for path in iter_files(root, ["crates/**/*.rs", "tools/**/*.py"]):
        for line_no, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
            for verb, req_id in REF_MARKER.findall(line):
                if req_id == "...":
                    continue
                references.setdefault(req_id, []).append(
                    {"verb": verb, "path": str(path), "line": str(line_no)}
                )
    return references


def build_report(root: Path) -> dict[str, object]:
    # r[impl cairn.supersede.tracey_coverage] r[impl cairn.supersede.tracey_coverage.check]
    requirements = collect_requirements(root)
    references = collect_references(root)
    missing = sorted(req_id for req_id in requirements if req_id not in references)
    dangling = sorted(req_id for req_id in references if req_id not in requirements)
    return {
        "requirements": len(requirements),
        "referenced": len([req_id for req_id in requirements if req_id in references]),
        "missing": missing,
        "dangling": dangling,
        "valid": not missing and not dangling,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    report = build_report(Path(args.root))
    if args.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    elif report["valid"]:
        print(f"tracey bootstrap coverage ok: {report['referenced']}/{report['requirements']} referenced")
    else:
        print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if report["valid"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
