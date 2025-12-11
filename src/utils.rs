//! Shared utility functions used across the documentation generator.
//!
//! This module contains small, general-purpose helpers that are used
//! by multiple components and don't belong to any specific domain.
//!
//! # Organization
//!
//! Utilities are organized into unit structs by category:
//! - [`PathUtils`]: Rust path manipulation utilities

/// Utilities for working with Rust paths (e.g., `std::vec::Vec`).
///
/// Rust paths use `::` as a separator. This struct provides methods
/// for common path operations used throughout the documentation generator.
///
/// # Design Note
///
/// This is a unit struct (no fields) that serves as a namespace for
/// related utility functions. All methods are associated functions
/// that don't require an instance.
///
/// # Examples
///
/// ```
/// use docs_md::utils::PathUtils;
///
/// // Extract the short name from a qualified path
/// assert_eq!(PathUtils::short_name("std::vec::Vec"), "Vec");
/// assert_eq!(PathUtils::short_name("Clone"), "Clone");
/// ```
pub struct PathUtils;

impl PathUtils {
    /// Extract the short name (last segment) from a qualified Rust path.
    ///
    /// Rust paths use `::` as a separator. This function returns the final
    /// segment, which is typically the item's simple name without module prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use docs_md::utils::PathUtils;
    ///
    /// assert_eq!(PathUtils::short_name("std::vec::Vec"), "Vec");
    /// assert_eq!(PathUtils::short_name("std::collections::HashMap"), "HashMap");
    /// assert_eq!(PathUtils::short_name("Clone"), "Clone");
    /// assert_eq!(PathUtils::short_name(""), "");
    /// ```
    ///
    /// # Edge Cases
    ///
    /// - Empty string returns empty string
    /// - Path ending with `::` returns empty string (e.g., `"foo::"` -> `""`)
    /// - Single segment returns itself (e.g., `"Vec"` -> `"Vec"`)
    #[inline]
    #[must_use]
    pub fn short_name(path: &str) -> &str {
        // Using rsplit().next() is more efficient than split().last()
        // because it doesn't need to traverse the entire iterator
        path.rsplit("::").next().unwrap_or(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_name_qualified_path() {
        assert_eq!(PathUtils::short_name("std::vec::Vec"), "Vec");
        assert_eq!(PathUtils::short_name("std::collections::HashMap"), "HashMap");
        assert_eq!(PathUtils::short_name("tokio::sync::mpsc::Sender"), "Sender");
    }

    #[test]
    fn short_name_simple() {
        assert_eq!(PathUtils::short_name("Vec"), "Vec");
        assert_eq!(PathUtils::short_name("Clone"), "Clone");
        assert_eq!(PathUtils::short_name("u32"), "u32");
    }

    #[test]
    fn short_name_edge_cases() {
        assert_eq!(PathUtils::short_name(""), "");
        assert_eq!(PathUtils::short_name("::"), "");
        assert_eq!(PathUtils::short_name("foo::"), "");
        assert_eq!(PathUtils::short_name("::foo"), "foo");
    }

    #[test]
    fn short_name_with_turbofish() {
        // Turbofish syntax `Type::<T>` contains `::` so it gets split
        // This extracts the generic part `<T>` since `::` is the separator
        assert_eq!(PathUtils::short_name("Type::<T>"), "<T>");

        // A single colon is NOT a Rust path separator
        assert_eq!(PathUtils::short_name("Type:T"), "Type:T");
    }
}
