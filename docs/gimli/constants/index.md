*[gimli](../index.md) / [constants](index.md)*

---

# Module `constants`

Constant definitions.

The DWARF spec's `DW_AT_*` type is represented as `struct DwAt(u16)`,
`DW_FORM_*` as `DwForm(u16)`, etc.

There are also exported const definitions for each constant.

## Structs

### `DwSect`

```rust
struct DwSect(u32);
```

The section type field in a `.dwp` unit index.

This is used for version 5 and later.

See Section 7.3.5.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSect`

- `fn clone(self: &Self) -> DwSect` — [`DwSect`](../index.md)

##### `impl Copy for DwSect`

##### `impl Debug for DwSect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwSect`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSect`

##### `impl Hash for DwSect`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwSect`

- `fn cmp(self: &Self, other: &DwSect) -> $crate::cmp::Ordering` — [`DwSect`](../index.md)

##### `impl PartialEq for DwSect`

- `fn eq(self: &Self, other: &DwSect) -> bool` — [`DwSect`](../index.md)

##### `impl PartialOrd for DwSect`

- `fn partial_cmp(self: &Self, other: &DwSect) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwSect`](../index.md)

##### `impl StructuralPartialEq for DwSect`

##### `impl<T> ToString for DwSect`

- `fn to_string(self: &Self) -> String`

### `DwSectV2`

```rust
struct DwSectV2(u32);
```

The section type field in a `.dwp` unit index with version 2.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwSectV2`

- `fn clone(self: &Self) -> DwSectV2` — [`DwSectV2`](../index.md)

##### `impl Copy for DwSectV2`

##### `impl Debug for DwSectV2`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwSectV2`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwSectV2`

##### `impl Hash for DwSectV2`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwSectV2`

- `fn cmp(self: &Self, other: &DwSectV2) -> $crate::cmp::Ordering` — [`DwSectV2`](../index.md)

##### `impl PartialEq for DwSectV2`

- `fn eq(self: &Self, other: &DwSectV2) -> bool` — [`DwSectV2`](../index.md)

##### `impl PartialOrd for DwSectV2`

- `fn partial_cmp(self: &Self, other: &DwSectV2) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwSectV2`](../index.md)

##### `impl StructuralPartialEq for DwSectV2`

##### `impl<T> ToString for DwSectV2`

- `fn to_string(self: &Self) -> String`

### `DwUt`

```rust
struct DwUt(u8);
```

The unit type field in a unit header.

See Section 7.5.1, Table 7.2.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwUt`

- `fn clone(self: &Self) -> DwUt` — [`DwUt`](../index.md)

##### `impl Copy for DwUt`

##### `impl Debug for DwUt`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwUt`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwUt`

##### `impl Hash for DwUt`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwUt`

- `fn cmp(self: &Self, other: &DwUt) -> $crate::cmp::Ordering` — [`DwUt`](../index.md)

##### `impl PartialEq for DwUt`

- `fn eq(self: &Self, other: &DwUt) -> bool` — [`DwUt`](../index.md)

##### `impl PartialOrd for DwUt`

- `fn partial_cmp(self: &Self, other: &DwUt) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwUt`](../index.md)

##### `impl StructuralPartialEq for DwUt`

##### `impl<T> ToString for DwUt`

- `fn to_string(self: &Self) -> String`

### `DwCfa`

```rust
struct DwCfa(u8);
```

The opcode for a call frame instruction.

Section 7.24:
> Call frame instructions are encoded in one or more bytes. The primary
> opcode is encoded in the high order two bits of the first byte (that is,
> opcode = byte >> 6). An operand or extended opcode may be encoded in the
> low order 6 bits. Additional operands are encoded in subsequent bytes.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwCfa`

- `fn clone(self: &Self) -> DwCfa` — [`DwCfa`](../index.md)

##### `impl Copy for DwCfa`

##### `impl Debug for DwCfa`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwCfa`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCfa`

##### `impl Hash for DwCfa`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwCfa`

- `fn cmp(self: &Self, other: &DwCfa) -> $crate::cmp::Ordering` — [`DwCfa`](../index.md)

##### `impl PartialEq for DwCfa`

- `fn eq(self: &Self, other: &DwCfa) -> bool` — [`DwCfa`](../index.md)

##### `impl PartialOrd for DwCfa`

- `fn partial_cmp(self: &Self, other: &DwCfa) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwCfa`](../index.md)

##### `impl StructuralPartialEq for DwCfa`

##### `impl<T> ToString for DwCfa`

- `fn to_string(self: &Self) -> String`

### `DwChildren`

```rust
struct DwChildren(u8);
```

The child determination encodings for DIE attributes.

See Section 7.5.3, Table 7.4.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwChildren`

- `fn clone(self: &Self) -> DwChildren` — [`DwChildren`](../index.md)

##### `impl Copy for DwChildren`

##### `impl Debug for DwChildren`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwChildren`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwChildren`

##### `impl Hash for DwChildren`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwChildren`

- `fn cmp(self: &Self, other: &DwChildren) -> $crate::cmp::Ordering` — [`DwChildren`](../index.md)

##### `impl PartialEq for DwChildren`

- `fn eq(self: &Self, other: &DwChildren) -> bool` — [`DwChildren`](../index.md)

##### `impl PartialOrd for DwChildren`

- `fn partial_cmp(self: &Self, other: &DwChildren) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwChildren`](../index.md)

##### `impl StructuralPartialEq for DwChildren`

##### `impl<T> ToString for DwChildren`

- `fn to_string(self: &Self) -> String`

### `DwTag`

```rust
struct DwTag(u16);
```

The tag encodings for DIE attributes.

See Section 7.5.3, Table 7.3.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwTag`

- `fn clone(self: &Self) -> DwTag` — [`DwTag`](../index.md)

##### `impl Copy for DwTag`

##### `impl Debug for DwTag`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwTag`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwTag`

##### `impl Hash for DwTag`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwTag`

- `fn cmp(self: &Self, other: &DwTag) -> $crate::cmp::Ordering` — [`DwTag`](../index.md)

##### `impl PartialEq for DwTag`

- `fn eq(self: &Self, other: &DwTag) -> bool` — [`DwTag`](../index.md)

##### `impl PartialOrd for DwTag`

- `fn partial_cmp(self: &Self, other: &DwTag) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwTag`](../index.md)

##### `impl StructuralPartialEq for DwTag`

##### `impl<T> ToString for DwTag`

- `fn to_string(self: &Self) -> String`

### `DwAt`

```rust
struct DwAt(u16);
```

The attribute encodings for DIE attributes.

See Section 7.5.4, Table 7.5.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAt`

- `fn clone(self: &Self) -> DwAt` — [`DwAt`](../index.md)

##### `impl Copy for DwAt`

##### `impl Debug for DwAt`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwAt`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAt`

##### `impl Hash for DwAt`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwAt`

- `fn cmp(self: &Self, other: &DwAt) -> $crate::cmp::Ordering` — [`DwAt`](../index.md)

##### `impl PartialEq for DwAt`

- `fn eq(self: &Self, other: &DwAt) -> bool` — [`DwAt`](../index.md)

##### `impl PartialOrd for DwAt`

- `fn partial_cmp(self: &Self, other: &DwAt) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwAt`](../index.md)

##### `impl StructuralPartialEq for DwAt`

##### `impl<T> ToString for DwAt`

- `fn to_string(self: &Self) -> String`

### `DwForm`

```rust
struct DwForm(u16);
```

The attribute form encodings for DIE attributes.

See Section 7.5.6, Table 7.6.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwForm`

- `fn clone(self: &Self) -> DwForm` — [`DwForm`](../index.md)

##### `impl Copy for DwForm`

##### `impl Debug for DwForm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwForm`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwForm`

##### `impl Hash for DwForm`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwForm`

- `fn cmp(self: &Self, other: &DwForm) -> $crate::cmp::Ordering` — [`DwForm`](../index.md)

##### `impl PartialEq for DwForm`

- `fn eq(self: &Self, other: &DwForm) -> bool` — [`DwForm`](../index.md)

##### `impl PartialOrd for DwForm`

- `fn partial_cmp(self: &Self, other: &DwForm) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwForm`](../index.md)

##### `impl StructuralPartialEq for DwForm`

##### `impl<T> ToString for DwForm`

- `fn to_string(self: &Self) -> String`

### `DwAte`

```rust
struct DwAte(u8);
```

The encodings of the constants used in the `DW_AT_encoding` attribute.

See Section 7.8, Table 7.11.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAte`

- `fn clone(self: &Self) -> DwAte` — [`DwAte`](../index.md)

##### `impl Copy for DwAte`

##### `impl Debug for DwAte`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwAte`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAte`

##### `impl Hash for DwAte`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwAte`

- `fn cmp(self: &Self, other: &DwAte) -> $crate::cmp::Ordering` — [`DwAte`](../index.md)

##### `impl PartialEq for DwAte`

- `fn eq(self: &Self, other: &DwAte) -> bool` — [`DwAte`](../index.md)

##### `impl PartialOrd for DwAte`

- `fn partial_cmp(self: &Self, other: &DwAte) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwAte`](../index.md)

##### `impl StructuralPartialEq for DwAte`

##### `impl<T> ToString for DwAte`

- `fn to_string(self: &Self) -> String`

### `DwLle`

```rust
struct DwLle(u8);
```

The encodings of the constants used in location list entries.

See Section 7.7.3, Table 7.10.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLle`

- `fn clone(self: &Self) -> DwLle` — [`DwLle`](../index.md)

##### `impl Copy for DwLle`

##### `impl Debug for DwLle`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwLle`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLle`

##### `impl Hash for DwLle`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwLle`

- `fn cmp(self: &Self, other: &DwLle) -> $crate::cmp::Ordering` — [`DwLle`](../index.md)

##### `impl PartialEq for DwLle`

- `fn eq(self: &Self, other: &DwLle) -> bool` — [`DwLle`](../index.md)

##### `impl PartialOrd for DwLle`

- `fn partial_cmp(self: &Self, other: &DwLle) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwLle`](../index.md)

##### `impl StructuralPartialEq for DwLle`

##### `impl<T> ToString for DwLle`

- `fn to_string(self: &Self) -> String`

### `DwDs`

```rust
struct DwDs(u8);
```

The encodings of the constants used in the `DW_AT_decimal_sign` attribute.

See Section 7.8, Table 7.12.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDs`

- `fn clone(self: &Self) -> DwDs` — [`DwDs`](../index.md)

##### `impl Copy for DwDs`

##### `impl Debug for DwDs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwDs`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDs`

##### `impl Hash for DwDs`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwDs`

- `fn cmp(self: &Self, other: &DwDs) -> $crate::cmp::Ordering` — [`DwDs`](../index.md)

##### `impl PartialEq for DwDs`

- `fn eq(self: &Self, other: &DwDs) -> bool` — [`DwDs`](../index.md)

##### `impl PartialOrd for DwDs`

- `fn partial_cmp(self: &Self, other: &DwDs) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwDs`](../index.md)

##### `impl StructuralPartialEq for DwDs`

##### `impl<T> ToString for DwDs`

- `fn to_string(self: &Self) -> String`

### `DwEnd`

```rust
struct DwEnd(u8);
```

The encodings of the constants used in the `DW_AT_endianity` attribute.

See Section 7.8, Table 7.13.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwEnd`

- `fn clone(self: &Self) -> DwEnd` — [`DwEnd`](../index.md)

##### `impl Copy for DwEnd`

##### `impl Debug for DwEnd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwEnd`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEnd`

##### `impl Hash for DwEnd`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwEnd`

- `fn cmp(self: &Self, other: &DwEnd) -> $crate::cmp::Ordering` — [`DwEnd`](../index.md)

##### `impl PartialEq for DwEnd`

- `fn eq(self: &Self, other: &DwEnd) -> bool` — [`DwEnd`](../index.md)

##### `impl PartialOrd for DwEnd`

- `fn partial_cmp(self: &Self, other: &DwEnd) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwEnd`](../index.md)

##### `impl StructuralPartialEq for DwEnd`

##### `impl<T> ToString for DwEnd`

- `fn to_string(self: &Self) -> String`

### `DwAccess`

```rust
struct DwAccess(u8);
```

The encodings of the constants used in the `DW_AT_accessibility` attribute.

See Section 7.9, Table 7.14.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAccess`

- `fn clone(self: &Self) -> DwAccess` — [`DwAccess`](../index.md)

##### `impl Copy for DwAccess`

##### `impl Debug for DwAccess`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwAccess`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAccess`

##### `impl Hash for DwAccess`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwAccess`

- `fn cmp(self: &Self, other: &DwAccess) -> $crate::cmp::Ordering` — [`DwAccess`](../index.md)

##### `impl PartialEq for DwAccess`

- `fn eq(self: &Self, other: &DwAccess) -> bool` — [`DwAccess`](../index.md)

##### `impl PartialOrd for DwAccess`

- `fn partial_cmp(self: &Self, other: &DwAccess) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwAccess`](../index.md)

##### `impl StructuralPartialEq for DwAccess`

##### `impl<T> ToString for DwAccess`

- `fn to_string(self: &Self) -> String`

### `DwVis`

```rust
struct DwVis(u8);
```

The encodings of the constants used in the `DW_AT_visibility` attribute.

See Section 7.10, Table 7.15.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVis`

- `fn clone(self: &Self) -> DwVis` — [`DwVis`](../index.md)

##### `impl Copy for DwVis`

##### `impl Debug for DwVis`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwVis`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVis`

##### `impl Hash for DwVis`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwVis`

- `fn cmp(self: &Self, other: &DwVis) -> $crate::cmp::Ordering` — [`DwVis`](../index.md)

##### `impl PartialEq for DwVis`

- `fn eq(self: &Self, other: &DwVis) -> bool` — [`DwVis`](../index.md)

##### `impl PartialOrd for DwVis`

- `fn partial_cmp(self: &Self, other: &DwVis) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwVis`](../index.md)

##### `impl StructuralPartialEq for DwVis`

##### `impl<T> ToString for DwVis`

- `fn to_string(self: &Self) -> String`

### `DwVirtuality`

```rust
struct DwVirtuality(u8);
```

The encodings of the constants used in the `DW_AT_virtuality` attribute.

See Section 7.11, Table 7.16.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwVirtuality`

- `fn clone(self: &Self) -> DwVirtuality` — [`DwVirtuality`](../index.md)

##### `impl Copy for DwVirtuality`

##### `impl Debug for DwVirtuality`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwVirtuality`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwVirtuality`

##### `impl Hash for DwVirtuality`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwVirtuality`

- `fn cmp(self: &Self, other: &DwVirtuality) -> $crate::cmp::Ordering` — [`DwVirtuality`](../index.md)

##### `impl PartialEq for DwVirtuality`

- `fn eq(self: &Self, other: &DwVirtuality) -> bool` — [`DwVirtuality`](../index.md)

##### `impl PartialOrd for DwVirtuality`

- `fn partial_cmp(self: &Self, other: &DwVirtuality) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwVirtuality`](../index.md)

##### `impl StructuralPartialEq for DwVirtuality`

##### `impl<T> ToString for DwVirtuality`

- `fn to_string(self: &Self) -> String`

### `DwLang`

```rust
struct DwLang(u16);
```

The encodings of the constants used in the `DW_AT_language` attribute.

