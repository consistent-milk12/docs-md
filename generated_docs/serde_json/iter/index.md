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

*Defined in [`serde_json-1.0.145/src/iter.rs:3-20`](../../../.source_1765210505/serde_json-1.0.145/src/iter.rs#L3-L20)*

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

##### `impl<I> IntoIterator for LineColIterator<I>`

- <span id="linecoliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="linecoliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="linecoliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for LineColIterator<I>`

- <span id="linecoliterator-iterator-type-item"></span>`type Item = Result<u8, Error>`

- <span id="linecoliterator-next"></span>`fn next(&mut self) -> Option<io::Result<u8>>` — [`Result`](../io/index.md#result)

