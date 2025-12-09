#!/usr/bin/env python3
"""Inspect impl blocks for a specific type in rustdoc JSON.

Usage:
    python scripts/inspect_impl.py <json_file> <type_name> [--trait <trait_name>]

Examples:
    python scripts/inspect_impl.py target/doc/cargo_docs_md.json TocEntry
    python scripts/inspect_impl.py target/doc/cargo_docs_md.json TocEntry --trait Pointable
"""

import argparse
import json
import sys


def find_type_id(data: dict, type_name: str) -> str | None:
    """Find the ID of a type by name."""
    for id, item in data["index"].items():
        if item.get("name") == type_name:
            inner = item.get("inner", {})
            if isinstance(inner, dict) and ("struct" in inner or "enum" in inner):
                return id
    return None


def get_impl_ids(data: dict, type_id: str) -> list:
    """Get impl IDs for a type."""
    item = data["index"].get(type_id)
    if not item:
        return []

    inner = item.get("inner", {})
    if "struct" in inner:
        return inner["struct"].get("impls", [])
    elif "enum" in inner:
        return inner["enum"].get("impls", [])
    return []


def inspect_impl(data: dict, impl_id: str) -> dict | None:
    """Extract impl information."""
    impl_item = data["index"].get(str(impl_id))
    if not impl_item:
        return None

    inner = impl_item.get("inner", {})
    if "impl" not in inner:
        return None

    impl_data = inner["impl"]
    trait_ref = impl_data.get("trait_")

    return {
        "id": impl_id,
        "trait": trait_ref.get("path") if trait_ref else "inherent",
        "for_": impl_data.get("for_"),
        "generics": impl_data.get("generics", {}),
        "is_synthetic": impl_data.get("is_synthetic", False),
        "items": impl_data.get("items", []),
    }


def main():
    parser = argparse.ArgumentParser(description="Inspect impl blocks in rustdoc JSON")
    parser.add_argument("json_file", help="Path to rustdoc JSON file")
    parser.add_argument("type_name", help="Name of the type to inspect")
    parser.add_argument("--trait", dest="trait_name", help="Filter by trait name")
    parser.add_argument(
        "--verbose", "-v", action="store_true", help="Show full details"
    )
    args = parser.parse_args()

    with open(args.json_file) as f:
        data = json.load(f)

    type_id = find_type_id(data, args.type_name)
    if not type_id:
        print(f"Type '{args.type_name}' not found", file=sys.stderr)
        sys.exit(1)

    print(f"Found {args.type_name} with ID: {type_id}")

    impl_ids = get_impl_ids(data, type_id)
    print(f"Total impls: {len(impl_ids)}\n")

    for impl_id in impl_ids:
        info = inspect_impl(data, impl_id)
        if not info:
            continue

        # Filter by trait if specified
        if args.trait_name and args.trait_name not in info["trait"]:
            continue

        print(f"Trait: {info['trait']}")
        if info["is_synthetic"]:
            print("  [SYNTHETIC]")

        if info["for_"]:
            if args.verbose:
                print(f"  for_: {json.dumps(info['for_'], indent=4)}")
            else:
                print(f"  for_: {info['for_']}")

        gen_params = info["generics"].get("params", [])
        if gen_params:
            if args.verbose:
                print(f"  generics.params: {json.dumps(gen_params, indent=4)}")
            else:
                print(f"  generics.params: {gen_params}")

        if args.verbose and info["items"]:
            print(f"  items: {info['items']}")

        print()


if __name__ == "__main__":
    main()