See Section 7.12, Table 7.17.

#### Implementations

- `fn default_lower_bound(self: Self) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for DwLang`

- `fn clone(self: &Self) -> DwLang` — [`DwLang`](../index.md)

##### `impl Copy for DwLang`

##### `impl Debug for DwLang`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwLang`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLang`

##### `impl Hash for DwLang`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwLang`

- `fn cmp(self: &Self, other: &DwLang) -> $crate::cmp::Ordering` — [`DwLang`](../index.md)

##### `impl PartialEq for DwLang`

- `fn eq(self: &Self, other: &DwLang) -> bool` — [`DwLang`](../index.md)

##### `impl PartialOrd for DwLang`

- `fn partial_cmp(self: &Self, other: &DwLang) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwLang`](../index.md)

##### `impl StructuralPartialEq for DwLang`

##### `impl<T> ToString for DwLang`

- `fn to_string(self: &Self) -> String`

### `DwAddr`

```rust
struct DwAddr(u64);
```

The encodings of the constants used in the `DW_AT_address_class` attribute.

There is only one value that is common to all target architectures.
See Section 7.13.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwAddr`

- `fn clone(self: &Self) -> DwAddr` — [`DwAddr`](../index.md)

##### `impl Copy for DwAddr`

##### `impl Debug for DwAddr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwAddr`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwAddr`

##### `impl Hash for DwAddr`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwAddr`

- `fn cmp(self: &Self, other: &DwAddr) -> $crate::cmp::Ordering` — [`DwAddr`](../index.md)

##### `impl PartialEq for DwAddr`

- `fn eq(self: &Self, other: &DwAddr) -> bool` — [`DwAddr`](../index.md)

##### `impl PartialOrd for DwAddr`

- `fn partial_cmp(self: &Self, other: &DwAddr) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwAddr`](../index.md)

##### `impl StructuralPartialEq for DwAddr`

##### `impl<T> ToString for DwAddr`

- `fn to_string(self: &Self) -> String`

### `DwId`

```rust
struct DwId(u8);
```

The encodings of the constants used in the `DW_AT_identifier_case` attribute.

See Section 7.14, Table 7.18.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwId`

- `fn clone(self: &Self) -> DwId` — [`DwId`](../index.md)

##### `impl Copy for DwId`

##### `impl Debug for DwId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwId`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwId`

##### `impl Hash for DwId`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwId`

- `fn cmp(self: &Self, other: &DwId) -> $crate::cmp::Ordering` — [`DwId`](../index.md)

##### `impl PartialEq for DwId`

- `fn eq(self: &Self, other: &DwId) -> bool` — [`DwId`](../index.md)

##### `impl PartialOrd for DwId`

- `fn partial_cmp(self: &Self, other: &DwId) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwId`](../index.md)

##### `impl StructuralPartialEq for DwId`

##### `impl<T> ToString for DwId`

- `fn to_string(self: &Self) -> String`

### `DwCc`

```rust
struct DwCc(u8);
```

The encodings of the constants used in the `DW_AT_calling_convention` attribute.

See Section 7.15, Table 7.19.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwCc`

- `fn clone(self: &Self) -> DwCc` — [`DwCc`](../index.md)

##### `impl Copy for DwCc`

##### `impl Debug for DwCc`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwCc`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwCc`

##### `impl Hash for DwCc`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwCc`

- `fn cmp(self: &Self, other: &DwCc) -> $crate::cmp::Ordering` — [`DwCc`](../index.md)

##### `impl PartialEq for DwCc`

- `fn eq(self: &Self, other: &DwCc) -> bool` — [`DwCc`](../index.md)

##### `impl PartialOrd for DwCc`

- `fn partial_cmp(self: &Self, other: &DwCc) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwCc`](../index.md)

##### `impl StructuralPartialEq for DwCc`

##### `impl<T> ToString for DwCc`

- `fn to_string(self: &Self) -> String`

### `DwInl`

```rust
struct DwInl(u8);
```

The encodings of the constants used in the `DW_AT_inline` attribute.

See Section 7.16, Table 7.20.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwInl`

- `fn clone(self: &Self) -> DwInl` — [`DwInl`](../index.md)

##### `impl Copy for DwInl`

##### `impl Debug for DwInl`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwInl`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwInl`

##### `impl Hash for DwInl`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwInl`

- `fn cmp(self: &Self, other: &DwInl) -> $crate::cmp::Ordering` — [`DwInl`](../index.md)

##### `impl PartialEq for DwInl`

- `fn eq(self: &Self, other: &DwInl) -> bool` — [`DwInl`](../index.md)

##### `impl PartialOrd for DwInl`

- `fn partial_cmp(self: &Self, other: &DwInl) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwInl`](../index.md)

##### `impl StructuralPartialEq for DwInl`

##### `impl<T> ToString for DwInl`

- `fn to_string(self: &Self) -> String`

### `DwOrd`

```rust
struct DwOrd(u8);
```

The encodings of the constants used in the `DW_AT_ordering` attribute.

See Section 7.17, Table 7.17.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOrd`

- `fn clone(self: &Self) -> DwOrd` — [`DwOrd`](../index.md)

##### `impl Copy for DwOrd`

##### `impl Debug for DwOrd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwOrd`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOrd`

##### `impl Hash for DwOrd`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwOrd`

- `fn cmp(self: &Self, other: &DwOrd) -> $crate::cmp::Ordering` — [`DwOrd`](../index.md)

##### `impl PartialEq for DwOrd`

- `fn eq(self: &Self, other: &DwOrd) -> bool` — [`DwOrd`](../index.md)

##### `impl PartialOrd for DwOrd`

- `fn partial_cmp(self: &Self, other: &DwOrd) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwOrd`](../index.md)

##### `impl StructuralPartialEq for DwOrd`

##### `impl<T> ToString for DwOrd`

- `fn to_string(self: &Self) -> String`

### `DwDsc`

```rust
struct DwDsc(u8);
```

The encodings of the constants used in the `DW_AT_discr_list` attribute.

See Section 7.18, Table 7.22.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDsc`

- `fn clone(self: &Self) -> DwDsc` — [`DwDsc`](../index.md)

##### `impl Copy for DwDsc`

##### `impl Debug for DwDsc`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwDsc`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDsc`

##### `impl Hash for DwDsc`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwDsc`

- `fn cmp(self: &Self, other: &DwDsc) -> $crate::cmp::Ordering` — [`DwDsc`](../index.md)

##### `impl PartialEq for DwDsc`

- `fn eq(self: &Self, other: &DwDsc) -> bool` — [`DwDsc`](../index.md)

##### `impl PartialOrd for DwDsc`

- `fn partial_cmp(self: &Self, other: &DwDsc) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwDsc`](../index.md)

##### `impl StructuralPartialEq for DwDsc`

##### `impl<T> ToString for DwDsc`

- `fn to_string(self: &Self) -> String`

### `DwIdx`

```rust
struct DwIdx(u16);
```

Name index attribute encodings.

See Section 7.19, Table 7.23.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwIdx`

- `fn clone(self: &Self) -> DwIdx` — [`DwIdx`](../index.md)

##### `impl Copy for DwIdx`

##### `impl Debug for DwIdx`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwIdx`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwIdx`

##### `impl Hash for DwIdx`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwIdx`

- `fn cmp(self: &Self, other: &DwIdx) -> $crate::cmp::Ordering` — [`DwIdx`](../index.md)

##### `impl PartialEq for DwIdx`

- `fn eq(self: &Self, other: &DwIdx) -> bool` — [`DwIdx`](../index.md)

##### `impl PartialOrd for DwIdx`

- `fn partial_cmp(self: &Self, other: &DwIdx) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwIdx`](../index.md)

##### `impl StructuralPartialEq for DwIdx`

##### `impl<T> ToString for DwIdx`

- `fn to_string(self: &Self) -> String`

### `DwDefaulted`

```rust
struct DwDefaulted(u8);
```

The encodings of the constants used in the `DW_AT_defaulted` attribute.

See Section 7.20, Table 7.24.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwDefaulted`

- `fn clone(self: &Self) -> DwDefaulted` — [`DwDefaulted`](../index.md)

##### `impl Copy for DwDefaulted`

##### `impl Debug for DwDefaulted`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwDefaulted`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwDefaulted`

##### `impl Hash for DwDefaulted`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwDefaulted`

- `fn cmp(self: &Self, other: &DwDefaulted) -> $crate::cmp::Ordering` — [`DwDefaulted`](../index.md)

##### `impl PartialEq for DwDefaulted`

- `fn eq(self: &Self, other: &DwDefaulted) -> bool` — [`DwDefaulted`](../index.md)

##### `impl PartialOrd for DwDefaulted`

- `fn partial_cmp(self: &Self, other: &DwDefaulted) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwDefaulted`](../index.md)

##### `impl StructuralPartialEq for DwDefaulted`

##### `impl<T> ToString for DwDefaulted`

- `fn to_string(self: &Self) -> String`

### `DwLns`

```rust
struct DwLns(u8);
```

The encodings for the standard opcodes for line number information.

See Section 7.22, Table 7.25.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLns`

- `fn clone(self: &Self) -> DwLns` — [`DwLns`](../index.md)

##### `impl Copy for DwLns`

##### `impl Debug for DwLns`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwLns`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLns`

##### `impl Hash for DwLns`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwLns`

- `fn cmp(self: &Self, other: &DwLns) -> $crate::cmp::Ordering` — [`DwLns`](../index.md)

##### `impl PartialEq for DwLns`

- `fn eq(self: &Self, other: &DwLns) -> bool` — [`DwLns`](../index.md)

##### `impl PartialOrd for DwLns`

- `fn partial_cmp(self: &Self, other: &DwLns) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwLns`](../index.md)

##### `impl StructuralPartialEq for DwLns`

##### `impl<T> ToString for DwLns`

- `fn to_string(self: &Self) -> String`

### `DwLne`

```rust
struct DwLne(u8);
```

The encodings for the extended opcodes for line number information.

See Section 7.22, Table 7.26.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLne`

- `fn clone(self: &Self) -> DwLne` — [`DwLne`](../index.md)

##### `impl Copy for DwLne`

##### `impl Debug for DwLne`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwLne`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLne`

##### `impl Hash for DwLne`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwLne`

- `fn cmp(self: &Self, other: &DwLne) -> $crate::cmp::Ordering` — [`DwLne`](../index.md)

##### `impl PartialEq for DwLne`

- `fn eq(self: &Self, other: &DwLne) -> bool` — [`DwLne`](../index.md)

##### `impl PartialOrd for DwLne`

- `fn partial_cmp(self: &Self, other: &DwLne) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwLne`](../index.md)

##### `impl StructuralPartialEq for DwLne`

##### `impl<T> ToString for DwLne`

- `fn to_string(self: &Self) -> String`

### `DwLnct`

```rust
struct DwLnct(u16);
```

The encodings for the line number header entry formats.

See Section 7.22, Table 7.27.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwLnct`

- `fn clone(self: &Self) -> DwLnct` — [`DwLnct`](../index.md)

##### `impl Copy for DwLnct`

##### `impl Debug for DwLnct`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwLnct`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwLnct`

##### `impl Hash for DwLnct`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwLnct`

- `fn cmp(self: &Self, other: &DwLnct) -> $crate::cmp::Ordering` — [`DwLnct`](../index.md)

##### `impl PartialEq for DwLnct`

- `fn eq(self: &Self, other: &DwLnct) -> bool` — [`DwLnct`](../index.md)

##### `impl PartialOrd for DwLnct`

- `fn partial_cmp(self: &Self, other: &DwLnct) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwLnct`](../index.md)

##### `impl StructuralPartialEq for DwLnct`

##### `impl<T> ToString for DwLnct`

- `fn to_string(self: &Self) -> String`

### `DwMacinfo`

```rust
struct DwMacinfo(u8);
```

Type codes for macro definitions in the `.debug_macinfo` section.

See Section 7.22, Figure 39 for DWARF 4.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacinfo`

- `fn clone(self: &Self) -> DwMacinfo` — [`DwMacinfo`](../index.md)

##### `impl Copy for DwMacinfo`

##### `impl Debug for DwMacinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwMacinfo`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacinfo`

##### `impl Hash for DwMacinfo`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwMacinfo`

- `fn cmp(self: &Self, other: &DwMacinfo) -> $crate::cmp::Ordering` — [`DwMacinfo`](../index.md)

##### `impl PartialEq for DwMacinfo`

- `fn eq(self: &Self, other: &DwMacinfo) -> bool` — [`DwMacinfo`](../index.md)

##### `impl PartialOrd for DwMacinfo`

- `fn partial_cmp(self: &Self, other: &DwMacinfo) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwMacinfo`](../index.md)

##### `impl StructuralPartialEq for DwMacinfo`

##### `impl<T> ToString for DwMacinfo`

- `fn to_string(self: &Self) -> String`

### `DwMacro`

```rust
struct DwMacro(u8);
```

The encodings for macro information entry types.

See Section 7.23, Table 7.28 for DWARF 5.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwMacro`

- `fn clone(self: &Self) -> DwMacro` — [`DwMacro`](../index.md)

##### `impl Copy for DwMacro`

##### `impl Debug for DwMacro`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwMacro`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwMacro`

##### `impl Hash for DwMacro`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwMacro`

- `fn cmp(self: &Self, other: &DwMacro) -> $crate::cmp::Ordering` — [`DwMacro`](../index.md)

##### `impl PartialEq for DwMacro`

- `fn eq(self: &Self, other: &DwMacro) -> bool` — [`DwMacro`](../index.md)

##### `impl PartialOrd for DwMacro`

- `fn partial_cmp(self: &Self, other: &DwMacro) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwMacro`](../index.md)

##### `impl StructuralPartialEq for DwMacro`

##### `impl<T> ToString for DwMacro`

- `fn to_string(self: &Self) -> String`

### `DwRle`

```rust
struct DwRle(u8);
```

Range list entry encoding values.

See Section 7.25, Table 7.30.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwRle`

- `fn clone(self: &Self) -> DwRle` — [`DwRle`](../index.md)

##### `impl Copy for DwRle`

##### `impl Debug for DwRle`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwRle`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwRle`

##### `impl Hash for DwRle`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwRle`

- `fn cmp(self: &Self, other: &DwRle) -> $crate::cmp::Ordering` — [`DwRle`](../index.md)

##### `impl PartialEq for DwRle`

- `fn eq(self: &Self, other: &DwRle) -> bool` — [`DwRle`](../index.md)

##### `impl PartialOrd for DwRle`

- `fn partial_cmp(self: &Self, other: &DwRle) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwRle`](../index.md)

##### `impl StructuralPartialEq for DwRle`

##### `impl<T> ToString for DwRle`

- `fn to_string(self: &Self) -> String`

### `DwOp`

```rust
struct DwOp(u8);
```

The encodings for DWARF expression operations.

See Section 7.7.1, Table 7.9.

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for DwOp`

- `fn clone(self: &Self) -> DwOp` — [`DwOp`](../index.md)

##### `impl Copy for DwOp`

##### `impl Debug for DwOp`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwOp`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwOp`

##### `impl Hash for DwOp`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwOp`

- `fn cmp(self: &Self, other: &DwOp) -> $crate::cmp::Ordering` — [`DwOp`](../index.md)

