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

use std::path::PathBuf;

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
        .map(|p| public_only.get(p).map_or(0, |s| s.len()))
        .sum();
    let private_len: usize = with_private
        .paths()
        .iter()
        .map(|p| with_private.get(p).map_or(0, |s| s.len()))
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
    let registry = LinkRegistry::build(&krate, true);

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
    let registry = LinkRegistry::build(&krate, false);

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
