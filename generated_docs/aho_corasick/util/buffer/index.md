*[aho_corasick](../../index.md) / [util](../index.md) / [buffer](index.md)*

---

# Module `buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buffer`](#buffer) | struct | A fairly simple roll buffer for supporting stream searches. |
| [`DEFAULT_BUFFER_CAPACITY`](#default-buffer-capacity) | const | The default buffer capacity that we use for the stream buffer. |

## Structs

### `Buffer`

```rust
struct Buffer {
    buf: alloc::vec::Vec<u8>,
    min: usize,
    end: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/buffer.rs:35-44`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/buffer.rs#L35-L44)*

A fairly simple roll buffer for supporting stream searches.

This buffer acts as a temporary place to store a fixed amount of data when
reading from a stream. Its central purpose is to allow "rolling" some
suffix of the data to the beginning of the buffer before refilling it with
more data from the stream. For example, let's say we are trying to match
"foobar" on a stream. When we report the match, we'd like to not only
report the correct offsets at which the match occurs, but also the matching
bytes themselves. So let's say our stream is a file with the following
contents: `test test foobar test test`. Now assume that we happen to read
the aforementioned file in two chunks: `test test foo` and `bar test test`.
Naively, it would not be possible to report a single contiguous `foobar`
match, but this roll buffer allows us to do that. Namely, after the second
read, the contents of the buffer should be `st foobar test test`, where the
search should ultimately resume immediately after `foo`. (The prefix `st `
is included because the roll buffer saves N bytes at the end of the buffer,
where N is the maximum possible length of a match.)

A lot of the logic for dealing with this is unfortunately split out between
this roll buffer and the `StreamChunkIter`.

Note also that this buffer is not actually required to just report matches.
Because a `Match` is just some offsets. But it *is* required for supporting
things like `try_stream_replace_all` because that needs some mechanism for
knowing which bytes in the stream correspond to a match and which don't. So
when a match occurs across two `read` calls, *something* needs to retain
the bytes from the previous `read` call because you don't know before the
second read call whether a match exists or not.

#### Fields

- **`buf`**: `alloc::vec::Vec<u8>`

  The raw buffer contents. This has a fixed size and never increases.

- **`min`**: `usize`

  The minimum size of the buffer, which is equivalent to the maximum
  possible length of a match. This corresponds to the amount that we
  roll

- **`end`**: `usize`

  The end of the contents of this buffer.

#### Implementations

- <span id="buffer-new"></span>`fn new(min_buffer_len: usize) -> Buffer` — [`Buffer`](#buffer)

  Create a new buffer for stream searching. The minimum buffer length

  given should be the size of the maximum possible match length.

- <span id="buffer-buffer"></span>`fn buffer(&self) -> &[u8]`

  Return the contents of this buffer.

- <span id="buffer-min-buffer-len"></span>`fn min_buffer_len(&self) -> usize`

  Return the minimum size of the buffer. The only way a buffer may be

  smaller than this is if the stream itself contains less than the

  minimum buffer amount.

- <span id="buffer-free-buffer"></span>`fn free_buffer(&mut self) -> &mut [u8]`

  Return all free capacity in this buffer.

- <span id="buffer-fill"></span>`fn fill<R: std::io::Read>(&mut self, rdr: R) -> std::io::Result<bool>`

  Refill the contents of this buffer by reading as much as possible into

  this buffer's free capacity. If no more bytes could be read, then this

  returns false. Otherwise, this reads until it has filled the buffer

  past the minimum amount.

- <span id="buffer-roll"></span>`fn roll(&mut self)`

  Roll the contents of the buffer so that the suffix of this buffer is

  moved to the front and all other contents are dropped. The size of the

  suffix corresponds precisely to the minimum buffer length.

  

  This should only be called when the entire contents of this buffer have

  been searched.

#### Trait Implementations

##### `impl Any for Buffer`

- <span id="buffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Buffer`

- <span id="buffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Buffer`

- <span id="buffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Buffer`

- <span id="buffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Buffer`

- <span id="buffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Buffer`

- <span id="buffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Buffer`

- <span id="buffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Buffer`

- <span id="buffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `DEFAULT_BUFFER_CAPACITY`
```rust
const DEFAULT_BUFFER_CAPACITY: usize = 65_536usize;
```

*Defined in [`aho-corasick-1.1.4/src/util/buffer.rs:4`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/buffer.rs#L4)*

The default buffer capacity that we use for the stream buffer.