##### `impl PartialEq for DwOp`

- `fn eq(self: &Self, other: &DwOp) -> bool` — [`DwOp`](../index.md)

##### `impl PartialOrd for DwOp`

- `fn partial_cmp(self: &Self, other: &DwOp) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwOp`](../index.md)

##### `impl StructuralPartialEq for DwOp`

##### `impl<T> ToString for DwOp`

- `fn to_string(self: &Self) -> String`

### `DwEhPe`

```rust
struct DwEhPe(u8);
```

Pointer encoding used by `.eh_frame`.

The four lower bits describe the
format of the pointer, the upper four bits describe how the encoding should
be applied.

Defined in `<https://refspecs.linuxfoundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html>`

#### Implementations

- `fn static_string(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl BitOr for DwEhPe`

- `type Output = DwEhPe`

- `fn bitor(self: Self, rhs: DwEhPe) -> DwEhPe` — [`DwEhPe`](../index.md)

##### `impl Clone for DwEhPe`

- `fn clone(self: &Self) -> DwEhPe` — [`DwEhPe`](../index.md)

##### `impl Copy for DwEhPe`

##### `impl Debug for DwEhPe`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DwEhPe`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for DwEhPe`

##### `impl Hash for DwEhPe`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for DwEhPe`

- `fn cmp(self: &Self, other: &DwEhPe) -> $crate::cmp::Ordering` — [`DwEhPe`](../index.md)

##### `impl PartialEq for DwEhPe`

- `fn eq(self: &Self, other: &DwEhPe) -> bool` — [`DwEhPe`](../index.md)

##### `impl PartialOrd for DwEhPe`

- `fn partial_cmp(self: &Self, other: &DwEhPe) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DwEhPe`](../index.md)

##### `impl StructuralPartialEq for DwEhPe`

##### `impl<T> ToString for DwEhPe`

- `fn to_string(self: &Self) -> String`

## Constants

### `DW_SECT_INFO`

```rust
const DW_SECT_INFO: DwSect;
```

### `DW_SECT_ABBREV`

```rust
const DW_SECT_ABBREV: DwSect;
```

### `DW_SECT_LINE`

```rust
const DW_SECT_LINE: DwSect;
```

### `DW_SECT_LOCLISTS`

```rust
const DW_SECT_LOCLISTS: DwSect;
```

### `DW_SECT_STR_OFFSETS`

```rust
const DW_SECT_STR_OFFSETS: DwSect;
```

### `DW_SECT_MACRO`

```rust
const DW_SECT_MACRO: DwSect;
```

### `DW_SECT_RNGLISTS`

```rust
const DW_SECT_RNGLISTS: DwSect;
```

### `DW_SECT_V2_INFO`

```rust
const DW_SECT_V2_INFO: DwSectV2;
```

### `DW_SECT_V2_TYPES`

```rust
const DW_SECT_V2_TYPES: DwSectV2;
```

### `DW_SECT_V2_ABBREV`

```rust
const DW_SECT_V2_ABBREV: DwSectV2;
```

### `DW_SECT_V2_LINE`

```rust
const DW_SECT_V2_LINE: DwSectV2;
```

### `DW_SECT_V2_LOC`

```rust
const DW_SECT_V2_LOC: DwSectV2;
```

### `DW_SECT_V2_STR_OFFSETS`

```rust
const DW_SECT_V2_STR_OFFSETS: DwSectV2;
```

### `DW_SECT_V2_MACINFO`

```rust
const DW_SECT_V2_MACINFO: DwSectV2;
```

### `DW_SECT_V2_MACRO`

```rust
const DW_SECT_V2_MACRO: DwSectV2;
```

### `DW_UT_compile`

```rust
const DW_UT_compile: DwUt;
```

### `DW_UT_type`

```rust
const DW_UT_type: DwUt;
```

### `DW_UT_partial`

```rust
const DW_UT_partial: DwUt;
```

### `DW_UT_skeleton`

```rust
const DW_UT_skeleton: DwUt;
```

### `DW_UT_split_compile`

```rust
const DW_UT_split_compile: DwUt;
```

### `DW_UT_split_type`

```rust
const DW_UT_split_type: DwUt;
```

### `DW_UT_lo_user`

```rust
const DW_UT_lo_user: DwUt;
```

### `DW_UT_hi_user`

```rust
const DW_UT_hi_user: DwUt;
```

### `DW_CFA_advance_loc`

```rust
const DW_CFA_advance_loc: DwCfa;
```

### `DW_CFA_offset`

```rust
const DW_CFA_offset: DwCfa;
```

### `DW_CFA_restore`

```rust
const DW_CFA_restore: DwCfa;
```

### `DW_CFA_nop`

```rust
const DW_CFA_nop: DwCfa;
```

### `DW_CFA_set_loc`

```rust
const DW_CFA_set_loc: DwCfa;
```

### `DW_CFA_advance_loc1`

```rust
const DW_CFA_advance_loc1: DwCfa;
```

### `DW_CFA_advance_loc2`

```rust
const DW_CFA_advance_loc2: DwCfa;
```

### `DW_CFA_advance_loc4`

```rust
const DW_CFA_advance_loc4: DwCfa;
```

### `DW_CFA_offset_extended`

```rust
const DW_CFA_offset_extended: DwCfa;
```

### `DW_CFA_restore_extended`

```rust
const DW_CFA_restore_extended: DwCfa;
```

### `DW_CFA_undefined`

```rust
const DW_CFA_undefined: DwCfa;
```

### `DW_CFA_same_value`

```rust
const DW_CFA_same_value: DwCfa;
```

### `DW_CFA_register`

```rust
const DW_CFA_register: DwCfa;
```

### `DW_CFA_remember_state`

```rust
const DW_CFA_remember_state: DwCfa;
```

### `DW_CFA_restore_state`

```rust
const DW_CFA_restore_state: DwCfa;
```

### `DW_CFA_def_cfa`

```rust
const DW_CFA_def_cfa: DwCfa;
```

### `DW_CFA_def_cfa_register`

```rust
const DW_CFA_def_cfa_register: DwCfa;
```

### `DW_CFA_def_cfa_offset`

```rust
const DW_CFA_def_cfa_offset: DwCfa;
```

### `DW_CFA_def_cfa_expression`

```rust
const DW_CFA_def_cfa_expression: DwCfa;
```

### `DW_CFA_expression`

```rust
const DW_CFA_expression: DwCfa;
```

### `DW_CFA_offset_extended_sf`

```rust
const DW_CFA_offset_extended_sf: DwCfa;
```

### `DW_CFA_def_cfa_sf`

```rust
const DW_CFA_def_cfa_sf: DwCfa;
```

### `DW_CFA_def_cfa_offset_sf`

```rust
const DW_CFA_def_cfa_offset_sf: DwCfa;
```

### `DW_CFA_val_offset`

```rust
const DW_CFA_val_offset: DwCfa;
```

### `DW_CFA_val_offset_sf`

```rust
const DW_CFA_val_offset_sf: DwCfa;
```

### `DW_CFA_val_expression`

```rust
const DW_CFA_val_expression: DwCfa;
```

### `DW_CFA_lo_user`

```rust
const DW_CFA_lo_user: DwCfa;
```

### `DW_CFA_hi_user`

```rust
const DW_CFA_hi_user: DwCfa;
```

### `DW_CFA_MIPS_advance_loc8`

```rust
const DW_CFA_MIPS_advance_loc8: DwCfa;
```

### `DW_CFA_GNU_window_save`

```rust
const DW_CFA_GNU_window_save: DwCfa;
```

### `DW_CFA_GNU_args_size`

```rust
const DW_CFA_GNU_args_size: DwCfa;
```

### `DW_CFA_GNU_negative_offset_extended`

```rust
const DW_CFA_GNU_negative_offset_extended: DwCfa;
```

### `DW_CFA_AARCH64_negate_ra_state`

```rust
const DW_CFA_AARCH64_negate_ra_state: DwCfa;
```

### `DW_CHILDREN_no`

```rust
const DW_CHILDREN_no: DwChildren;
```

### `DW_CHILDREN_yes`

```rust
const DW_CHILDREN_yes: DwChildren;
```

### `DW_TAG_null`

```rust
const DW_TAG_null: DwTag;
```

### `DW_TAG_global_subroutine`

```rust
const DW_TAG_global_subroutine: DwTag;
```

### `DW_TAG_global_variable`

```rust
const DW_TAG_global_variable: DwTag;
```

### `DW_TAG_local_variable`

```rust
const DW_TAG_local_variable: DwTag;
```

### `DW_TAG_subroutine`

```rust
const DW_TAG_subroutine: DwTag;
```

### `DW_TAG_array_type`

```rust
const DW_TAG_array_type: DwTag;
```

### `DW_TAG_class_type`

```rust
const DW_TAG_class_type: DwTag;
```

### `DW_TAG_entry_point`

```rust
const DW_TAG_entry_point: DwTag;
```

### `DW_TAG_enumeration_type`

```rust
const DW_TAG_enumeration_type: DwTag;
```

### `DW_TAG_formal_parameter`

```rust
const DW_TAG_formal_parameter: DwTag;
```

### `DW_TAG_imported_declaration`

```rust
const DW_TAG_imported_declaration: DwTag;
```

### `DW_TAG_label`

```rust
const DW_TAG_label: DwTag;
```

### `DW_TAG_lexical_block`

```rust
const DW_TAG_lexical_block: DwTag;
```

### `DW_TAG_member`

```rust
const DW_TAG_member: DwTag;
```

### `DW_TAG_pointer_type`

```rust
const DW_TAG_pointer_type: DwTag;
```

### `DW_TAG_reference_type`

```rust
const DW_TAG_reference_type: DwTag;
```

### `DW_TAG_compile_unit`

```rust
const DW_TAG_compile_unit: DwTag;
```

### `DW_TAG_string_type`

```rust
const DW_TAG_string_type: DwTag;
```

### `DW_TAG_structure_type`

```rust
const DW_TAG_structure_type: DwTag;
```

### `DW_TAG_subroutine_type`

```rust
const DW_TAG_subroutine_type: DwTag;
```

### `DW_TAG_typedef`

```rust
const DW_TAG_typedef: DwTag;
```

### `DW_TAG_union_type`

```rust
const DW_TAG_union_type: DwTag;
```

### `DW_TAG_unspecified_parameters`

```rust
const DW_TAG_unspecified_parameters: DwTag;
```

### `DW_TAG_variant`

```rust
const DW_TAG_variant: DwTag;
```

### `DW_TAG_common_block`

```rust
const DW_TAG_common_block: DwTag;
```

### `DW_TAG_common_inclusion`

```rust
const DW_TAG_common_inclusion: DwTag;
```

### `DW_TAG_inheritance`

```rust
const DW_TAG_inheritance: DwTag;
```

### `DW_TAG_inlined_subroutine`

```rust
const DW_TAG_inlined_subroutine: DwTag;
```

### `DW_TAG_module`

```rust
const DW_TAG_module: DwTag;
```

### `DW_TAG_ptr_to_member_type`

```rust
const DW_TAG_ptr_to_member_type: DwTag;
```

### `DW_TAG_set_type`

```rust
const DW_TAG_set_type: DwTag;
```

### `DW_TAG_subrange_type`

```rust
const DW_TAG_subrange_type: DwTag;
```

### `DW_TAG_with_stmt`

```rust
const DW_TAG_with_stmt: DwTag;
```

### `DW_TAG_access_declaration`

```rust
const DW_TAG_access_declaration: DwTag;
```

### `DW_TAG_base_type`

```rust
const DW_TAG_base_type: DwTag;
```

### `DW_TAG_catch_block`

```rust
const DW_TAG_catch_block: DwTag;
```

### `DW_TAG_const_type`

```rust
const DW_TAG_const_type: DwTag;
```

### `DW_TAG_constant`

```rust
const DW_TAG_constant: DwTag;
```

### `DW_TAG_enumerator`

```rust
const DW_TAG_enumerator: DwTag;
```

### `DW_TAG_file_type`

```rust
const DW_TAG_file_type: DwTag;
```

### `DW_TAG_friend`

```rust
const DW_TAG_friend: DwTag;
```

### `DW_TAG_namelist`

```rust
const DW_TAG_namelist: DwTag;
```

### `DW_TAG_namelist_item`

```rust
const DW_TAG_namelist_item: DwTag;
```

### `DW_TAG_packed_type`

```rust
const DW_TAG_packed_type: DwTag;
```

### `DW_TAG_subprogram`

```rust
const DW_TAG_subprogram: DwTag;
```

### `DW_TAG_template_type_parameter`

```rust
const DW_TAG_template_type_parameter: DwTag;
```

### `DW_TAG_template_value_parameter`

```rust
const DW_TAG_template_value_parameter: DwTag;
```

### `DW_TAG_thrown_type`

```rust
const DW_TAG_thrown_type: DwTag;
```

### `DW_TAG_try_block`

```rust
const DW_TAG_try_block: DwTag;
```

### `DW_TAG_variant_part`

```rust
const DW_TAG_variant_part: DwTag;
```

### `DW_TAG_variable`

```rust
const DW_TAG_variable: DwTag;
```

### `DW_TAG_volatile_type`

```rust
const DW_TAG_volatile_type: DwTag;
```

### `DW_TAG_dwarf_procedure`

```rust
const DW_TAG_dwarf_procedure: DwTag;
```

### `DW_TAG_restrict_type`

```rust
const DW_TAG_restrict_type: DwTag;
```

### `DW_TAG_interface_type`

```rust
const DW_TAG_interface_type: DwTag;
```

### `DW_TAG_namespace`

```rust
const DW_TAG_namespace: DwTag;
```

### `DW_TAG_imported_module`

```rust
const DW_TAG_imported_module: DwTag;
```

### `DW_TAG_unspecified_type`

```rust
const DW_TAG_unspecified_type: DwTag;
```

### `DW_TAG_partial_unit`

```rust
const DW_TAG_partial_unit: DwTag;
```

### `DW_TAG_imported_unit`

```rust
const DW_TAG_imported_unit: DwTag;
```

### `DW_TAG_condition`

```rust
const DW_TAG_condition: DwTag;
```

### `DW_TAG_shared_type`

```rust
const DW_TAG_shared_type: DwTag;
```

### `DW_TAG_type_unit`

```rust
const DW_TAG_type_unit: DwTag;
```

### `DW_TAG_rvalue_reference_type`

```rust
const DW_TAG_rvalue_reference_type: DwTag;
```

### `DW_TAG_template_alias`

```rust
const DW_TAG_template_alias: DwTag;
```

### `DW_TAG_coarray_type`

```rust
const DW_TAG_coarray_type: DwTag;
```

### `DW_TAG_generic_subrange`

```rust
const DW_TAG_generic_subrange: DwTag;
```

### `DW_TAG_dynamic_type`

```rust
const DW_TAG_dynamic_type: DwTag;
```

### `DW_TAG_atomic_type`

```rust
const DW_TAG_atomic_type: DwTag;
```

### `DW_TAG_call_site`

```rust
const DW_TAG_call_site: DwTag;
```

### `DW_TAG_call_site_parameter`

```rust
const DW_TAG_call_site_parameter: DwTag;
```

### `DW_TAG_skeleton_unit`

```rust
const DW_TAG_skeleton_unit: DwTag;
```

