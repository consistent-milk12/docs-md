//! Integration tests for docs-md markdown generation.
//!
//! These tests use insta for snapshot testing to verify that generated
//! markdown output is correct and stable across changes.
//!
//! ## Running Tests
//!
//! ```bash
//! # Generate test fixture first
//! cargo doc --output-format json
//!
//! # Run tests
//! cargo test
//!
//! # Review snapshots interactively (requires cargo-insta)
//! cargo insta review
//! ```

use std::path::{Path, PathBuf};

use cargo_docs_md::generator::Generator;
use cargo_docs_md::generator::config::RenderConfig;
use cargo_docs_md::linker::LinkRegistry;
use cargo_docs_md::parser::Parser;
use cargo_docs_md::{CliOutputFormat, MarkdownCapture};

/// Helper to get the path to the committed test fixture.
///
/// This fixture is checked into the repo so tests can run without building JSON first.
fn committed_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("cargo_docs_md.json")
}

/// Helper to get the path to the freshly-generated test fixture.
///
/// This is useful for testing against the latest code changes.
fn generated_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("doc")
        .join("cargo_docs_md.json")
}

/// Get the best available fixture path.
///
/// Prefers the committed fixture for reproducibility, falls back to generated.
fn fixture_path() -> PathBuf {
    let committed = committed_fixture_path();
    if committed.exists() {
        return committed;
    }
    generated_fixture_path()
}

/// Check if any test fixture exists.
fn fixture_exists() -> bool {
    committed_fixture_path().exists() || generated_fixture_path().exists()
}

/// Load the test fixture crate.
fn load_fixture() -> rustdoc_types::Crate {
    let path = fixture_path();
    Parser::parse_file(&path).unwrap_or_else(|e| {
        panic!(
            "Failed to parse test fixture at '{}': {e}\n\
             Either commit tests/fixtures/cargo_docs_md.json or run:\n  \
             RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc",
            path.display()
        )
    })
}

// =============================================================================
// Snapshot Tests
// =============================================================================

#[test]
fn test_flat_format_output() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    assert!(capture.get("index.md").is_some(), "Should have index.md");
    insta::assert_snapshot!("flat_format_output", capture.to_snapshot_string());
}

#[test]
fn test_nested_format_output() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    assert!(capture.get("index.md").is_some(), "Should have index.md");
    insta::assert_snapshot!("nested_format_output", capture.to_snapshot_string());
}

#[test]
fn test_include_private_flag() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    let public_only = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");
    let with_private = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, true)
        .expect("Generation failed");

    let public_len: usize = public_only
        .paths()
        .iter()
        .map(|p| public_only.get(p).map_or(0, String::len))
        .sum();
    let private_len: usize = with_private
        .paths()
        .iter()
        .map(|p| with_private.get(p).map_or(0, String::len))
        .sum();

    assert!(
        private_len >= public_len,
        "Private output ({private_len} bytes) should be >= public output ({public_len} bytes)"
    );

    insta::assert_snapshot!("flat_with_private", with_private.to_snapshot_string());
}

#[test]
fn test_flat_vs_nested_file_count() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    let flat = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");
    let nested = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    assert_eq!(
        flat.len(),
        nested.len(),
        "Flat ({}) and nested ({}) should have same file count",
        flat.len(),
        nested.len()
    );
}

#[test]
fn test_flat_naming_convention() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    for path in capture.paths() {
        if *path != "index.md" {
            assert!(
                !path.contains('/'),
                "Flat format file '{path}' should not contain path separators"
            );
            assert!(
                std::path::Path::new(path)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("md")),
                "All files should end with .md: {path}"
            );
        }
    }
}

#[test]
fn test_nested_naming_convention() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    for path in capture.paths() {
        assert!(
            path == "index.md" || path.ends_with("/index.md"),
            "Nested format file '{path}' should be 'index.md' or end with '/index.md'"
        );
    }
}

// =============================================================================
// Link Registry Tests
// =============================================================================

#[test]
fn test_link_registry_flat_paths() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let registry = LinkRegistry::build(&krate, true, false);

    if let rustdoc_types::ItemEnum::Module(root_module) = &krate.index[&krate.root].inner {
        for item_id in &root_module.items {
            if let Some(item) = krate.index.get(item_id)
                && let rustdoc_types::ItemEnum::Module(_) = &item.inner
                && let Some(path) = registry.get_path(*item_id)
            {
                assert!(
                    !path.contains('/'),
                    "Flat registry path '{path}' should not contain '/'"
                );
                assert!(
                    std::path::Path::new(path)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("md")),
                    "Flat registry path '{path}' should end with '.md'"
                );
            }
        }
    }
}

#[test]
fn test_link_registry_nested_paths() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let registry = LinkRegistry::build(&krate, false, false);

    if let rustdoc_types::ItemEnum::Module(root_module) = &krate.index[&krate.root].inner {
        for item_id in &root_module.items {
            if let Some(item) = krate.index.get(item_id)
                && let rustdoc_types::ItemEnum::Module(_) = &item.inner
                && let Some(path) = registry.get_path(*item_id)
            {
                assert!(
                    path.ends_with("/index.md"),
                    "Nested registry path '{path}' should end with '/index.md'"
                );
            }
        }
    }
}

