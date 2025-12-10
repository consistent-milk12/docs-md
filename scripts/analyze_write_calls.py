#!/usr/bin/env python3
"""
Analyze write!/writeln! calls to compute theoretical memory overhead
if they were all push_str + format! instead.

Uses search_index.json to calculate real-world scaling based on actual
documented items.

Usage: python3 scripts/analyze_write_calls.py [--search-index PATH]
"""

import argparse
import json
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# =============================================================================
# Constants
# =============================================================================

STRING_HEADER_BYTES = 24  # Rust String struct: ptr (8) + len (8) + cap (8)
ALLOC_CYCLES = 100  # Approximate CPU cycles per malloc
DEALLOC_CYCLES = 50  # Approximate CPU cycles per free
COPY_CYCLES_PER_BYTE = 0.25  # Approximate CPU cycles per byte memcpy
CPU_FREQ_GHZ = 3.0  # Reference CPU frequency for time estimates

# Estimated write! calls per item type (based on code analysis)
WRITES_PER_ITEM = {
    "struct": 8,  # heading, code block, fields, impl blocks
    "enum": 10,  # heading, code block, variants, impl blocks
    "trait": 12,  # heading, code block, methods, implementors
    "fn": 4,  # heading, signature, docs
    "type": 4,  # heading, definition, docs
    "const": 3,  # heading, value, docs
    "macro": 3,  # heading, docs
    "mod": 6,  # heading, contents list, docs
}


# =============================================================================
# Data Classes
# =============================================================================


@dataclass
class WriteCall:
    """Represents a single write!/writeln! call in the source code."""

    file: str
    line: int
    macro: str  # 'write' or 'writeln'
    content: str
    num_interpolations: int
    estimated_size: int


@dataclass
class SearchIndexStats:
    """Statistics from search_index.json."""

    total_items: int
    items_by_kind: Dict[str, int]
    crate_name: Optional[str]


# =============================================================================
# Source Code Analysis
# =============================================================================


def get_write_calls() -> List[str]:
    """Get all write!/writeln! calls from grep output."""
    result = subprocess.run(
        ["grep", "-rn", r"write!\|writeln!", "src/", "--include=*.rs"],
        capture_output=True,
        text=True,
    )

    lines = result.stdout.strip().split("\n")

    # Filter out comments, imports, and use statements
    filtered = [
        line
        for line in lines
        if line
        and "//.*write" not in line
        and "std::fmt::Write" not in line
        and "use.*Write" not in line
        and ("write!" in line or "writeln!" in line)
    ]

    return filtered


def parse_call(line: str) -> Optional[WriteCall]:
    """Parse a grep output line into a WriteCall."""
    parts = line.split(":", 2)
    if len(parts) < 3:
        return None

    file_path, line_num, content = parts[0], int(parts[1]), parts[2].strip()
    macro = "writeln" if "writeln!" in content else "write"
    interpolations = len(re.findall(r"\{[^}]*\}", content))
    estimated_size = estimate_output_size(content, interpolations)

    return WriteCall(
        file=file_path,
        line=line_num,
        macro=macro,
        content=content[:80] + ("..." if len(content) > 80 else ""),
        num_interpolations=interpolations,
        estimated_size=estimated_size,
    )


def estimate_output_size(content: str, interpolations: int) -> int:
    """Estimate the output size of a write call in bytes."""
    match = re.search(r'write!?\s*\([^,]+,\s*"([^"]*)"', content)
    if match:
        fmt_str = match.group(1)
        base_size = len(fmt_str) - (interpolations * 2)

        # Estimate variable sizes based on context
        if "docs" in content.lower() or "process" in content.lower():
            var_size = 100  # Documentation strings are long
        elif "signature" in content.lower() or "generics" in content.lower():
            var_size = 40  # Type signatures are medium
        else:
            var_size = 15  # Names and anchors are short

        return max(base_size + (interpolations * var_size), 10)
    return 30  # Default estimate


def categorize_calls(calls: List[WriteCall]) -> Dict[str, List[WriteCall]]:
    """Categorize calls by complexity."""
    categories = {
        "literal_only": [],
        "single_var": [],
        "simple_format": [],
        "complex_format": [],
    }

    for call in calls:
        if call is None:
            continue
        if call.num_interpolations == 0:
            categories["literal_only"].append(call)
        elif call.num_interpolations == 1:
            if re.search(r'"\{[^}]+\}"', call.content):
                categories["single_var"].append(call)
            else:
                categories["simple_format"].append(call)
        elif call.num_interpolations <= 2:
            categories["simple_format"].append(call)
        else:
            categories["complex_format"].append(call)

    return categories