### `DW_TAG_immutable_type`

```rust
const DW_TAG_immutable_type: DwTag;
```

### `DW_TAG_lo_user`

```rust
const DW_TAG_lo_user: DwTag;
```

### `DW_TAG_hi_user`

```rust
const DW_TAG_hi_user: DwTag;
```

### `DW_TAG_MIPS_loop`

```rust
const DW_TAG_MIPS_loop: DwTag;
```

### `DW_TAG_HP_array_descriptor`

```rust
const DW_TAG_HP_array_descriptor: DwTag;
```

### `DW_TAG_HP_Bliss_field`

```rust
const DW_TAG_HP_Bliss_field: DwTag;
```

### `DW_TAG_HP_Bliss_field_set`

```rust
const DW_TAG_HP_Bliss_field_set: DwTag;
```

### `DW_TAG_format_label`

```rust
const DW_TAG_format_label: DwTag;
```

### `DW_TAG_function_template`

```rust
const DW_TAG_function_template: DwTag;
```

### `DW_TAG_class_template`

```rust
const DW_TAG_class_template: DwTag;
```

### `DW_TAG_GNU_BINCL`

```rust
const DW_TAG_GNU_BINCL: DwTag;
```

### `DW_TAG_GNU_EINCL`

```rust
const DW_TAG_GNU_EINCL: DwTag;
```

### `DW_TAG_GNU_template_template_param`

```rust
const DW_TAG_GNU_template_template_param: DwTag;
```

### `DW_TAG_GNU_template_parameter_pack`

```rust
const DW_TAG_GNU_template_parameter_pack: DwTag;
```

### `DW_TAG_GNU_formal_parameter_pack`

```rust
const DW_TAG_GNU_formal_parameter_pack: DwTag;
```

### `DW_TAG_GNU_call_site`

```rust
const DW_TAG_GNU_call_site: DwTag;
```

### `DW_TAG_GNU_call_site_parameter`

```rust
const DW_TAG_GNU_call_site_parameter: DwTag;
```

### `DW_TAG_APPLE_property`

```rust
const DW_TAG_APPLE_property: DwTag;
```

### `DW_TAG_SUN_function_template`

```rust
const DW_TAG_SUN_function_template: DwTag;
```

### `DW_TAG_SUN_class_template`

```rust
const DW_TAG_SUN_class_template: DwTag;
```

### `DW_TAG_SUN_struct_template`

```rust
const DW_TAG_SUN_struct_template: DwTag;
```

### `DW_TAG_SUN_union_template`

```rust
const DW_TAG_SUN_union_template: DwTag;
```

### `DW_TAG_SUN_indirect_inheritance`

```rust
const DW_TAG_SUN_indirect_inheritance: DwTag;
```

### `DW_TAG_SUN_codeflags`

```rust
const DW_TAG_SUN_codeflags: DwTag;
```

### `DW_TAG_SUN_memop_info`

```rust
const DW_TAG_SUN_memop_info: DwTag;
```

### `DW_TAG_SUN_omp_child_func`

```rust
const DW_TAG_SUN_omp_child_func: DwTag;
```

### `DW_TAG_SUN_rtti_descriptor`

```rust
const DW_TAG_SUN_rtti_descriptor: DwTag;
```

### `DW_TAG_SUN_dtor_info`

```rust
const DW_TAG_SUN_dtor_info: DwTag;
```

### `DW_TAG_SUN_dtor`

```rust
const DW_TAG_SUN_dtor: DwTag;
```

### `DW_TAG_SUN_f90_interface`

```rust
const DW_TAG_SUN_f90_interface: DwTag;
```

### `DW_TAG_SUN_fortran_vax_structure`

```rust
const DW_TAG_SUN_fortran_vax_structure: DwTag;
```

### `DW_TAG_ALTIUM_circ_type`

```rust
const DW_TAG_ALTIUM_circ_type: DwTag;
```

### `DW_TAG_ALTIUM_mwa_circ_type`

```rust
const DW_TAG_ALTIUM_mwa_circ_type: DwTag;
```

### `DW_TAG_ALTIUM_rev_carry_type`

```rust
const DW_TAG_ALTIUM_rev_carry_type: DwTag;
```

### `DW_TAG_ALTIUM_rom`

```rust
const DW_TAG_ALTIUM_rom: DwTag;
```

### `DW_TAG_upc_shared_type`

```rust
const DW_TAG_upc_shared_type: DwTag;
```

### `DW_TAG_upc_strict_type`

```rust
const DW_TAG_upc_strict_type: DwTag;
```

### `DW_TAG_upc_relaxed_type`

```rust
const DW_TAG_upc_relaxed_type: DwTag;
```

### `DW_TAG_PGI_kanji_type`

```rust
const DW_TAG_PGI_kanji_type: DwTag;
```

### `DW_TAG_PGI_interface_block`

```rust
const DW_TAG_PGI_interface_block: DwTag;
```

### `DW_TAG_BORLAND_property`

```rust
const DW_TAG_BORLAND_property: DwTag;
```

### `DW_TAG_BORLAND_Delphi_string`

```rust
const DW_TAG_BORLAND_Delphi_string: DwTag;
```

### `DW_TAG_BORLAND_Delphi_dynamic_array`

```rust
const DW_TAG_BORLAND_Delphi_dynamic_array: DwTag;
```

### `DW_TAG_BORLAND_Delphi_set`

```rust
const DW_TAG_BORLAND_Delphi_set: DwTag;
```

### `DW_TAG_BORLAND_Delphi_variant`

```rust
const DW_TAG_BORLAND_Delphi_variant: DwTag;
```

### `DW_AT_null`

```rust
const DW_AT_null: DwAt;
```

### `DW_AT_fund_type`

```rust
const DW_AT_fund_type: DwAt;
```

### `DW_AT_mod_fund_type`

```rust
const DW_AT_mod_fund_type: DwAt;
```

### `DW_AT_user_def_type`

```rust
const DW_AT_user_def_type: DwAt;
```

### `DW_AT_mod_u_d_type`

```rust
const DW_AT_mod_u_d_type: DwAt;
```

### `DW_AT_subscr_data`

```rust
const DW_AT_subscr_data: DwAt;
```

### `DW_AT_element_list`

```rust
const DW_AT_element_list: DwAt;
```

### `DW_AT_member`

```rust
const DW_AT_member: DwAt;
```

### `DW_AT_friends`

```rust
const DW_AT_friends: DwAt;
```

### `DW_AT_program`

```rust
const DW_AT_program: DwAt;
```

### `DW_AT_private`

```rust
const DW_AT_private: DwAt;
```

### `DW_AT_protected`

```rust
const DW_AT_protected: DwAt;
```

### `DW_AT_public`

```rust
const DW_AT_public: DwAt;
```

### `DW_AT_pure_virtual`

```rust
const DW_AT_pure_virtual: DwAt;
```

### `DW_AT_virtual`

```rust
const DW_AT_virtual: DwAt;
```

### `DW_AT_specification_v1`

```rust
const DW_AT_specification_v1: DwAt;
```

### `DW_AT_sibling`

```rust
const DW_AT_sibling: DwAt;
```

### `DW_AT_location`

```rust
const DW_AT_location: DwAt;
```

### `DW_AT_name`

```rust
const DW_AT_name: DwAt;
```

### `DW_AT_ordering`

```rust
const DW_AT_ordering: DwAt;
```

### `DW_AT_byte_size`

```rust
const DW_AT_byte_size: DwAt;
```

### `DW_AT_bit_offset`

```rust
const DW_AT_bit_offset: DwAt;
```

### `DW_AT_bit_size`

```rust
const DW_AT_bit_size: DwAt;
```

### `DW_AT_stmt_list`

```rust
const DW_AT_stmt_list: DwAt;
```

### `DW_AT_low_pc`

```rust
const DW_AT_low_pc: DwAt;
```

### `DW_AT_high_pc`

```rust
const DW_AT_high_pc: DwAt;
```

### `DW_AT_language`

```rust
const DW_AT_language: DwAt;
```

### `DW_AT_discr`

```rust
const DW_AT_discr: DwAt;
```

### `DW_AT_discr_value`

```rust
const DW_AT_discr_value: DwAt;
```

### `DW_AT_visibility`

```rust
const DW_AT_visibility: DwAt;
```

### `DW_AT_import`

```rust
const DW_AT_import: DwAt;
```

### `DW_AT_string_length`

```rust
const DW_AT_string_length: DwAt;
```

### `DW_AT_common_reference`

```rust
const DW_AT_common_reference: DwAt;
```

### `DW_AT_comp_dir`

```rust
const DW_AT_comp_dir: DwAt;
```

### `DW_AT_const_value`

```rust
const DW_AT_const_value: DwAt;
```

### `DW_AT_containing_type`

```rust
const DW_AT_containing_type: DwAt;
```

### `DW_AT_default_value`

```rust
const DW_AT_default_value: DwAt;
```

### `DW_AT_inline`

```rust
const DW_AT_inline: DwAt;
```

### `DW_AT_is_optional`

```rust
const DW_AT_is_optional: DwAt;
```

### `DW_AT_lower_bound`

```rust
const DW_AT_lower_bound: DwAt;
```

### `DW_AT_producer`

```rust
const DW_AT_producer: DwAt;
```

### `DW_AT_prototyped`

```rust
const DW_AT_prototyped: DwAt;
```

### `DW_AT_return_addr`

```rust
const DW_AT_return_addr: DwAt;
```

### `DW_AT_start_scope`

```rust
const DW_AT_start_scope: DwAt;
```

### `DW_AT_bit_stride`

```rust
const DW_AT_bit_stride: DwAt;
```

### `DW_AT_upper_bound`

```rust
const DW_AT_upper_bound: DwAt;
```

### `DW_AT_abstract_origin`

```rust
const DW_AT_abstract_origin: DwAt;
```

### `DW_AT_accessibility`

```rust
const DW_AT_accessibility: DwAt;
```

### `DW_AT_address_class`

```rust
const DW_AT_address_class: DwAt;
```

### `DW_AT_artificial`

```rust
const DW_AT_artificial: DwAt;
```

### `DW_AT_base_types`

```rust
const DW_AT_base_types: DwAt;
```

### `DW_AT_calling_convention`

```rust
const DW_AT_calling_convention: DwAt;
```

### `DW_AT_count`

```rust
const DW_AT_count: DwAt;
```

### `DW_AT_data_member_location`

```rust
const DW_AT_data_member_location: DwAt;
```

### `DW_AT_decl_column`

```rust
const DW_AT_decl_column: DwAt;
```

### `DW_AT_decl_file`

```rust
const DW_AT_decl_file: DwAt;
```

### `DW_AT_decl_line`

```rust
const DW_AT_decl_line: DwAt;
```

### `DW_AT_declaration`

```rust
const DW_AT_declaration: DwAt;
```

### `DW_AT_discr_list`

```rust
const DW_AT_discr_list: DwAt;
```

### `DW_AT_encoding`

```rust
const DW_AT_encoding: DwAt;
```

### `DW_AT_external`

```rust
const DW_AT_external: DwAt;
```

### `DW_AT_frame_base`

```rust
const DW_AT_frame_base: DwAt;
```

### `DW_AT_friend`

```rust
const DW_AT_friend: DwAt;
```

### `DW_AT_identifier_case`

```rust
const DW_AT_identifier_case: DwAt;
```

### `DW_AT_macro_info`

```rust
const DW_AT_macro_info: DwAt;
```

### `DW_AT_namelist_item`

```rust
const DW_AT_namelist_item: DwAt;
```

### `DW_AT_priority`

```rust
const DW_AT_priority: DwAt;
```

### `DW_AT_segment`

```rust
const DW_AT_segment: DwAt;
```

### `DW_AT_specification`

```rust
const DW_AT_specification: DwAt;
```

### `DW_AT_static_link`

```rust
const DW_AT_static_link: DwAt;
```

### `DW_AT_type`

```rust
const DW_AT_type: DwAt;
```

### `DW_AT_use_location`

```rust
const DW_AT_use_location: DwAt;
```

### `DW_AT_variable_parameter`

```rust
const DW_AT_variable_parameter: DwAt;
```

### `DW_AT_virtuality`

```rust
const DW_AT_virtuality: DwAt;
```

### `DW_AT_vtable_elem_location`

```rust
const DW_AT_vtable_elem_location: DwAt;
```

### `DW_AT_allocated`

```rust
const DW_AT_allocated: DwAt;
```

### `DW_AT_associated`

```rust
const DW_AT_associated: DwAt;
```

### `DW_AT_data_location`

```rust
const DW_AT_data_location: DwAt;
```

### `DW_AT_byte_stride`

```rust
const DW_AT_byte_stride: DwAt;
```

### `DW_AT_entry_pc`

```rust
const DW_AT_entry_pc: DwAt;
```

### `DW_AT_use_UTF8`

```rust
const DW_AT_use_UTF8: DwAt;
```

### `DW_AT_extension`

```rust
const DW_AT_extension: DwAt;
```

### `DW_AT_ranges`

```rust
const DW_AT_ranges: DwAt;
```

### `DW_AT_trampoline`

```rust
const DW_AT_trampoline: DwAt;
```

### `DW_AT_call_column`

```rust
const DW_AT_call_column: DwAt;
```

### `DW_AT_call_file`

```rust
const DW_AT_call_file: DwAt;
```

### `DW_AT_call_line`

```rust
const DW_AT_call_line: DwAt;
```

### `DW_AT_description`

```rust
const DW_AT_description: DwAt;
```

### `DW_AT_binary_scale`

```rust
const DW_AT_binary_scale: DwAt;
```

### `DW_AT_decimal_scale`

```rust
const DW_AT_decimal_scale: DwAt;
```

### `DW_AT_small`

```rust
const DW_AT_small: DwAt;
```

### `DW_AT_decimal_sign`

```rust
const DW_AT_decimal_sign: DwAt;
```

### `DW_AT_digit_count`

```rust
const DW_AT_digit_count: DwAt;
```

### `DW_AT_picture_string`

```rust
const DW_AT_picture_string: DwAt;
```

### `DW_AT_mutable`

```rust
const DW_AT_mutable: DwAt;
```

### `DW_AT_threads_scaled`

```rust
const DW_AT_threads_scaled: DwAt;
```

### `DW_AT_explicit`

```rust
const DW_AT_explicit: DwAt;
```

### `DW_AT_object_pointer`

```rust
const DW_AT_object_pointer: DwAt;
```

### `DW_AT_endianity`

```rust
const DW_AT_endianity: DwAt;
```

### `DW_AT_elemental`

```rust
const DW_AT_elemental: DwAt;
```

### `DW_AT_pure`

```rust
const DW_AT_pure: DwAt;
```

### `DW_AT_recursive`

```rust
const DW_AT_recursive: DwAt;
```

### `DW_AT_signature`

```rust
const DW_AT_signature: DwAt;
```

### `DW_AT_main_subprogram`

```rust
const DW_AT_main_subprogram: DwAt;
```

### `DW_AT_data_bit_offset`

```rust
const DW_AT_data_bit_offset: DwAt;
```

### `DW_AT_const_expr`

```rust
const DW_AT_const_expr: DwAt;
```

### `DW_AT_enum_class`

```rust
const DW_AT_enum_class: DwAt;
```

### `DW_AT_linkage_name`

