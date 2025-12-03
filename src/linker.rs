//! Cross-reference linking for markdown documentation.
//!
//! This module provides functionality to resolve links between items
//! across modules in the generated documentation.

use rustdoc_types::{Crate, Id, ItemEnum};
use std::collections::HashMap;

/// A registry mapping item IDs to their documentation paths
#[derive(Debug, Default)]
pub struct LinkRegistry {
    /// Maps item ID to the markdown file path (relative to output dir)
    item_paths: HashMap<Id, String>,
    /// Maps item ID to the item name for display
    item_names: HashMap<Id, String>,
}

impl LinkRegistry {
    /// Build a link registry from a crate
    pub fn build(krate: &Crate, flat_format: bool) -> Self {
        let mut registry = Self::default();

        // Get root module
        let Some(root) = krate.index.get(&krate.root) else {
            return registry;
        };

        // Process root module items
        if let ItemEnum::Module(module) = &root.inner {
            for item_id in &module.items {
                if let Some(item) = krate.index.get(item_id) {
                    match &item.inner {
                        ItemEnum::Module(_) => {
                            let module_name = item.name.as_deref().unwrap_or("unnamed");
                            let path = if flat_format {
                                format!("{}.md", module_name)
                            } else {
                                format!("{}/index.md", module_name)
                            };
                            registry.register_module_items(
                                krate,
                                item_id,
                                item,
                                &path,
                                module_name,
                                flat_format,
                            );
                        }
                        ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Trait(_) => {
                            let name = item.name.as_deref().unwrap_or("unnamed");
                            registry
                                .item_paths
                                .insert(*item_id, "index.md".to_string());
                            registry
                                .item_names
                                .insert(*item_id, name.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }

        registry
    }

    /// Register items within a module
    fn register_module_items(
        &mut self,
        krate: &Crate,
        module_id: &Id,
        module_item: &rustdoc_types::Item,
        path: &str,
        module_prefix: &str,
        flat_format: bool,
    ) {
        // Register the module itself
        let module_name = module_item.name.as_deref().unwrap_or("unnamed");
        self.item_paths.insert(*module_id, path.to_string());
        self.item_names
            .insert(*module_id, module_name.to_string());

        if let ItemEnum::Module(module) = &module_item.inner {
            for item_id in &module.items {
                if let Some(item) = krate.index.get(item_id) {
                    let name = item.name.as_deref().unwrap_or("unnamed");

                    match &item.inner {
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
                        ItemEnum::Module(_) => {
                            let sub_path = if flat_format {
                                format!("{}__{}.md", module_prefix, name)
                            } else {
                                format!("{}/{}/index.md", module_prefix, name)
                            };
                            let sub_prefix = format!("{}__{}", module_prefix, name);
                            self.register_module_items(
                                krate,
                                item_id,
                                item,
                                &sub_path,
                                &sub_prefix,
                                flat_format,
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Get the path to an item's documentation file
    pub fn get_path(&self, id: &Id) -> Option<&String> {
        self.item_paths.get(id)
    }

    /// Get the display name for an item
    pub fn get_name(&self, id: &Id) -> Option<&String> {
        self.item_names.get(id)
    }

    /// Create a markdown link to an item from a given source file
    pub fn create_link(&self, id: &Id, from_path: &str) -> Option<String> {
        let target_path = self.item_paths.get(id)?;
        let name = self.item_names.get(id)?;

        // Calculate relative path from source to target
        let relative_path = compute_relative_path(from_path, target_path);

        // Add anchor if pointing to same file
        let link = if target_path == from_path {
            format!("#{}", name.to_lowercase().replace(' ', "-"))
        } else {
            relative_path
        };

        Some(format!("[`{}`]({})", name, link))
    }
}

/// Compute the relative path from one file to another
fn compute_relative_path(from: &str, to: &str) -> String {
    // Simple case: same directory or root
    if from == to {
        return String::new();
    }

    let from_parts: Vec<&str> = from.split('/').collect();
    let to_parts: Vec<&str> = to.split('/').collect();

    // Find common prefix length
    let common_len = from_parts
        .iter()
        .zip(to_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // Number of directories to go up (exclude the filename)
    let ups = from_parts
        .len()
        .saturating_sub(1)
        .saturating_sub(common_len);

    // Build relative path
    let mut result = String::new();
    for _ in 0..ups {
        result.push_str("../");
    }

    // Add remaining path components from target
    for (i, part) in to_parts.iter().enumerate().skip(common_len) {
        if i > common_len {
            result.push('/');
        }
        result.push_str(part);
    }

    if result.is_empty() {
        to.to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_path_same_dir() {
        assert_eq!(compute_relative_path("index.md", "span.md"), "span.md");
    }

    #[test]
    fn test_relative_path_child_dir() {
        assert_eq!(
            compute_relative_path("index.md", "span/index.md"),
            "span/index.md"
        );
    }

    #[test]
    fn test_relative_path_parent_dir() {
        assert_eq!(
            compute_relative_path("span/index.md", "index.md"),
            "../index.md"
        );
    }

    #[test]
    fn test_relative_path_sibling_dir() {
        assert_eq!(
            compute_relative_path("span/index.md", "field/index.md"),
            "../field/index.md"
        );
    }
}
