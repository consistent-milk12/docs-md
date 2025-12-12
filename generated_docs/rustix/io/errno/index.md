*[rustix](../../index.md) / [io](../index.md) / [errno](index.md)*

---

# Module `errno`

The `Errno` type, which is a minimal wrapper around an error code.

We define the error constants as individual `const`s instead of an enum
because we may not know about all of the host's error values and we don't
want unrecognized values to create undefined behavior.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Errno`](#errno) | struct |  |
| [`retry_on_intr`](#retry-on-intr) | fn | Call `f` until it either succeeds or fails other than [`Errno::INTR`]. |
| [`Result`](#result) | type | A specialized [`Result`] type for `rustix` APIs. |

## Structs

### `Errno`

```rust
struct Errno(u16);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:51`](../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L51)*

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

- <span id="errno-from-io-error"></span>`fn from_io_error(io_err: &std::io::Error) -> Option<Self>`

- <span id="errno-raw-os-error"></span>`const fn raw_os_error(self) -> i32`

- <span id="errno-from-raw-os-error"></span>`const fn from_raw_os_error(raw: i32) -> Self`

- <span id="errno-from-errno"></span>`const fn from_errno(raw: u32) -> Self`

#### Trait Implementations

##### `impl Clone for Errno`

- <span id="errno-clone"></span>`fn clone(&self) -> Errno` — [`Errno`](../../backend/io/errno/index.md#errno)

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

- <span id="errno-eq"></span>`fn eq(&self, other: &Errno) -> bool` — [`Errno`](../../backend/io/errno/index.md#errno)

##### `impl StructuralPartialEq for Errno`

##### `impl ToString for Errno`

- <span id="errno-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `retry_on_intr`

```rust
fn retry_on_intr<T, F: FnMut() -> Result<T>>(f: F) -> Result<T>
```

*Defined in [`rustix-1.1.2/src/io/errno.rs:67-74`](../../../../.source_1765521767/rustix-1.1.2/src/io/errno.rs#L67-L74)*

Call `f` until it either succeeds or fails other than `Errno::INTR`.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Errno>;
```

*Defined in [`rustix-1.1.2/src/io/errno.rs:15`](../../../../.source_1765521767/rustix-1.1.2/src/io/errno.rs#L15)*

A specialized [`Result`](#result) type for `rustix` APIs.

