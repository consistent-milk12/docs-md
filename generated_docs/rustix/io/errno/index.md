*[rustix](../../index.md) / [io](../index.md) / [errno](index.md)*

---

# Module `errno`

The `Errno` type, which is a minimal wrapper around an error code.

We define the error constants as individual `const`s instead of an enum
because we may not know about all of the host's error values and we don't
want unrecognized values to create undefined behavior.

## Structs

### `Errno`

```rust
struct Errno(u16);
```

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../../addr2line/index.md),
but only holds an OS error code, and no extra error value.

# References
 - [POSIX]
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)










#### Implementations

- `fn from_io_error(io_err: &std::io::Error) -> Option<Self>`

- `const fn raw_os_error(self: Self) -> i32`

- `const fn from_raw_os_error(raw: i32) -> Self`

- `const fn from_errno(raw: u32) -> Self`

#### Trait Implementations

##### `impl Clone for Errno`

- `fn clone(self: &Self) -> Errno` — [`Errno`](../index.md)

##### `impl Copy for Errno`

##### `impl Debug for Errno`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Errno`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Errno`

##### `impl Error for Errno`

##### `impl Hash for Errno`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Errno`

- `fn eq(self: &Self, other: &Errno) -> bool` — [`Errno`](../index.md)

##### `impl StructuralPartialEq for Errno`

##### `impl<T> ToString for Errno`

- `fn to_string(self: &Self) -> String`

## Functions

### `retry_on_intr`

```rust
fn retry_on_intr<T, F: FnMut() -> Result<T>>(f: F) -> Result<T>
```

Call `f` until it either succeeds or fails other than `Errno::INTR`.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Errno>;
```

A specialized [`Result`](../index.md) type for `rustix` APIs.

