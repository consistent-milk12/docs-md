*[miette](../index.md) / [protocol](index.md)*

---

# Module `protocol`

This module defines the core of the miette protocol: a series of types and
traits that you can implement to get access to miette's (and related library's)
full reporting and such features.

## Structs

### `LabeledSpan`

```rust
struct LabeledSpan {
    label: Option<String>,
    span: SourceSpan,
    primary: bool,
}
```

A labeled [`SourceSpan`](../index.md).

#### Implementations

- `const fn new(label: Option<String>, offset: ByteOffset, len: usize) -> Self` — [`ByteOffset`](../index.md)

- `fn new_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md)

- `fn new_primary_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md)

- `fn set_label(self: &mut Self, label: Option<String>)`

- `fn at(span: impl Into<SourceSpan>, label: impl Into<String>) -> Self` — [`SourceSpan`](../index.md)

- `fn at_offset(offset: ByteOffset, label: impl Into<String>) -> Self` — [`ByteOffset`](../index.md)

- `fn underline(span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md)

- `fn label(self: &Self) -> Option<&str>`

- `const fn inner(self: &Self) -> &SourceSpan` — [`SourceSpan`](../index.md)

- `const fn offset(self: &Self) -> usize`

- `const fn len(self: &Self) -> usize`

- `const fn is_empty(self: &Self) -> bool`

- `const fn primary(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for LabeledSpan`

- `fn clone(self: &Self) -> LabeledSpan` — [`LabeledSpan`](../index.md)

##### `impl Debug for LabeledSpan`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LabeledSpan`

##### `impl<D> OwoColorize for LabeledSpan`

##### `impl PartialEq for LabeledSpan`

- `fn eq(self: &Self, other: &LabeledSpan) -> bool` — [`LabeledSpan`](../index.md)

##### `impl StructuralPartialEq for LabeledSpan`

### `MietteSpanContents<'a>`

```rust
struct MietteSpanContents<'a> {
    data: &'a [u8],
    span: SourceSpan,
    line: usize,
    column: usize,
    line_count: usize,
    name: Option<String>,
    language: Option<String>,
}
```

Basic implementation of the [`SpanContents`](../index.md) trait, for convenience.

#### Implementations

- `const fn new(data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](../index.md), [`MietteSpanContents`](../index.md)

- `const fn new_named(name: String, data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](../index.md), [`MietteSpanContents`](../index.md)

- `fn with_language(self: Self, language: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl<'a> Clone for MietteSpanContents<'a>`

- `fn clone(self: &Self) -> MietteSpanContents<'a>` — [`MietteSpanContents`](../index.md)

##### `impl<'a> Debug for MietteSpanContents<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for MietteSpanContents<'a>`

##### `impl<'a> SpanContents for MietteSpanContents<'a>`

- `fn data(self: &Self) -> &'a [u8]`

- `fn span(self: &Self) -> &SourceSpan` — [`SourceSpan`](../index.md)

- `fn line(self: &Self) -> usize`

- `fn column(self: &Self) -> usize`

- `fn line_count(self: &Self) -> usize`

- `fn name(self: &Self) -> Option<&str>`

- `fn language(self: &Self) -> Option<&str>`

### `SourceSpan`

```rust
struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}
```

Span within a [`SourceCode`](../index.md)

#### Fields

- **`offset`**: `SourceOffset`

  The start of the span.

- **`length`**: `usize`

  The total length of the span

#### Implementations

- `const fn new(start: SourceOffset, length: usize) -> Self` — [`SourceOffset`](../index.md)

- `const fn offset(self: &Self) -> usize`

- `const fn len(self: &Self) -> usize`

- `const fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for SourceSpan`

- `fn clone(self: &Self) -> SourceSpan` — [`SourceSpan`](../index.md)

##### `impl Copy for SourceSpan`

##### `impl Debug for SourceSpan`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SourceSpan`

##### `impl Hash for SourceSpan`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for SourceSpan`

- `fn cmp(self: &Self, other: &SourceSpan) -> $crate::cmp::Ordering` — [`SourceSpan`](../index.md)

##### `impl<D> OwoColorize for SourceSpan`

##### `impl PartialEq for SourceSpan`

- `fn eq(self: &Self, other: &SourceSpan) -> bool` — [`SourceSpan`](../index.md)

##### `impl PartialOrd for SourceSpan`

- `fn partial_cmp(self: &Self, other: &SourceSpan) -> $crate::option::Option<$crate::cmp::Ordering>` — [`SourceSpan`](../index.md)

##### `impl StructuralPartialEq for SourceSpan`

### `SourceOffset`

```rust
struct SourceOffset(ByteOffset);
```

Newtype that represents the [`ByteOffset`](../index.md) from the beginning of a [`SourceCode`](../index.md)

#### Implementations

- `const fn offset(self: &Self) -> ByteOffset` — [`ByteOffset`](../index.md)

- `fn from_location(source: impl AsRef<str>, loc_line: usize, loc_col: usize) -> Self`

- `fn from_current_location() -> Result<(String, Self), MietteError>` — [`MietteError`](../index.md)

#### Trait Implementations

##### `impl Clone for SourceOffset`

- `fn clone(self: &Self) -> SourceOffset` — [`SourceOffset`](../index.md)

##### `impl Copy for SourceOffset`

##### `impl Debug for SourceOffset`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SourceOffset`

##### `impl Hash for SourceOffset`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for SourceOffset`

- `fn cmp(self: &Self, other: &SourceOffset) -> $crate::cmp::Ordering` — [`SourceOffset`](../index.md)

##### `impl<D> OwoColorize for SourceOffset`

##### `impl PartialEq for SourceOffset`

- `fn eq(self: &Self, other: &SourceOffset) -> bool` — [`SourceOffset`](../index.md)

##### `impl PartialOrd for SourceOffset`

- `fn partial_cmp(self: &Self, other: &SourceOffset) -> $crate::option::Option<$crate::cmp::Ordering>` — [`SourceOffset`](../index.md)

##### `impl StructuralPartialEq for SourceOffset`

## Enums

### `Severity`

```rust
enum Severity {
    Advice,
    Warning,
    Error,
}
```

[`Diagnostic`](../index.md) severity. Intended to be used by
[`ReportHandler`](crate::ReportHandler)s to change the way different
[`Diagnostic`](../index.md)s are displayed. Defaults to [`Severity::Error`](../index.md).

#### Variants

- **`Advice`**

  Just some help. Here's how you could be doing it better.

- **`Warning`**

  Warning. Please take note.

- **`Error`**

  Critical failure. The program cannot continue.
  This is the default severity, if you don't specify another one.

#### Trait Implementations

##### `impl Clone for Severity`

- `fn clone(self: &Self) -> Severity` — [`Severity`](../index.md)

##### `impl Copy for Severity`

##### `impl Debug for Severity`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Severity`

- `fn default() -> Severity` — [`Severity`](../index.md)

##### `impl Eq for Severity`

##### `impl Ord for Severity`

- `fn cmp(self: &Self, other: &Severity) -> $crate::cmp::Ordering` — [`Severity`](../index.md)

##### `impl<D> OwoColorize for Severity`

##### `impl PartialEq for Severity`

- `fn eq(self: &Self, other: &Severity) -> bool` — [`Severity`](../index.md)

##### `impl PartialOrd for Severity`

- `fn partial_cmp(self: &Self, other: &Severity) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Severity`](../index.md)

