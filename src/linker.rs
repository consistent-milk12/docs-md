//! Cross-reference linking for markdown documentation.
//!
//! This module provides the `LinkRegistry` which maps rustdoc item IDs to their
//! corresponding markdown file paths. This enables creating clickable links
//! between items in the generated documentation.
//!
//! # How It Works
//!
//! 1. During initialization, `LinkRegistry::build()` traverses the entire crate
//!    and records where each item's documentation will be written.
//!
//! 2. During markdown generation, `create_link()` is called to generate
//!    relative links from one file to another.
//!
//! # Path Formats
//!
//! The registry supports two output formats:
//!
//! - **Flat**: `module.md`, `parent__child.md` (double-underscore separators)
//! - **Nested**: `module/index.md`, `parent/child/index.md` (directory structure)
//!
//! # Example
//!
//! ```ignore
//! let registry = LinkRegistry::build(&krate, true); // flat format
//! let link = registry.create_link(&some_id, "index.md");
//! // Returns: Some("[`ItemName`](module.md)")
//! ```

use rustdoc_types::{Crate, Id, ItemEnum};
use std::collections::HashMap;

/// Registry mapping item IDs to their documentation file paths.
///
/// This is the central data structure for cross-reference resolution.
/// It's built once during generation and queried whenever we need to
/// create links between items.
#[derive(Debug, Default)]
pub struct LinkRegistry {
    /// Maps each item's ID to the markdown file path where it's documented.
    ///
    /// Paths are relative to the output directory root.
    /// Examples: `"index.md"`, `"span.md"`, `"span/index.md"`
    item_paths: HashMap<Id, String>,

    /// Maps each item's ID to its display name.
    ///
    /// Used to generate the link text (e.g., `[`name`](path)`).
    /// This is typically the item's identifier without the full path.
    item_names: HashMap<Id, String>,
}

impl LinkRegistry {
    /// Build a link registry by traversing all items in the crate.
    ///
    /// This function walks the module tree starting from the root and records
    /// the file path where each item will be documented. The paths depend on
    /// the output format (flat vs nested).
    ///
    /// # Arguments
    ///
    /// * `krate` - The parsed rustdoc crate containing all items
    /// * `flat_format` - If true, use flat paths (`mod.md`); if false, use nested (`mod/index.md`)
    ///
    /// # Returns
    ///
    /// A populated `LinkRegistry` ready for link creation.
    ///
    /// # Algorithm
    ///
    /// 1. Start at the crate root module
    /// 2. For each top-level module: register it and recursively process children
    /// 3. For structs/enums/traits at root level: register them to `index.md`
    /// 4. Other items (functions, constants) are registered within their module's file
    pub fn build(krate: &Crate, flat_format: bool) -> Self {
        let mut registry = Self::default();

        // Get root module - if missing, return empty registry
        let Some(root) = krate.index.get(&krate.root) else {
            return registry;
        };

        // Process all items in the root module
        if let ItemEnum::Module(module) = &root.inner {
            for item_id in &module.items {
                if let Some(item) = krate.index.get(item_id) {
                    match &item.inner {
                        // Modules get their own files and are processed recursively
                        ItemEnum::Module(_) => {
                            let module_name = item.name.as_deref().unwrap_or("unnamed");

                            // Determine the file path based on output format
                            let path = if flat_format {
                                format!("{module_name}.md")
                            } else {
                                format!("{module_name}/index.md")
                            };

                            // Recursively register this module and all its contents
                            registry.register_module_items(
                                krate,
                                *item_id,
                                item,
                                &path,
                                module_name,
                                flat_format,
                            );
                        }

                        // Top-level types (structs, enums, traits) are in the root index.md
                        ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Trait(_) => {
                            let name = item.name.as_deref().unwrap_or("unnamed");
                            registry.item_paths.insert(*item_id, "index.md".to_string());
                            registry.item_names.insert(*item_id, name.to_string());
                        }

                        // Other items (functions, constants, etc.) are handled within modules
                        _ => {}
                    }
                }
            }
        }

        registry
    }

