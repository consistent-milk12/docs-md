# Crate `bytes`

Provides abstractions for working with bytes.

The `bytes` crate provides an efficient byte buffer structure
([`Bytes`](#bytes)) and traits for working with buffer
implementations ([`Buf`](#buf), [`BufMut`](#bufmut)).

# `Bytes`

`Bytes` is an efficient container for storing and operating on contiguous
slices of memory. It is intended for use primarily in networking code, but
could have applications elsewhere as well.

`Bytes` values facilitate zero-copy network programming by allowing multiple
`Bytes` objects to point to the same underlying memory. This is managed by
using a reference count to track when the memory is no longer needed and can
be freed.

A `Bytes` handle can be created directly from an existing byte store (such as `&[u8]`
or `Vec<u8>`), but usually a `BytesMut` is used first and written to. For
example:

```rust
use bytes::{BytesMut, BufMut};

let mut buf = BytesMut::with_capacity(1024);
buf.put(&b"hello world"[..]);
buf.put_u16(1234);

let a = buf.split();
assert_eq!(a, b"hello world\x04\xD2"[..]);

buf.put(&b"goodbye world"[..]);

let b = buf.split();
assert_eq!(b, b"goodbye world"[..]);

assert_eq!(buf.capacity(), 998);
```

In the above example, only a single buffer of 1024 is allocated. The handles
`a` and `b` will share the underlying buffer and maintain indices tracking
the view into the buffer represented by the handle.

See the [struct docs](`Bytes`) for more details.

# `Buf`, `BufMut`

These two traits provide read and write access to buffers. The underlying
storage may or may not be in contiguous memory. For example, `Bytes` is a
buffer that guarantees contiguous memory, but a [rope](#rope) stores the bytes in
disjoint chunks. `Buf` and `BufMut` maintain cursors tracking the current
position in the underlying byte storage. When bytes are read or written, the
cursor is advanced.

[rope](#rope): https://en.wikipedia.org/wiki/Rope_(data_structure)

## Relation with `Read` and `Write`

At first glance, it may seem that `Buf` and `BufMut` overlap in
functionality with `std::io::Read` and `std::io::Write`. However, they
serve different purposes. A buffer is the value that is provided as an
argument to `Read::read` and `Write::write`. `Read` and `Write` may then
perform a syscall, which has the potential of failing. Operations on `Buf`
and `BufMut` are infallible.

## Modules

- [`buf`](buf/index.md) - Utilities for working with buffers.

## Structs

### `Bytes`

```rust
struct Bytes {
    // [REDACTED: Private Fields]
}
```

A cheaply cloneable and sliceable chunk of contiguous memory.

`Bytes` is an efficient container for storing and operating on contiguous
slices of memory. It is intended for use primarily in networking code, but
could have applications elsewhere as well.

`Bytes` values facilitate zero-copy network programming by allowing multiple
`Bytes` objects to point to the same underlying memory.

`Bytes` does not have a single implementation. It is an interface, whose
exact behavior is implemented through dynamic dispatch in several underlying
implementations of `Bytes`.

All `Bytes` implementations must fulfill the following requirements:
- They are cheaply cloneable and thereby shareable between an unlimited amount
  of components, for example by modifying a reference count.
- Instances can be sliced to refer to a subset of the original buffer.

```rust
use bytes::Bytes;

let mut mem = Bytes::from("Hello world");
let a = mem.slice(0..5);

assert_eq!(a, "Hello");

let b = mem.split_to(6);

assert_eq!(mem, "world");
assert_eq!(b, "Hello ");
```

# Memory layout

The `Bytes` struct itself is fairly small, limited to 4 `usize` fields used
to track information about which segment of the underlying memory the
`Bytes` handle has access to.

`Bytes` keeps both a pointer to the shared state containing the full memory
slice and a pointer to the start of the region visible by the handle.
`Bytes` also tracks the length of its view into the memory.

# Sharing

`Bytes` contains a vtable, which allows implementations of `Bytes` to define
how sharing/cloning is implemented in detail.
When `Bytes::clone()` is called, `Bytes` will call the vtable function for
cloning the backing storage in order to share it behind multiple `Bytes`
instances.

For `Bytes` implementations which refer to constant memory (e.g. created
via `Bytes::from_static()`) the cloning implementation will be a no-op.

For `Bytes` implementations which point to a reference counted shared storage
(e.g. an `Arc<[u8]>`), sharing will be implemented by increasing the
reference count.

Due to this mechanism, multiple `Bytes` instances may point to the same
shared memory region.
Each `Bytes` instance can point to different sections within that
memory region, and `Bytes` instances may or may not have overlapping views
into the memory.

The following diagram visualizes a scenario where 2 `Bytes` instances make
use of an `Arc`-based backing storage, and provide access to different views:

```text

   Arc ptrs                   ┌─────────┐
   ________________________ / │ Bytes 2 │
  /                           └─────────┘
 /          ┌───────────┐     |         |
|_________/ │  Bytes 1  │     |         |
|           └───────────┘     |         |
|           |           | ___/ data     | tail
|      data |      tail |/              |
v           v           v               v
┌─────┬─────┬───────────┬───────────────┬─────┐
│ Arc │     │           │               │     │
└─────┴─────┴───────────┴───────────────┴─────┘
```

#### Implementations

- `const fn new() -> Self`
  Creates a new empty `Bytes`.

- `const fn from_static(bytes: &'static [u8]) -> Self`
  Creates a new `Bytes` from a static slice.

- `fn from_owner<T>(owner: T) -> Self`
  Create [Bytes] with a buffer whose lifetime is controlled

- `const fn len(self: &Self) -> usize`
  Returns the number of bytes contained in this `Bytes`.

- `const fn is_empty(self: &Self) -> bool`
  Returns true if the `Bytes` has a length of 0.

- `fn is_unique(self: &Self) -> bool`
  Returns true if this is the only reference to the data and

- `fn copy_from_slice(data: &[u8]) -> Self`
  Creates `Bytes` instance from slice, by copying it.

- `fn slice(self: &Self, range: impl RangeBounds<usize>) -> Self`
  Returns a slice of self for the provided range.

- `fn slice_ref(self: &Self, subset: &[u8]) -> Self`
  Returns a slice of self that is equivalent to the given `subset`.

- `fn split_off(self: &mut Self, at: usize) -> Self`
  Splits the bytes into two at the given index.

- `fn split_to(self: &mut Self, at: usize) -> Self`
  Splits the bytes into two at the given index.

- `fn truncate(self: &mut Self, len: usize)`
  Shortens the buffer, keeping the first `len` bytes and dropping the

- `fn clear(self: &mut Self)`
  Clears the buffer, removing all data.

- `fn try_into_mut(self: Self) -> Result<BytesMut, Bytes>`
  Try to convert self into `BytesMut`.

#### Trait Implementations

##### `impl From`

- `fn from(slice: Box<[u8]>) -> Bytes`

##### `impl From`

- `fn from(s: String) -> Bytes`

##### `impl From`

- `fn from(vec: Vec<u8>) -> Bytes`

##### `impl From`

- `fn from(slice: &'static [u8]) -> Bytes`

##### `impl From`

- `fn from(src: BytesMut) -> Bytes`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(slice: &'static str) -> Bytes`

##### `impl FromIterator`

- `fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator`

- `type Item = u8`

- `type IntoIter = IntoIter<Bytes>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow`

- `fn borrow(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Buf`

- `fn remaining(self: &Self) -> usize`

- `fn chunk(self: &Self) -> &[u8]`

- `fn advance(self: &mut Self, cnt: usize)`

- `fn copy_to_bytes(self: &mut Self, len: usize) -> Self`

##### `impl Clone`

- `fn clone(self: &Self) -> Bytes`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl LowerHex`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Bytes) -> cmp::Ordering`

##### `impl PartialEq<'a, T: ?Sized>`

- `fn eq(self: &Self, other: &&'a T) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &[u8]) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &BytesMut) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Bytes) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Vec<u8>) -> bool`

##### `impl PartialOrd<'a, T: ?Sized>`

- `fn partial_cmp(self: &Self, other: &&'a T) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Bytes) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &[u8]) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Vec<u8>) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sync`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result`

##### `impl Default`

- `fn default() -> Bytes`

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &[u8]`

##### `impl Send`

### `BytesMut`

```rust
struct BytesMut {
    // [REDACTED: Private Fields]
}
```

A unique reference to a contiguous slice of memory.

`BytesMut` represents a unique view into a potentially shared memory region.
Given the uniqueness guarantee, owners of `BytesMut` handles are able to
mutate the memory.

`BytesMut` can be thought of as containing a `buf: Arc<Vec<u8>>`, an offset
into `buf`, a slice length, and a guarantee that no other `BytesMut` for the
same `buf` overlaps with its slice. That guarantee means that a write lock
is not required.

# Growth

`BytesMut`'s `BufMut` implementation will implicitly grow its buffer as
necessary. However, explicitly reserving the required space up-front before
a series of inserts will be more efficient.

# Examples

```rust
use bytes::{BytesMut, BufMut};

let mut buf = BytesMut::with_capacity(64);

buf.put_u8(b'h');
buf.put_u8(b'e');
buf.put(&b"llo"[..]);

assert_eq!(&buf[..], b"hello");

// Freeze the buffer so that it can be shared
let a = buf.freeze();

// This does not allocate, instead `b` points to the same memory.
let b = a.clone();

assert_eq!(&a[..], b"hello");
assert_eq!(&b[..], b"hello");
```

#### Implementations

- `fn with_capacity(capacity: usize) -> BytesMut`
  Creates a new `BytesMut` with the specified capacity.

- `fn new() -> BytesMut`
  Creates a new `BytesMut` with default capacity.

- `fn len(self: &Self) -> usize`
  Returns the number of bytes contained in this `BytesMut`.

- `fn is_empty(self: &Self) -> bool`
  Returns true if the `BytesMut` has a length of 0.

- `fn capacity(self: &Self) -> usize`
  Returns the number of bytes the `BytesMut` can hold without reallocating.

- `fn freeze(self: Self) -> Bytes`
  Converts `self` into an immutable `Bytes`.

- `fn zeroed(len: usize) -> BytesMut`
  Creates a new `BytesMut` containing `len` zeros.

- `fn split_off(self: &mut Self, at: usize) -> BytesMut`
  Splits the bytes into two at the given index.

- `fn split(self: &mut Self) -> BytesMut`
  Removes the bytes from the current view, returning them in a new

- `fn split_to(self: &mut Self, at: usize) -> BytesMut`
  Splits the buffer into two at the given index.

- `fn truncate(self: &mut Self, len: usize)`
  Shortens the buffer, keeping the first `len` bytes and dropping the

- `fn clear(self: &mut Self)`
  Clears the buffer, removing all data. Existing capacity is preserved.

- `fn resize(self: &mut Self, new_len: usize, value: u8)`
  Resizes the buffer so that `len` is equal to `new_len`.

- `unsafe fn set_len(self: &mut Self, len: usize)`
  Sets the length of the buffer.

- `fn reserve(self: &mut Self, additional: usize)`
  Reserves capacity for at least `additional` more bytes to be inserted

- `fn try_reclaim(self: &mut Self, additional: usize) -> bool`
  Attempts to cheaply reclaim already allocated capacity for at least `additional` more

- `fn extend_from_slice(self: &mut Self, extend: &[u8])`
  Appends given bytes to this `BytesMut`.

- `fn unsplit(self: &mut Self, other: BytesMut)`
  Absorbs a `BytesMut` that was previously split off.

- `fn spare_capacity_mut(self: &mut Self) -> &mut [MaybeUninit<u8>]`
  Returns the remaining spare capacity of the buffer as a slice of `MaybeUninit<u8>`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(bytes: Bytes) -> Self`
  Convert self into `BytesMut`.

##### `impl From<'a>`

- `fn from(src: &'a [u8]) -> BytesMut`

##### `impl From<'a>`

- `fn from(src: &'a str) -> BytesMut`

##### `impl FromIterator`

- `fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self`

##### `impl FromIterator<'a>`

- `fn from_iter<T: IntoIterator<Item = &'a u8>>(into_iter: T) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator`

- `type Item = u8`

- `type IntoIter = IntoIter<BytesMut>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsMut`

- `fn as_mut(self: &mut Self) -> &mut [u8]`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl Borrow`

- `fn borrow(self: &Self) -> &[u8]`

##### `impl BorrowMut`

- `fn borrow_mut(self: &mut Self) -> &mut [u8]`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Buf`

- `fn remaining(self: &Self) -> usize`

- `fn chunk(self: &Self) -> &[u8]`

- `fn advance(self: &mut Self, cnt: usize)`

- `fn copy_to_bytes(self: &mut Self, len: usize) -> Bytes`

##### `impl BufMut`

- `fn remaining_mut(self: &Self) -> usize`

- `unsafe fn advance_mut(self: &mut Self, cnt: usize)`

- `fn chunk_mut(self: &mut Self) -> &mut UninitSlice`

- `fn put<T: Buf>(self: &mut Self, src: T)`

- `fn put_slice(self: &mut Self, src: &[u8])`

- `fn put_bytes(self: &mut Self, val: u8, cnt: usize)`

##### `impl Clone`

- `fn clone(self: &Self) -> BytesMut`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl Eq`

##### `impl Extend`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl Extend<'a>`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl Extend`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl LowerHex`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result`

##### `impl Ord`

- `fn cmp(self: &Self, other: &BytesMut) -> cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Bytes) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &BytesMut) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq<'a, T: ?Sized>`

- `fn eq(self: &Self, other: &&'a T) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &[u8]) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Vec<u8>) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`

##### `impl PartialOrd<'a, T: ?Sized>`

- `fn partial_cmp(self: &Self, other: &&'a T) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &[u8]) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &BytesMut) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Vec<u8>) -> Option<cmp::Ordering>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sync`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result`

##### `impl Write`

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

- `fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result`

##### `impl Default`

- `fn default() -> BytesMut`

##### `impl Deref`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &[u8]`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut [u8]`

##### `impl Send`

### `TryGetError`

```rust
struct TryGetError {
    pub requested: usize,
    pub available: usize,
}
```

Error type for the `try_get_` methods of [`Buf`](#buf).
Indicates that there were not enough remaining
bytes in the buffer while attempting
to get a value from a [`Buf`](#buf) with one
of the `try_get_` methods.

#### Fields

- **`requested`**: `usize`

  The number of bytes necessary to get the value

- **`available`**: `usize`

  The number of bytes available in the buffer

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TryGetError) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