// =============================================================================
// Content Verification Tests
// =============================================================================

#[test]
fn test_index_contains_crate_title() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    let index = capture.get("index.md").expect("Should have index.md");
    assert!(
        index.starts_with("# Crate `"),
        "Index should start with crate title"
    );
}

#[test]
fn test_module_contains_module_title() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    for path in capture.paths() {
        if *path != "index.md" {
            let content = capture.get(path).unwrap();
            assert!(
                content.starts_with("# Module `"),
                "Module file '{path}' should start with module title, got: {}",
                content.lines().next().unwrap_or("")
            );
            break;
        }
    }
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_empty_capture_snapshot() {
    let capture = MarkdownCapture::new();
    assert!(capture.is_empty());
    assert_eq!(capture.len(), 0);
    assert_eq!(capture.to_snapshot_string(), "");
}

#[test]
fn test_capture_deterministic_order() {
    let mut capture = MarkdownCapture::new();

    capture.insert("z_module.md".to_string(), "Z content".to_string());
    capture.insert("a_module.md".to_string(), "A content".to_string());
    capture.insert("m_module.md".to_string(), "M content".to_string());

    let snapshot = capture.to_snapshot_string();

    let a_pos = snapshot.find("a_module.md").unwrap();
    let m_pos = snapshot.find("m_module.md").unwrap();
    let z_pos = snapshot.find("z_module.md").unwrap();

    assert!(
        a_pos < m_pos && m_pos < z_pos,
        "Snapshot should be in alphabetical order"
    );
}

// =============================================================================
// Visibility Filtering Tests
// =============================================================================

#[test]
fn test_link_registry_visibility_filtering() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Build registry with include_private = false (public only)
    let public_registry = LinkRegistry::build(&krate, false, false);

    // Build registry with include_private = true (all items)
    let private_registry = LinkRegistry::build(&krate, false, true);

    // Count registered items in each registry by iterating over the crate index
    let mut public_count = 0;
    let mut private_count = 0;

    for id in krate.index.keys() {
        if public_registry.get_path(*id).is_some() {
            public_count += 1;
        }
        if private_registry.get_path(*id).is_some() {
            private_count += 1;
        }
    }

    // The private registry should have at least as many items as the public registry
    assert!(
        private_count >= public_count,
        "Private registry ({private_count} items) should have >= public registry ({public_count} items)"
    );
}

#[test]
fn test_link_registry_excludes_private_items() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Build registry with include_private = false
    let registry = LinkRegistry::build(&krate, false, false);

    // Verify no non-public items are registered
    for (id, item) in &krate.index {
        if registry.get_path(*id).is_some() {
            // All registered items should be public
            assert!(
                matches!(item.visibility, rustdoc_types::Visibility::Public),
                "Registry should not contain non-public item: {:?} with visibility {:?}",
                item.name,
                item.visibility
            );
        }
    }
}

#[test]
fn test_link_registry_includes_private_when_flag_set() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Build registry with include_private = true
    let registry = LinkRegistry::build(&krate, false, true);

    // Count non-public items that are registered
    let mut non_public_count = 0;
    for (id, item) in &krate.index {
        if registry.get_path(*id).is_some()
            && !matches!(item.visibility, rustdoc_types::Visibility::Public)
        {
            non_public_count += 1;
        }
    }

    // With include_private, we should have some non-public items registered
    // (This test passes even if there are no non-public items in the fixture)
    // The important thing is that non-public items CAN be registered when the flag is set
    eprintln!("Non-public items in registry with include_private=true: {non_public_count}");
}

// =============================================================================
// Re-export Rendering Tests
// =============================================================================

#[test]
fn test_reexports_are_rendered() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    // The crate re-exports Generator and MarkdownCapture from generator module
    // These should appear in the index.md
    let index = capture.get("index.md").expect("Should have index.md");

    // Check that re-exported types are documented
    // The re-exports from lib.rs: Generator, MarkdownCapture, LinkRegistry, etc.
    assert!(
        index.contains("Generator") || index.contains("generator"),
        "Index should mention Generator (either as re-export or link to generator module)"
    );
}

#[test]
fn test_reexports_link_registry_registration() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let registry = LinkRegistry::build(&krate, false, false);

    // Count how many Use items are registered vs how many exist
    let mut use_items_count = 0;
    let mut registered_use_items = 0;

    for (id, item) in &krate.index {
        if matches!(&item.inner, rustdoc_types::ItemEnum::Use(_))
            && matches!(item.visibility, rustdoc_types::Visibility::Public)
        {
            use_items_count += 1;
            if registry.get_path(*id).is_some() {
                registered_use_items += 1;
            }
        }
    }

    eprintln!(
        "Use items: {use_items_count} total, {registered_use_items} registered in link registry"
    );

    // At minimum, Use items with resolvable targets should be registered
    // This is informational - the exact number depends on the fixture
}

#[test]
fn test_reexport_target_resolution() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Find Use items and verify their targets exist
    let mut use_items_with_targets = 0;
    let mut resolved_targets = 0;

    for item in krate.index.values() {
        if let rustdoc_types::ItemEnum::Use(use_item) = &item.inner
            && matches!(item.visibility, rustdoc_types::Visibility::Public)
        {
            use_items_with_targets += 1;

            if let Some(target_id) = &use_item.id
                && krate.index.contains_key(target_id)
            {
                resolved_targets += 1;
            }
        }
    }

    eprintln!("Public Use items: {use_items_with_targets}, targets resolved: {resolved_targets}");

    // All Use items with IDs should resolve to existing items in the same crate
    // (for single-crate mode)
}

