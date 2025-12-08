//! Benchmarks for docs-md performance-critical operations.
//!
//! Run with: `cargo bench`
//!
//! For simd-json benchmarks: `cargo bench --features simd-json`
//!
//! These benchmarks measure:
//! - JSON parsing (standard vs simd-json)
//! - `slugify_anchor`: Anchor slug generation (ASCII fast path vs Unicode)
//! - Registry lookups: Zero-allocation hashbrown `raw_entry` API
//! - Type rendering: String allocations
//! - Path computation: Cross-crate link resolution

use std::hint::black_box;

fn main() {
    divan::main();
}

// =============================================================================
// JSON Parsing Benchmarks
// =============================================================================

mod json_parsing {
    use super::black_box;

    /// Get a sample JSON file path for benchmarking
    fn get_sample_json() -> Option<std::path::PathBuf> {
        // Try to find a medium-sized JSON file in target/doc
        let doc_dir = std::path::Path::new("target/doc");
        if !doc_dir.exists() {
            return None;
        }

        // Look for a reasonably sized crate JSON
        let candidates = ["serde_json.json", "regex.json", "http.json", "ureq.json"];
        for name in candidates {
            let path = doc_dir.join(name);
            if path.exists() {
                return Some(path);
            }
        }

        // Fall back to any JSON file
        std::fs::read_dir(doc_dir)
            .ok()?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .find(|p| p.extension().is_some_and(|e| e == "json"))
    }

    #[divan::bench]
    fn parse_json_serde(bencher: divan::Bencher) {
        let Some(path) = get_sample_json() else {
            eprintln!("No JSON file found for benchmarking. Run `cargo doc` first.");
            return;
        };

        let content = std::fs::read_to_string(&path).unwrap();

        bencher
            .with_inputs(|| content.clone())
            .bench_values(|json| {
                let _: rustdoc_types::Crate = serde_json::from_str(black_box(&json)).unwrap();
            });
    }

    #[cfg(feature = "simd-json")]
    #[divan::bench]
    fn parse_json_simd(bencher: divan::Bencher) {
        let Some(path) = get_sample_json() else {
            eprintln!("No JSON file found for benchmarking. Run `cargo doc` first.");
            return;
        };

        let content = std::fs::read_to_string(&path).unwrap();

        bencher
            .with_inputs(|| content.clone().into_bytes())
            .bench_values(|mut bytes| {
                let _: rustdoc_types::Crate = simd_json::from_slice(black_box(&mut bytes)).unwrap();
            });
    }
}

// =============================================================================
// slugify_anchor Benchmarks
// =============================================================================

mod slugify {
    use cargo_docs_md::slugify_anchor;

    use super::black_box;

    // ASCII inputs (use fast path)
    const ASCII_SIMPLE: &str = "HashMap";
    const ASCII_GENERIC: &str = "HashMap<K, V>";
    const ASCII_NESTED: &str = "Result<Option<Vec<T>>, Error>";
    const ASCII_UNDERSCORES: &str = "my_long_function_name_with_many_parts";
    const ASCII_COMPLEX: &str = "impl<'a, T: Clone + Send> Iterator<Item = &'a T>";
    const ASCII_BACKTICKS: &str = "`ConfigBuilder`";

    // Unicode input (uses slow path with NFC normalization)
    const UNICODE: &str = "Größe_café_naïve_日本語";

    #[divan::bench]
    fn ascii_simple() -> String {
        slugify_anchor(black_box(ASCII_SIMPLE))
    }

    #[divan::bench]
    fn ascii_generic() -> String {
        slugify_anchor(black_box(ASCII_GENERIC))
    }

    #[divan::bench]
    fn ascii_nested_generic() -> String {
        slugify_anchor(black_box(ASCII_NESTED))
    }

    #[divan::bench]
    fn ascii_underscores() -> String {
        slugify_anchor(black_box(ASCII_UNDERSCORES))
    }

    #[divan::bench]
    fn ascii_complex() -> String {
        slugify_anchor(black_box(ASCII_COMPLEX))
    }

    #[divan::bench]
    fn ascii_backticks() -> String {
        slugify_anchor(black_box(ASCII_BACKTICKS))
    }

    #[divan::bench]
    fn unicode_nfc() -> String {
        slugify_anchor(black_box(UNICODE))
    }
}

