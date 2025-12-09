//! Categorization system for trait implementations.
//!
//! This module provides [`ImplCategory`], an enum that classifies trait implementations
//! into logical groups for better documentation organization. Rather than displaying
//! all trait impls in a flat alphabetical list, they can be grouped by purpose:
//!
//! - **Derive traits**: Common `#[derive(...)]` traits like `Clone`, `Debug`
//! - **Conversion traits**: Type conversions like `From`, `Into`, `AsRef`
//! - **Iterator traits**: Iteration support like `Iterator`, `IntoIterator`
//! - **Operator traits**: Operator overloading from `std::ops`
//! - And more...
//!
//! # Example Output
//!
//! When rendered, impl blocks are grouped into sections:
//!
//! ```markdown
//! #### Trait Implementations
//!
//! ##### Conversion
//! - `impl From<String> for MyType`
//! - `impl Into<String> for MyType`
//!
//! ##### Iterator
//! - `impl Iterator for MyType`
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use cargo_docs_md::generator::ImplCategory;
//!
//! // Categorize by trait path
//! let category = ImplCategory::from_trait_path(Some("std::clone::Clone"));
//! assert_eq!(category, ImplCategory::Derive);
//!
//! // Get display name for section headers
//! assert_eq!(category.display_name(), "Derived Traits");
//!
//! // Inherent impls have no trait
//! let inherent = ImplCategory::from_trait_path(None);
//! assert_eq!(inherent, ImplCategory::Inherent);
//! ```

use std::cmp::Ordering;

/// Logical category for trait implementations.
///
/// Each variant represents a group of related traits that serve a similar purpose.
/// This categorization enables documentation to be organized by functionality rather
/// than alphabetically, making it easier for users to find relevant implementations.
///
/// # Variant Order
///
/// The variants are ordered by their typical importance/frequency of use:
/// 1. `Inherent` - Direct methods on the type (most commonly referenced)
/// 2. `Derive` - Standard derived traits (`Clone`, `Debug`, etc.)
/// 3. `Conversion` - Type conversion traits (`From`, `Into`, etc.)
/// 4. `Access` - Smart pointer/indexing traits (`Deref`, `Index`)
/// 5. `Iterator` - Iteration support
/// 6. `Operator` - Operator overloading
/// 7. `Formatting` - Display/formatting traits
/// 8. `Io` - I/O traits (less common)
/// 9. `Other` - Catch-all for unrecognized traits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplCategory {
    /// Inherent implementations (no trait).
    ///
    /// These are methods defined directly on the type:
    /// ```rust,ignore
    /// impl MyType {
    ///     fn new() -> Self { ... }
    ///     fn method(&self) { ... }
    /// }
    /// ```
    Inherent,

    /// Common derived traits from `#[derive(...)]`.
    ///
    /// Includes: `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq`,
    /// `Hash`, `PartialOrd`, `Ord`.
    ///
    /// These traits have standard, predictable implementations that users
    /// typically don't need to examine in detail.
    Derive,

    /// Type conversion traits.
    ///
    /// Includes: `From`, `Into`, `TryFrom`, `TryInto`, `AsRef`, `AsMut`,
    /// `Borrow`, `BorrowMut`.
    ///
    /// These traits define how a type can be converted to/from other types,
    /// which is essential for understanding type interoperability.
    Conversion,

    /// Iterator-related traits.
    ///
    /// Includes: `Iterator`, `IntoIterator`, `FromIterator`, `Extend`,
    /// `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`.
    ///
    /// These traits define how a type participates in Rust's iteration ecosystem.
    Iterator,

    /// I/O traits from `std::io`.
    ///
    /// Includes: `Read`, `Write`, `Seek`, `BufRead`, `BufWrite`.
    ///
    /// These traits define how a type can be used for input/output operations.
    Io,

    /// Operator overloading traits from `std::ops`.
    ///
    /// Includes all arithmetic, bitwise, and compound assignment operators:
    /// `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`, `Not`, `BitAnd`, `BitOr`,
    /// `BitXor`, `Shl`, `Shr`, and their `*Assign` variants.
    ///
    /// These traits define custom behavior for Rust's operators (`+`, `-`, etc.).
    Operator,

    /// Smart pointer and indexing traits.
    ///
    /// Includes: `Deref`, `DerefMut`, `Index`, `IndexMut`.
    ///
    /// These traits define how a type can be dereferenced or indexed,
    /// which is crucial for wrapper types and collections.
    Access,

    /// Display and formatting traits.
    ///
    /// Includes: `Display`, `LowerHex`, `UpperHex`, `Octal`, `Binary`,
    /// `Pointer`, `LowerExp`, `UpperExp`.
    ///
    /// These traits define how a type is formatted for output. `Display` is
    /// particularly important as it defines user-facing string representation.
    Formatting,

    /// Any trait that doesn't fit other categories.
    ///
    /// This is the fallback for:
    /// - Third-party traits (e.g., `serde::Serialize`)
    /// - Less common std traits (e.g., `Drop`, `Send`, `Sync`)
    /// - Domain-specific traits
    Other,
}

