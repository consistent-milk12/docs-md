*[rustix](../../../index.md) / [backend](../../index.md) / [io](../index.md) / [errno](index.md)*

---

# Module `errno`

The `rustix` `Errno` type.

This type holds an OS error code, which conceptually corresponds to an
`errno` value.

# Safety

Linux uses error codes in `-4095..0`; we use rustc attributes to describe
this restricted range of values.

## Structs

### `Errno`

```rust
struct Errno(u16);
```

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../../../addr2line/index.md),
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
 - [`glibc`](../../../../libc/new/glibc/index.md)










#### Implementations

- `fn from_io_error(io_err: &std::io::Error) -> Option<Self>`

- `const fn raw_os_error(self: Self) -> i32`

- `const fn from_raw_os_error(raw: i32) -> Self`

- `const fn from_errno(raw: u32) -> Self`

#### Trait Implementations

##### `impl Clone for Errno`

- `fn clone(self: &Self) -> Errno` — [`Errno`](../../../io/index.md)

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

- `fn eq(self: &Self, other: &Errno) -> bool` — [`Errno`](../../../io/index.md)

##### `impl StructuralPartialEq for Errno`

##### `impl<T> ToString for Errno`

- `fn to_string(self: &Self) -> String`

## Functions

### `try_decode_c_int`

```rust
fn try_decode_c_int<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<c::c_int>
```

Check for an error from the result of a syscall which encodes a
`c::c_int` on success.

### `try_decode_c_uint`

```rust
fn try_decode_c_uint<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<c::c_uint>
```

Check for an error from the result of a syscall which encodes a
`c::c_uint` on success.

### `try_decode_usize`

```rust
fn try_decode_usize<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<usize>
```

Check for an error from the result of a syscall which encodes a `usize` on
success.

### `try_decode_void_star`

```rust
fn try_decode_void_star<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<*mut c::c_void>
```

Check for an error from the result of a syscall which encodes a
`*mut c_void` on success.

### `try_decode_u64`

```rust
fn try_decode_u64<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<u64>
```

Check for an error from the result of a syscall which encodes a
`u64` on success.

### `try_decode_raw_fd`

```rust
unsafe fn try_decode_raw_fd<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<crate::backend::fd::RawFd>
```

Check for an error from the result of a syscall which encodes a file
descriptor on success.

# Safety

This must only be used with syscalls which return file descriptors on
success.

### `try_decode_void`

```rust
unsafe fn try_decode_void<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<()>
```

Check for an error from the result of a syscall which encodes no value on
success. On success, return the unconsumed `raw` value.

# Safety

This must only be used with syscalls which return no value on success.