```rust
const DW_AT_linkage_name: DwAt;
```

### `DW_AT_string_length_bit_size`

```rust
const DW_AT_string_length_bit_size: DwAt;
```

### `DW_AT_string_length_byte_size`

```rust
const DW_AT_string_length_byte_size: DwAt;
```

### `DW_AT_rank`

```rust
const DW_AT_rank: DwAt;
```

### `DW_AT_str_offsets_base`

```rust
const DW_AT_str_offsets_base: DwAt;
```

### `DW_AT_addr_base`

```rust
const DW_AT_addr_base: DwAt;
```

### `DW_AT_rnglists_base`

```rust
const DW_AT_rnglists_base: DwAt;
```

### `DW_AT_dwo_name`

```rust
const DW_AT_dwo_name: DwAt;
```

### `DW_AT_reference`

```rust
const DW_AT_reference: DwAt;
```

### `DW_AT_rvalue_reference`

```rust
const DW_AT_rvalue_reference: DwAt;
```

### `DW_AT_macros`

```rust
const DW_AT_macros: DwAt;
```

### `DW_AT_call_all_calls`

```rust
const DW_AT_call_all_calls: DwAt;
```

### `DW_AT_call_all_source_calls`

```rust
const DW_AT_call_all_source_calls: DwAt;
```

### `DW_AT_call_all_tail_calls`

```rust
const DW_AT_call_all_tail_calls: DwAt;
```

### `DW_AT_call_return_pc`

```rust
const DW_AT_call_return_pc: DwAt;
```

### `DW_AT_call_value`

```rust
const DW_AT_call_value: DwAt;
```

### `DW_AT_call_origin`

```rust
const DW_AT_call_origin: DwAt;
```

### `DW_AT_call_parameter`

```rust
const DW_AT_call_parameter: DwAt;
```

### `DW_AT_call_pc`

```rust
const DW_AT_call_pc: DwAt;
```

### `DW_AT_call_tail_call`

```rust
const DW_AT_call_tail_call: DwAt;
```

### `DW_AT_call_target`

```rust
const DW_AT_call_target: DwAt;
```

### `DW_AT_call_target_clobbered`

```rust
const DW_AT_call_target_clobbered: DwAt;
```

### `DW_AT_call_data_location`

```rust
const DW_AT_call_data_location: DwAt;
```

### `DW_AT_call_data_value`

```rust
const DW_AT_call_data_value: DwAt;
```

### `DW_AT_noreturn`

```rust
const DW_AT_noreturn: DwAt;
```

### `DW_AT_alignment`

```rust
const DW_AT_alignment: DwAt;
```

### `DW_AT_export_symbols`

```rust
const DW_AT_export_symbols: DwAt;
```

### `DW_AT_deleted`

```rust
const DW_AT_deleted: DwAt;
```

### `DW_AT_defaulted`

```rust
const DW_AT_defaulted: DwAt;
```

### `DW_AT_loclists_base`

```rust
const DW_AT_loclists_base: DwAt;
```

### `DW_AT_lo_user`

```rust
const DW_AT_lo_user: DwAt;
```

### `DW_AT_hi_user`

```rust
const DW_AT_hi_user: DwAt;
```

### `DW_AT_MIPS_fde`

```rust
const DW_AT_MIPS_fde: DwAt;
```

### `DW_AT_MIPS_loop_begin`

```rust
const DW_AT_MIPS_loop_begin: DwAt;
```

### `DW_AT_MIPS_tail_loop_begin`

```rust
const DW_AT_MIPS_tail_loop_begin: DwAt;
```

### `DW_AT_MIPS_epilog_begin`

```rust
const DW_AT_MIPS_epilog_begin: DwAt;
```

### `DW_AT_MIPS_loop_unroll_factor`

```rust
const DW_AT_MIPS_loop_unroll_factor: DwAt;
```

### `DW_AT_MIPS_software_pipeline_depth`

```rust
const DW_AT_MIPS_software_pipeline_depth: DwAt;
```

### `DW_AT_MIPS_linkage_name`

```rust
const DW_AT_MIPS_linkage_name: DwAt;
```

### `DW_AT_MIPS_stride`

```rust
const DW_AT_MIPS_stride: DwAt;
```

### `DW_AT_MIPS_abstract_name`

```rust
const DW_AT_MIPS_abstract_name: DwAt;
```

### `DW_AT_MIPS_clone_origin`

```rust
const DW_AT_MIPS_clone_origin: DwAt;
```

### `DW_AT_MIPS_has_inlines`

```rust
const DW_AT_MIPS_has_inlines: DwAt;
```

### `DW_AT_MIPS_stride_byte`

```rust
const DW_AT_MIPS_stride_byte: DwAt;
```

### `DW_AT_MIPS_stride_elem`

```rust
const DW_AT_MIPS_stride_elem: DwAt;
```

### `DW_AT_MIPS_ptr_dopetype`

```rust
const DW_AT_MIPS_ptr_dopetype: DwAt;
```

### `DW_AT_MIPS_allocatable_dopetype`

```rust
const DW_AT_MIPS_allocatable_dopetype: DwAt;
```

### `DW_AT_MIPS_assumed_shape_dopetype`

```rust
const DW_AT_MIPS_assumed_shape_dopetype: DwAt;
```

### `DW_AT_MIPS_assumed_size`

```rust
const DW_AT_MIPS_assumed_size: DwAt;
```

### `DW_AT_INTEL_other_endian`

```rust
const DW_AT_INTEL_other_endian: DwAt;
```

### `DW_AT_sf_names`

```rust
const DW_AT_sf_names: DwAt;
```

### `DW_AT_src_info`

```rust
const DW_AT_src_info: DwAt;
```

### `DW_AT_mac_info`

```rust
const DW_AT_mac_info: DwAt;
```

### `DW_AT_src_coords`

```rust
const DW_AT_src_coords: DwAt;
```

### `DW_AT_body_begin`

```rust
const DW_AT_body_begin: DwAt;
```

### `DW_AT_body_end`

```rust
const DW_AT_body_end: DwAt;
```

### `DW_AT_GNU_vector`

```rust
const DW_AT_GNU_vector: DwAt;
```

### `DW_AT_GNU_guarded_by`

```rust
const DW_AT_GNU_guarded_by: DwAt;
```

### `DW_AT_GNU_pt_guarded_by`

```rust
const DW_AT_GNU_pt_guarded_by: DwAt;
```

### `DW_AT_GNU_guarded`

```rust
const DW_AT_GNU_guarded: DwAt;
```

### `DW_AT_GNU_pt_guarded`

```rust
const DW_AT_GNU_pt_guarded: DwAt;
```

### `DW_AT_GNU_locks_excluded`

```rust
const DW_AT_GNU_locks_excluded: DwAt;
```

### `DW_AT_GNU_exclusive_locks_required`

```rust
const DW_AT_GNU_exclusive_locks_required: DwAt;
```

### `DW_AT_GNU_shared_locks_required`

```rust
const DW_AT_GNU_shared_locks_required: DwAt;
```

### `DW_AT_GNU_odr_signature`

```rust
const DW_AT_GNU_odr_signature: DwAt;
```

### `DW_AT_GNU_template_name`

```rust
const DW_AT_GNU_template_name: DwAt;
```

### `DW_AT_GNU_call_site_value`

```rust
const DW_AT_GNU_call_site_value: DwAt;
```

### `DW_AT_GNU_call_site_data_value`

```rust
const DW_AT_GNU_call_site_data_value: DwAt;
```

### `DW_AT_GNU_call_site_target`

```rust
const DW_AT_GNU_call_site_target: DwAt;
```

### `DW_AT_GNU_call_site_target_clobbered`

```rust
const DW_AT_GNU_call_site_target_clobbered: DwAt;
```

### `DW_AT_GNU_tail_call`

```rust
const DW_AT_GNU_tail_call: DwAt;
```

### `DW_AT_GNU_all_tail_call_sites`

```rust
const DW_AT_GNU_all_tail_call_sites: DwAt;
```

### `DW_AT_GNU_all_call_sites`

```rust
const DW_AT_GNU_all_call_sites: DwAt;
```

### `DW_AT_GNU_all_source_call_sites`

```rust
const DW_AT_GNU_all_source_call_sites: DwAt;
```

### `DW_AT_GNU_macros`

```rust
const DW_AT_GNU_macros: DwAt;
```

### `DW_AT_GNU_deleted`

```rust
const DW_AT_GNU_deleted: DwAt;
```

### `DW_AT_GNU_dwo_name`

```rust
const DW_AT_GNU_dwo_name: DwAt;
```

### `DW_AT_GNU_dwo_id`

```rust
const DW_AT_GNU_dwo_id: DwAt;
```

### `DW_AT_GNU_ranges_base`

```rust
const DW_AT_GNU_ranges_base: DwAt;
```

### `DW_AT_GNU_addr_base`

```rust
const DW_AT_GNU_addr_base: DwAt;
```

### `DW_AT_GNU_pubnames`

```rust
const DW_AT_GNU_pubnames: DwAt;
```

### `DW_AT_GNU_pubtypes`

```rust
const DW_AT_GNU_pubtypes: DwAt;
```

### `DW_AT_GNU_discriminator`

```rust
const DW_AT_GNU_discriminator: DwAt;
```

### `DW_AT_GNU_locviews`

```rust
const DW_AT_GNU_locviews: DwAt;
```

### `DW_AT_GNU_entry_view`

```rust
const DW_AT_GNU_entry_view: DwAt;
```

### `DW_AT_SUN_template`

```rust
const DW_AT_SUN_template: DwAt;
```

### `DW_AT_SUN_alignment`

```rust
const DW_AT_SUN_alignment: DwAt;
```

### `DW_AT_SUN_vtable`

```rust
const DW_AT_SUN_vtable: DwAt;
```

### `DW_AT_SUN_count_guarantee`

```rust
const DW_AT_SUN_count_guarantee: DwAt;
```

### `DW_AT_SUN_command_line`

```rust
const DW_AT_SUN_command_line: DwAt;
```

### `DW_AT_SUN_vbase`

```rust
const DW_AT_SUN_vbase: DwAt;
```

### `DW_AT_SUN_compile_options`

```rust
const DW_AT_SUN_compile_options: DwAt;
```

### `DW_AT_SUN_language`

```rust
const DW_AT_SUN_language: DwAt;
```

### `DW_AT_SUN_browser_file`

```rust
const DW_AT_SUN_browser_file: DwAt;
```

### `DW_AT_SUN_vtable_abi`

```rust
const DW_AT_SUN_vtable_abi: DwAt;
```

### `DW_AT_SUN_func_offsets`

```rust
const DW_AT_SUN_func_offsets: DwAt;
```

### `DW_AT_SUN_cf_kind`

```rust
const DW_AT_SUN_cf_kind: DwAt;
```

### `DW_AT_SUN_vtable_index`

```rust
const DW_AT_SUN_vtable_index: DwAt;
```

### `DW_AT_SUN_omp_tpriv_addr`

```rust
const DW_AT_SUN_omp_tpriv_addr: DwAt;
```

### `DW_AT_SUN_omp_child_func`

```rust
const DW_AT_SUN_omp_child_func: DwAt;
```

### `DW_AT_SUN_func_offset`

```rust
const DW_AT_SUN_func_offset: DwAt;
```

### `DW_AT_SUN_memop_type_ref`

```rust
const DW_AT_SUN_memop_type_ref: DwAt;
```

### `DW_AT_SUN_profile_id`

```rust
const DW_AT_SUN_profile_id: DwAt;
```

### `DW_AT_SUN_memop_signature`

```rust
const DW_AT_SUN_memop_signature: DwAt;
```

### `DW_AT_SUN_obj_dir`

```rust
const DW_AT_SUN_obj_dir: DwAt;
```

### `DW_AT_SUN_obj_file`

```rust
const DW_AT_SUN_obj_file: DwAt;
```

### `DW_AT_SUN_original_name`

```rust
const DW_AT_SUN_original_name: DwAt;
```

### `DW_AT_SUN_hwcprof_signature`

```rust
const DW_AT_SUN_hwcprof_signature: DwAt;
```

### `DW_AT_SUN_amd64_parmdump`

```rust
const DW_AT_SUN_amd64_parmdump: DwAt;
```

### `DW_AT_SUN_part_link_name`

```rust
const DW_AT_SUN_part_link_name: DwAt;
```

### `DW_AT_SUN_link_name`

```rust
const DW_AT_SUN_link_name: DwAt;
```

### `DW_AT_SUN_pass_with_const`

```rust
const DW_AT_SUN_pass_with_const: DwAt;
```

### `DW_AT_SUN_return_with_const`

```rust
const DW_AT_SUN_return_with_const: DwAt;
```

### `DW_AT_SUN_import_by_name`

```rust
const DW_AT_SUN_import_by_name: DwAt;
```

### `DW_AT_SUN_f90_pointer`

```rust
const DW_AT_SUN_f90_pointer: DwAt;
```

### `DW_AT_SUN_pass_by_ref`

```rust
const DW_AT_SUN_pass_by_ref: DwAt;
```

### `DW_AT_SUN_f90_allocatable`

```rust
const DW_AT_SUN_f90_allocatable: DwAt;
```

### `DW_AT_SUN_f90_assumed_shape_array`

```rust
const DW_AT_SUN_f90_assumed_shape_array: DwAt;
```

### `DW_AT_SUN_c_vla`

```rust
const DW_AT_SUN_c_vla: DwAt;
```

### `DW_AT_SUN_return_value_ptr`

```rust
const DW_AT_SUN_return_value_ptr: DwAt;
```

### `DW_AT_SUN_dtor_start`

```rust
const DW_AT_SUN_dtor_start: DwAt;
```

### `DW_AT_SUN_dtor_length`

```rust
const DW_AT_SUN_dtor_length: DwAt;
```

### `DW_AT_SUN_dtor_state_initial`

```rust
const DW_AT_SUN_dtor_state_initial: DwAt;
```

### `DW_AT_SUN_dtor_state_final`

```rust
const DW_AT_SUN_dtor_state_final: DwAt;
```

### `DW_AT_SUN_dtor_state_deltas`

```rust
const DW_AT_SUN_dtor_state_deltas: DwAt;
```

### `DW_AT_SUN_import_by_lname`

```rust
const DW_AT_SUN_import_by_lname: DwAt;
```

### `DW_AT_SUN_f90_use_only`

```rust
const DW_AT_SUN_f90_use_only: DwAt;
```

### `DW_AT_SUN_namelist_spec`

```rust
const DW_AT_SUN_namelist_spec: DwAt;
```

### `DW_AT_SUN_is_omp_child_func`

```rust
const DW_AT_SUN_is_omp_child_func: DwAt;
```

### `DW_AT_SUN_fortran_main_alias`

```rust
const DW_AT_SUN_fortran_main_alias: DwAt;
```

### `DW_AT_SUN_fortran_based`

```rust
const DW_AT_SUN_fortran_based: DwAt;
```

### `DW_AT_ALTIUM_loclist`

```rust
const DW_AT_ALTIUM_loclist: DwAt;
```

### `DW_AT_use_GNAT_descriptive_type`

```rust
const DW_AT_use_GNAT_descriptive_type: DwAt;
```

### `DW_AT_GNAT_descriptive_type`

```rust
const DW_AT_GNAT_descriptive_type: DwAt;
```

