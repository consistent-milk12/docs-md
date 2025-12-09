#!/usr/bin/env python3
"""Dump detailed type information from rustdoc JSON.

Usage:
    python scripts/dump_type_info.py <json_file> <type_name>

Examples:
    python scripts/dump_type_info.py target/doc/cargo_docs_md.json TocEntry
    python scripts/dump_type_info.py target/doc/cargo_docs_md.json TocGenerator
"""

import argparse
import json
import sys


def find_type(data: dict, type_name: str) -> tuple[str, dict] | None:
    """Find a type by name and return (id, item)."""
    for id, item in data["index"].items():
        if item.get("name") == type_name:
            inner = item.get("inner", {})
            if isinstance(inner, dict) and (
                "struct" in inner or "enum" in inner or "trait" in inner
            ):
                return (id, item)
    return None


def main():
    parser = argparse.ArgumentParser(
        description="Dump type information from rustdoc JSON"
    )
    parser.add_argument("json_file", help="Path to rustdoc JSON file")
    parser.add_argument("type_name", help="Name of the type to dump")
    parser.add_argument("--full", action="store_true", help="Dump full JSON")
    args = parser.parse_args()

    with open(args.json_file) as f:
        data = json.load(f)

    result = find_type(data, args.type_name)
    if not result:
        print(f"Type '{args.type_name}' not found", file=sys.stderr)
        sys.exit(1)

    type_id, item = result
    print(f"ID: {type_id}")
    print(f"Name: {item.get('name')}")
    print(f"Visibility: {item.get('visibility')}")
    print(f"Crate ID: {item.get('crate_id')}")

    inner = item.get("inner", {})
    if "struct" in inner:
        struct_data = inner["struct"]
        print("\nKind: struct")
        print(f"Fields: {struct_data.get('kind', {})}")
        print(f"Generics: {struct_data.get('generics', {})}")
        print(f"Impls count: {len(struct_data.get('impls', []))}")

        if args.full:
            print("\nFull struct data:")
            print(json.dumps(struct_data, indent=2))

    elif "enum" in inner:
        enum_data = inner["enum"]
        print("\nKind: enum")
        print(f"Variants: {len(enum_data.get('variants', []))}")
        print(f"Generics: {enum_data.get('generics', {})}")
        print(f"Impls count: {len(enum_data.get('impls', []))}")

        if args.full:
            print("\nFull enum data:")
            print(json.dumps(enum_data, indent=2))

    elif "trait" in inner:
        trait_data = inner["trait"]
        print("\nKind: trait")
        print(f"Items: {len(trait_data.get('items', []))}")
        print(f"Generics: {trait_data.get('generics', {})}")
        print(f"Implementations count: {len(trait_data.get('implementations', []))}")

        if args.full:
            print("\nFull trait data:")
            print(json.dumps(trait_data, indent=2))


if __name__ == "__main__":
    main()