def calculate_base_overhead(calls: List[WriteCall]) -> Tuple[int, int, int]:
    """
    Calculate base memory overhead per "unit" of work.

    Returns: (allocations, bytes_overhead, copy_overhead)
    """
    total_allocs = 0
    total_bytes = 0
    total_copies = 0

    for call in calls:
        if call is None:
            continue
        if call.num_interpolations > 0:
            total_allocs += 1
            total_bytes += STRING_HEADER_BYTES + call.estimated_size
            total_copies += call.estimated_size

    return total_allocs, total_bytes, total_copies


# =============================================================================
# Search Index Analysis
# =============================================================================


def load_search_index(path: Path) -> Optional[SearchIndexStats]:
    """Load and analyze search_index.json."""
    if not path.exists():
        return None

    try:
        with open(path) as f:
            data = json.load(f)

        items = data.get("items", [])

        # Count by kind
        kind_counts: Dict[str, int] = {}
        for item in items:
            kind = item.get("kind", "unknown")
            kind_counts[kind] = kind_counts.get(kind, 0) + 1

        return SearchIndexStats(
            total_items=len(items),
            items_by_kind=kind_counts,
            crate_name=None,  # Could extract from path
        )
    except (json.JSONDecodeError, KeyError) as e:
        print(f"Warning: Could not parse search_index.json: {e}")
        return None


def estimate_write_calls_for_index(stats: SearchIndexStats) -> int:
    """Estimate total write! calls needed to generate docs for all items."""
    total = 0
    for kind, count in stats.items_by_kind.items():
        writes_per = WRITES_PER_ITEM.get(kind, 4)  # Default 4 for unknown
        total += count * writes_per
    return total


# =============================================================================
# Output Formatting
# =============================================================================


def format_bytes(b: int) -> str:
    """Format bytes in human-readable form."""
    if b < 1024:
        return f"{b} B"
    elif b < 1024 * 1024:
        return f"{b / 1024:.1f} KB"
    else:
        return f"{b / (1024 * 1024):.2f} MB"


def format_number(n: int) -> str:
    """Format large numbers with commas."""
    return f"{n:,}"


def print_section(title: str, width: int = 70):
    """Print a section header."""
    print()
    print("=" * width)
    print(title)
    print("=" * width)


def print_subsection(title: str, width: int = 50):
    """Print a subsection header."""
    print()
    print(title)
    print("-" * width)


# =============================================================================
# Main Analysis
# =============================================================================