// ============================================================================
// Trait name constants for categorization
// ============================================================================

/// Traits commonly derived via `#[derive(...)]`.
///
/// These have standard implementations that don't usually need documentation scrutiny.
const DERIVE_TRAITS: &[&str] = &[
    "Clone",
    "Copy",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "PartialOrd",
    "Ord",
];

/// Type conversion traits.
///
/// These define how types can be converted to/from each other.
const CONVERSION_TRAITS: &[&str] = &[
    "From",
    "Into",
    "TryFrom",
    "TryInto",
    "AsRef",
    "AsMut",
    "Borrow",
    "BorrowMut",
    "ToOwned",
];

/// Iterator-related traits.
///
/// These define participation in Rust's iteration ecosystem.
const ITERATOR_TRAITS: &[&str] = &[
    "Iterator",
    "IntoIterator",
    "FromIterator",
    "Extend",
    "DoubleEndedIterator",
    "ExactSizeIterator",
    "FusedIterator",
];

/// I/O traits from `std::io`.
const IO_TRAITS: &[&str] = &["Read", "Write", "Seek", "BufRead", "BufWrite"];

/// Operator overloading traits from `std::ops`.
///
/// Includes arithmetic, bitwise, and compound assignment operators.
const OPERATOR_TRAITS: &[&str] = &[
    // Arithmetic operators
    "Add",
    "Sub",
    "Mul",
    "Div",
    "Rem",
    "Neg",
    // Logical/bitwise operators
    "Not",
    "BitAnd",
    "BitOr",
    "BitXor",
    "Shl",
    "Shr",
    // Compound assignment operators
    "AddAssign",
    "SubAssign",
    "MulAssign",
    "DivAssign",
    "RemAssign",
    "BitAndAssign",
    "BitOrAssign",
    "BitXorAssign",
    "ShlAssign",
    "ShrAssign",
];

/// Smart pointer and indexing traits.
///
/// These define dereferencing and indexing behavior.
const ACCESS_TRAITS: &[&str] = &["Deref", "DerefMut", "Index", "IndexMut"];

/// Display and formatting traits from `std::fmt`.
///
/// These define how types are formatted for output.
const FORMATTING_TRAITS: &[&str] = &[
    "Display", "LowerHex", "UpperHex", "Octal", "Binary", "Pointer", "LowerExp", "UpperExp",
];