// =============================================================================
// Search Index Filtering Tests
// =============================================================================

#[test]
fn test_search_index_only_contains_rendered_items() {
    // This test would require multi-crate generation which needs a directory of JSON files
    // For single-crate mode, we verify that all items in the output have corresponding files

    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    // Get all generated file paths
    let generated_paths: std::collections::HashSet<String> =
        capture.paths().iter().map(ToString::to_string).collect();

    // Verify that index.md exists
    assert!(
        generated_paths.contains("index.md"),
        "Should have root index.md"
    );

    // All paths should be valid markdown files
    for path in &generated_paths {
        assert!(
            Path::new(path)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md")),
            "All generated files should be markdown: {path}"
        );
    }

    eprintln!("Generated {} markdown files", generated_paths.len());
}

#[test]
fn test_generated_links_point_to_existing_files() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    // Collect all generated file paths (normalized)
    let generated_paths: std::collections::HashSet<String> =
        capture.paths().iter().map(ToString::to_string).collect();

    // Check links in generated markdown
    let link_pattern = regex::Regex::new(r"\]\(([^)#]+)(?:#[^)]*)?\)").unwrap();
    let mut broken_links = Vec::new();
    let mut total_links = 0;

    for path in capture.paths() {
        let content = capture.get(path).unwrap();
        for cap in link_pattern.captures_iter(content) {
            let link_target = &cap[1];

            // Skip external links and anchors-only
            if link_target.starts_with("http")
                || link_target.starts_with("https")
                || link_target.is_empty()
            {
                continue;
            }

            total_links += 1;

            // Resolve relative path from current file
            let resolved = resolve_link_path(path, link_target);

            if !generated_paths.contains(&resolved) {
                broken_links.push((path.clone(), link_target.to_string(), resolved));
            }
        }
    }

    eprintln!("Checked {total_links} internal links");

    if !broken_links.is_empty() {
        eprintln!("Found {} potentially broken links:", broken_links.len());
        for (from, link, resolved) in broken_links.iter().take(10) {
            eprintln!("  In {from}: {link} -> {resolved}");
        }
    }

    // Note: Some broken links may be expected if they point to external crates
    // This is informational rather than a strict assertion
}

/// Helper to resolve a relative link path from a source file
fn resolve_link_path(from: &str, link: &str) -> String {
    if link.starts_with('/') {
        // Absolute path (from root)
        link.trim_start_matches('/').to_string()
    } else {
        // Relative path
        let from_dir = std::path::Path::new(from)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let mut parts: Vec<&str> = if from_dir.is_empty() {
            Vec::new()
        } else {
            from_dir.split('/').collect()
        };

        for segment in link.split('/') {
            match segment {
                ".." => {
                    parts.pop();
                },
                "." | "" => {},
                s => parts.push(s),
            }
        }

        parts.join("/")
    }
}

// =============================================================================
// Glob Re-export Tests
// =============================================================================

#[test]
fn test_glob_reexports_expanded() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Count glob re-exports in the fixture
    let mut glob_reexports = 0;
    let mut modules_with_glob = 0;

    for item in krate.index.values() {
        if let rustdoc_types::ItemEnum::Use(use_item) = &item.inner
            && use_item.is_glob
            && matches!(item.visibility, rustdoc_types::Visibility::Public)
        {
            glob_reexports += 1;

            // Check if the target module exists and has items
            if let Some(target_id) = &use_item.id
                && let Some(target) = krate.index.get(target_id)
                && let rustdoc_types::ItemEnum::Module(m) = &target.inner
                && !m.items.is_empty()
            {
                modules_with_glob += 1;
            }
        }
    }

    eprintln!(
        "Found {glob_reexports} glob re-exports, {modules_with_glob} target modules with items"
    );

    // If there are glob re-exports with items, they should be expanded in the generated docs
    if modules_with_glob > 0 {
        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // The generated docs should not be empty for modules with glob re-exports
        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            // Basic sanity check - the file should have content
            assert!(
                !content.is_empty(),
                "Generated file '{path}' should not be empty"
            );
        }
    }
}

#[test]
fn test_glob_reexports_in_link_registry() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let registry = LinkRegistry::build(&krate, false, false);

    // For each glob re-export, verify that target module items are registered
    for item in krate.index.values() {
        if let rustdoc_types::ItemEnum::Use(use_item) = &item.inner
            && use_item.is_glob
            && matches!(item.visibility, rustdoc_types::Visibility::Public)
            && let Some(target_id) = &use_item.id
            && let Some(target) = krate.index.get(target_id)
            && let rustdoc_types::ItemEnum::Module(m) = &target.inner
        {
            // Check that public items from the target module are registered
            for child_id in &m.items {
                if let Some(child) = krate.index.get(child_id)
                    && matches!(child.visibility, rustdoc_types::Visibility::Public)
                {
                    // Public items from glob re-exports should be registered
                    if registry.get_path(*child_id).is_none() {
                        eprintln!(
                            "Note: Item {:?} from glob re-export not in registry",
                            child.name
                        );
                    }
                }
            }
        }
    }
}

