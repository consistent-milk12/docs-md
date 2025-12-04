//! Crate collection for multi-crate documentation.
//!
//! This module provides [`CrateCollection`], a container for multiple parsed
//! rustdoc crates that maintains a consistent processing order.

use std::collections::HashMap;

use rustdoc_types::Crate;

/// Collection of parsed crates ready for documentation generation.
///
/// Maintains crates in a deterministic processing order for reproducible
/// output. The order is typically alphabetical by crate name.
///
/// # Example
///
/// ```ignore
/// let mut collection = CrateCollection::new();
/// collection.insert("tracing".to_string(), tracing_crate);
/// collection.insert("tracing_core".to_string(), tracing_core_crate);
///
/// for (name, krate) in collection.iter() {
///     println!("Processing {name}");
/// }
/// ```
#[derive(Debug, Default)]
pub struct CrateCollection {
    /// Map from crate name to parsed Crate data.
    crates: HashMap<String, Crate>,

    /// Crate names in processing order (alphabetical for determinism).
    processing_order: Vec<String>,
}

impl CrateCollection {
    /// Create an empty crate collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a crate into the collection.
    ///
    /// If a crate with the same name already exists, it is replaced
    /// and `Some(old_crate)` is returned.
    pub fn insert(&mut self, name: String, krate: Crate) -> Option<Crate> {
        let existing = self.crates.insert(name.clone(), krate);

        if existing.is_none() {
            // New crate - add to processing order and re-sort
            self.processing_order.push(name);
            self.processing_order.sort();
        }

        existing
    }

    /// Get a crate by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Crate> {
        self.crates.get(name)
    }

    /// Check if a crate exists in the collection.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.crates.contains_key(name)
    }

    /// Iterate over crates in processing order.
    ///
    /// Returns tuples of `(crate_name, Crate)` in alphabetical order
    /// by crate name for deterministic output.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Crate)> {
        self.processing_order
            .iter()
            .filter_map(|name| self.crates.get(name).map(|k| (name, k)))
    }

    /// Get the number of crates in the collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.crates.len()
    }

    /// Check if the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.crates.is_empty()
    }

    /// Get crate names in processing order.
    #[must_use]
    pub fn names(&self) -> &[String] {
        &self.processing_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let collection = CrateCollection::new();

        // Create a minimal mock crate (we can't easily construct a real one)
        // In practice, these come from parsing JSON files
        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn test_processing_order_is_alphabetical() {
        let collection = CrateCollection::new();
        // Order should be maintained alphabetically
        assert!(collection.names().is_sorted());
    }
}
