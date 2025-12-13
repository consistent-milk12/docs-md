*[rustix](../../index.md) / [io](../index.md) / [dup](index.md)*

---

# Module `dup`

Functions which duplicate file descriptors.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DupFlags`](#dupflags) | struct |  |
| [`dup`](#dup) | fn | `dup(fd)`—Creates a new `OwnedFd` instance that shares the same underlying [file description] as `fd`. |
| [`dup2`](#dup2) | fn | `dup2(fd, new)`—Changes the [file description] of a file descriptor. |
| [`dup3`](#dup3) | fn | `dup3(fd, new, flags)`—Changes the [file description] of a file descriptor, with flags. |

## Structs

### `DupFlags`

```rust
struct DupFlags(<DupFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:44-57`](../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L44-L57)*

`O_*` constants for use with [`dup2`](../index.md).


#### Implementations

- <span id="dupflags-const-cloexec"></span>`const CLOEXEC: Self`

#### Trait Implementations

##### `impl Any for DupFlags`

- <span id="dupflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for DupFlags`

- <span id="dupflags-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for DupFlags`

- <span id="dupflags-bitand-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for DupFlags`

- <span id="dupflags-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for DupFlags`

- <span id="dupflags-bitor-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitor"></span>`fn bitor(self, other: DupFlags) -> Self` — [`DupFlags`](../../backend/io/types/index.md#dupflags)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for DupFlags`

- <span id="dupflags-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for DupFlags`

- <span id="dupflags-bitxor-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for DupFlags`

- <span id="dupflags-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl<T> Borrow for DupFlags`

- <span id="dupflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DupFlags`

- <span id="dupflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DupFlags`

- <span id="dupflags-clone"></span>`fn clone(&self) -> DupFlags` — [`DupFlags`](../../backend/io/types/index.md#dupflags)

##### `impl CloneToUninit for DupFlags`

- <span id="dupflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DupFlags`

##### `impl Debug for DupFlags`

- <span id="dupflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DupFlags`

##### `impl Extend for DupFlags`

- <span id="dupflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for DupFlags`

- <span id="dupflags-flags-const-flags"></span>`const FLAGS: &'static [Flag<DupFlags>]`

- <span id="dupflags-flags-type-bits"></span>`type Bits = u32`

- <span id="dupflags-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md#c-uint)

- <span id="dupflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> DupFlags` — [`c_uint`](../../ffi/index.md#c-uint), [`DupFlags`](../../backend/io/types/index.md#dupflags)

##### `impl<T> From for DupFlags`

- <span id="dupflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for DupFlags`

- <span id="dupflags-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Hash for DupFlags`

- <span id="dupflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for DupFlags`

- <span id="dupflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DupFlags`

- <span id="dupflags-intoiterator-type-item"></span>`type Item = DupFlags`

- <span id="dupflags-intoiterator-type-intoiter"></span>`type IntoIter = Iter<DupFlags>`

- <span id="dupflags-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for DupFlags`

- <span id="dupflags-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for DupFlags`

- <span id="dupflags-not-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for DupFlags`

- <span id="dupflags-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for DupFlags`

- <span id="dupflags-partialeq-eq"></span>`fn eq(&self, other: &DupFlags) -> bool` — [`DupFlags`](../../backend/io/types/index.md#dupflags)

##### `impl PublicFlags for DupFlags`

- <span id="dupflags-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="dupflags-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for DupFlags`

##### `impl Sub for DupFlags`

- <span id="dupflags-sub-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for DupFlags`

- <span id="dupflags-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl ToOwned for DupFlags`

- <span id="dupflags-toowned-type-owned"></span>`type Owned = T`

- <span id="dupflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dupflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DupFlags`

- <span id="dupflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dupflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DupFlags`

- <span id="dupflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dupflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex for DupFlags`

- <span id="dupflags-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

## Functions

### `dup`

```rust
fn dup<Fd: AsFd>(fd: Fd) -> io::Result<crate::fd::OwnedFd>
```

*Defined in [`rustix-1.1.2/src/io/dup.rs:44-46`](../../../../.source_1765521767/rustix-1.1.2/src/io/dup.rs#L44-L46)*

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

*Defined in [`rustix-1.1.2/src/io/dup.rs:89-91`](../../../../.source_1765521767/rustix-1.1.2/src/io/dup.rs#L89-L91)*

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

*Defined in [`rustix-1.1.2/src/io/dup.rs:123-125`](../../../../.source_1765521767/rustix-1.1.2/src/io/dup.rs#L123-L125)*

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