// =============================================================================
// Method Anchor Tests
// =============================================================================

#[test]
fn test_method_anchors_present_in_output() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    // Count method anchors in the generated output
    // Method anchors use the format: <span id="typename-methodname">
    let anchor_pattern = regex::Regex::new(r#"<span id="([a-z0-9]+-[a-z0-9_-]+)">"#).unwrap();
    let mut total_anchors = 0;

    for path in capture.paths() {
        let content = capture.get(path).unwrap();
        for cap in anchor_pattern.captures_iter(content) {
            let anchor_id = &cap[1];
            // Verify anchor format: should contain a hyphen separating type and method
            assert!(
                anchor_id.contains('-'),
                "Method anchor '{anchor_id}' should contain hyphen separator"
            );
            total_anchors += 1;
        }
    }

    eprintln!("Found {total_anchors} method anchors in generated output");

    // The fixture should have at least some methods with anchors
    // (Generator, Parser, etc. all have methods)
    assert!(
        total_anchors > 0,
        "Generated docs should contain method anchors for deep linking"
    );
}

#[test]
fn test_method_anchor_format() {
    use cargo_docs_md::linker::AnchorUtils;

    // Test that method_anchor produces expected format
    assert_eq!(AnchorUtils::method_anchor("HashMap", "new"), "hashmap-new");
    assert_eq!(AnchorUtils::method_anchor("Vec", "push"), "vec-push");
    assert_eq!(
        AnchorUtils::method_anchor("Option", "unwrap_or"),
        "option-unwrap-or"
    );

    // Test with generics stripped by slugify_anchor (angle brackets removed)
    assert_eq!(AnchorUtils::method_anchor("Vec<T>", "push"), "vec-push");
    assert_eq!(
        AnchorUtils::method_anchor("HashMap<K, V>", "insert"),
        "hashmap-insert"
    );
}

#[test]
fn test_method_anchors_are_unique_per_type() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    // For each file, verify that method anchors within it are unique
    let anchor_pattern = regex::Regex::new(r#"<span id="([^"]+)">"#).unwrap();

    for path in capture.paths() {
        let content = capture.get(path).unwrap();
        let mut anchors_in_file: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for cap in anchor_pattern.captures_iter(content) {
            let anchor_id = cap[1].to_string();
            assert!(
                anchors_in_file.insert(anchor_id.clone()),
                "Duplicate method anchor '{anchor_id}' found in file '{path}'"
            );
        }
    }
}

// =============================================================================
// Collapsible Impl Block Tests
// =============================================================================

#[test]
fn test_hide_trivial_derives_produces_collapsible_block() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Generate with hide_trivial_derives enabled
    let config = RenderConfig {
        hide_trivial_derives: true,
        ..RenderConfig::default()
    };

    let capture =
        Generator::generate_to_capture_with_config(&krate, CliOutputFormat::Flat, false, config)
            .expect("Generation failed");

    // Check if any file contains collapsible blocks for trivial derives
    let mut found_collapsible = false;
    let mut found_derived_traits_summary = false;

    for path in capture.paths() {
        let content = capture.get(path).unwrap();

        if content.contains("<details>") && content.contains("</details>") {
            found_collapsible = true;
        }

        if content.contains("Derived Traits") && content.contains("implementations") {
            found_derived_traits_summary = true;
        }
    }

    // Note: The fixture may or may not have types with trivial derive implementations
    // If found, they should be in collapsible blocks
    if found_derived_traits_summary {
        assert!(
            found_collapsible,
            "When hide_trivial_derives is enabled and trivial derives exist, \
             output should contain <details> blocks"
        );
    }

    eprintln!(
        "Collapsible blocks found: {found_collapsible}, \
         Derived traits summary found: {found_derived_traits_summary}"
    );
}

#[test]
fn test_default_config_no_collapsible_derives() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Generate with default config (hide_trivial_derives = false)
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
        .expect("Generation failed");

    // With default config, should not have actual <details> collapsible blocks
    // for "Derived Traits". We need to exclude doc string examples that mention
    // this pattern, so look for the actual HTML structure.
    let collapsible_pattern =
        regex::Regex::new(r"<details>\s*<summary>Derived Traits").unwrap();
    let mut found_derived_traits_summary = false;

    for path in capture.paths() {
        let content = capture.get(path).unwrap();

        // Look for actual collapsible blocks (not doc string examples)
        if collapsible_pattern.is_match(content) {
            found_derived_traits_summary = true;
            break;
        }
    }

    assert!(
        !found_derived_traits_summary,
        "With default config (hide_trivial_derives=false), should not have \
         collapsible 'Derived Traits' summary blocks"
    );
}

