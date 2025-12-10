#!/usr/bin/env python3
"""Script index and help system for docs-md Python utilities.

Usage:
    python3 scripts/index.py              # List all scripts
    python3 scripts/index.py <name>       # Show help for specific script
    python3 scripts/index.py --all        # Show detailed help for all scripts

Examples:
    python3 scripts/index.py
    python3 scripts/index.py inspect_impl
    python3 scripts/index.py check_generated_docs
"""

import argparse
import subprocess
import sys
from pathlib import Path

# Registry of scripts with descriptions
# Format: "script_name": "brief description"
SCRIPTS = {
    "analyze_write_calls": "Analyze write!/writeln! calls for memory overhead",
    "check_generated_docs": "Check generated markdown docs for common issues",
    "dump_type_info": "Dump detailed type information from rustdoc JSON",
    "find_blanket_impls": "Find blanket impls (impl<T> Trait for T) in rustdoc JSON",
    "inspect_impl": "Inspect impl blocks for a specific type in rustdoc JSON",
}

# Example usages for each script
EXAMPLES = {
    "analyze_write_calls": [
        "python3 scripts/analyze_write_calls.py",
    ],
    "inspect_impl": [
        "python3 scripts/inspect_impl.py target/doc/cargo_docs_md.json TocEntry",
        "python3 scripts/inspect_impl.py target/doc/cargo_docs_md.json TocEntry --trait Pointable",
        "python3 scripts/inspect_impl.py target/doc/cargo_docs_md.json TocEntry -v",
    ],
    "find_blanket_impls": [
        "python3 scripts/find_blanket_impls.py target/doc/cargo_docs_md.json",
        "python3 scripts/find_blanket_impls.py target/doc/cargo_docs_md.json --limit 50",
    ],
    "dump_type_info": [
        "python3 scripts/dump_type_info.py target/doc/cargo_docs_md.json TocEntry",
        "python3 scripts/dump_type_info.py target/doc/cargo_docs_md.json TocGenerator --full",
    ],
    "check_generated_docs": [
        "python3 scripts/check_generated_docs.py generated_docs/",
        "python3 scripts/check_generated_docs.py generated_docs/ --file cargo_docs_md/generator/toc/index.md",
        "python3 scripts/check_generated_docs.py generated_docs/ -v",
    ],
}


def get_script_path(name: str) -> Path:
    """Get the path to a script by name."""
    scripts_dir = Path(__file__).parent
    return scripts_dir / f"{name}.py"


def list_scripts():
    """List all available scripts with brief descriptions."""
    print("Available scripts in scripts/:\n")

    max_name_len = max(len(name) for name in SCRIPTS)

    for name, description in sorted(SCRIPTS.items()):
        script_path = get_script_path(name)
        status = "✓" if script_path.exists() else "✗"
        print(f"  {status} {name:<{max_name_len}}  {description}")

    print("\nUsage:")
    print("  python3 scripts/index.py <name>       Show help for a script")
    print("  python3 scripts/<name>.py --help      Run script's own help")


def show_script_help(name: str):
    """Show detailed help for a specific script."""
    if name not in SCRIPTS:
        print(f"Unknown script: {name}", file=sys.stderr)
        print(
            f"Available scripts: {', '.join(sorted(SCRIPTS.keys()))}", file=sys.stderr
        )
        sys.exit(1)

    script_path = get_script_path(name)

    print(f"Script: {name}.py")
    print(f"Description: {SCRIPTS[name]}")
    print()

    if not script_path.exists():
        print(f"WARNING: Script file not found at {script_path}", file=sys.stderr)
        return

    # Show examples
    if name in EXAMPLES:
        print("Examples:")
        for example in EXAMPLES[name]:
            print(f"  {example}")
        print()

    # Try to get the script's own help
    print("Full help (from --help):")
    print("-" * 60)
    try:
        result = subprocess.run(
            [sys.executable, str(script_path), "--help"],
            capture_output=True,
            text=True,
            timeout=5,
        )
        print(result.stdout)
        if result.stderr:
            print(result.stderr, file=sys.stderr)
    except subprocess.TimeoutExpired:
        print("(help timed out)", file=sys.stderr)
    except Exception as e:
        print(f"(could not get help: {e})", file=sys.stderr)


def show_all_help():
    """Show detailed help for all scripts."""
    for name in sorted(SCRIPTS.keys()):
        print("=" * 60)
        show_script_help(name)
        print()


def main():
    parser = argparse.ArgumentParser(
        description="Script index and help system for docs-md Python utilities",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 scripts/index.py                    List all scripts
  python3 scripts/index.py inspect_impl       Show help for inspect_impl
  python3 scripts/index.py --all              Show help for all scripts
""",
    )
    parser.add_argument(
        "script_name",
        nargs="?",
        help="Name of script to show help for (without .py extension)",
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Show detailed help for all scripts",
    )

    args = parser.parse_args()

    if args.all:
        show_all_help()
    elif args.script_name:
        show_script_help(args.script_name)
    else:
        list_scripts()


if __name__ == "__main__":
    main()