    /// Recursively register all items within a module.
    ///
    /// This is called for each module in the crate to populate the registry
    /// with all items that can be linked to.
    ///
    /// # Arguments
    ///
    /// * `krate` - The full crate for looking up item details
    /// * `module_id` - ID of the module being registered
    /// * `module_item` - The module's Item data
    /// * `path` - File path where this module's docs will be written
    /// * `module_prefix` - Prefix for building child paths (e.g., "parent" or "`parent__child`")
    /// * `flat_format` - Whether to use flat naming convention
    fn register_module_items(
        &mut self,
        krate: &Crate,
        module_id: Id,
        module_item: &rustdoc_types::Item,
        path: &str,
        module_prefix: &str,
        flat_format: bool,
    ) {
        // First, register the module itself
        let module_name = module_item.name.as_deref().unwrap_or("unnamed");
        self.item_paths.insert(module_id, path.to_string());
        self.item_names.insert(module_id, module_name.to_string());

        // Then register all items within this module
        if let ItemEnum::Module(module) = &module_item.inner {
            for item_id in &module.items {
                if let Some(item) = krate.index.get(item_id) {
                    let name = item.name.as_deref().unwrap_or("unnamed");

                    match &item.inner {
                        // Types and functions within this module are documented in its file
                        // They'll be linked with anchors (e.g., module.md#structname)
                        ItemEnum::Struct(_)
                        | ItemEnum::Enum(_)
                        | ItemEnum::Trait(_)
                        | ItemEnum::Function(_)
                        | ItemEnum::Constant { .. }
                        | ItemEnum::TypeAlias(_)
                        | ItemEnum::Macro(_) => {
                            self.item_paths.insert(*item_id, path.to_string());
                            self.item_names.insert(*item_id, name.to_string());
                        }

                        // Nested modules get their own files - recurse into them
                        ItemEnum::Module(_) => {
                            // Build the file path for this submodule
                            let sub_path = if flat_format {
                                // Flat: parent__child__grandchild.md
                                format!("{module_prefix}__{name}.md")
                            } else {
                                // Nested: parent/child/grandchild/index.md
                                format!("{module_prefix}/{name}/index.md")
                            };

                            // Build the prefix for any further nesting
                            let sub_prefix = format!("{module_prefix}__{name}");

                            // Recurse into the submodule
                            self.register_module_items(
                                krate,
                                *item_id,
                                item,
                                &sub_path,
                                &sub_prefix,
                                flat_format,
                            );
                        }

                        // Other item types (impl blocks, etc.) don't need registration
                        _ => {}
                    }
                }
            }
        }
    }

    /// Get the file path where an item is documented.
    ///
    /// # Arguments
    ///
    /// * `id` - The item's unique ID from rustdoc JSON
    ///
    /// # Returns
    ///
    /// The relative file path (e.g., `"span.md"` or `"span/index.md"`),
    /// or `None` if the item isn't registered.
    pub fn get_path(&self, id: Id) -> Option<&String> {
        self.item_paths.get(&id)
    }

    /// Get the display name for an item.
    ///
    /// # Arguments
    ///
    /// * `id` - The item's unique ID from rustdoc JSON
    ///
    /// # Returns
    ///
    /// The item's name for display in links (e.g., `"Span"`),
    /// or `None` if the item isn't registered.
    pub fn get_name(&self, id: Id) -> Option<&String> {
        self.item_names.get(&id)
    }

