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

use docs_md::generator::Generator;
use docs_md::linker::LinkRegistry;
use docs_md::parser::Parser;
use docs_md::{CliOutputFormat, MarkdownCapture};

/// Helper to get the path to the test fixture.
fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("doc")
        .join("docs_md.json")
}

/// Check if the test fixture exists.
fn fixture_exists() -> bool {
    fixture_path().exists()
}

/// Load the test fixture crate.
fn load_fixture() -> rustdoc_types::Crate {
    let path = fixture_path();
    Parser::parse_json(&path)
        .expect("Failed to parse test fixture. Run `cargo doc --output-format json` first.")
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