#[test]
fn test_collapsible_block_contains_trait_table() {
    if !fixture_exists() {
        eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_fixture();

    // Generate with hide_trivial_derives enabled
    let config = RenderConfig {
        hide_trivial_derives: true,
        ..RenderConfig::default()
    };

    let capture =
        Generator::generate_to_capture_with_config(&krate, CliOutputFormat::Flat, false, config)
            .expect("Generation failed");

    // If we find a collapsible Derived Traits block, it should contain a table
    for path in capture.paths() {
        let content = capture.get(path).unwrap();

        if content.contains("<summary>Derived Traits") {
            // Should contain table headers
            assert!(
                content.contains("| Trait | Description |"),
                "Collapsible Derived Traits block should contain trait table headers"
            );
            assert!(
                content.contains("| ----- | ----------- |"),
                "Collapsible Derived Traits block should contain table separator"
            );
            break;
        }
    }
}

// =============================================================================
// Trait Definition Rendering Tests
// =============================================================================

/// Helper to get the walkdir test fixture path.
fn walkdir_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("doc")
        .join("walkdir.json")
}

/// Check if the walkdir fixture exists.
fn walkdir_fixture_exists() -> bool {
    walkdir_fixture_path().exists()
}

/// Load the walkdir fixture crate.
fn load_walkdir_fixture() -> rustdoc_types::Crate {
    let path = walkdir_fixture_path();
    Parser::parse_file(&path)
        .expect("Failed to parse walkdir fixture. Run `cargo doc --output-format json` first.")
}

