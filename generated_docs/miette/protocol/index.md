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
  - [`box_error_impls!`](#box-error-impls)
  - [`box_borrow_impls!`](#box-borrow-impls)

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
| [`box_error_impls!`](#box-error-impls) | macro |  |
| [`box_borrow_impls!`](#box-borrow-impls) | macro |  |

## Structs

### `LabeledSpan`

```rust
struct LabeledSpan {
    label: Option<String>,
    span: SourceSpan,
    primary: bool,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:250-255`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L250-L255)*

A labeled [`SourceSpan`](../index.md).

#### Implementations

- <span id="labeledspan-new"></span>`const fn new(label: Option<String>, offset: ByteOffset, len: usize) -> Self` — [`ByteOffset`](../index.md#byteoffset)

  Makes a new labeled span.

- <span id="labeledspan-new-with-span"></span>`fn new_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md#sourcespan)

  Makes a new labeled span using an existing span.

- <span id="labeledspan-new-primary-with-span"></span>`fn new_primary_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md#sourcespan)

  Makes a new labeled primary span using an existing span.

- <span id="labeledspan-set-label"></span>`fn set_label(&mut self, label: Option<String>)`

  Change the text of the label

- <span id="labeledspan-at"></span>`fn at(span: impl Into<SourceSpan>, label: impl Into<String>) -> Self` — [`SourceSpan`](../index.md#sourcespan)

  Makes a new label at specified span

  

  # Examples

  ```rust

  use miette::LabeledSpan;

  

  let source = "Cpp is the best";

  let label = LabeledSpan::at(0..3, "should be Rust");

  assert_eq!(

      label,

      LabeledSpan::new(Some("should be Rust".to_string()), 0, 3)

  )

  ```

- <span id="labeledspan-at-offset"></span>`fn at_offset(offset: ByteOffset, label: impl Into<String>) -> Self` — [`ByteOffset`](../index.md#byteoffset)

  Makes a new label that points at a specific offset.

  

  # Examples

  ```rust

  use miette::LabeledSpan;

  

  let source = "(2 + 2";

  let label = LabeledSpan::at_offset(4, "expected a closing parenthesis");

  assert_eq!(

      label,

      LabeledSpan::new(Some("expected a closing parenthesis".to_string()), 4, 0)

  )

  ```

- <span id="labeledspan-underline"></span>`fn underline(span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](../index.md#sourcespan)

  Makes a new label without text, that underlines a specific span.

  

  # Examples

  ```rust

  use miette::LabeledSpan;

  

  let source = "You have an eror here";

  let label = LabeledSpan::underline(12..16);

  assert_eq!(label, LabeledSpan::new(None, 12, 4))

  ```

- <span id="labeledspan-label"></span>`fn label(&self) -> Option<&str>`

  Gets the (optional) label string for this `LabeledSpan`.

- <span id="labeledspan-inner"></span>`const fn inner(&self) -> &SourceSpan` — [`SourceSpan`](../index.md#sourcespan)

  Returns a reference to the inner [`SourceSpan`](../index.md).

- <span id="labeledspan-offset"></span>`const fn offset(&self) -> usize`

  Returns the 0-based starting byte offset.

- <span id="labeledspan-len"></span>`const fn len(&self) -> usize`

  Returns the number of bytes this `LabeledSpan` spans.

- <span id="labeledspan-is-empty"></span>`const fn is_empty(&self) -> bool`

  True if this `LabeledSpan` is empty.

- <span id="labeledspan-primary"></span>`const fn primary(&self) -> bool`

  True if this `LabeledSpan` is a primary span.

#### Trait Implementations

##### `impl Any for LabeledSpan`

- <span id="labeledspan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LabeledSpan`

- <span id="labeledspan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LabeledSpan`

- <span id="labeledspan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LabeledSpan`

- <span id="labeledspan-clone"></span>`fn clone(&self) -> LabeledSpan` — [`LabeledSpan`](../index.md#labeledspan)

##### `impl CloneToUninit for LabeledSpan`

- <span id="labeledspan-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for LabeledSpan`

- <span id="labeledspan-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LabeledSpan`

##### `impl<T> From for LabeledSpan`

- <span id="labeledspan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LabeledSpan`

- <span id="labeledspan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LabeledSpan`

##### `impl PartialEq for LabeledSpan`

- <span id="labeledspan-partialeq-eq"></span>`fn eq(&self, other: &LabeledSpan) -> bool` — [`LabeledSpan`](../index.md#labeledspan)

##### `impl StructuralPartialEq for LabeledSpan`

##### `impl ToOwned for LabeledSpan`

- <span id="labeledspan-toowned-type-owned"></span>`type Owned = T`

- <span id="labeledspan-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="labeledspan-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LabeledSpan`

- <span id="labeledspan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="labeledspan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LabeledSpan`

- <span id="labeledspan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="labeledspan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miette-7.6.0/src/protocol.rs:458-473`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L458-L473)*

Basic implementation of the [`SpanContents`](../index.md) trait, for convenience.

#### Implementations

- <span id="miettespancontents-new"></span>`const fn new(data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](../index.md#sourcespan), [`MietteSpanContents`](../index.md#miettespancontents)

  Make a new [`MietteSpanContents`](../index.md) object.

- <span id="miettespancontents-new-named"></span>`const fn new_named(name: String, data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](../index.md#sourcespan), [`MietteSpanContents`](../index.md#miettespancontents)

  Make a new [`MietteSpanContents`](../index.md) object, with a name for its 'file'.

- <span id="miettespancontents-with-language"></span>`fn with_language(self, language: impl Into<String>) -> Self`

  Sets the [`language`](SpanContents::language) for syntax highlighting.

#### Trait Implementations

##### `impl Any for MietteSpanContents<'a>`

- <span id="miettespancontents-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MietteSpanContents<'a>`

- <span id="miettespancontents-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MietteSpanContents<'a>`

- <span id="miettespancontents-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MietteSpanContents<'a>`

- <span id="miettespancontents-clone"></span>`fn clone(&self) -> MietteSpanContents<'a>` — [`MietteSpanContents`](../index.md#miettespancontents)

##### `impl CloneToUninit for MietteSpanContents<'a>`

- <span id="miettespancontents-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MietteSpanContents<'a>`

- <span id="miettespancontents-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MietteSpanContents<'a>`

- <span id="miettespancontents-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MietteSpanContents<'a>`

- <span id="miettespancontents-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MietteSpanContents<'a>`

##### `impl SpanContents for MietteSpanContents<'a>`

- <span id="miettespancontents-spancontents-data"></span>`fn data(&self) -> &'a [u8]`

- <span id="miettespancontents-spancontents-span"></span>`fn span(&self) -> &SourceSpan` — [`SourceSpan`](../index.md#sourcespan)

- <span id="miettespancontents-spancontents-line"></span>`fn line(&self) -> usize`

- <span id="miettespancontents-spancontents-column"></span>`fn column(&self) -> usize`

- <span id="miettespancontents-spancontents-line-count"></span>`fn line_count(&self) -> usize`

- <span id="miettespancontents-spancontents-name"></span>`fn name(&self) -> Option<&str>`

- <span id="miettespancontents-spancontents-language"></span>`fn language(&self) -> Option<&str>`

##### `impl ToOwned for MietteSpanContents<'a>`

- <span id="miettespancontents-toowned-type-owned"></span>`type Owned = T`

- <span id="miettespancontents-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="miettespancontents-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MietteSpanContents<'a>`

- <span id="miettespancontents-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="miettespancontents-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MietteSpanContents<'a>`

- <span id="miettespancontents-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="miettespancontents-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SourceSpan`

```rust
struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:549-554`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L549-L554)*

Span within a [`SourceCode`](../index.md)

#### Fields

- **`offset`**: `SourceOffset`

  The start of the span.

- **`length`**: `usize`

  The total length of the span

#### Implementations

- <span id="sourcespan-new"></span>`const fn new(start: SourceOffset, length: usize) -> Self` — [`SourceOffset`](../index.md#sourceoffset)

  Create a new [`SourceSpan`](../index.md).

- <span id="sourcespan-offset"></span>`const fn offset(&self) -> usize`

  The absolute offset, in bytes, from the beginning of a [`SourceCode`](../index.md).

- <span id="sourcespan-len"></span>`const fn len(&self) -> usize`

  Total length of the [`SourceSpan`](../index.md), in bytes.

- <span id="sourcespan-is-empty"></span>`const fn is_empty(&self) -> bool`

  Whether this [`SourceSpan`](../index.md) has a length of zero. It may still be useful

  to point to a specific point.

#### Trait Implementations

##### `impl Any for SourceSpan`

- <span id="sourcespan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceSpan`

- <span id="sourcespan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceSpan`

- <span id="sourcespan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SourceSpan`

- <span id="sourcespan-clone"></span>`fn clone(&self) -> SourceSpan` — [`SourceSpan`](../index.md#sourcespan)

##### `impl CloneToUninit for SourceSpan`

- <span id="sourcespan-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SourceSpan`

##### `impl Debug for SourceSpan`

- <span id="sourcespan-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SourceSpan`

##### `impl<T> From for SourceSpan`

- <span id="sourcespan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SourceSpan`

- <span id="sourcespan-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SourceSpan`

- <span id="sourcespan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for SourceSpan`

- <span id="sourcespan-ord-cmp"></span>`fn cmp(&self, other: &SourceSpan) -> cmp::Ordering` — [`SourceSpan`](../index.md#sourcespan)

##### `impl OwoColorize for SourceSpan`

##### `impl PartialEq for SourceSpan`

- <span id="sourcespan-partialeq-eq"></span>`fn eq(&self, other: &SourceSpan) -> bool` — [`SourceSpan`](../index.md#sourcespan)

##### `impl PartialOrd for SourceSpan`

- <span id="sourcespan-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SourceSpan) -> option::Option<cmp::Ordering>` — [`SourceSpan`](../index.md#sourcespan)

##### `impl StructuralPartialEq for SourceSpan`

##### `impl ToOwned for SourceSpan`

- <span id="sourcespan-toowned-type-owned"></span>`type Owned = T`

- <span id="sourcespan-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sourcespan-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SourceSpan`

- <span id="sourcespan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcespan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceSpan`

- <span id="sourcespan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcespan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SourceOffset`

```rust
struct SourceOffset(ByteOffset);
```

*Defined in [`miette-7.6.0/src/protocol.rs:673`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L673)*

Newtype that represents the [`ByteOffset`](../index.md) from the beginning of a [`SourceCode`](../index.md)

#### Implementations

- <span id="sourceoffset-offset"></span>`const fn offset(&self) -> ByteOffset` — [`ByteOffset`](../index.md#byteoffset)

  Actual byte offset.

- <span id="sourceoffset-from-location"></span>`fn from_location(source: impl AsRef<str>, loc_line: usize, loc_col: usize) -> Self`

  Little utility to help convert 1-based line/column locations into

  miette-compatible Spans

  

  This function is infallible: Giving an out-of-range line/column pair

  will return the offset of the last byte in the source.

- <span id="sourceoffset-from-current-location"></span>`fn from_current_location() -> Result<(String, Self), MietteError>` — [`MietteError`](../index.md#mietteerror)

  Returns an offset for the _file_ location of wherever this function is

  called. If you want to get _that_ caller's location, mark this

  function's caller with `#[track_caller]` (and so on and so forth).

  

  Returns both the filename that was given and the offset of the caller

  as a [`SourceOffset`](../index.md).

  

  Keep in mind that this fill only work if the file your Rust source

  file was compiled from is actually available at that location. If

  you're shipping binaries for your application, you'll want to ignore

  the Err case or otherwise report it.

#### Trait Implementations

##### `impl Any for SourceOffset`

- <span id="sourceoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceOffset`

- <span id="sourceoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceOffset`

- <span id="sourceoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SourceOffset`

- <span id="sourceoffset-clone"></span>`fn clone(&self) -> SourceOffset` — [`SourceOffset`](../index.md#sourceoffset)

##### `impl CloneToUninit for SourceOffset`

- <span id="sourceoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SourceOffset`

##### `impl Debug for SourceOffset`

- <span id="sourceoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SourceOffset`

##### `impl<T> From for SourceOffset`

- <span id="sourceoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SourceOffset`

- <span id="sourceoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SourceOffset`

- <span id="sourceoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for SourceOffset`

- <span id="sourceoffset-ord-cmp"></span>`fn cmp(&self, other: &SourceOffset) -> cmp::Ordering` — [`SourceOffset`](../index.md#sourceoffset)

##### `impl OwoColorize for SourceOffset`

##### `impl PartialEq for SourceOffset`

- <span id="sourceoffset-partialeq-eq"></span>`fn eq(&self, other: &SourceOffset) -> bool` — [`SourceOffset`](../index.md#sourceoffset)

##### `impl PartialOrd for SourceOffset`

- <span id="sourceoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SourceOffset) -> option::Option<cmp::Ordering>` — [`SourceOffset`](../index.md#sourceoffset)

##### `impl StructuralPartialEq for SourceOffset`

##### `impl ToOwned for SourceOffset`

- <span id="sourceoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="sourceoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sourceoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SourceOffset`

- <span id="sourceoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourceoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceOffset`

- <span id="sourceoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourceoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Severity`

```rust
enum Severity {
    Advice,
    Warning,
    Error,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:189-198`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L189-L198)*

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

##### `impl Any for Severity`

- <span id="severity-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Severity`

- <span id="severity-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Severity`

- <span id="severity-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Severity`

- <span id="severity-clone"></span>`fn clone(&self) -> Severity` — [`Severity`](../index.md#severity)

##### `impl CloneToUninit for Severity`

- <span id="severity-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Severity`

##### `impl Debug for Severity`

- <span id="severity-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Severity`

- <span id="severity-default"></span>`fn default() -> Severity` — [`Severity`](../index.md#severity)

##### `impl Eq for Severity`

##### `impl<T> From for Severity`

- <span id="severity-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Severity`

- <span id="severity-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Severity`

- <span id="severity-ord-cmp"></span>`fn cmp(&self, other: &Severity) -> cmp::Ordering` — [`Severity`](../index.md#severity)

##### `impl OwoColorize for Severity`

##### `impl PartialEq for Severity`

- <span id="severity-partialeq-eq"></span>`fn eq(&self, other: &Severity) -> bool` — [`Severity`](../index.md#severity)

##### `impl PartialOrd for Severity`

- <span id="severity-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Severity) -> option::Option<cmp::Ordering>` — [`Severity`](../index.md#severity)

##### `impl StructuralPartialEq for Severity`

##### `impl ToOwned for Severity`

- <span id="severity-toowned-type-owned"></span>`type Owned = T`

- <span id="severity-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="severity-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Severity`

- <span id="severity-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="severity-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Severity`

- <span id="severity-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="severity-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Diagnostic`

```rust
trait Diagnostic: std::error::Error { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:20-70`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L20-L70)*

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

- [`BoxedError`](../eyreish/wrapper/index.md#boxederror)
- [`ContextError`](../eyreish/error/index.md#contexterror)
- [`DiagnosticError`](../eyreish/into_diagnostic/index.md#diagnosticerror)
- [`DisplayError`](../eyreish/wrapper/index.md#displayerror)
- [`InstallError`](../index.md#installerror)
- [`MessageError`](../eyreish/wrapper/index.md#messageerror)
- [`MietteDiagnostic`](../index.md#miettediagnostic)
- [`MietteError`](../index.md#mietteerror)
- [`Panic`](../panic/index.md#panic)
- [`WithSourceCode`](../eyreish/wrapper/index.md#withsourcecode)
- `std::convert::Infallible`

### `SourceCode`

```rust
trait SourceCode: Send + Sync { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:236-245`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L236-L245)*

Represents readable source code of some sort.

This trait is able to support simple `SourceCode` types like [`String`](../../cargo_platform/index.md)s, as
well as more involved types like indexes into centralized `SourceMap`-like
types, file handles, and even network streams.

If you can read it, you can source it, and it's not necessary to read the
whole thing--meaning you should be able to support `SourceCode`s which are
gigabytes or larger in size.

#### Required Methods

- `fn read_span<'a>(self: &'a Self, span: &SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<Box<dyn SpanContents<'a>>, MietteError>`

  Read the bytes for a specific span from this `SourceCode`, keeping a

#### Implementors

- [`NamedSource`](../index.md#namedsource)
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

*Defined in [`miette-7.6.0/src/protocol.rs:426-452`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L426-L452)*

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

- [`MietteSpanContents`](../index.md#miettespancontents)

## Type Aliases

### `ByteOffset`

```rust
type ByteOffset = usize;
```

*Defined in [`miette-7.6.0/src/protocol.rs:666`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L666)*

"Raw" type for the byte offset from the beginning of a [`SourceCode`](../index.md).

## Macros

### `box_error_impls!`

*Defined in [`miette-7.6.0/src/protocol.rs:72-86`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L72-L86)*

### `box_borrow_impls!`

*Defined in [`miette-7.6.0/src/protocol.rs:94-104`](../../../.source_1765633015/miette-7.6.0/src/protocol.rs#L94-L104)*

