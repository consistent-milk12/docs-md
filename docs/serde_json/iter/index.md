*[serde_json](../index.md) / [iter](index.md)*

---

# Module `iter`

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

- `fn new(iter: I) -> LineColIterator<I>` — [`LineColIterator`](#linecoliterator)

- `fn line(self: &Self) -> usize`

- `fn col(self: &Self) -> usize`

- `fn byte_offset(self: &Self) -> usize`

#### Trait Implementations

##### `impl<I> IntoIterator for LineColIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I> Iterator for LineColIterator<I>`

- `type Item = Result<u8, Error>`

- `fn next(self: &mut Self) -> Option<io::Result<u8>>`

