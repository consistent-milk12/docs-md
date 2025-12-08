*[aho_corasick](../../index.md) / [util](../index.md) / [buffer](index.md)*

---

# Module `buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buffer`](#buffer) | struct | A fairly simple roll buffer for supporting stream searches. |
| [`DEFAULT_BUFFER_CAPACITY`](#default_buffer_capacity) | const | The default buffer capacity that we use for the stream buffer. |

## Structs

### `Buffer`

```rust
struct Buffer {
    buf: alloc::vec::Vec<u8>,
    min: usize,
    end: usize,
}
```

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

- <span id="buffer-buffer"></span>`fn buffer(&self) -> &[u8]`

- <span id="buffer-min-buffer-len"></span>`fn min_buffer_len(&self) -> usize`

- <span id="buffer-free-buffer"></span>`fn free_buffer(&mut self) -> &mut [u8]`

- <span id="buffer-fill"></span>`fn fill<R: std::io::Read>(&mut self, rdr: R) -> std::io::Result<bool>`

- <span id="buffer-roll"></span>`fn roll(&mut self)`

#### Trait Implementations

##### `impl Debug for Buffer`

- <span id="buffer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Constants

### `DEFAULT_BUFFER_CAPACITY`

```rust
const DEFAULT_BUFFER_CAPACITY: usize = 65_536usize;
```

The default buffer capacity that we use for the stream buffer.

