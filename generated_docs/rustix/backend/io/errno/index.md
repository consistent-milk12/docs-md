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
  - [`try_decode_c_int`](#try-decode-c-int)
  - [`try_decode_c_uint`](#try-decode-c-uint)
  - [`try_decode_usize`](#try-decode-usize)
  - [`try_decode_void_star`](#try-decode-void-star)
  - [`try_decode_u64`](#try-decode-u64)
  - [`try_decode_raw_fd`](#try-decode-raw-fd)
  - [`try_decode_void`](#try-decode-void)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Errno`](#errno) | struct | `errno`—An error code. |
| [`try_decode_c_int`](#try-decode-c-int) | fn | Check for an error from the result of a syscall which encodes a `c::c_int` on success. |
| [`try_decode_c_uint`](#try-decode-c-uint) | fn | Check for an error from the result of a syscall which encodes a `c::c_uint` on success. |
| [`try_decode_usize`](#try-decode-usize) | fn | Check for an error from the result of a syscall which encodes a `usize` on success. |
| [`try_decode_void_star`](#try-decode-void-star) | fn | Check for an error from the result of a syscall which encodes a `*mut c_void` on success. |
| [`try_decode_u64`](#try-decode-u64) | fn | Check for an error from the result of a syscall which encodes a `u64` on success. |
| [`try_decode_raw_fd`](#try-decode-raw-fd) | fn | Check for an error from the result of a syscall which encodes a file descriptor on success. |
| [`try_decode_void`](#try-decode-void) | fn | Check for an error from the result of a syscall which encodes no value on success. |

## Structs

### `Errno`

```rust
struct Errno(u16);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:51`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L51)*

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../../../cargo_docs_md/error/index.md),
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

- <span id="errno-from-io-error"></span>`fn from_io_error(io_err: &std::io::Error) -> Option<Self>`

- <span id="errno-raw-os-error"></span>`const fn raw_os_error(self) -> i32`

- <span id="errno-from-raw-os-error"></span>`const fn from_raw_os_error(raw: i32) -> Self`

- <span id="errno-from-errno"></span>`const fn from_errno(raw: u32) -> Self`

#### Trait Implementations

##### `impl Clone for Errno`

- <span id="errno-clone"></span>`fn clone(&self) -> Errno` — [`Errno`](#errno)

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

- <span id="errno-eq"></span>`fn eq(&self, other: &Errno) -> bool` — [`Errno`](#errno)

##### `impl StructuralPartialEq for Errno`

##### `impl ToString for Errno`

- <span id="errno-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `try_decode_c_int`

```rust
fn try_decode_c_int<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<c::c_int>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:102-112`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L102-L112)*

Check for an error from the result of a syscall which encodes a
`c::c_int` on success.

### `try_decode_c_uint`

```rust
fn try_decode_c_uint<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<c::c_uint>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:117-127`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L117-L127)*

Check for an error from the result of a syscall which encodes a
`c::c_uint` on success.

### `try_decode_usize`

```rust
fn try_decode_usize<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:132-140`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L132-L140)*

Check for an error from the result of a syscall which encodes a `usize` on
success.

### `try_decode_void_star`

```rust
fn try_decode_void_star<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<*mut c::c_void>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:145-155`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L145-L155)*

Check for an error from the result of a syscall which encodes a
`*mut c_void` on success.

### `try_decode_u64`

```rust
fn try_decode_u64<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<u64>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:161-169`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L161-L169)*

Check for an error from the result of a syscall which encodes a
`u64` on success.

### `try_decode_raw_fd`

```rust
unsafe fn try_decode_raw_fd<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<crate::backend::fd::RawFd>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:179-199`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L179-L199)*

Check for an error from the result of a syscall which encodes a file
descriptor on success.

# Safety

This must only be used with syscalls which return file descriptors on
success.

### `try_decode_void`

```rust
unsafe fn try_decode_void<Num: RetNumber>(raw: crate::backend::reg::RetReg<Num>) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:208-230`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L208-L230)*

Check for an error from the result of a syscall which encodes no value on
success. On success, return the unconsumed `raw` value.

# Safety

This must only be used with syscalls which return no value on success.