def main():
    parser = argparse.ArgumentParser(
        description="Analyze write!/writeln! memory overhead"
    )
    parser.add_argument(
        "--search-index",
        "-s",
        type=Path,
        default=Path("generated_docs/search_index.json"),
        help="Path to search_index.json for real-world scaling",
    )
    args = parser.parse_args()

    print_section("WRITE!/WRITELN! MEMORY OVERHEAD ANALYSIS")

    # =========================================================================
    # Part 1: Source Code Analysis
    # =========================================================================

    print_subsection("SOURCE CODE ANALYSIS")

    raw_lines = get_write_calls()
    calls = [parse_call(line) for line in raw_lines]
    calls = [c for c in calls if c is not None]

    print(f"Total write!/writeln! calls in codebase: {len(calls)}")

    categories = categorize_calls(calls)
    print()
    print("Call categories:")
    print(f"  Literal only (no interpolation):  {len(categories['literal_only']):>4}")
    print(f"  Single variable:                  {len(categories['single_var']):>4}")
    print(f"  Simple format (1-2 vars):         {len(categories['simple_format']):>4}")
    print(f"  Complex format (3+ vars):         {len(categories['complex_format']):>4}")

    # Base overhead calculation
    base_allocs, base_bytes, base_copies = calculate_base_overhead(calls)

    print()
    print("Calls with interpolation (would allocate with format!):")
    print(f"  Count: {base_allocs}")
    print(f"  Avg estimated output size: {base_copies // max(base_allocs, 1)} bytes")

    # =========================================================================
    # Part 2: Search Index Analysis
    # =========================================================================

    search_stats = load_search_index(args.search_index)

    if search_stats:
        print_subsection(f"SEARCH INDEX ANALYSIS ({args.search_index})")

        print(f"Total documented items: {format_number(search_stats.total_items)}")
        print()
        print("Items by kind:")

        sorted_kinds = sorted(search_stats.items_by_kind.items(), key=lambda x: -x[1])
        for kind, count in sorted_kinds:
            writes = WRITES_PER_ITEM.get(kind, 4)
            print(
                f"  {kind:8} {count:>6}  (~{writes} writes each = {count * writes:>6} calls)"
            )

        estimated_calls = estimate_write_calls_for_index(search_stats)
        print()
        print(
            f"Estimated total write! calls for this index: {format_number(estimated_calls)}"
        )

    # =========================================================================
    # Part 3: Theoretical Overhead Calculation
    # =========================================================================

    print_subsection("THEORETICAL OVERHEAD (if using push_str + format!)")

    # Calculate scaling factor
    if search_stats:
        # Use actual data: estimated calls / base calls in code
        # This represents how many times each code path executes
        scaling_factor = estimate_write_calls_for_index(search_stats) / max(
            len(calls), 1
        )
        scaling_source = "search_index.json"
    else:
        scaling_factor = 1.0
        scaling_source = "single pass (no search_index found)"

    # Scale the overhead
    scaled_allocs = int(base_allocs * scaling_factor)
    scaled_bytes = int(base_bytes * scaling_factor)
    scaled_copies = int(base_copies * scaling_factor)
    total_overhead = scaled_bytes + scaled_copies

    print(f"Scaling factor: {scaling_factor:.1f}x (based on {scaling_source})")
    print()
    print("Memory overhead avoided by using write!:")
    print(f"  Temporary String allocations:  {format_number(scaled_allocs)}")
    print(
        f"  String header overhead:        {format_bytes(scaled_allocs * STRING_HEADER_BYTES)}"
    )
    print(
        f"  Content allocation overhead:   {format_bytes(scaled_bytes - scaled_allocs * STRING_HEADER_BYTES)}"
    )
    print(f"  Redundant memcpy overhead:     {format_bytes(scaled_copies)}")
    print("  ─────────────────────────────────────────")
    print(f"  TOTAL MEMORY SAVED:            {format_bytes(total_overhead)}")

    # =========================================================================
    # Part 4: CPU Overhead Calculation
    # =========================================================================

    print_subsection("CPU OVERHEAD (if using push_str + format!)")

    alloc_cycles = scaled_allocs * ALLOC_CYCLES
    dealloc_cycles = scaled_allocs * DEALLOC_CYCLES
    copy_cycles = int(scaled_copies * COPY_CYCLES_PER_BYTE)
    total_cycles = alloc_cycles + dealloc_cycles + copy_cycles

    time_ms = total_cycles / (CPU_FREQ_GHZ * 1e9) * 1000
    time_us = time_ms * 1000

    print(f"CPU cycles avoided (at {CPU_FREQ_GHZ} GHz reference):")
    print(f"  malloc() calls:      {format_number(alloc_cycles):>15} cycles")
    print(f"  free() calls:        {format_number(dealloc_cycles):>15} cycles")
    print(f"  Extra memcpy:        {format_number(copy_cycles):>15} cycles")
    print("  ─────────────────────────────────────────")
    print(f"  TOTAL CYCLES SAVED:  {format_number(total_cycles):>15} cycles")
    print()
    print(f"  Time equivalent:     {time_ms:.3f} ms ({time_us:.1f} µs)")

    # =========================================================================
    # Part 5: File Hotspots
    # =========================================================================

    print_subsection("TOP FILES BY WRITE! CALL COUNT")

    file_counts: Dict[str, int] = {}
    for call in calls:
        file_counts[call.file] = file_counts.get(call.file, 0) + 1

    sorted_files = sorted(file_counts.items(), key=lambda x: -x[1])
    for file, count in sorted_files[:10]:
        pct = count / len(calls) * 100
        print(f"  {file:45} {count:>4} ({pct:>5.1f}%)")

    # =========================================================================
    # Part 6: Summary
    # =========================================================================

    print_section("SUMMARY")

    if search_stats:
        items_str = format_number(search_stats.total_items)
    else:
        items_str = "unknown"

    print(f"""
Documentation scope:
  - Documented items:        {items_str}
  - write!/writeln! calls:   {len(calls)} (in source)
  - Estimated runtime calls: {format_number(int(len(calls) * scaling_factor))}

By using write!/writeln! instead of push_str + format!, this run avoids:

  - {format_number(scaled_allocs)} temporary String allocations
  - {format_bytes(total_overhead)} of memory overhead
  - {format_number(total_cycles)} CPU cycles (~{time_ms:.2f} ms)

Secondary benefits:
  - Reduced heap fragmentation
  - Better cache locality (sequential writes)
  - Lower allocator contention in parallel builds
  - Consistent memory profile across runs
""")


if __name__ == "__main__":
    main()
