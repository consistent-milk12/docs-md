*[miette](../index.md) / [protocol](index.md)*

---

# Module `protocol`

This module defines the core of the miette protocol: a series of types and
traits that you can implement to get access to miette's (and related library's)
full reporting and such features.

## Contents

- [Structs](#structs)
  - [`LabeledSpan`](#labeledspan)
  - [`MietteSpanContents`](#miettespancontents)
  - [`SourceSpan`](#sourcespan)
  - [`SourceOffset`](#sourceoffset)
- [Enums](#enums)
  - [`Severity`](#severity)
- [Traits](#traits)
  - [`Diagnostic`](#diagnostic)
  - [`SourceCode`](#sourcecode)
  - [`SpanContents`](#spancontents)
- [Type Aliases](#type-aliases)
  - [`ByteOffset`](#byteoffset)
- [Macros](#macros)
  - [`box_error_impls!`](#box_error_impls)
  - [`box_borrow_impls!`](#box_borrow_impls)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LabeledSpan`](#labeledspan) | struct | A labeled [`SourceSpan`]. |
| [`MietteSpanContents`](#miettespancontents) | struct | Basic implementation of the [`SpanContents`] trait, for convenience. |
| [`SourceSpan`](#sourcespan) | struct | Span within a [`SourceCode`] |
| [`SourceOffset`](#sourceoffset) | struct | Newtype that represents the [`ByteOffset`] from the beginning of a [`SourceCode`] |
| [`Severity`](#severity) | enum | [`Diagnostic`] severity. |
| [`Diagnostic`](#diagnostic) | trait | Adds rich metadata to your Error that can be used by [`Report`](crate::Report) to print really nice and human-friendly error messages. |
| [`SourceCode`](#sourcecode) | trait | Represents readable source code of some sort. |
| [`SpanContents`](#spancontents) | trait | Contents of a [`SourceCode`] covered by [`SourceSpan`]. |
| [`ByteOffset`](#byteoffset) | type | "Raw" type for the byte offset from the beginning of a [`SourceCode`]. |
| [`box_error_impls!`](#box_error_impls) | macro |  |
| [`box_borrow_impls!`](#box_borrow_impls) | macro |  |

## Structs

### `LabeledSpan`

```rust
struct LabeledSpan {
    label: Option<String>,
    span: SourceSpan,
    primary: bool,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:250-255`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L250-L255)*

A labeled [`SourceSpan`](../index.md).

#### Implementations

- <span id="labeledspan-new"></span>`const fn new(label: Option<String>, offset: ByteOffset, len: usize) -> Self` — [`ByteOffset`](../index.md)

- <span id="labeledspan-new-with-span"></span>`fn new_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md)

- <span id="labeledspan-new-primary-with-span"></span>`fn new_primary_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md)

- <span id="labeledspan-set-label"></span>`fn set_label(&mut self, label: Option<String>)`

- <span id="labeledspan-at"></span>`fn at(span: impl Into<SourceSpan>, label: impl Into<String>) -> Self` — [`SourceSpan`](../index.md)

- <span id="labeledspan-at-offset"></span>`fn at_offset(offset: ByteOffset, label: impl Into<String>) -> Self` — [`ByteOffset`](../index.md)

- <span id="labeledspan-underline"></span>`fn underline(span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md)

- <span id="labeledspan-label"></span>`fn label(&self) -> Option<&str>`

- <span id="labeledspan-inner"></span>`const fn inner(&self) -> &SourceSpan` — [`SourceSpan`](../index.md)

- <span id="labeledspan-offset"></span>`const fn offset(&self) -> usize`

- <span id="labeledspan-len"></span>`const fn len(&self) -> usize`

- <span id="labeledspan-is-empty"></span>`const fn is_empty(&self) -> bool`

- <span id="labeledspan-primary"></span>`const fn primary(&self) -> bool`

#### Trait Implementations

##### `impl Clone for LabeledSpan`

- <span id="labeledspan-clone"></span>`fn clone(&self) -> LabeledSpan` — [`LabeledSpan`](../index.md)

##### `impl Debug for LabeledSpan`

- <span id="labeledspan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LabeledSpan`

##### `impl OwoColorize for LabeledSpan`

##### `impl PartialEq for LabeledSpan`

- <span id="labeledspan-eq"></span>`fn eq(&self, other: &LabeledSpan) -> bool` — [`LabeledSpan`](../index.md)

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

*Defined in [`miette-7.6.0/src/protocol.rs:458-473`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L458-L473)*

Basic implementation of the [`SpanContents`](../index.md) trait, for convenience.

#### Implementations

- <span id="miettespancontents-new"></span>`const fn new(data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](../index.md), [`MietteSpanContents`](../index.md)

- <span id="miettespancontents-new-named"></span>`const fn new_named(name: String, data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](../index.md), [`MietteSpanContents`](../index.md)

- <span id="miettespancontents-with-language"></span>`fn with_language(self, language: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for MietteSpanContents<'a>`

- <span id="miettespancontents-clone"></span>`fn clone(&self) -> MietteSpanContents<'a>` — [`MietteSpanContents`](../index.md)

##### `impl Debug for MietteSpanContents<'a>`

- <span id="miettespancontents-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for MietteSpanContents<'a>`

##### `impl SpanContents for MietteSpanContents<'a>`

- <span id="miettespancontents-data"></span>`fn data(&self) -> &'a [u8]`

- <span id="miettespancontents-span"></span>`fn span(&self) -> &SourceSpan` — [`SourceSpan`](../index.md)

- <span id="miettespancontents-line"></span>`fn line(&self) -> usize`

- <span id="miettespancontents-column"></span>`fn column(&self) -> usize`

- <span id="miettespancontents-line-count"></span>`fn line_count(&self) -> usize`

- <span id="miettespancontents-name"></span>`fn name(&self) -> Option<&str>`

- <span id="miettespancontents-language"></span>`fn language(&self) -> Option<&str>`

### `SourceSpan`

```rust
struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:549-554`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L549-L554)*

Span within a [`SourceCode`](../index.md)

#### Fields

- **`offset`**: `SourceOffset`

  The start of the span.

- **`length`**: `usize`

  The total length of the span

#### Implementations

- <span id="sourcespan-new"></span>`const fn new(start: SourceOffset, length: usize) -> Self` — [`SourceOffset`](../index.md)

- <span id="sourcespan-offset"></span>`const fn offset(&self) -> usize`

- <span id="sourcespan-len"></span>`const fn len(&self) -> usize`

- <span id="sourcespan-is-empty"></span>`const fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for SourceSpan`

- <span id="sourcespan-clone"></span>`fn clone(&self) -> SourceSpan` — [`SourceSpan`](../index.md)

##### `impl Copy for SourceSpan`

##### `impl Debug for SourceSpan`

- <span id="sourcespan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SourceSpan`

##### `impl Hash for SourceSpan`

- <span id="sourcespan-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SourceSpan`

- <span id="sourcespan-cmp"></span>`fn cmp(&self, other: &SourceSpan) -> cmp::Ordering` — [`SourceSpan`](../index.md)

##### `impl OwoColorize for SourceSpan`

##### `impl PartialEq for SourceSpan`

- <span id="sourcespan-eq"></span>`fn eq(&self, other: &SourceSpan) -> bool` — [`SourceSpan`](../index.md)

##### `impl PartialOrd for SourceSpan`

- <span id="sourcespan-partial-cmp"></span>`fn partial_cmp(&self, other: &SourceSpan) -> option::Option<cmp::Ordering>` — [`SourceSpan`](../index.md)

##### `impl StructuralPartialEq for SourceSpan`

### `SourceOffset`

```rust
struct SourceOffset(ByteOffset);
```

*Defined in [`miette-7.6.0/src/protocol.rs:673`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L673)*

Newtype that represents the [`ByteOffset`](../index.md) from the beginning of a [`SourceCode`](../index.md)

#### Implementations

- <span id="sourceoffset-offset"></span>`const fn offset(&self) -> ByteOffset` — [`ByteOffset`](../index.md)

- <span id="sourceoffset-from-location"></span>`fn from_location(source: impl AsRef<str>, loc_line: usize, loc_col: usize) -> Self`

- <span id="sourceoffset-from-current-location"></span>`fn from_current_location() -> Result<(String, Self), MietteError>` — [`MietteError`](../index.md)

#### Trait Implementations

##### `impl Clone for SourceOffset`

- <span id="sourceoffset-clone"></span>`fn clone(&self) -> SourceOffset` — [`SourceOffset`](../index.md)

##### `impl Copy for SourceOffset`

##### `impl Debug for SourceOffset`

- <span id="sourceoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SourceOffset`

##### `impl Hash for SourceOffset`

- <span id="sourceoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SourceOffset`

- <span id="sourceoffset-cmp"></span>`fn cmp(&self, other: &SourceOffset) -> cmp::Ordering` — [`SourceOffset`](../index.md)

##### `impl OwoColorize for SourceOffset`

##### `impl PartialEq for SourceOffset`

- <span id="sourceoffset-eq"></span>`fn eq(&self, other: &SourceOffset) -> bool` — [`SourceOffset`](../index.md)

##### `impl PartialOrd for SourceOffset`

- <span id="sourceoffset-partial-cmp"></span>`fn partial_cmp(&self, other: &SourceOffset) -> option::Option<cmp::Ordering>` — [`SourceOffset`](../index.md)

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

*Defined in [`miette-7.6.0/src/protocol.rs:189-198`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L189-L198)*

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

- <span id="severity-clone"></span>`fn clone(&self) -> Severity` — [`Severity`](../index.md)

##### `impl Copy for Severity`

##### `impl Debug for Severity`

- <span id="severity-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Severity`

- <span id="severity-default"></span>`fn default() -> Severity` — [`Severity`](../index.md)

##### `impl Eq for Severity`

##### `impl Ord for Severity`

- <span id="severity-cmp"></span>`fn cmp(&self, other: &Severity) -> cmp::Ordering` — [`Severity`](../index.md)

##### `impl OwoColorize for Severity`

##### `impl PartialEq for Severity`

- <span id="severity-eq"></span>`fn eq(&self, other: &Severity) -> bool` — [`Severity`](../index.md)

##### `impl PartialOrd for Severity`

- <span id="severity-partial-cmp"></span>`fn partial_cmp(&self, other: &Severity) -> option::Option<cmp::Ordering>` — [`Severity`](../index.md)

##### `impl StructuralPartialEq for Severity`

## Traits

### `Diagnostic`

```rust
trait Diagnostic: std::error::Error { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:20-70`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L20-L70)*

Adds rich metadata to your Error that can be used by
[`Report`](crate::Report) to print really nice and human-friendly error
messages.

#### Provided Methods

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  Unique diagnostic code that can be used to look up more information

- `fn severity(&self) -> Option<Severity>`

  Diagnostic severity. This may be used by

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  Additional help text related to this `Diagnostic`. Do you have any

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  URL to visit for a more detailed explanation/help about this

- `fn source_code(&self) -> Option<&dyn SourceCode>`

  Source code to apply this `Diagnostic`'s `Diagnostic::labels` to.

- `fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>`

  Labels to apply to this `Diagnostic`'s `Diagnostic::source_code`

- `fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>`

  Additional related `Diagnostic`s.

- `fn diagnostic_source(&self) -> Option<&dyn Diagnostic>`

  The cause of the error.

#### Implementors

- [`BoxedError`](../eyreish/wrapper/index.md)
- [`ContextError`](../eyreish/error/index.md)
- [`DiagnosticError`](../eyreish/into_diagnostic/index.md)
- [`DisplayError`](../eyreish/wrapper/index.md)
- [`InstallError`](../index.md)
- [`MessageError`](../eyreish/wrapper/index.md)
- [`MietteDiagnostic`](../index.md)
- [`MietteError`](../index.md)
- [`Panic`](../panic/index.md)
- [`WithSourceCode`](../eyreish/wrapper/index.md)
- `std::convert::Infallible`

### `SourceCode`

```rust
trait SourceCode: Send + Sync { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:236-245`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L236-L245)*

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

#### Implementors

- [`NamedSource`](../index.md)
- `&[u8]`
- `&str`
- `String`
- `Vec<u8>`
- `[u8]`
- `std::borrow::Cow<'_, T>`
- `std::sync::Arc<T>`
- `str`

### `SpanContents<'a>`

```rust
trait SpanContents<'a> { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:426-452`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L426-L452)*

Contents of a [`SourceCode`](../index.md) covered by [`SourceSpan`](../index.md).

Includes line and column information to optimize highlight calculations.

#### Required Methods

- `fn data(&self) -> &'a [u8]`

  Reference to the data inside the associated span, in bytes.

- `fn span(&self) -> &SourceSpan`

  [`SourceSpan`](../index.md) representing the span covered by this `SpanContents`.

- `fn line(&self) -> usize`

  The 0-indexed line in the associated [`SourceCode`](../index.md) where the data

- `fn column(&self) -> usize`

  The 0-indexed column in the associated [`SourceCode`](../index.md) where the data

- `fn line_count(&self) -> usize`

  Total number of lines covered by this `SpanContents`.

#### Provided Methods

- `fn name(&self) -> Option<&str>`

  An optional (file?) name for the container of this `SpanContents`.

- `fn language(&self) -> Option<&str>`

  Optional method. The language name for this source code, if any.

#### Implementors

- [`MietteSpanContents`](../index.md)

## Type Aliases

### `ByteOffset`

```rust
type ByteOffset = usize;
```

*Defined in [`miette-7.6.0/src/protocol.rs:666`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L666)*

"Raw" type for the byte offset from the beginning of a [`SourceCode`](../index.md).

## Macros

### `box_error_impls!`

*Defined in [`miette-7.6.0/src/protocol.rs:72-86`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L72-L86)*

### `box_borrow_impls!`

*Defined in [`miette-7.6.0/src/protocol.rs:94-104`](../../../.source_1765210505/miette-7.6.0/src/protocol.rs#L94-L104)*