    /// Create a markdown link to an item from a given source file.
    ///
    /// This is the main method used during markdown generation to create
    /// clickable links between documented items.
    ///
    /// # Arguments
    ///
    /// * `id` - The target item's ID
    /// * `from_path` - The source file creating the link (e.g., `"index.md"`)
    ///
    /// # Returns
    ///
    /// A formatted markdown link like `[``ItemName``](path/to/file.md)`,
    /// or `None` if the target item isn't registered.
    ///
    /// # Link Types
    ///
    /// - **Same file**: Returns an anchor link (`#itemname`)
    /// - **Different file**: Returns a relative path (`../other/file.md`)
    ///
    /// # Example
    ///
    /// ```ignore
    /// // From index.md linking to span.md
    /// registry.create_link(&span_id, "index.md")
    /// // Returns: Some("[`Span`](span.md)")
    ///
    /// // From span/index.md linking to index.md
    /// registry.create_link(&root_id, "span/index.md")
    /// // Returns: Some("[`crate`](../index.md)")
    /// ```
    pub fn create_link(&self, id: Id, from_path: &str) -> Option<String> {
        let target_path = self.item_paths.get(&id)?;
        let name = self.item_names.get(&id)?;

        // Calculate relative path from source to target file
        let relative_path = compute_relative_path(from_path, target_path);

        // Determine the link destination:
        // - Same file: use an anchor (#name)
        // - Different file: use the relative path
        let link = if target_path == from_path {
            // Convert name to anchor format: lowercase, spaces to dashes
            format!("#{}", name.to_lowercase().replace(' ', "-"))
        } else {
            relative_path
        };

        // Format as markdown link with backticks around the name
        Some(format!("[`{name}`]({link})"))
    }
}

/// Compute the relative path from one file to another.
///
/// This function calculates the relative path needed to navigate from
/// one markdown file to another within the generated documentation.
///
/// # Arguments
///
/// * `from` - The source file path (e.g., `"span/index.md"`)
/// * `to` - The target file path (e.g., `"field/index.md"`)
///
/// # Returns
///
/// A relative path string (e.g., `"../field/index.md"`)
///
/// # Algorithm
///
/// 1. Split both paths into components
/// 2. Find the common prefix (shared directories)
/// 3. Count how many `../` are needed to go up from source
/// 4. Append the remaining target path components
///
/// # Examples
///
/// - Same directory: `"index.md"` → `"span.md"` = `"span.md"`
/// - Into subdirectory: `"index.md"` → `"span/index.md"` = `"span/index.md"`
/// - Up to parent: `"span/index.md"` → `"index.md"` = `"../index.md"`
/// - Sibling directory: `"span/index.md"` → `"field/index.md"` = `"../field/index.md"`
fn compute_relative_path(from: &str, to: &str) -> String {
    // Same file - no path needed
    if from == to {
        return String::new();
    }

    // Split paths into directory components
    let from_parts: Vec<&str> = from.split('/').collect();
    let to_parts: Vec<&str> = to.split('/').collect();

    // Find how many leading components are the same
    // e.g., "a/b/c.md" and "a/b/d.md" share ["a", "b"]
    let common_len = from_parts
        .iter()
        .zip(to_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // Calculate how many directories to go up from the source.
    // We subtract 1 because the last component is the filename, not a directory.
    // e.g., from "a/b/c.md" we need to go up 2 directories to reach root.
    let ups = from_parts
        .len()
        .saturating_sub(1) // Don't count the filename
        .saturating_sub(common_len); // Don't count shared directories

    // Build the relative path
    let mut result = String::new();

    // Add "../" for each directory we need to go up
    for _ in 0..ups {
        result.push_str("../");
    }

    // Add the remaining path components from the target
    for (i, part) in to_parts.iter().enumerate().skip(common_len) {
        if i > common_len {
            result.push('/');
        }
        result.push_str(part);
    }

    // If result is empty (same directory), just use the target path
    if result.is_empty() {
        to.to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test: Files in the same directory need no path prefix.
    #[test]
    fn test_relative_path_same_dir() {
        assert_eq!(compute_relative_path("index.md", "span.md"), "span.md");
    }

    /// Test: Linking from root into a subdirectory.
    #[test]
    fn test_relative_path_child_dir() {
        assert_eq!(
            compute_relative_path("index.md", "span/index.md"),
            "span/index.md"
        );
    }

    /// Test: Linking from subdirectory back to root.
    #[test]
    fn test_relative_path_parent_dir() {
        assert_eq!(
            compute_relative_path("span/index.md", "index.md"),
            "../index.md"
        );
    }

    /// Test: Linking between sibling directories.
    #[test]
    fn test_relative_path_sibling_dir() {
        assert_eq!(
            compute_relative_path("span/index.md", "field/index.md"),
            "../field/index.md"
        );
    }
}
