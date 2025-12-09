#!/usr/bin/env python3
"""Find blanket impls in rustdoc JSON.

Blanket impls are implementations like `impl<T> Trait for T` where the
`for_` type is a generic type parameter.

Usage:
    python scripts/find_blanket_impls.py <json_file> [--type <type_name>]

Examples:
    python scripts/find_blanket_impls.py target/doc/cargo_docs_md.json
    python scripts/find_blanket_impls.py target/doc/cargo_docs_md.json --type TocEntry
"""

import argparse
import json


def is_generic_type(type_data: dict | str | None) -> bool:
    """Check if a type is a generic type parameter like `T`."""
    if type_data is None:
        return False

    if isinstance(type_data, str):
        return False

    if isinstance(type_data, dict):
        # Direct generic like {"generic": "T"}
        if "generic" in type_data:
            return True

        # Resolved path - check args
        if "resolved_path" in type_data:
            path_data = type_data["resolved_path"]
            args = path_data.get("args")
            if args and "angle_bracketed" in args:
                for arg in args["angle_bracketed"].get("args", []):
                    if isinstance(arg, dict) and "type" in arg:
                        if is_generic_type(arg["type"]):
                            return True
            return False

        # Reference - check inner type
        if "borrowed_ref" in type_data:
            return is_generic_type(type_data["borrowed_ref"].get("type"))

        # Tuple - check all elements
        if "tuple" in type_data:
            return any(is_generic_type(t) for t in type_data["tuple"])

    return False


def find_blanket_impls(data: dict, type_name: str | None = None):
    """Find all blanket impls, optionally filtered by type."""
    results = []

    for id, item in data["index"].items():
        inner = item.get("inner", {})
        if not isinstance(inner, dict) or "impl" not in inner:
            continue

        impl_data = inner["impl"]
        trait_ref = impl_data.get("trait_")
        if not trait_ref:
            continue

        for_type = impl_data.get("for_")
        gen_params = impl_data.get("generics", {}).get("params", [])

        # Check if it's a blanket impl (for_ type is generic)
        if is_generic_type(for_type) and gen_params:
            trait_path = trait_ref.get("path", "unknown")

            # If filtering by type, check if this impl applies to that type
            if type_name:
                # This would require more complex logic to match
                # For now, just collect all blanket impls
                pass

            results.append(
                {
                    "id": id,
                    "trait": trait_path,
                    "for_": for_type,
                    "generics": gen_params,
                }
            )

    return results


def main():
    parser = argparse.ArgumentParser(description="Find blanket impls in rustdoc JSON")
    parser.add_argument("json_file", help="Path to rustdoc JSON file")
    parser.add_argument("--type", dest="type_name", help="Filter by type name")
    parser.add_argument("--limit", type=int, default=20, help="Max results to show")
    args = parser.parse_args()

    with open(args.json_file) as f:
        data = json.load(f)

    blanket_impls = find_blanket_impls(data, args.type_name)

    print(f"Found {len(blanket_impls)} blanket impls\n")

    for impl in blanket_impls[: args.limit]:
        print(f"Trait: {impl['trait']}")
        print(f"  for_: {impl['for_']}")
        print(f"  generics: {[p.get('name', p) for p in impl['generics']]}")
        print()

    if len(blanket_impls) > args.limit:
        print(f"... and {len(blanket_impls) - args.limit} more")


if __name__ == "__main__":
    main()
