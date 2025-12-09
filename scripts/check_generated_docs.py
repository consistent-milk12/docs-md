#!/usr/bin/env python3
"""Check generated markdown docs for common issues.

This script scans generated markdown files for known problems like:
- Duplicate anchor IDs
- Broken self-referential links
- Generic parameters on concrete type impls

Usage:
    python scripts/check_generated_docs.py <docs_dir> [--file <specific_file>]

Examples:
    python scripts/check_generated_docs.py generated_docs/
    python scripts/check_generated_docs.py generated_docs/ --file cargo_docs_md/generator/toc/index.md
"""

import argparse
import re
import sys
from collections import defaultdict
from pathlib import Path


def check_duplicate_anchors(content: str, _file_path: Path) -> list[str]:
    """Check for duplicate anchor IDs in a file."""
    issues = []
    anchor_pattern = re.compile(r'<span id="([^"]+)">')
    anchors = anchor_pattern.findall(content)

    seen = defaultdict(list)
    for i, anchor in enumerate(anchors, 1):
        seen[anchor].append(i)

    for anchor, occurrences in seen.items():
        if len(occurrences) > 1:
            issues.append(f"Duplicate anchor '{anchor}' found {len(occurrences)} times")

    return issues


def check_self_referential_links(content: str, _file_path: Path) -> list[str]:
    """Check for links that point to parent index when they should be local anchors."""
    issues = []

    # Pattern: [`TypeName`](../index.md) where TypeName is defined in this file
    link_pattern = re.compile(r"\[`([^`]+)`\]\(\.\./index\.md\)")
    heading_pattern = re.compile(r"^###?\s+`?(\w+)`?", re.MULTILINE)

    # Get all type/item names defined in this file
    defined_names = set(heading_pattern.findall(content))

    # Find links to ../index.md
    for match in link_pattern.finditer(content):
        link_text = match.group(1)
        # Extract just the type name (remove generics, etc.)
        type_name = link_text.split("<")[0].split("::")[-1]

        if type_name in defined_names:
            line_num = content[: match.start()].count("\n") + 1
            issues.append(
                f"Line {line_num}: Link to '{link_text}' points to ../index.md "
                f"but '{type_name}' is defined in this file (should be #{type_name.lower()})"
            )

    return issues


def check_generic_params_on_concrete(content: str, _file_path: Path) -> list[str]:
    """Check for impl blocks with generic params on concrete types."""
    issues = []

    # Pattern: impl<T> TraitName for ConcreteType (not T)
    impl_pattern = re.compile(r"`impl<([^>]+)>\s+(\w+)\s+for\s+(\w+)`")

    for match in impl_pattern.finditer(content):
        generic_param = match.group(1).strip()
        trait_name = match.group(2)
        for_type = match.group(3)

        # If the for_type is NOT the generic param, it's suspicious
        if for_type != generic_param and not for_type.startswith(generic_param):
            line_num = content[: match.start()].count("\n") + 1
            issues.append(
                f"Line {line_num}: `impl<{generic_param}> {trait_name} for {for_type}` "
                f"has generic param but concrete type"
            )

    return issues


def check_file(file_path: Path) -> dict[str, list[str]]:
    """Run all checks on a single file."""
    content = file_path.read_text()

    return {
        "duplicate_anchors": check_duplicate_anchors(content, file_path),
        "self_referential_links": check_self_referential_links(content, file_path),
        "generic_on_concrete": check_generic_params_on_concrete(content, file_path),
    }


def main():
    parser = argparse.ArgumentParser(description="Check generated docs for issues")
    parser.add_argument("docs_dir", help="Path to generated docs directory")
    parser.add_argument(
        "--file",
        dest="specific_file",
        help="Check only this file (relative to docs_dir)",
    )
    parser.add_argument(
        "--verbose", "-v", action="store_true", help="Show all files checked"
    )
    args = parser.parse_args()

    docs_path = Path(args.docs_dir)
    if not docs_path.exists():
        print(f"Directory not found: {docs_path}", file=sys.stderr)
        sys.exit(1)

    if args.specific_file:
        files = [docs_path / args.specific_file]
    else:
        files = list(docs_path.rglob("*.md"))

    total_issues = 0
    files_with_issues = 0

    for file_path in sorted(files):
        if args.verbose:
            print(f"Checking {file_path.relative_to(docs_path)}...")

        issues = check_file(file_path)
        file_has_issues = False

        for category, category_issues in issues.items():
            if category_issues:
                if not file_has_issues:
                    print(f"\n{file_path.relative_to(docs_path)}:")
                    file_has_issues = True
                    files_with_issues += 1

                print(f"  [{category}]")
                for issue in category_issues:
                    print(f"    - {issue}")
                    total_issues += 1

    print(f"\n{'=' * 60}")
    print(f"Checked {len(files)} files")
    print(f"Found {total_issues} issues in {files_with_issues} files")

    sys.exit(1 if total_issues > 0 else 0)


if __name__ == "__main__":
    main()
