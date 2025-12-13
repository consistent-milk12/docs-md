*[regex_syntax](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Formatter`](#formatter) | struct | A helper type for formatting nice error messages. |
| [`Spans`](#spans) | struct | This type represents an arbitrary number of error spans in a way that makes it convenient to notate the regex pattern. |
| [`Error`](#error) | enum | This error type encompasses any error that can be returned by this crate. |
| [`repeat_char`](#repeat-char) | fn |  |

## Structs

### `Formatter<'e, E>`

```rust
struct Formatter<'e, E> {
    pattern: &'e str,
    err: &'e E,
    span: &'e ast::Span,
    aux_span: Option<&'e ast::Span>,
}
```

*Defined in [`regex-syntax-0.8.8/src/error.rs:55-65`](../../../.source_1765633015/regex-syntax-0.8.8/src/error.rs#L55-L65)*

A helper type for formatting nice error messages.

This type is responsible for reporting regex parse errors in a nice human
readable format. Most of its complexity is from interspersing notational
markers pointing out the position where an error occurred.

#### Fields

- **`pattern`**: `&'e str`

  The original regex pattern in which the error occurred.

- **`err`**: `&'e E`

  The error kind. It must impl fmt::Display.

- **`span`**: `&'e ast::Span`

  The primary span of the error.

- **`aux_span`**: `Option<&'e ast::Span>`

  An auxiliary and optional span, in case the error needs to point to
  two locations (e.g., when reporting a duplicate capture group name).

#### Trait Implementations

##### `impl Any for Formatter<'e, E>`

- <span id="formatter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Formatter<'e, E>`

- <span id="formatter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Formatter<'e, E>`

- <span id="formatter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: fmt::Debug> Debug for Formatter<'e, E>`

- <span id="formatter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: core::fmt::Display> Display for Formatter<'e, E>`

- <span id="formatter-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Formatter<'e, E>`

- <span id="formatter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Formatter<'e, E>`

- <span id="formatter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for Formatter<'e, E>`

- <span id="formatter-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Formatter<'e, E>`

- <span id="formatter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="formatter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Formatter<'e, E>`

- <span id="formatter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="formatter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Spans<'p>`

```rust
struct Spans<'p> {
    pattern: &'p str,
    line_number_width: usize,
    by_line: alloc::vec::Vec<alloc::vec::Vec<ast::Span>>,
    multi_line: alloc::vec::Vec<ast::Span>,
}
```

*Defined in [`regex-syntax-0.8.8/src/error.rs:134-152`](../../../.source_1765633015/regex-syntax-0.8.8/src/error.rs#L134-L152)*

This type represents an arbitrary number of error spans in a way that makes
it convenient to notate the regex pattern. ("Notate" means "point out
exactly where the error occurred in the regex pattern.")

Technically, we can only ever have two spans given our current error
structure. However, after toiling with a specific algorithm for handling
two spans, it became obvious that an algorithm to handle an arbitrary
number of spans was actually much simpler.

#### Fields

- **`pattern`**: `&'p str`

  The original regex pattern string.

- **`line_number_width`**: `usize`

  The total width that should be used for line numbers. The width is
  used for left padding the line numbers for alignment.
  
  A value of `0` means line numbers should not be displayed. That is,
  the pattern is itself only one line.

- **`by_line`**: `alloc::vec::Vec<alloc::vec::Vec<ast::Span>>`

  All error spans that occur on a single line. This sequence always has
  length equivalent to the number of lines in `pattern`, where the index
  of the sequence represents a line number, starting at `0`. The spans
  in each line are sorted in ascending order.

- **`multi_line`**: `alloc::vec::Vec<ast::Span>`

  All error spans that occur over one or more lines. That is, the start
  and end position of the span have different line numbers. The spans are
  sorted in ascending order.

#### Implementations

- <span id="spans-from-formatter"></span>`fn from_formatter<'e, E: core::fmt::Display>(fmter: &'p Formatter<'e, E>) -> Spans<'p>` — [`Formatter`](#formatter), [`Spans`](#spans)

  Build a sequence of spans from a formatter.

- <span id="spans-add"></span>`fn add(&mut self, span: ast::Span)` — [`Span`](../ast/index.md#span)

  Add the given span to this sequence, putting it in the right place.

- <span id="spans-notate"></span>`fn notate(&self) -> String`

  Notate the pattern string with carets (`^`) pointing at each span

  location. This only applies to spans that occur within a single line.

- <span id="spans-notate-line"></span>`fn notate_line(&self, i: usize) -> Option<String>`

  Return notes for the line indexed at `i` (zero-based). If there are no

  spans for the given line, then `None` is returned. Otherwise, an

  appropriately space padded string with correctly positioned `^` is

  returned, accounting for line numbers.

- <span id="spans-left-pad-line-number"></span>`fn left_pad_line_number(&self, n: usize) -> String`

  Left pad the given line number with spaces such that it is aligned with

  other line numbers.

- <span id="spans-line-number-padding"></span>`fn line_number_padding(&self) -> usize`

  Return the line number padding beginning at the start of each line of

  the pattern.

  

  If the pattern is only one line, then this returns a fixed padding

  for visual indentation.

#### Trait Implementations

##### `impl Any for Spans<'p>`

- <span id="spans-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Spans<'p>`

- <span id="spans-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Spans<'p>`

- <span id="spans-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Spans<'p>`

- <span id="spans-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Spans<'p>`

- <span id="spans-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Spans<'p>`

- <span id="spans-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="spans-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Spans<'p>`

- <span id="spans-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="spans-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Error`

```rust
enum Error {
    Parse(ast::Error),
    Translate(hir::Error),
}
```

*Defined in [`regex-syntax-0.8.8/src/error.rs:16-23`](../../../.source_1765633015/regex-syntax-0.8.8/src/error.rs#L16-L23)*

This error type encompasses any error that can be returned by this crate.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

#### Variants

- **`Parse`**

  An error that occurred while translating concrete syntax into abstract
  syntax (AST).

- **`Translate`**

  An error that occurred while translating abstract syntax into a high
  level intermediate representation (HIR).

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToOwned for Error`

- <span id="error-toowned-type-owned"></span>`type Owned = T`

- <span id="error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `repeat_char`

```rust
fn repeat_char(c: char, count: usize) -> alloc::string::String
```

*Defined in [`regex-syntax-0.8.8/src/error.rs:268-270`](../../../.source_1765633015/regex-syntax-0.8.8/src/error.rs#L268-L270)*

