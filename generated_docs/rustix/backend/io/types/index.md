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

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:4-18`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L4-L18)*

`FD_*` constants for use with [`fcntl_getfd`](../syscalls/index.md) and [`fcntl_setfd`](../../../io/index.md).



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

- <span id="fdflags-bitor"></span>`fn bitor(self, other: FdFlags) -> Self` — [`FdFlags`](#fdflags)

##### `impl BitOrAssign for FdFlags`

- <span id="fdflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for FdFlags`

- <span id="fdflags-type-output"></span>`type Output = FdFlags`

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

- <span id="fdflags-const-flags"></span>`const FLAGS: &'static [Flag<FdFlags>]`

- <span id="fdflags-type-bits"></span>`type Bits = u32`

- <span id="fdflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md#c-uint)

- <span id="fdflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`c_uint`](../../../ffi/index.md#c-uint), [`FdFlags`](#fdflags)

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

- <span id="fdflags-eq"></span>`fn eq(&self, other: &FdFlags) -> bool` — [`FdFlags`](#fdflags)

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

### `ReadWriteFlags`

```rust
struct ReadWriteFlags(<ReadWriteFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:20-42`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L20-L42)*

`RWF_*` constants for use with [`preadv2`](../../../io/index.md) and [`pwritev2`](../syscalls/index.md).



#### Implementations

- <span id="readwriteflags-const-dsync"></span>`const DSYNC: Self`

- <span id="readwriteflags-const-hipri"></span>`const HIPRI: Self`

- <span id="readwriteflags-const-sync"></span>`const SYNC: Self`

- <span id="readwriteflags-const-nowait"></span>`const NOWAIT: Self`

- <span id="readwriteflags-const-append"></span>`const APPEND: Self`

#### Trait Implementations

##### `impl Binary for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for ReadWriteFlags`

- <span id="readwriteflags-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for ReadWriteFlags`

- <span id="readwriteflags-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for ReadWriteFlags`

- <span id="readwriteflags-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitor"></span>`fn bitor(self, other: ReadWriteFlags) -> Self` — [`ReadWriteFlags`](#readwriteflags)

##### `impl BitOrAssign for ReadWriteFlags`

- <span id="readwriteflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for ReadWriteFlags`

- <span id="readwriteflags-type-output"></span>`type Output = ReadWriteFlags`

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

- <span id="readwriteflags-const-flags"></span>`const FLAGS: &'static [Flag<ReadWriteFlags>]`

- <span id="readwriteflags-type-bits"></span>`type Bits = u32`

- <span id="readwriteflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md#c-uint)

- <span id="readwriteflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> ReadWriteFlags` — [`c_uint`](../../../ffi/index.md#c-uint), [`ReadWriteFlags`](#readwriteflags)

##### `impl FromIterator for ReadWriteFlags`

- <span id="readwriteflags-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for ReadWriteFlags`

- <span id="readwriteflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for ReadWriteFlags`

- <span id="readwriteflags-type-item"></span>`type Item = ReadWriteFlags`

- <span id="readwriteflags-type-intoiter"></span>`type IntoIter = Iter<ReadWriteFlags>`

- <span id="readwriteflags-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for ReadWriteFlags`

- <span id="readwriteflags-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-not"></span>`fn not(self) -> Self`

##### `impl Octal for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for ReadWriteFlags`

- <span id="readwriteflags-eq"></span>`fn eq(&self, other: &ReadWriteFlags) -> bool` — [`ReadWriteFlags`](#readwriteflags)

##### `impl PublicFlags for ReadWriteFlags`

- <span id="readwriteflags-type-primitive"></span>`type Primitive = u32`

- <span id="readwriteflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for ReadWriteFlags`

##### `impl Sub for ReadWriteFlags`

- <span id="readwriteflags-type-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for ReadWriteFlags`

- <span id="readwriteflags-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `DupFlags`

```rust
struct DupFlags(<DupFlags as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/types.rs:44-57`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/io/types.rs#L44-L57)*

`O_*` constants for use with [`dup2`](../../../io/index.md).


#### Implementations

- <span id="dupflags-const-cloexec"></span>`const CLOEXEC: Self`

#### Trait Implementations

##### `impl Binary for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for DupFlags`

- <span id="dupflags-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for DupFlags`

- <span id="dupflags-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for DupFlags`

- <span id="dupflags-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitor"></span>`fn bitor(self, other: DupFlags) -> Self` — [`DupFlags`](#dupflags)

##### `impl BitOrAssign for DupFlags`

- <span id="dupflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for DupFlags`

- <span id="dupflags-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for DupFlags`

- <span id="dupflags-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for DupFlags`

- <span id="dupflags-clone"></span>`fn clone(&self) -> DupFlags` — [`DupFlags`](#dupflags)

##### `impl Copy for DupFlags`

##### `impl Debug for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DupFlags`

##### `impl Extend for DupFlags`

- <span id="dupflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for DupFlags`

- <span id="dupflags-const-flags"></span>`const FLAGS: &'static [Flag<DupFlags>]`

- <span id="dupflags-type-bits"></span>`type Bits = u32`

- <span id="dupflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md#c-uint)

- <span id="dupflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> DupFlags` — [`c_uint`](../../../ffi/index.md#c-uint), [`DupFlags`](#dupflags)

##### `impl FromIterator for DupFlags`

- <span id="dupflags-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for DupFlags`

- <span id="dupflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for DupFlags`

- <span id="dupflags-type-item"></span>`type Item = DupFlags`

- <span id="dupflags-type-intoiter"></span>`type IntoIter = Iter<DupFlags>`

- <span id="dupflags-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for DupFlags`

- <span id="dupflags-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-not"></span>`fn not(self) -> Self`

##### `impl Octal for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for DupFlags`

- <span id="dupflags-eq"></span>`fn eq(&self, other: &DupFlags) -> bool` — [`DupFlags`](#dupflags)

##### `impl PublicFlags for DupFlags`

- <span id="dupflags-type-primitive"></span>`type Primitive = u32`

- <span id="dupflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for DupFlags`

##### `impl Sub for DupFlags`

- <span id="dupflags-type-output"></span>`type Output = DupFlags`

- <span id="dupflags-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for DupFlags`

- <span id="dupflags-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

