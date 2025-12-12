*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [fat](index.md)*

---

# Module `fat`

## Contents

- [Structs](#structs)
  - [`FatArch32`](#fatarch32)
  - [`FatArch64`](#fatarch64)
  - [`FatHeader`](#fatheader)
  - [`MachOFatFile`](#machofatfile)
- [Traits](#traits)
  - [`FatArch`](#fatarch)
- [Type Aliases](#type-aliases)
  - [`MachOFatFile32`](#machofatfile32)
  - [`MachOFatFile64`](#machofatfile64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FatArch32`](#fatarch32) | struct |  |
| [`FatArch64`](#fatarch64) | struct |  |
| [`FatHeader`](#fatheader) | struct |  |
| [`MachOFatFile`](#machofatfile) | struct | A Mach-O universal binary. |
| [`FatArch`](#fatarch) | trait | A trait for generic access to [`macho::FatArch32`] and [`macho::FatArch64`]. |
| [`MachOFatFile32`](#machofatfile32) | type | A 32-bit Mach-O universal binary. |
| [`MachOFatFile64`](#machofatfile64) | type | A 64-bit Mach-O universal binary. |

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

*Defined in [`object-0.37.3/src/macho.rs:717-728`](../../../../../.source_1765210505/object-0.37.3/src/macho.rs#L717-L728)*

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

- <span id="fatarch32-clone"></span>`fn clone(&self) -> FatArch32` — [`FatArch32`](../../../macho/index.md#fatarch32)

##### `impl Copy for FatArch32`

##### `impl Debug for FatArch32`

- <span id="fatarch32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FatArch for FatArch32`

- <span id="fatarch32-fatarch-type-word"></span>`type Word = u32`

- <span id="fatarch32-fatarch-const-magic"></span>`const MAGIC: u32`

- <span id="fatarch32-cputype"></span>`fn cputype(&self) -> u32`

- <span id="fatarch32-cpusubtype"></span>`fn cpusubtype(&self) -> u32`

- <span id="fatarch32-offset"></span>`fn offset(&self) -> <Self as >::Word` — [`FatArch`](../index.md#fatarch)

- <span id="fatarch32-size"></span>`fn size(&self) -> <Self as >::Word` — [`FatArch`](../index.md#fatarch)

- <span id="fatarch32-align"></span>`fn align(&self) -> u32`

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

*Defined in [`object-0.37.3/src/macho.rs:743-756`](../../../../../.source_1765210505/object-0.37.3/src/macho.rs#L743-L756)*

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

- <span id="fatarch64-clone"></span>`fn clone(&self) -> FatArch64` — [`FatArch64`](../../../macho/index.md#fatarch64)

##### `impl Copy for FatArch64`

##### `impl Debug for FatArch64`

- <span id="fatarch64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FatArch for FatArch64`

- <span id="fatarch64-fatarch-type-word"></span>`type Word = u64`

- <span id="fatarch64-fatarch-const-magic"></span>`const MAGIC: u32`

- <span id="fatarch64-cputype"></span>`fn cputype(&self) -> u32`

- <span id="fatarch64-cpusubtype"></span>`fn cpusubtype(&self) -> u32`

- <span id="fatarch64-offset"></span>`fn offset(&self) -> <Self as >::Word` — [`FatArch`](../index.md#fatarch)

- <span id="fatarch64-size"></span>`fn size(&self) -> <Self as >::Word` — [`FatArch`](../index.md#fatarch)

- <span id="fatarch64-align"></span>`fn align(&self) -> u32`

##### `impl Pod for FatArch64`

### `FatHeader`

```rust
struct FatHeader {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub nfat_arch: crate::endian::U32<crate::endian::BigEndian>,
}
```

*Defined in [`object-0.37.3/src/macho.rs:708-713`](../../../../../.source_1765210505/object-0.37.3/src/macho.rs#L708-L713)*

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  FAT_MAGIC or FAT_MAGIC_64

- **`nfat_arch`**: `crate::endian::U32<crate::endian::BigEndian>`

  number of structs that follow

#### Trait Implementations

##### `impl Clone for FatHeader`

- <span id="fatheader-clone"></span>`fn clone(&self) -> FatHeader` — [`FatHeader`](../../../macho/index.md#fatheader)

##### `impl Copy for FatHeader`

##### `impl Debug for FatHeader`

- <span id="fatheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pod for FatHeader`

### `MachOFatFile<'data, Fat: FatArch>`

```rust
struct MachOFatFile<'data, Fat: FatArch> {
    header: &'data macho::FatHeader,
    arches: &'data [Fat],
}
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:25-28`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L25-L28)*

A Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat32`](../../../index.md) or [`crate::FileKind::MachOFat64`](../../../index.md).

#### Implementations

- <span id="machofatfile-parse"></span>`fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

- <span id="machofatfile-header"></span>`fn header(&self) -> &'data macho::FatHeader` — [`FatHeader`](../../../macho/index.md#fatheader)

- <span id="machofatfile-arches"></span>`fn arches(&self) -> &'data [Fat]`

#### Trait Implementations

##### `impl<Fat: clone::Clone + FatArch> Clone for MachOFatFile<'data, Fat>`

- <span id="machofatfile-clone"></span>`fn clone(&self) -> MachOFatFile<'data, Fat>` — [`MachOFatFile`](../index.md#machofatfile)

##### `impl<Fat: fmt::Debug + FatArch> Debug for MachOFatFile<'data, Fat>`

- <span id="machofatfile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `FatArch`

```rust
trait FatArch: Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:59-90`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L59-L90)*

A trait for generic access to [`macho::FatArch32`](../../../macho/index.md) and [`macho::FatArch64`](../../../macho/index.md).

#### Associated Types

- `type Word: 1`

#### Associated Constants

- `const MAGIC: u32`

#### Required Methods

- `fn cputype(&self) -> u32`

- `fn cpusubtype(&self) -> u32`

- `fn offset(&self) -> <Self as >::Word`

- `fn size(&self) -> <Self as >::Word`

- `fn align(&self) -> u32`

#### Provided Methods

- `fn architecture(&self) -> Architecture`

- `fn file_range(&self) -> (u64, u64)`

- `fn data<'data, R: ReadRef<'data>>(&self, file: R) -> Result<&'data [u8]>`

#### Implementors

- [`FatArch32`](../../../macho/index.md#fatarch32)
- [`FatArch64`](../../../macho/index.md#fatarch64)

## Type Aliases

### `MachOFatFile32<'data>`

```rust
type MachOFatFile32<'data> = MachOFatFile<'data, macho::FatArch32>;
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:12`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L12)*

A 32-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat32`](../../../index.md).

### `MachOFatFile64<'data>`

```rust
type MachOFatFile64<'data> = MachOFatFile<'data, macho::FatArch64>;
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:18`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L18)*

A 64-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat64`](../../../index.md).

