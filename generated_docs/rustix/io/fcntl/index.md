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
| [`fcntl_getfd`](#fcntl-getfd) | fn | `fcntl(fd, F_GETFD)`—Returns a file descriptor's flags. |
| [`fcntl_setfd`](#fcntl-setfd) | fn | `fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags. |
| [`fcntl_dupfd_cloexec`](#fcntl-dupfd-cloexec) | fn | `fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value at least `min`, that has `O_CLOEXEC` set and that shares the same underlying [file description] as `fd`. |

## Structs

### `FdFlags`

```rust
struct FdFlags(<FdFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:4-18`](../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L4-L18)*

`FD_*` constants for use with [`fcntl_getfd`](../index.md) and [`fcntl_setfd`](../index.md).



#### Implementations

- <span id="fdflags-const-cloexec"></span>`const CLOEXEC: Self`

#### Trait Implementations

##### `impl Any for FdFlags`

- <span id="fdflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for FdFlags`

- <span id="fdflags-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for FdFlags`

- <span id="fdflags-bitand-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for FdFlags`

- <span id="fdflags-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for FdFlags`

- <span id="fdflags-bitor-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitor"></span>`fn bitor(self, other: FdFlags) -> Self` — [`FdFlags`](../../backend/io/types/index.md#fdflags)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for FdFlags`

- <span id="fdflags-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for FdFlags`

- <span id="fdflags-bitxor-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for FdFlags`

- <span id="fdflags-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl<T> Borrow for FdFlags`

- <span id="fdflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FdFlags`

- <span id="fdflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FdFlags`

- <span id="fdflags-clone"></span>`fn clone(&self) -> FdFlags` — [`FdFlags`](../../backend/io/types/index.md#fdflags)

##### `impl CloneToUninit for FdFlags`

- <span id="fdflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FdFlags`

##### `impl Debug for FdFlags`

- <span id="fdflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FdFlags`

##### `impl Extend for FdFlags`

- <span id="fdflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for FdFlags`

- <span id="fdflags-flags-const-flags"></span>`const FLAGS: &'static [Flag<FdFlags>]`

- <span id="fdflags-flags-type-bits"></span>`type Bits = u32`

- <span id="fdflags-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md#c-uint)

- <span id="fdflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`c_uint`](../../ffi/index.md#c-uint), [`FdFlags`](../../backend/io/types/index.md#fdflags)

##### `impl<T> From for FdFlags`

- <span id="fdflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for FdFlags`

- <span id="fdflags-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Hash for FdFlags`

- <span id="fdflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FdFlags`

- <span id="fdflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for FdFlags`

- <span id="fdflags-intoiterator-type-item"></span>`type Item = FdFlags`

- <span id="fdflags-intoiterator-type-intoiter"></span>`type IntoIter = Iter<FdFlags>`

- <span id="fdflags-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for FdFlags`

- <span id="fdflags-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for FdFlags`

- <span id="fdflags-not-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for FdFlags`

- <span id="fdflags-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for FdFlags`

- <span id="fdflags-partialeq-eq"></span>`fn eq(&self, other: &FdFlags) -> bool` — [`FdFlags`](../../backend/io/types/index.md#fdflags)

##### `impl PublicFlags for FdFlags`

- <span id="fdflags-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="fdflags-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for FdFlags`

##### `impl Sub for FdFlags`

- <span id="fdflags-sub-type-output"></span>`type Output = FdFlags`

- <span id="fdflags-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for FdFlags`

- <span id="fdflags-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl ToOwned for FdFlags`

- <span id="fdflags-toowned-type-owned"></span>`type Owned = T`

- <span id="fdflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fdflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FdFlags`

- <span id="fdflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fdflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FdFlags`

- <span id="fdflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fdflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex for FdFlags`

- <span id="fdflags-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

## Functions

### `fcntl_getfd`

```rust
fn fcntl_getfd<Fd: AsFd>(fd: Fd) -> io::Result<FdFlags>
```

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:40-42`](../../../../.source_1765521767/rustix-1.1.2/src/io/fcntl.rs#L40-L42)*

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

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:68-70`](../../../../.source_1765521767/rustix-1.1.2/src/io/fcntl.rs#L68-L70)*

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

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:105-107`](../../../../.source_1765521767/rustix-1.1.2/src/io/fcntl.rs#L105-L107)*

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