impl ImplCategory {
    /// Categorize a trait implementation by its trait path.
    ///
    /// This method examines the trait path and returns the appropriate category.
    /// It handles both simple trait names (`"Clone"`) and fully-qualified paths
    /// (`"std::clone::Clone"`).
    ///
    /// # Arguments
    ///
    /// * `path` - The trait path, or `None` for inherent implementations
    ///
    /// # Returns
    ///
    /// The [`ImplCategory`] that best matches the trait.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Inherent impl (no trait)
    /// assert_eq!(ImplCategory::from_trait_path(None), ImplCategory::Inherent);
    ///
    /// // Simple trait name
    /// assert_eq!(ImplCategory::from_trait_path(Some("Clone")), ImplCategory::Derive);
    ///
    /// // Fully-qualified path
    /// assert_eq!(
    ///     ImplCategory::from_trait_path(Some("std::clone::Clone")),
    ///     ImplCategory::Derive
    /// );
    ///
    /// // Operator from std::ops
    /// assert_eq!(
    ///     ImplCategory::from_trait_path(Some("std::ops::Add")),
    ///     ImplCategory::Operator
    /// );
    ///
    /// // Unknown trait
    /// assert_eq!(
    ///     ImplCategory::from_trait_path(Some("serde::Serialize")),
    ///     ImplCategory::Other
    /// );
    /// ```
    #[must_use]
    pub fn from_trait_path(path: Option<&str>) -> Self {
        // No trait path means inherent implementation
        let Some(path) = path else {
            return Self::Inherent;
        };

        // Extract the trait name (last segment of the path)
        // e.g., "std::clone::Clone" -> "Clone"
        // Using rsplit().next() is more efficient than split().last()
        let trait_name = path.rsplit("::").next().unwrap_or(path);

        // Check each category in order of specificity
        // Note: We check Operator before others because std::ops traits
        // might have generic names that could conflict

        // First, check if this is an std::ops trait by path prefix
        // This catches cases where we might miss a trait in our list
        if path.contains("std::ops::") || path.contains("core::ops::") {
            return Self::Operator;
        }

        // Check against our known trait lists
        if DERIVE_TRAITS.contains(&trait_name) {
            Self::Derive
        } else if CONVERSION_TRAITS.contains(&trait_name) {
            Self::Conversion
        } else if ITERATOR_TRAITS.contains(&trait_name) {
            Self::Iterator
        } else if IO_TRAITS.contains(&trait_name) {
            Self::Io
        } else if OPERATOR_TRAITS.contains(&trait_name) {
            Self::Operator
        } else if ACCESS_TRAITS.contains(&trait_name) {
            Self::Access
        } else if FORMATTING_TRAITS.contains(&trait_name) {
            Self::Formatting
        } else {
            Self::Other
        }
    }

    /// Get the human-readable display name for this category.
    ///
    /// This name is suitable for use as a section header in documentation.
    ///
    /// # Returns
    ///
    /// A static string with the display name.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// assert_eq!(ImplCategory::Inherent.display_name(), "Implementations");
    /// assert_eq!(ImplCategory::Derive.display_name(), "Derived Traits");
    /// assert_eq!(ImplCategory::Conversion.display_name(), "Conversion");
    /// ```
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Inherent => "Implementations",
            Self::Derive => "Derived Traits",
            Self::Conversion => "Conversion",
            Self::Iterator => "Iterator",
            Self::Io => "I/O",
            Self::Operator => "Operators",
            Self::Access => "Deref & Indexing",
            Self::Formatting => "Formatting",
            Self::Other => "Other Traits",
        }
    }

    /// Get the sort order for this category.
    ///
    /// Lower numbers appear first in documentation. This ordering reflects
    /// typical importance and frequency of use.
    ///
    /// # Returns
    ///
    /// A `u8` value representing the sort order (0-8).
    #[must_use]
    const fn sort_order(self) -> u8 {
        match self {
            Self::Inherent => 0,   // Most important: direct methods
            Self::Derive => 1,     // Common derived traits
            Self::Conversion => 2, // Type conversions
            Self::Access => 3,     // Deref/Index (often used)
            Self::Iterator => 4,   // Iteration support
            Self::Operator => 5,   // Operator overloading
            Self::Formatting => 6, // Display/formatting
            Self::Io => 7,         // I/O (less common)
            Self::Other => 8,      // Everything else (last)
        }
    }
}

