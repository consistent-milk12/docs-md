//! Configuration for markdown rendering.
//!
//! This module provides [`RenderConfig`] for controlling how documentation
//! is rendered, and [`SourceConfig`] for source code integration options.

use std::path::PathBuf;

/// Configuration options for markdown rendering.
#[derive(Debug, Clone)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "CLI flags map directly to these bools; cache optimization unnecessary here"
)]
pub struct RenderConfig {
    /// Generate table of contents for modules with more than this many items.
    pub toc_threshold: usize,

    /// Generate quick reference tables at the top of modules.
    pub quick_reference: bool,

    /// Group impl blocks by category (Derive, Conversion, Iterator, etc.).
    pub group_impls: bool,

    /// Hide trivial derive implementations (Clone, Copy, Debug, etc.).
    pub hide_trivial_derives: bool,

    /// Generate method-level anchors for deep linking.
    pub method_anchors: bool,

    /// Include full method documentation instead of first-paragraph summaries.
    ///
    /// When `false` (default), method docs in impl blocks show only the first
    /// paragraph (up to the first blank line). When `true`, the complete
    /// documentation is included.
    pub full_method_docs: bool,

    /// Source code integration options.
    pub include_source: SourceConfig,
}

/// Configuration for source code integration.
///
/// Requires the `source-parsing` feature to have any effect.
#[derive(Debug, Clone, Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "TODO: Consider cache line efficiency later"
)]
pub struct SourceConfig {
    /// Include function bodies in collapsible sections.
    pub function_bodies: bool,

    /// Show actual values for constants and statics.
    pub const_values: bool,

    /// Include private items in a separate section.
    pub private_items: bool,

    /// Add <file:line> references to items.
    pub source_locations: bool,

    /// Path to the `.source_*` directory containing collected dependency sources.
    ///
    /// When set, source location references will use paths relative to this directory
    /// and generate clickable links. When `None`, absolute paths from rustdoc JSON
    /// are displayed without links.
    pub source_dir: Option<PathBuf>,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            toc_threshold: 10,
            quick_reference: true,
            group_impls: true,
            hide_trivial_derives: false,
            method_anchors: true,
            full_method_docs: false,
            include_source: SourceConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_config_default_values() {
        let config = RenderConfig::default();

        assert_eq!(config.toc_threshold, 10);
        assert!(config.quick_reference);
        assert!(config.group_impls);
        assert!(!config.hide_trivial_derives);
        assert!(config.method_anchors);
        assert!(!config.full_method_docs);
    }

    #[test]
    fn source_config_default_values() {
        let config = SourceConfig::default();

        assert!(!config.function_bodies);
        assert!(!config.const_values);
        assert!(!config.private_items);
        assert!(!config.source_locations);
    }

    #[test]
    fn render_config_source_config_default() {
        let config = RenderConfig::default();
        let source = &config.include_source;

        // Source config within RenderConfig should also have defaults
        assert!(!source.function_bodies);
        assert!(!source.const_values);
        assert!(!source.private_items);
        assert!(!source.source_locations);
    }

    #[test]
    fn render_config_custom_values() {
        let config = RenderConfig {
            toc_threshold: 5,
            quick_reference: false,
            group_impls: false,
            hide_trivial_derives: true,
            method_anchors: false,
            full_method_docs: true,
            include_source: SourceConfig {
                function_bodies: true,
                const_values: true,
                private_items: true,
                source_locations: true,
                source_dir: Some(PathBuf::from(".source_12345")),
            },
        };

        assert_eq!(config.toc_threshold, 5);
        assert!(!config.quick_reference);
        assert!(!config.group_impls);
        assert!(config.hide_trivial_derives);
        assert!(!config.method_anchors);
        assert!(config.full_method_docs);
        assert!(config.include_source.function_bodies);
        assert!(config.include_source.const_values);
        assert!(config.include_source.private_items);
        assert!(config.include_source.source_locations);
        assert!(config.include_source.source_dir.is_some());
    }
}
