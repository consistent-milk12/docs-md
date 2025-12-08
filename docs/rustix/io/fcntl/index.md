*[rustix](../../index.md) / [io](../index.md) / [fcntl](index.md)*

---

# Module `fcntl`

The Unix `fcntl` function is effectively lots of different functions hidden
behind a single dynamic dispatch interface. In order to provide a type-safe
API, rustix makes them all separate functions so that they can have
dedicated static type signatures.

`fcntl` functions which are not specific to files or directories live in
the [`io`](../../maybe_polyfill/io/index.md) module instead.


## Structs

### `FdFlags`

```rust
struct FdFlags(<FdFlags as $crate::__private::PublicFlags>::Internal);
```

`FD_*` constants for use with [`fcntl_getfd`](../index.md) and [`fcntl_setfd`](../index.md).



#### Implementations

- `const fn empty() -> Self`

- `const fn all() -> Self`

- `const fn bits(self: &Self) -> ffi::c_uint`

- `const fn from_bits(bits: ffi::c_uint) -> $crate::__private::core::option::Option<Self>`

- `const fn from_bits_truncate(bits: ffi::c_uint) -> Self`

- `const fn from_bits_retain(bits: ffi::c_uint) -> Self`

- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>`

- `const fn is_empty(self: &Self) -> bool`

- `const fn is_all(self: &Self) -> bool`

- `const fn intersects(self: &Self, other: Self) -> bool`

- `const fn contains(self: &Self, other: Self) -> bool`

- `fn insert(self: &mut Self, other: Self)`

- `fn remove(self: &mut Self, other: Self)`

- `fn toggle(self: &mut Self, other: Self)`

- `fn set(self: &mut Self, other: Self, value: bool)`

- `const fn intersection(self: Self, other: Self) -> Self`

- `const fn union(self: Self, other: Self) -> Self`

- `const fn difference(self: Self, other: Self) -> Self`

- `const fn symmetric_difference(self: Self, other: Self) -> Self`

- `const fn complement(self: Self) -> Self`

#### Trait Implementations

##### `impl Binary for FdFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for FdFlags`

- `type Output = FdFlags`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for FdFlags`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for FdFlags`

- `type Output = FdFlags`

- `fn bitor(self: Self, other: FdFlags) -> Self` — [`FdFlags`](../../backend/io/types/index.md)

##### `impl BitOrAssign for FdFlags`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for FdFlags`

- `type Output = FdFlags`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for FdFlags`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for FdFlags`

- `fn clone(self: &Self) -> FdFlags` — [`FdFlags`](../../backend/io/types/index.md)

##### `impl Copy for FdFlags`

##### `impl Debug for FdFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FdFlags`

##### `impl Extend for FdFlags`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for FdFlags`

- `const FLAGS: &'static [$crate::Flag<FdFlags>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint`

- `fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`FdFlags`](../../backend/io/types/index.md)

##### `impl FromIterator for FdFlags`

- `fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for FdFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoIterator for FdFlags`

- `type Item = FdFlags`

- `type IntoIter = Iter<FdFlags>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl LowerHex for FdFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl Not for FdFlags`

- `type Output = FdFlags`

- `fn not(self: Self) -> Self`

##### `impl Octal for FdFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl PartialEq for FdFlags`

- `fn eq(self: &Self, other: &FdFlags) -> bool` — [`FdFlags`](../../backend/io/types/index.md)

##### `impl PublicFlags for FdFlags`

- `type Primitive = u32`

- `type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for FdFlags`

##### `impl Sub for FdFlags`

- `type Output = FdFlags`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for FdFlags`

- `fn sub_assign(self: &mut Self, other: Self)`

##### `impl UpperHex for FdFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

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











