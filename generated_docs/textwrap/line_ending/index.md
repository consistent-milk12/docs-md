*[textwrap](../index.md) / [line_ending](index.md)*

---

# Module `line_ending`

Line ending detection and conversion.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NonEmptyLines`](#nonemptylines) | struct | An iterator over the lines of a string, as tuples of string slice and [`LineEnding`] value; it only emits non-empty lines (i.e. having some content before the terminating `\r\n` or `\n`). |
| [`LineEnding`](#lineending) | enum | Supported line endings. |

## Structs

### `NonEmptyLines<'a>`

```rust
struct NonEmptyLines<'a>(&'a str);
```

*Defined in [`textwrap-0.16.2/src/line_ending.rs:35`](../../../.source_1765521767/textwrap-0.16.2/src/line_ending.rs#L35)*

An iterator over the lines of a string, as tuples of string slice
and [`LineEnding`](#lineending) value; it only emits non-empty lines (i.e. having
some content before the terminating `\r\n` or `\n`).

This struct is used internally by the library.

#### Trait Implementations

##### `impl Any for NonEmptyLines<'a>`

- <span id="nonemptylines-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NonEmptyLines<'a>`

- <span id="nonemptylines-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NonEmptyLines<'a>`

- <span id="nonemptylines-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NonEmptyLines<'a>`

- <span id="nonemptylines-clone"></span>`fn clone(&self) -> NonEmptyLines<'a>` — [`NonEmptyLines`](#nonemptylines)

##### `impl CloneToUninit for NonEmptyLines<'a>`

- <span id="nonemptylines-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for NonEmptyLines<'a>`

##### `impl Debug for NonEmptyLines<'a>`

- <span id="nonemptylines-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NonEmptyLines<'a>`

- <span id="nonemptylines-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NonEmptyLines<'a>`

- <span id="nonemptylines-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for NonEmptyLines<'a>`

- <span id="nonemptylines-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nonemptylines-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nonemptylines-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NonEmptyLines<'a>`

- <span id="nonemptylines-iterator-type-item"></span>`type Item = (&'a str, Option<LineEnding>)`

- <span id="nonemptylines-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for NonEmptyLines<'a>`

- <span id="nonemptylines-toowned-type-owned"></span>`type Owned = T`

- <span id="nonemptylines-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nonemptylines-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NonEmptyLines<'a>`

- <span id="nonemptylines-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nonemptylines-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NonEmptyLines<'a>`

- <span id="nonemptylines-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nonemptylines-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `LineEnding`

```rust
enum LineEnding {
    CRLF,
    LF,
}
```

*Defined in [`textwrap-0.16.2/src/line_ending.rs:8-16`](../../../.source_1765521767/textwrap-0.16.2/src/line_ending.rs#L8-L16)*

Supported line endings. Like in the Rust standard library, two line
endings are supported: `\r\n` and `\n`

#### Variants

- **`CRLF`**

  _Carriage return and line feed_ – a line ending sequence
  historically used in Windows. Corresponds to the sequence
  of ASCII control characters `0x0D 0x0A` or `\r\n`

- **`LF`**

  _Line feed_ – a line ending historically used in Unix.
   Corresponds to the ASCII control character `0x0A` or `\n`

#### Implementations

- <span id="lineending-as-str"></span>`const fn as_str(&self) -> &'static str`

  Turns this [`LineEnding`](#lineending) value into its ASCII representation.

#### Trait Implementations

##### `impl Any for LineEnding`

- <span id="lineending-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineEnding`

- <span id="lineending-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineEnding`

- <span id="lineending-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LineEnding`

- <span id="lineending-clone"></span>`fn clone(&self) -> LineEnding` — [`LineEnding`](#lineending)

##### `impl CloneToUninit for LineEnding`

- <span id="lineending-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LineEnding`

##### `impl Debug for LineEnding`

- <span id="lineending-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineEnding`

##### `impl<T> From for LineEnding`

- <span id="lineending-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineEnding`

- <span id="lineending-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LineEnding`

- <span id="lineending-partialeq-eq"></span>`fn eq(&self, other: &LineEnding) -> bool` — [`LineEnding`](#lineending)

##### `impl StructuralPartialEq for LineEnding`

##### `impl ToOwned for LineEnding`

- <span id="lineending-toowned-type-owned"></span>`type Owned = T`

- <span id="lineending-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lineending-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineEnding`

- <span id="lineending-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lineending-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineEnding`

- <span id="lineending-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lineending-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

