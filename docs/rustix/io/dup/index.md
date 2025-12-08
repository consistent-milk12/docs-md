*[rustix](../../index.md) / [io](../index.md) / [dup](index.md)*

---

# Module `dup`

Functions which duplicate file descriptors.

## Structs

### `DupFlags`

```rust
struct DupFlags(<DupFlags as $crate::__private::PublicFlags>::Internal);
```

`O_*` constants for use with [`dup2`](../index.md).


#### Implementations

- `const fn iter(self: &Self) -> $crate::iter::Iter<DupFlags>` — [`DupFlags`](#dupflags)

- `const fn iter_names(self: &Self) -> $crate::iter::IterNames<DupFlags>` — [`DupFlags`](#dupflags)

#### Trait Implementations

##### `impl Binary for DupFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for DupFlags`

- `type Output = DupFlags`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for DupFlags`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for DupFlags`

- `type Output = DupFlags`

- `fn bitor(self: Self, other: DupFlags) -> Self` — [`DupFlags`](#dupflags)

##### `impl BitOrAssign for DupFlags`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for DupFlags`

- `type Output = DupFlags`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for DupFlags`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for DupFlags`

- `fn clone(self: &Self) -> DupFlags` — [`DupFlags`](#dupflags)

##### `impl Copy for DupFlags`

##### `impl Debug for DupFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for DupFlags`

##### `impl Extend for DupFlags`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for DupFlags`

- `const FLAGS: &'static [$crate::Flag<DupFlags>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> DupFlags` — [`c_uint`](../../ffi/index.md), [`DupFlags`](#dupflags)

##### `impl FromIterator for DupFlags`

- `fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for DupFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoIterator for DupFlags`

- `type Item = DupFlags`

- `type IntoIter = Iter<DupFlags>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl LowerHex for DupFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl Not for DupFlags`

- `type Output = DupFlags`

- `fn not(self: Self) -> Self`

##### `impl Octal for DupFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl PartialEq for DupFlags`

- `fn eq(self: &Self, other: &DupFlags) -> bool` — [`DupFlags`](#dupflags)

##### `impl PublicFlags for DupFlags`

- `type Primitive = u32`

- `type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for DupFlags`

##### `impl Sub for DupFlags`

- `type Output = DupFlags`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for DupFlags`

- `fn sub_assign(self: &mut Self, other: Self)`

##### `impl UpperHex for DupFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

## Functions

### `dup`

```rust
fn dup<Fd: AsFd>(fd: Fd) -> io::Result<crate::fd::OwnedFd>
```

`dup(fd)`—Creates a new `OwnedFd` instance that shares the same
underlying [file description] as `fd`.

This function does not set the `O_CLOEXEC` flag. To do a `dup` that does
set `O_CLOEXEC`, use [`fcntl_dupfd_cloexec`](../../backend/io/syscalls/index.md).

POSIX guarantees that `dup` will use the lowest unused file descriptor,
however it is not safe in general to rely on this, as file descriptors may
be unexpectedly allocated on other threads or in libraries.

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












### `dup2`

```rust
fn dup2<Fd: AsFd>(fd: Fd, new: &mut crate::fd::OwnedFd) -> io::Result<()>
```

`dup2(fd, new)`—Changes the [file description] of a file descriptor.

`dup2` conceptually closes `new` and then sets the file description for
`new` to be the same as the one for `fd`. This is a very unusual operation,
and should only be used on file descriptors where you know how `new` will
be subsequently used.

This function does not set the `O_CLOEXEC` flag. To do a `dup2` that does
set `O_CLOEXEC`, use [`dup3`](../index.md) with `DupFlags::CLOEXEC` on platforms which
support it, or [`fcntl_dupfd_cloexec`](../../backend/io/syscalls/index.md).

For `dup2` to stdin, stdout, and stderr, see `stdio::dup2_stdin`,
`stdio::dup2_stdout`, and `stdio::dup2_stderr`.

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















### `dup3`

```rust
fn dup3<Fd: AsFd>(fd: Fd, new: &mut crate::fd::OwnedFd, flags: DupFlags) -> io::Result<()>
```

`dup3(fd, new, flags)`—Changes the [file description] of a file
descriptor, with flags.

`dup3` is the same as [`dup2`](../index.md) but adds an additional flags operand, and it
fails in the case that `fd` and `new` have the same file descriptor value.
This additional difference is the reason this function isn't named
`dup2_with`.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]







