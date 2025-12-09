*[rustix](../../index.md) / [io](../index.md) / [fcntl](index.md)*

---

# Module `fcntl`

The Unix `fcntl` function is effectively lots of different functions hidden
behind a single dynamic dispatch interface. In order to provide a type-safe
API, rustix makes them all separate functions so that they can have
dedicated static type signatures.

`fcntl` functions which are not specific to files or directories live in
the [`io`](../../maybe_polyfill/io/index.md) module instead.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FdFlags`](#fdflags) | struct |  |
| [`fcntl_getfd`](#fcntl_getfd) | fn | `fcntl(fd, F_GETFD)`—Returns a file descriptor's flags. |
| [`fcntl_setfd`](#fcntl_setfd) | fn | `fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags. |
| [`fcntl_dupfd_cloexec`](#fcntl_dupfd_cloexec) | fn | `fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value at least `min`, that has `O_CLOEXEC` set and that shares the same underlying [file description] as `fd`. |

## Structs

### `FdFlags`

```rust
struct FdFlags(<FdFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:4-18`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L4-L18)*

`FD_*` constants for use with [`fcntl_getfd`](../../backend/io/syscalls/index.md) and [`fcntl_setfd`](../../backend/io/syscalls/index.md).



#### Implementations

- <span id="fdflags-const-cloexec"></span>`const CLOEXEC: Self`

#### Trait Implementations

##### `impl Binary for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for FdFlags`

- <span id="fdflags-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for FdFlags`

- <span id="fdflags-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for FdFlags`

- <span id="fdflags-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitor"></span>`fn bitor(self, other: FdFlags) -> Self` — [`FdFlags`](../../backend/io/types/index.md)

##### `impl BitOrAssign for FdFlags`

- <span id="fdflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for FdFlags`

- <span id="fdflags-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for FdFlags`

- <span id="fdflags-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for FdFlags`

- <span id="fdflags-clone"></span>`fn clone(&self) -> FdFlags` — [`FdFlags`](../../backend/io/types/index.md)

##### `impl Copy for FdFlags`

##### `impl Debug for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FdFlags`

##### `impl Extend for FdFlags`

- <span id="fdflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for FdFlags`

- <span id="fdflags-const-flags"></span>`const FLAGS: &'static [Flag<FdFlags>]`

- <span id="fdflags-type-bits"></span>`type Bits = u32`

- <span id="fdflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="fdflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`c_uint`](../../ffi/index.md), [`FdFlags`](../../backend/io/types/index.md)

##### `impl FromIterator for FdFlags`

- <span id="fdflags-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for FdFlags`

- <span id="fdflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for FdFlags`

- <span id="fdflags-type-item"></span>`type Item = FdFlags`

- <span id="fdflags-type-intoiter"></span>`type IntoIter = Iter<FdFlags>`

- <span id="fdflags-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for FdFlags`

- <span id="fdflags-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-not"></span>`fn not(self) -> Self`

##### `impl Octal for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for FdFlags`

- <span id="fdflags-eq"></span>`fn eq(&self, other: &FdFlags) -> bool` — [`FdFlags`](../../backend/io/types/index.md)

##### `impl PublicFlags for FdFlags`

- <span id="fdflags-type-primitive"></span>`type Primitive = u32`

- <span id="fdflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for FdFlags`

##### `impl Sub for FdFlags`

- <span id="fdflags-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for FdFlags`

- <span id="fdflags-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

## Functions

### `fcntl_getfd`

```rust
fn fcntl_getfd<Fd: AsFd>(fd: Fd) -> io::Result<FdFlags>
```

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:40-42`](../../../../.source_1765210505/rustix-1.1.2/src/io/fcntl.rs#L40-L42)*

`fcntl(fd, F_GETFD)`—Returns a file descriptor's flags.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)










### `fcntl_setfd`

```rust
fn fcntl_setfd<Fd: AsFd>(fd: Fd, flags: FdFlags) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:68-70`](../../../../.source_1765210505/rustix-1.1.2/src/io/fcntl.rs#L68-L70)*

`fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)










### `fcntl_dupfd_cloexec`

```rust
fn fcntl_dupfd_cloexec<Fd: AsFd>(fd: Fd, min: backend::fd::RawFd) -> io::Result<backend::fd::OwnedFd>
```

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:105-107`](../../../../.source_1765210505/rustix-1.1.2/src/io/fcntl.rs#L105-L107)*

`fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value
at least `min`, that has `O_CLOEXEC` set and that shares the same
underlying [file description] as `fd`.

POSIX guarantees that `F_DUPFD_CLOEXEC` will use the lowest unused file
descriptor which is at least `min`, however it is not safe in general to
rely on this, as file descriptors may be unexpectedly allocated on other
threads or in libraries.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)