// ============================================================================
// Ordering implementation
// ============================================================================

impl PartialOrd for ImplCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ImplCategory {
    /// Compare categories by their display order.
    ///
    /// Categories are ordered by typical importance/frequency:
    /// `Inherent` < `Derive` < `Conversion` < `Access` < `Iterator`
    /// < `Operator` < `Formatting` < `Io` < `Other`
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_order().cmp(&other.sort_order())
    }
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // from_trait_path tests
    // ------------------------------------------------------------------------

    mod from_trait_path {
        use super::*;

        #[test]
        fn none_returns_inherent() {
            assert_eq!(ImplCategory::from_trait_path(None), ImplCategory::Inherent);
        }

        #[test]
        fn empty_string_returns_other() {
            // Edge case: empty string should fall through to Other
            assert_eq!(ImplCategory::from_trait_path(Some("")), ImplCategory::Other);
        }

        // --- Derive traits ---

        #[test]
        fn clone_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Clone")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn copy_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Copy")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn debug_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Debug")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn default_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Default")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn partial_eq_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("PartialEq")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn eq_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Eq")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn hash_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Hash")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn partial_ord_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("PartialOrd")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn ord_is_derive() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Ord")),
                ImplCategory::Derive
            );
        }

        // --- Conversion traits ---

        #[test]
        fn from_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("From")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn into_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Into")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn try_from_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("TryFrom")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn try_into_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("TryInto")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn as_ref_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("AsRef")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn as_mut_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("AsMut")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn borrow_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Borrow")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn borrow_mut_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("BorrowMut")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn to_owned_is_conversion() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("ToOwned")),
                ImplCategory::Conversion
            );
        }

        // --- Iterator traits ---

        #[test]
        fn iterator_is_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Iterator")),
                ImplCategory::Iterator
            );
        }

        #[test]
        fn into_iterator_is_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("IntoIterator")),
                ImplCategory::Iterator
            );
        }

        #[test]
        fn from_iterator_is_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("FromIterator")),
                ImplCategory::Iterator
            );
        }

        #[test]
        fn extend_is_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Extend")),
                ImplCategory::Iterator
            );
        }

        #[test]
        fn double_ended_iterator_is_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("DoubleEndedIterator")),
                ImplCategory::Iterator
            );
        }

        #[test]
        fn exact_size_iterator_is_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("ExactSizeIterator")),
                ImplCategory::Iterator
            );
        }

        #[test]
        fn fused_iterator_is_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("FusedIterator")),
                ImplCategory::Iterator
            );
        }

        // --- I/O traits ---

        #[test]
        fn read_is_io() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Read")),
                ImplCategory::Io
            );
        }

        #[test]
        fn write_is_io() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Write")),
                ImplCategory::Io
            );
        }

        #[test]
        fn seek_is_io() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Seek")),
                ImplCategory::Io
            );
        }

        #[test]
        fn buf_read_is_io() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("BufRead")),
                ImplCategory::Io
            );
        }

        #[test]
        fn buf_write_is_io() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("BufWrite")),
                ImplCategory::Io
            );
        }

        // --- Operator traits ---

        #[test]
        fn add_is_operator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Add")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn sub_is_operator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Sub")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn mul_is_operator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Mul")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn div_is_operator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Div")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn neg_is_operator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Neg")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn bit_and_is_operator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("BitAnd")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn add_assign_is_operator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("AddAssign")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn std_ops_path_is_operator() {
            // Full path from std::ops should be detected
            assert_eq!(
                ImplCategory::from_trait_path(Some("std::ops::Add")),
                ImplCategory::Operator
            );
        }

        #[test]
        fn core_ops_path_is_operator() {
            // Full path from core::ops should be detected
            assert_eq!(
                ImplCategory::from_trait_path(Some("core::ops::Sub")),
                ImplCategory::Operator
            );
        }

        // --- Access traits ---

        #[test]
        fn deref_is_access() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Deref")),
                ImplCategory::Access
            );
        }

        #[test]
        fn deref_mut_is_access() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("DerefMut")),
                ImplCategory::Access
            );
        }

        #[test]
        fn index_is_access() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Index")),
                ImplCategory::Access
            );
        }

        #[test]
        fn index_mut_is_access() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("IndexMut")),
                ImplCategory::Access
            );
        }

        // --- Formatting traits ---

        #[test]
        fn display_is_formatting() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Display")),
                ImplCategory::Formatting
            );
        }

        #[test]
        fn lower_hex_is_formatting() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("LowerHex")),
                ImplCategory::Formatting
            );
        }

        #[test]
        fn upper_hex_is_formatting() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("UpperHex")),
                ImplCategory::Formatting
            );
        }

        #[test]
        fn octal_is_formatting() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Octal")),
                ImplCategory::Formatting
            );
        }

        #[test]
        fn binary_is_formatting() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Binary")),
                ImplCategory::Formatting
            );
        }

        #[test]
        fn pointer_is_formatting() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Pointer")),
                ImplCategory::Formatting
            );
        }

        // --- Full paths ---

        #[test]
        fn full_path_clone() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("std::clone::Clone")),
                ImplCategory::Derive
            );
        }

        #[test]
        fn full_path_from() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("std::convert::From")),
                ImplCategory::Conversion
            );
        }

        #[test]
        fn full_path_iterator() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("std::iter::Iterator")),
                ImplCategory::Iterator
            );
        }

        #[test]
        fn full_path_display() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("std::fmt::Display")),
                ImplCategory::Formatting
            );
        }

        #[test]
        fn full_path_read() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("std::io::Read")),
                ImplCategory::Io
            );
        }

        // --- Other/Unknown ---

        #[test]
        fn unknown_trait_is_other() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("UnknownTrait")),
                ImplCategory::Other
            );
        }

        #[test]
        fn serde_serialize_is_other() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("serde::Serialize")),
                ImplCategory::Other
            );
        }

        #[test]
        fn custom_trait_is_other() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("my_crate::MyTrait")),
                ImplCategory::Other
            );
        }

        #[test]
        fn send_is_other() {
            // Marker traits go to Other
            assert_eq!(
                ImplCategory::from_trait_path(Some("Send")),
                ImplCategory::Other
            );
        }

        #[test]
        fn sync_is_other() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Sync")),
                ImplCategory::Other
            );
        }

        #[test]
        fn drop_is_other() {
            assert_eq!(
                ImplCategory::from_trait_path(Some("Drop")),
                ImplCategory::Other
            );
        }
    }

    // ------------------------------------------------------------------------
    // display_name tests
    // ------------------------------------------------------------------------

    mod display_name {
        use super::*;

        #[test]
        fn inherent_display_name() {
            assert_eq!(ImplCategory::Inherent.display_name(), "Implementations");
        }

        #[test]
        fn derive_display_name() {
            assert_eq!(ImplCategory::Derive.display_name(), "Derived Traits");
        }

        #[test]
        fn conversion_display_name() {
            assert_eq!(ImplCategory::Conversion.display_name(), "Conversion");
        }

        #[test]
        fn iterator_display_name() {
            assert_eq!(ImplCategory::Iterator.display_name(), "Iterator");
        }

        #[test]
        fn io_display_name() {
            assert_eq!(ImplCategory::Io.display_name(), "I/O");
        }

        #[test]
        fn operator_display_name() {
            assert_eq!(ImplCategory::Operator.display_name(), "Operators");
        }

        #[test]
        fn access_display_name() {
            assert_eq!(ImplCategory::Access.display_name(), "Deref & Indexing");
        }

        #[test]
        fn formatting_display_name() {
            assert_eq!(ImplCategory::Formatting.display_name(), "Formatting");
        }

        #[test]
        fn other_display_name() {
            assert_eq!(ImplCategory::Other.display_name(), "Other Traits");
        }
    }

    // ------------------------------------------------------------------------
    // Ordering tests
    // ------------------------------------------------------------------------

    mod ordering {
        use super::*;

        #[test]
        fn inherent_is_first() {
            assert!(ImplCategory::Inherent < ImplCategory::Derive);
            assert!(ImplCategory::Inherent < ImplCategory::Other);
        }

        #[test]
        fn other_is_last() {
            assert!(ImplCategory::Other > ImplCategory::Inherent);
            assert!(ImplCategory::Other > ImplCategory::Derive);
            assert!(ImplCategory::Other > ImplCategory::Io);
        }

        #[test]
        fn derive_before_conversion() {
            assert!(ImplCategory::Derive < ImplCategory::Conversion);
        }

        #[test]
        fn conversion_before_access() {
            assert!(ImplCategory::Conversion < ImplCategory::Access);
        }

        #[test]
        fn access_before_iterator() {
            assert!(ImplCategory::Access < ImplCategory::Iterator);
        }

        #[test]
        fn iterator_before_operator() {
            assert!(ImplCategory::Iterator < ImplCategory::Operator);
        }

        #[test]
        fn operator_before_formatting() {
            assert!(ImplCategory::Operator < ImplCategory::Formatting);
        }

        #[test]
        fn formatting_before_io() {
            assert!(ImplCategory::Formatting < ImplCategory::Io);
        }

        #[test]
        fn io_before_other() {
            assert!(ImplCategory::Io < ImplCategory::Other);
        }

        #[test]
        fn full_ordering() {
            // Test the complete ordering chain
            let mut categories = vec![
                ImplCategory::Other,
                ImplCategory::Formatting,
                ImplCategory::Inherent,
                ImplCategory::Operator,
                ImplCategory::Derive,
                ImplCategory::Io,
                ImplCategory::Iterator,
                ImplCategory::Access,
                ImplCategory::Conversion,
            ];

            categories.sort();

            assert_eq!(
                categories,
                vec![
                    ImplCategory::Inherent,
                    ImplCategory::Derive,
                    ImplCategory::Conversion,
                    ImplCategory::Access,
                    ImplCategory::Iterator,
                    ImplCategory::Operator,
                    ImplCategory::Formatting,
                    ImplCategory::Io,
                    ImplCategory::Other,
                ]
            );
        }

        #[test]
        fn same_category_is_equal() {
            assert_eq!(
                ImplCategory::Derive.cmp(&ImplCategory::Derive),
                Ordering::Equal
            );
        }
    }

    // ------------------------------------------------------------------------
    // Trait implementation tests
    // ------------------------------------------------------------------------

    mod traits {
        use std::collections::HashSet;

        use super::*;

        #[test]
        fn category_is_copy() {
            let cat = ImplCategory::Derive;
            let cat2 = cat; // Copy
            assert_eq!(cat, cat2);
        }

        #[test]
        fn category_is_clone() {
            let cat = ImplCategory::Iterator;
            let cat2 = cat;
            assert_eq!(cat, cat2);
        }

        #[test]
        fn category_is_hashable() {
            let mut set = HashSet::new();
            set.insert(ImplCategory::Derive);
            set.insert(ImplCategory::Conversion);
            set.insert(ImplCategory::Derive); // Duplicate

            assert_eq!(set.len(), 2);
            assert!(set.contains(&ImplCategory::Derive));
            assert!(set.contains(&ImplCategory::Conversion));
        }

        #[test]
        fn category_debug_format() {
            let debug_str = format!("{:?}", ImplCategory::Formatting);
            assert_eq!(debug_str, "Formatting");
        }
    }
}
