*[object](../../index.md) / [read](../index.md) / [util](index.md)*

---

# Module `util`

## Structs

### `Bytes<'data>`

```rust
struct Bytes<'data>(&'data [u8]);
```

A newtype for byte slices.

It has these important features:
- no methods that can panic, such as `Index`
- convenience methods for `Pod` types
- a useful `Debug` implementation

#### Implementations

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn skip(self: &mut Self, offset: usize) -> Result<(), ()>`

- `fn read_bytes(self: &mut Self, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](../index.md)

- `fn read_bytes_at(self: Self, offset: usize, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](../index.md)

- `fn read<T: Pod>(self: &mut Self) -> Result<&'data T, ()>`

- `fn read_at<T: Pod>(self: Self, offset: usize) -> Result<&'data T, ()>`

- `fn read_slice<T: Pod>(self: &mut Self, count: usize) -> Result<&'data [T], ()>`

- `fn read_slice_at<T: Pod>(self: Self, offset: usize, count: usize) -> Result<&'data [T], ()>`

- `fn read_string(self: &mut Self) -> Result<&'data [u8], ()>`

- `fn read_string_at(self: Self, offset: usize) -> Result<&'data [u8], ()>`

- `fn read_uleb128(self: &mut Self) -> Result<u64, ()>`

- `fn read_sleb128(self: &mut Self) -> Result<i64, ()>`

#### Trait Implementations

##### `impl<'data> Clone for Bytes<'data>`

- `fn clone(self: &Self) -> Bytes<'data>` — [`Bytes`](../index.md)

##### `impl<'data> Copy for Bytes<'data>`

##### `impl<'data> Debug for Bytes<'data>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data> Default for Bytes<'data>`

- `fn default() -> Bytes<'data>` — [`Bytes`](../index.md)

##### `impl<'data> Eq for Bytes<'data>`

##### `impl<'data> PartialEq for Bytes<'data>`

- `fn eq(self: &Self, other: &Bytes<'data>) -> bool` — [`Bytes`](../index.md)

##### `impl<'data> StructuralPartialEq for Bytes<'data>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

#### Trait Implementations

##### `impl Debug for DebugByte`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

#### Trait Implementations

##### `impl Debug for DebugLen`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ByteString<'data>`

```rust
struct ByteString<'data>(&'data [u8]);
```

A newtype for byte strings.

For byte slices that are strings of an unknown encoding.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

#### Trait Implementations

##### `impl<'data> Clone for ByteString<'data>`

- `fn clone(self: &Self) -> ByteString<'data>` — [`ByteString`](#bytestring)

##### `impl<'data> Copy for ByteString<'data>`

##### `impl<'data> Debug for ByteString<'data>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data> Default for ByteString<'data>`

- `fn default() -> ByteString<'data>` — [`ByteString`](#bytestring)

##### `impl<'data> Eq for ByteString<'data>`

##### `impl<'data> PartialEq for ByteString<'data>`

- `fn eq(self: &Self, other: &ByteString<'data>) -> bool` — [`ByteString`](#bytestring)

##### `impl<'data> StructuralPartialEq for ByteString<'data>`

### `StringTable<'data, R>`

```rust
struct StringTable<'data, R>
where
    R: ReadRef<'data> {
    data: Option<R>,
    start: u64,
    end: u64,
    marker: core::marker::PhantomData<&'data ()>,
}
```

A table of zero-terminated strings.

This is used by most file formats for strings such as section names and symbol names.

#### Implementations

- `fn new(data: R, start: u64, end: u64) -> Self`

- `fn get(self: &Self, offset: u32) -> Result<&'data [u8], ()>`

#### Trait Implementations

##### `impl<'data, R> Clone for StringTable<'data, R>`

- `fn clone(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

##### `impl<'data, R> Copy for StringTable<'data, R>`

##### `impl<'data, R> Debug for StringTable<'data, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R: ReadRef<'data>> Default for StringTable<'data, R>`

- `fn default() -> Self`

## Functions

### `debug_list_bytes`

```rust
fn debug_list_bytes(bytes: &[u8], fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

### `align`

```rust
fn align(offset: usize, size: usize) -> usize
```

### `data_range`

```rust
fn data_range(data: &[u8], data_address: u64, range_address: u64, size: u64) -> Option<&[u8]>
```