### `DW_AT_GNU_numerator`

```rust
const DW_AT_GNU_numerator: DwAt;
```

### `DW_AT_GNU_denominator`

```rust
const DW_AT_GNU_denominator: DwAt;
```

### `DW_AT_GNU_bias`

```rust
const DW_AT_GNU_bias: DwAt;
```

### `DW_AT_upc_threads_scaled`

```rust
const DW_AT_upc_threads_scaled: DwAt;
```

### `DW_AT_PGI_lbase`

```rust
const DW_AT_PGI_lbase: DwAt;
```

### `DW_AT_PGI_soffset`

```rust
const DW_AT_PGI_soffset: DwAt;
```

### `DW_AT_PGI_lstride`

```rust
const DW_AT_PGI_lstride: DwAt;
```

### `DW_AT_BORLAND_property_read`

```rust
const DW_AT_BORLAND_property_read: DwAt;
```

### `DW_AT_BORLAND_property_write`

```rust
const DW_AT_BORLAND_property_write: DwAt;
```

### `DW_AT_BORLAND_property_implements`

```rust
const DW_AT_BORLAND_property_implements: DwAt;
```

### `DW_AT_BORLAND_property_index`

```rust
const DW_AT_BORLAND_property_index: DwAt;
```

### `DW_AT_BORLAND_property_default`

```rust
const DW_AT_BORLAND_property_default: DwAt;
```

### `DW_AT_BORLAND_Delphi_unit`

```rust
const DW_AT_BORLAND_Delphi_unit: DwAt;
```

### `DW_AT_BORLAND_Delphi_class`

```rust
const DW_AT_BORLAND_Delphi_class: DwAt;
```

### `DW_AT_BORLAND_Delphi_record`

```rust
const DW_AT_BORLAND_Delphi_record: DwAt;
```

### `DW_AT_BORLAND_Delphi_metaclass`

```rust
const DW_AT_BORLAND_Delphi_metaclass: DwAt;
```

### `DW_AT_BORLAND_Delphi_constructor`

```rust
const DW_AT_BORLAND_Delphi_constructor: DwAt;
```

### `DW_AT_BORLAND_Delphi_destructor`

```rust
const DW_AT_BORLAND_Delphi_destructor: DwAt;
```

### `DW_AT_BORLAND_Delphi_anonymous_method`

```rust
const DW_AT_BORLAND_Delphi_anonymous_method: DwAt;
```

### `DW_AT_BORLAND_Delphi_interface`

```rust
const DW_AT_BORLAND_Delphi_interface: DwAt;
```

### `DW_AT_BORLAND_Delphi_ABI`

```rust
const DW_AT_BORLAND_Delphi_ABI: DwAt;
```

### `DW_AT_BORLAND_Delphi_return`

```rust
const DW_AT_BORLAND_Delphi_return: DwAt;
```

### `DW_AT_BORLAND_Delphi_frameptr`

```rust
const DW_AT_BORLAND_Delphi_frameptr: DwAt;
```

### `DW_AT_BORLAND_closure`

```rust
const DW_AT_BORLAND_closure: DwAt;
```

### `DW_AT_LLVM_include_path`

```rust
const DW_AT_LLVM_include_path: DwAt;
```

### `DW_AT_LLVM_config_macros`

```rust
const DW_AT_LLVM_config_macros: DwAt;
```

### `DW_AT_LLVM_isysroot`

```rust
const DW_AT_LLVM_isysroot: DwAt;
```

### `DW_AT_APPLE_optimized`

```rust
const DW_AT_APPLE_optimized: DwAt;
```

### `DW_AT_APPLE_flags`

```rust
const DW_AT_APPLE_flags: DwAt;
```

### `DW_AT_APPLE_isa`

```rust
const DW_AT_APPLE_isa: DwAt;
```

### `DW_AT_APPLE_block`

```rust
const DW_AT_APPLE_block: DwAt;
```

### `DW_AT_APPLE_major_runtime_vers`

```rust
const DW_AT_APPLE_major_runtime_vers: DwAt;
```

### `DW_AT_APPLE_runtime_class`

```rust
const DW_AT_APPLE_runtime_class: DwAt;
```

### `DW_AT_APPLE_omit_frame_ptr`

```rust
const DW_AT_APPLE_omit_frame_ptr: DwAt;
```

### `DW_AT_APPLE_property_name`

```rust
const DW_AT_APPLE_property_name: DwAt;
```

### `DW_AT_APPLE_property_getter`

```rust
const DW_AT_APPLE_property_getter: DwAt;
```

### `DW_AT_APPLE_property_setter`

```rust
const DW_AT_APPLE_property_setter: DwAt;
```

### `DW_AT_APPLE_property_attribute`

```rust
const DW_AT_APPLE_property_attribute: DwAt;
```

### `DW_AT_APPLE_objc_complete_type`

```rust
const DW_AT_APPLE_objc_complete_type: DwAt;
```

### `DW_AT_APPLE_property`

```rust
const DW_AT_APPLE_property: DwAt;
```

### `DW_FORM_null`

```rust
const DW_FORM_null: DwForm;
```

### `DW_FORM_ref`

```rust
const DW_FORM_ref: DwForm;
```

### `DW_FORM_addr`

```rust
const DW_FORM_addr: DwForm;
```

### `DW_FORM_block2`

```rust
const DW_FORM_block2: DwForm;
```

### `DW_FORM_block4`

```rust
const DW_FORM_block4: DwForm;
```

### `DW_FORM_data2`

```rust
const DW_FORM_data2: DwForm;
```

### `DW_FORM_data4`

```rust
const DW_FORM_data4: DwForm;
```

### `DW_FORM_data8`

```rust
const DW_FORM_data8: DwForm;
```

### `DW_FORM_string`

```rust
const DW_FORM_string: DwForm;
```

### `DW_FORM_block`

```rust
const DW_FORM_block: DwForm;
```

### `DW_FORM_block1`

```rust
const DW_FORM_block1: DwForm;
```

### `DW_FORM_data1`

```rust
const DW_FORM_data1: DwForm;
```

### `DW_FORM_flag`

```rust
const DW_FORM_flag: DwForm;
```

### `DW_FORM_sdata`

```rust
const DW_FORM_sdata: DwForm;
```

### `DW_FORM_strp`

```rust
const DW_FORM_strp: DwForm;
```

### `DW_FORM_udata`

```rust
const DW_FORM_udata: DwForm;
```

### `DW_FORM_ref_addr`

```rust
const DW_FORM_ref_addr: DwForm;
```

### `DW_FORM_ref1`

```rust
const DW_FORM_ref1: DwForm;
```

### `DW_FORM_ref2`

```rust
const DW_FORM_ref2: DwForm;
```

### `DW_FORM_ref4`

```rust
const DW_FORM_ref4: DwForm;
```

### `DW_FORM_ref8`

```rust
const DW_FORM_ref8: DwForm;
```

### `DW_FORM_ref_udata`

```rust
const DW_FORM_ref_udata: DwForm;
```

### `DW_FORM_indirect`

```rust
const DW_FORM_indirect: DwForm;
```

### `DW_FORM_sec_offset`

```rust
const DW_FORM_sec_offset: DwForm;
```

### `DW_FORM_exprloc`

```rust
const DW_FORM_exprloc: DwForm;
```

### `DW_FORM_flag_present`

```rust
const DW_FORM_flag_present: DwForm;
```

### `DW_FORM_ref_sig8`

```rust
const DW_FORM_ref_sig8: DwForm;
```

### `DW_FORM_strx`

```rust
const DW_FORM_strx: DwForm;
```

### `DW_FORM_addrx`

```rust
const DW_FORM_addrx: DwForm;
```

### `DW_FORM_ref_sup4`

```rust
const DW_FORM_ref_sup4: DwForm;
```

### `DW_FORM_strp_sup`

```rust
const DW_FORM_strp_sup: DwForm;
```

### `DW_FORM_data16`

```rust
const DW_FORM_data16: DwForm;
```

### `DW_FORM_line_strp`

```rust
const DW_FORM_line_strp: DwForm;
```

### `DW_FORM_implicit_const`

```rust
const DW_FORM_implicit_const: DwForm;
```

### `DW_FORM_loclistx`

```rust
const DW_FORM_loclistx: DwForm;
```

### `DW_FORM_rnglistx`

```rust
const DW_FORM_rnglistx: DwForm;
```

### `DW_FORM_ref_sup8`

```rust
const DW_FORM_ref_sup8: DwForm;
```

### `DW_FORM_strx1`

```rust
const DW_FORM_strx1: DwForm;
```

### `DW_FORM_strx2`

```rust
const DW_FORM_strx2: DwForm;
```

### `DW_FORM_strx3`

```rust
const DW_FORM_strx3: DwForm;
```

### `DW_FORM_strx4`

```rust
const DW_FORM_strx4: DwForm;
```

### `DW_FORM_addrx1`

```rust
const DW_FORM_addrx1: DwForm;
```

### `DW_FORM_addrx2`

```rust
const DW_FORM_addrx2: DwForm;
```

### `DW_FORM_addrx3`

```rust
const DW_FORM_addrx3: DwForm;
```

### `DW_FORM_addrx4`

```rust
const DW_FORM_addrx4: DwForm;
```

### `DW_FORM_GNU_addr_index`

```rust
const DW_FORM_GNU_addr_index: DwForm;
```

### `DW_FORM_GNU_str_index`

```rust
const DW_FORM_GNU_str_index: DwForm;
```

### `DW_FORM_GNU_ref_alt`

```rust
const DW_FORM_GNU_ref_alt: DwForm;
```

### `DW_FORM_GNU_strp_alt`

```rust
const DW_FORM_GNU_strp_alt: DwForm;
```

### `DW_ATE_address`

```rust
const DW_ATE_address: DwAte;
```

### `DW_ATE_boolean`

```rust
const DW_ATE_boolean: DwAte;
```

### `DW_ATE_complex_float`

```rust
const DW_ATE_complex_float: DwAte;
```

### `DW_ATE_float`

```rust
const DW_ATE_float: DwAte;
```

### `DW_ATE_signed`

```rust
const DW_ATE_signed: DwAte;
```

### `DW_ATE_signed_char`

```rust
const DW_ATE_signed_char: DwAte;
```

### `DW_ATE_unsigned`

```rust
const DW_ATE_unsigned: DwAte;
```

### `DW_ATE_unsigned_char`

```rust
const DW_ATE_unsigned_char: DwAte;
```

### `DW_ATE_imaginary_float`

```rust
const DW_ATE_imaginary_float: DwAte;
```

### `DW_ATE_packed_decimal`

```rust
const DW_ATE_packed_decimal: DwAte;
```

### `DW_ATE_numeric_string`

```rust
const DW_ATE_numeric_string: DwAte;
```

### `DW_ATE_edited`

```rust
const DW_ATE_edited: DwAte;
```

### `DW_ATE_signed_fixed`

```rust
const DW_ATE_signed_fixed: DwAte;
```

### `DW_ATE_unsigned_fixed`

```rust
const DW_ATE_unsigned_fixed: DwAte;
```

### `DW_ATE_decimal_float`

```rust
const DW_ATE_decimal_float: DwAte;
```

### `DW_ATE_UTF`

```rust
const DW_ATE_UTF: DwAte;
```

### `DW_ATE_UCS`

```rust
const DW_ATE_UCS: DwAte;
```

### `DW_ATE_ASCII`

```rust
const DW_ATE_ASCII: DwAte;
```

### `DW_ATE_lo_user`

```rust
const DW_ATE_lo_user: DwAte;
```

### `DW_ATE_hi_user`

```rust
const DW_ATE_hi_user: DwAte;
```

### `DW_LLE_end_of_list`

```rust
const DW_LLE_end_of_list: DwLle;
```

### `DW_LLE_base_addressx`

```rust
const DW_LLE_base_addressx: DwLle;
```

### `DW_LLE_startx_endx`

```rust
const DW_LLE_startx_endx: DwLle;
```

### `DW_LLE_startx_length`

```rust
const DW_LLE_startx_length: DwLle;
```

### `DW_LLE_offset_pair`

```rust
const DW_LLE_offset_pair: DwLle;
```

### `DW_LLE_default_location`

```rust
const DW_LLE_default_location: DwLle;
```

### `DW_LLE_base_address`

```rust
const DW_LLE_base_address: DwLle;
```

### `DW_LLE_start_end`

```rust
const DW_LLE_start_end: DwLle;
```

### `DW_LLE_start_length`

```rust
const DW_LLE_start_length: DwLle;
```

### `DW_LLE_GNU_view_pair`

```rust
const DW_LLE_GNU_view_pair: DwLle;
```

### `DW_DS_unsigned`

```rust
const DW_DS_unsigned: DwDs;
```

### `DW_DS_leading_overpunch`

```rust
const DW_DS_leading_overpunch: DwDs;
```

### `DW_DS_trailing_overpunch`

```rust
const DW_DS_trailing_overpunch: DwDs;
```

### `DW_DS_leading_separate`

```rust
const DW_DS_leading_separate: DwDs;
```

### `DW_DS_trailing_separate`

```rust
const DW_DS_trailing_separate: DwDs;
```

### `DW_END_default`

```rust
const DW_END_default: DwEnd;
```

### `DW_END_big`

```rust
const DW_END_big: DwEnd;
```

### `DW_END_little`

```rust
const DW_END_little: DwEnd;
```

### `DW_END_lo_user`

```rust
const DW_END_lo_user: DwEnd;
```

### `DW_END_hi_user`

```rust
const DW_END_hi_user: DwEnd;
```

### `DW_ACCESS_public`

```rust
const DW_ACCESS_public: DwAccess;
```

### `DW_ACCESS_protected`

```rust
const DW_ACCESS_protected: DwAccess;
```

### `DW_ACCESS_private`

```rust
const DW_ACCESS_private: DwAccess;
```

### `DW_VIS_local`

```rust
const DW_VIS_local: DwVis;
```

### `DW_VIS_exported`

```rust
const DW_VIS_exported: DwVis;
```

### `DW_VIS_qualified`

```rust
const DW_VIS_qualified: DwVis;
```

### `DW_VIRTUALITY_none`

```rust
const DW_VIRTUALITY_none: DwVirtuality;
```

### `DW_VIRTUALITY_virtual`

```rust
const DW_VIRTUALITY_virtual: DwVirtuality;
```

### `DW_VIRTUALITY_pure_virtual`

```rust
const DW_VIRTUALITY_pure_virtual: DwVirtuality;
```

### `DW_LANG_C89`

```rust
const DW_LANG_C89: DwLang;
```

### `DW_LANG_C`

```rust
const DW_LANG_C: DwLang;
```

### `DW_LANG_Ada83`

```rust
const DW_LANG_Ada83: DwLang;
```

### `DW_LANG_C_plus_plus`

```rust
const DW_LANG_C_plus_plus: DwLang;
```

### `DW_LANG_Cobol74`

```rust
const DW_LANG_Cobol74: DwLang;
```

### `DW_LANG_Cobol85`

```rust
const DW_LANG_Cobol85: DwLang;
```

### `DW_LANG_Fortran77`

```rust
const DW_LANG_Fortran77: DwLang;
```

### `DW_LANG_Fortran90`

```rust
const DW_LANG_Fortran90: DwLang;
```

### `DW_LANG_Pascal83`

