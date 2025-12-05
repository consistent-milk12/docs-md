*[rustix](../index.md) / [buffer](index.md)*

---

# Module `buffer`

Utilities for functions that return data via buffers.

## Structs

### `SpareCapacity<'a, T>`

```rust
struct SpareCapacity<'a, T>();
```

A type that implements [`Buffer`](#buffer) by appending to a `Vec`, up to its
capacity.

To use this, use the [`spare_capacity`](#spare-capacity) function.

Because this uses the capacity, and never reallocates, the `Vec` should
have some non-empty spare capacity.

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

##### `impl Buffer<'a, T>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Buffer<T>`

```rust
trait Buffer<T>: private::Sealed<T> { ... }
```

A memory buffer that may be uninitialized.

There are three types that implement the `Buffer` trait, and the type you
use determines the return type of the functions that use it:

| If you pass a…           | You get back a… |
| ------------------------ | --------------- |
| `&mut [u8]`              | `usize`, indicating the number of elements initialized. |
| `&mut [MaybeUninit<u8>]` | `(&mut [u8], &mut [MaybeUninit<u8>])`, holding the initialized and uninitialized subslices. |
| [`SpareCapacity`](#sparecapacity)        | `usize`, indicating the number of elements initialized. And the `Vec` is extended. |

# Examples

Passing a `&mut [u8]`:

```rust
use rustix::io::read;
fn example(fd: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
let mut buf = [0_u8; 64];
let nread = read(fd, &mut buf)?;
// `nread` is the number of bytes read.
Ok(())
}
```

Passing a `&mut [MaybeUninit<u8>]`:

```rust
use rustix::io::read;
use std::mem::MaybeUninit;
fn example(fd: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
let mut buf = [MaybeUninit::<u8>::uninit(); 64];
let (init, uninit) = read(fd, &mut buf)?;
// `init` is a `&mut [u8](#u8)` with the initialized bytes.
// `uninit` is a `&mut [MaybeUninit<u8>]` with the remaining bytes.
Ok(())
}
```

Passing a [`SpareCapacity`](#sparecapacity), via the [`spare_capacity`](#spare-capacity) helper function:

```rust
use rustix::io::read;
use rustix::buffer::spare_capacity;
fn example(fd: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
let mut buf = Vec::with_capacity(64);
let nread = read(fd, spare_capacity(&mut buf))?;
// `nread` is the number of bytes read.
// Also, `buf.len()` is now `nread` elements longer than it was before.
Ok(())
}
```

# Guide to error messages

Sometimes code using `Buffer` can encounter non-obvious error messages.
Here are some we've encountered, along with ways to fix them.

If you see errors like
"cannot move out of `self` which is behind a mutable reference"
and
"move occurs because `x` has type `&mut [u8]`, which does not implement the `Copy` trait",
replace `x` with `&mut *x`. See `error_buffer_wrapper` in
examples/buffer_errors.rs.

If you see errors like
"type annotations needed"
and
"cannot infer type of the type parameter `Buf` declared on the function `read`",
you may need to change a `&mut []` to `&mut [0_u8; 0]`. See
`error_empty_slice` in examples/buffer_errors.rs.

If you see errors like
"the trait bound `[MaybeUninit<u8>; 1]: Buffer<u8>` is not satisfied",
add a `&mut` to pass the array by reference instead of by value. See
`error_array_by_value` in examples/buffer_errors.rs.

If you see errors like
"cannot move out of `x`, a captured variable in an `FnMut` closure",
try replacing `x` with `&mut *x`, or, if that doesn't work, try moving a
`let` into the closure body. See `error_retry_closure` and
`error_retry_indirect_closure` in examples/buffer_errors.rs.

If you see errors like
"captured variable cannot escape `FnMut` closure body",
use an explicit loop instead of `retry_on_intr`, assuming you're using
that. See `error_retry_closure_uninit` in examples/buffer_errors.rs.

## Functions

### `spare_capacity`

```rust
fn spare_capacity<'a, T>(v: &'a mut alloc::vec::Vec<T>) -> SpareCapacity<'a, T>
```

Construct an [`SpareCapacity`](#sparecapacity), which implements [`Buffer`](#buffer).

This wraps a `Vec` and uses the spare capacity of the `Vec` as the buffer
to receive data in, automatically calling `set_len` on the `Vec` to set the
length to include the received elements.

This uses the existing capacity, and never allocates, so the `Vec` should
have some non-empty spare capacity!

# Examples

```rust
fn test(input: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {
use rustix::buffer::spare_capacity;
use rustix::io::{read, Errno};

let mut buf = Vec::with_capacity(1024);
match read(input, spare_capacity(&mut buf)) {
    Ok(0) => { /* end of stream */ }
    Ok(n) => { /* `buf` is now `n` bytes longer */ }
    Err(Errno::INTR) => { /* `buf` is unmodified */ }
    Err(e) => {
        return Err(e);
    }
}

Ok(())
}
```

