*[serde_json](../index.md) / [iter](index.md)*

---

# Module `iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LineColIterator`](#linecoliterator) | struct |  |

## Structs

### `LineColIterator<I>`

```rust
struct LineColIterator<I> {
    iter: I,
    line: usize,
    col: usize,
    start_of_line: usize,
}
```

*Defined in [`serde_json-1.0.145/src/iter.rs:3-20`](../../../.source_1765521767/serde_json-1.0.145/src/iter.rs#L3-L20)*

#### Fields

- **`line`**: `usize`

  Index of the current line. Characters in the first line of the input
  (before the first newline character) are in line 1.

- **`col`**: `usize`

  Index of the current column. The first character in the input and any
  characters immediately following a newline character are in column 1.
  The column is 0 immediately after a newline character has been read.

- **`start_of_line`**: `usize`

  Byte offset of the start of the current line. This is the sum of lengths
  of all previous lines. Keeping track of things this way allows efficient
  computation of the current line, column, and byte offset while only
  updating one of the counters in `next()` in the common case.

#### Implementations

- <span id="linecoliterator-new"></span>`fn new(iter: I) -> LineColIterator<I>` — [`LineColIterator`](#linecoliterator)

- <span id="linecoliterator-line"></span>`fn line(&self) -> usize`

- <span id="linecoliterator-col"></span>`fn col(&self) -> usize`

- <span id="linecoliterator-byte-offset"></span>`fn byte_offset(&self) -> usize`

#### Trait Implementations

##### `impl Any for LineColIterator<I>`

- <span id="linecoliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineColIterator<I>`

- <span id="linecoliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineColIterator<I>`

- <span id="linecoliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LineColIterator<I>`

- <span id="linecoliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineColIterator<I>`

- <span id="linecoliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for LineColIterator<I>`

- <span id="linecoliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="linecoliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="linecoliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for LineColIterator<I>`

- <span id="linecoliterator-iterator-type-item"></span>`type Item = Result<u8, Error>`

- <span id="linecoliterator-iterator-next"></span>`fn next(&mut self) -> Option<io::Result<u8>>` — [`Result`](../io/index.md#result)

##### `impl<U> TryFrom for LineColIterator<I>`

- <span id="linecoliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linecoliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineColIterator<I>`

- <span id="linecoliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linecoliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

