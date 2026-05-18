#!/usr/bin/env python3
"""Export Nickel policy to generated JSON, or check freshness."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_SOURCE = ROOT / "cairn-policy" / "default.ncl"
DEFAULT_OUTPUT = ROOT / "cairn-policy" / "generated" / "cairn-policy.json"
FIXTURES = ROOT / "cairn-policy" / "fixtures"


def run(cmd: list[str], *, expect_ok: bool = True) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(cmd, cwd=ROOT, text=True, capture_output=True)
    if expect_ok and result.returncode != 0:
        sys.stderr.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    return result


def nickel_export(source: Path) -> str:
    result = run(["nickel", "export", "--format", "json", str(source)])
    parsed = json.loads(result.stdout)
    check_policy_semantics(parsed, source)
    return json.dumps(parsed, indent=2, sort_keys=True) + "\n"


def require(condition: bool, message: str, source: Path) -> None:
    if not condition:
        raise SystemExit(f"policy semantic check failed for {source}: {message}")


def require_unique(values: list[str], label: str, source: Path) -> None:
    seen: set[str] = set()
    duplicates: set[str] = set()
    for value in values:
        if value in seen:
            duplicates.add(value)
        seen.add(value)
    require(not duplicates, f"duplicate {label}: {', '.join(sorted(duplicates))}", source)


def check_artifact_dependencies(policy: dict, source: Path) -> None:
    for schema in policy.get("schemas", []):
        artifact_ids = [artifact.get("id") for artifact in schema.get("artifacts", [])]
        require_unique(artifact_ids, f"artifact ids in schema {schema.get('id')}", source)
        known = set(artifact_ids)
        for artifact in schema.get("artifacts", []):
            for required in artifact.get("requires", []):
                require(
                    required in known,
                    f"artifact {artifact.get('id')} requires unknown artifact {required}",
                    source,
                )


def check_policy_semantics(policy: dict, source: Path) -> None:
    require(policy.get("version") == 1, "version must be 1", source)
    receipt_hash = policy.get("receipt_hash_policy", {})
    require(receipt_hash.get("algorithm") == "BLAKE3", "receipt hash algorithm must be BLAKE3", source)
    require(
        receipt_hash.get("canonicalization") == "serde_json_compact_sorted_keys",
        "receipt hash canonicalization must be serde_json_compact_sorted_keys",
        source,
    )
    for field in ["schemas", "lifecycle_stages", "receipt_schemas"]:
        require(bool(policy.get(field)), f"{field} must be non-empty", source)
    require(bool(policy.get("gate_policy", {}).get("stages")), "gate_policy.stages must be non-empty", source)
    marker_policy = policy.get("task_marker_policy", {})
    markers = marker_policy.get("markers", [])
    require(bool(markers), "task_marker_policy.markers must be non-empty", source)
    require_unique(policy.get("lifecycle_stages", []), "lifecycle stages", source)
    require_unique(marker_policy.get("stages", []), "task marker stages", source)
    require_unique([marker.get("id") for marker in markers], "marker ids", source)
    require_unique([marker.get("token") for marker in markers], "marker tokens", source)
    receipt_schemas = policy.get("receipt_schemas", [])
    require_unique([schema.get("command") for schema in receipt_schemas], "receipt schema commands", source)
    for schema in receipt_schemas:
        command = schema.get("command")
        require_unique(schema.get("top_level_keys", []), f"{command} top-level receipt keys", source)
        require_unique(schema.get("action_keys", []), f"{command} action keys", source)
        require_unique(schema.get("issue_keys", []), f"{command} issue keys", source)
        for hash_field in schema.get("hash_fields", []):
            require(
                hash_field in schema.get("top_level_keys", []),
                f"{command} hash field {hash_field} is not a top-level key",
                source,
            )
    exemption_marker = policy.get("exemption_marker_policy", {})
    required_fields = exemption_marker.get("required_fields", [])
    for field in ["reason", "owner", "until"]:
        require(field in required_fields, f"exemption marker missing required {field} field", source)
    for field in exemption_marker.get("date_fields", []):
        require(field in required_fields, f"exemption date field {field} must also be required", source)
    for exemption in policy.get("exemptions", []):
        require(
            re.fullmatch(r"\d{4}-\d{2}-\d{2}", exemption.get("expires", "")) is not None,
            f"exemption {exemption.get('scope')} expires must be YYYY-MM-DD",
            source,
        )
    check_artifact_dependencies(policy, source)


def check_invalid_fixtures() -> None:
    invalids = sorted(FIXTURES.glob("invalid-*.ncl"))
    for fixture in invalids:
        result = run(["nickel", "export", "--format", "json", str(fixture)], expect_ok=False)
        if result.returncode != 0:
            continue
        try:
            check_policy_semantics(json.loads(result.stdout), fixture)
        except SystemExit:
            continue
        raise SystemExit(f"expected invalid policy fixture to fail: {fixture}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated output is stale")
    parser.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()

    rendered = nickel_export(args.source)
    check_invalid_fixtures()

    if args.check:
        existing = args.output.read_text() if args.output.exists() else ""
        if existing != rendered:
            raise SystemExit(f"generated policy is stale: {args.output}")
        print(f"policy fresh: {args.output}")
        return

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(rendered)
    print(f"wrote {args.output}")


if __name__ == "__main__":
    main()