// =============================================================================
// Registry Lookup Benchmarks (hashbrown raw_entry vs std HashMap)
// =============================================================================

mod registry_lookup {
    use std::collections::HashMap as StdHashMap;
    use std::hash::Hash;

    use super::black_box;

    /// Borrowed key for zero-allocation lookups
    #[derive(Hash, PartialEq, Eq)]
    struct BorrowedKey<'a>(&'a str, u32);

    /// Setup maps with realistic data
    #[expect(clippy::type_complexity)]
    fn setup_maps() -> (
        StdHashMap<(String, u32), String>,
        hashbrown::HashMap<(String, u32), String>,
    ) {
        let mut std_map = StdHashMap::new();
        let mut hb_map = hashbrown::HashMap::new();

        let crate_names = [
            "ureq", "http", "rustls", "serde", "tokio", "tracing", "regex", "clap",
        ];
        for crate_name in crate_names {
            for id in 0..500u32 {
                let path = format!("{crate_name}/module{}/index.md", id % 20);
                std_map.insert((crate_name.to_string(), id), path.clone());
                hb_map.insert((crate_name.to_string(), id), path);
            }
        }

        (std_map, hb_map)
    }

    #[divan::bench]
    fn std_hashmap_alloc_lookup(bencher: divan::Bencher) {
        let (std_map, _) = setup_maps();

        bencher.bench_local(|| {
            // This allocates a new String for the key on every lookup
            let key = (black_box("http").to_string(), black_box(250u32));
            std_map.get(&key)
        });
    }

    #[divan::bench]
    fn hashbrown_raw_entry_zero_alloc(bencher: divan::Bencher) {
        use std::hash::BuildHasher;

        let (_, hb_map) = setup_maps();

        bencher.bench_local(|| {
            // Zero allocation - uses borrowed key with pre-computed hash
            let borrowed = BorrowedKey(black_box("http"), black_box(250u32));
            let hash = hb_map.hasher().hash_one(&borrowed);

            hb_map
                .raw_entry()
                .from_hash(hash, |k| k.0 == borrowed.0 && k.1 == borrowed.1)
                .map(|(_, v)| v)
        });
    }
}

// =============================================================================
// String Allocation Benchmarks (Clone vs Cow)
// =============================================================================

mod string_alloc {
    use std::borrow::Cow;

    use super::black_box;

    const TYPE_PATHS: &[&str] = &[
        "std::collections::HashMap",
        "Vec",
        "Option",
        "Result<T, E>",
        "Box<dyn Iterator<Item = String>>",
        "Arc<Mutex<Vec<String>>>",
        "&'static str",
        "impl Future<Output = Result<(), Error>>",
    ];

    #[divan::bench]
    fn clone_strings(bencher: divan::Bencher) {
        bencher.bench_local(|| {
            for path in TYPE_PATHS {
                let _cloned: String = black_box(*path).to_string();
            }
        });
    }

    #[divan::bench]
    fn cow_borrowed(bencher: divan::Bencher) {
        bencher.bench_local(|| {
            for path in TYPE_PATHS {
                let _cow: Cow<'_, str> = Cow::Borrowed(black_box(*path));
            }
        });
    }

    #[divan::bench]
    fn cow_owned_when_needed(bencher: divan::Bencher) {
        bencher.bench_local(|| {
            for path in TYPE_PATHS {
                // Simulate: only allocate if we need to modify
                let cow: Cow<'_, str> = if path.contains('<') {
                    // Complex type - would need processing
                    Cow::Owned(black_box(*path).to_string())
                } else {
                    // Simple type - borrow
                    Cow::Borrowed(black_box(*path))
                };
                black_box(cow);
            }
        });
    }
}

// =============================================================================
// Path Computation Benchmarks
// =============================================================================

mod path_computation {
    use super::black_box;

