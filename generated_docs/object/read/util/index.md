*[object](../../index.md) / [read](../index.md) / [util](index.md)*

---

# Module `util`

## Contents

- [Structs](#structs)
  - [`Bytes`](#bytes)
  - [`DebugByte`](#debugbyte)
  - [`DebugLen`](#debuglen)
  - [`ByteString`](#bytestring)
  - [`StringTable`](#stringtable)
- [Functions](#functions)
  - [`debug_list_bytes`](#debug_list_bytes)
  - [`align`](#align)
  - [`data_range`](#data_range)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bytes`](#bytes) | struct | A newtype for byte slices. |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |
| [`ByteString`](#bytestring) | struct | A newtype for byte strings. |
| [`StringTable`](#stringtable) | struct | A table of zero-terminated strings. |
| [`debug_list_bytes`](#debug_list_bytes) | fn |  |
| [`align`](#align) | fn |  |
| [`data_range`](#data_range) | fn |  |

## Structs

### `Bytes<'data>`

```rust
struct Bytes<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:16`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L16)*

A newtype for byte slices.

It has these important features:
- no methods that can panic, such as `Index`
- convenience methods for `Pod` types
- a useful `Debug` implementation

#### Implementations

- <span id="bytes-len"></span>`fn len(&self) -> usize`

- <span id="bytes-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="bytes-skip"></span>`fn skip(&mut self, offset: usize) -> Result<(), ()>`

- <span id="bytes-read-bytes"></span>`fn read_bytes(&mut self, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](../index.md#bytes)

- <span id="bytes-read-bytes-at"></span>`fn read_bytes_at(self, offset: usize, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](../index.md#bytes)

- <span id="bytes-read"></span>`fn read<T: Pod>(&mut self) -> Result<&'data T, ()>`

- <span id="bytes-read-at"></span>`fn read_at<T: Pod>(self, offset: usize) -> Result<&'data T, ()>`

- <span id="bytes-read-slice"></span>`fn read_slice<T: Pod>(&mut self, count: usize) -> Result<&'data [T], ()>`

- <span id="bytes-read-slice-at"></span>`fn read_slice_at<T: Pod>(self, offset: usize, count: usize) -> Result<&'data [T], ()>`

- <span id="bytes-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8], ()>`

- <span id="bytes-read-string-at"></span>`fn read_string_at(self, offset: usize) -> Result<&'data [u8], ()>`

- <span id="bytes-read-uleb128"></span>`fn read_uleb128(&mut self) -> Result<u64, ()>`

- <span id="bytes-read-sleb128"></span>`fn read_sleb128(&mut self) -> Result<i64, ()>`

#### Trait Implementations

##### `impl Clone for Bytes<'data>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'data>` — [`Bytes`](../index.md#bytes)

##### `impl Copy for Bytes<'data>`

##### `impl Debug for Bytes<'data>`

- <span id="bytes-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bytes<'data>`

- <span id="bytes-default"></span>`fn default() -> Bytes<'data>` — [`Bytes`](../index.md#bytes)

##### `impl Eq for Bytes<'data>`

##### `impl PartialEq for Bytes<'data>`

- <span id="bytes-eq"></span>`fn eq(&self, other: &Bytes<'data>) -> bool` — [`Bytes`](../index.md#bytes)

##### `impl StructuralPartialEq for Bytes<'data>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`object-0.37.3/src/read/util.rs:222`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L222)*

#### Trait Implementations

##### `impl Debug for DebugByte`

- <span id="debugbyte-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

*Defined in [`object-0.37.3/src/read/util.rs:230`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L230)*

#### Trait Implementations

##### `impl Debug for DebugLen`

- <span id="debuglen-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ByteString<'data>`

```rust
struct ByteString<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:244`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L244)*

A newtype for byte strings.

For byte slices that are strings of an unknown encoding.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

#### Trait Implementations

##### `impl Clone for ByteString<'data>`

- <span id="bytestring-clone"></span>`fn clone(&self) -> ByteString<'data>` — [`ByteString`](#bytestring)

##### `impl Copy for ByteString<'data>`

##### `impl Debug for ByteString<'data>`

- <span id="bytestring-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteString<'data>`

- <span id="bytestring-default"></span>`fn default() -> ByteString<'data>` — [`ByteString`](#bytestring)

##### `impl Eq for ByteString<'data>`

##### `impl PartialEq for ByteString<'data>`

- <span id="bytestring-eq"></span>`fn eq(&self, other: &ByteString<'data>) -> bool` — [`ByteString`](#bytestring)

##### `impl StructuralPartialEq for ByteString<'data>`

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

*Defined in [`object-0.37.3/src/read/util.rs:274-282`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L274-L282)*

A table of zero-terminated strings.

This is used by most file formats for strings such as section names and symbol names.

#### Implementations

- <span id="stringtable-new"></span>`fn new(data: R, start: u64, end: u64) -> Self`

- <span id="stringtable-get"></span>`fn get(&self, offset: u32) -> Result<&'data [u8], ()>`

#### Trait Implementations

##### `impl<'data, R> Clone for StringTable<'data, R>`

- <span id="stringtable-clone"></span>`fn clone(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md#stringtable)

##### `impl<'data, R> Copy for StringTable<'data, R>`

##### `impl<'data, R> Debug for StringTable<'data, R>`

- <span id="stringtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, R: ReadRef<'data>> Default for StringTable<'data, R>`

- <span id="stringtable-default"></span>`fn default() -> Self`

## Functions

### `debug_list_bytes`

```rust
fn debug_list_bytes(bytes: &[u8], fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`object-0.37.3/src/read/util.rs:213-220`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L213-L220)*

### `align`

```rust
fn align(offset: usize, size: usize) -> usize
```

*Defined in [`object-0.37.3/src/read/util.rs:254-256`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L254-L256)*

### `data_range`

```rust
fn data_range(data: &[u8], data_address: u64, range_address: u64, size: u64) -> Option<&[u8]>
```

*Defined in [`object-0.37.3/src/read/util.rs:259-268`](../../../../.source_1765210505/object-0.37.3/src/read/util.rs#L259-L268)*

