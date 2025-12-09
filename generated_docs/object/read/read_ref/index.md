*[object](../../index.md) / [read](../index.md) / [read_ref](index.md)*

---

# Module `read_ref`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReadRef`](#readref) | trait | A trait for reading references to [`Pod`] types from a block of data. |
| [`Result`](#result) | type |  |

## Traits

### `ReadRef<'a>`

```rust
trait ReadRef<'a>: Clone + Copy { ... }
```

*Defined in [`object-0.37.3/src/read/read_ref.rs:49-124`](../../../../.source_1765210505/object-0.37.3/src/read/read_ref.rs#L49-L124)*

A trait for reading references to [`Pod`](../../index.md) types from a block of data.

This allows parsers to handle both of these cases:
- the block of data exists in memory, and it is desirable
  to use references to this block instead of copying it,
- the block of data exists in storage, and it is desirable
  to read on demand to minimize I/O and memory usage.

A block of data typically exists in memory as a result of using a memory
mapped file, and the crate was written with this use case in mind.
Reading the entire file into a `Vec` is also possible, but it often uses
more I/O and memory.
Both of these are handled by the `ReadRef` implementation for `&[u8]`.

For the second use case, the `ReadRef` trait is implemented for
[`&ReadCache`](super::ReadCache). This is useful for environments where
memory mapped files are not available or not suitable, such as WebAssembly.
This differs from reading into a `Vec` in that it only reads the portions
of the file that are needed for parsing.

The methods accept `self` by value because `Self` is expected to behave
similar to a reference: it may be a reference with a lifetime of `'a`,
or it may be a wrapper of a reference.

The `Clone` and `Copy` bounds are for convenience, and since `Self` is
expected to be similar to a reference, these are easily satisfied.

Object file parsers typically use offsets to locate the structures
in the block, and will most commonly use the `*_at` methods to
read a structure at a known offset.

Occasionally file parsers will need to treat the block as a stream,
and so convenience methods are provided that update an offset with
the size that was read.

#### Required Methods

- `fn len(self) -> result::Result<u64, ()>`

  The total size of the block of data.

- `fn read_bytes_at(self, offset: u64, size: u64) -> result::Result<&'a [u8], ()>`

  Get a reference to a `u8` slice at the given offset.

- `fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> result::Result<&'a [u8], ()>`

  Get a reference to a delimited `u8` slice which starts at range.start.

#### Provided Methods

- `fn read_bytes(self, offset: &mut u64, size: u64) -> result::Result<&'a [u8], ()>`

  Get a reference to a `u8` slice at the given offset, and update the offset.

- `fn read<T: Pod>(self, offset: &mut u64) -> result::Result<&'a T, ()>`

  Get a reference to a `Pod` type at the given offset, and update the offset.

- `fn read_at<T: Pod>(self, offset: u64) -> result::Result<&'a T, ()>`

  Get a reference to a `Pod` type at the given offset.

- `fn read_slice<T: Pod>(self, offset: &mut u64, count: usize) -> result::Result<&'a [T], ()>`

  Get a reference to a slice of a `Pod` type at the given offset, and update the offset.

- `fn read_slice_at<T: Pod>(self, offset: u64, count: usize) -> result::Result<&'a [T], ()>`

  Get a reference to a slice of a `Pod` type at the given offset.

#### Implementors

- [`ReadCacheRange`](../index.md)
- `&'a ReadCache<R>`
- `&'a [u8]`

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, ()>;
```

*Defined in [`object-0.37.3/src/read/read_ref.rs:9`](../../../../.source_1765210505/object-0.37.3/src/read/read_ref.rs#L9)*

