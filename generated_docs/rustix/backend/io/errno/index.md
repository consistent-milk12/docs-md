*[rustix](../../../index.md) / [backend](../../index.md) / [io](../index.md) / [errno](index.md)*

---

# Module `errno`

The `rustix` `Errno` type.

This type holds an OS error code, which conceptually corresponds to an
`errno` value.

# Safety

Linux uses error codes in `-4095..0`; we use rustc attributes to describe
this restricted range of values.

## Contents

- [Structs](#structs)
  - [`Errno`](#errno)
- [Functions](#functions)
  - [`try_decode_c_int`](#try_decode_c_int)
  - [`try_decode_c_uint`](#try_decode_c_uint)
  - [`try_decode_usize`](#try_decode_usize)
  - [`try_decode_void_star`](#try_decode_void_star)
  - [`try_decode_u64`](#try_decode_u64)
  - [`try_decode_raw_fd`](#try_decode_raw_fd)
  - [`try_decode_void`](#try_decode_void)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Errno`](#errno) | struct | `errno`—An error code. |
| [`try_decode_c_int`](#try_decode_c_int) | fn | Check for an error from the result of a syscall which encodes a |
| [`try_decode_c_uint`](#try_decode_c_uint) | fn | Check for an error from the result of a syscall which encodes a |
| [`try_decode_usize`](#try_decode_usize) | fn | Check for an error from the result of a syscall which encodes a `usize` on |
| [`try_decode_void_star`](#try_decode_void_star) | fn | Check for an error from the result of a syscall which encodes a |
| [`try_decode_u64`](#try_decode_u64) | fn | Check for an error from the result of a syscall which encodes a |
| [`try_decode_raw_fd`](#try_decode_raw_fd) | fn | Check for an error from the result of a syscall which encodes a file |
| [`try_decode_void`](#try_decode_void) | fn | Check for an error from the result of a syscall which encodes no value on |

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

- <span id="errno-kind"></span>`fn kind(self) -> std::io::ErrorKind`

#### Trait Implementations

##### `impl Clone for Errno`

- <span id="errno-clone"></span>`fn clone(&self) -> Errno` — [`Errno`](../../../io/index.md)

##### `impl Copy for Errno`

##### `impl Debug for Errno`

- <span id="errno-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Errno`

- <span id="errno-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Errno`

##### `impl Error for Errno`

##### `impl Hash for Errno`

- <span id="errno-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Errno`

- <span id="errno-eq"></span>`fn eq(&self, other: &Errno) -> bool` — [`Errno`](../../../io/index.md)

##### `impl StructuralPartialEq for Errno`

##### `impl<T> ToString for Errno`

- <span id="errno-to-string"></span>`fn to_string(&self) -> String`

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

