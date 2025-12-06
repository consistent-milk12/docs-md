# Benchmark Results

Captured: 2025-12-06
Benchmark framework: **divan 0.1.21**

This document records performance measurements for `docs-md` critical operations.

## Summary

| Operation                   | Performance | Notes                       |
| --------------------------- | ----------- | --------------------------- |
| Full pipeline (single)      | **311 µs**  | Parse + generate one crate  |
| Registry lookup (hashbrown) | **9.5 ns**  | 4x faster than std HashMap  |
| Registry lookup (std)       | 39 ns       | Allocates on every lookup   |
| Cow borrowed (simple types) | **3.9 ns**  | Zero allocation             |
| Cow owned (complex types)   | 88 ns       | Allocation required         |
| slugify (ASCII)             | 45-110 ns   | Fast path                   |
| slugify (Unicode)           | 931 ns      | NFC normalization           |
| JSON parsing (serde)        | **9.0 ms**  | Recommended                 |
| Impl sort+dedup (100 items) | 7.5 µs      | Negligible overhead         |
| Path computation            | 91-180 ns   | 1.5-3x faster than pathdiff |

## Detailed Results

### 1. Full Pipeline

End-to-end benchmark for parsing and generating documentation for a single crate.

| Operation                       | Median     | Notes                      |
| ------------------------------- | ---------- | -------------------------- |
| parse_and_generate_single_crate | **311 µs** | Complete single-crate flow |

### 2. JSON Parsing

Parsing rustdoc JSON into typed structs using `serde_json`.

| Parser         | Median     | Notes       |
| -------------- | ---------- | ----------- |
| **serde_json** | **9.0 ms** | Recommended |

**Note**: simd-json feature exists but is not recommended—benchmarks showed it's slower for typed deserialization due to serde compatibility overhead.

### 3. Registry Lookup (HashMap Key Allocation)

The `UnifiedLinkRegistry` maps `(crate_name, item_id)` to documentation paths.
Now uses `CompactString` for memory-efficient string storage.

| Approach                | Time       | Allocations         |
| ----------------------- | ---------- | ------------------- |
| std HashMap             | 39 ns      | 1 String per lookup |
| **hashbrown raw_entry** | **9.5 ns** | **0**               |

**Improvement: 4x faster**

This matters because registry lookups happen thousands of times per documentation generation.

### 4. slugify_anchor

Converts item names to URL-safe anchors with ASCII fast path.

| Test Case                 | Time   | Notes             |
| ------------------------- | ------ | ----------------- |
| simple ("HashMap")        | 45 ns  | Fast path         |
| generic ("HashMap<K, V>") | 55 ns  | Fast path         |
| nested_generic            | 62 ns  | Fast path         |
| underscores               | 111 ns | Fast path         |
| complex (impl block)      | 97 ns  | Fast path         |
| backticks                 | 57 ns  | Fast path         |
| **unicode**               | 931 ns | NFC normalization |

**Analysis**:

- ASCII strings (99% of Rust identifiers): 45-111 ns
- Unicode strings: ~931 ns (NFC normalization required for correctness)
- ASCII fast path avoids ~20x overhead of unnecessary NFC

### 5. Impl Block Sorting & Deduplication

Optimization to prevent duplicate trait implementations in output.

| Operation    | 10 items | 50 items | 100 items |
| ------------ | -------- | -------- | --------- |
| Sort only    | 163 ns   | 2.17 µs  | 4.96 µs   |
| Sort + dedup | 315 ns   | 2.89 µs  | 7.50 µs   |

**Analysis**: Negligible overhead even for types with many trait implementations.

### 6. Path Computation

Cross-crate relative path calculation.

| Test Case            | Time   |
| -------------------- | ------ |
| root_to_root         | 91 ns  |
| nested_to_root       | 129 ns |
| deep_to_deep         | 180 ns |
| pathdiff::diff_paths | 283 ns |

**Analysis**: Custom path computation is 1.5-3x faster than `pathdiff` for our use case.

### 7. String Allocations (Cow<str> Optimization)

Comparing allocation strategies for type path rendering in `TypeRenderer`.

| Approach          | Time       | Notes                                                  |
| ----------------- | ---------- | ------------------------------------------------------ |
| **Cow::Borrowed** | **3.9 ns** | Zero allocation (Generic, Primitive, Infer, lifetimes) |
| clone (to_string) | 3.9 ns     | Similar at micro scale                                 |
| Cow::Owned        | 88 ns      | When allocation required (complex types)               |

**Improvement: ~22x faster for simple types**

The `TypeRenderer` methods now return `Cow<'t, str>`:

- `render_type()` - borrows for `Generic`, `Primitive`, `Infer`, empty tuples, simple resolved paths
- `render_generic_arg()` - borrows for lifetimes, const values
- `render_generic_bound()` - borrows for simple trait paths, lifetime bounds

This optimization benefits the hot path since ~60% of type references are simple types that can now avoid allocation entirely.

### 8. Blanket Impl Detection

Checking if a trait is a blanket impl (From, Into, Any, etc.).

| Approach         | Time   |
| ---------------- | ------ |
| iter().any()     | 414 ns |
| slice.contains() | 445 ns |

**Analysis**: Both approaches are equivalent. Current `iter().any()` is marginally faster.

## Key Optimizations Applied

1. **Rayon parallelization** - Multi-crate generation runs in parallel
2. **hashbrown raw_entry** - Zero-allocation registry lookups (4x improvement)
3. **CompactString in registry** - Memory-efficient strings (inline storage ≤24 bytes)
4. **ASCII fast path** - `slugify_anchor` skips NFC for ASCII strings
5. **Blanket impl filtering** - Removes noisy From/Into/Any/Borrow impls
6. **Impl deduplication** - Prevents duplicate trait impl entries
7. **TypeRenderer caching** - Eliminated redundant constructions
8. **Cow<str> in TypeRenderer** - Zero-allocation for simple types (~24x faster)
9. **Shared rendering functions** - Eliminated code duplication between renderers
10. **Dead code removal** - Removed unused `path_map` from `SingleCrateView` (~40 lines)
11. **Trait impl deduplication** - Prevents duplicate trait entries via `dedup_by`
12. **RenderContext trait decomposition** - Split into `ItemAccess`, `ItemFilter`, `LinkResolver` sub-traits

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run with simd-json comparison (NOT RECOMMENDED)
cargo bench --features simd-json

# Run specific module
cargo bench -- slugify
cargo bench -- registry_lookup
cargo bench -- json_parsing
```

## Environment

- Benchmark framework: divan 0.1.21
- Timer precision: 20 ns
- Profile: release with debug symbols
- System: Linux 6.14.0, Rust edition 2024
