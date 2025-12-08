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
| [`unnamed`](#unnamed) | struct |  |
| [`fcntl_getfd`](#fcntl_getfd) | fn | `fcntl(fd, F_GETFD)`—Returns a file descriptor's flags. |
| [`fcntl_setfd`](#fcntl_setfd) | fn | `fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags. |
| [`fcntl_dupfd_cloexec`](#fcntl_dupfd_cloexec) | fn | `fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value |

## Structs

### `FdFlags`

```rust
struct FdFlags(<FdFlags as __private::PublicFlags>::Internal);
```

`FD_*` constants for use with [`fcntl_getfd`](../index.md) and [`fcntl_setfd`](../index.md).



#### Implementations

- <span id="fdflags-empty"></span>`const fn empty() -> Self`

- <span id="fdflags-all"></span>`const fn all() -> Self`

- <span id="fdflags-bits"></span>`const fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="fdflags-from-bits"></span>`const fn from_bits(bits: ffi::c_uint) -> __private::core::option::Option<Self>` — [`c_uint`](../../ffi/index.md)

- <span id="fdflags-from-bits-truncate"></span>`const fn from_bits_truncate(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="fdflags-from-bits-retain"></span>`const fn from_bits_retain(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="fdflags-from-name"></span>`fn from_name(name: &str) -> __private::core::option::Option<Self>`

- <span id="fdflags-is-empty"></span>`const fn is_empty(&self) -> bool`

- <span id="fdflags-is-all"></span>`const fn is_all(&self) -> bool`

- <span id="fdflags-intersects"></span>`const fn intersects(&self, other: Self) -> bool`

- <span id="fdflags-contains"></span>`const fn contains(&self, other: Self) -> bool`

- <span id="fdflags-insert"></span>`fn insert(&mut self, other: Self)`

- <span id="fdflags-remove"></span>`fn remove(&mut self, other: Self)`

- <span id="fdflags-toggle"></span>`fn toggle(&mut self, other: Self)`

- <span id="fdflags-set"></span>`fn set(&mut self, other: Self, value: bool)`

- <span id="fdflags-intersection"></span>`const fn intersection(self, other: Self) -> Self`

- <span id="fdflags-union"></span>`const fn union(self, other: Self) -> Self`

- <span id="fdflags-difference"></span>`const fn difference(self, other: Self) -> Self`

- <span id="fdflags-symmetric-difference"></span>`const fn symmetric_difference(self, other: Self) -> Self`

- <span id="fdflags-complement"></span>`const fn complement(self) -> Self`

#### Trait Implementations

##### `impl Binary for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for FdFlags`

- <span id="fdflags-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for FdFlags`

- <span id="fdflags-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for FdFlags`

- <span id="fdflags-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitor"></span>`fn bitor(self, other: FdFlags) -> Self` — [`FdFlags`](#fdflags)

##### `impl BitOrAssign for FdFlags`

- <span id="fdflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for FdFlags`

- <span id="fdflags-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for FdFlags`

- <span id="fdflags-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for FdFlags`

- <span id="fdflags-clone"></span>`fn clone(&self) -> FdFlags` — [`FdFlags`](#fdflags)

##### `impl Copy for FdFlags`

##### `impl Debug for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FdFlags`

##### `impl Extend for FdFlags`

- <span id="fdflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for FdFlags`

- <span id="fdflags-flags"></span>`const FLAGS: &'static [Flag<FdFlags>]`

- <span id="fdflags-bits"></span>`type Bits = u32`

- <span id="fdflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="fdflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`c_uint`](../../ffi/index.md), [`FdFlags`](#fdflags)

##### `impl FromIterator for FdFlags`

- <span id="fdflags-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for FdFlags`

- <span id="fdflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for FdFlags`

- <span id="fdflags-item"></span>`type Item = FdFlags`

- <span id="fdflags-intoiter"></span>`type IntoIter = Iter<FdFlags>`

- <span id="fdflags-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for FdFlags`

- <span id="fdflags-output"></span>`type Output = FdFlags`

- <span id="fdflags-not"></span>`fn not(self) -> Self`

##### `impl Octal for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for FdFlags`

- <span id="fdflags-eq"></span>`fn eq(&self, other: &FdFlags) -> bool` — [`FdFlags`](#fdflags)

##### `impl PublicFlags for FdFlags`

- <span id="fdflags-primitive"></span>`type Primitive = u32`

- <span id="fdflags-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for FdFlags`

##### `impl Sub for FdFlags`

- <span id="fdflags-output"></span>`type Output = FdFlags`

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











