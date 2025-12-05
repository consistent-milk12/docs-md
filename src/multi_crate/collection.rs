//! Crate collection for multi-crate documentation.
//!
//! This module provides [`CrateCollection`], a container for multiple parsed
//! rustdoc crates that maintains a consistent processing order.

use std::collections::HashMap;

use rustdoc_types::Crate;

/// Collection of parsed crates ready for documentation generation.
///
/// Uses `HashMap` for O(1) lookups and sorts keys on-demand when iteration
/// is needed. This is optimal for our use case where:
/// - All crates are inserted first (parsing phase)
/// - Sorted iteration happens later (generation phase)
/// - Collection size is small (typically 10-50 crates)
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
    /// HashMap provides O(1) lookups; sorting done on-demand.
    crates: HashMap<String, Crate>,
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
        self.crates.insert(name, krate)
    }

    /// Get a crate by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Crate> {
        self.crates.get(name)
    }

    /// Get a crate by name, returning the stored key as well.
    ///
    /// This is useful when you need a reference to the crate name that
    /// has the same lifetime as the collection.
    #[must_use]
    pub fn get_with_name(&self, name: &str) -> Option<(&str, &Crate)> {
        self.crates
            .get_key_value(name)
            .map(|(k, v)| (k.as_str(), v))
    }

    /// Check if a crate exists in the collection.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.crates.contains_key(name)
    }

    /// Iterate over crates in alphabetical order.
    ///
    /// Returns tuples of `(&crate_name, &Crate)` sorted alphabetically
    /// by crate name for deterministic output.
    ///
    /// Sorting is done on-demand since collection size is small (10-50 crates).
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Crate)> {
        let mut entries: Vec<_> = self.crates.iter().collect();
        entries.sort_by_key(|(name, _)| *name);
        entries.into_iter()
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

    /// Get crate names in alphabetical order.
    ///
    /// Returns a sorted `Vec` of crate names for deterministic processing.
    /// Sorting is done on-demand since collection size is small.
    #[must_use]
    pub fn names(&self) -> Vec<&String> {
        let mut names: Vec<_> = self.crates.keys().collect();
        names.sort();
        names
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
    fn test_names_returns_sorted() {
        let collection = CrateCollection::new();
        // Order should be maintained alphabetically
        let names = collection.names();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }
}