    #[divan::bench(args = [
        ("root_to_root", "index.md", "http", "index.md"),
        ("nested_to_root", "agent/config/index.md", "http", "index.md"),
        ("deep_to_deep", "a/b/c/d/index.md", "other", "x/y/z/index.md"),
    ])]
    fn cross_crate_path(bencher: divan::Bencher, args: (&str, &str, &str, &str)) {
        let (_name, current, target_crate, target_path) = args;

        bencher.bench_local(|| {
            let depth = black_box(current).matches('/').count();
            let prefix = "../".repeat(depth + 1);
            format!(
                "{}{}/{}",
                prefix,
                black_box(target_crate),
                black_box(target_path)
            )
        });
    }

    #[divan::bench]
    fn pathdiff_relative(bencher: divan::Bencher) {
        use std::path::Path;

        let from = Path::new("ureq/agent/config/index.md");
        let to = Path::new("http/response/index.md");

        bencher
            .bench_local(|| pathdiff::diff_paths(black_box(to), black_box(from.parent().unwrap())));
    }
}

// =============================================================================
// Impl Block Sorting Benchmarks
// =============================================================================

mod impl_sorting {

    /// Simulated impl sort keys (like what we generate for deduplication)
    fn generate_impl_keys(count: usize) -> Vec<String> {
        let traits = [
            "Debug", "Clone", "Default", "Display", "From", "Into", "Hash", "Eq",
        ];
        let types = ["MyStruct", "Config", "Builder", "Error", "Response"];

        let mut keys = Vec::with_capacity(count);
        for i in 0..count {
            let trait_name = traits[i % traits.len()];
            let type_name = types[i % types.len()];
            keys.push(format!("{trait_name}::{type_name}"));
        }
        // Shuffle to simulate unsorted input
        keys.reverse();
        keys
    }

    #[divan::bench(args = [10, 50, 100])]
    fn sort_impl_keys(bencher: divan::Bencher, count: usize) {
        bencher
            .with_inputs(|| generate_impl_keys(count))
            .bench_values(|mut keys| {
                keys.sort();
                keys
            });
    }

    #[divan::bench(args = [10, 50, 100])]
    fn sort_and_dedup_impl_keys(bencher: divan::Bencher, count: usize) {
        bencher
            .with_inputs(|| {
                let mut keys = generate_impl_keys(count);
                // Add some duplicates
                let len = keys.len();
                for i in 0..len / 4 {
                    keys.push(keys[i].clone());
                }
                keys
            })
            .bench_values(|mut keys| {
                keys.sort();
                keys.dedup();
                keys
            });
    }
}

// =============================================================================
// Blanket Impl Detection Benchmarks
// =============================================================================

mod blanket_impl {

    const BLANKET_TRAITS: &[&str] = &[
        "From",
        "Into",
        "TryFrom",
        "TryInto",
        "Any",
        "Borrow",
        "BorrowMut",
        "ToOwned",
        "CloneToUninit",
    ];

    const TEST_TRAITS: &[&str] = &[
        "core::convert::From",
        "std::convert::Into",
        "Debug",
        "Clone",
        "std::any::Any",
        "Display",
        "core::borrow::Borrow",
        "Iterator",
        "Default",
        "std::borrow::BorrowMut",
    ];

    #[divan::bench]
    fn check_blanket_trait_contains(bencher: divan::Bencher) {
        bencher.bench_local(|| {
            for trait_path in TEST_TRAITS {
                let trait_name = trait_path.split("::").last().unwrap_or(trait_path);
                let _is_blanket = BLANKET_TRAITS.contains(&trait_name);
            }
        });
    }

    #[divan::bench]
    fn check_blanket_trait_any(bencher: divan::Bencher) {
        bencher.bench_local(|| {
            for trait_path in TEST_TRAITS {
                let trait_name = trait_path.split("::").last().unwrap_or(trait_path);
                let _is_blanket = BLANKET_TRAITS.contains(&trait_name);
            }
        });
    }
}

// =============================================================================
// Full Pipeline Benchmarks (if JSON available)
// =============================================================================

mod full_pipeline {
    use super::black_box;

    #[divan::bench(sample_count = 10)]
    fn parse_and_generate_single_crate(bencher: divan::Bencher) {
        // Find a small crate JSON for testing
        let doc_dir = std::path::Path::new("target/doc");
        let json_path = doc_dir.join("cfg_if.json");

        if !json_path.exists() {
            eprintln!("cfg_if.json not found. Run `cargo doc` first.");
            return;
        }

        let content = std::fs::read_to_string(&json_path).unwrap();

        bencher
            .with_inputs(|| content.clone())
            .bench_values(|json| {
                let krate: rustdoc_types::Crate = serde_json::from_str(&json).unwrap();
                // Just parse - don't write to disk
                black_box(krate)
            });
    }
}
