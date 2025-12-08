*[rustix](../../index.md) / [io](../index.md) / [read_write](index.md)*

---

# Module `read_write`

`read` and `write`, optionally positioned, optionally vectored.

## Contents

- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`read`](#read)
  - [`write`](#write)
  - [`pread`](#pread)
  - [`pwrite`](#pwrite)
  - [`readv`](#readv)
  - [`writev`](#writev)
  - [`preadv`](#preadv)
  - [`pwritev`](#pwritev)
  - [`preadv2`](#preadv2)
  - [`pwritev2`](#pwritev2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`read`](#read) | fn | `read(fd, buf)`—Reads from a stream. |
| [`write`](#write) | fn | `write(fd, buf)`—Writes to a stream. |
| [`pread`](#pread) | fn | `pread(fd, buf, offset)`—Reads from a file at a given position. |
| [`pwrite`](#pwrite) | fn | `pwrite(fd, bufs)`—Writes to a file at a given position. |
| [`readv`](#readv) | fn | `readv(fd, bufs)`—Reads from a stream into multiple buffers. |
| [`writev`](#writev) | fn | `writev(fd, bufs)`—Writes to a stream from multiple buffers. |
| [`preadv`](#preadv) | fn | `preadv(fd, bufs, offset)`—Reads from a file at a given position into |
| [`pwritev`](#pwritev) | fn | `pwritev(fd, bufs, offset)`—Writes to a file at a given position from |
| [`preadv2`](#preadv2) | fn | `preadv2(fd, bufs, offset, flags)`—Reads data, with several options. |
| [`pwritev2`](#pwritev2) | fn | `pwritev2(fd, bufs, offset, flags)`—Writes data, with several options. |

## Structs

### `IoSliceMut<'ctx, R>`

```rust
struct IoSliceMut<'ctx, R>
where
    R: gimli::Reader {
    unit: &'ctx crate::unit::ResUnit<R>,
    sections: &'ctx gimli::Dwarf<R>,
    function: &'ctx crate::function::Function<R>,
    inlined_functions: iter::Rev<alloc::vec::IntoIter<&'ctx crate::function::InlinedFunction<R>>>,
    next: Option<Location<'ctx>>,
}
```

*Re-exported from `addr2line`*

### `ReadWriteFlags`

```rust
struct ReadWriteFlags(<ReadWriteFlags as __private::PublicFlags>::Internal);
```

`RWF_*` constants for use with [`preadv2`](../index.md) and [`pwritev2`](../../backend/io/syscalls/index.md).



#### Implementations

- <span id="readwriteflags-empty"></span>`const fn empty() -> Self`

- <span id="readwriteflags-all"></span>`const fn all() -> Self`

- <span id="readwriteflags-bits"></span>`const fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="readwriteflags-from-bits"></span>`const fn from_bits(bits: ffi::c_uint) -> __private::core::option::Option<Self>` — [`c_uint`](../../ffi/index.md)

- <span id="readwriteflags-from-bits-truncate"></span>`const fn from_bits_truncate(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="readwriteflags-from-bits-retain"></span>`const fn from_bits_retain(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="readwriteflags-from-name"></span>`fn from_name(name: &str) -> __private::core::option::Option<Self>`

- <span id="readwriteflags-is-empty"></span>`const fn is_empty(&self) -> bool`

- <span id="readwriteflags-is-all"></span>`const fn is_all(&self) -> bool`

- <span id="readwriteflags-intersects"></span>`const fn intersects(&self, other: Self) -> bool`

- <span id="readwriteflags-contains"></span>`const fn contains(&self, other: Self) -> bool`

- <span id="readwriteflags-insert"></span>`fn insert(&mut self, other: Self)`

- <span id="readwriteflags-remove"></span>`fn remove(&mut self, other: Self)`

- <span id="readwriteflags-toggle"></span>`fn toggle(&mut self, other: Self)`

- <span id="readwriteflags-set"></span>`fn set(&mut self, other: Self, value: bool)`

- <span id="readwriteflags-intersection"></span>`const fn intersection(self, other: Self) -> Self`

- <span id="readwriteflags-union"></span>`const fn union(self, other: Self) -> Self`

- <span id="readwriteflags-difference"></span>`const fn difference(self, other: Self) -> Self`

- <span id="readwriteflags-symmetric-difference"></span>`const fn symmetric_difference(self, other: Self) -> Self`

- <span id="readwriteflags-complement"></span>`const fn complement(self) -> Self`

#### Trait Implementations

##### `impl Binary for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for ReadWriteFlags`

- <span id="readwriteflags-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for ReadWriteFlags`

- <span id="readwriteflags-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for ReadWriteFlags`

- <span id="readwriteflags-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitor"></span>`fn bitor(self, other: ReadWriteFlags) -> Self` — [`ReadWriteFlags`](#readwriteflags)

##### `impl BitOrAssign for ReadWriteFlags`

- <span id="readwriteflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for ReadWriteFlags`

- <span id="readwriteflags-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for ReadWriteFlags`

- <span id="readwriteflags-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for ReadWriteFlags`

- <span id="readwriteflags-clone"></span>`fn clone(&self) -> ReadWriteFlags` — [`ReadWriteFlags`](#readwriteflags)

##### `impl Copy for ReadWriteFlags`

##### `impl Debug for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ReadWriteFlags`

##### `impl Extend for ReadWriteFlags`

- <span id="readwriteflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for ReadWriteFlags`

- <span id="readwriteflags-flags"></span>`const FLAGS: &'static [Flag<ReadWriteFlags>]`

- <span id="readwriteflags-bits"></span>`type Bits = u32`

- <span id="readwriteflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="readwriteflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> ReadWriteFlags` — [`c_uint`](../../ffi/index.md), [`ReadWriteFlags`](#readwriteflags)

##### `impl FromIterator for ReadWriteFlags`

- <span id="readwriteflags-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for ReadWriteFlags`

- <span id="readwriteflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for ReadWriteFlags`

- <span id="readwriteflags-item"></span>`type Item = ReadWriteFlags`

- <span id="readwriteflags-intoiter"></span>`type IntoIter = Iter<ReadWriteFlags>`

- <span id="readwriteflags-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for ReadWriteFlags`

- <span id="readwriteflags-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-not"></span>`fn not(self) -> Self`

##### `impl Octal for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for ReadWriteFlags`

- <span id="readwriteflags-eq"></span>`fn eq(&self, other: &ReadWriteFlags) -> bool` — [`ReadWriteFlags`](#readwriteflags)

##### `impl PublicFlags for ReadWriteFlags`

- <span id="readwriteflags-primitive"></span>`type Primitive = u32`

- <span id="readwriteflags-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for ReadWriteFlags`

##### `impl Sub for ReadWriteFlags`

- <span id="readwriteflags-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for ReadWriteFlags`

- <span id="readwriteflags-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

## Functions

### `read`

```rust
fn read<Fd: AsFd, Buf: Buffer<u8>>(fd: Fd, buf: Buf) -> io::Result<<Buf as >::Output>
```

`read(fd, buf)`—Reads from a stream.

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










### `write`

```rust
fn write<Fd: AsFd>(fd: Fd, buf: &[u8]) -> io::Result<usize>
```

`write(fd, buf)`—Writes to a stream.

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










### `pread`

```rust
fn pread<Fd: AsFd, Buf: Buffer<u8>>(fd: Fd, buf: Buf, offset: u64) -> io::Result<<Buf as >::Output>
```

`pread(fd, buf, offset)`—Reads from a file at a given position.

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










### `pwrite`

```rust
fn pwrite<Fd: AsFd>(fd: Fd, buf: &[u8], offset: u64) -> io::Result<usize>
```

`pwrite(fd, bufs)`—Writes to a file at a given position.

Contrary to POSIX, on many popular platforms including Linux and FreeBSD,
if the file is opened in append mode, this ignores the offset appends the
data to the end of the file.

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










### `readv`

```rust
fn readv<Fd: AsFd>(fd: Fd, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>
```

`readv(fd, bufs)`—Reads from a stream into multiple buffers.

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










### `writev`

```rust
fn writev<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>]) -> io::Result<usize>
```

`writev(fd, bufs)`—Writes to a stream from multiple buffers.

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










### `preadv`

```rust
fn preadv<Fd: AsFd>(fd: Fd, bufs: &mut [IoSliceMut<'_>], offset: u64) -> io::Result<usize>
```

`preadv(fd, bufs, offset)`—Reads from a file at a given position into
multiple buffers.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)








### `pwritev`

```rust
fn pwritev<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>], offset: u64) -> io::Result<usize>
```

`pwritev(fd, bufs, offset)`—Writes to a file at a given position from
multiple buffers.

Contrary to POSIX, on many popular platforms including Linux and FreeBSD,
if the file is opened in append mode, this ignores the offset appends the
data to the end of the file.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)








### `preadv2`

```rust
fn preadv2<Fd: AsFd>(fd: Fd, bufs: &mut [IoSliceMut<'_>], offset: u64, flags: ReadWriteFlags) -> io::Result<usize>
```

`preadv2(fd, bufs, offset, flags)`—Reads data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../../libc/new/glibc/index.md)



### `pwritev2`

```rust
fn pwritev2<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>], offset: u64, flags: ReadWriteFlags) -> io::Result<usize>
```

`pwritev2(fd, bufs, offset, flags)`—Writes data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../../libc/new/glibc/index.md)



