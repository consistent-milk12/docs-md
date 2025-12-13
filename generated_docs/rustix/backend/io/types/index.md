*[rustix](../../../index.md) / [backend](../../index.md) / [io](../index.md) / [types](index.md)*

---

# Module `types`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FdFlags`](#fdflags) | struct | `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`]. |
| [`ReadWriteFlags`](#readwriteflags) | struct | `RWF_*` constants for use with [`preadv2`] and [`pwritev2`]. |
| [`DupFlags`](#dupflags) | struct | `O_*` constants for use with [`dup2`]. |

## Structs

### `FdFlags`

```rust
struct FdFlags(<FdFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:4-18`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L4-L18)*

`FD_*` constants for use with [`fcntl_getfd`](../../../io/index.md) and [`fcntl_setfd`](../../../io/index.md).



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

- <span id="fdflags-bitor"></span>`fn bitor(self, other: FdFlags) -> Self` — [`FdFlags`](#fdflags)

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

- <span id="fdflags-clone"></span>`fn clone(&self) -> FdFlags` — [`FdFlags`](#fdflags)

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

- <span id="fdflags-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md#c-uint)

- <span id="fdflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`c_uint`](../../../ffi/index.md#c-uint), [`FdFlags`](#fdflags)

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

- <span id="fdflags-partialeq-eq"></span>`fn eq(&self, other: &FdFlags) -> bool` — [`FdFlags`](#fdflags)

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

### `ReadWriteFlags`

```rust
struct ReadWriteFlags(<ReadWriteFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:20-42`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L20-L42)*

`RWF_*` constants for use with [`preadv2`](../syscalls/index.md) and [`pwritev2`](../../../io/index.md).



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

- <span id="readwriteflags-bitor"></span>`fn bitor(self, other: ReadWriteFlags) -> Self` — [`ReadWriteFlags`](#readwriteflags)

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

- <span id="readwriteflags-clone"></span>`fn clone(&self) -> ReadWriteFlags` — [`ReadWriteFlags`](#readwriteflags)

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

- <span id="readwriteflags-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md#c-uint)

- <span id="readwriteflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> ReadWriteFlags` — [`c_uint`](../../../ffi/index.md#c-uint), [`ReadWriteFlags`](#readwriteflags)

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

- <span id="readwriteflags-partialeq-eq"></span>`fn eq(&self, other: &ReadWriteFlags) -> bool` — [`ReadWriteFlags`](#readwriteflags)

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

### `DupFlags`

```rust
struct DupFlags(<DupFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:44-57`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L44-L57)*

`O_*` constants for use with [`dup2`](../../../io/index.md).


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

- <span id="dupflags-bitor"></span>`fn bitor(self, other: DupFlags) -> Self` — [`DupFlags`](#dupflags)

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

- <span id="dupflags-clone"></span>`fn clone(&self) -> DupFlags` — [`DupFlags`](#dupflags)

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

- <span id="dupflags-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md#c-uint)

- <span id="dupflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> DupFlags` — [`c_uint`](../../../ffi/index.md#c-uint), [`DupFlags`](#dupflags)

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

- <span id="dupflags-partialeq-eq"></span>`fn eq(&self, other: &DupFlags) -> bool` — [`DupFlags`](#dupflags)

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

