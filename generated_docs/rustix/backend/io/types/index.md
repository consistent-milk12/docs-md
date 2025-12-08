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

`FD_*` constants for use with [`fcntl_getfd`](../../../io/index.md) and [`fcntl_setfd`](../../../io/index.md).



#### Implementations

- <span id="fdflags-empty"></span>`const fn empty() -> Self`

- <span id="fdflags-all"></span>`const fn all() -> Self`

- <span id="fdflags-bits"></span>`const fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- <span id="fdflags-from-bits"></span>`const fn from_bits(bits: ffi::c_uint) -> __private::core::option::Option<Self>` — [`c_uint`](../../../ffi/index.md)

- <span id="fdflags-from-bits-truncate"></span>`const fn from_bits_truncate(bits: ffi::c_uint) -> Self` — [`c_uint`](../../../ffi/index.md)

- <span id="fdflags-from-bits-retain"></span>`const fn from_bits_retain(bits: ffi::c_uint) -> Self` — [`c_uint`](../../../ffi/index.md)

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

- <span id="fdflags-bitor"></span>`fn bitor(self, other: FdFlags) -> Self` — [`FdFlags`](../../../io/fcntl/index.md)

##### `impl BitOrAssign for FdFlags`

- <span id="fdflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for FdFlags`

- <span id="fdflags-output"></span>`type Output = FdFlags`

- <span id="fdflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for FdFlags`

- <span id="fdflags-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for FdFlags`

- <span id="fdflags-clone"></span>`fn clone(&self) -> FdFlags` — [`FdFlags`](../../../io/fcntl/index.md)

##### `impl Copy for FdFlags`

##### `impl Debug for FdFlags`

- <span id="fdflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FdFlags`

##### `impl Extend for FdFlags`

- <span id="fdflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for FdFlags`

- <span id="fdflags-flags"></span>`const FLAGS: &'static [Flag<FdFlags>]`

- <span id="fdflags-bits"></span>`type Bits = u32`

- <span id="fdflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- <span id="fdflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`c_uint`](../../../ffi/index.md), [`FdFlags`](../../../io/fcntl/index.md)

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

- <span id="fdflags-eq"></span>`fn eq(&self, other: &FdFlags) -> bool` — [`FdFlags`](../../../io/fcntl/index.md)

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

### `ReadWriteFlags`

```rust
struct ReadWriteFlags(<ReadWriteFlags as __private::PublicFlags>::Internal);
```

`RWF_*` constants for use with [`preadv2`](../../../io/index.md) and [`pwritev2`](../syscalls/index.md).



#### Implementations

- <span id="readwriteflags-empty"></span>`const fn empty() -> Self`

- <span id="readwriteflags-all"></span>`const fn all() -> Self`

- <span id="readwriteflags-bits"></span>`const fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- <span id="readwriteflags-from-bits"></span>`const fn from_bits(bits: ffi::c_uint) -> __private::core::option::Option<Self>` — [`c_uint`](../../../ffi/index.md)

- <span id="readwriteflags-from-bits-truncate"></span>`const fn from_bits_truncate(bits: ffi::c_uint) -> Self` — [`c_uint`](../../../ffi/index.md)

- <span id="readwriteflags-from-bits-retain"></span>`const fn from_bits_retain(bits: ffi::c_uint) -> Self` — [`c_uint`](../../../ffi/index.md)

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

- <span id="readwriteflags-bitor"></span>`fn bitor(self, other: ReadWriteFlags) -> Self` — [`ReadWriteFlags`](../../../io/read_write/index.md)

##### `impl BitOrAssign for ReadWriteFlags`

- <span id="readwriteflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for ReadWriteFlags`

- <span id="readwriteflags-output"></span>`type Output = ReadWriteFlags`

- <span id="readwriteflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for ReadWriteFlags`

- <span id="readwriteflags-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for ReadWriteFlags`

- <span id="readwriteflags-clone"></span>`fn clone(&self) -> ReadWriteFlags` — [`ReadWriteFlags`](../../../io/read_write/index.md)

##### `impl Copy for ReadWriteFlags`

##### `impl Debug for ReadWriteFlags`

- <span id="readwriteflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ReadWriteFlags`

##### `impl Extend for ReadWriteFlags`

- <span id="readwriteflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for ReadWriteFlags`

- <span id="readwriteflags-flags"></span>`const FLAGS: &'static [Flag<ReadWriteFlags>]`

- <span id="readwriteflags-bits"></span>`type Bits = u32`

- <span id="readwriteflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- <span id="readwriteflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> ReadWriteFlags` — [`c_uint`](../../../ffi/index.md), [`ReadWriteFlags`](../../../io/read_write/index.md)

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

- <span id="readwriteflags-eq"></span>`fn eq(&self, other: &ReadWriteFlags) -> bool` — [`ReadWriteFlags`](../../../io/read_write/index.md)

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

### `DupFlags`

```rust
struct DupFlags(<DupFlags as __private::PublicFlags>::Internal);
```

`O_*` constants for use with [`dup2`](../syscalls/index.md).


#### Implementations

- <span id="dupflags-cloexec"></span>`const CLOEXEC: Self`

#### Trait Implementations

##### `impl Binary for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for DupFlags`

- <span id="dupflags-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for DupFlags`

- <span id="dupflags-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for DupFlags`

- <span id="dupflags-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitor"></span>`fn bitor(self, other: DupFlags) -> Self` — [`DupFlags`](../../../io/dup/index.md)

##### `impl BitOrAssign for DupFlags`

- <span id="dupflags-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for DupFlags`

- <span id="dupflags-output"></span>`type Output = DupFlags`

- <span id="dupflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for DupFlags`

- <span id="dupflags-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for DupFlags`

- <span id="dupflags-clone"></span>`fn clone(&self) -> DupFlags` — [`DupFlags`](../../../io/dup/index.md)

##### `impl Copy for DupFlags`

##### `impl Debug for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DupFlags`

##### `impl Extend for DupFlags`

- <span id="dupflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for DupFlags`

- <span id="dupflags-flags"></span>`const FLAGS: &'static [Flag<DupFlags>]`

- <span id="dupflags-bits"></span>`type Bits = u32`

- <span id="dupflags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- <span id="dupflags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> DupFlags` — [`c_uint`](../../../ffi/index.md), [`DupFlags`](../../../io/dup/index.md)

##### `impl FromIterator for DupFlags`

- <span id="dupflags-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for DupFlags`

- <span id="dupflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for DupFlags`

- <span id="dupflags-item"></span>`type Item = DupFlags`

- <span id="dupflags-intoiter"></span>`type IntoIter = Iter<DupFlags>`

- <span id="dupflags-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for DupFlags`

- <span id="dupflags-output"></span>`type Output = DupFlags`

- <span id="dupflags-not"></span>`fn not(self) -> Self`

##### `impl Octal for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for DupFlags`

- <span id="dupflags-eq"></span>`fn eq(&self, other: &DupFlags) -> bool` — [`DupFlags`](../../../io/dup/index.md)

##### `impl PublicFlags for DupFlags`

- <span id="dupflags-primitive"></span>`type Primitive = u32`

- <span id="dupflags-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for DupFlags`

##### `impl Sub for DupFlags`

- <span id="dupflags-output"></span>`type Output = DupFlags`

- <span id="dupflags-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for DupFlags`

- <span id="dupflags-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for DupFlags`

- <span id="dupflags-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