```rust
const DW_LANG_Pascal83: DwLang;
```

### `DW_LANG_Modula2`

```rust
const DW_LANG_Modula2: DwLang;
```

### `DW_LANG_Java`

```rust
const DW_LANG_Java: DwLang;
```

### `DW_LANG_C99`

```rust
const DW_LANG_C99: DwLang;
```

### `DW_LANG_Ada95`

```rust
const DW_LANG_Ada95: DwLang;
```

### `DW_LANG_Fortran95`

```rust
const DW_LANG_Fortran95: DwLang;
```

### `DW_LANG_PLI`

```rust
const DW_LANG_PLI: DwLang;
```

### `DW_LANG_ObjC`

```rust
const DW_LANG_ObjC: DwLang;
```

### `DW_LANG_ObjC_plus_plus`

```rust
const DW_LANG_ObjC_plus_plus: DwLang;
```

### `DW_LANG_UPC`

```rust
const DW_LANG_UPC: DwLang;
```

### `DW_LANG_D`

```rust
const DW_LANG_D: DwLang;
```

### `DW_LANG_Python`

```rust
const DW_LANG_Python: DwLang;
```

### `DW_LANG_OpenCL`

```rust
const DW_LANG_OpenCL: DwLang;
```

### `DW_LANG_Go`

```rust
const DW_LANG_Go: DwLang;
```

### `DW_LANG_Modula3`

```rust
const DW_LANG_Modula3: DwLang;
```

### `DW_LANG_Haskell`

```rust
const DW_LANG_Haskell: DwLang;
```

### `DW_LANG_C_plus_plus_03`

```rust
const DW_LANG_C_plus_plus_03: DwLang;
```

### `DW_LANG_C_plus_plus_11`

```rust
const DW_LANG_C_plus_plus_11: DwLang;
```

### `DW_LANG_OCaml`

```rust
const DW_LANG_OCaml: DwLang;
```

### `DW_LANG_Rust`

```rust
const DW_LANG_Rust: DwLang;
```

### `DW_LANG_C11`

```rust
const DW_LANG_C11: DwLang;
```

### `DW_LANG_Swift`

```rust
const DW_LANG_Swift: DwLang;
```

### `DW_LANG_Julia`

```rust
const DW_LANG_Julia: DwLang;
```

### `DW_LANG_Dylan`

```rust
const DW_LANG_Dylan: DwLang;
```

### `DW_LANG_C_plus_plus_14`

```rust
const DW_LANG_C_plus_plus_14: DwLang;
```

### `DW_LANG_Fortran03`

```rust
const DW_LANG_Fortran03: DwLang;
```

### `DW_LANG_Fortran08`

```rust
const DW_LANG_Fortran08: DwLang;
```

### `DW_LANG_RenderScript`

```rust
const DW_LANG_RenderScript: DwLang;
```

### `DW_LANG_BLISS`

```rust
const DW_LANG_BLISS: DwLang;
```

### `DW_LANG_Kotlin`

```rust
const DW_LANG_Kotlin: DwLang;
```

### `DW_LANG_Zig`

```rust
const DW_LANG_Zig: DwLang;
```

### `DW_LANG_Crystal`

```rust
const DW_LANG_Crystal: DwLang;
```

### `DW_LANG_C_plus_plus_17`

```rust
const DW_LANG_C_plus_plus_17: DwLang;
```

### `DW_LANG_C_plus_plus_20`

```rust
const DW_LANG_C_plus_plus_20: DwLang;
```

### `DW_LANG_C17`

```rust
const DW_LANG_C17: DwLang;
```

### `DW_LANG_Fortran18`

```rust
const DW_LANG_Fortran18: DwLang;
```

### `DW_LANG_Ada2005`

```rust
const DW_LANG_Ada2005: DwLang;
```

### `DW_LANG_Ada2012`

```rust
const DW_LANG_Ada2012: DwLang;
```

### `DW_LANG_lo_user`

```rust
const DW_LANG_lo_user: DwLang;
```

### `DW_LANG_hi_user`

```rust
const DW_LANG_hi_user: DwLang;
```

### `DW_LANG_Mips_Assembler`

```rust
const DW_LANG_Mips_Assembler: DwLang;
```

### `DW_LANG_GOOGLE_RenderScript`

```rust
const DW_LANG_GOOGLE_RenderScript: DwLang;
```

### `DW_LANG_SUN_Assembler`

```rust
const DW_LANG_SUN_Assembler: DwLang;
```

### `DW_LANG_ALTIUM_Assembler`

```rust
const DW_LANG_ALTIUM_Assembler: DwLang;
```

### `DW_LANG_BORLAND_Delphi`

```rust
const DW_LANG_BORLAND_Delphi: DwLang;
```

### `DW_ADDR_none`

```rust
const DW_ADDR_none: DwAddr;
```

### `DW_ID_case_sensitive`

```rust
const DW_ID_case_sensitive: DwId;
```

### `DW_ID_up_case`

```rust
const DW_ID_up_case: DwId;
```

### `DW_ID_down_case`

```rust
const DW_ID_down_case: DwId;
```

### `DW_ID_case_insensitive`

```rust
const DW_ID_case_insensitive: DwId;
```

### `DW_CC_normal`

```rust
const DW_CC_normal: DwCc;
```

### `DW_CC_program`

```rust
const DW_CC_program: DwCc;
```

### `DW_CC_nocall`

```rust
const DW_CC_nocall: DwCc;
```

### `DW_CC_pass_by_reference`

```rust
const DW_CC_pass_by_reference: DwCc;
```

### `DW_CC_pass_by_value`

```rust
const DW_CC_pass_by_value: DwCc;
```

### `DW_CC_lo_user`

```rust
const DW_CC_lo_user: DwCc;
```

### `DW_CC_hi_user`

```rust
const DW_CC_hi_user: DwCc;
```

### `DW_INL_not_inlined`

```rust
const DW_INL_not_inlined: DwInl;
```

### `DW_INL_inlined`

```rust
const DW_INL_inlined: DwInl;
```

### `DW_INL_declared_not_inlined`

```rust
const DW_INL_declared_not_inlined: DwInl;
```

### `DW_INL_declared_inlined`

```rust
const DW_INL_declared_inlined: DwInl;
```

### `DW_ORD_row_major`

```rust
const DW_ORD_row_major: DwOrd;
```

### `DW_ORD_col_major`

```rust
const DW_ORD_col_major: DwOrd;
```

### `DW_DSC_label`

```rust
const DW_DSC_label: DwDsc;
```

### `DW_DSC_range`

```rust
const DW_DSC_range: DwDsc;
```

### `DW_IDX_compile_unit`

```rust
const DW_IDX_compile_unit: DwIdx;
```

### `DW_IDX_type_unit`

```rust
const DW_IDX_type_unit: DwIdx;
```

### `DW_IDX_die_offset`

```rust
const DW_IDX_die_offset: DwIdx;
```

### `DW_IDX_parent`

```rust
const DW_IDX_parent: DwIdx;
```

### `DW_IDX_type_hash`

```rust
const DW_IDX_type_hash: DwIdx;
```

### `DW_IDX_lo_user`

```rust
const DW_IDX_lo_user: DwIdx;
```

### `DW_IDX_hi_user`

```rust
const DW_IDX_hi_user: DwIdx;
```

### `DW_DEFAULTED_no`

```rust
const DW_DEFAULTED_no: DwDefaulted;
```

### `DW_DEFAULTED_in_class`

```rust
const DW_DEFAULTED_in_class: DwDefaulted;
```

### `DW_DEFAULTED_out_of_class`

```rust
const DW_DEFAULTED_out_of_class: DwDefaulted;
```

### `DW_LNS_copy`

```rust
const DW_LNS_copy: DwLns;
```

### `DW_LNS_advance_pc`

```rust
const DW_LNS_advance_pc: DwLns;
```

### `DW_LNS_advance_line`

```rust
const DW_LNS_advance_line: DwLns;
```

### `DW_LNS_set_file`

```rust
const DW_LNS_set_file: DwLns;
```

### `DW_LNS_set_column`

```rust
const DW_LNS_set_column: DwLns;
```

### `DW_LNS_negate_stmt`

```rust
const DW_LNS_negate_stmt: DwLns;
```

### `DW_LNS_set_basic_block`

```rust
const DW_LNS_set_basic_block: DwLns;
```

### `DW_LNS_const_add_pc`

```rust
const DW_LNS_const_add_pc: DwLns;
```

### `DW_LNS_fixed_advance_pc`

```rust
const DW_LNS_fixed_advance_pc: DwLns;
```

### `DW_LNS_set_prologue_end`

```rust
const DW_LNS_set_prologue_end: DwLns;
```

### `DW_LNS_set_epilogue_begin`

```rust
const DW_LNS_set_epilogue_begin: DwLns;
```

### `DW_LNS_set_isa`

```rust
const DW_LNS_set_isa: DwLns;
```

### `DW_LNE_end_sequence`

```rust
const DW_LNE_end_sequence: DwLne;
```

### `DW_LNE_set_address`

```rust
const DW_LNE_set_address: DwLne;
```

### `DW_LNE_define_file`

```rust
const DW_LNE_define_file: DwLne;
```

### `DW_LNE_set_discriminator`

```rust
const DW_LNE_set_discriminator: DwLne;
```

### `DW_LNE_lo_user`

```rust
const DW_LNE_lo_user: DwLne;
```

### `DW_LNE_hi_user`

```rust
const DW_LNE_hi_user: DwLne;
```

### `DW_LNCT_path`

```rust
const DW_LNCT_path: DwLnct;
```

### `DW_LNCT_directory_index`

```rust
const DW_LNCT_directory_index: DwLnct;
```

### `DW_LNCT_timestamp`

```rust
const DW_LNCT_timestamp: DwLnct;
```

### `DW_LNCT_size`

```rust
const DW_LNCT_size: DwLnct;
```

### `DW_LNCT_MD5`

```rust
const DW_LNCT_MD5: DwLnct;
```

### `DW_LNCT_lo_user`

```rust
const DW_LNCT_lo_user: DwLnct;
```

### `DW_LNCT_LLVM_source`

```rust
const DW_LNCT_LLVM_source: DwLnct;
```

### `DW_LNCT_hi_user`

```rust
const DW_LNCT_hi_user: DwLnct;
```

### `DW_MACINFO_define`

```rust
const DW_MACINFO_define: DwMacinfo;
```

### `DW_MACINFO_undef`

```rust
const DW_MACINFO_undef: DwMacinfo;
```

### `DW_MACINFO_start_file`

```rust
const DW_MACINFO_start_file: DwMacinfo;
```

### `DW_MACINFO_end_file`

```rust
const DW_MACINFO_end_file: DwMacinfo;
```

### `DW_MACINFO_vendor_ext`

```rust
const DW_MACINFO_vendor_ext: DwMacinfo;
```

### `DW_MACRO_define`

```rust
const DW_MACRO_define: DwMacro;
```

### `DW_MACRO_undef`

```rust
const DW_MACRO_undef: DwMacro;
```

### `DW_MACRO_start_file`

```rust
const DW_MACRO_start_file: DwMacro;
```

### `DW_MACRO_end_file`

```rust
const DW_MACRO_end_file: DwMacro;
```

### `DW_MACRO_define_strp`

```rust
const DW_MACRO_define_strp: DwMacro;
```

### `DW_MACRO_undef_strp`

```rust
const DW_MACRO_undef_strp: DwMacro;
```

### `DW_MACRO_import`

```rust
const DW_MACRO_import: DwMacro;
```

### `DW_MACRO_define_sup`

```rust
const DW_MACRO_define_sup: DwMacro;
```

### `DW_MACRO_undef_sup`

```rust
const DW_MACRO_undef_sup: DwMacro;
```

### `DW_MACRO_import_sup`

```rust
const DW_MACRO_import_sup: DwMacro;
```

### `DW_MACRO_define_strx`

```rust
const DW_MACRO_define_strx: DwMacro;
```

### `DW_MACRO_undef_strx`

```rust
const DW_MACRO_undef_strx: DwMacro;
```

### `DW_MACRO_lo_user`

```rust
const DW_MACRO_lo_user: DwMacro;
```

### `DW_MACRO_hi_user`

```rust
const DW_MACRO_hi_user: DwMacro;
```

### `DW_RLE_end_of_list`

```rust
const DW_RLE_end_of_list: DwRle;
```

### `DW_RLE_base_addressx`

```rust
const DW_RLE_base_addressx: DwRle;
```

### `DW_RLE_startx_endx`

```rust
const DW_RLE_startx_endx: DwRle;
```

### `DW_RLE_startx_length`

```rust
const DW_RLE_startx_length: DwRle;
```

### `DW_RLE_offset_pair`

```rust
const DW_RLE_offset_pair: DwRle;
```

### `DW_RLE_base_address`

```rust
const DW_RLE_base_address: DwRle;
```

### `DW_RLE_start_end`

```rust
const DW_RLE_start_end: DwRle;
```

### `DW_RLE_start_length`

```rust
const DW_RLE_start_length: DwRle;
```

### `DW_OP_addr`

```rust
const DW_OP_addr: DwOp;
```

### `DW_OP_deref`

```rust
const DW_OP_deref: DwOp;
```

### `DW_OP_const1u`

```rust
const DW_OP_const1u: DwOp;
```

### `DW_OP_const1s`

```rust
const DW_OP_const1s: DwOp;
```

### `DW_OP_const2u`

```rust
const DW_OP_const2u: DwOp;
```

### `DW_OP_const2s`

```rust
const DW_OP_const2s: DwOp;
```

### `DW_OP_const4u`

```rust
const DW_OP_const4u: DwOp;
```

### `DW_OP_const4s`

```rust
const DW_OP_const4s: DwOp;
```

### `DW_OP_const8u`

```rust
const DW_OP_const8u: DwOp;
```

### `DW_OP_const8s`

```rust
const DW_OP_const8s: DwOp;
```

### `DW_OP_constu`

```rust
const DW_OP_constu: DwOp;
```

### `DW_OP_consts`

```rust
const DW_OP_consts: DwOp;
```

### `DW_OP_dup`

```rust
const DW_OP_dup: DwOp;
```

### `DW_OP_drop`

```rust
const DW_OP_drop: DwOp;
```

### `DW_OP_over`

```rust
const DW_OP_over: DwOp;
```

### `DW_OP_pick`

```rust
const DW_OP_pick: DwOp;
```

### `DW_OP_swap`

```rust
const DW_OP_swap: DwOp;
```

### `DW_OP_rot`

```rust
const DW_OP_rot: DwOp;
```

### `DW_OP_xderef`

```rust
const DW_OP_xderef: DwOp;
```

### `DW_OP_abs`

```rust
const DW_OP_abs: DwOp;
```

### `DW_OP_and`

```rust
const DW_OP_and: DwOp;
```

### `DW_OP_div`

```rust
const DW_OP_div: DwOp;
```

### `DW_OP_minus`

```rust
const DW_OP_minus: DwOp;
```

### `DW_OP_mod`

```rust
const DW_OP_mod: DwOp;
```

### `DW_OP_mul`

```rust
const DW_OP_mul: DwOp;
```

### `DW_OP_neg`

```rust
const DW_OP_neg: DwOp;
```

### `DW_OP_not`

```rust
const DW_OP_not: DwOp;
```

