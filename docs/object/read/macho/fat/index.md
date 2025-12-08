*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [fat](index.md)*

---

# Module `fat`

## Structs

### `FatArch32`

```rust
struct FatArch32 {
    pub cputype: crate::endian::U32<crate::endian::BigEndian>,
    pub cpusubtype: crate::endian::U32<crate::endian::BigEndian>,
    pub offset: crate::endian::U32<crate::endian::BigEndian>,
    pub size: crate::endian::U32<crate::endian::BigEndian>,
    pub align: crate::endian::U32<crate::endian::BigEndian>,
}
```

#### Fields

- **`cputype`**: `crate::endian::U32<crate::endian::BigEndian>`

  cpu specifier (int)

- **`cpusubtype`**: `crate::endian::U32<crate::endian::BigEndian>`

  machine specifier (int)

- **`offset`**: `crate::endian::U32<crate::endian::BigEndian>`

  file offset to this object file

- **`size`**: `crate::endian::U32<crate::endian::BigEndian>`

  size of this object file

- **`align`**: `crate::endian::U32<crate::endian::BigEndian>`

  alignment as a power of 2

#### Trait Implementations

##### `impl Clone for FatArch32`

- `fn clone(self: &Self) -> FatArch32` — [`FatArch32`](../../../macho/index.md)

##### `impl Copy for FatArch32`

##### `impl Debug for FatArch32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FatArch for FatArch32`

- `type Word = u32`

- `const MAGIC: u32`

- `fn cputype(self: &Self) -> u32`

- `fn cpusubtype(self: &Self) -> u32`

- `fn offset(self: &Self) -> <Self as >::Word` — [`FatArch`](../index.md)

- `fn size(self: &Self) -> <Self as >::Word` — [`FatArch`](../index.md)

- `fn align(self: &Self) -> u32`

##### `impl Pod for FatArch32`

### `FatArch64`

```rust
struct FatArch64 {
    pub cputype: crate::endian::U32<crate::endian::BigEndian>,
    pub cpusubtype: crate::endian::U32<crate::endian::BigEndian>,
    pub offset: crate::endian::U64<crate::endian::BigEndian>,
    pub size: crate::endian::U64<crate::endian::BigEndian>,
    pub align: crate::endian::U32<crate::endian::BigEndian>,
    pub reserved: crate::endian::U32<crate::endian::BigEndian>,
}
```

#### Fields

- **`cputype`**: `crate::endian::U32<crate::endian::BigEndian>`

  cpu specifier (int)

- **`cpusubtype`**: `crate::endian::U32<crate::endian::BigEndian>`

  machine specifier (int)

- **`offset`**: `crate::endian::U64<crate::endian::BigEndian>`

  file offset to this object file

- **`size`**: `crate::endian::U64<crate::endian::BigEndian>`

  size of this object file

- **`align`**: `crate::endian::U32<crate::endian::BigEndian>`

  alignment as a power of 2

- **`reserved`**: `crate::endian::U32<crate::endian::BigEndian>`

  reserved

#### Trait Implementations

##### `impl Clone for FatArch64`

- `fn clone(self: &Self) -> FatArch64` — [`FatArch64`](../../../macho/index.md)

##### `impl Copy for FatArch64`

##### `impl Debug for FatArch64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FatArch for FatArch64`

- `type Word = u64`

- `const MAGIC: u32`

- `fn cputype(self: &Self) -> u32`

- `fn cpusubtype(self: &Self) -> u32`

- `fn offset(self: &Self) -> <Self as >::Word` — [`FatArch`](../index.md)

- `fn size(self: &Self) -> <Self as >::Word` — [`FatArch`](../index.md)

- `fn align(self: &Self) -> u32`

##### `impl Pod for FatArch64`

### `FatHeader`

```rust
struct FatHeader {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub nfat_arch: crate::endian::U32<crate::endian::BigEndian>,
}
```

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  FAT_MAGIC or FAT_MAGIC_64

- **`nfat_arch`**: `crate::endian::U32<crate::endian::BigEndian>`

  number of structs that follow

#### Trait Implementations

##### `impl Clone for FatHeader`

- `fn clone(self: &Self) -> FatHeader` — [`FatHeader`](../../../macho/index.md)

##### `impl Copy for FatHeader`

##### `impl Debug for FatHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for FatHeader`

### `MachOFatFile<'data, Fat: FatArch>`

```rust
struct MachOFatFile<'data, Fat: FatArch> {
    header: &'data macho::FatHeader,
    arches: &'data [Fat],
}
```

A Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat32`](../../../index.md) or [`crate::FileKind::MachOFat64`](../../../index.md).

#### Implementations

- `fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn header(self: &Self) -> &'data macho::FatHeader` — [`FatHeader`](../../../macho/index.md)

- `fn arches(self: &Self) -> &'data [Fat]`

#### Trait Implementations

##### `impl<'data, Fat: $crate::clone::Clone + FatArch> Clone for MachOFatFile<'data, Fat>`

- `fn clone(self: &Self) -> MachOFatFile<'data, Fat>` — [`MachOFatFile`](../index.md)

##### `impl<'data, Fat: $crate::fmt::Debug + FatArch> Debug for MachOFatFile<'data, Fat>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `FatArch`

```rust
trait FatArch: Pod { ... }
```

A trait for generic access to [`macho::FatArch32`](../../../macho/index.md) and [`macho::FatArch64`](../../../macho/index.md).

#### Required Methods

- `type Word: 1`

- `const MAGIC: u32`

- `fn cputype(self: &Self) -> u32`

- `fn cpusubtype(self: &Self) -> u32`

- `fn offset(self: &Self) -> <Self as >::Word`

- `fn size(self: &Self) -> <Self as >::Word`

- `fn align(self: &Self) -> u32`

- `fn architecture(self: &Self) -> Architecture`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data<'data, R: ReadRef<'data>>(self: &Self, file: R) -> Result<&'data [u8]>`

## Type Aliases

### `MachOFatFile32<'data>`

```rust
type MachOFatFile32<'data> = MachOFatFile<'data, macho::FatArch32>;
```

A 32-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat32`](../../../index.md).

### `MachOFatFile64<'data>`

```rust
type MachOFatFile64<'data> = MachOFatFile<'data, macho::FatArch64>;
```

A 64-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat64`](../../../index.md).

