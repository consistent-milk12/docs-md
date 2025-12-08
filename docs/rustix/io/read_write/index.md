*[rustix](../../index.md) / [io](../index.md) / [read_write](index.md)*

---

# Module `read_write`

`read` and `write`, optionally positioned, optionally vectored.

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
struct ReadWriteFlags(<ReadWriteFlags as $crate::__private::PublicFlags>::Internal);
```

`RWF_*` constants for use with [`preadv2`](../index.md) and [`pwritev2`](../../backend/io/syscalls/index.md).



#### Implementations

- `const fn iter(self: &Self) -> $crate::iter::Iter<ReadWriteFlags>` — [`ReadWriteFlags`](../../backend/io/types/index.md)

- `const fn iter_names(self: &Self) -> $crate::iter::IterNames<ReadWriteFlags>` — [`ReadWriteFlags`](../../backend/io/types/index.md)

#### Trait Implementations

##### `impl Binary for ReadWriteFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for ReadWriteFlags`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn bitor(self: Self, other: ReadWriteFlags) -> Self` — [`ReadWriteFlags`](../../backend/io/types/index.md)

##### `impl BitOrAssign for ReadWriteFlags`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for ReadWriteFlags`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for ReadWriteFlags`

- `fn clone(self: &Self) -> ReadWriteFlags` — [`ReadWriteFlags`](../../backend/io/types/index.md)

##### `impl Copy for ReadWriteFlags`

##### `impl Debug for ReadWriteFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ReadWriteFlags`

##### `impl Extend for ReadWriteFlags`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for ReadWriteFlags`

- `const FLAGS: &'static [$crate::Flag<ReadWriteFlags>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint`

- `fn from_bits_retain(bits: ffi::c_uint) -> ReadWriteFlags` — [`ReadWriteFlags`](../../backend/io/types/index.md)

##### `impl FromIterator for ReadWriteFlags`

- `fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for ReadWriteFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoIterator for ReadWriteFlags`

- `type Item = ReadWriteFlags`

- `type IntoIter = Iter<ReadWriteFlags>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl LowerHex for ReadWriteFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl Not for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn not(self: Self) -> Self`

##### `impl Octal for ReadWriteFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl PartialEq for ReadWriteFlags`

- `fn eq(self: &Self, other: &ReadWriteFlags) -> bool` — [`ReadWriteFlags`](../../backend/io/types/index.md)

##### `impl PublicFlags for ReadWriteFlags`

- `type Primitive = u32`

- `type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for ReadWriteFlags`

##### `impl Sub for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for ReadWriteFlags`

- `fn sub_assign(self: &mut Self, other: Self)`

##### `impl UpperHex for ReadWriteFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

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