##### `impl StructuralPartialEq for Severity`

## Traits

### `Diagnostic`

```rust
trait Diagnostic: std::error::Error { ... }
```

Adds rich metadata to your Error that can be used by
[`Report`](crate::Report) to print really nice and human-friendly error
messages.

#### Required Methods

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  Unique diagnostic code that can be used to look up more information

- `fn severity(self: &Self) -> Option<Severity>`

  Diagnostic severity. This may be used by

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  Additional help text related to this `Diagnostic`. Do you have any

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  URL to visit for a more detailed explanation/help about this

- `fn source_code(self: &Self) -> Option<&dyn SourceCode>`

  Source code to apply this `Diagnostic`'s `Diagnostic::labels` to.

- `fn labels(self: &Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>`

  Labels to apply to this `Diagnostic`'s `Diagnostic::source_code`

- `fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>`

  Additional related `Diagnostic`s.

- `fn diagnostic_source(self: &Self) -> Option<&dyn Diagnostic>`

  The cause of the error.

### `SourceCode`

```rust
trait SourceCode: Send + Sync { ... }
```

Represents readable source code of some sort.

This trait is able to support simple `SourceCode` types like [`String`](../../clap_builder/index.md)s, as
well as more involved types like indexes into centralized `SourceMap`-like
types, file handles, and even network streams.

If you can read it, you can source it, and it's not necessary to read the
whole thing--meaning you should be able to support `SourceCode`s which are
gigabytes or larger in size.

#### Required Methods

- `fn read_span<'a>(self: &'a Self, span: &SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<Box<dyn SpanContents<'a>>, MietteError>`

  Read the bytes for a specific span from this `SourceCode`, keeping a

### `SpanContents<'a>`

```rust
trait SpanContents<'a> { ... }
```

Contents of a [`SourceCode`](../index.md) covered by [`SourceSpan`](../index.md).

Includes line and column information to optimize highlight calculations.

#### Required Methods

- `fn data(self: &Self) -> &'a [u8]`

  Reference to the data inside the associated span, in bytes.

- `fn span(self: &Self) -> &SourceSpan`

  [`SourceSpan`](../index.md) representing the span covered by this `SpanContents`.

- `fn name(self: &Self) -> Option<&str>`

  An optional (file?) name for the container of this `SpanContents`.

- `fn line(self: &Self) -> usize`

  The 0-indexed line in the associated [`SourceCode`](../index.md) where the data

- `fn column(self: &Self) -> usize`

  The 0-indexed column in the associated [`SourceCode`](../index.md) where the data

- `fn line_count(self: &Self) -> usize`

  Total number of lines covered by this `SpanContents`.

- `fn language(self: &Self) -> Option<&str>`

  Optional method. The language name for this source code, if any.

## Type Aliases

### `ByteOffset`

```rust
type ByteOffset = usize;
```

"Raw" type for the byte offset from the beginning of a [`SourceCode`](../index.md).

## Macros

### `box_error_impls!`

### `box_borrow_impls!`

