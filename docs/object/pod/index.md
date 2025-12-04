*[object](../index.md) / [pod](index.md)*

---

# Module `pod`

Tools for converting file format structures to and from bytes.

This module should be replaced once rust provides safe transmutes.

## Traits

### `Pod`

```rust
trait Pod: Copy + 'static { ... }
```

A trait for types that can safely be converted from and to byte slices.

# Safety
A type that is `Pod` must:
- be `#[repr(C)]` or `#[repr(transparent)]`
- have no invalid byte values
- have no padding

## Functions

### `from_bytes`

```rust
fn from_bytes<T: Pod>(data: &[u8]) -> result::Result<(&T, &[u8]), ()>
```

Cast the head of a byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `from_bytes_mut`

```rust
fn from_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<(&mut T, &mut [u8]), ()>
```

Cast the head of a mutable byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes`

```rust
fn slice_from_bytes<T: Pod>(data: &[u8], count: usize) -> result::Result<(&[T], &[u8]), ()>
```

Cast the head of a byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes_mut`

```rust
fn slice_from_bytes_mut<T: Pod>(data: &mut [u8], count: usize) -> result::Result<(&mut [T], &mut [u8]), ()>
```

Cast the head of a mutable byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_all_bytes`

```rust
fn slice_from_all_bytes<T: Pod>(data: &[u8]) -> result::Result<&[T], ()>
```

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `slice_from_all_bytes_mut`

```rust
fn slice_from_all_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<&mut [T], ()>
```

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `bytes_of`

```rust
fn bytes_of<T: Pod>(val: &T) -> &[u8]
```

Cast a `Pod` type to a byte slice.

### `bytes_of_mut`

```rust
fn bytes_of_mut<T: Pod>(val: &mut T) -> &mut [u8]
```

Cast a `Pod` type to a mutable byte slice.

### `bytes_of_slice`

```rust
fn bytes_of_slice<T: Pod>(val: &[T]) -> &[u8]
```

Cast a slice of a `Pod` type to a byte slice.

### `bytes_of_slice_mut`

```rust
fn bytes_of_slice_mut<T: Pod>(val: &mut [T]) -> &mut [u8]
```

Cast a slice of a `Pod` type to a mutable byte slice.