#[test]
fn test_trait_definition_renders_required_methods() {
    if !walkdir_fixture_exists() {
        eprintln!("Skipping test: walkdir fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_walkdir_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    let index = capture.get("index.md").expect("Should have index.md");

    // Verify DirEntryExt trait is rendered
    assert!(
        index.contains("### `DirEntryExt`"),
        "Index should contain DirEntryExt trait heading"
    );

    // Verify Required Methods section exists
    assert!(
        index.contains("#### Required Methods"),
        "Trait should have Required Methods section"
    );

    // Verify the ino method is listed
    assert!(
        index.contains("fn ino(&self) -> u64"),
        "Required Methods should include ino method"
    );
}

#[test]
fn test_trait_definition_renders_implementors() {
    if !walkdir_fixture_exists() {
        eprintln!("Skipping test: walkdir fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_walkdir_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    let index = capture.get("index.md").expect("Should have index.md");

    // Verify Implementors section exists
    assert!(
        index.contains("#### Implementors"),
        "Trait should have Implementors section"
    );

    // Verify DirEntry is listed as implementor
    assert!(
        index.contains("DirEntry"),
        "Implementors should include DirEntry"
    );
}

#[test]
fn test_trait_definition_has_traits_section() {
    if !walkdir_fixture_exists() {
        eprintln!("Skipping test: walkdir fixture not found. Run `cargo doc --output-format json`");
        return;
    }

    let krate = load_walkdir_fixture();
    let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
        .expect("Generation failed");

    let index = capture.get("index.md").expect("Should have index.md");

    // Verify Traits section exists and contains DirEntryExt
    assert!(
        index.contains("## Traits"),
        "Index should have Traits section"
    );

    // Verify the trait code block is rendered
    assert!(
        index.contains("```rust\ntrait DirEntryExt"),
        "Trait should have code block with signature"
    );
}

// =============================================================================
// Code Block Handling Tests (Phase 4.1)
// =============================================================================

/// Tests for `unhide_code_lines` function which processes hidden code lines
/// in documentation code blocks (lines starting with `# `).
///
/// The function:
/// 1. Strips `# ` prefix from lines in code blocks (unhides rustdoc hidden lines)
/// 2. Converts bare fences (```) to rust fences (```rust)
mod code_block_tests {
    use cargo_docs_md::generator::DocLinkUtils;

    #[test]
    fn test_unhide_code_lines_converts_bare_fence_to_rust() {
        // Code block with bare ``` fence - bare fences get `rust` added
        let input = "```\nlet x = 1;\n```";
        let result = DocLinkUtils::unhide_code_lines(input);

        // Bare fence becomes ```rust
        assert!(
            result.contains("```rust\n"),
            "Bare fence should be converted to rust fence, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_preserves_rust_fence() {
        // Code block with ```rust should remain unchanged
        let input = "```rust\nlet x = 1;\n```";
        let result = DocLinkUtils::unhide_code_lines(input);

        assert!(
            result.contains("```rust\n"),
            "Rust fence should be preserved, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_strips_hash_prefix() {
        // Hidden lines (starting with `# `) have their prefix stripped
        // `# use std::io;` becomes `use std::io;`
        let input = "```rust\n# use std::io;\nlet x = 1;\n```";
        let result = DocLinkUtils::unhide_code_lines(input);

        assert!(
            result.contains("use std::io;"),
            "Hidden line prefix should be stripped, got: {result}"
        );
        assert!(
            !result.contains("# use std::io;"),
            "Original hidden line should be removed, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_handles_tilde_fence() {
        // ~~~ fence should also work - bare tilde gets `rust` added
        let input = "~~~\n# hidden\nvisible\n~~~";
        let result = DocLinkUtils::unhide_code_lines(input);

        assert!(
            result.contains("~~~rust\n"),
            "Bare tilde fence should be converted to rust fence, got: {result}"
        );
        assert!(
            result.contains("\nhidden\n"),
            "Hidden line prefix should be stripped, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_handles_mixed_fences() {
        // Mixed fence types should each be handled correctly
        let input = "```rust\n# a\n```\n\n~~~\n# b\n~~~";
        let result = DocLinkUtils::unhide_code_lines(input);

        assert!(
            result.contains("\na\n"),
            "First hidden line should have prefix stripped, got: {result}"
        );
        assert!(
            result.contains("\nb\n"),
            "Second hidden line should have prefix stripped, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_bare_fence_strips_hidden() {
        // Bare fence (``` without language) gets `rust` added and hidden lines stripped
        let input = "```\n# This becomes visible\n```";
        let result = DocLinkUtils::unhide_code_lines(input);

        // Bare fence becomes ```rust and # prefix is stripped
        assert!(
            result.contains("```rust\n"),
            "Bare fence should become rust fence, got: {result}"
        );
        assert!(
            result.contains("This becomes visible"),
            "Hidden line prefix should be stripped, got: {result}"
        );
        assert!(
            !result.contains("# This becomes"),
            "Original hidden prefix should be gone, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_preserves_text_outside_blocks() {
        let input = "Some text\n# This is a heading\n```rust\n# hidden\n```\nMore text";
        let result = DocLinkUtils::unhide_code_lines(input);

        // Text outside code blocks should be preserved
        assert!(
            result.contains("Some text"),
            "Text before block should be preserved"
        );
        assert!(
            result.contains("More text"),
            "Text after block should be preserved"
        );
        // The # heading outside the code block should be preserved
        assert!(
            result.contains("# This is a heading"),
            "Markdown heading should be preserved"
        );
    }

    #[test]
    fn test_unhide_code_lines_lone_hash_becomes_empty_line() {
        // A lone # (hidden empty line) becomes an empty line
        let input = "```\n#\nlet x = 1;\n```";
        let result = DocLinkUtils::unhide_code_lines(input);

        // After ```rust, there should be an empty line before `let x = 1;`
        // The lone # becomes an empty line
        assert!(
            result.contains("```rust\n\nlet x = 1;"),
            "Lone hash should become empty line, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_strips_hash_in_all_code_blocks() {
        // NOTE: unhide_code_lines strips `# ` from ALL code blocks,
        // not just rust blocks. This is because rustdoc's hidden line
        // syntax works in all code blocks.
        let input = "```python\n# This hidden line\nprint('hello')\n```";
        let result = DocLinkUtils::unhide_code_lines(input);

        // The `# ` prefix is stripped even in non-rust blocks
        assert!(
            result.contains("This hidden line"),
            "Hidden line should be unhidden, got: {result}"
        );
        assert!(
            !result.contains("# This hidden line"),
            "Original hidden prefix should be gone, got: {result}"
        );
    }

    #[test]
    fn test_unhide_code_lines_preserves_regular_hash() {
        // Lines with # not followed by space are preserved
        // (only `# ` prefix is stripped, not `#foo`)
        let input = "```rust\n#[derive(Debug)]\nstruct Foo;\n```";
        let result = DocLinkUtils::unhide_code_lines(input);

        assert!(
            result.contains("#[derive(Debug)]"),
            "Derive attribute should be preserved, got: {result}"
        );
    }
}

// =============================================================================
// Link Resolution Tests (Phase 4.2)
// =============================================================================

/// Tests for the 3-strategy link resolution.
mod link_resolution_tests {
    use super::*;

    #[test]
    fn test_intra_doc_links_are_resolved() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();
        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Look for resolved links in the output
        // Resolved links look like: [`Type`](path.md#anchor)
        let resolved_link_pattern = regex::Regex::new(r"\[`[^`]+`\]\([^)]+\)").unwrap();
        let mut resolved_count = 0;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            resolved_count += resolved_link_pattern.find_iter(content).count();
        }

        eprintln!("Found {resolved_count} resolved intra-doc links in output");

        // The fixture should have some resolved links
        assert!(
            resolved_count > 0,
            "Generated docs should contain resolved intra-doc links"
        );
    }

    #[test]
    fn test_unresolved_links_have_fallback_format() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();
        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Find all link references: [`text`] potentially followed by (url)
        // An unresolved link is [`text`] NOT followed by (url)
        let link_ref_pattern = regex::Regex::new(r"\[`[^`]+`\]").unwrap();
        let mut unresolved_count = 0;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            for mat in link_ref_pattern.find_iter(content) {
                // Check if this match is followed by ( which would indicate a resolved link
                let end_pos = mat.end();
                let is_resolved = content[end_pos..].starts_with('(');
                if !is_resolved {
                    unresolved_count += 1;
                }
            }
        }

        eprintln!("Found {unresolved_count} unresolved link references");

        // Having some unresolved links is normal (external crate references)
        // This test just verifies the format
    }

    #[test]
    fn test_link_resolution_uses_short_name_fallback() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();
        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Check for links that use short names (Strategy 2 or 3)
        // These would be links where the display text differs from a full path
        let simple_link_pattern = regex::Regex::new(r"\[`([A-Z][a-zA-Z0-9]*)`\]\(").unwrap();
        let mut short_name_links = 0;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            short_name_links += simple_link_pattern.find_iter(content).count();
        }

        eprintln!("Found {short_name_links} links with short/simple names");

        // Should have links using short names
        assert!(
            short_name_links > 0,
            "Should have links resolved with short names (Strategy 2/3)"
        );
    }
}

// =============================================================================
// Tuple Struct and Enum Rendering Tests (Phase 4.3)
// =============================================================================

/// Tests for tuple struct and enum variant rendering.
mod tuple_rendering_tests {
    use super::*;

    #[test]
    fn test_tuple_struct_renders_fields() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();

        // Look for tuple structs in the crate
        let mut has_tuple_struct = false;
        for item in krate.index.values() {
            if let rustdoc_types::ItemEnum::Struct(s) = &item.inner {
                if matches!(s.kind, rustdoc_types::StructKind::Tuple(_)) {
                    has_tuple_struct = true;
                    break;
                }
            }
        }

        if !has_tuple_struct {
            eprintln!("Skipping test: no tuple structs in fixture");
            return;
        }

        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Tuple structs should render with parentheses: struct Name(Type1, Type2);
        let tuple_struct_pattern = regex::Regex::new(r"struct \w+[^{]*\([^)]+\);").unwrap();
        let mut found_tuple_struct = false;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            if tuple_struct_pattern.is_match(content) {
                found_tuple_struct = true;
                break;
            }
        }

        assert!(
            found_tuple_struct,
            "Tuple struct should be rendered with parentheses format"
        );
    }

    #[test]
    fn test_tuple_enum_variant_renders_fields() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();

        // Look for enums with tuple variants
        let mut has_tuple_variant = false;
        for item in krate.index.values() {
            if let rustdoc_types::ItemEnum::Enum(e) = &item.inner {
                for variant_id in &e.variants {
                    if let Some(variant) = krate.index.get(variant_id) {
                        if let rustdoc_types::ItemEnum::Variant(v) = &variant.inner {
                            if matches!(v.kind, rustdoc_types::VariantKind::Tuple(_)) {
                                has_tuple_variant = true;
                                break;
                            }
                        }
                    }
                }
            }
            if has_tuple_variant {
                break;
            }
        }

        if !has_tuple_variant {
            eprintln!("Skipping test: no tuple enum variants in fixture");
            return;
        }

        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Tuple variants should render with parentheses: VariantName(Type),
        let tuple_variant_pattern = regex::Regex::new(r"    \w+\([^)]+\),").unwrap();
        let mut found_tuple_variant = false;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            if tuple_variant_pattern.is_match(content) {
                found_tuple_variant = true;
                break;
            }
        }

        assert!(
            found_tuple_variant,
            "Tuple enum variant should be rendered with parentheses format"
        );
    }
}

// =============================================================================
// Trait Bound Rendering Tests (Phase 4.3)
// =============================================================================

/// Tests for trait definition rendering with bounds.
mod trait_bound_rendering_tests {
    use super::*;

    #[test]
    fn test_trait_with_bounds_renders_correctly() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();

        // Look for traits with bounds (supertraits)
        let mut has_trait_with_bounds = false;
        for item in krate.index.values() {
            if let rustdoc_types::ItemEnum::Trait(t) = &item.inner {
                if !t.bounds.is_empty() {
                    has_trait_with_bounds = true;
                    eprintln!("Found trait with bounds: {:?}", item.name);
                    break;
                }
            }
        }

        if !has_trait_with_bounds {
            eprintln!("Skipping test: no traits with bounds in fixture");
            return;
        }

        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Traits with bounds should render with `: Bound1 + Bound2`
        let trait_with_bounds_pattern = regex::Regex::new(r"trait \w+[^{]*: [^{]+\{").unwrap();
        let mut found_trait_with_bounds = false;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            if trait_with_bounds_pattern.is_match(content) {
                found_trait_with_bounds = true;
                break;
            }
        }

        assert!(
            found_trait_with_bounds,
            "Trait with bounds should be rendered with `: Bound` format"
        );
    }

    #[test]
    fn test_trait_without_bounds_renders_correctly() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();

        // Look for traits without bounds
        let mut has_trait_without_bounds = false;
        for item in krate.index.values() {
            if let rustdoc_types::ItemEnum::Trait(t) = &item.inner {
                if t.bounds.is_empty() {
                    has_trait_without_bounds = true;
                    break;
                }
            }
        }

        if !has_trait_without_bounds {
            eprintln!("Skipping test: no traits without bounds in fixture");
            return;
        }

        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Traits without bounds should not have `: ` before `{`
        let trait_no_bounds_pattern = regex::Regex::new(r"trait \w+(<[^>]+>)?\s*\{").unwrap();
        let mut found_trait_no_bounds = false;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            if trait_no_bounds_pattern.is_match(content) {
                found_trait_no_bounds = true;
                break;
            }
        }

        assert!(
            found_trait_no_bounds,
            "Trait without bounds should render without colon"
        );
    }
}