### `DW_OP_or`

```rust
const DW_OP_or: DwOp;
```

### `DW_OP_plus`

```rust
const DW_OP_plus: DwOp;
```

### `DW_OP_plus_uconst`

```rust
const DW_OP_plus_uconst: DwOp;
```

### `DW_OP_shl`

```rust
const DW_OP_shl: DwOp;
```

### `DW_OP_shr`

```rust
const DW_OP_shr: DwOp;
```

### `DW_OP_shra`

```rust
const DW_OP_shra: DwOp;
```

### `DW_OP_xor`

```rust
const DW_OP_xor: DwOp;
```

### `DW_OP_bra`

```rust
const DW_OP_bra: DwOp;
```

### `DW_OP_eq`

```rust
const DW_OP_eq: DwOp;
```

### `DW_OP_ge`

```rust
const DW_OP_ge: DwOp;
```

### `DW_OP_gt`

```rust
const DW_OP_gt: DwOp;
```

### `DW_OP_le`

```rust
const DW_OP_le: DwOp;
```

### `DW_OP_lt`

```rust
const DW_OP_lt: DwOp;
```

### `DW_OP_ne`

```rust
const DW_OP_ne: DwOp;
```

### `DW_OP_skip`

```rust
const DW_OP_skip: DwOp;
```

### `DW_OP_lit0`

```rust
const DW_OP_lit0: DwOp;
```

### `DW_OP_lit1`

```rust
const DW_OP_lit1: DwOp;
```

### `DW_OP_lit2`

```rust
const DW_OP_lit2: DwOp;
```

### `DW_OP_lit3`

```rust
const DW_OP_lit3: DwOp;
```

### `DW_OP_lit4`

```rust
const DW_OP_lit4: DwOp;
```

### `DW_OP_lit5`

```rust
const DW_OP_lit5: DwOp;
```

### `DW_OP_lit6`

```rust
const DW_OP_lit6: DwOp;
```

### `DW_OP_lit7`

```rust
const DW_OP_lit7: DwOp;
```

### `DW_OP_lit8`

```rust
const DW_OP_lit8: DwOp;
```

### `DW_OP_lit9`

```rust
const DW_OP_lit9: DwOp;
```

### `DW_OP_lit10`

```rust
const DW_OP_lit10: DwOp;
```

### `DW_OP_lit11`

```rust
const DW_OP_lit11: DwOp;
```

### `DW_OP_lit12`

```rust
const DW_OP_lit12: DwOp;
```

### `DW_OP_lit13`

```rust
const DW_OP_lit13: DwOp;
```

### `DW_OP_lit14`

```rust
const DW_OP_lit14: DwOp;
```

### `DW_OP_lit15`

```rust
const DW_OP_lit15: DwOp;
```

### `DW_OP_lit16`

```rust
const DW_OP_lit16: DwOp;
```

### `DW_OP_lit17`

```rust
const DW_OP_lit17: DwOp;
```

### `DW_OP_lit18`

```rust
const DW_OP_lit18: DwOp;
```

### `DW_OP_lit19`

```rust
const DW_OP_lit19: DwOp;
```

### `DW_OP_lit20`

```rust
const DW_OP_lit20: DwOp;
```

### `DW_OP_lit21`

```rust
const DW_OP_lit21: DwOp;
```

### `DW_OP_lit22`

```rust
const DW_OP_lit22: DwOp;
```

### `DW_OP_lit23`

```rust
const DW_OP_lit23: DwOp;
```

### `DW_OP_lit24`

```rust
const DW_OP_lit24: DwOp;
```

### `DW_OP_lit25`

```rust
const DW_OP_lit25: DwOp;
```

### `DW_OP_lit26`

```rust
const DW_OP_lit26: DwOp;
```

### `DW_OP_lit27`

```rust
const DW_OP_lit27: DwOp;
```

### `DW_OP_lit28`

```rust
const DW_OP_lit28: DwOp;
```

### `DW_OP_lit29`

```rust
const DW_OP_lit29: DwOp;
```

### `DW_OP_lit30`

```rust
const DW_OP_lit30: DwOp;
```

### `DW_OP_lit31`

```rust
const DW_OP_lit31: DwOp;
```

### `DW_OP_reg0`

```rust
const DW_OP_reg0: DwOp;
```

### `DW_OP_reg1`

```rust
const DW_OP_reg1: DwOp;
```

### `DW_OP_reg2`

```rust
const DW_OP_reg2: DwOp;
```

### `DW_OP_reg3`

```rust
const DW_OP_reg3: DwOp;
```

### `DW_OP_reg4`

```rust
const DW_OP_reg4: DwOp;
```

### `DW_OP_reg5`

```rust
const DW_OP_reg5: DwOp;
```

### `DW_OP_reg6`

```rust
const DW_OP_reg6: DwOp;
```

### `DW_OP_reg7`

```rust
const DW_OP_reg7: DwOp;
```

### `DW_OP_reg8`

```rust
const DW_OP_reg8: DwOp;
```

### `DW_OP_reg9`

```rust
const DW_OP_reg9: DwOp;
```

### `DW_OP_reg10`

```rust
const DW_OP_reg10: DwOp;
```

### `DW_OP_reg11`

```rust
const DW_OP_reg11: DwOp;
```

### `DW_OP_reg12`

```rust
const DW_OP_reg12: DwOp;
```

### `DW_OP_reg13`

```rust
const DW_OP_reg13: DwOp;
```

### `DW_OP_reg14`

```rust
const DW_OP_reg14: DwOp;
```

### `DW_OP_reg15`

```rust
const DW_OP_reg15: DwOp;
```

### `DW_OP_reg16`

```rust
const DW_OP_reg16: DwOp;
```

### `DW_OP_reg17`

```rust
const DW_OP_reg17: DwOp;
```

### `DW_OP_reg18`

```rust
const DW_OP_reg18: DwOp;
```

### `DW_OP_reg19`

```rust
const DW_OP_reg19: DwOp;
```

### `DW_OP_reg20`

```rust
const DW_OP_reg20: DwOp;
```

### `DW_OP_reg21`

```rust
const DW_OP_reg21: DwOp;
```

### `DW_OP_reg22`

```rust
const DW_OP_reg22: DwOp;
```

### `DW_OP_reg23`

```rust
const DW_OP_reg23: DwOp;
```

### `DW_OP_reg24`

```rust
const DW_OP_reg24: DwOp;
```

### `DW_OP_reg25`

```rust
const DW_OP_reg25: DwOp;
```

### `DW_OP_reg26`

```rust
const DW_OP_reg26: DwOp;
```

### `DW_OP_reg27`

```rust
const DW_OP_reg27: DwOp;
```

### `DW_OP_reg28`

```rust
const DW_OP_reg28: DwOp;
```

### `DW_OP_reg29`

```rust
const DW_OP_reg29: DwOp;
```

### `DW_OP_reg30`

```rust
const DW_OP_reg30: DwOp;
```

### `DW_OP_reg31`

```rust
const DW_OP_reg31: DwOp;
```

### `DW_OP_breg0`

```rust
const DW_OP_breg0: DwOp;
```

### `DW_OP_breg1`

```rust
const DW_OP_breg1: DwOp;
```

### `DW_OP_breg2`

```rust
const DW_OP_breg2: DwOp;
```

### `DW_OP_breg3`

```rust
const DW_OP_breg3: DwOp;
```

### `DW_OP_breg4`

```rust
const DW_OP_breg4: DwOp;
```

### `DW_OP_breg5`

```rust
const DW_OP_breg5: DwOp;
```

### `DW_OP_breg6`

```rust
const DW_OP_breg6: DwOp;
```

### `DW_OP_breg7`

```rust
const DW_OP_breg7: DwOp;
```

### `DW_OP_breg8`

```rust
const DW_OP_breg8: DwOp;
```

### `DW_OP_breg9`

```rust
const DW_OP_breg9: DwOp;
```

### `DW_OP_breg10`

```rust
const DW_OP_breg10: DwOp;
```

### `DW_OP_breg11`

```rust
const DW_OP_breg11: DwOp;
```

### `DW_OP_breg12`

```rust
const DW_OP_breg12: DwOp;
```

### `DW_OP_breg13`

```rust
const DW_OP_breg13: DwOp;
```

### `DW_OP_breg14`

```rust
const DW_OP_breg14: DwOp;
```

### `DW_OP_breg15`

```rust
const DW_OP_breg15: DwOp;
```

### `DW_OP_breg16`

```rust
const DW_OP_breg16: DwOp;
```

### `DW_OP_breg17`

```rust
const DW_OP_breg17: DwOp;
```

### `DW_OP_breg18`

```rust
const DW_OP_breg18: DwOp;
```

### `DW_OP_breg19`

```rust
const DW_OP_breg19: DwOp;
```

### `DW_OP_breg20`

```rust
const DW_OP_breg20: DwOp;
```

### `DW_OP_breg21`

```rust
const DW_OP_breg21: DwOp;
```

### `DW_OP_breg22`

```rust
const DW_OP_breg22: DwOp;
```

### `DW_OP_breg23`

```rust
const DW_OP_breg23: DwOp;
```

### `DW_OP_breg24`

```rust
const DW_OP_breg24: DwOp;
```

### `DW_OP_breg25`

```rust
const DW_OP_breg25: DwOp;
```

### `DW_OP_breg26`

```rust
const DW_OP_breg26: DwOp;
```

### `DW_OP_breg27`

```rust
const DW_OP_breg27: DwOp;
```

### `DW_OP_breg28`

```rust
const DW_OP_breg28: DwOp;
```

### `DW_OP_breg29`

```rust
const DW_OP_breg29: DwOp;
```

### `DW_OP_breg30`

```rust
const DW_OP_breg30: DwOp;
```

### `DW_OP_breg31`

```rust
const DW_OP_breg31: DwOp;
```

### `DW_OP_regx`

```rust
const DW_OP_regx: DwOp;
```

### `DW_OP_fbreg`

```rust
const DW_OP_fbreg: DwOp;
```

### `DW_OP_bregx`

```rust
const DW_OP_bregx: DwOp;
```

### `DW_OP_piece`

```rust
const DW_OP_piece: DwOp;
```

### `DW_OP_deref_size`

```rust
const DW_OP_deref_size: DwOp;
```

### `DW_OP_xderef_size`

```rust
const DW_OP_xderef_size: DwOp;
```

### `DW_OP_nop`

```rust
const DW_OP_nop: DwOp;
```

### `DW_OP_push_object_address`

```rust
const DW_OP_push_object_address: DwOp;
```

### `DW_OP_call2`

```rust
const DW_OP_call2: DwOp;
```

### `DW_OP_call4`

```rust
const DW_OP_call4: DwOp;
```

### `DW_OP_call_ref`

```rust
const DW_OP_call_ref: DwOp;
```

### `DW_OP_form_tls_address`

```rust
const DW_OP_form_tls_address: DwOp;
```

### `DW_OP_call_frame_cfa`

```rust
const DW_OP_call_frame_cfa: DwOp;
```

### `DW_OP_bit_piece`

```rust
const DW_OP_bit_piece: DwOp;
```

### `DW_OP_implicit_value`

```rust
const DW_OP_implicit_value: DwOp;
```

### `DW_OP_stack_value`

```rust
const DW_OP_stack_value: DwOp;
```

### `DW_OP_implicit_pointer`

```rust
const DW_OP_implicit_pointer: DwOp;
```

### `DW_OP_addrx`

```rust
const DW_OP_addrx: DwOp;
```

### `DW_OP_constx`

```rust
const DW_OP_constx: DwOp;
```

### `DW_OP_entry_value`

```rust
const DW_OP_entry_value: DwOp;
```

### `DW_OP_const_type`

```rust
const DW_OP_const_type: DwOp;
```

### `DW_OP_regval_type`

```rust
const DW_OP_regval_type: DwOp;
```

### `DW_OP_deref_type`

```rust
const DW_OP_deref_type: DwOp;
```

### `DW_OP_xderef_type`

```rust
const DW_OP_xderef_type: DwOp;
```

### `DW_OP_convert`

```rust
const DW_OP_convert: DwOp;
```

### `DW_OP_reinterpret`

```rust
const DW_OP_reinterpret: DwOp;
```

### `DW_OP_GNU_push_tls_address`

```rust
const DW_OP_GNU_push_tls_address: DwOp;
```

### `DW_OP_GNU_implicit_pointer`

```rust
const DW_OP_GNU_implicit_pointer: DwOp;
```

### `DW_OP_GNU_entry_value`

```rust
const DW_OP_GNU_entry_value: DwOp;
```

### `DW_OP_GNU_const_type`

```rust
const DW_OP_GNU_const_type: DwOp;
```

### `DW_OP_GNU_regval_type`

```rust
const DW_OP_GNU_regval_type: DwOp;
```

### `DW_OP_GNU_deref_type`

```rust
const DW_OP_GNU_deref_type: DwOp;
```

### `DW_OP_GNU_convert`

```rust
const DW_OP_GNU_convert: DwOp;
```

### `DW_OP_GNU_reinterpret`

```rust
const DW_OP_GNU_reinterpret: DwOp;
```

### `DW_OP_GNU_parameter_ref`

```rust
const DW_OP_GNU_parameter_ref: DwOp;
```

### `DW_OP_GNU_addr_index`

```rust
const DW_OP_GNU_addr_index: DwOp;
```

### `DW_OP_GNU_const_index`

```rust
const DW_OP_GNU_const_index: DwOp;
```

### `DW_OP_WASM_location`

```rust
const DW_OP_WASM_location: DwOp;
```

### `DW_EH_PE_uleb128`

```rust
const DW_EH_PE_uleb128: DwEhPe;
```

### `DW_EH_PE_udata2`

```rust
const DW_EH_PE_udata2: DwEhPe;
```

### `DW_EH_PE_udata4`

```rust
const DW_EH_PE_udata4: DwEhPe;
```

### `DW_EH_PE_udata8`

```rust
const DW_EH_PE_udata8: DwEhPe;
```

### `DW_EH_PE_sleb128`

```rust
const DW_EH_PE_sleb128: DwEhPe;
```

### `DW_EH_PE_sdata2`

```rust
const DW_EH_PE_sdata2: DwEhPe;
```

### `DW_EH_PE_sdata4`

```rust
const DW_EH_PE_sdata4: DwEhPe;
```

### `DW_EH_PE_sdata8`

```rust
const DW_EH_PE_sdata8: DwEhPe;
```

### `DW_EH_PE_pcrel`

```rust
const DW_EH_PE_pcrel: DwEhPe;
```

### `DW_EH_PE_textrel`

```rust
const DW_EH_PE_textrel: DwEhPe;
```

### `DW_EH_PE_datarel`

```rust
const DW_EH_PE_datarel: DwEhPe;
```

### `DW_EH_PE_funcrel`

```rust
const DW_EH_PE_funcrel: DwEhPe;
```

### `DW_EH_PE_aligned`

```rust
const DW_EH_PE_aligned: DwEhPe;
```

### `DW_EH_PE_indirect`

```rust
const DW_EH_PE_indirect: DwEhPe;
```

### `DW_EH_PE_absptr`

```rust
const DW_EH_PE_absptr: DwEhPe;
```

### `DW_EH_PE_omit`

```rust
const DW_EH_PE_omit: DwEhPe;
```

