*[cargo_docs_md](../../index.md) / [generator](../index.md) / [impl_category](index.md)*

---

# Module `impl_category`

Categorization system for trait implementations.

This module provides [`ImplCategory`](#implcategory), an enum that classifies trait implementations
into logical groups for better documentation organization. Rather than displaying
all trait impls in a flat alphabetical list, they can be grouped by purpose:

- **Derive traits**: Common `#[derive(...)]` traits like `Clone`, `Debug`
- **Conversion traits**: Type conversions like `From`, `Into`, `AsRef`
- **Iterator traits**: Iteration support like `Iterator`, `IntoIterator`
- **Operator traits**: Operator overloading from `std::ops`
- And more...

# Example Output

When rendered, impl blocks are grouped into sections:

```markdown
#### Trait Implementations

##### Conversion
- `impl From<String> for MyType`
- `impl Into<String> for MyType`

##### Iterator
- `impl Iterator for MyType`
```

# Usage

```rust,ignore
use cargo_docs_md::generator::ImplCategory;

// Categorize by trait path
let category = ImplCategory::from_trait_path(Some("std::clone::Clone"));
assert_eq!(category, ImplCategory::Derive);

// Get display name for section headers
assert_eq!(category.display_name(), "Derived Traits");

// Inherent impls have no trait
let inherent = ImplCategory::from_trait_path(None);
assert_eq!(inherent, ImplCategory::Inherent);
```

## Contents

- [Enums](#enums)
  - [`ImplCategory`](#implcategory)
- [Constants](#constants)
  - [`DERIVE_TRAITS`](#derive_traits)
  - [`CONVERSION_TRAITS`](#conversion_traits)
  - [`ITERATOR_TRAITS`](#iterator_traits)
  - [`IO_TRAITS`](#io_traits)
  - [`OPERATOR_TRAITS`](#operator_traits)
  - [`ACCESS_TRAITS`](#access_traits)
  - [`FORMATTING_TRAITS`](#formatting_traits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImplCategory`](#implcategory) | enum | Logical category for trait implementations. |
| [`DERIVE_TRAITS`](#derive_traits) | const | Traits commonly derived via `#[derive(...)]`. |
| [`CONVERSION_TRAITS`](#conversion_traits) | const | Type conversion traits. |
| [`ITERATOR_TRAITS`](#iterator_traits) | const | Iterator-related traits. |
| [`IO_TRAITS`](#io_traits) | const | I/O traits from `std::io`. |
| [`OPERATOR_TRAITS`](#operator_traits) | const | Operator overloading traits from `std::ops`. |
| [`ACCESS_TRAITS`](#access_traits) | const | Smart pointer and indexing traits. |
| [`FORMATTING_TRAITS`](#formatting_traits) | const | Display and formatting traits from `std::fmt`. |

## Enums

### `ImplCategory`

```rust
enum ImplCategory {
    Inherent,
    Derive,
    Conversion,
    Iterator,
    Io,
    Operator,
    Access,
    Formatting,
    Other,
}
```

*Defined in `src/generator/impl_category.rs:66-144`*

Logical category for trait implementations.

Each variant represents a group of related traits that serve a similar purpose.
This categorization enables documentation to be organized by functionality rather
than alphabetically, making it easier for users to find relevant implementations.

# Variant Order

The variants are ordered by their typical importance/frequency of use:
1. `Inherent` - Direct methods on the type (most commonly referenced)
2. `Derive` - Standard derived traits (`Clone`, `Debug`, etc.)
3. `Conversion` - Type conversion traits (`From`, `Into`, etc.)
4. `Access` - Smart pointer/indexing traits (`Deref`, `Index`)
5. `Iterator` - Iteration support
6. `Operator` - Operator overloading
7. `Formatting` - Display/formatting traits
8. `Io` - I/O traits (less common)
9. `Other` - Catch-all for unrecognized traits

#### Variants

- **`Inherent`**

  Inherent implementations (no trait).
  
  These are methods defined directly on the type:
  ```rust,ignore
  impl MyType {
      fn new() -> Self { ... }
      fn method(&self) { ... }
  }
  ```

- **`Derive`**

  Common derived traits from `#[derive(...)]`.
  
  Includes: `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq`,
  `Hash`, `PartialOrd`, `Ord`.
  
  These traits have standard, predictable implementations that users
  typically don't need to examine in detail.

- **`Conversion`**

  Type conversion traits.
  
  Includes: `From`, `Into`, `TryFrom`, `TryInto`, `AsRef`, `AsMut`,
  `Borrow`, `BorrowMut`.
  
  These traits define how a type can be converted to/from other types,
  which is essential for understanding type interoperability.

- **`Iterator`**

  Iterator-related traits.
  
  Includes: `Iterator`, `IntoIterator`, `FromIterator`, `Extend`,
  `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`.
  
  These traits define how a type participates in Rust's iteration ecosystem.

- **`Io`**

  I/O traits from `std::io`.
  
  Includes: `Read`, `Write`, `Seek`, `BufRead`, `BufWrite`.
  
  These traits define how a type can be used for input/output operations.

- **`Operator`**

  Operator overloading traits from `std::ops`.
  
  Includes all arithmetic, bitwise, and compound assignment operators:
  `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`, `Not`, `BitAnd`, `BitOr`,
  `BitXor`, `Shl`, `Shr`, and their `*Assign` variants.
  
  These traits define custom behavior for Rust's operators (`+`, `-`, etc.).

- **`Access`**

  Smart pointer and indexing traits.
  
  Includes: `Deref`, `DerefMut`, `Index`, `IndexMut`.
  
  These traits define how a type can be dereferenced or indexed,
  which is crucial for wrapper types and collections.

- **`Formatting`**

  Display and formatting traits.
  
  Includes: `Display`, `LowerHex`, `UpperHex`, `Octal`, `Binary`,
  `Pointer`, `LowerExp`, `UpperExp`.
  
  These traits define how a type is formatted for output. `Display` is
  particularly important as it defines user-facing string representation.

- **`Other`**

  Any trait that doesn't fit other categories.
  
  This is the fallback for:
  - Third-party traits (e.g., `serde::Serialize`)
  - Less common std traits (e.g., `Drop`, `Send`, `Sync`)
  - Domain-specific traits

#### Implementations

- <span id="implcategory-from-trait-path"></span>`fn from_trait_path(path: Option<&str>) -> Self`

- <span id="implcategory-display-name"></span>`const fn display_name(&self) -> &'static str`

- <span id="implcategory-sort-order"></span>`const fn sort_order(self) -> u8`

#### Trait Implementations

##### `impl Clone for ImplCategory`

- <span id="implcategory-clone"></span>`fn clone(&self) -> ImplCategory` — [`ImplCategory`](#implcategory)

##### `impl Comparable for ImplCategory`

- <span id="implcategory-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for ImplCategory`

##### `impl Debug for ImplCategory`

- <span id="implcategory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImplCategory`

##### `impl Equivalent for ImplCategory`

- <span id="implcategory-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ImplCategory`

- <span id="implcategory-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Instrument for ImplCategory`

##### `impl IntoEither for ImplCategory`

##### `impl Ord for ImplCategory`

- <span id="implcategory-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl OwoColorize for ImplCategory`

##### `impl PartialEq for ImplCategory`

- <span id="implcategory-eq"></span>`fn eq(&self, other: &ImplCategory) -> bool` — [`ImplCategory`](#implcategory)

##### `impl PartialOrd for ImplCategory`

- <span id="implcategory-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Pointable for ImplCategory`

- <span id="implcategory-const-align"></span>`const ALIGN: usize`

- <span id="implcategory-type-init"></span>`type Init = T`

- <span id="implcategory-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implcategory-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implcategory-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implcategory-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for ImplCategory`

##### `impl WithSubscriber for ImplCategory`

## Constants

### `DERIVE_TRAITS`
```rust
const DERIVE_TRAITS: &[&str];
```

*Defined in `src/generator/impl_category.rs:153-163`*

Traits commonly derived via `#[derive(...)]`.

These have standard implementations that don't usually need documentation scrutiny.

### `CONVERSION_TRAITS`
```rust
const CONVERSION_TRAITS: &[&str];
```

*Defined in `src/generator/impl_category.rs:168-178`*

Type conversion traits.

These define how types can be converted to/from each other.

### `ITERATOR_TRAITS`
```rust
const ITERATOR_TRAITS: &[&str];
```

*Defined in `src/generator/impl_category.rs:183-191`*

Iterator-related traits.

These define participation in Rust's iteration ecosystem.

### `IO_TRAITS`
```rust
const IO_TRAITS: &[&str];
```

*Defined in `src/generator/impl_category.rs:194`*

I/O traits from `std::io`.

### `OPERATOR_TRAITS`
```rust
const OPERATOR_TRAITS: &[&str];
```

*Defined in `src/generator/impl_category.rs:199-225`*

Operator overloading traits from `std::ops`.

Includes arithmetic, bitwise, and compound assignment operators.

### `ACCESS_TRAITS`
```rust
const ACCESS_TRAITS: &[&str];
```

*Defined in `src/generator/impl_category.rs:230`*

Smart pointer and indexing traits.

These define dereferencing and indexing behavior.

### `FORMATTING_TRAITS`
```rust
const FORMATTING_TRAITS: &[&str];
```

*Defined in `src/generator/impl_category.rs:235-237`*

Display and formatting traits from `std::fmt`.

These define how types are formatted for output.

