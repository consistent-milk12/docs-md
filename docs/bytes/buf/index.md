*[bytes](../index.md) / [buf](index.md)*

---

# Module `buf`

Utilities for working with buffers.

A buffer is any structure that contains a sequence of bytes. The bytes may
or may not be stored in contiguous memory. This module contains traits used
to abstract over buffers as well as utilities for working with buffer types.

# `Buf`, `BufMut`

These are the two foundational traits for abstractly working with buffers.
They can be thought as iterators for byte structures. They offer additional
performance over `Iterator` by providing an API optimized for byte slices.

See [`Buf`](../index.md) and [`BufMut`](../index.md) for more details.

[rope](#rope): https://en.wikipedia.org/wiki/Rope_(data_structure)

## Structs

### `Chain<T, U>`

```rust
struct Chain<T, U> {
    // [REDACTED: Private Fields]
}
```

A `Chain` sequences two buffers.

`Chain` is an adapter that links two underlying buffers and provides a
continuous view across both buffers. It is able to sequence either immutable
buffers ([`Buf`](../index.md) values) or mutable buffers ([`BufMut`](../index.md) values).

This struct is generally created by calling `Buf::chain`. Please see that
function's documentation for more detail.

# Examples

```rust
use bytes::{Bytes, Buf};

let mut buf = (&b"hello "[..])
    .chain(&b"world"[..]);

let full: Bytes = buf.copy_to_bytes(11);
assert_eq!(full[..], b"hello world"[..]);
```


#### Implementations

- `fn first_ref(self: &Self) -> &T`
  Gets a reference to the first underlying `Buf`.

- `fn first_mut(self: &mut Self) -> &mut T`
  Gets a mutable reference to the first underlying `Buf`.

- `fn last_ref(self: &Self) -> &U`
  Gets a reference to the last underlying `Buf`.

- `fn last_mut(self: &mut Self) -> &mut U`
  Gets a mutable reference to the last underlying `Buf`.

- `fn into_inner(self: Self) -> (T, U)`
  Consumes this `Chain`, returning the underlying values.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<T, U>`

- `type Item = u8`

- `type IntoIter = IntoIter<Chain<T, U>>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Buf<T, U>`

- `fn remaining(self: &Self) -> usize`

- `fn chunk(self: &Self) -> &[u8]`

- `fn advance(self: &mut Self, cnt: usize)`

- `fn chunks_vectored<'a>(self: &'a Self, dst: &mut [IoSlice<'a>]) -> usize`

- `fn copy_to_bytes(self: &mut Self, len: usize) -> crate::Bytes`

##### `impl BufMut<T, U>`

- `fn remaining_mut(self: &Self) -> usize`

- `fn chunk_mut(self: &mut Self) -> &mut UninitSlice`

- `unsafe fn advance_mut(self: &mut Self, cnt: usize)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug, U: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    // [REDACTED: Private Fields]
}
```

Iterator over the bytes contained by the buffer.

# Examples

Basic usage:

```rust
use bytes::Bytes;

let buf = Bytes::from(&b"abc"[..]);
let mut iter = buf.into_iter();

assert_eq!(iter.next(), Some(b'a'));
assert_eq!(iter.next(), Some(b'b'));
assert_eq!(iter.next(), Some(b'c'));
assert_eq!(iter.next(), None);
```

#### Implementations

- `fn new(inner: T) -> IntoIter<T>`
  Creates an iterator over the bytes contained by the buffer.

- `fn into_inner(self: Self) -> T`
  Consumes this `IntoIter`, returning the underlying value.

- `fn get_ref(self: &Self) -> &T`
  Gets a reference to the underlying `Buf`.

- `fn get_mut(self: &mut Self) -> &mut T`
  Gets a mutable reference to the underlying `Buf`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl ExactSizeIterator<T: Buf>`

##### `impl Iterator<T: Buf>`

- `type Item = u8`

- `fn next(self: &mut Self) -> Option<u8>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Limit<T>`

```rust
struct Limit<T> {
    // [REDACTED: Private Fields]
}
```

A `BufMut` adapter which limits the amount of bytes that can be written
to an underlying buffer.

#### Implementations

- `fn into_inner(self: Self) -> T`
  Consumes this `Limit`, returning the underlying value.

- `fn get_ref(self: &Self) -> &T`
  Gets a reference to the underlying `BufMut`.

- `fn get_mut(self: &mut Self) -> &mut T`
  Gets a mutable reference to the underlying `BufMut`.

- `fn limit(self: &Self) -> usize`
  Returns the maximum number of bytes that can be written

- `fn set_limit(self: &mut Self, lim: usize)`
  Sets the maximum number of bytes that can be written.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl BufMut<T: BufMut>`

- `fn remaining_mut(self: &Self) -> usize`

- `fn chunk_mut(self: &mut Self) -> &mut UninitSlice`

- `unsafe fn advance_mut(self: &mut Self, cnt: usize)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Take<T>`

```rust
struct Take<T> {
    // [REDACTED: Private Fields]
}
```

A `Buf` adapter which limits the bytes read from an underlying buffer.

This struct is generally created by calling `take()` on `Buf`. See
documentation of [`take()`](Buf::take) for more details.

#### Implementations

- `fn into_inner(self: Self) -> T`
  Consumes this `Take`, returning the underlying value.

- `fn get_ref(self: &Self) -> &T`
  Gets a reference to the underlying `Buf`.

- `fn get_mut(self: &mut Self) -> &mut T`
  Gets a mutable reference to the underlying `Buf`.

- `fn limit(self: &Self) -> usize`
  Returns the maximum number of bytes that can be read.

- `fn set_limit(self: &mut Self, lim: usize)`
  Sets the maximum number of bytes that can be read.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Buf<T: Buf>`

- `fn remaining(self: &Self) -> usize`

- `fn chunk(self: &Self) -> &[u8]`

- `fn advance(self: &mut Self, cnt: usize)`

- `fn copy_to_bytes(self: &mut Self, len: usize) -> crate::Bytes`

- `fn chunks_vectored<'a>(self: &'a Self, dst: &mut [IoSlice<'a>]) -> usize`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `UninitSlice`

```rust
struct UninitSlice();
```

Uninitialized byte slice.

Returned by `BufMut::chunk_mut()`, the referenced byte slice may be
uninitialized. The wrapper provides safe access without introducing
undefined behavior.

The safety invariants of this wrapper are:

 1. Reading from an `UninitSlice` is undefined behavior.
 2. Writing uninitialized bytes to an `UninitSlice` is undefined behavior.

The difference between `&mut UninitSlice` and `&mut [MaybeUninit<u8>]` is
that it is possible in safe code to write uninitialized bytes to an
`&mut [MaybeUninit<u8>]`, which this type prohibits.

#### Implementations

- `fn new(slice: &mut [u8]) -> &mut UninitSlice`
  Creates a `&mut UninitSlice` wrapping a slice of initialised memory.

- `fn uninit(slice: &mut [MaybeUninit<u8>]) -> &mut UninitSlice`
  Creates a `&mut UninitSlice` wrapping a slice of uninitialised memory.

- `unsafe fn from_raw_parts_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut UninitSlice`
  Create a `&mut UninitSlice` from a pointer and a length.

- `fn write_byte(self: &mut Self, index: usize, byte: u8)`
  Write a single byte at the specified offset.

- `fn copy_from_slice(self: &mut Self, src: &[u8])`
  Copies bytes from `src` into `self`.

- `fn as_mut_ptr(self: &mut Self) -> *mut u8`
  Return a raw pointer to the slice's buffer.

- `unsafe fn as_uninit_slice_mut(self: &mut Self) -> &mut [MaybeUninit<u8>]`
  Return a `&mut [MaybeUninit<u8>]` to this slice's buffer.

- `fn len(self: &Self) -> usize`
  Returns the number of bytes in the slice.

#### Trait Implementations

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Index`

- `type Output = UninitSlice`

- `fn index(self: &Self, index: Range<usize>) -> &UninitSlice`

##### `impl Index`

- `type Output = UninitSlice`

- `fn index(self: &Self, index: RangeToInclusive<usize>) -> &UninitSlice`

##### `impl Index`

- `type Output = UninitSlice`

- `fn index(self: &Self, index: RangeFrom<usize>) -> &UninitSlice`

##### `impl Index`

- `type Output = UninitSlice`

- `fn index(self: &Self, index: RangeInclusive<usize>) -> &UninitSlice`

##### `impl Index`

- `type Output = UninitSlice`

- `fn index(self: &Self, index: RangeFull) -> &UninitSlice`

##### `impl Index`

- `type Output = UninitSlice`

- `fn index(self: &Self, index: RangeTo<usize>) -> &UninitSlice`

##### `impl IndexMut`

- `fn index_mut(self: &mut Self, index: RangeTo<usize>) -> &mut UninitSlice`

##### `impl IndexMut`

- `fn index_mut(self: &mut Self, index: RangeFrom<usize>) -> &mut UninitSlice`

##### `impl IndexMut`

- `fn index_mut(self: &mut Self, index: Range<usize>) -> &mut UninitSlice`

##### `impl IndexMut`

- `fn index_mut(self: &mut Self, index: RangeToInclusive<usize>) -> &mut UninitSlice`

##### `impl IndexMut`

- `fn index_mut(self: &mut Self, index: RangeFull) -> &mut UninitSlice`

##### `impl IndexMut`

- `fn index_mut(self: &mut Self, index: RangeInclusive<usize>) -> &mut UninitSlice`

##### `impl Debug`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Reader<B>`

```rust
struct Reader<B> {
    // [REDACTED: Private Fields]
}
```

A `Buf` adapter which implements `io::Read` for the inner value.

This struct is generally created by calling `reader()` on `Buf`. See
documentation of [`reader()`](Buf::reader) for more
details.

#### Implementations

- `fn get_ref(self: &Self) -> &B`
  Gets a reference to the underlying `Buf`.

- `fn get_mut(self: &mut Self) -> &mut B`
  Gets a mutable reference to the underlying `Buf`.

- `fn into_inner(self: Self) -> B`
  Consumes this `Reader`, returning the underlying value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl BufRead<B: Buf + Sized>`

- `fn fill_buf(self: &mut Self) -> io::Result<&[u8]>`

- `fn consume(self: &mut Self, amt: usize)`

##### `impl Read<B: Buf + Sized>`

- `fn read(self: &mut Self, dst: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<B: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Writer<B>`

```rust
struct Writer<B> {
    // [REDACTED: Private Fields]
}
```

A `BufMut` adapter which implements `io::Write` for the inner value.

This struct is generally created by calling `writer()` on `BufMut`. See
documentation of [`writer()`](BufMut::writer) for more
details.

#### Implementations

- `fn get_ref(self: &Self) -> &B`
  Gets a reference to the underlying `BufMut`.

- `fn get_mut(self: &mut Self) -> &mut B`
  Gets a mutable reference to the underlying `BufMut`.

- `fn into_inner(self: Self) -> B`
  Consumes this `Writer`, returning the underlying value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<B: BufMut + Sized>`

- `fn write(self: &mut Self, src: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<B: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

