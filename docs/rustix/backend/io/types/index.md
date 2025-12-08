*[rustix](../../../index.md) / [backend](../../index.md) / [io](../index.md) / [types](index.md)*

---

# Module `types`

## Structs

### `FdFlags`

```rust
struct FdFlags(<FdFlags as $crate::__private::PublicFlags>::Internal);
```

`FD_*` constants for use with [`fcntl_getfd`](../../../io/index.md) and [`fcntl_setfd`](../syscalls/index.md).



#### Implementations

- `const CLOEXEC: Self`

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

- `fn bitor(self: Self, other: FdFlags) -> Self` — [`FdFlags`](../../../io/fcntl/index.md)

##### `impl BitOrAssign for FdFlags`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for FdFlags`

- `type Output = FdFlags`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for FdFlags`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for FdFlags`

- `fn clone(self: &Self) -> FdFlags` — [`FdFlags`](../../../io/fcntl/index.md)

##### `impl Copy for FdFlags`

##### `impl Debug for FdFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FdFlags`

##### `impl Extend for FdFlags`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for FdFlags`

- `const FLAGS: &'static [$crate::Flag<FdFlags>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> FdFlags` — [`c_uint`](../../../ffi/index.md), [`FdFlags`](../../../io/fcntl/index.md)

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

- `fn eq(self: &Self, other: &FdFlags) -> bool` — [`FdFlags`](../../../io/fcntl/index.md)

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

### `ReadWriteFlags`

```rust
struct ReadWriteFlags(<ReadWriteFlags as $crate::__private::PublicFlags>::Internal);
```

`RWF_*` constants for use with [`preadv2`](../syscalls/index.md) and [`pwritev2`](../../../io/index.md).



#### Implementations

- `const fn empty() -> Self`

- `const fn all() -> Self`

- `const fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- `const fn from_bits(bits: ffi::c_uint) -> $crate::__private::core::option::Option<Self>` — [`c_uint`](../../../ffi/index.md)

- `const fn from_bits_truncate(bits: ffi::c_uint) -> Self` — [`c_uint`](../../../ffi/index.md)

- `const fn from_bits_retain(bits: ffi::c_uint) -> Self` — [`c_uint`](../../../ffi/index.md)

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

##### `impl Binary for ReadWriteFlags`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for ReadWriteFlags`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn bitor(self: Self, other: ReadWriteFlags) -> Self` — [`ReadWriteFlags`](../../../io/read_write/index.md)

##### `impl BitOrAssign for ReadWriteFlags`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for ReadWriteFlags`

- `type Output = ReadWriteFlags`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for ReadWriteFlags`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for ReadWriteFlags`

- `fn clone(self: &Self) -> ReadWriteFlags` — [`ReadWriteFlags`](../../../io/read_write/index.md)

##### `impl Copy for ReadWriteFlags`

##### `impl Debug for ReadWriteFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ReadWriteFlags`

##### `impl Extend for ReadWriteFlags`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for ReadWriteFlags`

- `const FLAGS: &'static [$crate::Flag<ReadWriteFlags>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> ReadWriteFlags` — [`c_uint`](../../../ffi/index.md), [`ReadWriteFlags`](../../../io/read_write/index.md)

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

- `fn eq(self: &Self, other: &ReadWriteFlags) -> bool` — [`ReadWriteFlags`](../../../io/read_write/index.md)

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

### `DupFlags`

```rust
struct DupFlags(<DupFlags as $crate::__private::PublicFlags>::Internal);
```

`O_*` constants for use with [`dup2`](../../../io/index.md).


#### Implementations

- `const fn iter(self: &Self) -> $crate::iter::Iter<DupFlags>` — [`DupFlags`](../../../io/dup/index.md)

- `const fn iter_names(self: &Self) -> $crate::iter::IterNames<DupFlags>` — [`DupFlags`](../../../io/dup/index.md)

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

- `fn bitor(self: Self, other: DupFlags) -> Self` — [`DupFlags`](../../../io/dup/index.md)

##### `impl BitOrAssign for DupFlags`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for DupFlags`

- `type Output = DupFlags`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for DupFlags`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for DupFlags`

- `fn clone(self: &Self) -> DupFlags` — [`DupFlags`](../../../io/dup/index.md)

##### `impl Copy for DupFlags`

##### `impl Debug for DupFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for DupFlags`

##### `impl Extend for DupFlags`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for DupFlags`

- `const FLAGS: &'static [$crate::Flag<DupFlags>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../../../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> DupFlags` — [`c_uint`](../../../ffi/index.md), [`DupFlags`](../../../io/dup/index.md)

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

- `fn eq(self: &Self, other: &DupFlags) -> bool` — [`DupFlags`](../../../io/dup/index.md)

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