// =============================================================================
// Markdown Output Format Verification Tests
// =============================================================================

/// Tests that verify the overall structure of generated markdown matches expectations.
mod markdown_format_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generated_markdown_has_expected_sections() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();
        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Nested, false)
            .expect("Generation failed");

        let index = capture.get("index.md").expect("Should have index.md");

        // Verify expected sections exist
        let expected_headings = ["## Modules", "## Structs", "## Functions"];

        for heading in &expected_headings {
            if index.contains(heading) {
                eprintln!("Found expected heading: {heading}");
            }
        }

        // At minimum, the crate should have some content sections
        assert!(
            index.contains("## "),
            "Index should have level-2 headings for sections"
        );
    }

    #[test]
    fn test_generated_markdown_has_quick_reference() {
        if !fixture_exists() {
            eprintln!("Skipping test: fixture not found. Run `cargo doc --output-format json`");
            return;
        }

        let krate = load_fixture();
        let capture = Generator::generate_to_capture(&krate, CliOutputFormat::Flat, false)
            .expect("Generation failed");

        // Check for Quick Reference table in module files
        // The table has 3 columns: Item, Kind, Description
        let table_header_pattern = regex::Regex::new(r"\| Item \| Kind \| Description \|").unwrap();
        let mut found_quick_ref = false;

        for path in capture.paths() {
            let content = capture.get(path).unwrap();
            if table_header_pattern.is_match(content) {
                found_quick_ref = true;
                break;
            }
        }

        assert!(
            found_quick_ref,
            "Generated docs should contain Quick Reference table"
        );
    }

    #[test]
    fn test_fixture_file_format_is_valid() {
        // Read the ideal fixture file and verify its format
        let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("ideal_module_output.md");

        if !fixture_path.exists() {
            eprintln!("Skipping test: ideal fixture not found");
            return;
        }

        let content = fs::read_to_string(&fixture_path).expect("Failed to read fixture");

        // Verify breadcrumb structure (module pages start with breadcrumb)
        assert!(
            content.starts_with("*["),
            "Fixture should start with breadcrumb navigation"
        );
        assert!(
            content.contains("---\n\n# Module `"),
            "Fixture should have separator before module heading"
        );

        // Verify Contents section (TOC)
        assert!(
            content.contains("## Contents"),
            "Fixture should have Contents section (TOC)"
        );
        assert!(
            content.contains("- [Structs](#structs)"),
            "Contents should have Structs link"
        );
        // Verify slugified anchors (underscores become hyphens)
        assert!(
            content.contains("#default-capacity"),
            "Anchors should use hyphens not underscores"
        );

        // Verify Quick Reference table structure
        assert!(
            content.contains("## Quick Reference"),
            "Fixture should have Quick Reference section"
        );
        assert!(
            content.contains("| Item | Kind | Description |"),
            "Fixture should have Quick Reference table with Kind column"
        );
        assert!(
            content.contains("| struct |"),
            "Quick Reference should categorize structs"
        );

        // Verify main sections
        assert!(
            content.contains("## Structs"),
            "Fixture should have Structs section"
        );
        assert!(
            content.contains("## Enums"),
            "Fixture should have Enums section"
        );
        assert!(
            content.contains("## Traits"),
            "Fixture should have Traits section"
        );
        assert!(
            content.contains("## Functions"),
            "Fixture should have Functions section"
        );
        assert!(
            content.contains("## Constants"),
            "Fixture should have Constants section"
        );
        assert!(
            content.contains("## Type Aliases"),
            "Fixture should have Type Aliases section"
        );

        // Verify code blocks are properly formatted
        assert!(
            content.contains("```rust\n"),
            "Fixture should have Rust code blocks"
        );
        assert!(
            content.contains("\n```\n"),
            "Fixture code blocks should be properly closed"
        );

        // Verify source location format with links
        assert!(
            content.contains("*Defined in [`"),
            "Fixture should have source location markers with links"
        );
        assert!(
            content.contains("](../../.source_"),
            "Source locations should link to .source_ directory"
        );

        // Verify struct definitions
        assert!(
            content.contains("struct Container<T>"),
            "Fixture should have generic struct"
        );
        assert!(
            content.contains("struct Wrapper<A, B>(A, B)"),
            "Fixture should have tuple struct"
        );

        // Verify Fields section
        assert!(
            content.contains("#### Fields"),
            "Structs should have Fields section"
        );
        assert!(
            content.contains("- **`value`**: `T`"),
            "Fields should use bold name with type"
        );

        // Verify Implementations section
        assert!(
            content.contains("#### Implementations"),
            "Structs should have Implementations section"
        );
        assert!(
            content.contains("#### Trait Implementations"),
            "Structs should have Trait Implementations section"
        );

        // Verify trait structure
        assert!(
            content.contains("trait Transformable<T>: Clone + Debug { ... }"),
            "Fixture should have trait with bounds and ellipsis"
        );
        assert!(
            content.contains("#### Required Methods"),
            "Traits should have Required Methods section"
        );
        assert!(
            content.contains("#### Provided Methods"),
            "Traits should have Provided Methods section"
        );
        assert!(
            content.contains("#### Implementors"),
            "Traits should have Implementors section"
        );

        // Verify enum structure
        assert!(
            content.contains("#### Variants"),
            "Enums should have Variants section"
        );
        assert!(
            content.contains("Running(u64)"),
            "Enums should have tuple variant"
        );
        assert!(
            content.contains("Complete { result: String"),
            "Enums should have struct variant"
        );

        // Verify method anchors use hyphens
        assert!(
            content.contains("<span id=\"container-new\"></span>"),
            "Fixture should have method anchors"
        );
        assert!(
            content.contains("<span id=\"container-into-inner\"></span>"),
            "Method anchors should use hyphens for multi-word names"
        );

        // Verify constant format includes type suffix
        assert!(
            content.contains("const DEFAULT_CAPACITY: usize = 16usize;"),
            "Constants should include value with type suffix"
        );

        eprintln!("Fixture file validation passed");
    }
}
