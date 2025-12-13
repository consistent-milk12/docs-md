*[rustix](../../index.md) / [io](../index.md) / [read_write](index.md)*

---

# Module `read_write`

`read` and `write`, optionally positioned, optionally vectored.

## Contents

- [Structs](#structs)
  - [`IoSliceMut`](#ioslicemut)
  - [`ReadWriteFlags`](#readwriteflags)
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
| [`IoSliceMut`](#ioslicemut) | struct |  |
| [`ReadWriteFlags`](#readwriteflags) | struct |  |
| [`read`](#read) | fn | `read(fd, buf)`—Reads from a stream. |
| [`write`](#write) | fn | `write(fd, buf)`—Writes to a stream. |
| [`pread`](#pread) | fn | `pread(fd, buf, offset)`—Reads from a file at a given position. |
| [`pwrite`](#pwrite) | fn | `pwrite(fd, bufs)`—Writes to a file at a given position. |
| [`readv`](#readv) | fn | `readv(fd, bufs)`—Reads from a stream into multiple buffers. |
| [`writev`](#writev) | fn | `writev(fd, bufs)`—Writes to a stream from multiple buffers. |
| [`preadv`](#preadv) | fn | `preadv(fd, bufs, offset)`—Reads from a file at a given position into multiple buffers. |
| [`pwritev`](#pwritev) | fn | `pwritev(fd, bufs, offset)`—Writes to a file at a given position from multiple buffers. |
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

*Defined in [`addr2line-0.25.1/src/frame.rs:43-52`](../../../../.source_1765633015/addr2line-0.25.1/src/frame.rs#L43-L52)*

*Re-exported from `addr2line`*

#### Trait Implementations

##### `impl Any for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frameiterframes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`Buffer`](../../buffer/index.md#buffer), [`buffer`](../../buffer/index.md#buffer)

##### `impl<U> TryInto for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frameiterframes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`Buffer`](../../buffer/index.md#buffer), [`buffer`](../../buffer/index.md#buffer)

### `ReadWriteFlags`

```rust
struct ReadWriteFlags(<ReadWriteFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:20-42`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L20-L42)*

`RWF_*` constants for use with [`preadv2`](../index.md) and [`pwritev2`](../index.md).



#### Implementations

- <span id="readwriteflags-const-dsync"></span>`const DSYNC: Self`

- <span id="readwriteflags-const-hipri"></span>`const HIPRI: Self`

- <span id="readwriteflags-const-sync"></span>`const SYNC: Self`

- <span id="readwriteflags-const-nowait"></span>`const NOWAIT: Self`

- <span id="readwriteflags-const-append"></span>`const APPEND: Self`

#### Trait Implementations

##### `impl Any for ReadWriteFlags`

- <span id="readwriteflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for ReadWriteFlags`

- <span id="readwriteflags-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for ReadWriteFlags`

- <span id="readwriteflags-bitand-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for ReadWriteFlags`

- <span id="readwriteflags-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for ReadWriteFlags`

- <span id="readwriteflags-bitor-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitor"></span>`fn bitor(self, other: ReadWriteFlags) -> Self` — [`ReadWriteFlags`](../../backend/io/types/index.md#readwriteflags)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for ReadWriteFlags`

- <span id="readwriteflags-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for ReadWriteFlags`

- <span id="readwriteflags-bitxor-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for ReadWriteFlags`

- <span id="readwriteflags-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl<T> Borrow for ReadWriteFlags`

- <span id="readwriteflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadWriteFlags`

- <span id="readwriteflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ReadWriteFlags`

- <span id="readwriteflags-clone"></span>`fn clone(&self) -> ReadWriteFlags` — [`ReadWriteFlags`](../../backend/io/types/index.md#readwriteflags)

##### `impl CloneToUninit for ReadWriteFlags`

- <span id="readwriteflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ReadWriteFlags`

##### `impl Debug for ReadWriteFlags`

- <span id="readwriteflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ReadWriteFlags`

##### `impl Extend for ReadWriteFlags`

- <span id="readwriteflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for ReadWriteFlags`

- <span id="readwriteflags-flags-const-flags"></span>`const FLAGS: &'static [Flag<ReadWriteFlags>]`

- <span id="readwriteflags-flags-type-bits"></span>`type Bits = u32`

- <span id="readwriteflags-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md#c-uint)

- <span id="readwriteflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> ReadWriteFlags` — [`c_uint`](../../ffi/index.md#c-uint), [`ReadWriteFlags`](../../backend/io/types/index.md#readwriteflags)

##### `impl<T> From for ReadWriteFlags`

- <span id="readwriteflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for ReadWriteFlags`

- <span id="readwriteflags-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Hash for ReadWriteFlags`

- <span id="readwriteflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ReadWriteFlags`

- <span id="readwriteflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ReadWriteFlags`

- <span id="readwriteflags-intoiterator-type-item"></span>`type Item = ReadWriteFlags`

- <span id="readwriteflags-intoiterator-type-intoiter"></span>`type IntoIter = Iter<ReadWriteFlags>`

- <span id="readwriteflags-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for ReadWriteFlags`

- <span id="readwriteflags-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for ReadWriteFlags`

- <span id="readwriteflags-not-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for ReadWriteFlags`

- <span id="readwriteflags-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for ReadWriteFlags`

- <span id="readwriteflags-partialeq-eq"></span>`fn eq(&self, other: &ReadWriteFlags) -> bool` — [`ReadWriteFlags`](../../backend/io/types/index.md#readwriteflags)

##### `impl PublicFlags for ReadWriteFlags`

- <span id="readwriteflags-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="readwriteflags-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for ReadWriteFlags`

##### `impl Sub for ReadWriteFlags`

- <span id="readwriteflags-sub-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for ReadWriteFlags`

- <span id="readwriteflags-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl ToOwned for ReadWriteFlags`

- <span id="readwriteflags-toowned-type-owned"></span>`type Owned = T`

- <span id="readwriteflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="readwriteflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReadWriteFlags`

- <span id="readwriteflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readwriteflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadWriteFlags`

- <span id="readwriteflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readwriteflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex for ReadWriteFlags`

- <span id="readwriteflags-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

## Functions

### `read`

```rust
fn read<Fd: AsFd, Buf: Buffer<u8>>(fd: Fd, buf: Buf) -> io::Result<<Buf as >::Output>
```

*Defined in [`rustix-1.1.2/src/io/read_write.rs:39-44`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L39-L44)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:69-71`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L69-L71)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:97-106`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L97-L106)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:136-138`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L136-L138)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:164-166`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L164-L166)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:192-194`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L192-L194)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:227-229`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L227-L229)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:266-268`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L266-L268)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:282-289`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L282-L289)*

`preadv2(fd, bufs, offset, flags)`—Reads data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../../libc/new/glibc/index.md)



### `pwritev2`

```rust
fn pwritev2<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>], offset: u64, flags: ReadWriteFlags) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/io/read_write.rs:303-310`](../../../../.source_1765633015/rustix-1.1.2/src/io/read_write.rs#L303-L310)*

`pwritev2(fd, bufs, offset, flags)`—Writes data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../../libc/new/glibc/index.md)



