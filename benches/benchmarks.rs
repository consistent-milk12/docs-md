//! Benchmarks for docs-md performance-critical operations.
//!
//! Run with: `cargo bench`
//!
//! These benchmarks establish baselines for:
//! - `slugify_anchor`: Anchor slug generation
//! - Registry lookups: `HashMap` operations in `UnifiedLinkRegistry`
//! - Type rendering: String allocations in `TypeRenderer`

use std::collections::HashMap;
use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
// Import the actual implementation
use docs_md::slugify_anchor;

// =============================================================================
// slugify_anchor benchmarks
// =============================================================================

/// ASCII-only implementation (no unicode normalization) for comparison
fn slugify_anchor_ascii_only(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut in_generics = 0;
    let mut last_was_hyphen = true;

    for ch in name.chars() {
        match ch {
            '`' => {},
            '<' => in_generics += 1,
            '>' => {
                if in_generics > 0 {
                    in_generics -= 1;
                }
            },
            _ if in_generics == 0 => {
                if ch.is_alphanumeric() {
                    result.push(ch.to_ascii_lowercase());
                    last_was_hyphen = false;
                } else if (ch == ' ' || ch == '_' || ch == '-') && !last_was_hyphen {
                    result.push('-');
                    last_was_hyphen = true;
                }
            },
            _ => {},
        }
    }

    if result.ends_with('-') {
        result.pop();
    }

    result
}

fn bench_slugify_anchor(c: &mut Criterion) {
    let mut group = c.benchmark_group("slugify_anchor");

    // ASCII test cases (will use fast path)
    let ascii_cases = [
        ("simple", "HashMap"),
        ("generic", "HashMap<K, V>"),
        ("nested_generic", "Result<Option<Vec<T>>, Error>"),
        ("with_underscores", "my_long_function_name"),
        ("with_spaces", "Some Function Name"),
        (
            "complex",
            "impl<'a, T: Clone + Send> Iterator<Item = &'a T>",
        ),
        ("backticks", "`ConfigBuilder`"),
    ];

    // Unicode test case (will use slow path with NFC)
    let unicode_case = ("unicode", "Größe_café_naïve");

    // Benchmark ASCII cases - compare baseline vs optimized
    for (name, input) in ascii_cases {
        group.throughput(Throughput::Bytes(input.len() as u64));

        // Baseline: ASCII-only implementation
        group.bench_with_input(BenchmarkId::new("baseline", name), &input, |b, input| {
            b.iter(|| slugify_anchor_ascii_only(black_box(input)));
        });

        // Optimized: Current implementation with fast ASCII path
        group.bench_with_input(BenchmarkId::new("optimized", name), &input, |b, input| {
            b.iter(|| slugify_anchor(black_box(input)));
        });
    }

    // Benchmark Unicode case separately
    let (name, input) = unicode_case;
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_with_input(BenchmarkId::new("baseline", name), &input, |b, input| {
        b.iter(|| slugify_anchor_ascii_only(black_box(input)));
    });

    group.bench_with_input(BenchmarkId::new("optimized", name), &input, |b, input| {
        b.iter(|| slugify_anchor(black_box(input)));
    });

    group.finish();
}

// =============================================================================
// HashMap key allocation benchmarks (simulating registry lookups)
// =============================================================================

/// Borrowed key for raw_entry lookup (mirrors the actual implementation)
#[derive(Hash, PartialEq, Eq)]
struct BorrowedKey<'a>(&'a str, u32);

fn bench_hashmap_lookups(c: &mut Criterion) {
    use std::hash::BuildHasher;

    let mut group = c.benchmark_group("registry_lookup");

    // Old approach: std HashMap with String keys (allocates on every lookup)
    let mut std_map: HashMap<(String, u32), String> = HashMap::new();

    // New approach: hashbrown with raw_entry (zero-allocation lookups)
    let mut hb_map: hashbrown::HashMap<(String, u32), String> = hashbrown::HashMap::new();

    // Populate with realistic data
    let crate_names = ["ureq", "http", "rustls", "serde", "tokio", "tracing"];
    for crate_name in crate_names {
        for id in 0..1000u32 {
            let path = format!("{crate_name}/module{}/index.md", id % 10);
            std_map.insert((crate_name.to_string(), id), path.clone());
            hb_map.insert((crate_name.to_string(), id), path);
        }
    }

    // Benchmark: Old approach (allocates String for key on every lookup)
    group.bench_function("std_alloc_lookup", |b| {
        let crate_name = "http";
        let id = 500u32;
        b.iter(|| {
            // This allocates a new String every time
            let key = (black_box(crate_name).to_string(), black_box(id));
            std_map.get(&key)
        });
    });

    // Benchmark: New approach (raw_entry with borrowed key, zero allocation)
    group.bench_function("hashbrown_raw_entry", |b| {
        let crate_name = "http";
        let id = 500u32;
        b.iter(|| {
            let borrowed = BorrowedKey(black_box(crate_name), black_box(id));
            let hash = hb_map.hasher().hash_one(&borrowed);
            hb_map
                .raw_entry()
                .from_hash(hash, |k| k.0 == borrowed.0 && k.1 == borrowed.1)
                .map(|(_, v)| v)
        });
    });

    group.finish();
}

// =============================================================================
// String allocation benchmarks
// =============================================================================

fn bench_string_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_alloc");

    // Simulate type path cloning (current behavior in TypeRenderer)
    let type_paths = [
        "std::collections::HashMap",
        "Vec",
        "Option",
        "Result<T, E>",
        "Box<dyn Iterator<Item = String>>",
    ];

    group.bench_function("clone_path", |b| {
        b.iter(|| {
            for path in &type_paths {
                let _cloned: String = black_box(*path).to_string();
            }
        });
    });

    // Simulate with Cow (would avoid allocation for borrowed case)
    group.bench_function("cow_borrowed", |b| {
        use std::borrow::Cow;
        b.iter(|| {
            for path in &type_paths {
                let _cow: Cow<'_, str> = Cow::Borrowed(black_box(*path));
            }
        });
    });

    group.finish();
}

// =============================================================================
// Path computation benchmarks
// =============================================================================

fn bench_path_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_computation");

    // Simulate cross-crate path computation
    let test_cases = [
        ("root_to_root", "index.md", "http", "index.md"),
        ("nested_to_root", "agent/index.md", "http", "index.md"),
        ("deep_nested", "a/b/c/index.md", "other", "x/y/index.md"),
    ];

    for (name, current, target_crate, target_path) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("cross_crate_path", name),
            &(current, target_crate, target_path),
            |b, (current, target_crate, target_path)| {
                b.iter(|| {
                    let depth = black_box(*current).matches('/').count();
                    let prefix = "../".repeat(depth + 1);
                    format!(
                        "{}{}/{}",
                        prefix,
                        black_box(*target_crate),
                        black_box(*target_path)
                    )
                });
            },
        );
    }

    group.finish();
}

// =============================================================================
// Criterion setup
// =============================================================================

criterion_group!(
    benches,
    bench_slugify_anchor,
    bench_hashmap_lookups,
    bench_string_allocations,
    bench_path_computation,
);

criterion_main!(benches);
