*[object](../index.md) / [elf](index.md)*

---

# Module `elf`

ELF definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is the equivalent of /usr/include/elf.h, and is based heavily on it.

## Structs

### `FileHeader32<E: Endian>`

```rust
struct FileHeader32<E: Endian> {
    pub e_ident: Ident,
    pub e_type: crate::endian::U16<E>,
    pub e_machine: crate::endian::U16<E>,
    pub e_version: crate::endian::U32<E>,
    pub e_entry: crate::endian::U32<E>,
    pub e_phoff: crate::endian::U32<E>,
    pub e_shoff: crate::endian::U32<E>,
    pub e_flags: crate::endian::U32<E>,
    pub e_ehsize: crate::endian::U16<E>,
    pub e_phentsize: crate::endian::U16<E>,
    pub e_phnum: crate::endian::U16<E>,
    pub e_shentsize: crate::endian::U16<E>,
    pub e_shnum: crate::endian::U16<E>,
    pub e_shstrndx: crate::endian::U16<E>,
}
```

The header at the start of every 32-bit ELF file.

#### Fields

- **`e_ident`**: `Ident`

  Magic number and other information.

- **`e_type`**: `crate::endian::U16<E>`

  Object file type. One of the `ET_*` constants.

- **`e_machine`**: `crate::endian::U16<E>`

  Architecture. One of the `EM_*` constants.

- **`e_version`**: `crate::endian::U32<E>`

  Object file version. Must be `EV_CURRENT`.

- **`e_entry`**: `crate::endian::U32<E>`

  Entry point virtual address.

- **`e_phoff`**: `crate::endian::U32<E>`

  Program header table file offset.

- **`e_shoff`**: `crate::endian::U32<E>`

  Section header table file offset.

- **`e_flags`**: `crate::endian::U32<E>`

  Processor-specific flags.
  
  A combination of the `EF_*` constants.

- **`e_ehsize`**: `crate::endian::U16<E>`

  Size in bytes of this header.

- **`e_phentsize`**: `crate::endian::U16<E>`

  Program header table entry size.

- **`e_phnum`**: `crate::endian::U16<E>`

  Program header table entry count.
  
  If the count is greater than or equal to `PN_XNUM` then this field is set to
  `PN_XNUM` and the count is stored in the `sh_info` field of section 0.

- **`e_shentsize`**: `crate::endian::U16<E>`

  Section header table entry size.

- **`e_shnum`**: `crate::endian::U16<E>`

  Section header table entry count.
  
  If the count is greater than or equal to `SHN_LORESERVE` then this field is set to
  `0` and the count is stored in the `sh_size` field of section 0.
  first section header.

- **`e_shstrndx`**: `crate::endian::U16<E>`

  Section header string table index.
  
  If the index is greater than or equal to `SHN_LORESERVE` then this field is set to
  `SHN_XINDEX` and the index is stored in the `sh_link` field of section 0.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> FileHeader32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl FileHeader<Endian: endian::Endian>`

- `type Word = u32`

- `type Sword = i32`

- `type Endian = Endian`

- `type ProgramHeader = ProgramHeader32<Endian>`

- `type SectionHeader = SectionHeader32<Endian>`

- `type CompressionHeader = CompressionHeader32<Endian>`

- `type NoteHeader = NoteHeader32<Endian>`

- `type Dyn = Dyn32<Endian>`

- `type Sym = Sym32<Endian>`

- `type Rel = Rel32<Endian>`

- `type Rela = Rela32<Endian>`

- `type Relr = Relr32<Endian>`

- `fn is_type_64(self: &Self) -> bool`

- `fn is_type_64_sized() -> bool`

- `fn e_ident(self: &Self) -> &elf::Ident`

- `fn e_type(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_machine(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_version(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_entry(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_phoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_shoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_ehsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shstrndx(self: &Self, endian: <Self as >::Endian) -> u16`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FileHeader64<E: Endian>`

```rust
struct FileHeader64<E: Endian> {
    pub e_ident: Ident,
    pub e_type: crate::endian::U16<E>,
    pub e_machine: crate::endian::U16<E>,
    pub e_version: crate::endian::U32<E>,
    pub e_entry: crate::endian::U64<E>,
    pub e_phoff: crate::endian::U64<E>,
    pub e_shoff: crate::endian::U64<E>,
    pub e_flags: crate::endian::U32<E>,
    pub e_ehsize: crate::endian::U16<E>,
    pub e_phentsize: crate::endian::U16<E>,
    pub e_phnum: crate::endian::U16<E>,
    pub e_shentsize: crate::endian::U16<E>,
    pub e_shnum: crate::endian::U16<E>,
    pub e_shstrndx: crate::endian::U16<E>,
}
```

The header at the start of every 64-bit ELF file.

#### Fields

- **`e_ident`**: `Ident`

  Magic number and other information.

- **`e_type`**: `crate::endian::U16<E>`

  Object file type. One of the `ET_*` constants.

- **`e_machine`**: `crate::endian::U16<E>`

  Architecture. One of the `EM_*` constants.

- **`e_version`**: `crate::endian::U32<E>`

  Object file version. Must be `EV_CURRENT`.

- **`e_entry`**: `crate::endian::U64<E>`

  Entry point virtual address.

- **`e_phoff`**: `crate::endian::U64<E>`

  Program header table file offset.

- **`e_shoff`**: `crate::endian::U64<E>`

  Section header table file offset.

- **`e_flags`**: `crate::endian::U32<E>`

  Processor-specific flags.
  
  A combination of the `EF_*` constants.

- **`e_ehsize`**: `crate::endian::U16<E>`

  Size in bytes of this header.

- **`e_phentsize`**: `crate::endian::U16<E>`

  Program header table entry size.

- **`e_phnum`**: `crate::endian::U16<E>`

  Program header table entry count.
  
  If the count is greater than or equal to `PN_XNUM` then this field is set to
  `PN_XNUM` and the count is stored in the `sh_info` field of section 0.

- **`e_shentsize`**: `crate::endian::U16<E>`

  Section header table entry size.

- **`e_shnum`**: `crate::endian::U16<E>`

  Section header table entry count.
  
  If the count is greater than or equal to `SHN_LORESERVE` then this field is set to
  `0` and the count is stored in the `sh_size` field of section 0.
  first section header.

- **`e_shstrndx`**: `crate::endian::U16<E>`

  Section header string table index.
  
  If the index is greater than or equal to `SHN_LORESERVE` then this field is set to
  `SHN_XINDEX` and the index is stored in the `sh_link` field of section 0.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> FileHeader64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl FileHeader<Endian: endian::Endian>`

- `type Word = u64`

- `type Sword = i64`

- `type Endian = Endian`

- `type ProgramHeader = ProgramHeader64<Endian>`

- `type SectionHeader = SectionHeader64<Endian>`

- `type CompressionHeader = CompressionHeader64<Endian>`

- `type NoteHeader = NoteHeader32<Endian>`

- `type Dyn = Dyn64<Endian>`

- `type Sym = Sym64<Endian>`

- `type Rel = Rel64<Endian>`

- `type Rela = Rela64<Endian>`

- `type Relr = Relr64<Endian>`

- `fn is_type_64(self: &Self) -> bool`

- `fn is_type_64_sized() -> bool`

- `fn e_ident(self: &Self) -> &elf::Ident`

- `fn e_type(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_machine(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_version(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_entry(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_phoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_shoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_ehsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shstrndx(self: &Self, endian: <Self as >::Endian) -> u16`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Ident`

```rust
struct Ident {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
}
```

Magic number and other information.

Contained in the file header.

#### Fields

- **`magic`**: `[u8; 4]`

  Magic number. Must be `ELFMAG`.

- **`class`**: `u8`

  File class. One of the `ELFCLASS*` constants.

- **`data`**: `u8`

  Data encoding. One of the `ELFDATA*` constants.

- **`version`**: `u8`

  ELF version. Must be `EV_CURRENT`.

- **`os_abi`**: `u8`

  OS ABI identification. One of the `ELFOSABI*` constants.

- **`abi_version`**: `u8`

  ABI version.
  
  The meaning of this field depends on the `os_abi` value.

- **`padding`**: `[u8; 7]`

  Padding bytes.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Ident`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SectionHeader32<E: Endian>`

```rust
struct SectionHeader32<E: Endian> {
    pub sh_name: crate::endian::U32<E>,
    pub sh_type: crate::endian::U32<E>,
    pub sh_flags: crate::endian::U32<E>,
    pub sh_addr: crate::endian::U32<E>,
    pub sh_offset: crate::endian::U32<E>,
    pub sh_size: crate::endian::U32<E>,
    pub sh_link: crate::endian::U32<E>,
    pub sh_info: crate::endian::U32<E>,
    pub sh_addralign: crate::endian::U32<E>,
    pub sh_entsize: crate::endian::U32<E>,
}
```

Section header.

#### Fields

- **`sh_name`**: `crate::endian::U32<E>`

  Section name.
  
  This is an offset into the section header string table.

- **`sh_type`**: `crate::endian::U32<E>`

  Section type. One of the `SHT_*` constants.

- **`sh_flags`**: `crate::endian::U32<E>`

  Section flags. A combination of the `SHF_*` constants.

- **`sh_addr`**: `crate::endian::U32<E>`

  Section virtual address at execution.

- **`sh_offset`**: `crate::endian::U32<E>`

  Section file offset.

- **`sh_size`**: `crate::endian::U32<E>`

  Section size in bytes.

- **`sh_link`**: `crate::endian::U32<E>`

  Link to another section.
  
  The section relationship depends on the `sh_type` value.

- **`sh_info`**: `crate::endian::U32<E>`

  Additional section information.
  
  The meaning of this field depends on the `sh_type` value.

- **`sh_addralign`**: `crate::endian::U32<E>`

  Section alignment.

- **`sh_entsize`**: `crate::endian::U32<E>`

  Entry size if the section holds a table.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> SectionHeader32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl SectionHeader<Endian: endian::Endian>`

- `type Elf = FileHeader32<Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `fn sh_name(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_flags(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_link(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_info(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_entsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

### `SectionHeader64<E: Endian>`

```rust
struct SectionHeader64<E: Endian> {
    pub sh_name: crate::endian::U32<E>,
    pub sh_type: crate::endian::U32<E>,
    pub sh_flags: crate::endian::U64<E>,
    pub sh_addr: crate::endian::U64<E>,
    pub sh_offset: crate::endian::U64<E>,
    pub sh_size: crate::endian::U64<E>,
    pub sh_link: crate::endian::U32<E>,
    pub sh_info: crate::endian::U32<E>,
    pub sh_addralign: crate::endian::U64<E>,
    pub sh_entsize: crate::endian::U64<E>,
}
```

Section header.

#### Fields

- **`sh_name`**: `crate::endian::U32<E>`

  Section name.
  
  This is an offset into the section header string table.

- **`sh_type`**: `crate::endian::U32<E>`

  Section type. One of the `SHT_*` constants.

- **`sh_flags`**: `crate::endian::U64<E>`

  Section flags. A combination of the `SHF_*` constants.

- **`sh_addr`**: `crate::endian::U64<E>`

  Section virtual address at execution.

- **`sh_offset`**: `crate::endian::U64<E>`

  Section file offset.

- **`sh_size`**: `crate::endian::U64<E>`

  Section size in bytes.

- **`sh_link`**: `crate::endian::U32<E>`

  Link to another section.
  
  The section relationship depends on the `sh_type` value.

- **`sh_info`**: `crate::endian::U32<E>`

  Additional section information.
  
  The meaning of this field depends on the `sh_type` value.

- **`sh_addralign`**: `crate::endian::U64<E>`

  Section alignment.

- **`sh_entsize`**: `crate::endian::U64<E>`

  Entry size if the section holds a table.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> SectionHeader64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl SectionHeader<Endian: endian::Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `type Elf = FileHeader64<Endian>`

- `fn sh_name(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_flags(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_link(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_info(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_entsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

### `CompressionHeader32<E: Endian>`

```rust
struct CompressionHeader32<E: Endian> {
    pub ch_type: crate::endian::U32Bytes<E>,
    pub ch_size: crate::endian::U32Bytes<E>,
    pub ch_addralign: crate::endian::U32Bytes<E>,
}
```

Section compression header.

Used when `SHF_COMPRESSED` is set.

Note: this type currently allows for misaligned headers, but that may be
changed in a future version.

#### Fields

- **`ch_type`**: `crate::endian::U32Bytes<E>`

  Compression format. One of the `ELFCOMPRESS_*` values.

- **`ch_size`**: `crate::endian::U32Bytes<E>`

  Uncompressed data size.

- **`ch_addralign`**: `crate::endian::U32Bytes<E>`

  Uncompressed data alignment.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> CompressionHeader32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl CompressionHeader<Endian: endian::Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `fn ch_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn ch_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn ch_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> CompressionHeader32<E>`

### `CompressionHeader64<E: Endian>`

```rust
struct CompressionHeader64<E: Endian> {
    pub ch_type: crate::endian::U32Bytes<E>,
    pub ch_reserved: crate::endian::U32Bytes<E>,
    pub ch_size: crate::endian::U64Bytes<E>,
    pub ch_addralign: crate::endian::U64Bytes<E>,
}
```

Section compression header.

Used when `SHF_COMPRESSED` is set.

Note: this type currently allows for misaligned headers, but that may be
changed in a future version.

#### Fields

- **`ch_type`**: `crate::endian::U32Bytes<E>`

  Compression format. One of the `ELFCOMPRESS_*` values.

- **`ch_reserved`**: `crate::endian::U32Bytes<E>`

  Reserved.

- **`ch_size`**: `crate::endian::U64Bytes<E>`

  Uncompressed data size.

- **`ch_addralign`**: `crate::endian::U64Bytes<E>`

  Uncompressed data alignment.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> CompressionHeader64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl CompressionHeader<Endian: endian::Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `fn ch_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn ch_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn ch_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> CompressionHeader64<E>`

### `Sym32<E: Endian>`

```rust
struct Sym32<E: Endian> {
    pub st_name: crate::endian::U32<E>,
    pub st_value: crate::endian::U32<E>,
    pub st_size: crate::endian::U32<E>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: crate::endian::U16<E>,
}
```

Symbol table entry.

#### Fields

- **`st_name`**: `crate::endian::U32<E>`

  Symbol name.
  
  This is an offset into the symbol string table.

- **`st_value`**: `crate::endian::U32<E>`

  Symbol value.

- **`st_size`**: `crate::endian::U32<E>`

  Symbol size.

- **`st_info`**: `u8`

  Symbol type and binding.
  
  Use the `st_type` and `st_bind` methods to access this value.

- **`st_other`**: `u8`

  Symbol visibility.
  
  Use the `st_visibility` method to access this value.

- **`st_shndx`**: `crate::endian::U16<E>`

  Section index or one of the `SHN_*` values.

#### Implementations

- `fn st_bind(self: &Self) -> u8`
  Get the `st_bind` component of the `st_info` field.

- `fn st_type(self: &Self) -> u8`
  Get the `st_type` component of the `st_info` field.

- `fn set_st_info(self: &mut Self, st_bind: u8, st_type: u8)`
  Set the `st_info` field given the `st_bind` and `st_type` components.

- `fn st_visibility(self: &Self) -> u8`
  Get the `st_visibility` component of the `st_info` field.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Sym32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Sym<Endian: endian::Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `fn st_name(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn st_info(self: &Self) -> u8`

- `fn st_bind(self: &Self) -> u8`

- `fn st_type(self: &Self) -> u8`

- `fn st_other(self: &Self) -> u8`

- `fn st_visibility(self: &Self) -> u8`

- `fn st_shndx(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn st_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn st_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> Sym32<E>`

### `Sym64<E: Endian>`

```rust
struct Sym64<E: Endian> {
    pub st_name: crate::endian::U32<E>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: crate::endian::U16<E>,
    pub st_value: crate::endian::U64<E>,
    pub st_size: crate::endian::U64<E>,
}
```

Symbol table entry.

#### Fields

- **`st_name`**: `crate::endian::U32<E>`

  Symbol name.
  
  This is an offset into the symbol string table.

- **`st_info`**: `u8`

  Symbol type and binding.
  
  Use the `st_bind` and `st_type` methods to access this value.

- **`st_other`**: `u8`

  Symbol visibility.
  
  Use the `st_visibility` method to access this value.

- **`st_shndx`**: `crate::endian::U16<E>`

  Section index or one of the `SHN_*` values.

- **`st_value`**: `crate::endian::U64<E>`

  Symbol value.

- **`st_size`**: `crate::endian::U64<E>`

  Symbol size.

#### Implementations

- `fn st_bind(self: &Self) -> u8`
  Get the `st_bind` component of the `st_info` field.

- `fn st_type(self: &Self) -> u8`
  Get the `st_type` component of the `st_info` field.

- `fn set_st_info(self: &mut Self, st_bind: u8, st_type: u8)`
  Set the `st_info` field given the `st_bind` and `st_type` components.

- `fn st_visibility(self: &Self) -> u8`
  Get the `st_visibility` component of the `st_info` field.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Sym64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Sym<Endian: endian::Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `fn st_name(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn st_info(self: &Self) -> u8`

- `fn st_bind(self: &Self) -> u8`

- `fn st_type(self: &Self) -> u8`

- `fn st_other(self: &Self) -> u8`

- `fn st_visibility(self: &Self) -> u8`

- `fn st_shndx(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn st_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn st_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> Sym64<E>`

### `Syminfo32<E: Endian>`

```rust
struct Syminfo32<E: Endian> {
    pub si_boundto: crate::endian::U16<E>,
    pub si_flags: crate::endian::U16<E>,
}
```

Additional information about a `Sym32`.

#### Fields

- **`si_boundto`**: `crate::endian::U16<E>`

  Direct bindings, symbol bound to.

- **`si_flags`**: `crate::endian::U16<E>`

  Per symbol flags.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Syminfo32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Syminfo64<E: Endian>`

```rust
struct Syminfo64<E: Endian> {
    pub si_boundto: crate::endian::U16<E>,
    pub si_flags: crate::endian::U16<E>,
}
```

Additional information about a `Sym64`.

#### Fields

- **`si_boundto`**: `crate::endian::U16<E>`

  Direct bindings, symbol bound to.

- **`si_flags`**: `crate::endian::U16<E>`

  Per symbol flags.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Syminfo64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Rel32<E: Endian>`

```rust
struct Rel32<E: Endian> {
    pub r_offset: crate::endian::U32<E>,
    pub r_info: crate::endian::U32<E>,
}
```

Relocation table entry without explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U32<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U32<E>`

  Relocation type and symbol index.

#### Implementations

- `fn r_sym(self: &Self, endian: E) -> u32`
  Get the `r_sym` component of the `r_info` field.

- `fn r_type(self: &Self, endian: E) -> u32`
  Get the `r_type` component of the `r_info` field.

- `fn r_info(endian: E, r_sym: u32, r_type: u8) -> U32<E>`
  Calculate the `r_info` field given the `r_sym` and `r_type` components.

- `fn set_r_info(self: &mut Self, endian: E, r_sym: u32, r_type: u8)`
  Set the `r_info` field given the `r_sym` and `r_type` components.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Rel32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Rel<Endian: endian::Endian>`

- `type Word = u32`

- `type Sword = i32`

- `type Endian = Endian`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_sym(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian) -> u32`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Rela32<E: Endian>`

```rust
struct Rela32<E: Endian> {
    pub r_offset: crate::endian::U32<E>,
    pub r_info: crate::endian::U32<E>,
    pub r_addend: crate::endian::I32<E>,
}
```

Relocation table entry with explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U32<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U32<E>`

  Relocation type and symbol index.

- **`r_addend`**: `crate::endian::I32<E>`

  Explicit addend.

#### Implementations

- `fn r_sym(self: &Self, endian: E) -> u32`
  Get the `r_sym` component of the `r_info` field.

- `fn r_type(self: &Self, endian: E) -> u32`
  Get the `r_type` component of the `r_info` field.

- `fn r_info(endian: E, r_sym: u32, r_type: u8) -> U32<E>`
  Calculate the `r_info` field given the `r_sym` and `r_type` components.

- `fn set_r_info(self: &mut Self, endian: E, r_sym: u32, r_type: u8)`
  Set the `r_info` field given the `r_sym` and `r_type` components.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<E: Endian>`

- `fn from(rel: Rel32<E>) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Rela32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Rela<Endian: endian::Endian>`

- `type Word = u32`

- `type Sword = i32`

- `type Endian = Endian`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian, _is_mips64el: bool) -> <Self as >::Word`

- `fn r_addend(self: &Self, endian: <Self as >::Endian) -> <Self as >::Sword`

- `fn r_sym(self: &Self, endian: <Self as >::Endian, _is_mips64el: bool) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian, _is_mips64el: bool) -> u32`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Rel64<E: Endian>`

```rust
struct Rel64<E: Endian> {
    pub r_offset: crate::endian::U64<E>,
    pub r_info: crate::endian::U64<E>,
}
```

Relocation table entry without explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U64<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U64<E>`

  Relocation type and symbol index.

#### Implementations

- `fn r_sym(self: &Self, endian: E) -> u32`
  Get the `r_sym` component of the `r_info` field.

- `fn r_type(self: &Self, endian: E) -> u32`
  Get the `r_type` component of the `r_info` field.

- `fn r_info(endian: E, r_sym: u32, r_type: u32) -> U64<E>`
  Calculate the `r_info` field given the `r_sym` and `r_type` components.

- `fn set_r_info(self: &mut Self, endian: E, r_sym: u32, r_type: u32)`
  Set the `r_info` field given the `r_sym` and `r_type` components.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Rel64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Rel<Endian: endian::Endian>`

- `type Word = u64`

- `type Sword = i64`

- `type Endian = Endian`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_sym(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian) -> u32`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Rela64<E: Endian>`

```rust
struct Rela64<E: Endian> {
    pub r_offset: crate::endian::U64<E>,
    pub r_info: crate::endian::U64<E>,
    pub r_addend: crate::endian::I64<E>,
}
```

Relocation table entry with explicit addend.

#### Fields

- **`r_offset`**: `crate::endian::U64<E>`

  Relocation address.

- **`r_info`**: `crate::endian::U64<E>`

  Relocation type and symbol index.

- **`r_addend`**: `crate::endian::I64<E>`

  Explicit addend.

#### Implementations

- `fn r_sym(self: &Self, endian: E, is_mips64el: bool) -> u32`
  Get the `r_sym` component of the `r_info` field.

- `fn r_type(self: &Self, endian: E, is_mips64el: bool) -> u32`
  Get the `r_type` component of the `r_info` field.

- `fn r_info(endian: E, is_mips64el: bool, r_sym: u32, r_type: u32) -> U64<E>`
  Calculate the `r_info` field given the `r_sym` and `r_type` components.

- `fn set_r_info(self: &mut Self, endian: E, is_mips64el: bool, r_sym: u32, r_type: u32)`
  Set the `r_info` field given the `r_sym` and `r_type` components.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<E: Endian>`

- `fn from(rel: Rel64<E>) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Rela64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Rela<Endian: endian::Endian>`

- `type Word = u64`

- `type Sword = i64`

- `type Endian = Endian`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word`

- `fn r_addend(self: &Self, endian: <Self as >::Endian) -> <Self as >::Sword`

- `fn r_sym(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Relr32<E: Endian>`

```rust
struct Relr32<E: Endian>(crate::endian::U32<E>);
```

32-bit relative relocation table entry.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Relr32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Relr<Endian: endian::Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `const COUNT: u8`

- `fn get(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Relr64<E: Endian>`

```rust
struct Relr64<E: Endian>(crate::endian::U64<E>);
```

64-bit relative relocation table entry.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Relr64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl Relr<Endian: endian::Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `const COUNT: u8`

- `fn get(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ProgramHeader32<E: Endian>`

```rust
struct ProgramHeader32<E: Endian> {
    pub p_type: crate::endian::U32<E>,
    pub p_offset: crate::endian::U32<E>,
    pub p_vaddr: crate::endian::U32<E>,
    pub p_paddr: crate::endian::U32<E>,
    pub p_filesz: crate::endian::U32<E>,
    pub p_memsz: crate::endian::U32<E>,
    pub p_flags: crate::endian::U32<E>,
    pub p_align: crate::endian::U32<E>,
}
```

Program segment header.

#### Fields

- **`p_type`**: `crate::endian::U32<E>`

  Segment type. One of the `PT_*` constants.

- **`p_offset`**: `crate::endian::U32<E>`

  Segment file offset.

- **`p_vaddr`**: `crate::endian::U32<E>`

  Segment virtual address.

- **`p_paddr`**: `crate::endian::U32<E>`

  Segment physical address.

- **`p_filesz`**: `crate::endian::U32<E>`

  Segment size in the file.

- **`p_memsz`**: `crate::endian::U32<E>`

  Segment size in memory.

- **`p_flags`**: `crate::endian::U32<E>`

  Segment flags. A combination of the `PF_*` constants.

- **`p_align`**: `crate::endian::U32<E>`

  Segment alignment.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> ProgramHeader32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ProgramHeader<Endian: endian::Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `type Elf = FileHeader32<Endian>`

- `fn p_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_vaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_paddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_filesz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_memsz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_align(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ProgramHeader64<E: Endian>`

```rust
struct ProgramHeader64<E: Endian> {
    pub p_type: crate::endian::U32<E>,
    pub p_flags: crate::endian::U32<E>,
    pub p_offset: crate::endian::U64<E>,
    pub p_vaddr: crate::endian::U64<E>,
    pub p_paddr: crate::endian::U64<E>,
    pub p_filesz: crate::endian::U64<E>,
    pub p_memsz: crate::endian::U64<E>,
    pub p_align: crate::endian::U64<E>,
}
```

Program segment header.

#### Fields

- **`p_type`**: `crate::endian::U32<E>`

  Segment type. One of the `PT_*` constants.

- **`p_flags`**: `crate::endian::U32<E>`

  Segment flags. A combination of the `PF_*` constants.

- **`p_offset`**: `crate::endian::U64<E>`

  Segment file offset.

- **`p_vaddr`**: `crate::endian::U64<E>`

  Segment virtual address.

- **`p_paddr`**: `crate::endian::U64<E>`

  Segment physical address.

- **`p_filesz`**: `crate::endian::U64<E>`

  Segment size in the file.

- **`p_memsz`**: `crate::endian::U64<E>`

  Segment size in memory.

- **`p_align`**: `crate::endian::U64<E>`

  Segment alignment.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> ProgramHeader64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ProgramHeader<Endian: endian::Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `type Elf = FileHeader64<Endian>`

- `fn p_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_vaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_paddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_filesz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_memsz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_align(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Dyn32<E: Endian>`

```rust
struct Dyn32<E: Endian> {
    pub d_tag: crate::endian::U32<E>,
    pub d_val: crate::endian::U32<E>,
}
```

Dynamic section entry.

#### Fields

- **`d_tag`**: `crate::endian::U32<E>`

  Dynamic entry type.

- **`d_val`**: `crate::endian::U32<E>`

  Value (integer or address).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Dyn32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Dyn<Endian: endian::Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `fn d_tag(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn d_val(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Dyn64<E: Endian>`

```rust
struct Dyn64<E: Endian> {
    pub d_tag: crate::endian::U64<E>,
    pub d_val: crate::endian::U64<E>,
}
```

Dynamic section entry.

#### Fields

- **`d_tag`**: `crate::endian::U64<E>`

  Dynamic entry type.

- **`d_val`**: `crate::endian::U64<E>`

  Value (integer or address).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Dyn64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Dyn<Endian: endian::Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `fn d_tag(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn d_val(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Versym<E: Endian>`

```rust
struct Versym<E: Endian>(crate::endian::U16<E>);
```

Version symbol information

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Versym<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Verdef<E: Endian>`

```rust
struct Verdef<E: Endian> {
    pub vd_version: crate::endian::U16<E>,
    pub vd_flags: crate::endian::U16<E>,
    pub vd_ndx: crate::endian::U16<E>,
    pub vd_cnt: crate::endian::U16<E>,
    pub vd_hash: crate::endian::U32<E>,
    pub vd_aux: crate::endian::U32<E>,
    pub vd_next: crate::endian::U32<E>,
}
```

Version definition sections

#### Fields

- **`vd_version`**: `crate::endian::U16<E>`

  Version revision

- **`vd_flags`**: `crate::endian::U16<E>`

  Version information

- **`vd_ndx`**: `crate::endian::U16<E>`

  Version Index

- **`vd_cnt`**: `crate::endian::U16<E>`

  Number of associated aux entries

- **`vd_hash`**: `crate::endian::U32<E>`

  Version name hash value

- **`vd_aux`**: `crate::endian::U32<E>`

  Offset in bytes to verdaux array

- **`vd_next`**: `crate::endian::U32<E>`

  Offset in bytes to next verdef entry

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Verdef<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Verdaux<E: Endian>`

```rust
struct Verdaux<E: Endian> {
    pub vda_name: crate::endian::U32<E>,
    pub vda_next: crate::endian::U32<E>,
}
```

Auxiliary version information.

#### Fields

- **`vda_name`**: `crate::endian::U32<E>`

  Version or dependency names

- **`vda_next`**: `crate::endian::U32<E>`

  Offset in bytes to next verdaux

#### Implementations

- `fn name<'data, R: ReadRef<'data>>(self: &Self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>`
  Parse the version name from the string table.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Verdaux<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Verneed<E: Endian>`

```rust
struct Verneed<E: Endian> {
    pub vn_version: crate::endian::U16<E>,
    pub vn_cnt: crate::endian::U16<E>,
    pub vn_file: crate::endian::U32<E>,
    pub vn_aux: crate::endian::U32<E>,
    pub vn_next: crate::endian::U32<E>,
}
```

Version dependency.

#### Fields

- **`vn_version`**: `crate::endian::U16<E>`

  Version of structure

- **`vn_cnt`**: `crate::endian::U16<E>`

  Number of associated aux entries

- **`vn_file`**: `crate::endian::U32<E>`

  Offset of filename for this dependency

- **`vn_aux`**: `crate::endian::U32<E>`

  Offset in bytes to vernaux array

- **`vn_next`**: `crate::endian::U32<E>`

  Offset in bytes to next verneed entry

#### Implementations

- `fn file<'data, R: ReadRef<'data>>(self: &Self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>`
  Parse the file from the string table.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Verneed<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Vernaux<E: Endian>`

```rust
struct Vernaux<E: Endian> {
    pub vna_hash: crate::endian::U32<E>,
    pub vna_flags: crate::endian::U16<E>,
    pub vna_other: crate::endian::U16<E>,
    pub vna_name: crate::endian::U32<E>,
    pub vna_next: crate::endian::U32<E>,
}
```

Auxiliary needed version information.

#### Fields

- **`vna_hash`**: `crate::endian::U32<E>`

  Hash value of dependency name

- **`vna_flags`**: `crate::endian::U16<E>`

  Dependency specific information

- **`vna_other`**: `crate::endian::U16<E>`

  Version Index

- **`vna_name`**: `crate::endian::U32<E>`

  Dependency name string offset

- **`vna_next`**: `crate::endian::U32<E>`

  Offset in bytes to next vernaux entry

#### Implementations

- `fn name<'data, R: ReadRef<'data>>(self: &Self, endian: Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>`
  Parse the version name from the string table.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> Vernaux<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `NoteHeader32<E: Endian>`

```rust
struct NoteHeader32<E: Endian> {
    pub n_namesz: crate::endian::U32<E>,
    pub n_descsz: crate::endian::U32<E>,
    pub n_type: crate::endian::U32<E>,
}
```

Note section entry header.

A note consists of a header followed by a variable length name and descriptor.

#### Fields

- **`n_namesz`**: `crate::endian::U32<E>`

  Length of the note's name.
  
  Some known names are defined by the `ELF_NOTE_*` constants.

- **`n_descsz`**: `crate::endian::U32<E>`

  Length of the note's descriptor.
  
  The content of the descriptor depends on the note name and type.

- **`n_type`**: `crate::endian::U32<E>`

  Type of the note.
  
  One of the `NT_*` constants. The note name determines which
  `NT_*` constants are valid.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> NoteHeader32<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl NoteHeader<Endian: endian::Endian>`

- `type Endian = Endian`

- `fn n_namesz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_type(self: &Self, endian: <Self as >::Endian) -> u32`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `NoteHeader64<E: Endian>`

```rust
struct NoteHeader64<E: Endian> {
    pub n_namesz: crate::endian::U32<E>,
    pub n_descsz: crate::endian::U32<E>,
    pub n_type: crate::endian::U32<E>,
}
```

Note section entry header.

#### Fields

- **`n_namesz`**: `crate::endian::U32<E>`

  Length of the note's name.
  
  Some known names are defined by the `ELF_NOTE_*` constants.

- **`n_descsz`**: `crate::endian::U32<E>`

  Length of the note's descriptor.
  
  The content of the descriptor depends on the note name and type.

- **`n_type`**: `crate::endian::U32<E>`

  Type of the note.
  
  One of the `NT_*` constants. The note name determines which
  `NT_*` constants are valid.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> NoteHeader64<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl NoteHeader<Endian: endian::Endian>`

- `type Endian = Endian`

- `fn n_namesz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_type(self: &Self, endian: <Self as >::Endian) -> u32`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HashHeader<E: Endian>`

```rust
struct HashHeader<E: Endian> {
    pub bucket_count: crate::endian::U32<E>,
    pub chain_count: crate::endian::U32<E>,
}
```

Header of `SHT_HASH` section.

#### Fields

- **`bucket_count`**: `crate::endian::U32<E>`

  The number of hash buckets.

- **`chain_count`**: `crate::endian::U32<E>`

  The number of chain values.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> HashHeader<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GnuHashHeader<E: Endian>`

```rust
struct GnuHashHeader<E: Endian> {
    pub bucket_count: crate::endian::U32<E>,
    pub symbol_base: crate::endian::U32<E>,
    pub bloom_count: crate::endian::U32<E>,
    pub bloom_shift: crate::endian::U32<E>,
}
```

Header of `SHT_GNU_HASH` section.

#### Fields

- **`bucket_count`**: `crate::endian::U32<E>`

  The number of hash buckets.

- **`symbol_base`**: `crate::endian::U32<E>`

  The symbol table index of the first symbol in the hash.

- **`bloom_count`**: `crate::endian::U32<E>`

  The number of words in the bloom filter.
  
  Must be a non-zero power of 2.

- **`bloom_shift`**: `crate::endian::U32<E>`

  The bit shift count for the bloom filter.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> GnuHashHeader<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Pod<E: Endian>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<E: $crate::fmt::Debug + Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `hash`

```rust
fn hash(name: &[u8]) -> u32
```

Calculate the SysV hash for a symbol name.

Used for `SHT_HASH`.

### `gnu_hash`

```rust
fn gnu_hash(name: &[u8]) -> u32
```

Calculate the GNU hash for a symbol name.

Used for `SHT_GNU_HASH`.

### `ef_e2k_mach_to_flag`

```rust
const fn ef_e2k_mach_to_flag(e_flags: u32, x: u32) -> u32
```

Encode `E_E2K_MACH_*` into `FileHeader*::e_flags`.

### `ef_e2k_flag_to_mach`

```rust
const fn ef_e2k_flag_to_mach(e_flags: u32) -> u32
```

Decode `E_E2K_MACH_*` from `FileHeader*::e_flags`.

## Constants

### `ELFMAG`

```rust
const ELFMAG: [u8; 4];
```

File identification bytes stored in `Ident::magic`.

### `ELFCLASSNONE`

```rust
const ELFCLASSNONE: u8 = 0u8;
```

Invalid class.

### `ELFCLASS32`

```rust
const ELFCLASS32: u8 = 1u8;
```

32-bit object.

### `ELFCLASS64`

```rust
const ELFCLASS64: u8 = 2u8;
```

64-bit object.

### `ELFDATANONE`

```rust
const ELFDATANONE: u8 = 0u8;
```

Invalid data encoding.

### `ELFDATA2LSB`

```rust
const ELFDATA2LSB: u8 = 1u8;
```

2's complement, little endian.

### `ELFDATA2MSB`

```rust
const ELFDATA2MSB: u8 = 2u8;
```

2's complement, big endian.

### `ELFOSABI_NONE`

```rust
const ELFOSABI_NONE: u8 = 0u8;
```

UNIX System V ABI.

### `ELFOSABI_SYSV`

```rust
const ELFOSABI_SYSV: u8 = 0u8;
```

UNIX System V ABI.

Alias.

### `ELFOSABI_HPUX`

```rust
const ELFOSABI_HPUX: u8 = 1u8;
```

HP-UX.

### `ELFOSABI_NETBSD`

```rust
const ELFOSABI_NETBSD: u8 = 2u8;
```

NetBSD.

### `ELFOSABI_GNU`

```rust
const ELFOSABI_GNU: u8 = 3u8;
```

Object uses GNU ELF extensions.

### `ELFOSABI_LINUX`

```rust
const ELFOSABI_LINUX: u8 = 3u8;
```

Object uses GNU ELF extensions.

Compatibility alias.

### `ELFOSABI_HURD`

```rust
const ELFOSABI_HURD: u8 = 4u8;
```

GNU/Hurd.

### `ELFOSABI_SOLARIS`

```rust
const ELFOSABI_SOLARIS: u8 = 6u8;
```

Sun Solaris.

### `ELFOSABI_AIX`

```rust
const ELFOSABI_AIX: u8 = 7u8;
```

IBM AIX.

### `ELFOSABI_IRIX`

```rust
const ELFOSABI_IRIX: u8 = 8u8;
```

SGI Irix.

### `ELFOSABI_FREEBSD`

```rust
const ELFOSABI_FREEBSD: u8 = 9u8;
```

FreeBSD.

### `ELFOSABI_TRU64`

```rust
const ELFOSABI_TRU64: u8 = 10u8;
```

Compaq TRU64 UNIX.

### `ELFOSABI_MODESTO`

```rust
const ELFOSABI_MODESTO: u8 = 11u8;
```

Novell Modesto.

### `ELFOSABI_OPENBSD`

```rust
const ELFOSABI_OPENBSD: u8 = 12u8;
```

OpenBSD.

### `ELFOSABI_OPENVMS`

```rust
const ELFOSABI_OPENVMS: u8 = 13u8;
```

OpenVMS.

### `ELFOSABI_NSK`

```rust
const ELFOSABI_NSK: u8 = 14u8;
```

Hewlett-Packard Non-Stop Kernel.

### `ELFOSABI_AROS`

```rust
const ELFOSABI_AROS: u8 = 15u8;
```

AROS

### `ELFOSABI_FENIXOS`

```rust
const ELFOSABI_FENIXOS: u8 = 16u8;
```

FenixOS

### `ELFOSABI_CLOUDABI`

```rust
const ELFOSABI_CLOUDABI: u8 = 17u8;
```

Nuxi CloudABI

### `ELFOSABI_ARM_AEABI`

```rust
const ELFOSABI_ARM_AEABI: u8 = 64u8;
```

ARM EABI.

### `ELFOSABI_ARM`

```rust
const ELFOSABI_ARM: u8 = 97u8;
```

ARM.

### `ELFOSABI_STANDALONE`

```rust
const ELFOSABI_STANDALONE: u8 = 255u8;
```

Standalone (embedded) application.

### `ET_NONE`

```rust
const ET_NONE: u16 = 0u16;
```

No file type.

### `ET_REL`

```rust
const ET_REL: u16 = 1u16;
```

Relocatable file.

### `ET_EXEC`

```rust
const ET_EXEC: u16 = 2u16;
```

Executable file.

### `ET_DYN`

```rust
const ET_DYN: u16 = 3u16;
```

Shared object file.

### `ET_CORE`

```rust
const ET_CORE: u16 = 4u16;
```

Core file.

### `ET_LOOS`

```rust
const ET_LOOS: u16 = 65_024u16;
```

OS-specific range start.

### `ET_HIOS`

```rust
const ET_HIOS: u16 = 65_279u16;
```

OS-specific range end.

### `ET_LOPROC`

```rust
const ET_LOPROC: u16 = 65_280u16;
```

Processor-specific range start.

### `ET_HIPROC`

```rust
const ET_HIPROC: u16 = 65_535u16;
```

Processor-specific range end.

### `EM_NONE`

```rust
const EM_NONE: u16 = 0u16;
```

No machine

### `EM_M32`

```rust
const EM_M32: u16 = 1u16;
```

AT&T WE 32100

### `EM_SPARC`

```rust
const EM_SPARC: u16 = 2u16;
```

SUN SPARC

### `EM_386`

```rust
const EM_386: u16 = 3u16;
```

Intel 80386

### `EM_68K`

```rust
const EM_68K: u16 = 4u16;
```

Motorola m68k family

### `EM_88K`

```rust
const EM_88K: u16 = 5u16;
```

Motorola m88k family

### `EM_IAMCU`

```rust
const EM_IAMCU: u16 = 6u16;
```

Intel MCU

### `EM_860`

```rust
const EM_860: u16 = 7u16;
```

Intel 80860

### `EM_MIPS`

```rust
const EM_MIPS: u16 = 8u16;
```

MIPS R3000 big-endian

### `EM_S370`

```rust
const EM_S370: u16 = 9u16;
```

IBM System/370

### `EM_MIPS_RS3_LE`

```rust
const EM_MIPS_RS3_LE: u16 = 10u16;
```

MIPS R3000 little-endian

### `EM_PARISC`

```rust
const EM_PARISC: u16 = 15u16;
```

HPPA

### `EM_VPP500`

```rust
const EM_VPP500: u16 = 17u16;
```

Fujitsu VPP500

### `EM_SPARC32PLUS`

```rust
const EM_SPARC32PLUS: u16 = 18u16;
```

Sun's "v8plus"

### `EM_960`

```rust
const EM_960: u16 = 19u16;
```

Intel 80960

### `EM_PPC`

```rust
const EM_PPC: u16 = 20u16;
```

PowerPC

### `EM_PPC64`

```rust
const EM_PPC64: u16 = 21u16;
```

PowerPC 64-bit

### `EM_S390`

```rust
const EM_S390: u16 = 22u16;
```

IBM S390

### `EM_SPU`

```rust
const EM_SPU: u16 = 23u16;
```

IBM SPU/SPC

### `EM_V800`

```rust
const EM_V800: u16 = 36u16;
```

NEC V800 series

### `EM_FR20`

```rust
const EM_FR20: u16 = 37u16;
```

Fujitsu FR20

### `EM_RH32`

```rust
const EM_RH32: u16 = 38u16;
```

TRW RH-32

### `EM_RCE`

```rust
const EM_RCE: u16 = 39u16;
```

Motorola RCE

### `EM_ARM`

```rust
const EM_ARM: u16 = 40u16;
```

ARM

### `EM_FAKE_ALPHA`

```rust
const EM_FAKE_ALPHA: u16 = 41u16;
```

Digital Alpha

### `EM_SH`

```rust
const EM_SH: u16 = 42u16;
```

Hitachi SH

### `EM_SPARCV9`

```rust
const EM_SPARCV9: u16 = 43u16;
```

SPARC v9 64-bit

### `EM_TRICORE`

```rust
const EM_TRICORE: u16 = 44u16;
```

Siemens Tricore

### `EM_ARC`

```rust
const EM_ARC: u16 = 45u16;
```

Argonaut RISC Core

### `EM_H8_300`

```rust
const EM_H8_300: u16 = 46u16;
```

Hitachi H8/300

### `EM_H8_300H`

```rust
const EM_H8_300H: u16 = 47u16;
```

Hitachi H8/300H

### `EM_H8S`

```rust
const EM_H8S: u16 = 48u16;
```

Hitachi H8S

### `EM_H8_500`

```rust
const EM_H8_500: u16 = 49u16;
```

Hitachi H8/500

### `EM_IA_64`

```rust
const EM_IA_64: u16 = 50u16;
```

Intel Merced

### `EM_MIPS_X`

```rust
const EM_MIPS_X: u16 = 51u16;
```

Stanford MIPS-X

### `EM_COLDFIRE`

```rust
const EM_COLDFIRE: u16 = 52u16;
```

Motorola Coldfire

### `EM_68HC12`

```rust
const EM_68HC12: u16 = 53u16;
```

Motorola M68HC12

### `EM_MMA`

```rust
const EM_MMA: u16 = 54u16;
```

Fujitsu MMA Multimedia Accelerator

### `EM_PCP`

```rust
const EM_PCP: u16 = 55u16;
```

Siemens PCP

### `EM_NCPU`

```rust
const EM_NCPU: u16 = 56u16;
```

Sony nCPU embeeded RISC

### `EM_NDR1`

```rust
const EM_NDR1: u16 = 57u16;
```

Denso NDR1 microprocessor

### `EM_STARCORE`

```rust
const EM_STARCORE: u16 = 58u16;
```

Motorola Start*Core processor

### `EM_ME16`

```rust
const EM_ME16: u16 = 59u16;
```

Toyota ME16 processor

### `EM_ST100`

```rust
const EM_ST100: u16 = 60u16;
```

STMicroelectronic ST100 processor

### `EM_TINYJ`

```rust
const EM_TINYJ: u16 = 61u16;
```

Advanced Logic Corp. Tinyj emb.fam

### `EM_X86_64`

```rust
const EM_X86_64: u16 = 62u16;
```

AMD x86-64 architecture

### `EM_PDSP`

```rust
const EM_PDSP: u16 = 63u16;
```

Sony DSP Processor

### `EM_PDP10`

```rust
const EM_PDP10: u16 = 64u16;
```

Digital PDP-10

### `EM_PDP11`

```rust
const EM_PDP11: u16 = 65u16;
```

Digital PDP-11

### `EM_FX66`

```rust
const EM_FX66: u16 = 66u16;
```

Siemens FX66 microcontroller

### `EM_ST9PLUS`

```rust
const EM_ST9PLUS: u16 = 67u16;
```

STMicroelectronics ST9+ 8/16 mc

### `EM_ST7`

```rust
const EM_ST7: u16 = 68u16;
```

STmicroelectronics ST7 8 bit mc

### `EM_68HC16`

```rust
const EM_68HC16: u16 = 69u16;
```

Motorola MC68HC16 microcontroller

### `EM_68HC11`

```rust
const EM_68HC11: u16 = 70u16;
```

Motorola MC68HC11 microcontroller

### `EM_68HC08`

```rust
const EM_68HC08: u16 = 71u16;
```

Motorola MC68HC08 microcontroller

### `EM_68HC05`

```rust
const EM_68HC05: u16 = 72u16;
```

Motorola MC68HC05 microcontroller

### `EM_SVX`

```rust
const EM_SVX: u16 = 73u16;
```

Silicon Graphics SVx

### `EM_ST19`

```rust
const EM_ST19: u16 = 74u16;
```

STMicroelectronics ST19 8 bit mc

### `EM_VAX`

```rust
const EM_VAX: u16 = 75u16;
```

Digital VAX

### `EM_CRIS`

```rust
const EM_CRIS: u16 = 76u16;
```

Axis Communications 32-bit emb.proc

### `EM_JAVELIN`

```rust
const EM_JAVELIN: u16 = 77u16;
```

Infineon Technologies 32-bit emb.proc

### `EM_FIREPATH`

```rust
const EM_FIREPATH: u16 = 78u16;
```

Element 14 64-bit DSP Processor

### `EM_ZSP`

```rust
const EM_ZSP: u16 = 79u16;
```

LSI Logic 16-bit DSP Processor

### `EM_MMIX`

```rust
const EM_MMIX: u16 = 80u16;
```

Donald Knuth's educational 64-bit proc

### `EM_HUANY`

```rust
const EM_HUANY: u16 = 81u16;
```

Harvard University machine-independent object files

### `EM_PRISM`

```rust
const EM_PRISM: u16 = 82u16;
```

SiTera Prism

### `EM_AVR`

```rust
const EM_AVR: u16 = 83u16;
```

Atmel AVR 8-bit microcontroller

### `EM_FR30`

```rust
const EM_FR30: u16 = 84u16;
```

Fujitsu FR30

### `EM_D10V`

```rust
const EM_D10V: u16 = 85u16;
```

Mitsubishi D10V

### `EM_D30V`

```rust
const EM_D30V: u16 = 86u16;
```

Mitsubishi D30V

### `EM_V850`

```rust
const EM_V850: u16 = 87u16;
```

NEC v850

### `EM_M32R`

```rust
const EM_M32R: u16 = 88u16;
```

Mitsubishi M32R

### `EM_MN10300`

```rust
const EM_MN10300: u16 = 89u16;
```

Matsushita MN10300

### `EM_MN10200`

```rust
const EM_MN10200: u16 = 90u16;
```

Matsushita MN10200

### `EM_PJ`

```rust
const EM_PJ: u16 = 91u16;
```

picoJava

### `EM_OPENRISC`

```rust
const EM_OPENRISC: u16 = 92u16;
```

OpenRISC 32-bit embedded processor

### `EM_ARC_COMPACT`

```rust
const EM_ARC_COMPACT: u16 = 93u16;
```

ARC International ARCompact

### `EM_XTENSA`

```rust
const EM_XTENSA: u16 = 94u16;
```

Tensilica Xtensa Architecture

### `EM_VIDEOCORE`

```rust
const EM_VIDEOCORE: u16 = 95u16;
```

Alphamosaic VideoCore

### `EM_TMM_GPP`

```rust
const EM_TMM_GPP: u16 = 96u16;
```

Thompson Multimedia General Purpose Proc

### `EM_NS32K`

```rust
const EM_NS32K: u16 = 97u16;
```

National Semi. 32000

### `EM_TPC`

```rust
const EM_TPC: u16 = 98u16;
```

Tenor Network TPC

### `EM_SNP1K`

```rust
const EM_SNP1K: u16 = 99u16;
```

Trebia SNP 1000

### `EM_ST200`

```rust
const EM_ST200: u16 = 100u16;
```

STMicroelectronics ST200

### `EM_IP2K`

```rust
const EM_IP2K: u16 = 101u16;
```

Ubicom IP2xxx

### `EM_MAX`

```rust
const EM_MAX: u16 = 102u16;
```

MAX processor

### `EM_CR`

```rust
const EM_CR: u16 = 103u16;
```

National Semi. CompactRISC

### `EM_F2MC16`

```rust
const EM_F2MC16: u16 = 104u16;
```

Fujitsu F2MC16

### `EM_MSP430`

```rust
const EM_MSP430: u16 = 105u16;
```

Texas Instruments msp430

### `EM_BLACKFIN`

```rust
const EM_BLACKFIN: u16 = 106u16;
```

Analog Devices Blackfin DSP

### `EM_SE_C33`

```rust
const EM_SE_C33: u16 = 107u16;
```

Seiko Epson S1C33 family

### `EM_SEP`

```rust
const EM_SEP: u16 = 108u16;
```

Sharp embedded microprocessor

### `EM_ARCA`

```rust
const EM_ARCA: u16 = 109u16;
```

Arca RISC

### `EM_UNICORE`

```rust
const EM_UNICORE: u16 = 110u16;
```

PKU-Unity & MPRC Peking Uni. mc series

### `EM_EXCESS`

```rust
const EM_EXCESS: u16 = 111u16;
```

eXcess configurable cpu

### `EM_DXP`

```rust
const EM_DXP: u16 = 112u16;
```

Icera Semi. Deep Execution Processor

### `EM_ALTERA_NIOS2`

```rust
const EM_ALTERA_NIOS2: u16 = 113u16;
```

Altera Nios II

### `EM_CRX`

```rust
const EM_CRX: u16 = 114u16;
```

National Semi. CompactRISC CRX

### `EM_XGATE`

```rust
const EM_XGATE: u16 = 115u16;
```

Motorola XGATE

### `EM_C166`

```rust
const EM_C166: u16 = 116u16;
```

Infineon C16x/XC16x

### `EM_M16C`

```rust
const EM_M16C: u16 = 117u16;
```

Renesas M16C

### `EM_DSPIC30F`

```rust
const EM_DSPIC30F: u16 = 118u16;
```

Microchip Technology dsPIC30F

### `EM_CE`

```rust
const EM_CE: u16 = 119u16;
```

Freescale Communication Engine RISC

### `EM_M32C`

```rust
const EM_M32C: u16 = 120u16;
```

Renesas M32C

### `EM_TSK3000`

```rust
const EM_TSK3000: u16 = 131u16;
```

Altium TSK3000

### `EM_RS08`

```rust
const EM_RS08: u16 = 132u16;
```

Freescale RS08

### `EM_SHARC`

```rust
const EM_SHARC: u16 = 133u16;
```

Analog Devices SHARC family

### `EM_ECOG2`

```rust
const EM_ECOG2: u16 = 134u16;
```

Cyan Technology eCOG2

### `EM_SCORE7`

```rust
const EM_SCORE7: u16 = 135u16;
```

Sunplus S+core7 RISC

### `EM_DSP24`

```rust
const EM_DSP24: u16 = 136u16;
```

New Japan Radio (NJR) 24-bit DSP

### `EM_VIDEOCORE3`

```rust
const EM_VIDEOCORE3: u16 = 137u16;
```

Broadcom VideoCore III

### `EM_LATTICEMICO32`

```rust
const EM_LATTICEMICO32: u16 = 138u16;
```

RISC for Lattice FPGA

### `EM_SE_C17`

```rust
const EM_SE_C17: u16 = 139u16;
```

Seiko Epson C17

### `EM_TI_C6000`

```rust
const EM_TI_C6000: u16 = 140u16;
```

Texas Instruments TMS320C6000 DSP

### `EM_TI_C2000`

```rust
const EM_TI_C2000: u16 = 141u16;
```

Texas Instruments TMS320C2000 DSP

### `EM_TI_C5500`

```rust
const EM_TI_C5500: u16 = 142u16;
```

Texas Instruments TMS320C55x DSP

### `EM_TI_ARP32`

```rust
const EM_TI_ARP32: u16 = 143u16;
```

Texas Instruments App. Specific RISC

### `EM_TI_PRU`

```rust
const EM_TI_PRU: u16 = 144u16;
```

Texas Instruments Prog. Realtime Unit

### `EM_MMDSP_PLUS`

```rust
const EM_MMDSP_PLUS: u16 = 160u16;
```

STMicroelectronics 64bit VLIW DSP

### `EM_CYPRESS_M8C`

```rust
const EM_CYPRESS_M8C: u16 = 161u16;
```

Cypress M8C

### `EM_R32C`

```rust
const EM_R32C: u16 = 162u16;
```

Renesas R32C

### `EM_TRIMEDIA`

```rust
const EM_TRIMEDIA: u16 = 163u16;
```

NXP Semi. TriMedia

### `EM_HEXAGON`

```rust
const EM_HEXAGON: u16 = 164u16;
```

QUALCOMM Hexagon

### `EM_8051`

```rust
const EM_8051: u16 = 165u16;
```

Intel 8051 and variants

### `EM_STXP7X`

```rust
const EM_STXP7X: u16 = 166u16;
```

STMicroelectronics STxP7x

### `EM_NDS32`

```rust
const EM_NDS32: u16 = 167u16;
```

Andes Tech. compact code emb. RISC

### `EM_ECOG1X`

```rust
const EM_ECOG1X: u16 = 168u16;
```

Cyan Technology eCOG1X

### `EM_MAXQ30`

```rust
const EM_MAXQ30: u16 = 169u16;
```

Dallas Semi. MAXQ30 mc

### `EM_XIMO16`

```rust
const EM_XIMO16: u16 = 170u16;
```

New Japan Radio (NJR) 16-bit DSP

### `EM_MANIK`

```rust
const EM_MANIK: u16 = 171u16;
```

M2000 Reconfigurable RISC

### `EM_CRAYNV2`

```rust
const EM_CRAYNV2: u16 = 172u16;
```

Cray NV2 vector architecture

### `EM_RX`

```rust
const EM_RX: u16 = 173u16;
```

Renesas RX

### `EM_METAG`

```rust
const EM_METAG: u16 = 174u16;
```

Imagination Tech. META

### `EM_MCST_ELBRUS`

```rust
const EM_MCST_ELBRUS: u16 = 175u16;
```

MCST Elbrus

### `EM_ECOG16`

```rust
const EM_ECOG16: u16 = 176u16;
```

Cyan Technology eCOG16

### `EM_CR16`

```rust
const EM_CR16: u16 = 177u16;
```

National Semi. CompactRISC CR16

### `EM_ETPU`

```rust
const EM_ETPU: u16 = 178u16;
```

Freescale Extended Time Processing Unit

### `EM_SLE9X`

```rust
const EM_SLE9X: u16 = 179u16;
```

Infineon Tech. SLE9X

### `EM_L10M`

```rust
const EM_L10M: u16 = 180u16;
```

Intel L10M

### `EM_K10M`

```rust
const EM_K10M: u16 = 181u16;
```

Intel K10M

### `EM_AARCH64`

```rust
const EM_AARCH64: u16 = 183u16;
```

ARM AARCH64

### `EM_AVR32`

```rust
const EM_AVR32: u16 = 185u16;
```

Amtel 32-bit microprocessor

### `EM_STM8`

```rust
const EM_STM8: u16 = 186u16;
```

STMicroelectronics STM8

### `EM_TILE64`

```rust
const EM_TILE64: u16 = 187u16;
```

Tileta TILE64

### `EM_TILEPRO`

```rust
const EM_TILEPRO: u16 = 188u16;
```

Tilera TILEPro

### `EM_MICROBLAZE`

```rust
const EM_MICROBLAZE: u16 = 189u16;
```

Xilinx MicroBlaze

### `EM_CUDA`

```rust
const EM_CUDA: u16 = 190u16;
```

NVIDIA CUDA

### `EM_TILEGX`

```rust
const EM_TILEGX: u16 = 191u16;
```

Tilera TILE-Gx

### `EM_CLOUDSHIELD`

```rust
const EM_CLOUDSHIELD: u16 = 192u16;
```

CloudShield

### `EM_COREA_1ST`

```rust
const EM_COREA_1ST: u16 = 193u16;
```

KIPO-KAIST Core-A 1st gen.

### `EM_COREA_2ND`

```rust
const EM_COREA_2ND: u16 = 194u16;
```

KIPO-KAIST Core-A 2nd gen.

### `EM_ARC_COMPACT2`

```rust
const EM_ARC_COMPACT2: u16 = 195u16;
```

Synopsys ARCompact V2

### `EM_OPEN8`

```rust
const EM_OPEN8: u16 = 196u16;
```

Open8 RISC

### `EM_RL78`

```rust
const EM_RL78: u16 = 197u16;
```

Renesas RL78

### `EM_VIDEOCORE5`

```rust
const EM_VIDEOCORE5: u16 = 198u16;
```

Broadcom VideoCore V

### `EM_78KOR`

```rust
const EM_78KOR: u16 = 199u16;
```

Renesas 78KOR

### `EM_56800EX`

```rust
const EM_56800EX: u16 = 200u16;
```

Freescale 56800EX DSC

### `EM_BA1`

```rust
const EM_BA1: u16 = 201u16;
```

Beyond BA1

### `EM_BA2`

```rust
const EM_BA2: u16 = 202u16;
```

Beyond BA2

### `EM_XCORE`

```rust
const EM_XCORE: u16 = 203u16;
```

XMOS xCORE

### `EM_MCHP_PIC`

```rust
const EM_MCHP_PIC: u16 = 204u16;
```

Microchip 8-bit PIC(r)

### `EM_KM32`

```rust
const EM_KM32: u16 = 210u16;
```

KM211 KM32

### `EM_KMX32`

```rust
const EM_KMX32: u16 = 211u16;
```

KM211 KMX32

### `EM_EMX16`

```rust
const EM_EMX16: u16 = 212u16;
```

KM211 KMX16

### `EM_EMX8`

```rust
const EM_EMX8: u16 = 213u16;
```

KM211 KMX8

### `EM_KVARC`

```rust
const EM_KVARC: u16 = 214u16;
```

KM211 KVARC

### `EM_CDP`

```rust
const EM_CDP: u16 = 215u16;
```

Paneve CDP

### `EM_COGE`

```rust
const EM_COGE: u16 = 216u16;
```

Cognitive Smart Memory Processor

### `EM_COOL`

```rust
const EM_COOL: u16 = 217u16;
```

Bluechip CoolEngine

### `EM_NORC`

```rust
const EM_NORC: u16 = 218u16;
```

Nanoradio Optimized RISC

### `EM_CSR_KALIMBA`

```rust
const EM_CSR_KALIMBA: u16 = 219u16;
```

CSR Kalimba

### `EM_Z80`

```rust
const EM_Z80: u16 = 220u16;
```

Zilog Z80

### `EM_VISIUM`

```rust
const EM_VISIUM: u16 = 221u16;
```

Controls and Data Services VISIUMcore

### `EM_FT32`

```rust
const EM_FT32: u16 = 222u16;
```

FTDI Chip FT32

### `EM_MOXIE`

```rust
const EM_MOXIE: u16 = 223u16;
```

Moxie processor

### `EM_AMDGPU`

```rust
const EM_AMDGPU: u16 = 224u16;
```

AMD GPU

### `EM_RISCV`

```rust
const EM_RISCV: u16 = 243u16;
```

RISC-V

### `EM_BPF`

```rust
const EM_BPF: u16 = 247u16;
```

Linux BPF -- in-kernel virtual machine

### `EM_CSKY`

```rust
const EM_CSKY: u16 = 252u16;
```

C-SKY

### `EM_LOONGARCH`

```rust
const EM_LOONGARCH: u16 = 258u16;
```

Loongson LoongArch

### `EM_SBF`

```rust
const EM_SBF: u16 = 263u16;
```

Solana Binary Format

### `EM_ALPHA`

```rust
const EM_ALPHA: u16 = 36_902u16;
```

Digital Alpha

### `EV_NONE`

```rust
const EV_NONE: u8 = 0u8;
```

Invalid ELF version.

### `EV_CURRENT`

```rust
const EV_CURRENT: u8 = 1u8;
```

Current ELF version.

### `SHN_UNDEF`

```rust
const SHN_UNDEF: u16 = 0u16;
```

Undefined section.

### `SHN_LORESERVE`

```rust
const SHN_LORESERVE: u16 = 65_280u16;
```

OS-specific range start.
Start of reserved section indices.

### `SHN_LOPROC`

```rust
const SHN_LOPROC: u16 = 65_280u16;
```

Start of processor-specific section indices.

### `SHN_HIPROC`

```rust
const SHN_HIPROC: u16 = 65_311u16;
```

End of processor-specific section indices.

### `SHN_LOOS`

```rust
const SHN_LOOS: u16 = 65_312u16;
```

Start of OS-specific section indices.

### `SHN_HIOS`

```rust
const SHN_HIOS: u16 = 65_343u16;
```

End of OS-specific section indices.

### `SHN_ABS`

```rust
const SHN_ABS: u16 = 65_521u16;
```

Associated symbol is absolute.

### `SHN_COMMON`

```rust
const SHN_COMMON: u16 = 65_522u16;
```

Associated symbol is common.

### `SHN_XINDEX`

```rust
const SHN_XINDEX: u16 = 65_535u16;
```

Section index is in the `SHT_SYMTAB_SHNDX` section.

### `SHN_HIRESERVE`

```rust
const SHN_HIRESERVE: u16 = 65_535u16;
```

End of reserved section indices.

### `SHT_NULL`

```rust
const SHT_NULL: u32 = 0u32;
```

Section header table entry is unused.

### `SHT_PROGBITS`

```rust
const SHT_PROGBITS: u32 = 1u32;
```

Program data.

### `SHT_SYMTAB`

```rust
const SHT_SYMTAB: u32 = 2u32;
```

Symbol table.

### `SHT_STRTAB`

```rust
const SHT_STRTAB: u32 = 3u32;
```

String table.

### `SHT_RELA`

```rust
const SHT_RELA: u32 = 4u32;
```

Relocation entries with explicit addends.

### `SHT_HASH`

```rust
const SHT_HASH: u32 = 5u32;
```

Symbol hash table.

### `SHT_DYNAMIC`

```rust
const SHT_DYNAMIC: u32 = 6u32;
```

Dynamic linking information.

### `SHT_NOTE`

```rust
const SHT_NOTE: u32 = 7u32;
```

Notes.

### `SHT_NOBITS`

```rust
const SHT_NOBITS: u32 = 8u32;
```

Program space with no data (bss).

### `SHT_REL`

```rust
const SHT_REL: u32 = 9u32;
```

Relocation entries without explicit addends.

### `SHT_SHLIB`

```rust
const SHT_SHLIB: u32 = 10u32;
```

Reserved section type.

### `SHT_DYNSYM`

```rust
const SHT_DYNSYM: u32 = 11u32;
```

Dynamic linker symbol table.

### `SHT_INIT_ARRAY`

```rust
const SHT_INIT_ARRAY: u32 = 14u32;
```

Array of constructors.

### `SHT_FINI_ARRAY`

```rust
const SHT_FINI_ARRAY: u32 = 15u32;
```

Array of destructors.

### `SHT_PREINIT_ARRAY`

```rust
const SHT_PREINIT_ARRAY: u32 = 16u32;
```

Array of pre-constructors.

### `SHT_GROUP`

```rust
const SHT_GROUP: u32 = 17u32;
```

Section group.

### `SHT_SYMTAB_SHNDX`

```rust
const SHT_SYMTAB_SHNDX: u32 = 18u32;
```

Extended section indices for a symbol table.

### `SHT_RELR`

```rust
const SHT_RELR: u32 = 19u32;
```

Relocation entries; only offsets.

### `SHT_CREL`

```rust
const SHT_CREL: u32 = 1_073_741_844u32;
```

Experimental CREL relocations. LLVM will change the value and
break compatibility in the future.

### `SHT_LOOS`

```rust
const SHT_LOOS: u32 = 1_610_612_736u32;
```

Start of OS-specific section types.

### `SHT_LLVM_DEPENDENT_LIBRARIES`

```rust
const SHT_LLVM_DEPENDENT_LIBRARIES: u32 = 1_879_002_116u32;
```

LLVM-style dependent libraries.

### `SHT_GNU_SFRAME`

```rust
const SHT_GNU_SFRAME: u32 = 1_879_048_180u32;
```

GNU SFrame stack trace format.

### `SHT_GNU_ATTRIBUTES`

```rust
const SHT_GNU_ATTRIBUTES: u32 = 1_879_048_181u32;
```

Object attributes.

### `SHT_GNU_HASH`

```rust
const SHT_GNU_HASH: u32 = 1_879_048_182u32;
```

GNU-style hash table.

### `SHT_GNU_LIBLIST`

```rust
const SHT_GNU_LIBLIST: u32 = 1_879_048_183u32;
```

Prelink library list

### `SHT_CHECKSUM`

```rust
const SHT_CHECKSUM: u32 = 1_879_048_184u32;
```

Checksum for DSO content.

### `SHT_LOSUNW`

```rust
const SHT_LOSUNW: u32 = 1_879_048_186u32;
```

Sun-specific low bound.

### `SHT_SUNW_move`

```rust
const SHT_SUNW_move: u32 = 1_879_048_186u32;
```

### `SHT_SUNW_COMDAT`

```rust
const SHT_SUNW_COMDAT: u32 = 1_879_048_187u32;
```

### `SHT_SUNW_syminfo`

```rust
const SHT_SUNW_syminfo: u32 = 1_879_048_188u32;
```

### `SHT_GNU_VERDEF`

```rust
const SHT_GNU_VERDEF: u32 = 1_879_048_189u32;
```

Version definition section.

### `SHT_GNU_VERNEED`

```rust
const SHT_GNU_VERNEED: u32 = 1_879_048_190u32;
```

Version needs section.

### `SHT_GNU_VERSYM`

```rust
const SHT_GNU_VERSYM: u32 = 1_879_048_191u32;
```

Version symbol table.

### `SHT_HISUNW`

```rust
const SHT_HISUNW: u32 = 1_879_048_191u32;
```

Sun-specific high bound.

### `SHT_HIOS`

```rust
const SHT_HIOS: u32 = 1_879_048_191u32;
```

End of OS-specific section types.

### `SHT_LOPROC`

```rust
const SHT_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific section types.

### `SHT_HIPROC`

```rust
const SHT_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific section types.

### `SHT_LOUSER`

```rust
const SHT_LOUSER: u32 = 2_147_483_648u32;
```

Start of application-specific section types.

### `SHT_HIUSER`

```rust
const SHT_HIUSER: u32 = 2_415_919_103u32;
```

End of application-specific section types.

### `SHF_WRITE`

```rust
const SHF_WRITE: u32 = 1u32;
```

Section is writable.

### `SHF_ALLOC`

```rust
const SHF_ALLOC: u32 = 2u32;
```

Section occupies memory during execution.

### `SHF_EXECINSTR`

```rust
const SHF_EXECINSTR: u32 = 4u32;
```

Section is executable.

### `SHF_MERGE`

```rust
const SHF_MERGE: u32 = 16u32;
```

Section may be be merged to eliminate duplication.

### `SHF_STRINGS`

```rust
const SHF_STRINGS: u32 = 32u32;
```

Section contains nul-terminated strings.

### `SHF_INFO_LINK`

```rust
const SHF_INFO_LINK: u32 = 64u32;
```

The `sh_info` field contains a section header table index.

### `SHF_LINK_ORDER`

```rust
const SHF_LINK_ORDER: u32 = 128u32;
```

Section has special ordering requirements when combining sections.

### `SHF_OS_NONCONFORMING`

```rust
const SHF_OS_NONCONFORMING: u32 = 256u32;
```

Section requires special OS-specific handling.

### `SHF_GROUP`

```rust
const SHF_GROUP: u32 = 512u32;
```

Section is a member of a group.

### `SHF_TLS`

```rust
const SHF_TLS: u32 = 1_024u32;
```

Section holds thread-local storage.

### `SHF_COMPRESSED`

```rust
const SHF_COMPRESSED: u32 = 2_048u32;
```

Section is compressed.

Compressed sections begin with one of the `CompressionHeader*` headers.

### `SHF_MASKOS`

```rust
const SHF_MASKOS: u32 = 267_386_880u32;
```

OS-specific section flags.

### `SHF_GNU_RETAIN`

```rust
const SHF_GNU_RETAIN: u32 = 2_097_152u32;
```

Section should not be garbage collected by the linker.

### `SHF_GNU_MBIND`

```rust
const SHF_GNU_MBIND: u32 = 16_777_216u32;
```

Mbind section.

### `SHF_MASKPROC`

```rust
const SHF_MASKPROC: u32 = 4_026_531_840u32;
```

Processor-specific section flags.

### `SHF_EXCLUDE`

```rust
const SHF_EXCLUDE: u32 = 2_147_483_648u32;
```

This section is excluded from the final executable or shared library.

### `ELFCOMPRESS_ZLIB`

```rust
const ELFCOMPRESS_ZLIB: u32 = 1u32;
```

ZLIB/DEFLATE algorithm.

### `ELFCOMPRESS_ZSTD`

```rust
const ELFCOMPRESS_ZSTD: u32 = 2u32;
```

Zstandard algorithm.

### `ELFCOMPRESS_LOOS`

```rust
const ELFCOMPRESS_LOOS: u32 = 1_610_612_736u32;
```

Start of OS-specific compression types.

### `ELFCOMPRESS_HIOS`

```rust
const ELFCOMPRESS_HIOS: u32 = 1_879_048_191u32;
```

End of OS-specific compression types.

### `ELFCOMPRESS_LOPROC`

```rust
const ELFCOMPRESS_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific compression types.

### `ELFCOMPRESS_HIPROC`

```rust
const ELFCOMPRESS_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific compression types.

### `GRP_COMDAT`

```rust
const GRP_COMDAT: u32 = 1u32;
```

Mark group as COMDAT.

### `SYMINFO_BT_SELF`

```rust
const SYMINFO_BT_SELF: u16 = 65_535u16;
```

Symbol bound to self

### `SYMINFO_BT_PARENT`

```rust
const SYMINFO_BT_PARENT: u16 = 65_534u16;
```

Symbol bound to parent

### `SYMINFO_BT_LOWRESERVE`

```rust
const SYMINFO_BT_LOWRESERVE: u16 = 65_280u16;
```

Beginning of reserved entries

### `SYMINFO_FLG_DIRECT`

```rust
const SYMINFO_FLG_DIRECT: u16 = 1u16;
```

Direct bound symbol

### `SYMINFO_FLG_PASSTHRU`

```rust
const SYMINFO_FLG_PASSTHRU: u16 = 2u16;
```

Pass-thru symbol for translator

### `SYMINFO_FLG_COPY`

```rust
const SYMINFO_FLG_COPY: u16 = 4u16;
```

Symbol is a copy-reloc

### `SYMINFO_FLG_LAZYLOAD`

```rust
const SYMINFO_FLG_LAZYLOAD: u16 = 8u16;
```

Symbol bound to object to be lazy loaded

### `SYMINFO_NONE`

```rust
const SYMINFO_NONE: u16 = 0u16;
```

### `SYMINFO_CURRENT`

```rust
const SYMINFO_CURRENT: u16 = 1u16;
```

### `SYMINFO_NUM`

```rust
const SYMINFO_NUM: u16 = 2u16;
```

### `STB_LOCAL`

```rust
const STB_LOCAL: u8 = 0u8;
```

Local symbol.

### `STB_GLOBAL`

```rust
const STB_GLOBAL: u8 = 1u8;
```

Global symbol.

### `STB_WEAK`

```rust
const STB_WEAK: u8 = 2u8;
```

Weak symbol.

### `STB_LOOS`

```rust
const STB_LOOS: u8 = 10u8;
```

Start of OS-specific symbol binding.

### `STB_GNU_UNIQUE`

```rust
const STB_GNU_UNIQUE: u8 = 10u8;
```

Unique symbol.

### `STB_HIOS`

```rust
const STB_HIOS: u8 = 12u8;
```

End of OS-specific symbol binding.

### `STB_LOPROC`

```rust
const STB_LOPROC: u8 = 13u8;
```

Start of processor-specific symbol binding.

### `STB_HIPROC`

```rust
const STB_HIPROC: u8 = 15u8;
```

End of processor-specific symbol binding.

### `STT_NOTYPE`

```rust
const STT_NOTYPE: u8 = 0u8;
```

Symbol type is unspecified.

### `STT_OBJECT`

```rust
const STT_OBJECT: u8 = 1u8;
```

Symbol is a data object.

### `STT_FUNC`

```rust
const STT_FUNC: u8 = 2u8;
```

Symbol is a code object.

### `STT_SECTION`

```rust
const STT_SECTION: u8 = 3u8;
```

Symbol is associated with a section.

### `STT_FILE`

```rust
const STT_FILE: u8 = 4u8;
```

Symbol's name is a file name.

### `STT_COMMON`

```rust
const STT_COMMON: u8 = 5u8;
```

Symbol is a common data object.

### `STT_TLS`

```rust
const STT_TLS: u8 = 6u8;
```

Symbol is a thread-local storage object.

### `STT_LOOS`

```rust
const STT_LOOS: u8 = 10u8;
```

Start of OS-specific symbol types.

### `STT_GNU_IFUNC`

```rust
const STT_GNU_IFUNC: u8 = 10u8;
```

Symbol is an indirect code object.

### `STT_HIOS`

```rust
const STT_HIOS: u8 = 12u8;
```

End of OS-specific symbol types.

### `STT_LOPROC`

```rust
const STT_LOPROC: u8 = 13u8;
```

Start of processor-specific symbol types.

### `STT_HIPROC`

```rust
const STT_HIPROC: u8 = 15u8;
```

End of processor-specific symbol types.

### `STV_DEFAULT`

```rust
const STV_DEFAULT: u8 = 0u8;
```

Default symbol visibility rules.

### `STV_INTERNAL`

```rust
const STV_INTERNAL: u8 = 1u8;
```

Processor specific hidden class.

### `STV_HIDDEN`

```rust
const STV_HIDDEN: u8 = 2u8;
```

Symbol is not visible to other components.

### `STV_PROTECTED`

```rust
const STV_PROTECTED: u8 = 3u8;
```

Symbol is visible to other components, but is not preemptible.

### `PN_XNUM`

```rust
const PN_XNUM: u16 = 65_535u16;
```

Special value for `FileHeader*::e_phnum`.

This indicates that the real number of program headers is too large to fit into e_phnum.
Instead the real value is in the field `sh_info` of section 0.

### `PT_NULL`

```rust
const PT_NULL: u32 = 0u32;
```

Program header table entry is unused.

### `PT_LOAD`

```rust
const PT_LOAD: u32 = 1u32;
```

Loadable program segment.

### `PT_DYNAMIC`

```rust
const PT_DYNAMIC: u32 = 2u32;
```

Dynamic linking information.

### `PT_INTERP`

```rust
const PT_INTERP: u32 = 3u32;
```

Program interpreter.

### `PT_NOTE`

```rust
const PT_NOTE: u32 = 4u32;
```

Auxiliary information.

### `PT_SHLIB`

```rust
const PT_SHLIB: u32 = 5u32;
```

Reserved.

### `PT_PHDR`

```rust
const PT_PHDR: u32 = 6u32;
```

Segment contains the program header table.

### `PT_TLS`

```rust
const PT_TLS: u32 = 7u32;
```

Thread-local storage segment.

### `PT_LOOS`

```rust
const PT_LOOS: u32 = 1_610_612_736u32;
```

Start of OS-specific segment types.

### `PT_GNU_EH_FRAME`

```rust
const PT_GNU_EH_FRAME: u32 = 1_685_382_480u32;
```

GCC `.eh_frame_hdr` segment.

### `PT_GNU_STACK`

```rust
const PT_GNU_STACK: u32 = 1_685_382_481u32;
```

Indicates stack executability.

### `PT_GNU_RELRO`

```rust
const PT_GNU_RELRO: u32 = 1_685_382_482u32;
```

Read-only after relocation.

### `PT_GNU_PROPERTY`

```rust
const PT_GNU_PROPERTY: u32 = 1_685_382_483u32;
```

Segment containing `.note.gnu.property` section.

### `PT_GNU_SFRAME`

```rust
const PT_GNU_SFRAME: u32 = 1_685_382_484u32;
```

GNU SFrame stack trace format.

### `PT_HIOS`

```rust
const PT_HIOS: u32 = 1_879_048_191u32;
```

End of OS-specific segment types.

### `PT_LOPROC`

```rust
const PT_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific segment types.

### `PT_HIPROC`

```rust
const PT_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific segment types.

### `PF_X`

```rust
const PF_X: u32 = 1u32;
```

Segment is executable.

### `PF_W`

```rust
const PF_W: u32 = 2u32;
```

Segment is writable.

### `PF_R`

```rust
const PF_R: u32 = 4u32;
```

Segment is readable.

### `PF_MASKOS`

```rust
const PF_MASKOS: u32 = 267_386_880u32;
```

OS-specific segment flags.

### `PF_MASKPROC`

```rust
const PF_MASKPROC: u32 = 4_026_531_840u32;
```

Processor-specific segment flags.

### `ELF_NOTE_CORE`

```rust
const ELF_NOTE_CORE: &[u8];
```

Note name for core files.

### `ELF_NOTE_LINUX`

```rust
const ELF_NOTE_LINUX: &[u8];
```

Note name for linux core files.

Notes in linux core files may also use `ELF_NOTE_CORE`.

### `NT_PRSTATUS`

```rust
const NT_PRSTATUS: u32 = 1u32;
```

Contains copy of prstatus struct.

### `NT_PRFPREG`

```rust
const NT_PRFPREG: u32 = 2u32;
```

Contains copy of fpregset struct.

### `NT_FPREGSET`

```rust
const NT_FPREGSET: u32 = 2u32;
```

Contains copy of fpregset struct.

### `NT_PRPSINFO`

```rust
const NT_PRPSINFO: u32 = 3u32;
```

Contains copy of prpsinfo struct.

### `NT_PRXREG`

```rust
const NT_PRXREG: u32 = 4u32;
```

Contains copy of prxregset struct.

### `NT_TASKSTRUCT`

```rust
const NT_TASKSTRUCT: u32 = 4u32;
```

Contains copy of task structure.

### `NT_PLATFORM`

```rust
const NT_PLATFORM: u32 = 5u32;
```

String from sysinfo(SI_PLATFORM).

### `NT_AUXV`

```rust
const NT_AUXV: u32 = 6u32;
```

Contains copy of auxv array.

### `NT_GWINDOWS`

```rust
const NT_GWINDOWS: u32 = 7u32;
```

Contains copy of gwindows struct.

### `NT_ASRS`

```rust
const NT_ASRS: u32 = 8u32;
```

Contains copy of asrset struct.

### `NT_PSTATUS`

```rust
const NT_PSTATUS: u32 = 10u32;
```

Contains copy of pstatus struct.

### `NT_PSINFO`

```rust
const NT_PSINFO: u32 = 13u32;
```

Contains copy of psinfo struct.

### `NT_PRCRED`

```rust
const NT_PRCRED: u32 = 14u32;
```

Contains copy of prcred struct.

### `NT_UTSNAME`

```rust
const NT_UTSNAME: u32 = 15u32;
```

Contains copy of utsname struct.

### `NT_LWPSTATUS`

```rust
const NT_LWPSTATUS: u32 = 16u32;
```

Contains copy of lwpstatus struct.

### `NT_LWPSINFO`

```rust
const NT_LWPSINFO: u32 = 17u32;
```

Contains copy of lwpinfo struct.

### `NT_PRFPXREG`

```rust
const NT_PRFPXREG: u32 = 20u32;
```

Contains copy of fprxregset struct.

### `NT_SIGINFO`

```rust
const NT_SIGINFO: u32 = 1_397_311_305u32;
```

Contains copy of siginfo_t, size might increase.

### `NT_FILE`

```rust
const NT_FILE: u32 = 1_179_208_773u32;
```

Contains information about mapped files.

### `NT_PRXFPREG`

```rust
const NT_PRXFPREG: u32 = 1_189_489_535u32;
```

Contains copy of user_fxsr_struct.

### `NT_PPC_VMX`

```rust
const NT_PPC_VMX: u32 = 256u32;
```

PowerPC Altivec/VMX registers.

### `NT_PPC_SPE`

```rust
const NT_PPC_SPE: u32 = 257u32;
```

PowerPC SPE/EVR registers.

### `NT_PPC_VSX`

```rust
const NT_PPC_VSX: u32 = 258u32;
```

PowerPC VSX registers.

### `NT_PPC_TAR`

```rust
const NT_PPC_TAR: u32 = 259u32;
```

Target Address Register.

### `NT_PPC_PPR`

```rust
const NT_PPC_PPR: u32 = 260u32;
```

Program Priority Register.

### `NT_PPC_DSCR`

```rust
const NT_PPC_DSCR: u32 = 261u32;
```

Data Stream Control Register.

### `NT_PPC_EBB`

```rust
const NT_PPC_EBB: u32 = 262u32;
```

Event Based Branch Registers.

### `NT_PPC_PMU`

```rust
const NT_PPC_PMU: u32 = 263u32;
```

Performance Monitor Registers.

### `NT_PPC_TM_CGPR`

```rust
const NT_PPC_TM_CGPR: u32 = 264u32;
```

TM checkpointed GPR Registers.

### `NT_PPC_TM_CFPR`

```rust
const NT_PPC_TM_CFPR: u32 = 265u32;
```

TM checkpointed FPR Registers.

### `NT_PPC_TM_CVMX`

```rust
const NT_PPC_TM_CVMX: u32 = 266u32;
```

TM checkpointed VMX Registers.

### `NT_PPC_TM_CVSX`

```rust
const NT_PPC_TM_CVSX: u32 = 267u32;
```

TM checkpointed VSX Registers.

### `NT_PPC_TM_SPR`

```rust
const NT_PPC_TM_SPR: u32 = 268u32;
```

TM Special Purpose Registers.

### `NT_PPC_TM_CTAR`

```rust
const NT_PPC_TM_CTAR: u32 = 269u32;
```

TM checkpointed Target Address Register.

### `NT_PPC_TM_CPPR`

```rust
const NT_PPC_TM_CPPR: u32 = 270u32;
```

TM checkpointed Program Priority Register.

### `NT_PPC_TM_CDSCR`

```rust
const NT_PPC_TM_CDSCR: u32 = 271u32;
```

TM checkpointed Data Stream Control Register.

### `NT_PPC_PKEY`

```rust
const NT_PPC_PKEY: u32 = 272u32;
```

Memory Protection Keys registers.

### `NT_386_TLS`

```rust
const NT_386_TLS: u32 = 512u32;
```

i386 TLS slots (struct user_desc).

### `NT_386_IOPERM`

```rust
const NT_386_IOPERM: u32 = 513u32;
```

x86 io permission bitmap (1=deny).

### `NT_X86_XSTATE`

```rust
const NT_X86_XSTATE: u32 = 514u32;
```

x86 extended state using xsave.

### `NT_S390_HIGH_GPRS`

```rust
const NT_S390_HIGH_GPRS: u32 = 768u32;
```

s390 upper register halves.

### `NT_S390_TIMER`

```rust
const NT_S390_TIMER: u32 = 769u32;
```

s390 timer register.

### `NT_S390_TODCMP`

```rust
const NT_S390_TODCMP: u32 = 770u32;
```

s390 TOD clock comparator register.

### `NT_S390_TODPREG`

```rust
const NT_S390_TODPREG: u32 = 771u32;
```

s390 TOD programmable register.

### `NT_S390_CTRS`

```rust
const NT_S390_CTRS: u32 = 772u32;
```

s390 control registers.

### `NT_S390_PREFIX`

```rust
const NT_S390_PREFIX: u32 = 773u32;
```

s390 prefix register.

### `NT_S390_LAST_BREAK`

```rust
const NT_S390_LAST_BREAK: u32 = 774u32;
```

s390 breaking event address.

### `NT_S390_SYSTEM_CALL`

```rust
const NT_S390_SYSTEM_CALL: u32 = 775u32;
```

s390 system call restart data.

### `NT_S390_TDB`

```rust
const NT_S390_TDB: u32 = 776u32;
```

s390 transaction diagnostic block.

### `NT_S390_VXRS_LOW`

```rust
const NT_S390_VXRS_LOW: u32 = 777u32;
```

s390 vector registers 0-15 upper half.

### `NT_S390_VXRS_HIGH`

```rust
const NT_S390_VXRS_HIGH: u32 = 778u32;
```

s390 vector registers 16-31.

### `NT_S390_GS_CB`

```rust
const NT_S390_GS_CB: u32 = 779u32;
```

s390 guarded storage registers.

### `NT_S390_GS_BC`

```rust
const NT_S390_GS_BC: u32 = 780u32;
```

s390 guarded storage broadcast control block.

### `NT_S390_RI_CB`

```rust
const NT_S390_RI_CB: u32 = 781u32;
```

s390 runtime instrumentation.

### `NT_ARM_VFP`

```rust
const NT_ARM_VFP: u32 = 1_024u32;
```

ARM VFP/NEON registers.

### `NT_ARM_TLS`

```rust
const NT_ARM_TLS: u32 = 1_025u32;
```

ARM TLS register.

### `NT_ARM_HW_BREAK`

```rust
const NT_ARM_HW_BREAK: u32 = 1_026u32;
```

ARM hardware breakpoint registers.

### `NT_ARM_HW_WATCH`

```rust
const NT_ARM_HW_WATCH: u32 = 1_027u32;
```

ARM hardware watchpoint registers.

### `NT_ARM_SYSTEM_CALL`

```rust
const NT_ARM_SYSTEM_CALL: u32 = 1_028u32;
```

ARM system call number.

### `NT_ARM_SVE`

```rust
const NT_ARM_SVE: u32 = 1_029u32;
```

ARM Scalable Vector Extension registers.

### `NT_VMCOREDD`

```rust
const NT_VMCOREDD: u32 = 1_792u32;
```

Vmcore Device Dump Note.

### `NT_MIPS_DSP`

```rust
const NT_MIPS_DSP: u32 = 2_048u32;
```

MIPS DSP ASE registers.

### `NT_MIPS_FP_MODE`

```rust
const NT_MIPS_FP_MODE: u32 = 2_049u32;
```

MIPS floating-point mode.

### `NT_VERSION`

```rust
const NT_VERSION: u32 = 1u32;
```

Note type for version string.

This note may appear in object files.

It must be handled as a special case because it has no descriptor, and instead
uses the note name as the version string.

### `DT_NULL`

```rust
const DT_NULL: u32 = 0u32;
```

Marks end of dynamic section

### `DT_NEEDED`

```rust
const DT_NEEDED: u32 = 1u32;
```

Name of needed library

### `DT_PLTRELSZ`

```rust
const DT_PLTRELSZ: u32 = 2u32;
```

Size in bytes of PLT relocs

### `DT_PLTGOT`

```rust
const DT_PLTGOT: u32 = 3u32;
```

Processor defined value

### `DT_HASH`

```rust
const DT_HASH: u32 = 4u32;
```

Address of symbol hash table

### `DT_STRTAB`

```rust
const DT_STRTAB: u32 = 5u32;
```

Address of string table

### `DT_SYMTAB`

```rust
const DT_SYMTAB: u32 = 6u32;
```

Address of symbol table

### `DT_RELA`

```rust
const DT_RELA: u32 = 7u32;
```

Address of Rela relocs

### `DT_RELASZ`

```rust
const DT_RELASZ: u32 = 8u32;
```

Total size of Rela relocs

### `DT_RELAENT`

```rust
const DT_RELAENT: u32 = 9u32;
```

Size of one Rela reloc

### `DT_STRSZ`

```rust
const DT_STRSZ: u32 = 10u32;
```

Size of string table

### `DT_SYMENT`

```rust
const DT_SYMENT: u32 = 11u32;
```

Size of one symbol table entry

### `DT_INIT`

```rust
const DT_INIT: u32 = 12u32;
```

Address of init function

### `DT_FINI`

```rust
const DT_FINI: u32 = 13u32;
```

Address of termination function

### `DT_SONAME`

```rust
const DT_SONAME: u32 = 14u32;
```

Name of shared object

### `DT_RPATH`

```rust
const DT_RPATH: u32 = 15u32;
```

Library search path (deprecated)

### `DT_SYMBOLIC`

```rust
const DT_SYMBOLIC: u32 = 16u32;
```

Start symbol search here

### `DT_REL`

```rust
const DT_REL: u32 = 17u32;
```

Address of Rel relocs

### `DT_RELSZ`

```rust
const DT_RELSZ: u32 = 18u32;
```

Total size of Rel relocs

### `DT_RELENT`

```rust
const DT_RELENT: u32 = 19u32;
```

Size of one Rel reloc

### `DT_PLTREL`

```rust
const DT_PLTREL: u32 = 20u32;
```

Type of reloc in PLT

### `DT_DEBUG`

```rust
const DT_DEBUG: u32 = 21u32;
```

For debugging; unspecified

### `DT_TEXTREL`

```rust
const DT_TEXTREL: u32 = 22u32;
```

Reloc might modify .text

### `DT_JMPREL`

```rust
const DT_JMPREL: u32 = 23u32;
```

Address of PLT relocs

### `DT_BIND_NOW`

```rust
const DT_BIND_NOW: u32 = 24u32;
```

Process relocations of object

### `DT_INIT_ARRAY`

```rust
const DT_INIT_ARRAY: u32 = 25u32;
```

Array with addresses of init fct

### `DT_FINI_ARRAY`

```rust
const DT_FINI_ARRAY: u32 = 26u32;
```

Array with addresses of fini fct

### `DT_INIT_ARRAYSZ`

```rust
const DT_INIT_ARRAYSZ: u32 = 27u32;
```

Size in bytes of DT_INIT_ARRAY

### `DT_FINI_ARRAYSZ`

```rust
const DT_FINI_ARRAYSZ: u32 = 28u32;
```

Size in bytes of DT_FINI_ARRAY

### `DT_RUNPATH`

```rust
const DT_RUNPATH: u32 = 29u32;
```

Library search path

### `DT_FLAGS`

```rust
const DT_FLAGS: u32 = 30u32;
```

Flags for the object being loaded

### `DT_ENCODING`

```rust
const DT_ENCODING: u32 = 32u32;
```

Start of encoded range

### `DT_PREINIT_ARRAY`

```rust
const DT_PREINIT_ARRAY: u32 = 32u32;
```

Array with addresses of preinit fct

### `DT_PREINIT_ARRAYSZ`

```rust
const DT_PREINIT_ARRAYSZ: u32 = 33u32;
```

size in bytes of DT_PREINIT_ARRAY

### `DT_SYMTAB_SHNDX`

```rust
const DT_SYMTAB_SHNDX: u32 = 34u32;
```

Address of SYMTAB_SHNDX section

### `DT_LOOS`

```rust
const DT_LOOS: u32 = 1_610_612_749u32;
```

Start of OS-specific

### `DT_HIOS`

```rust
const DT_HIOS: u32 = 1_879_044_096u32;
```

End of OS-specific

### `DT_LOPROC`

```rust
const DT_LOPROC: u32 = 1_879_048_192u32;
```

Start of processor-specific

### `DT_HIPROC`

```rust
const DT_HIPROC: u32 = 2_147_483_647u32;
```

End of processor-specific

### `DT_VALRNGLO`

```rust
const DT_VALRNGLO: u32 = 1_879_047_424u32;
```

### `DT_GNU_PRELINKED`

```rust
const DT_GNU_PRELINKED: u32 = 1_879_047_669u32;
```

Prelinking timestamp

### `DT_GNU_CONFLICTSZ`

```rust
const DT_GNU_CONFLICTSZ: u32 = 1_879_047_670u32;
```

Size of conflict section

### `DT_GNU_LIBLISTSZ`

```rust
const DT_GNU_LIBLISTSZ: u32 = 1_879_047_671u32;
```

Size of library list

### `DT_CHECKSUM`

```rust
const DT_CHECKSUM: u32 = 1_879_047_672u32;
```

### `DT_PLTPADSZ`

```rust
const DT_PLTPADSZ: u32 = 1_879_047_673u32;
```

### `DT_MOVEENT`

```rust
const DT_MOVEENT: u32 = 1_879_047_674u32;
```

### `DT_MOVESZ`

```rust
const DT_MOVESZ: u32 = 1_879_047_675u32;
```

### `DT_FEATURE_1`

```rust
const DT_FEATURE_1: u32 = 1_879_047_676u32;
```

Feature selection (DTF_*).

### `DT_POSFLAG_1`

```rust
const DT_POSFLAG_1: u32 = 1_879_047_677u32;
```

Flags for DT_* entries, affecting the following DT_* entry.

### `DT_SYMINSZ`

```rust
const DT_SYMINSZ: u32 = 1_879_047_678u32;
```

Size of syminfo table (in bytes)

### `DT_SYMINENT`

```rust
const DT_SYMINENT: u32 = 1_879_047_679u32;
```

Entry size of syminfo

### `DT_VALRNGHI`

```rust
const DT_VALRNGHI: u32 = 1_879_047_679u32;
```

### `DT_ADDRRNGLO`

```rust
const DT_ADDRRNGLO: u32 = 1_879_047_680u32;
```

### `DT_GNU_HASH`

```rust
const DT_GNU_HASH: u32 = 1_879_047_925u32;
```

GNU-style hash table.

### `DT_TLSDESC_PLT`

```rust
const DT_TLSDESC_PLT: u32 = 1_879_047_926u32;
```

### `DT_TLSDESC_GOT`

```rust
const DT_TLSDESC_GOT: u32 = 1_879_047_927u32;
```

### `DT_GNU_CONFLICT`

```rust
const DT_GNU_CONFLICT: u32 = 1_879_047_928u32;
```

Start of conflict section

### `DT_GNU_LIBLIST`

```rust
const DT_GNU_LIBLIST: u32 = 1_879_047_929u32;
```

Library list

### `DT_CONFIG`

```rust
const DT_CONFIG: u32 = 1_879_047_930u32;
```

Configuration information.

### `DT_DEPAUDIT`

```rust
const DT_DEPAUDIT: u32 = 1_879_047_931u32;
```

Dependency auditing.

### `DT_AUDIT`

```rust
const DT_AUDIT: u32 = 1_879_047_932u32;
```

Object auditing.

### `DT_PLTPAD`

```rust
const DT_PLTPAD: u32 = 1_879_047_933u32;
```

PLT padding.

### `DT_MOVETAB`

```rust
const DT_MOVETAB: u32 = 1_879_047_934u32;
```

Move table.

### `DT_SYMINFO`

```rust
const DT_SYMINFO: u32 = 1_879_047_935u32;
```

Syminfo table.

### `DT_ADDRRNGHI`

```rust
const DT_ADDRRNGHI: u32 = 1_879_047_935u32;
```

### `DT_VERSYM`

```rust
const DT_VERSYM: u32 = 1_879_048_176u32;
```

### `DT_RELACOUNT`

```rust
const DT_RELACOUNT: u32 = 1_879_048_185u32;
```

### `DT_RELCOUNT`

```rust
const DT_RELCOUNT: u32 = 1_879_048_186u32;
```

### `DT_FLAGS_1`

```rust
const DT_FLAGS_1: u32 = 1_879_048_187u32;
```

State flags, see DF_1_* below.

### `DT_VERDEF`

```rust
const DT_VERDEF: u32 = 1_879_048_188u32;
```

Address of version definition table

### `DT_VERDEFNUM`

```rust
const DT_VERDEFNUM: u32 = 1_879_048_189u32;
```

Number of version definitions

### `DT_VERNEED`

```rust
const DT_VERNEED: u32 = 1_879_048_190u32;
```

Address of table with needed versions

### `DT_VERNEEDNUM`

```rust
const DT_VERNEEDNUM: u32 = 1_879_048_191u32;
```

Number of needed versions

### `DT_AUXILIARY`

```rust
const DT_AUXILIARY: u32 = 2_147_483_645u32;
```

Shared object to load before self

### `DT_FILTER`

```rust
const DT_FILTER: u32 = 2_147_483_647u32;
```

Shared object to get values from

### `DF_ORIGIN`

```rust
const DF_ORIGIN: u32 = 1u32;
```

Object may use DF_ORIGIN

### `DF_SYMBOLIC`

```rust
const DF_SYMBOLIC: u32 = 2u32;
```

Symbol resolutions starts here

### `DF_TEXTREL`

```rust
const DF_TEXTREL: u32 = 4u32;
```

Object contains text relocations

### `DF_BIND_NOW`

```rust
const DF_BIND_NOW: u32 = 8u32;
```

No lazy binding for this object

### `DF_STATIC_TLS`

```rust
const DF_STATIC_TLS: u32 = 16u32;
```

Module uses the static TLS model

### `DF_1_NOW`

```rust
const DF_1_NOW: u32 = 1u32;
```

Set RTLD_NOW for this object.

### `DF_1_GLOBAL`

```rust
const DF_1_GLOBAL: u32 = 2u32;
```

Set RTLD_GLOBAL for this object.

### `DF_1_GROUP`

```rust
const DF_1_GROUP: u32 = 4u32;
```

Set RTLD_GROUP for this object.

### `DF_1_NODELETE`

```rust
const DF_1_NODELETE: u32 = 8u32;
```

Set RTLD_NODELETE for this object.

### `DF_1_LOADFLTR`

```rust
const DF_1_LOADFLTR: u32 = 16u32;
```

Trigger filtee loading at runtime.

### `DF_1_INITFIRST`

```rust
const DF_1_INITFIRST: u32 = 32u32;
```

Set RTLD_INITFIRST for this object.

### `DF_1_NOOPEN`

```rust
const DF_1_NOOPEN: u32 = 64u32;
```

Set RTLD_NOOPEN for this object.

### `DF_1_ORIGIN`

```rust
const DF_1_ORIGIN: u32 = 128u32;
```

$ORIGIN must be handled.

### `DF_1_DIRECT`

```rust
const DF_1_DIRECT: u32 = 256u32;
```

Direct binding enabled.

### `DF_1_TRANS`

```rust
const DF_1_TRANS: u32 = 512u32;
```

### `DF_1_INTERPOSE`

```rust
const DF_1_INTERPOSE: u32 = 1_024u32;
```

Object is used to interpose.

### `DF_1_NODEFLIB`

```rust
const DF_1_NODEFLIB: u32 = 2_048u32;
```

Ignore default lib search path.

### `DF_1_NODUMP`

```rust
const DF_1_NODUMP: u32 = 4_096u32;
```

Object can't be dldump'ed.

### `DF_1_CONFALT`

```rust
const DF_1_CONFALT: u32 = 8_192u32;
```

Configuration alternative created.

### `DF_1_ENDFILTEE`

```rust
const DF_1_ENDFILTEE: u32 = 16_384u32;
```

Filtee terminates filters search.

### `DF_1_DISPRELDNE`

```rust
const DF_1_DISPRELDNE: u32 = 32_768u32;
```

Disp reloc applied at build time.

### `DF_1_DISPRELPND`

```rust
const DF_1_DISPRELPND: u32 = 65_536u32;
```

Disp reloc applied at run-time.

### `DF_1_NODIRECT`

```rust
const DF_1_NODIRECT: u32 = 131_072u32;
```

Object has no-direct binding.

### `DF_1_IGNMULDEF`

```rust
const DF_1_IGNMULDEF: u32 = 262_144u32;
```

### `DF_1_NOKSYMS`

```rust
const DF_1_NOKSYMS: u32 = 524_288u32;
```

### `DF_1_NOHDR`

```rust
const DF_1_NOHDR: u32 = 1_048_576u32;
```

### `DF_1_EDITED`

```rust
const DF_1_EDITED: u32 = 2_097_152u32;
```

Object is modified after built.

### `DF_1_NORELOC`

```rust
const DF_1_NORELOC: u32 = 4_194_304u32;
```

### `DF_1_SYMINTPOSE`

```rust
const DF_1_SYMINTPOSE: u32 = 8_388_608u32;
```

Object has individual interposers.

### `DF_1_GLOBAUDIT`

```rust
const DF_1_GLOBAUDIT: u32 = 16_777_216u32;
```

Global auditing required.

### `DF_1_SINGLETON`

```rust
const DF_1_SINGLETON: u32 = 33_554_432u32;
```

Singleton symbols are used.

### `DF_1_STUB`

```rust
const DF_1_STUB: u32 = 67_108_864u32;
```

### `DF_1_PIE`

```rust
const DF_1_PIE: u32 = 134_217_728u32;
```

### `VERSYM_HIDDEN`

```rust
const VERSYM_HIDDEN: u16 = 32_768u16;
```

Symbol is hidden.

### `VERSYM_VERSION`

```rust
const VERSYM_VERSION: u16 = 32_767u16;
```

Symbol version index.

### `VER_DEF_NONE`

```rust
const VER_DEF_NONE: u16 = 0u16;
```

No version

### `VER_DEF_CURRENT`

```rust
const VER_DEF_CURRENT: u16 = 1u16;
```

Current version

### `VER_FLG_BASE`

```rust
const VER_FLG_BASE: u16 = 1u16;
```

Version definition of file itself

### `VER_FLG_WEAK`

```rust
const VER_FLG_WEAK: u16 = 2u16;
```

Weak version identifier

### `VER_NDX_LOCAL`

```rust
const VER_NDX_LOCAL: u16 = 0u16;
```

Symbol is local.

### `VER_NDX_GLOBAL`

```rust
const VER_NDX_GLOBAL: u16 = 1u16;
```

Symbol is global.

### `VER_NEED_NONE`

```rust
const VER_NEED_NONE: u16 = 0u16;
```

No version

### `VER_NEED_CURRENT`

```rust
const VER_NEED_CURRENT: u16 = 1u16;
```

Current version

### `ELF_NOTE_SOLARIS`

```rust
const ELF_NOTE_SOLARIS: &[u8];
```

Solaris entries in the note section have this name.

### `NT_SOLARIS_PAGESIZE_HINT`

```rust
const NT_SOLARIS_PAGESIZE_HINT: u32 = 1u32;
```

Desired pagesize for the binary.

### `ELF_NOTE_GNU`

```rust
const ELF_NOTE_GNU: &[u8];
```

GNU entries in the note section have this name.

### `ELF_NOTE_GO`

```rust
const ELF_NOTE_GO: &[u8];
```

Go entries in the note section have this name.

### `NT_GNU_ABI_TAG`

```rust
const NT_GNU_ABI_TAG: u32 = 1u32;
```

ABI information.

The descriptor consists of words:
- word 0: OS descriptor
- word 1: major version of the ABI
- word 2: minor version of the ABI
- word 3: subminor version of the ABI

### `ELF_NOTE_OS_LINUX`

```rust
const ELF_NOTE_OS_LINUX: u32 = 0u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `ELF_NOTE_OS_GNU`

```rust
const ELF_NOTE_OS_GNU: u32 = 1u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `ELF_NOTE_OS_SOLARIS2`

```rust
const ELF_NOTE_OS_SOLARIS2: u32 = 2u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `ELF_NOTE_OS_FREEBSD`

```rust
const ELF_NOTE_OS_FREEBSD: u32 = 3u32;
```

OS descriptor for `NT_GNU_ABI_TAG`.

### `NT_GNU_HWCAP`

```rust
const NT_GNU_HWCAP: u32 = 2u32;
```

Synthetic hwcap information.

The descriptor begins with two words:
- word 0: number of entries
- word 1: bitmask of enabled entries

Then follow variable-length entries, one byte followed by a
'\0'-terminated hwcap name string.  The byte gives the bit
number to test if enabled, (1U << bit) & bitmask.  */

### `NT_GNU_BUILD_ID`

```rust
const NT_GNU_BUILD_ID: u32 = 3u32;
```

Build ID bits as generated by `ld --build-id`.

The descriptor consists of any nonzero number of bytes.

### `NT_GO_BUILD_ID`

```rust
const NT_GO_BUILD_ID: u32 = 4u32;
```

Build ID bits as generated by Go's gc compiler.

The descriptor consists of any nonzero number of bytes.

### `NT_GNU_GOLD_VERSION`

```rust
const NT_GNU_GOLD_VERSION: u32 = 4u32;
```

Version note generated by GNU gold containing a version string.

### `NT_GNU_PROPERTY_TYPE_0`

```rust
const NT_GNU_PROPERTY_TYPE_0: u32 = 5u32;
```

Program property.

### `GNU_PROPERTY_STACK_SIZE`

```rust
const GNU_PROPERTY_STACK_SIZE: u32 = 1u32;
```

Stack size.

### `GNU_PROPERTY_NO_COPY_ON_PROTECTED`

```rust
const GNU_PROPERTY_NO_COPY_ON_PROTECTED: u32 = 2u32;
```

No copy relocation on protected data symbol.

### `GNU_PROPERTY_UINT32_AND_LO`

```rust
const GNU_PROPERTY_UINT32_AND_LO: u32 = 2_952_790_016u32;
```

### `GNU_PROPERTY_UINT32_AND_HI`

```rust
const GNU_PROPERTY_UINT32_AND_HI: u32 = 2_952_822_783u32;
```

### `GNU_PROPERTY_UINT32_OR_LO`

```rust
const GNU_PROPERTY_UINT32_OR_LO: u32 = 2_952_822_784u32;
```

### `GNU_PROPERTY_UINT32_OR_HI`

```rust
const GNU_PROPERTY_UINT32_OR_HI: u32 = 2_952_855_551u32;
```

### `GNU_PROPERTY_1_NEEDED`

```rust
const GNU_PROPERTY_1_NEEDED: u32 = 2_952_822_784u32;
```

The needed properties by the object file.  */

### `GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS`

```rust
const GNU_PROPERTY_1_NEEDED_INDIRECT_EXTERN_ACCESS: u32 = 1u32;
```

Set if the object file requires canonical function pointers and
cannot be used with copy relocation.

### `GNU_PROPERTY_LOPROC`

```rust
const GNU_PROPERTY_LOPROC: u32 = 3_221_225_472u32;
```

Processor-specific semantics, lo

### `GNU_PROPERTY_HIPROC`

```rust
const GNU_PROPERTY_HIPROC: u32 = 3_758_096_383u32;
```

Processor-specific semantics, hi

### `GNU_PROPERTY_LOUSER`

```rust
const GNU_PROPERTY_LOUSER: u32 = 3_758_096_384u32;
```

Application-specific semantics, lo

### `GNU_PROPERTY_HIUSER`

```rust
const GNU_PROPERTY_HIUSER: u32 = 4_294_967_295u32;
```

Application-specific semantics, hi

### `GNU_PROPERTY_AARCH64_FEATURE_1_AND`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_1_AND: u32 = 3_221_225_472u32;
```

AArch64 specific GNU properties.

### `GNU_PROPERTY_AARCH64_FEATURE_PAUTH`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_PAUTH: u32 = 3_221_225_473u32;
```

### `GNU_PROPERTY_AARCH64_FEATURE_1_BTI`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_1_BTI: u32 = 1u32;
```

### `GNU_PROPERTY_AARCH64_FEATURE_1_PAC`

```rust
const GNU_PROPERTY_AARCH64_FEATURE_1_PAC: u32 = 2u32;
```

### `GNU_PROPERTY_X86_UINT32_AND_LO`

```rust
const GNU_PROPERTY_X86_UINT32_AND_LO: u32 = 3_221_225_474u32;
```

### `GNU_PROPERTY_X86_UINT32_AND_HI`

```rust
const GNU_PROPERTY_X86_UINT32_AND_HI: u32 = 3_221_258_239u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_LO`

```rust
const GNU_PROPERTY_X86_UINT32_OR_LO: u32 = 3_221_258_240u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_HI`

```rust
const GNU_PROPERTY_X86_UINT32_OR_HI: u32 = 3_221_291_007u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_AND_LO`

```rust
const GNU_PROPERTY_X86_UINT32_OR_AND_LO: u32 = 3_221_291_008u32;
```

### `GNU_PROPERTY_X86_UINT32_OR_AND_HI`

```rust
const GNU_PROPERTY_X86_UINT32_OR_AND_HI: u32 = 3_221_323_775u32;
```

### `GNU_PROPERTY_X86_ISA_1_USED`

```rust
const GNU_PROPERTY_X86_ISA_1_USED: u32 = 3_221_291_010u32;
```

The x86 instruction sets indicated by the corresponding bits are
used in program.  Their support in the hardware is optional.

### `GNU_PROPERTY_X86_ISA_1_NEEDED`

```rust
const GNU_PROPERTY_X86_ISA_1_NEEDED: u32 = 3_221_258_242u32;
```

The x86 instruction sets indicated by the corresponding bits are
used in program and they must be supported by the hardware.

### `GNU_PROPERTY_X86_FEATURE_1_AND`

```rust
const GNU_PROPERTY_X86_FEATURE_1_AND: u32 = 3_221_225_474u32;
```

X86 processor-specific features used in program.

### `GNU_PROPERTY_X86_ISA_1_BASELINE`

```rust
const GNU_PROPERTY_X86_ISA_1_BASELINE: u32 = 1u32;
```

GNU_PROPERTY_X86_ISA_1_BASELINE: CMOV, CX8 (cmpxchg8b), FPU (fld),
MMX, OSFXSR (fxsave), SCE (syscall), SSE and SSE2.

### `GNU_PROPERTY_X86_ISA_1_V2`

```rust
const GNU_PROPERTY_X86_ISA_1_V2: u32 = 2u32;
```

GNU_PROPERTY_X86_ISA_1_V2: GNU_PROPERTY_X86_ISA_1_BASELINE,
CMPXCHG16B (cmpxchg16b), LAHF-SAHF (lahf), POPCNT (popcnt), SSE3,
SSSE3, SSE4.1 and SSE4.2.

### `GNU_PROPERTY_X86_ISA_1_V3`

```rust
const GNU_PROPERTY_X86_ISA_1_V3: u32 = 4u32;
```

GNU_PROPERTY_X86_ISA_1_V3: GNU_PROPERTY_X86_ISA_1_V2, AVX, AVX2, BMI1,
BMI2, F16C, FMA, LZCNT, MOVBE, XSAVE.

### `GNU_PROPERTY_X86_ISA_1_V4`

```rust
const GNU_PROPERTY_X86_ISA_1_V4: u32 = 8u32;
```

GNU_PROPERTY_X86_ISA_1_V4: GNU_PROPERTY_X86_ISA_1_V3, AVX512F,
AVX512BW, AVX512CD, AVX512DQ and AVX512VL.

### `GNU_PROPERTY_X86_FEATURE_1_IBT`

```rust
const GNU_PROPERTY_X86_FEATURE_1_IBT: u32 = 1u32;
```

This indicates that all executable sections are compatible with IBT.

### `GNU_PROPERTY_X86_FEATURE_1_SHSTK`

```rust
const GNU_PROPERTY_X86_FEATURE_1_SHSTK: u32 = 2u32;
```

This indicates that all executable sections are compatible with SHSTK.

### `R_68K_NONE`

```rust
const R_68K_NONE: u32 = 0u32;
```

No reloc

### `R_68K_32`

```rust
const R_68K_32: u32 = 1u32;
```

Direct 32 bit

### `R_68K_16`

```rust
const R_68K_16: u32 = 2u32;
```

Direct 16 bit

### `R_68K_8`

```rust
const R_68K_8: u32 = 3u32;
```

Direct 8 bit

### `R_68K_PC32`

```rust
const R_68K_PC32: u32 = 4u32;
```

PC relative 32 bit

### `R_68K_PC16`

```rust
const R_68K_PC16: u32 = 5u32;
```

PC relative 16 bit

### `R_68K_PC8`

```rust
const R_68K_PC8: u32 = 6u32;
```

PC relative 8 bit

### `R_68K_GOT32`

```rust
const R_68K_GOT32: u32 = 7u32;
```

32 bit PC relative GOT entry

### `R_68K_GOT16`

```rust
const R_68K_GOT16: u32 = 8u32;
```

16 bit PC relative GOT entry

### `R_68K_GOT8`

```rust
const R_68K_GOT8: u32 = 9u32;
```

8 bit PC relative GOT entry

### `R_68K_GOT32O`

```rust
const R_68K_GOT32O: u32 = 10u32;
```

32 bit GOT offset

### `R_68K_GOT16O`

```rust
const R_68K_GOT16O: u32 = 11u32;
```

16 bit GOT offset

### `R_68K_GOT8O`

```rust
const R_68K_GOT8O: u32 = 12u32;
```

8 bit GOT offset

### `R_68K_PLT32`

```rust
const R_68K_PLT32: u32 = 13u32;
```

32 bit PC relative PLT address

### `R_68K_PLT16`

```rust
const R_68K_PLT16: u32 = 14u32;
```

16 bit PC relative PLT address

### `R_68K_PLT8`

```rust
const R_68K_PLT8: u32 = 15u32;
```

8 bit PC relative PLT address

### `R_68K_PLT32O`

```rust
const R_68K_PLT32O: u32 = 16u32;
```

32 bit PLT offset

### `R_68K_PLT16O`

```rust
const R_68K_PLT16O: u32 = 17u32;
```

16 bit PLT offset

### `R_68K_PLT8O`

```rust
const R_68K_PLT8O: u32 = 18u32;
```

8 bit PLT offset

### `R_68K_COPY`

```rust
const R_68K_COPY: u32 = 19u32;
```

Copy symbol at runtime

### `R_68K_GLOB_DAT`

```rust
const R_68K_GLOB_DAT: u32 = 20u32;
```

Create GOT entry

### `R_68K_JMP_SLOT`

```rust
const R_68K_JMP_SLOT: u32 = 21u32;
```

Create PLT entry

### `R_68K_RELATIVE`

```rust
const R_68K_RELATIVE: u32 = 22u32;
```

Adjust by program base

### `R_68K_TLS_GD32`

```rust
const R_68K_TLS_GD32: u32 = 25u32;
```

32 bit GOT offset for GD

### `R_68K_TLS_GD16`

```rust
const R_68K_TLS_GD16: u32 = 26u32;
```

16 bit GOT offset for GD

### `R_68K_TLS_GD8`

```rust
const R_68K_TLS_GD8: u32 = 27u32;
```

8 bit GOT offset for GD

### `R_68K_TLS_LDM32`

```rust
const R_68K_TLS_LDM32: u32 = 28u32;
```

32 bit GOT offset for LDM

### `R_68K_TLS_LDM16`

```rust
const R_68K_TLS_LDM16: u32 = 29u32;
```

16 bit GOT offset for LDM

### `R_68K_TLS_LDM8`

```rust
const R_68K_TLS_LDM8: u32 = 30u32;
```

8 bit GOT offset for LDM

### `R_68K_TLS_LDO32`

```rust
const R_68K_TLS_LDO32: u32 = 31u32;
```

32 bit module-relative offset

### `R_68K_TLS_LDO16`

```rust
const R_68K_TLS_LDO16: u32 = 32u32;
```

16 bit module-relative offset

### `R_68K_TLS_LDO8`

```rust
const R_68K_TLS_LDO8: u32 = 33u32;
```

8 bit module-relative offset

### `R_68K_TLS_IE32`

```rust
const R_68K_TLS_IE32: u32 = 34u32;
```

32 bit GOT offset for IE

### `R_68K_TLS_IE16`

```rust
const R_68K_TLS_IE16: u32 = 35u32;
```

16 bit GOT offset for IE

### `R_68K_TLS_IE8`

```rust
const R_68K_TLS_IE8: u32 = 36u32;
```

8 bit GOT offset for IE

### `R_68K_TLS_LE32`

```rust
const R_68K_TLS_LE32: u32 = 37u32;
```

32 bit offset relative to static TLS block

### `R_68K_TLS_LE16`

```rust
const R_68K_TLS_LE16: u32 = 38u32;
```

16 bit offset relative to static TLS block

### `R_68K_TLS_LE8`

```rust
const R_68K_TLS_LE8: u32 = 39u32;
```

8 bit offset relative to static TLS block

### `R_68K_TLS_DTPMOD32`

```rust
const R_68K_TLS_DTPMOD32: u32 = 40u32;
```

32 bit module number

### `R_68K_TLS_DTPREL32`

```rust
const R_68K_TLS_DTPREL32: u32 = 41u32;
```

32 bit module-relative offset

### `R_68K_TLS_TPREL32`

```rust
const R_68K_TLS_TPREL32: u32 = 42u32;
```

32 bit TP-relative offset

### `R_386_NONE`

```rust
const R_386_NONE: u32 = 0u32;
```

No reloc

### `R_386_32`

```rust
const R_386_32: u32 = 1u32;
```

Direct 32 bit

### `R_386_PC32`

```rust
const R_386_PC32: u32 = 2u32;
```

PC relative 32 bit

### `R_386_GOT32`

```rust
const R_386_GOT32: u32 = 3u32;
```

32 bit GOT entry

### `R_386_PLT32`

```rust
const R_386_PLT32: u32 = 4u32;
```

32 bit PLT address

### `R_386_COPY`

```rust
const R_386_COPY: u32 = 5u32;
```

Copy symbol at runtime

### `R_386_GLOB_DAT`

```rust
const R_386_GLOB_DAT: u32 = 6u32;
```

Create GOT entry

### `R_386_JMP_SLOT`

```rust
const R_386_JMP_SLOT: u32 = 7u32;
```

Create PLT entry

### `R_386_RELATIVE`

```rust
const R_386_RELATIVE: u32 = 8u32;
```

Adjust by program base

### `R_386_GOTOFF`

```rust
const R_386_GOTOFF: u32 = 9u32;
```

32 bit offset to GOT

### `R_386_GOTPC`

```rust
const R_386_GOTPC: u32 = 10u32;
```

32 bit PC relative offset to GOT

### `R_386_32PLT`

```rust
const R_386_32PLT: u32 = 11u32;
```

Direct 32 bit PLT address

### `R_386_TLS_TPOFF`

```rust
const R_386_TLS_TPOFF: u32 = 14u32;
```

Offset in static TLS block

### `R_386_TLS_IE`

```rust
const R_386_TLS_IE: u32 = 15u32;
```

Address of GOT entry for static TLS block offset

### `R_386_TLS_GOTIE`

```rust
const R_386_TLS_GOTIE: u32 = 16u32;
```

GOT entry for static TLS block offset

### `R_386_TLS_LE`

```rust
const R_386_TLS_LE: u32 = 17u32;
```

Offset relative to static TLS block

### `R_386_TLS_GD`

```rust
const R_386_TLS_GD: u32 = 18u32;
```

Direct 32 bit for GNU version of general dynamic thread local data

### `R_386_TLS_LDM`

```rust
const R_386_TLS_LDM: u32 = 19u32;
```

Direct 32 bit for GNU version of local dynamic thread local data in LE code

### `R_386_16`

```rust
const R_386_16: u32 = 20u32;
```

Direct 16 bit

### `R_386_PC16`

```rust
const R_386_PC16: u32 = 21u32;
```

PC relative 16 bit

### `R_386_8`

```rust
const R_386_8: u32 = 22u32;
```

Direct 8 bit

### `R_386_PC8`

```rust
const R_386_PC8: u32 = 23u32;
```

PC relative 8 bit

### `R_386_TLS_GD_32`

```rust
const R_386_TLS_GD_32: u32 = 24u32;
```

Direct 32 bit for general dynamic thread local data

### `R_386_TLS_GD_PUSH`

```rust
const R_386_TLS_GD_PUSH: u32 = 25u32;
```

Tag for pushl in GD TLS code

### `R_386_TLS_GD_CALL`

```rust
const R_386_TLS_GD_CALL: u32 = 26u32;
```

Relocation for call to __tls_get_addr()

### `R_386_TLS_GD_POP`

```rust
const R_386_TLS_GD_POP: u32 = 27u32;
```

Tag for popl in GD TLS code

### `R_386_TLS_LDM_32`

```rust
const R_386_TLS_LDM_32: u32 = 28u32;
```

Direct 32 bit for local dynamic thread local data in LE code

### `R_386_TLS_LDM_PUSH`

```rust
const R_386_TLS_LDM_PUSH: u32 = 29u32;
```

Tag for pushl in LDM TLS code

### `R_386_TLS_LDM_CALL`

```rust
const R_386_TLS_LDM_CALL: u32 = 30u32;
```

Relocation for call to __tls_get_addr() in LDM code

### `R_386_TLS_LDM_POP`

```rust
const R_386_TLS_LDM_POP: u32 = 31u32;
```

Tag for popl in LDM TLS code

### `R_386_TLS_LDO_32`

```rust
const R_386_TLS_LDO_32: u32 = 32u32;
```

Offset relative to TLS block

### `R_386_TLS_IE_32`

```rust
const R_386_TLS_IE_32: u32 = 33u32;
```

GOT entry for negated static TLS block offset

### `R_386_TLS_LE_32`

```rust
const R_386_TLS_LE_32: u32 = 34u32;
```

Negated offset relative to static TLS block

### `R_386_TLS_DTPMOD32`

```rust
const R_386_TLS_DTPMOD32: u32 = 35u32;
```

ID of module containing symbol

### `R_386_TLS_DTPOFF32`

```rust
const R_386_TLS_DTPOFF32: u32 = 36u32;
```

Offset in TLS block

### `R_386_TLS_TPOFF32`

```rust
const R_386_TLS_TPOFF32: u32 = 37u32;
```

Negated offset in static TLS block

### `R_386_SIZE32`

```rust
const R_386_SIZE32: u32 = 38u32;
```

32-bit symbol size

### `R_386_TLS_GOTDESC`

```rust
const R_386_TLS_GOTDESC: u32 = 39u32;
```

GOT offset for TLS descriptor.

### `R_386_TLS_DESC_CALL`

```rust
const R_386_TLS_DESC_CALL: u32 = 40u32;
```

Marker of call through TLS descriptor for relaxation.

### `R_386_TLS_DESC`

```rust
const R_386_TLS_DESC: u32 = 41u32;
```

TLS descriptor containing pointer to code and to argument, returning the TLS offset for the symbol.

### `R_386_IRELATIVE`

```rust
const R_386_IRELATIVE: u32 = 42u32;
```

Adjust indirectly by program base

### `R_386_GOT32X`

```rust
const R_386_GOT32X: u32 = 43u32;
```

Load from 32 bit GOT entry, relaxable.

### `R_SHARC_ADDR24_V3`

```rust
const R_SHARC_ADDR24_V3: u32 = 11u32;
```

24-bit absolute address in bits 23:0 of a 48-bit instr

Targets:

* Type 25a (PC_DIRECT)

### `R_SHARC_ADDR32_V3`

```rust
const R_SHARC_ADDR32_V3: u32 = 12u32;
```

32-bit absolute address in bits 31:0 of a 48-bit instr

Targets:

* Type 14a
* Type 14d
* Type 15a
* Type 16a
* Type 17a
* Type 18a
* Type 19a

### `R_SHARC_ADDR_VAR_V3`

```rust
const R_SHARC_ADDR_VAR_V3: u32 = 13u32;
```

32-bit absolute address in bits 31:0 of a 32-bit data location

Represented with `RelocationEncoding::Generic`

### `R_SHARC_PCRSHORT_V3`

```rust
const R_SHARC_PCRSHORT_V3: u32 = 14u32;
```

6-bit PC-relative address in bits 32:27 of a 48-bit instr

Targets:

* Type 9a
* Type 10a

### `R_SHARC_PCRLONG_V3`

```rust
const R_SHARC_PCRLONG_V3: u32 = 15u32;
```

24-bit PC-relative address in bits 23:0 of a 48-bit instr

Targets:

* Type 8a
* Type 12a (truncated to 23 bits after relocation)
* Type 13a (truncated to 23 bits after relocation)
* Type 25a (PC Relative)

### `R_SHARC_DATA6_V3`

```rust
const R_SHARC_DATA6_V3: u32 = 16u32;
```

6-bit absolute address in bits 32:27 of a 48-bit instr

Targets:

* Type 4a
* Type 4b
* Type 4d

### `R_SHARC_DATA16_V3`

```rust
const R_SHARC_DATA16_V3: u32 = 17u32;
```

16-bit absolute address in bits 39:24 of a 48-bit instr

Targets:

* Type 12a

### `R_SHARC_DATA6_VISA_V3`

```rust
const R_SHARC_DATA6_VISA_V3: u32 = 18u32;
```

6-bit absolute address into bits 16:11 of a 32-bit instr

Targets:

* Type 4b

### `R_SHARC_DATA7_VISA_V3`

```rust
const R_SHARC_DATA7_VISA_V3: u32 = 19u32;
```

7-bit absolute address into bits 6:0 of a 32-bit instr

### `R_SHARC_DATA16_VISA_V3`

```rust
const R_SHARC_DATA16_VISA_V3: u32 = 20u32;
```

16-bit absolute address into bits 15:0 of a 32-bit instr

### `R_SHARC_PCR6_VISA_V3`

```rust
const R_SHARC_PCR6_VISA_V3: u32 = 23u32;
```

6-bit PC-relative address into bits 16:11 of a Type B

Targets:

* Type 9b

### `R_SHARC_ADDR_VAR16_V3`

```rust
const R_SHARC_ADDR_VAR16_V3: u32 = 25u32;
```

16-bit absolute address into bits 15:0 of a 16-bit location.

Represented with `RelocationEncoding::Generic`

### `R_SHARC_CALC_PUSH_ADDR`

```rust
const R_SHARC_CALC_PUSH_ADDR: u32 = 224u32;
```

### `R_SHARC_CALC_PUSH_ADDEND`

```rust
const R_SHARC_CALC_PUSH_ADDEND: u32 = 225u32;
```

### `R_SHARC_CALC_ADD`

```rust
const R_SHARC_CALC_ADD: u32 = 226u32;
```

### `R_SHARC_CALC_SUB`

```rust
const R_SHARC_CALC_SUB: u32 = 227u32;
```

### `R_SHARC_CALC_MUL`

```rust
const R_SHARC_CALC_MUL: u32 = 228u32;
```

### `R_SHARC_CALC_DIV`

```rust
const R_SHARC_CALC_DIV: u32 = 229u32;
```

### `R_SHARC_CALC_MOD`

```rust
const R_SHARC_CALC_MOD: u32 = 230u32;
```

### `R_SHARC_CALC_LSHIFT`

```rust
const R_SHARC_CALC_LSHIFT: u32 = 231u32;
```

### `R_SHARC_CALC_RSHIFT`

```rust
const R_SHARC_CALC_RSHIFT: u32 = 232u32;
```

### `R_SHARC_CALC_AND`

```rust
const R_SHARC_CALC_AND: u32 = 233u32;
```

### `R_SHARC_CALC_OR`

```rust
const R_SHARC_CALC_OR: u32 = 234u32;
```

### `R_SHARC_CALC_XOR`

```rust
const R_SHARC_CALC_XOR: u32 = 235u32;
```

### `R_SHARC_CALC_PUSH_LEN`

```rust
const R_SHARC_CALC_PUSH_LEN: u32 = 236u32;
```

### `R_SHARC_CALC_NOT`

```rust
const R_SHARC_CALC_NOT: u32 = 246u32;
```

### `SHT_SHARC_ADI_ATTRIBUTES`

```rust
const SHT_SHARC_ADI_ATTRIBUTES: u32 = 1_879_048_194u32;
```

.adi.attributes

### `STT_SPARC_REGISTER`

```rust
const STT_SPARC_REGISTER: u8 = 13u8;
```

Global register reserved to app.

### `EF_SPARCV9_MM`

```rust
const EF_SPARCV9_MM: u32 = 3u32;
```

### `EF_SPARCV9_TSO`

```rust
const EF_SPARCV9_TSO: u32 = 0u32;
```

### `EF_SPARCV9_PSO`

```rust
const EF_SPARCV9_PSO: u32 = 1u32;
```

### `EF_SPARCV9_RMO`

```rust
const EF_SPARCV9_RMO: u32 = 2u32;
```

### `EF_SPARC_LEDATA`

```rust
const EF_SPARC_LEDATA: u32 = 8_388_608u32;
```

little endian data

### `EF_SPARC_EXT_MASK`

```rust
const EF_SPARC_EXT_MASK: u32 = 16_776_960u32;
```

### `EF_SPARC_32PLUS`

```rust
const EF_SPARC_32PLUS: u32 = 256u32;
```

generic V8+ features

### `EF_SPARC_SUN_US1`

```rust
const EF_SPARC_SUN_US1: u32 = 512u32;
```

Sun UltraSPARC1 extensions

### `EF_SPARC_HAL_R1`

```rust
const EF_SPARC_HAL_R1: u32 = 1_024u32;
```

HAL R1 extensions

### `EF_SPARC_SUN_US3`

```rust
const EF_SPARC_SUN_US3: u32 = 2_048u32;
```

Sun UltraSPARCIII extensions

### `R_SPARC_NONE`

```rust
const R_SPARC_NONE: u32 = 0u32;
```

No reloc

### `R_SPARC_8`

```rust
const R_SPARC_8: u32 = 1u32;
```

Direct 8 bit

### `R_SPARC_16`

```rust
const R_SPARC_16: u32 = 2u32;
```

Direct 16 bit

### `R_SPARC_32`

```rust
const R_SPARC_32: u32 = 3u32;
```

Direct 32 bit

### `R_SPARC_DISP8`

```rust
const R_SPARC_DISP8: u32 = 4u32;
```

PC relative 8 bit

### `R_SPARC_DISP16`

```rust
const R_SPARC_DISP16: u32 = 5u32;
```

PC relative 16 bit

### `R_SPARC_DISP32`

```rust
const R_SPARC_DISP32: u32 = 6u32;
```

PC relative 32 bit

### `R_SPARC_WDISP30`

```rust
const R_SPARC_WDISP30: u32 = 7u32;
```

PC relative 30 bit shifted

### `R_SPARC_WDISP22`

```rust
const R_SPARC_WDISP22: u32 = 8u32;
```

PC relative 22 bit shifted

### `R_SPARC_HI22`

```rust
const R_SPARC_HI22: u32 = 9u32;
```

High 22 bit

### `R_SPARC_22`

```rust
const R_SPARC_22: u32 = 10u32;
```

Direct 22 bit

### `R_SPARC_13`

```rust
const R_SPARC_13: u32 = 11u32;
```

Direct 13 bit

### `R_SPARC_LO10`

```rust
const R_SPARC_LO10: u32 = 12u32;
```

Truncated 10 bit

### `R_SPARC_GOT10`

```rust
const R_SPARC_GOT10: u32 = 13u32;
```

Truncated 10 bit GOT entry

### `R_SPARC_GOT13`

```rust
const R_SPARC_GOT13: u32 = 14u32;
```

13 bit GOT entry

### `R_SPARC_GOT22`

```rust
const R_SPARC_GOT22: u32 = 15u32;
```

22 bit GOT entry shifted

### `R_SPARC_PC10`

```rust
const R_SPARC_PC10: u32 = 16u32;
```

PC relative 10 bit truncated

### `R_SPARC_PC22`

```rust
const R_SPARC_PC22: u32 = 17u32;
```

PC relative 22 bit shifted

### `R_SPARC_WPLT30`

```rust
const R_SPARC_WPLT30: u32 = 18u32;
```

30 bit PC relative PLT address

### `R_SPARC_COPY`

```rust
const R_SPARC_COPY: u32 = 19u32;
```

Copy symbol at runtime

### `R_SPARC_GLOB_DAT`

```rust
const R_SPARC_GLOB_DAT: u32 = 20u32;
```

Create GOT entry

### `R_SPARC_JMP_SLOT`

```rust
const R_SPARC_JMP_SLOT: u32 = 21u32;
```

Create PLT entry

### `R_SPARC_RELATIVE`

```rust
const R_SPARC_RELATIVE: u32 = 22u32;
```

Adjust by program base

### `R_SPARC_UA32`

```rust
const R_SPARC_UA32: u32 = 23u32;
```

Direct 32 bit unaligned

### `R_SPARC_PLT32`

```rust
const R_SPARC_PLT32: u32 = 24u32;
```

Direct 32 bit ref to PLT entry

### `R_SPARC_HIPLT22`

```rust
const R_SPARC_HIPLT22: u32 = 25u32;
```

High 22 bit PLT entry

### `R_SPARC_LOPLT10`

```rust
const R_SPARC_LOPLT10: u32 = 26u32;
```

Truncated 10 bit PLT entry

### `R_SPARC_PCPLT32`

```rust
const R_SPARC_PCPLT32: u32 = 27u32;
```

PC rel 32 bit ref to PLT entry

### `R_SPARC_PCPLT22`

```rust
const R_SPARC_PCPLT22: u32 = 28u32;
```

PC rel high 22 bit PLT entry

### `R_SPARC_PCPLT10`

```rust
const R_SPARC_PCPLT10: u32 = 29u32;
```

PC rel trunc 10 bit PLT entry

### `R_SPARC_10`

```rust
const R_SPARC_10: u32 = 30u32;
```

Direct 10 bit

### `R_SPARC_11`

```rust
const R_SPARC_11: u32 = 31u32;
```

Direct 11 bit

### `R_SPARC_64`

```rust
const R_SPARC_64: u32 = 32u32;
```

Direct 64 bit

### `R_SPARC_OLO10`

```rust
const R_SPARC_OLO10: u32 = 33u32;
```

10bit with secondary 13bit addend

### `R_SPARC_HH22`

```rust
const R_SPARC_HH22: u32 = 34u32;
```

Top 22 bits of direct 64 bit

### `R_SPARC_HM10`

```rust
const R_SPARC_HM10: u32 = 35u32;
```

High middle 10 bits of ...

### `R_SPARC_LM22`

```rust
const R_SPARC_LM22: u32 = 36u32;
```

Low middle 22 bits of ...

### `R_SPARC_PC_HH22`

```rust
const R_SPARC_PC_HH22: u32 = 37u32;
```

Top 22 bits of pc rel 64 bit

### `R_SPARC_PC_HM10`

```rust
const R_SPARC_PC_HM10: u32 = 38u32;
```

High middle 10 bit of ...

### `R_SPARC_PC_LM22`

```rust
const R_SPARC_PC_LM22: u32 = 39u32;
```

Low miggle 22 bits of ...

### `R_SPARC_WDISP16`

```rust
const R_SPARC_WDISP16: u32 = 40u32;
```

PC relative 16 bit shifted

### `R_SPARC_WDISP19`

```rust
const R_SPARC_WDISP19: u32 = 41u32;
```

PC relative 19 bit shifted

### `R_SPARC_GLOB_JMP`

```rust
const R_SPARC_GLOB_JMP: u32 = 42u32;
```

was part of v9 ABI but was removed

### `R_SPARC_7`

```rust
const R_SPARC_7: u32 = 43u32;
```

Direct 7 bit

### `R_SPARC_5`

```rust
const R_SPARC_5: u32 = 44u32;
```

Direct 5 bit

### `R_SPARC_6`

```rust
const R_SPARC_6: u32 = 45u32;
```

Direct 6 bit

### `R_SPARC_DISP64`

```rust
const R_SPARC_DISP64: u32 = 46u32;
```

PC relative 64 bit

### `R_SPARC_PLT64`

```rust
const R_SPARC_PLT64: u32 = 47u32;
```

Direct 64 bit ref to PLT entry

### `R_SPARC_HIX22`

```rust
const R_SPARC_HIX22: u32 = 48u32;
```

High 22 bit complemented

### `R_SPARC_LOX10`

```rust
const R_SPARC_LOX10: u32 = 49u32;
```

Truncated 11 bit complemented

### `R_SPARC_H44`

```rust
const R_SPARC_H44: u32 = 50u32;
```

Direct high 12 of 44 bit

### `R_SPARC_M44`

```rust
const R_SPARC_M44: u32 = 51u32;
```

Direct mid 22 of 44 bit

### `R_SPARC_L44`

```rust
const R_SPARC_L44: u32 = 52u32;
```

Direct low 10 of 44 bit

### `R_SPARC_REGISTER`

```rust
const R_SPARC_REGISTER: u32 = 53u32;
```

Global register usage

### `R_SPARC_UA64`

```rust
const R_SPARC_UA64: u32 = 54u32;
```

Direct 64 bit unaligned

### `R_SPARC_UA16`

```rust
const R_SPARC_UA16: u32 = 55u32;
```

Direct 16 bit unaligned

### `R_SPARC_TLS_GD_HI22`

```rust
const R_SPARC_TLS_GD_HI22: u32 = 56u32;
```

### `R_SPARC_TLS_GD_LO10`

```rust
const R_SPARC_TLS_GD_LO10: u32 = 57u32;
```

### `R_SPARC_TLS_GD_ADD`

```rust
const R_SPARC_TLS_GD_ADD: u32 = 58u32;
```

### `R_SPARC_TLS_GD_CALL`

```rust
const R_SPARC_TLS_GD_CALL: u32 = 59u32;
```

### `R_SPARC_TLS_LDM_HI22`

```rust
const R_SPARC_TLS_LDM_HI22: u32 = 60u32;
```

### `R_SPARC_TLS_LDM_LO10`

```rust
const R_SPARC_TLS_LDM_LO10: u32 = 61u32;
```

### `R_SPARC_TLS_LDM_ADD`

```rust
const R_SPARC_TLS_LDM_ADD: u32 = 62u32;
```

### `R_SPARC_TLS_LDM_CALL`

```rust
const R_SPARC_TLS_LDM_CALL: u32 = 63u32;
```

### `R_SPARC_TLS_LDO_HIX22`

```rust
const R_SPARC_TLS_LDO_HIX22: u32 = 64u32;
```

### `R_SPARC_TLS_LDO_LOX10`

```rust
const R_SPARC_TLS_LDO_LOX10: u32 = 65u32;
```

### `R_SPARC_TLS_LDO_ADD`

```rust
const R_SPARC_TLS_LDO_ADD: u32 = 66u32;
```

### `R_SPARC_TLS_IE_HI22`

```rust
const R_SPARC_TLS_IE_HI22: u32 = 67u32;
```

### `R_SPARC_TLS_IE_LO10`

```rust
const R_SPARC_TLS_IE_LO10: u32 = 68u32;
```

### `R_SPARC_TLS_IE_LD`

```rust
const R_SPARC_TLS_IE_LD: u32 = 69u32;
```

### `R_SPARC_TLS_IE_LDX`

```rust
const R_SPARC_TLS_IE_LDX: u32 = 70u32;
```

### `R_SPARC_TLS_IE_ADD`

```rust
const R_SPARC_TLS_IE_ADD: u32 = 71u32;
```

### `R_SPARC_TLS_LE_HIX22`

```rust
const R_SPARC_TLS_LE_HIX22: u32 = 72u32;
```

### `R_SPARC_TLS_LE_LOX10`

```rust
const R_SPARC_TLS_LE_LOX10: u32 = 73u32;
```

### `R_SPARC_TLS_DTPMOD32`

```rust
const R_SPARC_TLS_DTPMOD32: u32 = 74u32;
```

### `R_SPARC_TLS_DTPMOD64`

```rust
const R_SPARC_TLS_DTPMOD64: u32 = 75u32;
```

### `R_SPARC_TLS_DTPOFF32`

```rust
const R_SPARC_TLS_DTPOFF32: u32 = 76u32;
```

### `R_SPARC_TLS_DTPOFF64`

```rust
const R_SPARC_TLS_DTPOFF64: u32 = 77u32;
```

### `R_SPARC_TLS_TPOFF32`

```rust
const R_SPARC_TLS_TPOFF32: u32 = 78u32;
```

### `R_SPARC_TLS_TPOFF64`

```rust
const R_SPARC_TLS_TPOFF64: u32 = 79u32;
```

### `R_SPARC_GOTDATA_HIX22`

```rust
const R_SPARC_GOTDATA_HIX22: u32 = 80u32;
```

### `R_SPARC_GOTDATA_LOX10`

```rust
const R_SPARC_GOTDATA_LOX10: u32 = 81u32;
```

### `R_SPARC_GOTDATA_OP_HIX22`

```rust
const R_SPARC_GOTDATA_OP_HIX22: u32 = 82u32;
```

### `R_SPARC_GOTDATA_OP_LOX10`

```rust
const R_SPARC_GOTDATA_OP_LOX10: u32 = 83u32;
```

### `R_SPARC_GOTDATA_OP`

```rust
const R_SPARC_GOTDATA_OP: u32 = 84u32;
```

### `R_SPARC_H34`

```rust
const R_SPARC_H34: u32 = 85u32;
```

### `R_SPARC_SIZE32`

```rust
const R_SPARC_SIZE32: u32 = 86u32;
```

### `R_SPARC_SIZE64`

```rust
const R_SPARC_SIZE64: u32 = 87u32;
```

### `R_SPARC_WDISP10`

```rust
const R_SPARC_WDISP10: u32 = 88u32;
```

### `R_SPARC_JMP_IREL`

```rust
const R_SPARC_JMP_IREL: u32 = 248u32;
```

### `R_SPARC_IRELATIVE`

```rust
const R_SPARC_IRELATIVE: u32 = 249u32;
```

### `R_SPARC_GNU_VTINHERIT`

```rust
const R_SPARC_GNU_VTINHERIT: u32 = 250u32;
```

### `R_SPARC_GNU_VTENTRY`

```rust
const R_SPARC_GNU_VTENTRY: u32 = 251u32;
```

### `R_SPARC_REV32`

```rust
const R_SPARC_REV32: u32 = 252u32;
```

### `DT_SPARC_REGISTER`

```rust
const DT_SPARC_REGISTER: u32 = 1_879_048_193u32;
```

### `EF_MIPS_NOREORDER`

```rust
const EF_MIPS_NOREORDER: u32 = 1u32;
```

A .noreorder directive was used.

### `EF_MIPS_PIC`

```rust
const EF_MIPS_PIC: u32 = 2u32;
```

Contains PIC code.

### `EF_MIPS_CPIC`

```rust
const EF_MIPS_CPIC: u32 = 4u32;
```

Uses PIC calling sequence.

### `EF_MIPS_XGOT`

```rust
const EF_MIPS_XGOT: u32 = 8u32;
```

### `EF_MIPS_64BIT_WHIRL`

```rust
const EF_MIPS_64BIT_WHIRL: u32 = 16u32;
```

### `EF_MIPS_ABI2`

```rust
const EF_MIPS_ABI2: u32 = 32u32;
```

### `EF_MIPS_ABI_ON32`

```rust
const EF_MIPS_ABI_ON32: u32 = 64u32;
```

### `EF_MIPS_FP64`

```rust
const EF_MIPS_FP64: u32 = 512u32;
```

Uses FP64 (12 callee-saved).

### `EF_MIPS_NAN2008`

```rust
const EF_MIPS_NAN2008: u32 = 1_024u32;
```

Uses IEEE 754-2008 NaN encoding.

### `EF_MIPS_ARCH`

```rust
const EF_MIPS_ARCH: u32 = 4_026_531_840u32;
```

MIPS architecture level.

### `EF_MIPS_ABI_O32`

```rust
const EF_MIPS_ABI_O32: u32 = 4_096u32;
```

The first MIPS 32 bit ABI

### `EF_MIPS_ABI_O64`

```rust
const EF_MIPS_ABI_O64: u32 = 8_192u32;
```

O32 ABI extended for 64-bit architectures

### `EF_MIPS_ABI_EABI32`

```rust
const EF_MIPS_ABI_EABI32: u32 = 12_288u32;
```

EABI in 32-bit mode

### `EF_MIPS_ABI_EABI64`

```rust
const EF_MIPS_ABI_EABI64: u32 = 16_384u32;
```

EABI in 64-bit mode

### `EF_MIPS_ABI`

```rust
const EF_MIPS_ABI: u32 = 61_440u32;
```

Mask for selecting EF_MIPS_ABI_ variant

### `EF_MIPS_ARCH_1`

```rust
const EF_MIPS_ARCH_1: u32 = 0u32;
```

-mips1 code.

### `EF_MIPS_ARCH_2`

```rust
const EF_MIPS_ARCH_2: u32 = 268_435_456u32;
```

-mips2 code.

### `EF_MIPS_ARCH_3`

```rust
const EF_MIPS_ARCH_3: u32 = 536_870_912u32;
```

-mips3 code.

### `EF_MIPS_ARCH_4`

```rust
const EF_MIPS_ARCH_4: u32 = 805_306_368u32;
```

-mips4 code.

### `EF_MIPS_ARCH_5`

```rust
const EF_MIPS_ARCH_5: u32 = 1_073_741_824u32;
```

-mips5 code.

### `EF_MIPS_ARCH_32`

```rust
const EF_MIPS_ARCH_32: u32 = 1_342_177_280u32;
```

MIPS32 code.

### `EF_MIPS_ARCH_64`

```rust
const EF_MIPS_ARCH_64: u32 = 1_610_612_736u32;
```

MIPS64 code.

### `EF_MIPS_ARCH_32R2`

```rust
const EF_MIPS_ARCH_32R2: u32 = 1_879_048_192u32;
```

MIPS32r2 code.

### `EF_MIPS_ARCH_64R2`

```rust
const EF_MIPS_ARCH_64R2: u32 = 2_147_483_648u32;
```

MIPS64r2 code.

### `EF_MIPS_ARCH_32R6`

```rust
const EF_MIPS_ARCH_32R6: u32 = 2_415_919_104u32;
```

MIPS32r6 code

### `EF_MIPS_ARCH_64R6`

```rust
const EF_MIPS_ARCH_64R6: u32 = 2_684_354_560u32;
```

MIPS64r6 code

### `SHN_MIPS_ACOMMON`

```rust
const SHN_MIPS_ACOMMON: u16 = 65_280u16;
```

Allocated common symbols.

### `SHN_MIPS_TEXT`

```rust
const SHN_MIPS_TEXT: u16 = 65_281u16;
```

Allocated test symbols.

### `SHN_MIPS_DATA`

```rust
const SHN_MIPS_DATA: u16 = 65_282u16;
```

Allocated data symbols.

### `SHN_MIPS_SCOMMON`

```rust
const SHN_MIPS_SCOMMON: u16 = 65_283u16;
```

Small common symbols.

### `SHN_MIPS_SUNDEFINED`

```rust
const SHN_MIPS_SUNDEFINED: u16 = 65_284u16;
```

Small undefined symbols.

### `SHT_MIPS_LIBLIST`

```rust
const SHT_MIPS_LIBLIST: u32 = 1_879_048_192u32;
```

Shared objects used in link.

### `SHT_MIPS_MSYM`

```rust
const SHT_MIPS_MSYM: u32 = 1_879_048_193u32;
```

### `SHT_MIPS_CONFLICT`

```rust
const SHT_MIPS_CONFLICT: u32 = 1_879_048_194u32;
```

Conflicting symbols.

### `SHT_MIPS_GPTAB`

```rust
const SHT_MIPS_GPTAB: u32 = 1_879_048_195u32;
```

Global data area sizes.

### `SHT_MIPS_UCODE`

```rust
const SHT_MIPS_UCODE: u32 = 1_879_048_196u32;
```

Reserved for SGI/MIPS compilers

### `SHT_MIPS_DEBUG`

```rust
const SHT_MIPS_DEBUG: u32 = 1_879_048_197u32;
```

MIPS ECOFF debugging info.

### `SHT_MIPS_REGINFO`

```rust
const SHT_MIPS_REGINFO: u32 = 1_879_048_198u32;
```

Register usage information.

### `SHT_MIPS_PACKAGE`

```rust
const SHT_MIPS_PACKAGE: u32 = 1_879_048_199u32;
```

### `SHT_MIPS_PACKSYM`

```rust
const SHT_MIPS_PACKSYM: u32 = 1_879_048_200u32;
```

### `SHT_MIPS_RELD`

```rust
const SHT_MIPS_RELD: u32 = 1_879_048_201u32;
```

### `SHT_MIPS_IFACE`

```rust
const SHT_MIPS_IFACE: u32 = 1_879_048_203u32;
```

### `SHT_MIPS_CONTENT`

```rust
const SHT_MIPS_CONTENT: u32 = 1_879_048_204u32;
```

### `SHT_MIPS_OPTIONS`

```rust
const SHT_MIPS_OPTIONS: u32 = 1_879_048_205u32;
```

Miscellaneous options.

### `SHT_MIPS_SHDR`

```rust
const SHT_MIPS_SHDR: u32 = 1_879_048_208u32;
```

### `SHT_MIPS_FDESC`

```rust
const SHT_MIPS_FDESC: u32 = 1_879_048_209u32;
```

### `SHT_MIPS_EXTSYM`

```rust
const SHT_MIPS_EXTSYM: u32 = 1_879_048_210u32;
```

### `SHT_MIPS_DENSE`

```rust
const SHT_MIPS_DENSE: u32 = 1_879_048_211u32;
```

### `SHT_MIPS_PDESC`

```rust
const SHT_MIPS_PDESC: u32 = 1_879_048_212u32;
```

### `SHT_MIPS_LOCSYM`

```rust
const SHT_MIPS_LOCSYM: u32 = 1_879_048_213u32;
```

### `SHT_MIPS_AUXSYM`

```rust
const SHT_MIPS_AUXSYM: u32 = 1_879_048_214u32;
```

### `SHT_MIPS_OPTSYM`

```rust
const SHT_MIPS_OPTSYM: u32 = 1_879_048_215u32;
```

### `SHT_MIPS_LOCSTR`

```rust
const SHT_MIPS_LOCSTR: u32 = 1_879_048_216u32;
```

### `SHT_MIPS_LINE`

```rust
const SHT_MIPS_LINE: u32 = 1_879_048_217u32;
```

### `SHT_MIPS_RFDESC`

```rust
const SHT_MIPS_RFDESC: u32 = 1_879_048_218u32;
```

### `SHT_MIPS_DELTASYM`

```rust
const SHT_MIPS_DELTASYM: u32 = 1_879_048_219u32;
```

### `SHT_MIPS_DELTAINST`

```rust
const SHT_MIPS_DELTAINST: u32 = 1_879_048_220u32;
```

### `SHT_MIPS_DELTACLASS`

```rust
const SHT_MIPS_DELTACLASS: u32 = 1_879_048_221u32;
```

### `SHT_MIPS_DWARF`

```rust
const SHT_MIPS_DWARF: u32 = 1_879_048_222u32;
```

DWARF debugging information.

### `SHT_MIPS_DELTADECL`

```rust
const SHT_MIPS_DELTADECL: u32 = 1_879_048_223u32;
```

### `SHT_MIPS_SYMBOL_LIB`

```rust
const SHT_MIPS_SYMBOL_LIB: u32 = 1_879_048_224u32;
```

### `SHT_MIPS_EVENTS`

```rust
const SHT_MIPS_EVENTS: u32 = 1_879_048_225u32;
```

Event section.

### `SHT_MIPS_TRANSLATE`

```rust
const SHT_MIPS_TRANSLATE: u32 = 1_879_048_226u32;
```

### `SHT_MIPS_PIXIE`

```rust
const SHT_MIPS_PIXIE: u32 = 1_879_048_227u32;
```

### `SHT_MIPS_XLATE`

```rust
const SHT_MIPS_XLATE: u32 = 1_879_048_228u32;
```

### `SHT_MIPS_XLATE_DEBUG`

```rust
const SHT_MIPS_XLATE_DEBUG: u32 = 1_879_048_229u32;
```

### `SHT_MIPS_WHIRL`

```rust
const SHT_MIPS_WHIRL: u32 = 1_879_048_230u32;
```

### `SHT_MIPS_EH_REGION`

```rust
const SHT_MIPS_EH_REGION: u32 = 1_879_048_231u32;
```

### `SHT_MIPS_XLATE_OLD`

```rust
const SHT_MIPS_XLATE_OLD: u32 = 1_879_048_232u32;
```

### `SHT_MIPS_PDR_EXCEPTION`

```rust
const SHT_MIPS_PDR_EXCEPTION: u32 = 1_879_048_233u32;
```

### `SHF_MIPS_GPREL`

```rust
const SHF_MIPS_GPREL: u32 = 268_435_456u32;
```

Must be in global data area.

### `SHF_MIPS_MERGE`

```rust
const SHF_MIPS_MERGE: u32 = 536_870_912u32;
```

### `SHF_MIPS_ADDR`

```rust
const SHF_MIPS_ADDR: u32 = 1_073_741_824u32;
```

### `SHF_MIPS_STRINGS`

```rust
const SHF_MIPS_STRINGS: u32 = 2_147_483_648u32;
```

### `SHF_MIPS_NOSTRIP`

```rust
const SHF_MIPS_NOSTRIP: u32 = 134_217_728u32;
```

### `SHF_MIPS_LOCAL`

```rust
const SHF_MIPS_LOCAL: u32 = 67_108_864u32;
```

### `SHF_MIPS_NAMES`

```rust
const SHF_MIPS_NAMES: u32 = 33_554_432u32;
```

### `SHF_MIPS_NODUPE`

```rust
const SHF_MIPS_NODUPE: u32 = 16_777_216u32;
```

### `STO_MIPS_PLT`

```rust
const STO_MIPS_PLT: u8 = 8u8;
```

### `STO_MIPS_SC_ALIGN_UNUSED`

```rust
const STO_MIPS_SC_ALIGN_UNUSED: u8 = 255u8;
```

Only valid for `STB_MIPS_SPLIT_COMMON`.

### `STB_MIPS_SPLIT_COMMON`

```rust
const STB_MIPS_SPLIT_COMMON: u8 = 13u8;
```

### `ODK_NULL`

```rust
const ODK_NULL: u32 = 0u32;
```

Undefined.

### `ODK_REGINFO`

```rust
const ODK_REGINFO: u32 = 1u32;
```

Register usage information.

### `ODK_EXCEPTIONS`

```rust
const ODK_EXCEPTIONS: u32 = 2u32;
```

Exception processing options.

### `ODK_PAD`

```rust
const ODK_PAD: u32 = 3u32;
```

Section padding options.

### `ODK_HWPATCH`

```rust
const ODK_HWPATCH: u32 = 4u32;
```

Hardware workarounds performed

### `ODK_FILL`

```rust
const ODK_FILL: u32 = 5u32;
```

record the fill value used by the linker.

### `ODK_TAGS`

```rust
const ODK_TAGS: u32 = 6u32;
```

reserve space for desktop tools to write.

### `ODK_HWAND`

```rust
const ODK_HWAND: u32 = 7u32;
```

HW workarounds.  'AND' bits when merging.

### `ODK_HWOR`

```rust
const ODK_HWOR: u32 = 8u32;
```

HW workarounds.  'OR' bits when merging.

### `OEX_FPU_MIN`

```rust
const OEX_FPU_MIN: u32 = 31u32;
```

FPE's which MUST be enabled.

### `OEX_FPU_MAX`

```rust
const OEX_FPU_MAX: u32 = 7_936u32;
```

FPE's which MAY be enabled.

### `OEX_PAGE0`

```rust
const OEX_PAGE0: u32 = 65_536u32;
```

page zero must be mapped.

### `OEX_SMM`

```rust
const OEX_SMM: u32 = 131_072u32;
```

Force sequential memory mode?

### `OEX_FPDBUG`

```rust
const OEX_FPDBUG: u32 = 262_144u32;
```

Force floating point debug mode?

### `OEX_PRECISEFP`

```rust
const OEX_PRECISEFP: u32 = 262_144u32;
```

### `OEX_DISMISS`

```rust
const OEX_DISMISS: u32 = 524_288u32;
```

Dismiss invalid address faults?

### `OEX_FPU_INVAL`

```rust
const OEX_FPU_INVAL: u32 = 16u32;
```

### `OEX_FPU_DIV0`

```rust
const OEX_FPU_DIV0: u32 = 8u32;
```

### `OEX_FPU_OFLO`

```rust
const OEX_FPU_OFLO: u32 = 4u32;
```

### `OEX_FPU_UFLO`

```rust
const OEX_FPU_UFLO: u32 = 2u32;
```

### `OEX_FPU_INEX`

```rust
const OEX_FPU_INEX: u32 = 1u32;
```

### `OHW_R4KEOP`

```rust
const OHW_R4KEOP: u32 = 1u32;
```

R4000 end-of-page patch.

### `OHW_R8KPFETCH`

```rust
const OHW_R8KPFETCH: u32 = 2u32;
```

may need R8000 prefetch patch.

### `OHW_R5KEOP`

```rust
const OHW_R5KEOP: u32 = 4u32;
```

R5000 end-of-page patch.

### `OHW_R5KCVTL`

```rust
const OHW_R5KCVTL: u32 = 8u32;
```

R5000 cvt.\[ds\].l bug.  clean=1.

### `OPAD_PREFIX`

```rust
const OPAD_PREFIX: u32 = 1u32;
```

### `OPAD_POSTFIX`

```rust
const OPAD_POSTFIX: u32 = 2u32;
```

### `OPAD_SYMBOL`

```rust
const OPAD_SYMBOL: u32 = 4u32;
```

### `OHWA0_R4KEOP_CHECKED`

```rust
const OHWA0_R4KEOP_CHECKED: u32 = 1u32;
```

### `OHWA1_R4KEOP_CLEAN`

```rust
const OHWA1_R4KEOP_CLEAN: u32 = 2u32;
```

### `R_MIPS_NONE`

```rust
const R_MIPS_NONE: u32 = 0u32;
```

No reloc

### `R_MIPS_16`

```rust
const R_MIPS_16: u32 = 1u32;
```

Direct 16 bit

### `R_MIPS_32`

```rust
const R_MIPS_32: u32 = 2u32;
```

Direct 32 bit

### `R_MIPS_REL32`

```rust
const R_MIPS_REL32: u32 = 3u32;
```

PC relative 32 bit

### `R_MIPS_26`

```rust
const R_MIPS_26: u32 = 4u32;
```

Direct 26 bit shifted

### `R_MIPS_HI16`

```rust
const R_MIPS_HI16: u32 = 5u32;
```

High 16 bit

### `R_MIPS_LO16`

```rust
const R_MIPS_LO16: u32 = 6u32;
```

Low 16 bit

### `R_MIPS_GPREL16`

```rust
const R_MIPS_GPREL16: u32 = 7u32;
```

GP relative 16 bit

### `R_MIPS_LITERAL`

```rust
const R_MIPS_LITERAL: u32 = 8u32;
```

16 bit literal entry

### `R_MIPS_GOT16`

```rust
const R_MIPS_GOT16: u32 = 9u32;
```

16 bit GOT entry

### `R_MIPS_PC16`

```rust
const R_MIPS_PC16: u32 = 10u32;
```

PC relative 16 bit

### `R_MIPS_CALL16`

```rust
const R_MIPS_CALL16: u32 = 11u32;
```

16 bit GOT entry for function

### `R_MIPS_GPREL32`

```rust
const R_MIPS_GPREL32: u32 = 12u32;
```

GP relative 32 bit

### `R_MIPS_SHIFT5`

```rust
const R_MIPS_SHIFT5: u32 = 16u32;
```

### `R_MIPS_SHIFT6`

```rust
const R_MIPS_SHIFT6: u32 = 17u32;
```

### `R_MIPS_64`

```rust
const R_MIPS_64: u32 = 18u32;
```

### `R_MIPS_GOT_DISP`

```rust
const R_MIPS_GOT_DISP: u32 = 19u32;
```

### `R_MIPS_GOT_PAGE`

```rust
const R_MIPS_GOT_PAGE: u32 = 20u32;
```

### `R_MIPS_GOT_OFST`

```rust
const R_MIPS_GOT_OFST: u32 = 21u32;
```

### `R_MIPS_GOT_HI16`

```rust
const R_MIPS_GOT_HI16: u32 = 22u32;
```

### `R_MIPS_GOT_LO16`

```rust
const R_MIPS_GOT_LO16: u32 = 23u32;
```

### `R_MIPS_SUB`

```rust
const R_MIPS_SUB: u32 = 24u32;
```

### `R_MIPS_INSERT_A`

```rust
const R_MIPS_INSERT_A: u32 = 25u32;
```

### `R_MIPS_INSERT_B`

```rust
const R_MIPS_INSERT_B: u32 = 26u32;
```

### `R_MIPS_DELETE`

```rust
const R_MIPS_DELETE: u32 = 27u32;
```

### `R_MIPS_HIGHER`

```rust
const R_MIPS_HIGHER: u32 = 28u32;
```

### `R_MIPS_HIGHEST`

```rust
const R_MIPS_HIGHEST: u32 = 29u32;
```

### `R_MIPS_CALL_HI16`

```rust
const R_MIPS_CALL_HI16: u32 = 30u32;
```

### `R_MIPS_CALL_LO16`

```rust
const R_MIPS_CALL_LO16: u32 = 31u32;
```

### `R_MIPS_SCN_DISP`

```rust
const R_MIPS_SCN_DISP: u32 = 32u32;
```

### `R_MIPS_REL16`

```rust
const R_MIPS_REL16: u32 = 33u32;
```

### `R_MIPS_ADD_IMMEDIATE`

```rust
const R_MIPS_ADD_IMMEDIATE: u32 = 34u32;
```

### `R_MIPS_PJUMP`

```rust
const R_MIPS_PJUMP: u32 = 35u32;
```

### `R_MIPS_RELGOT`

```rust
const R_MIPS_RELGOT: u32 = 36u32;
```

### `R_MIPS_JALR`

```rust
const R_MIPS_JALR: u32 = 37u32;
```

### `R_MIPS_TLS_DTPMOD32`

```rust
const R_MIPS_TLS_DTPMOD32: u32 = 38u32;
```

Module number 32 bit

### `R_MIPS_TLS_DTPREL32`

```rust
const R_MIPS_TLS_DTPREL32: u32 = 39u32;
```

Module-relative offset 32 bit

### `R_MIPS_TLS_DTPMOD64`

```rust
const R_MIPS_TLS_DTPMOD64: u32 = 40u32;
```

Module number 64 bit

### `R_MIPS_TLS_DTPREL64`

```rust
const R_MIPS_TLS_DTPREL64: u32 = 41u32;
```

Module-relative offset 64 bit

### `R_MIPS_TLS_GD`

```rust
const R_MIPS_TLS_GD: u32 = 42u32;
```

16 bit GOT offset for GD

### `R_MIPS_TLS_LDM`

```rust
const R_MIPS_TLS_LDM: u32 = 43u32;
```

16 bit GOT offset for LDM

### `R_MIPS_TLS_DTPREL_HI16`

```rust
const R_MIPS_TLS_DTPREL_HI16: u32 = 44u32;
```

Module-relative offset, high 16 bits

### `R_MIPS_TLS_DTPREL_LO16`

```rust
const R_MIPS_TLS_DTPREL_LO16: u32 = 45u32;
```

Module-relative offset, low 16 bits

### `R_MIPS_TLS_GOTTPREL`

```rust
const R_MIPS_TLS_GOTTPREL: u32 = 46u32;
```

16 bit GOT offset for IE

### `R_MIPS_TLS_TPREL32`

```rust
const R_MIPS_TLS_TPREL32: u32 = 47u32;
```

TP-relative offset, 32 bit

### `R_MIPS_TLS_TPREL64`

```rust
const R_MIPS_TLS_TPREL64: u32 = 48u32;
```

TP-relative offset, 64 bit

### `R_MIPS_TLS_TPREL_HI16`

```rust
const R_MIPS_TLS_TPREL_HI16: u32 = 49u32;
```

TP-relative offset, high 16 bits

### `R_MIPS_TLS_TPREL_LO16`

```rust
const R_MIPS_TLS_TPREL_LO16: u32 = 50u32;
```

TP-relative offset, low 16 bits

### `R_MIPS_GLOB_DAT`

```rust
const R_MIPS_GLOB_DAT: u32 = 51u32;
```

### `R_MIPS_COPY`

```rust
const R_MIPS_COPY: u32 = 126u32;
```

### `R_MIPS_JUMP_SLOT`

```rust
const R_MIPS_JUMP_SLOT: u32 = 127u32;
```

### `PT_MIPS_REGINFO`

```rust
const PT_MIPS_REGINFO: u32 = 1_879_048_192u32;
```

Register usage information.

### `PT_MIPS_RTPROC`

```rust
const PT_MIPS_RTPROC: u32 = 1_879_048_193u32;
```

Runtime procedure table.

### `PT_MIPS_OPTIONS`

```rust
const PT_MIPS_OPTIONS: u32 = 1_879_048_194u32;
```

### `PT_MIPS_ABIFLAGS`

```rust
const PT_MIPS_ABIFLAGS: u32 = 1_879_048_195u32;
```

FP mode requirement.

### `PF_MIPS_LOCAL`

```rust
const PF_MIPS_LOCAL: u32 = 268_435_456u32;
```

### `DT_MIPS_RLD_VERSION`

```rust
const DT_MIPS_RLD_VERSION: u32 = 1_879_048_193u32;
```

Runtime linker interface version

### `DT_MIPS_TIME_STAMP`

```rust
const DT_MIPS_TIME_STAMP: u32 = 1_879_048_194u32;
```

Timestamp

### `DT_MIPS_ICHECKSUM`

```rust
const DT_MIPS_ICHECKSUM: u32 = 1_879_048_195u32;
```

Checksum

### `DT_MIPS_IVERSION`

```rust
const DT_MIPS_IVERSION: u32 = 1_879_048_196u32;
```

Version string (string tbl index)

### `DT_MIPS_FLAGS`

```rust
const DT_MIPS_FLAGS: u32 = 1_879_048_197u32;
```

Flags

### `DT_MIPS_BASE_ADDRESS`

```rust
const DT_MIPS_BASE_ADDRESS: u32 = 1_879_048_198u32;
```

Base address

### `DT_MIPS_MSYM`

```rust
const DT_MIPS_MSYM: u32 = 1_879_048_199u32;
```

### `DT_MIPS_CONFLICT`

```rust
const DT_MIPS_CONFLICT: u32 = 1_879_048_200u32;
```

Address of CONFLICT section

### `DT_MIPS_LIBLIST`

```rust
const DT_MIPS_LIBLIST: u32 = 1_879_048_201u32;
```

Address of LIBLIST section

### `DT_MIPS_LOCAL_GOTNO`

```rust
const DT_MIPS_LOCAL_GOTNO: u32 = 1_879_048_202u32;
```

Number of local GOT entries

### `DT_MIPS_CONFLICTNO`

```rust
const DT_MIPS_CONFLICTNO: u32 = 1_879_048_203u32;
```

Number of CONFLICT entries

### `DT_MIPS_LIBLISTNO`

```rust
const DT_MIPS_LIBLISTNO: u32 = 1_879_048_208u32;
```

Number of LIBLIST entries

### `DT_MIPS_SYMTABNO`

```rust
const DT_MIPS_SYMTABNO: u32 = 1_879_048_209u32;
```

Number of DYNSYM entries

### `DT_MIPS_UNREFEXTNO`

```rust
const DT_MIPS_UNREFEXTNO: u32 = 1_879_048_210u32;
```

First external DYNSYM

### `DT_MIPS_GOTSYM`

```rust
const DT_MIPS_GOTSYM: u32 = 1_879_048_211u32;
```

First GOT entry in DYNSYM

### `DT_MIPS_HIPAGENO`

```rust
const DT_MIPS_HIPAGENO: u32 = 1_879_048_212u32;
```

Number of GOT page table entries

### `DT_MIPS_RLD_MAP`

```rust
const DT_MIPS_RLD_MAP: u32 = 1_879_048_214u32;
```

Address of run time loader map.

### `DT_MIPS_DELTA_CLASS`

```rust
const DT_MIPS_DELTA_CLASS: u32 = 1_879_048_215u32;
```

Delta C++ class definition.

### `DT_MIPS_DELTA_CLASS_NO`

```rust
const DT_MIPS_DELTA_CLASS_NO: u32 = 1_879_048_216u32;
```

Number of entries in DT_MIPS_DELTA_CLASS.

### `DT_MIPS_DELTA_INSTANCE`

```rust
const DT_MIPS_DELTA_INSTANCE: u32 = 1_879_048_217u32;
```

Delta C++ class instances.

### `DT_MIPS_DELTA_INSTANCE_NO`

```rust
const DT_MIPS_DELTA_INSTANCE_NO: u32 = 1_879_048_218u32;
```

Number of entries in DT_MIPS_DELTA_INSTANCE.

### `DT_MIPS_DELTA_RELOC`

```rust
const DT_MIPS_DELTA_RELOC: u32 = 1_879_048_219u32;
```

Delta relocations.

### `DT_MIPS_DELTA_RELOC_NO`

```rust
const DT_MIPS_DELTA_RELOC_NO: u32 = 1_879_048_220u32;
```

Number of entries in DT_MIPS_DELTA_RELOC.

### `DT_MIPS_DELTA_SYM`

```rust
const DT_MIPS_DELTA_SYM: u32 = 1_879_048_221u32;
```

Delta symbols that Delta relocations refer to.

### `DT_MIPS_DELTA_SYM_NO`

```rust
const DT_MIPS_DELTA_SYM_NO: u32 = 1_879_048_222u32;
```

Number of entries in DT_MIPS_DELTA_SYM.

### `DT_MIPS_DELTA_CLASSSYM`

```rust
const DT_MIPS_DELTA_CLASSSYM: u32 = 1_879_048_224u32;
```

Delta symbols that hold the class declaration.

### `DT_MIPS_DELTA_CLASSSYM_NO`

```rust
const DT_MIPS_DELTA_CLASSSYM_NO: u32 = 1_879_048_225u32;
```

Number of entries in DT_MIPS_DELTA_CLASSSYM.

### `DT_MIPS_CXX_FLAGS`

```rust
const DT_MIPS_CXX_FLAGS: u32 = 1_879_048_226u32;
```

Flags indicating for C++ flavor.

### `DT_MIPS_PIXIE_INIT`

```rust
const DT_MIPS_PIXIE_INIT: u32 = 1_879_048_227u32;
```

### `DT_MIPS_SYMBOL_LIB`

```rust
const DT_MIPS_SYMBOL_LIB: u32 = 1_879_048_228u32;
```

### `DT_MIPS_LOCALPAGE_GOTIDX`

```rust
const DT_MIPS_LOCALPAGE_GOTIDX: u32 = 1_879_048_229u32;
```

### `DT_MIPS_LOCAL_GOTIDX`

```rust
const DT_MIPS_LOCAL_GOTIDX: u32 = 1_879_048_230u32;
```

### `DT_MIPS_HIDDEN_GOTIDX`

```rust
const DT_MIPS_HIDDEN_GOTIDX: u32 = 1_879_048_231u32;
```

### `DT_MIPS_PROTECTED_GOTIDX`

```rust
const DT_MIPS_PROTECTED_GOTIDX: u32 = 1_879_048_232u32;
```

### `DT_MIPS_OPTIONS`

```rust
const DT_MIPS_OPTIONS: u32 = 1_879_048_233u32;
```

Address of .options.

### `DT_MIPS_INTERFACE`

```rust
const DT_MIPS_INTERFACE: u32 = 1_879_048_234u32;
```

Address of .interface.

### `DT_MIPS_DYNSTR_ALIGN`

```rust
const DT_MIPS_DYNSTR_ALIGN: u32 = 1_879_048_235u32;
```

### `DT_MIPS_INTERFACE_SIZE`

```rust
const DT_MIPS_INTERFACE_SIZE: u32 = 1_879_048_236u32;
```

Size of the .interface section.

### `DT_MIPS_RLD_TEXT_RESOLVE_ADDR`

```rust
const DT_MIPS_RLD_TEXT_RESOLVE_ADDR: u32 = 1_879_048_237u32;
```

Address of rld_text_rsolve function stored in GOT.

### `DT_MIPS_PERF_SUFFIX`

```rust
const DT_MIPS_PERF_SUFFIX: u32 = 1_879_048_238u32;
```

Default suffix of dso to be added by rld on dlopen() calls.

### `DT_MIPS_COMPACT_SIZE`

```rust
const DT_MIPS_COMPACT_SIZE: u32 = 1_879_048_239u32;
```

(O32)Size of compact rel section.

### `DT_MIPS_GP_VALUE`

```rust
const DT_MIPS_GP_VALUE: u32 = 1_879_048_240u32;
```

GP value for aux GOTs.

### `DT_MIPS_AUX_DYNAMIC`

```rust
const DT_MIPS_AUX_DYNAMIC: u32 = 1_879_048_241u32;
```

Address of aux .dynamic.

### `DT_MIPS_PLTGOT`

```rust
const DT_MIPS_PLTGOT: u32 = 1_879_048_242u32;
```

The address of .got.plt in an executable using the new non-PIC ABI.

### `DT_MIPS_RWPLT`

```rust
const DT_MIPS_RWPLT: u32 = 1_879_048_244u32;
```

The base of the PLT in an executable using the new non-PIC ABI if that PLT is writable.  For a non-writable PLT, this is omitted or has a zero value.

### `DT_MIPS_RLD_MAP_REL`

```rust
const DT_MIPS_RLD_MAP_REL: u32 = 1_879_048_245u32;
```

An alternative description of the classic MIPS RLD_MAP that is usable in a PIE as it stores a relative offset from the address of the tag rather than an absolute address.

### `RHF_NONE`

```rust
const RHF_NONE: u32 = 0u32;
```

No flags

### `RHF_QUICKSTART`

```rust
const RHF_QUICKSTART: u32 = 1u32;
```

Use quickstart

### `RHF_NOTPOT`

```rust
const RHF_NOTPOT: u32 = 2u32;
```

Hash size not power of 2

### `RHF_NO_LIBRARY_REPLACEMENT`

```rust
const RHF_NO_LIBRARY_REPLACEMENT: u32 = 4u32;
```

Ignore LD_LIBRARY_PATH

### `RHF_NO_MOVE`

```rust
const RHF_NO_MOVE: u32 = 8u32;
```

### `RHF_SGI_ONLY`

```rust
const RHF_SGI_ONLY: u32 = 16u32;
```

### `RHF_GUARANTEE_INIT`

```rust
const RHF_GUARANTEE_INIT: u32 = 32u32;
```

### `RHF_DELTA_C_PLUS_PLUS`

```rust
const RHF_DELTA_C_PLUS_PLUS: u32 = 64u32;
```

### `RHF_GUARANTEE_START_INIT`

```rust
const RHF_GUARANTEE_START_INIT: u32 = 128u32;
```

### `RHF_PIXIE`

```rust
const RHF_PIXIE: u32 = 256u32;
```

### `RHF_DEFAULT_DELAY_LOAD`

```rust
const RHF_DEFAULT_DELAY_LOAD: u32 = 512u32;
```

### `RHF_REQUICKSTART`

```rust
const RHF_REQUICKSTART: u32 = 1_024u32;
```

### `RHF_REQUICKSTARTED`

```rust
const RHF_REQUICKSTARTED: u32 = 2_048u32;
```

### `RHF_CORD`

```rust
const RHF_CORD: u32 = 4_096u32;
```

### `RHF_NO_UNRES_UNDEF`

```rust
const RHF_NO_UNRES_UNDEF: u32 = 8_192u32;
```

### `RHF_RLD_ORDER_SAFE`

```rust
const RHF_RLD_ORDER_SAFE: u32 = 16_384u32;
```

### `LL_NONE`

```rust
const LL_NONE: u32 = 0u32;
```

### `LL_EXACT_MATCH`

```rust
const LL_EXACT_MATCH: u32 = 1u32;
```

Require exact match

### `LL_IGNORE_INT_VER`

```rust
const LL_IGNORE_INT_VER: u32 = 2u32;
```

Ignore interface version

### `LL_REQUIRE_MINOR`

```rust
const LL_REQUIRE_MINOR: u32 = 4u32;
```

### `LL_EXPORTS`

```rust
const LL_EXPORTS: u32 = 8u32;
```

### `LL_DELAY_LOAD`

```rust
const LL_DELAY_LOAD: u32 = 16u32;
```

### `LL_DELTA`

```rust
const LL_DELTA: u32 = 32u32;
```

### `EF_PARISC_TRAPNIL`

```rust
const EF_PARISC_TRAPNIL: u32 = 65_536u32;
```

Trap nil pointer dereference.

### `EF_PARISC_EXT`

```rust
const EF_PARISC_EXT: u32 = 131_072u32;
```

Program uses arch. extensions.

### `EF_PARISC_LSB`

```rust
const EF_PARISC_LSB: u32 = 262_144u32;
```

Program expects little endian.

### `EF_PARISC_WIDE`

```rust
const EF_PARISC_WIDE: u32 = 524_288u32;
```

Program expects wide mode.

### `EF_PARISC_NO_KABP`

```rust
const EF_PARISC_NO_KABP: u32 = 1_048_576u32;
```

No kernel assisted branch prediction.

### `EF_PARISC_LAZYSWAP`

```rust
const EF_PARISC_LAZYSWAP: u32 = 4_194_304u32;
```

Allow lazy swapping.

### `EF_PARISC_ARCH`

```rust
const EF_PARISC_ARCH: u32 = 65_535u32;
```

Architecture version.

### `EFA_PARISC_1_0`

```rust
const EFA_PARISC_1_0: u32 = 523u32;
```

PA-RISC 1.0 big-endian.

### `EFA_PARISC_1_1`

```rust
const EFA_PARISC_1_1: u32 = 528u32;
```

PA-RISC 1.1 big-endian.

### `EFA_PARISC_2_0`

```rust
const EFA_PARISC_2_0: u32 = 532u32;
```

PA-RISC 2.0 big-endian.

### `SHN_PARISC_ANSI_COMMON`

```rust
const SHN_PARISC_ANSI_COMMON: u16 = 65_280u16;
```

Section for tentatively declared symbols in ANSI C.

### `SHN_PARISC_HUGE_COMMON`

```rust
const SHN_PARISC_HUGE_COMMON: u16 = 65_281u16;
```

Common blocks in huge model.

### `SHT_PARISC_EXT`

```rust
const SHT_PARISC_EXT: u32 = 1_879_048_192u32;
```

Contains product specific ext.

### `SHT_PARISC_UNWIND`

```rust
const SHT_PARISC_UNWIND: u32 = 1_879_048_193u32;
```

Unwind information.

### `SHT_PARISC_DOC`

```rust
const SHT_PARISC_DOC: u32 = 1_879_048_194u32;
```

Debug info for optimized code.

### `SHF_PARISC_SHORT`

```rust
const SHF_PARISC_SHORT: u32 = 536_870_912u32;
```

Section with short addressing.

### `SHF_PARISC_HUGE`

```rust
const SHF_PARISC_HUGE: u32 = 1_073_741_824u32;
```

Section far from gp.

### `SHF_PARISC_SBP`

```rust
const SHF_PARISC_SBP: u32 = 2_147_483_648u32;
```

Static branch prediction code.

### `STT_PARISC_MILLICODE`

```rust
const STT_PARISC_MILLICODE: u8 = 13u8;
```

Millicode function entry point.

### `STT_HP_OPAQUE`

```rust
const STT_HP_OPAQUE: u8 = 11u8;
```

### `STT_HP_STUB`

```rust
const STT_HP_STUB: u8 = 12u8;
```

### `R_PARISC_NONE`

```rust
const R_PARISC_NONE: u32 = 0u32;
```

No reloc.

### `R_PARISC_DIR32`

```rust
const R_PARISC_DIR32: u32 = 1u32;
```

Direct 32-bit reference.

### `R_PARISC_DIR21L`

```rust
const R_PARISC_DIR21L: u32 = 2u32;
```

Left 21 bits of eff. address.

### `R_PARISC_DIR17R`

```rust
const R_PARISC_DIR17R: u32 = 3u32;
```

Right 17 bits of eff. address.

### `R_PARISC_DIR17F`

```rust
const R_PARISC_DIR17F: u32 = 4u32;
```

17 bits of eff. address.

### `R_PARISC_DIR14R`

```rust
const R_PARISC_DIR14R: u32 = 6u32;
```

Right 14 bits of eff. address.

### `R_PARISC_PCREL32`

```rust
const R_PARISC_PCREL32: u32 = 9u32;
```

32-bit rel. address.

### `R_PARISC_PCREL21L`

```rust
const R_PARISC_PCREL21L: u32 = 10u32;
```

Left 21 bits of rel. address.

### `R_PARISC_PCREL17R`

```rust
const R_PARISC_PCREL17R: u32 = 11u32;
```

Right 17 bits of rel. address.

### `R_PARISC_PCREL17F`

```rust
const R_PARISC_PCREL17F: u32 = 12u32;
```

17 bits of rel. address.

### `R_PARISC_PCREL14R`

```rust
const R_PARISC_PCREL14R: u32 = 14u32;
```

Right 14 bits of rel. address.

### `R_PARISC_DPREL21L`

```rust
const R_PARISC_DPREL21L: u32 = 18u32;
```

Left 21 bits of rel. address.

### `R_PARISC_DPREL14R`

```rust
const R_PARISC_DPREL14R: u32 = 22u32;
```

Right 14 bits of rel. address.

### `R_PARISC_GPREL21L`

```rust
const R_PARISC_GPREL21L: u32 = 26u32;
```

GP-relative, left 21 bits.

### `R_PARISC_GPREL14R`

```rust
const R_PARISC_GPREL14R: u32 = 30u32;
```

GP-relative, right 14 bits.

### `R_PARISC_LTOFF21L`

```rust
const R_PARISC_LTOFF21L: u32 = 34u32;
```

LT-relative, left 21 bits.

### `R_PARISC_LTOFF14R`

```rust
const R_PARISC_LTOFF14R: u32 = 38u32;
```

LT-relative, right 14 bits.

### `R_PARISC_SECREL32`

```rust
const R_PARISC_SECREL32: u32 = 41u32;
```

32 bits section rel. address.

### `R_PARISC_SEGBASE`

```rust
const R_PARISC_SEGBASE: u32 = 48u32;
```

No relocation, set segment base.

### `R_PARISC_SEGREL32`

```rust
const R_PARISC_SEGREL32: u32 = 49u32;
```

32 bits segment rel. address.

### `R_PARISC_PLTOFF21L`

```rust
const R_PARISC_PLTOFF21L: u32 = 50u32;
```

PLT rel. address, left 21 bits.

### `R_PARISC_PLTOFF14R`

```rust
const R_PARISC_PLTOFF14R: u32 = 54u32;
```

PLT rel. address, right 14 bits.

### `R_PARISC_LTOFF_FPTR32`

```rust
const R_PARISC_LTOFF_FPTR32: u32 = 57u32;
```

32 bits LT-rel. function pointer.

### `R_PARISC_LTOFF_FPTR21L`

```rust
const R_PARISC_LTOFF_FPTR21L: u32 = 58u32;
```

LT-rel. fct ptr, left 21 bits.

### `R_PARISC_LTOFF_FPTR14R`

```rust
const R_PARISC_LTOFF_FPTR14R: u32 = 62u32;
```

LT-rel. fct ptr, right 14 bits.

### `R_PARISC_FPTR64`

```rust
const R_PARISC_FPTR64: u32 = 64u32;
```

64 bits function address.

### `R_PARISC_PLABEL32`

```rust
const R_PARISC_PLABEL32: u32 = 65u32;
```

32 bits function address.

### `R_PARISC_PLABEL21L`

```rust
const R_PARISC_PLABEL21L: u32 = 66u32;
```

Left 21 bits of fdesc address.

### `R_PARISC_PLABEL14R`

```rust
const R_PARISC_PLABEL14R: u32 = 70u32;
```

Right 14 bits of fdesc address.

### `R_PARISC_PCREL64`

```rust
const R_PARISC_PCREL64: u32 = 72u32;
```

64 bits PC-rel. address.

### `R_PARISC_PCREL22F`

```rust
const R_PARISC_PCREL22F: u32 = 74u32;
```

22 bits PC-rel. address.

### `R_PARISC_PCREL14WR`

```rust
const R_PARISC_PCREL14WR: u32 = 75u32;
```

PC-rel. address, right 14 bits.

### `R_PARISC_PCREL14DR`

```rust
const R_PARISC_PCREL14DR: u32 = 76u32;
```

PC rel. address, right 14 bits.

### `R_PARISC_PCREL16F`

```rust
const R_PARISC_PCREL16F: u32 = 77u32;
```

16 bits PC-rel. address.

### `R_PARISC_PCREL16WF`

```rust
const R_PARISC_PCREL16WF: u32 = 78u32;
```

16 bits PC-rel. address.

### `R_PARISC_PCREL16DF`

```rust
const R_PARISC_PCREL16DF: u32 = 79u32;
```

16 bits PC-rel. address.

### `R_PARISC_DIR64`

```rust
const R_PARISC_DIR64: u32 = 80u32;
```

64 bits of eff. address.

### `R_PARISC_DIR14WR`

```rust
const R_PARISC_DIR14WR: u32 = 83u32;
```

14 bits of eff. address.

### `R_PARISC_DIR14DR`

```rust
const R_PARISC_DIR14DR: u32 = 84u32;
```

14 bits of eff. address.

### `R_PARISC_DIR16F`

```rust
const R_PARISC_DIR16F: u32 = 85u32;
```

16 bits of eff. address.

### `R_PARISC_DIR16WF`

```rust
const R_PARISC_DIR16WF: u32 = 86u32;
```

16 bits of eff. address.

### `R_PARISC_DIR16DF`

```rust
const R_PARISC_DIR16DF: u32 = 87u32;
```

16 bits of eff. address.

### `R_PARISC_GPREL64`

```rust
const R_PARISC_GPREL64: u32 = 88u32;
```

64 bits of GP-rel. address.

### `R_PARISC_GPREL14WR`

```rust
const R_PARISC_GPREL14WR: u32 = 91u32;
```

GP-rel. address, right 14 bits.

### `R_PARISC_GPREL14DR`

```rust
const R_PARISC_GPREL14DR: u32 = 92u32;
```

GP-rel. address, right 14 bits.

### `R_PARISC_GPREL16F`

```rust
const R_PARISC_GPREL16F: u32 = 93u32;
```

16 bits GP-rel. address.

### `R_PARISC_GPREL16WF`

```rust
const R_PARISC_GPREL16WF: u32 = 94u32;
```

16 bits GP-rel. address.

### `R_PARISC_GPREL16DF`

```rust
const R_PARISC_GPREL16DF: u32 = 95u32;
```

16 bits GP-rel. address.

### `R_PARISC_LTOFF64`

```rust
const R_PARISC_LTOFF64: u32 = 96u32;
```

64 bits LT-rel. address.

### `R_PARISC_LTOFF14WR`

```rust
const R_PARISC_LTOFF14WR: u32 = 99u32;
```

LT-rel. address, right 14 bits.

### `R_PARISC_LTOFF14DR`

```rust
const R_PARISC_LTOFF14DR: u32 = 100u32;
```

LT-rel. address, right 14 bits.

### `R_PARISC_LTOFF16F`

```rust
const R_PARISC_LTOFF16F: u32 = 101u32;
```

16 bits LT-rel. address.

### `R_PARISC_LTOFF16WF`

```rust
const R_PARISC_LTOFF16WF: u32 = 102u32;
```

16 bits LT-rel. address.

### `R_PARISC_LTOFF16DF`

```rust
const R_PARISC_LTOFF16DF: u32 = 103u32;
```

16 bits LT-rel. address.

### `R_PARISC_SECREL64`

```rust
const R_PARISC_SECREL64: u32 = 104u32;
```

64 bits section rel. address.

### `R_PARISC_SEGREL64`

```rust
const R_PARISC_SEGREL64: u32 = 112u32;
```

64 bits segment rel. address.

### `R_PARISC_PLTOFF14WR`

```rust
const R_PARISC_PLTOFF14WR: u32 = 115u32;
```

PLT-rel. address, right 14 bits.

### `R_PARISC_PLTOFF14DR`

```rust
const R_PARISC_PLTOFF14DR: u32 = 116u32;
```

PLT-rel. address, right 14 bits.

### `R_PARISC_PLTOFF16F`

```rust
const R_PARISC_PLTOFF16F: u32 = 117u32;
```

16 bits LT-rel. address.

### `R_PARISC_PLTOFF16WF`

```rust
const R_PARISC_PLTOFF16WF: u32 = 118u32;
```

16 bits PLT-rel. address.

### `R_PARISC_PLTOFF16DF`

```rust
const R_PARISC_PLTOFF16DF: u32 = 119u32;
```

16 bits PLT-rel. address.

### `R_PARISC_LTOFF_FPTR64`

```rust
const R_PARISC_LTOFF_FPTR64: u32 = 120u32;
```

64 bits LT-rel. function ptr.

### `R_PARISC_LTOFF_FPTR14WR`

```rust
const R_PARISC_LTOFF_FPTR14WR: u32 = 123u32;
```

LT-rel. fct. ptr., right 14 bits.

### `R_PARISC_LTOFF_FPTR14DR`

```rust
const R_PARISC_LTOFF_FPTR14DR: u32 = 124u32;
```

LT-rel. fct. ptr., right 14 bits.

### `R_PARISC_LTOFF_FPTR16F`

```rust
const R_PARISC_LTOFF_FPTR16F: u32 = 125u32;
```

16 bits LT-rel. function ptr.

### `R_PARISC_LTOFF_FPTR16WF`

```rust
const R_PARISC_LTOFF_FPTR16WF: u32 = 126u32;
```

16 bits LT-rel. function ptr.

### `R_PARISC_LTOFF_FPTR16DF`

```rust
const R_PARISC_LTOFF_FPTR16DF: u32 = 127u32;
```

16 bits LT-rel. function ptr.

### `R_PARISC_LORESERVE`

```rust
const R_PARISC_LORESERVE: u32 = 128u32;
```

### `R_PARISC_COPY`

```rust
const R_PARISC_COPY: u32 = 128u32;
```

Copy relocation.

### `R_PARISC_IPLT`

```rust
const R_PARISC_IPLT: u32 = 129u32;
```

Dynamic reloc, imported PLT

### `R_PARISC_EPLT`

```rust
const R_PARISC_EPLT: u32 = 130u32;
```

Dynamic reloc, exported PLT

### `R_PARISC_TPREL32`

```rust
const R_PARISC_TPREL32: u32 = 153u32;
```

32 bits TP-rel. address.

### `R_PARISC_TPREL21L`

```rust
const R_PARISC_TPREL21L: u32 = 154u32;
```

TP-rel. address, left 21 bits.

### `R_PARISC_TPREL14R`

```rust
const R_PARISC_TPREL14R: u32 = 158u32;
```

TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP21L`

```rust
const R_PARISC_LTOFF_TP21L: u32 = 162u32;
```

LT-TP-rel. address, left 21 bits.

### `R_PARISC_LTOFF_TP14R`

```rust
const R_PARISC_LTOFF_TP14R: u32 = 166u32;
```

LT-TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP14F`

```rust
const R_PARISC_LTOFF_TP14F: u32 = 167u32;
```

14 bits LT-TP-rel. address.

### `R_PARISC_TPREL64`

```rust
const R_PARISC_TPREL64: u32 = 216u32;
```

64 bits TP-rel. address.

### `R_PARISC_TPREL14WR`

```rust
const R_PARISC_TPREL14WR: u32 = 219u32;
```

TP-rel. address, right 14 bits.

### `R_PARISC_TPREL14DR`

```rust
const R_PARISC_TPREL14DR: u32 = 220u32;
```

TP-rel. address, right 14 bits.

### `R_PARISC_TPREL16F`

```rust
const R_PARISC_TPREL16F: u32 = 221u32;
```

16 bits TP-rel. address.

### `R_PARISC_TPREL16WF`

```rust
const R_PARISC_TPREL16WF: u32 = 222u32;
```

16 bits TP-rel. address.

### `R_PARISC_TPREL16DF`

```rust
const R_PARISC_TPREL16DF: u32 = 223u32;
```

16 bits TP-rel. address.

### `R_PARISC_LTOFF_TP64`

```rust
const R_PARISC_LTOFF_TP64: u32 = 224u32;
```

64 bits LT-TP-rel. address.

### `R_PARISC_LTOFF_TP14WR`

```rust
const R_PARISC_LTOFF_TP14WR: u32 = 227u32;
```

LT-TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP14DR`

```rust
const R_PARISC_LTOFF_TP14DR: u32 = 228u32;
```

LT-TP-rel. address, right 14 bits.

### `R_PARISC_LTOFF_TP16F`

```rust
const R_PARISC_LTOFF_TP16F: u32 = 229u32;
```

16 bits LT-TP-rel. address.

### `R_PARISC_LTOFF_TP16WF`

```rust
const R_PARISC_LTOFF_TP16WF: u32 = 230u32;
```

16 bits LT-TP-rel. address.

### `R_PARISC_LTOFF_TP16DF`

```rust
const R_PARISC_LTOFF_TP16DF: u32 = 231u32;
```

16 bits LT-TP-rel. address.

### `R_PARISC_GNU_VTENTRY`

```rust
const R_PARISC_GNU_VTENTRY: u32 = 232u32;
```

### `R_PARISC_GNU_VTINHERIT`

```rust
const R_PARISC_GNU_VTINHERIT: u32 = 233u32;
```

### `R_PARISC_TLS_GD21L`

```rust
const R_PARISC_TLS_GD21L: u32 = 234u32;
```

GD 21-bit left.

### `R_PARISC_TLS_GD14R`

```rust
const R_PARISC_TLS_GD14R: u32 = 235u32;
```

GD 14-bit right.

### `R_PARISC_TLS_GDCALL`

```rust
const R_PARISC_TLS_GDCALL: u32 = 236u32;
```

GD call to __t_g_a.

### `R_PARISC_TLS_LDM21L`

```rust
const R_PARISC_TLS_LDM21L: u32 = 237u32;
```

LD module 21-bit left.

### `R_PARISC_TLS_LDM14R`

```rust
const R_PARISC_TLS_LDM14R: u32 = 238u32;
```

LD module 14-bit right.

### `R_PARISC_TLS_LDMCALL`

```rust
const R_PARISC_TLS_LDMCALL: u32 = 239u32;
```

LD module call to __t_g_a.

### `R_PARISC_TLS_LDO21L`

```rust
const R_PARISC_TLS_LDO21L: u32 = 240u32;
```

LD offset 21-bit left.

### `R_PARISC_TLS_LDO14R`

```rust
const R_PARISC_TLS_LDO14R: u32 = 241u32;
```

LD offset 14-bit right.

### `R_PARISC_TLS_DTPMOD32`

```rust
const R_PARISC_TLS_DTPMOD32: u32 = 242u32;
```

DTP module 32-bit.

### `R_PARISC_TLS_DTPMOD64`

```rust
const R_PARISC_TLS_DTPMOD64: u32 = 243u32;
```

DTP module 64-bit.

### `R_PARISC_TLS_DTPOFF32`

```rust
const R_PARISC_TLS_DTPOFF32: u32 = 244u32;
```

DTP offset 32-bit.

### `R_PARISC_TLS_DTPOFF64`

```rust
const R_PARISC_TLS_DTPOFF64: u32 = 245u32;
```

DTP offset 32-bit.

### `R_PARISC_TLS_LE21L`

```rust
const R_PARISC_TLS_LE21L: u32 = 154u32;
```

### `R_PARISC_TLS_LE14R`

```rust
const R_PARISC_TLS_LE14R: u32 = 158u32;
```

### `R_PARISC_TLS_IE21L`

```rust
const R_PARISC_TLS_IE21L: u32 = 162u32;
```

### `R_PARISC_TLS_IE14R`

```rust
const R_PARISC_TLS_IE14R: u32 = 166u32;
```

### `R_PARISC_TLS_TPREL32`

```rust
const R_PARISC_TLS_TPREL32: u32 = 153u32;
```

### `R_PARISC_TLS_TPREL64`

```rust
const R_PARISC_TLS_TPREL64: u32 = 216u32;
```

### `R_PARISC_HIRESERVE`

```rust
const R_PARISC_HIRESERVE: u32 = 255u32;
```

### `PT_HP_TLS`

```rust
const PT_HP_TLS: u32 = 1_610_612_736u32;
```

### `PT_HP_CORE_NONE`

```rust
const PT_HP_CORE_NONE: u32 = 1_610_612_737u32;
```

### `PT_HP_CORE_VERSION`

```rust
const PT_HP_CORE_VERSION: u32 = 1_610_612_738u32;
```

### `PT_HP_CORE_KERNEL`

```rust
const PT_HP_CORE_KERNEL: u32 = 1_610_612_739u32;
```

### `PT_HP_CORE_COMM`

```rust
const PT_HP_CORE_COMM: u32 = 1_610_612_740u32;
```

### `PT_HP_CORE_PROC`

```rust
const PT_HP_CORE_PROC: u32 = 1_610_612_741u32;
```

### `PT_HP_CORE_LOADABLE`

```rust
const PT_HP_CORE_LOADABLE: u32 = 1_610_612_742u32;
```

### `PT_HP_CORE_STACK`

```rust
const PT_HP_CORE_STACK: u32 = 1_610_612_743u32;
```

### `PT_HP_CORE_SHM`

```rust
const PT_HP_CORE_SHM: u32 = 1_610_612_744u32;
```

### `PT_HP_CORE_MMF`

```rust
const PT_HP_CORE_MMF: u32 = 1_610_612_745u32;
```

### `PT_HP_PARALLEL`

```rust
const PT_HP_PARALLEL: u32 = 1_610_612_752u32;
```

### `PT_HP_FASTBIND`

```rust
const PT_HP_FASTBIND: u32 = 1_610_612_753u32;
```

### `PT_HP_OPT_ANNOT`

```rust
const PT_HP_OPT_ANNOT: u32 = 1_610_612_754u32;
```

### `PT_HP_HSL_ANNOT`

```rust
const PT_HP_HSL_ANNOT: u32 = 1_610_612_755u32;
```

### `PT_HP_STACK`

```rust
const PT_HP_STACK: u32 = 1_610_612_756u32;
```

### `PT_PARISC_ARCHEXT`

```rust
const PT_PARISC_ARCHEXT: u32 = 1_879_048_192u32;
```

### `PT_PARISC_UNWIND`

```rust
const PT_PARISC_UNWIND: u32 = 1_879_048_193u32;
```

### `PF_PARISC_SBP`

```rust
const PF_PARISC_SBP: u32 = 134_217_728u32;
```

### `PF_HP_PAGE_SIZE`

```rust
const PF_HP_PAGE_SIZE: u32 = 1_048_576u32;
```

### `PF_HP_FAR_SHARED`

```rust
const PF_HP_FAR_SHARED: u32 = 2_097_152u32;
```

### `PF_HP_NEAR_SHARED`

```rust
const PF_HP_NEAR_SHARED: u32 = 4_194_304u32;
```

### `PF_HP_CODE`

```rust
const PF_HP_CODE: u32 = 16_777_216u32;
```

### `PF_HP_MODIFY`

```rust
const PF_HP_MODIFY: u32 = 33_554_432u32;
```

### `PF_HP_LAZYSWAP`

```rust
const PF_HP_LAZYSWAP: u32 = 67_108_864u32;
```

### `PF_HP_SBP`

```rust
const PF_HP_SBP: u32 = 134_217_728u32;
```

### `EF_ALPHA_32BIT`

```rust
const EF_ALPHA_32BIT: u32 = 1u32;
```

All addresses must be < 2GB.

### `EF_ALPHA_CANRELAX`

```rust
const EF_ALPHA_CANRELAX: u32 = 2u32;
```

Relocations for relaxing exist.

### `SHT_ALPHA_DEBUG`

```rust
const SHT_ALPHA_DEBUG: u32 = 1_879_048_193u32;
```

### `SHT_ALPHA_REGINFO`

```rust
const SHT_ALPHA_REGINFO: u32 = 1_879_048_194u32;
```

### `SHF_ALPHA_GPREL`

```rust
const SHF_ALPHA_GPREL: u32 = 268_435_456u32;
```

### `STO_ALPHA_NOPV`

```rust
const STO_ALPHA_NOPV: u8 = 128u8;
```

No PV required.

### `STO_ALPHA_STD_GPLOAD`

```rust
const STO_ALPHA_STD_GPLOAD: u8 = 136u8;
```

PV only used for initial ldgp.

### `R_ALPHA_NONE`

```rust
const R_ALPHA_NONE: u32 = 0u32;
```

No reloc

### `R_ALPHA_REFLONG`

```rust
const R_ALPHA_REFLONG: u32 = 1u32;
```

Direct 32 bit

### `R_ALPHA_REFQUAD`

```rust
const R_ALPHA_REFQUAD: u32 = 2u32;
```

Direct 64 bit

### `R_ALPHA_GPREL32`

```rust
const R_ALPHA_GPREL32: u32 = 3u32;
```

GP relative 32 bit

### `R_ALPHA_LITERAL`

```rust
const R_ALPHA_LITERAL: u32 = 4u32;
```

GP relative 16 bit w/optimization

### `R_ALPHA_LITUSE`

```rust
const R_ALPHA_LITUSE: u32 = 5u32;
```

Optimization hint for LITERAL

### `R_ALPHA_GPDISP`

```rust
const R_ALPHA_GPDISP: u32 = 6u32;
```

Add displacement to GP

### `R_ALPHA_BRADDR`

```rust
const R_ALPHA_BRADDR: u32 = 7u32;
```

PC+4 relative 23 bit shifted

### `R_ALPHA_HINT`

```rust
const R_ALPHA_HINT: u32 = 8u32;
```

PC+4 relative 16 bit shifted

### `R_ALPHA_SREL16`

```rust
const R_ALPHA_SREL16: u32 = 9u32;
```

PC relative 16 bit

### `R_ALPHA_SREL32`

```rust
const R_ALPHA_SREL32: u32 = 10u32;
```

PC relative 32 bit

### `R_ALPHA_SREL64`

```rust
const R_ALPHA_SREL64: u32 = 11u32;
```

PC relative 64 bit

### `R_ALPHA_GPRELHIGH`

```rust
const R_ALPHA_GPRELHIGH: u32 = 17u32;
```

GP relative 32 bit, high 16 bits

### `R_ALPHA_GPRELLOW`

```rust
const R_ALPHA_GPRELLOW: u32 = 18u32;
```

GP relative 32 bit, low 16 bits

### `R_ALPHA_GPREL16`

```rust
const R_ALPHA_GPREL16: u32 = 19u32;
```

GP relative 16 bit

### `R_ALPHA_COPY`

```rust
const R_ALPHA_COPY: u32 = 24u32;
```

Copy symbol at runtime

### `R_ALPHA_GLOB_DAT`

```rust
const R_ALPHA_GLOB_DAT: u32 = 25u32;
```

Create GOT entry

### `R_ALPHA_JMP_SLOT`

```rust
const R_ALPHA_JMP_SLOT: u32 = 26u32;
```

Create PLT entry

### `R_ALPHA_RELATIVE`

```rust
const R_ALPHA_RELATIVE: u32 = 27u32;
```

Adjust by program base

### `R_ALPHA_TLS_GD_HI`

```rust
const R_ALPHA_TLS_GD_HI: u32 = 28u32;
```

### `R_ALPHA_TLSGD`

```rust
const R_ALPHA_TLSGD: u32 = 29u32;
```

### `R_ALPHA_TLS_LDM`

```rust
const R_ALPHA_TLS_LDM: u32 = 30u32;
```

### `R_ALPHA_DTPMOD64`

```rust
const R_ALPHA_DTPMOD64: u32 = 31u32;
```

### `R_ALPHA_GOTDTPREL`

```rust
const R_ALPHA_GOTDTPREL: u32 = 32u32;
```

### `R_ALPHA_DTPREL64`

```rust
const R_ALPHA_DTPREL64: u32 = 33u32;
```

### `R_ALPHA_DTPRELHI`

```rust
const R_ALPHA_DTPRELHI: u32 = 34u32;
```

### `R_ALPHA_DTPRELLO`

```rust
const R_ALPHA_DTPRELLO: u32 = 35u32;
```

### `R_ALPHA_DTPREL16`

```rust
const R_ALPHA_DTPREL16: u32 = 36u32;
```

### `R_ALPHA_GOTTPREL`

```rust
const R_ALPHA_GOTTPREL: u32 = 37u32;
```

### `R_ALPHA_TPREL64`

```rust
const R_ALPHA_TPREL64: u32 = 38u32;
```

### `R_ALPHA_TPRELHI`

```rust
const R_ALPHA_TPRELHI: u32 = 39u32;
```

### `R_ALPHA_TPRELLO`

```rust
const R_ALPHA_TPRELLO: u32 = 40u32;
```

### `R_ALPHA_TPREL16`

```rust
const R_ALPHA_TPREL16: u32 = 41u32;
```

### `LITUSE_ALPHA_ADDR`

```rust
const LITUSE_ALPHA_ADDR: u32 = 0u32;
```

### `LITUSE_ALPHA_BASE`

```rust
const LITUSE_ALPHA_BASE: u32 = 1u32;
```

### `LITUSE_ALPHA_BYTOFF`

```rust
const LITUSE_ALPHA_BYTOFF: u32 = 2u32;
```

### `LITUSE_ALPHA_JSR`

```rust
const LITUSE_ALPHA_JSR: u32 = 3u32;
```

### `LITUSE_ALPHA_TLS_GD`

```rust
const LITUSE_ALPHA_TLS_GD: u32 = 4u32;
```

### `LITUSE_ALPHA_TLS_LDM`

```rust
const LITUSE_ALPHA_TLS_LDM: u32 = 5u32;
```

### `DT_ALPHA_PLTRO`

```rust
const DT_ALPHA_PLTRO: u32 = 1_879_048_192u32;
```

### `EF_PPC_EMB`

```rust
const EF_PPC_EMB: u32 = 2_147_483_648u32;
```

PowerPC embedded flag

### `EF_PPC_RELOCATABLE`

```rust
const EF_PPC_RELOCATABLE: u32 = 65_536u32;
```

PowerPC -mrelocatable flag

### `EF_PPC_RELOCATABLE_LIB`

```rust
const EF_PPC_RELOCATABLE_LIB: u32 = 32_768u32;
```

PowerPC -mrelocatable-lib flag

### `R_PPC_NONE`

```rust
const R_PPC_NONE: u32 = 0u32;
```

### `R_PPC_ADDR32`

```rust
const R_PPC_ADDR32: u32 = 1u32;
```

32bit absolute address

### `R_PPC_ADDR24`

```rust
const R_PPC_ADDR24: u32 = 2u32;
```

26bit address, 2 bits ignored.

### `R_PPC_ADDR16`

```rust
const R_PPC_ADDR16: u32 = 3u32;
```

16bit absolute address

### `R_PPC_ADDR16_LO`

```rust
const R_PPC_ADDR16_LO: u32 = 4u32;
```

lower 16bit of absolute address

### `R_PPC_ADDR16_HI`

```rust
const R_PPC_ADDR16_HI: u32 = 5u32;
```

high 16bit of absolute address

### `R_PPC_ADDR16_HA`

```rust
const R_PPC_ADDR16_HA: u32 = 6u32;
```

adjusted high 16bit

### `R_PPC_ADDR14`

```rust
const R_PPC_ADDR14: u32 = 7u32;
```

16bit address, 2 bits ignored

### `R_PPC_ADDR14_BRTAKEN`

```rust
const R_PPC_ADDR14_BRTAKEN: u32 = 8u32;
```

### `R_PPC_ADDR14_BRNTAKEN`

```rust
const R_PPC_ADDR14_BRNTAKEN: u32 = 9u32;
```

### `R_PPC_REL24`

```rust
const R_PPC_REL24: u32 = 10u32;
```

PC relative 26 bit

### `R_PPC_REL14`

```rust
const R_PPC_REL14: u32 = 11u32;
```

PC relative 16 bit

### `R_PPC_REL14_BRTAKEN`

```rust
const R_PPC_REL14_BRTAKEN: u32 = 12u32;
```

### `R_PPC_REL14_BRNTAKEN`

```rust
const R_PPC_REL14_BRNTAKEN: u32 = 13u32;
```

### `R_PPC_GOT16`

```rust
const R_PPC_GOT16: u32 = 14u32;
```

### `R_PPC_GOT16_LO`

```rust
const R_PPC_GOT16_LO: u32 = 15u32;
```

### `R_PPC_GOT16_HI`

```rust
const R_PPC_GOT16_HI: u32 = 16u32;
```

### `R_PPC_GOT16_HA`

```rust
const R_PPC_GOT16_HA: u32 = 17u32;
```

### `R_PPC_PLTREL24`

```rust
const R_PPC_PLTREL24: u32 = 18u32;
```

### `R_PPC_COPY`

```rust
const R_PPC_COPY: u32 = 19u32;
```

### `R_PPC_GLOB_DAT`

```rust
const R_PPC_GLOB_DAT: u32 = 20u32;
```

### `R_PPC_JMP_SLOT`

```rust
const R_PPC_JMP_SLOT: u32 = 21u32;
```

### `R_PPC_RELATIVE`

```rust
const R_PPC_RELATIVE: u32 = 22u32;
```

### `R_PPC_LOCAL24PC`

```rust
const R_PPC_LOCAL24PC: u32 = 23u32;
```

### `R_PPC_UADDR32`

```rust
const R_PPC_UADDR32: u32 = 24u32;
```

### `R_PPC_UADDR16`

```rust
const R_PPC_UADDR16: u32 = 25u32;
```

### `R_PPC_REL32`

```rust
const R_PPC_REL32: u32 = 26u32;
```

### `R_PPC_PLT32`

```rust
const R_PPC_PLT32: u32 = 27u32;
```

### `R_PPC_PLTREL32`

```rust
const R_PPC_PLTREL32: u32 = 28u32;
```

### `R_PPC_PLT16_LO`

```rust
const R_PPC_PLT16_LO: u32 = 29u32;
```

### `R_PPC_PLT16_HI`

```rust
const R_PPC_PLT16_HI: u32 = 30u32;
```

### `R_PPC_PLT16_HA`

```rust
const R_PPC_PLT16_HA: u32 = 31u32;
```

### `R_PPC_SDAREL16`

```rust
const R_PPC_SDAREL16: u32 = 32u32;
```

### `R_PPC_SECTOFF`

```rust
const R_PPC_SECTOFF: u32 = 33u32;
```

### `R_PPC_SECTOFF_LO`

```rust
const R_PPC_SECTOFF_LO: u32 = 34u32;
```

### `R_PPC_SECTOFF_HI`

```rust
const R_PPC_SECTOFF_HI: u32 = 35u32;
```

### `R_PPC_SECTOFF_HA`

```rust
const R_PPC_SECTOFF_HA: u32 = 36u32;
```

### `R_PPC_TLS`

```rust
const R_PPC_TLS: u32 = 67u32;
```

none    (sym+add)@tls

### `R_PPC_DTPMOD32`

```rust
const R_PPC_DTPMOD32: u32 = 68u32;
```

word32  (sym+add)@dtpmod

### `R_PPC_TPREL16`

```rust
const R_PPC_TPREL16: u32 = 69u32;
```

half16* (sym+add)@tprel

### `R_PPC_TPREL16_LO`

```rust
const R_PPC_TPREL16_LO: u32 = 70u32;
```

half16  (sym+add)@tprel@l

### `R_PPC_TPREL16_HI`

```rust
const R_PPC_TPREL16_HI: u32 = 71u32;
```

half16  (sym+add)@tprel@h

### `R_PPC_TPREL16_HA`

```rust
const R_PPC_TPREL16_HA: u32 = 72u32;
```

half16  (sym+add)@tprel@ha

### `R_PPC_TPREL32`

```rust
const R_PPC_TPREL32: u32 = 73u32;
```

word32  (sym+add)@tprel

### `R_PPC_DTPREL16`

```rust
const R_PPC_DTPREL16: u32 = 74u32;
```

half16*(sym+add)@dtprel

### `R_PPC_DTPREL16_LO`

```rust
const R_PPC_DTPREL16_LO: u32 = 75u32;
```

half16  (sym+add)@dtprel@l

### `R_PPC_DTPREL16_HI`

```rust
const R_PPC_DTPREL16_HI: u32 = 76u32;
```

half16  (sym+add)@dtprel@h

### `R_PPC_DTPREL16_HA`

```rust
const R_PPC_DTPREL16_HA: u32 = 77u32;
```

half16  (sym+add)@dtprel@ha

### `R_PPC_DTPREL32`

```rust
const R_PPC_DTPREL32: u32 = 78u32;
```

word32  (sym+add)@dtprel

### `R_PPC_GOT_TLSGD16`

```rust
const R_PPC_GOT_TLSGD16: u32 = 79u32;
```

half16* (sym+add)@got@tlsgd

### `R_PPC_GOT_TLSGD16_LO`

```rust
const R_PPC_GOT_TLSGD16_LO: u32 = 80u32;
```

half16  (sym+add)@got@tlsgd@l

### `R_PPC_GOT_TLSGD16_HI`

```rust
const R_PPC_GOT_TLSGD16_HI: u32 = 81u32;
```

half16  (sym+add)@got@tlsgd@h

### `R_PPC_GOT_TLSGD16_HA`

```rust
const R_PPC_GOT_TLSGD16_HA: u32 = 82u32;
```

half16  (sym+add)@got@tlsgd@ha

### `R_PPC_GOT_TLSLD16`

```rust
const R_PPC_GOT_TLSLD16: u32 = 83u32;
```

half16* (sym+add)@got@tlsld

### `R_PPC_GOT_TLSLD16_LO`

```rust
const R_PPC_GOT_TLSLD16_LO: u32 = 84u32;
```

half16  (sym+add)@got@tlsld@l

### `R_PPC_GOT_TLSLD16_HI`

```rust
const R_PPC_GOT_TLSLD16_HI: u32 = 85u32;
```

half16  (sym+add)@got@tlsld@h

### `R_PPC_GOT_TLSLD16_HA`

```rust
const R_PPC_GOT_TLSLD16_HA: u32 = 86u32;
```

half16  (sym+add)@got@tlsld@ha

### `R_PPC_GOT_TPREL16`

```rust
const R_PPC_GOT_TPREL16: u32 = 87u32;
```

half16* (sym+add)@got@tprel

### `R_PPC_GOT_TPREL16_LO`

```rust
const R_PPC_GOT_TPREL16_LO: u32 = 88u32;
```

half16  (sym+add)@got@tprel@l

### `R_PPC_GOT_TPREL16_HI`

```rust
const R_PPC_GOT_TPREL16_HI: u32 = 89u32;
```

half16  (sym+add)@got@tprel@h

### `R_PPC_GOT_TPREL16_HA`

```rust
const R_PPC_GOT_TPREL16_HA: u32 = 90u32;
```

half16  (sym+add)@got@tprel@ha

### `R_PPC_GOT_DTPREL16`

```rust
const R_PPC_GOT_DTPREL16: u32 = 91u32;
```

half16* (sym+add)@got@dtprel

### `R_PPC_GOT_DTPREL16_LO`

```rust
const R_PPC_GOT_DTPREL16_LO: u32 = 92u32;
```

half16* (sym+add)@got@dtprel@l

### `R_PPC_GOT_DTPREL16_HI`

```rust
const R_PPC_GOT_DTPREL16_HI: u32 = 93u32;
```

half16* (sym+add)@got@dtprel@h

### `R_PPC_GOT_DTPREL16_HA`

```rust
const R_PPC_GOT_DTPREL16_HA: u32 = 94u32;
```

half16* (sym+add)@got@dtprel@ha

### `R_PPC_TLSGD`

```rust
const R_PPC_TLSGD: u32 = 95u32;
```

none    (sym+add)@tlsgd

### `R_PPC_TLSLD`

```rust
const R_PPC_TLSLD: u32 = 96u32;
```

none    (sym+add)@tlsld

### `R_PPC_EMB_NADDR32`

```rust
const R_PPC_EMB_NADDR32: u32 = 101u32;
```

### `R_PPC_EMB_NADDR16`

```rust
const R_PPC_EMB_NADDR16: u32 = 102u32;
```

### `R_PPC_EMB_NADDR16_LO`

```rust
const R_PPC_EMB_NADDR16_LO: u32 = 103u32;
```

### `R_PPC_EMB_NADDR16_HI`

```rust
const R_PPC_EMB_NADDR16_HI: u32 = 104u32;
```

### `R_PPC_EMB_NADDR16_HA`

```rust
const R_PPC_EMB_NADDR16_HA: u32 = 105u32;
```

### `R_PPC_EMB_SDAI16`

```rust
const R_PPC_EMB_SDAI16: u32 = 106u32;
```

### `R_PPC_EMB_SDA2I16`

```rust
const R_PPC_EMB_SDA2I16: u32 = 107u32;
```

### `R_PPC_EMB_SDA2REL`

```rust
const R_PPC_EMB_SDA2REL: u32 = 108u32;
```

### `R_PPC_EMB_SDA21`

```rust
const R_PPC_EMB_SDA21: u32 = 109u32;
```

16 bit offset in SDA

### `R_PPC_EMB_MRKREF`

```rust
const R_PPC_EMB_MRKREF: u32 = 110u32;
```

### `R_PPC_EMB_RELSEC16`

```rust
const R_PPC_EMB_RELSEC16: u32 = 111u32;
```

### `R_PPC_EMB_RELST_LO`

```rust
const R_PPC_EMB_RELST_LO: u32 = 112u32;
```

### `R_PPC_EMB_RELST_HI`

```rust
const R_PPC_EMB_RELST_HI: u32 = 113u32;
```

### `R_PPC_EMB_RELST_HA`

```rust
const R_PPC_EMB_RELST_HA: u32 = 114u32;
```

### `R_PPC_EMB_BIT_FLD`

```rust
const R_PPC_EMB_BIT_FLD: u32 = 115u32;
```

### `R_PPC_EMB_RELSDA`

```rust
const R_PPC_EMB_RELSDA: u32 = 116u32;
```

16 bit relative offset in SDA

### `R_PPC_DIAB_SDA21_LO`

```rust
const R_PPC_DIAB_SDA21_LO: u32 = 180u32;
```

like EMB_SDA21, but lower 16 bit

### `R_PPC_DIAB_SDA21_HI`

```rust
const R_PPC_DIAB_SDA21_HI: u32 = 181u32;
```

like EMB_SDA21, but high 16 bit

### `R_PPC_DIAB_SDA21_HA`

```rust
const R_PPC_DIAB_SDA21_HA: u32 = 182u32;
```

like EMB_SDA21, adjusted high 16

### `R_PPC_DIAB_RELSDA_LO`

```rust
const R_PPC_DIAB_RELSDA_LO: u32 = 183u32;
```

like EMB_RELSDA, but lower 16 bit

### `R_PPC_DIAB_RELSDA_HI`

```rust
const R_PPC_DIAB_RELSDA_HI: u32 = 184u32;
```

like EMB_RELSDA, but high 16 bit

### `R_PPC_DIAB_RELSDA_HA`

```rust
const R_PPC_DIAB_RELSDA_HA: u32 = 185u32;
```

like EMB_RELSDA, adjusted high 16

### `R_PPC_IRELATIVE`

```rust
const R_PPC_IRELATIVE: u32 = 248u32;
```

GNU extension to support local ifunc.

### `R_PPC_REL16`

```rust
const R_PPC_REL16: u32 = 249u32;
```

half16   (sym+add-.)

### `R_PPC_REL16_LO`

```rust
const R_PPC_REL16_LO: u32 = 250u32;
```

half16   (sym+add-.)@l

### `R_PPC_REL16_HI`

```rust
const R_PPC_REL16_HI: u32 = 251u32;
```

half16   (sym+add-.)@h

### `R_PPC_REL16_HA`

```rust
const R_PPC_REL16_HA: u32 = 252u32;
```

half16   (sym+add-.)@ha

### `R_PPC_TOC16`

```rust
const R_PPC_TOC16: u32 = 255u32;
```

This is a phony reloc to handle any old fashioned TOC16 references that may
still be in object files.

### `DT_PPC_GOT`

```rust
const DT_PPC_GOT: u32 = 1_879_048_192u32;
```

### `DT_PPC_OPT`

```rust
const DT_PPC_OPT: u32 = 1_879_048_193u32;
```

### `PPC_OPT_TLS`

```rust
const PPC_OPT_TLS: u32 = 1u32;
```

### `R_PPC64_NONE`

```rust
const R_PPC64_NONE: u32 = 0u32;
```

### `R_PPC64_ADDR32`

```rust
const R_PPC64_ADDR32: u32 = 1u32;
```

32bit absolute address

### `R_PPC64_ADDR24`

```rust
const R_PPC64_ADDR24: u32 = 2u32;
```

26bit address, word aligned

### `R_PPC64_ADDR16`

```rust
const R_PPC64_ADDR16: u32 = 3u32;
```

16bit absolute address

### `R_PPC64_ADDR16_LO`

```rust
const R_PPC64_ADDR16_LO: u32 = 4u32;
```

lower 16bits of address

### `R_PPC64_ADDR16_HI`

```rust
const R_PPC64_ADDR16_HI: u32 = 5u32;
```

high 16bits of address.

### `R_PPC64_ADDR16_HA`

```rust
const R_PPC64_ADDR16_HA: u32 = 6u32;
```

adjusted high 16bits.

### `R_PPC64_ADDR14`

```rust
const R_PPC64_ADDR14: u32 = 7u32;
```

16bit address, word aligned

### `R_PPC64_ADDR14_BRTAKEN`

```rust
const R_PPC64_ADDR14_BRTAKEN: u32 = 8u32;
```

### `R_PPC64_ADDR14_BRNTAKEN`

```rust
const R_PPC64_ADDR14_BRNTAKEN: u32 = 9u32;
```

### `R_PPC64_REL24`

```rust
const R_PPC64_REL24: u32 = 10u32;
```

PC-rel. 26 bit, word aligned

### `R_PPC64_REL14`

```rust
const R_PPC64_REL14: u32 = 11u32;
```

PC relative 16 bit

### `R_PPC64_REL14_BRTAKEN`

```rust
const R_PPC64_REL14_BRTAKEN: u32 = 12u32;
```

### `R_PPC64_REL14_BRNTAKEN`

```rust
const R_PPC64_REL14_BRNTAKEN: u32 = 13u32;
```

### `R_PPC64_GOT16`

```rust
const R_PPC64_GOT16: u32 = 14u32;
```

### `R_PPC64_GOT16_LO`

```rust
const R_PPC64_GOT16_LO: u32 = 15u32;
```

### `R_PPC64_GOT16_HI`

```rust
const R_PPC64_GOT16_HI: u32 = 16u32;
```

### `R_PPC64_GOT16_HA`

```rust
const R_PPC64_GOT16_HA: u32 = 17u32;
```

### `R_PPC64_COPY`

```rust
const R_PPC64_COPY: u32 = 19u32;
```

### `R_PPC64_GLOB_DAT`

```rust
const R_PPC64_GLOB_DAT: u32 = 20u32;
```

### `R_PPC64_JMP_SLOT`

```rust
const R_PPC64_JMP_SLOT: u32 = 21u32;
```

### `R_PPC64_RELATIVE`

```rust
const R_PPC64_RELATIVE: u32 = 22u32;
```

### `R_PPC64_UADDR32`

```rust
const R_PPC64_UADDR32: u32 = 24u32;
```

### `R_PPC64_UADDR16`

```rust
const R_PPC64_UADDR16: u32 = 25u32;
```

### `R_PPC64_REL32`

```rust
const R_PPC64_REL32: u32 = 26u32;
```

### `R_PPC64_PLT32`

```rust
const R_PPC64_PLT32: u32 = 27u32;
```

### `R_PPC64_PLTREL32`

```rust
const R_PPC64_PLTREL32: u32 = 28u32;
```

### `R_PPC64_PLT16_LO`

```rust
const R_PPC64_PLT16_LO: u32 = 29u32;
```

### `R_PPC64_PLT16_HI`

```rust
const R_PPC64_PLT16_HI: u32 = 30u32;
```

### `R_PPC64_PLT16_HA`

```rust
const R_PPC64_PLT16_HA: u32 = 31u32;
```

### `R_PPC64_SECTOFF`

```rust
const R_PPC64_SECTOFF: u32 = 33u32;
```

### `R_PPC64_SECTOFF_LO`

```rust
const R_PPC64_SECTOFF_LO: u32 = 34u32;
```

### `R_PPC64_SECTOFF_HI`

```rust
const R_PPC64_SECTOFF_HI: u32 = 35u32;
```

### `R_PPC64_SECTOFF_HA`

```rust
const R_PPC64_SECTOFF_HA: u32 = 36u32;
```

### `R_PPC64_ADDR30`

```rust
const R_PPC64_ADDR30: u32 = 37u32;
```

word30 (S + A - P) >> 2

### `R_PPC64_ADDR64`

```rust
const R_PPC64_ADDR64: u32 = 38u32;
```

doubleword64 S + A

### `R_PPC64_ADDR16_HIGHER`

```rust
const R_PPC64_ADDR16_HIGHER: u32 = 39u32;
```

half16 #higher(S + A)

### `R_PPC64_ADDR16_HIGHERA`

```rust
const R_PPC64_ADDR16_HIGHERA: u32 = 40u32;
```

half16 #highera(S + A)

### `R_PPC64_ADDR16_HIGHEST`

```rust
const R_PPC64_ADDR16_HIGHEST: u32 = 41u32;
```

half16 #highest(S + A)

### `R_PPC64_ADDR16_HIGHESTA`

```rust
const R_PPC64_ADDR16_HIGHESTA: u32 = 42u32;
```

half16 #highesta(S + A)

### `R_PPC64_UADDR64`

```rust
const R_PPC64_UADDR64: u32 = 43u32;
```

doubleword64 S + A

### `R_PPC64_REL64`

```rust
const R_PPC64_REL64: u32 = 44u32;
```

doubleword64 S + A - P

### `R_PPC64_PLT64`

```rust
const R_PPC64_PLT64: u32 = 45u32;
```

doubleword64 L + A

### `R_PPC64_PLTREL64`

```rust
const R_PPC64_PLTREL64: u32 = 46u32;
```

doubleword64 L + A - P

### `R_PPC64_TOC16`

```rust
const R_PPC64_TOC16: u32 = 47u32;
```

half16* S + A - .TOC

### `R_PPC64_TOC16_LO`

```rust
const R_PPC64_TOC16_LO: u32 = 48u32;
```

half16 #lo(S + A - .TOC.)

### `R_PPC64_TOC16_HI`

```rust
const R_PPC64_TOC16_HI: u32 = 49u32;
```

half16 #hi(S + A - .TOC.)

### `R_PPC64_TOC16_HA`

```rust
const R_PPC64_TOC16_HA: u32 = 50u32;
```

half16 #ha(S + A - .TOC.)

### `R_PPC64_TOC`

```rust
const R_PPC64_TOC: u32 = 51u32;
```

doubleword64 .TOC

### `R_PPC64_PLTGOT16`

```rust
const R_PPC64_PLTGOT16: u32 = 52u32;
```

half16* M + A

### `R_PPC64_PLTGOT16_LO`

```rust
const R_PPC64_PLTGOT16_LO: u32 = 53u32;
```

half16 #lo(M + A)

### `R_PPC64_PLTGOT16_HI`

```rust
const R_PPC64_PLTGOT16_HI: u32 = 54u32;
```

half16 #hi(M + A)

### `R_PPC64_PLTGOT16_HA`

```rust
const R_PPC64_PLTGOT16_HA: u32 = 55u32;
```

half16 #ha(M + A)

### `R_PPC64_ADDR16_DS`

```rust
const R_PPC64_ADDR16_DS: u32 = 56u32;
```

half16ds* (S + A) >> 2

### `R_PPC64_ADDR16_LO_DS`

```rust
const R_PPC64_ADDR16_LO_DS: u32 = 57u32;
```

half16ds  #lo(S + A) >> 2

### `R_PPC64_GOT16_DS`

```rust
const R_PPC64_GOT16_DS: u32 = 58u32;
```

half16ds* (G + A) >> 2

### `R_PPC64_GOT16_LO_DS`

```rust
const R_PPC64_GOT16_LO_DS: u32 = 59u32;
```

half16ds  #lo(G + A) >> 2

### `R_PPC64_PLT16_LO_DS`

```rust
const R_PPC64_PLT16_LO_DS: u32 = 60u32;
```

half16ds  #lo(L + A) >> 2

### `R_PPC64_SECTOFF_DS`

```rust
const R_PPC64_SECTOFF_DS: u32 = 61u32;
```

half16ds* (R + A) >> 2

### `R_PPC64_SECTOFF_LO_DS`

```rust
const R_PPC64_SECTOFF_LO_DS: u32 = 62u32;
```

half16ds  #lo(R + A) >> 2

### `R_PPC64_TOC16_DS`

```rust
const R_PPC64_TOC16_DS: u32 = 63u32;
```

half16ds* (S + A - .TOC.) >> 2

### `R_PPC64_TOC16_LO_DS`

```rust
const R_PPC64_TOC16_LO_DS: u32 = 64u32;
```

half16ds  #lo(S + A - .TOC.) >> 2

### `R_PPC64_PLTGOT16_DS`

```rust
const R_PPC64_PLTGOT16_DS: u32 = 65u32;
```

half16ds* (M + A) >> 2

### `R_PPC64_PLTGOT16_LO_DS`

```rust
const R_PPC64_PLTGOT16_LO_DS: u32 = 66u32;
```

half16ds  #lo(M + A) >> 2

### `R_PPC64_TLS`

```rust
const R_PPC64_TLS: u32 = 67u32;
```

none    (sym+add)@tls

### `R_PPC64_DTPMOD64`

```rust
const R_PPC64_DTPMOD64: u32 = 68u32;
```

doubleword64 (sym+add)@dtpmod

### `R_PPC64_TPREL16`

```rust
const R_PPC64_TPREL16: u32 = 69u32;
```

half16* (sym+add)@tprel

### `R_PPC64_TPREL16_LO`

```rust
const R_PPC64_TPREL16_LO: u32 = 70u32;
```

half16  (sym+add)@tprel@l

### `R_PPC64_TPREL16_HI`

```rust
const R_PPC64_TPREL16_HI: u32 = 71u32;
```

half16  (sym+add)@tprel@h

### `R_PPC64_TPREL16_HA`

```rust
const R_PPC64_TPREL16_HA: u32 = 72u32;
```

half16  (sym+add)@tprel@ha

### `R_PPC64_TPREL64`

```rust
const R_PPC64_TPREL64: u32 = 73u32;
```

doubleword64 (sym+add)@tprel

### `R_PPC64_DTPREL16`

```rust
const R_PPC64_DTPREL16: u32 = 74u32;
```

half16* (sym+add)@dtprel

### `R_PPC64_DTPREL16_LO`

```rust
const R_PPC64_DTPREL16_LO: u32 = 75u32;
```

half16  (sym+add)@dtprel@l

### `R_PPC64_DTPREL16_HI`

```rust
const R_PPC64_DTPREL16_HI: u32 = 76u32;
```

half16  (sym+add)@dtprel@h

### `R_PPC64_DTPREL16_HA`

```rust
const R_PPC64_DTPREL16_HA: u32 = 77u32;
```

half16  (sym+add)@dtprel@ha

### `R_PPC64_DTPREL64`

```rust
const R_PPC64_DTPREL64: u32 = 78u32;
```

doubleword64 (sym+add)@dtprel

### `R_PPC64_GOT_TLSGD16`

```rust
const R_PPC64_GOT_TLSGD16: u32 = 79u32;
```

half16* (sym+add)@got@tlsgd

### `R_PPC64_GOT_TLSGD16_LO`

```rust
const R_PPC64_GOT_TLSGD16_LO: u32 = 80u32;
```

half16  (sym+add)@got@tlsgd@l

### `R_PPC64_GOT_TLSGD16_HI`

```rust
const R_PPC64_GOT_TLSGD16_HI: u32 = 81u32;
```

half16  (sym+add)@got@tlsgd@h

### `R_PPC64_GOT_TLSGD16_HA`

```rust
const R_PPC64_GOT_TLSGD16_HA: u32 = 82u32;
```

half16  (sym+add)@got@tlsgd@ha

### `R_PPC64_GOT_TLSLD16`

```rust
const R_PPC64_GOT_TLSLD16: u32 = 83u32;
```

half16* (sym+add)@got@tlsld

### `R_PPC64_GOT_TLSLD16_LO`

```rust
const R_PPC64_GOT_TLSLD16_LO: u32 = 84u32;
```

half16  (sym+add)@got@tlsld@l

### `R_PPC64_GOT_TLSLD16_HI`

```rust
const R_PPC64_GOT_TLSLD16_HI: u32 = 85u32;
```

half16  (sym+add)@got@tlsld@h

### `R_PPC64_GOT_TLSLD16_HA`

```rust
const R_PPC64_GOT_TLSLD16_HA: u32 = 86u32;
```

half16  (sym+add)@got@tlsld@ha

### `R_PPC64_GOT_TPREL16_DS`

```rust
const R_PPC64_GOT_TPREL16_DS: u32 = 87u32;
```

half16ds* (sym+add)@got@tprel

### `R_PPC64_GOT_TPREL16_LO_DS`

```rust
const R_PPC64_GOT_TPREL16_LO_DS: u32 = 88u32;
```

half16ds (sym+add)@got@tprel@l

### `R_PPC64_GOT_TPREL16_HI`

```rust
const R_PPC64_GOT_TPREL16_HI: u32 = 89u32;
```

half16  (sym+add)@got@tprel@h

### `R_PPC64_GOT_TPREL16_HA`

```rust
const R_PPC64_GOT_TPREL16_HA: u32 = 90u32;
```

half16  (sym+add)@got@tprel@ha

### `R_PPC64_GOT_DTPREL16_DS`

```rust
const R_PPC64_GOT_DTPREL16_DS: u32 = 91u32;
```

half16ds* (sym+add)@got@dtprel

### `R_PPC64_GOT_DTPREL16_LO_DS`

```rust
const R_PPC64_GOT_DTPREL16_LO_DS: u32 = 92u32;
```

half16ds (sym+add)@got@dtprel@l

### `R_PPC64_GOT_DTPREL16_HI`

```rust
const R_PPC64_GOT_DTPREL16_HI: u32 = 93u32;
```

half16  (sym+add)@got@dtprel@h

### `R_PPC64_GOT_DTPREL16_HA`

```rust
const R_PPC64_GOT_DTPREL16_HA: u32 = 94u32;
```

half16  (sym+add)@got@dtprel@ha

### `R_PPC64_TPREL16_DS`

```rust
const R_PPC64_TPREL16_DS: u32 = 95u32;
```

half16ds* (sym+add)@tprel

### `R_PPC64_TPREL16_LO_DS`

```rust
const R_PPC64_TPREL16_LO_DS: u32 = 96u32;
```

half16ds (sym+add)@tprel@l

### `R_PPC64_TPREL16_HIGHER`

```rust
const R_PPC64_TPREL16_HIGHER: u32 = 97u32;
```

half16  (sym+add)@tprel@higher

### `R_PPC64_TPREL16_HIGHERA`

```rust
const R_PPC64_TPREL16_HIGHERA: u32 = 98u32;
```

half16  (sym+add)@tprel@highera

### `R_PPC64_TPREL16_HIGHEST`

```rust
const R_PPC64_TPREL16_HIGHEST: u32 = 99u32;
```

half16  (sym+add)@tprel@highest

### `R_PPC64_TPREL16_HIGHESTA`

```rust
const R_PPC64_TPREL16_HIGHESTA: u32 = 100u32;
```

half16  (sym+add)@tprel@highesta

### `R_PPC64_DTPREL16_DS`

```rust
const R_PPC64_DTPREL16_DS: u32 = 101u32;
```

half16ds* (sym+add)@dtprel

### `R_PPC64_DTPREL16_LO_DS`

```rust
const R_PPC64_DTPREL16_LO_DS: u32 = 102u32;
```

half16ds (sym+add)@dtprel@l

### `R_PPC64_DTPREL16_HIGHER`

```rust
const R_PPC64_DTPREL16_HIGHER: u32 = 103u32;
```

half16  (sym+add)@dtprel@higher

### `R_PPC64_DTPREL16_HIGHERA`

```rust
const R_PPC64_DTPREL16_HIGHERA: u32 = 104u32;
```

half16  (sym+add)@dtprel@highera

### `R_PPC64_DTPREL16_HIGHEST`

```rust
const R_PPC64_DTPREL16_HIGHEST: u32 = 105u32;
```

half16  (sym+add)@dtprel@highest

### `R_PPC64_DTPREL16_HIGHESTA`

```rust
const R_PPC64_DTPREL16_HIGHESTA: u32 = 106u32;
```

half16  (sym+add)@dtprel@highesta

### `R_PPC64_TLSGD`

```rust
const R_PPC64_TLSGD: u32 = 107u32;
```

none    (sym+add)@tlsgd

### `R_PPC64_TLSLD`

```rust
const R_PPC64_TLSLD: u32 = 108u32;
```

none    (sym+add)@tlsld

### `R_PPC64_TOCSAVE`

```rust
const R_PPC64_TOCSAVE: u32 = 109u32;
```

none

### `R_PPC64_ADDR16_HIGH`

```rust
const R_PPC64_ADDR16_HIGH: u32 = 110u32;
```

### `R_PPC64_ADDR16_HIGHA`

```rust
const R_PPC64_ADDR16_HIGHA: u32 = 111u32;
```

### `R_PPC64_TPREL16_HIGH`

```rust
const R_PPC64_TPREL16_HIGH: u32 = 112u32;
```

### `R_PPC64_TPREL16_HIGHA`

```rust
const R_PPC64_TPREL16_HIGHA: u32 = 113u32;
```

### `R_PPC64_DTPREL16_HIGH`

```rust
const R_PPC64_DTPREL16_HIGH: u32 = 114u32;
```

### `R_PPC64_DTPREL16_HIGHA`

```rust
const R_PPC64_DTPREL16_HIGHA: u32 = 115u32;
```

### `R_PPC64_JMP_IREL`

```rust
const R_PPC64_JMP_IREL: u32 = 247u32;
```

GNU extension to support local ifunc.

### `R_PPC64_IRELATIVE`

```rust
const R_PPC64_IRELATIVE: u32 = 248u32;
```

GNU extension to support local ifunc.

### `R_PPC64_REL16`

```rust
const R_PPC64_REL16: u32 = 249u32;
```

half16   (sym+add-.)

### `R_PPC64_REL16_LO`

```rust
const R_PPC64_REL16_LO: u32 = 250u32;
```

half16   (sym+add-.)@l

### `R_PPC64_REL16_HI`

```rust
const R_PPC64_REL16_HI: u32 = 251u32;
```

half16   (sym+add-.)@h

### `R_PPC64_REL16_HA`

```rust
const R_PPC64_REL16_HA: u32 = 252u32;
```

half16   (sym+add-.)@ha

### `EF_PPC64_ABI`

```rust
const EF_PPC64_ABI: u32 = 3u32;
```

PowerPC64 bits specifying ABI.

1 for original function descriptor using ABI,
2 for revised ABI without function descriptors,
0 for unspecified or not using any features affected by the differences.

### `DT_PPC64_GLINK`

```rust
const DT_PPC64_GLINK: u32 = 1_879_048_192u32;
```

### `DT_PPC64_OPD`

```rust
const DT_PPC64_OPD: u32 = 1_879_048_193u32;
```

### `DT_PPC64_OPDSZ`

```rust
const DT_PPC64_OPDSZ: u32 = 1_879_048_194u32;
```

### `DT_PPC64_OPT`

```rust
const DT_PPC64_OPT: u32 = 1_879_048_195u32;
```

### `PPC64_OPT_TLS`

```rust
const PPC64_OPT_TLS: u32 = 1u32;
```

### `PPC64_OPT_MULTI_TOC`

```rust
const PPC64_OPT_MULTI_TOC: u32 = 2u32;
```

### `PPC64_OPT_LOCALENTRY`

```rust
const PPC64_OPT_LOCALENTRY: u32 = 4u32;
```

### `STO_PPC64_LOCAL_BIT`

```rust
const STO_PPC64_LOCAL_BIT: u8 = 5u8;
```

### `STO_PPC64_LOCAL_MASK`

```rust
const STO_PPC64_LOCAL_MASK: u8 = 224u8;
```

### `EF_ARM_RELEXEC`

```rust
const EF_ARM_RELEXEC: u32 = 1u32;
```

### `EF_ARM_HASENTRY`

```rust
const EF_ARM_HASENTRY: u32 = 2u32;
```

### `EF_ARM_INTERWORK`

```rust
const EF_ARM_INTERWORK: u32 = 4u32;
```

### `EF_ARM_APCS_26`

```rust
const EF_ARM_APCS_26: u32 = 8u32;
```

### `EF_ARM_APCS_FLOAT`

```rust
const EF_ARM_APCS_FLOAT: u32 = 16u32;
```

### `EF_ARM_PIC`

```rust
const EF_ARM_PIC: u32 = 32u32;
```

### `EF_ARM_ALIGN8`

```rust
const EF_ARM_ALIGN8: u32 = 64u32;
```

8-bit structure alignment is in use

### `EF_ARM_NEW_ABI`

```rust
const EF_ARM_NEW_ABI: u32 = 128u32;
```

### `EF_ARM_OLD_ABI`

```rust
const EF_ARM_OLD_ABI: u32 = 256u32;
```

### `EF_ARM_SOFT_FLOAT`

```rust
const EF_ARM_SOFT_FLOAT: u32 = 512u32;
```

### `EF_ARM_VFP_FLOAT`

```rust
const EF_ARM_VFP_FLOAT: u32 = 1_024u32;
```

### `EF_ARM_MAVERICK_FLOAT`

```rust
const EF_ARM_MAVERICK_FLOAT: u32 = 2_048u32;
```

### `EF_ARM_ABI_FLOAT_SOFT`

```rust
const EF_ARM_ABI_FLOAT_SOFT: u32 = 512u32;
```

NB conflicts with EF_ARM_SOFT_FLOAT

### `EF_ARM_ABI_FLOAT_HARD`

```rust
const EF_ARM_ABI_FLOAT_HARD: u32 = 1_024u32;
```

NB conflicts with EF_ARM_VFP_FLOAT

### `EF_ARM_SYMSARESORTED`

```rust
const EF_ARM_SYMSARESORTED: u32 = 4u32;
```

### `EF_ARM_DYNSYMSUSESEGIDX`

```rust
const EF_ARM_DYNSYMSUSESEGIDX: u32 = 8u32;
```

### `EF_ARM_MAPSYMSFIRST`

```rust
const EF_ARM_MAPSYMSFIRST: u32 = 16u32;
```

### `EF_ARM_BE8`

```rust
const EF_ARM_BE8: u32 = 8_388_608u32;
```

### `EF_ARM_LE8`

```rust
const EF_ARM_LE8: u32 = 4_194_304u32;
```

### `EF_ARM_EABIMASK`

```rust
const EF_ARM_EABIMASK: u32 = 4_278_190_080u32;
```

### `EF_ARM_EABI_UNKNOWN`

```rust
const EF_ARM_EABI_UNKNOWN: u32 = 0u32;
```

### `EF_ARM_EABI_VER1`

```rust
const EF_ARM_EABI_VER1: u32 = 16_777_216u32;
```

### `EF_ARM_EABI_VER2`

```rust
const EF_ARM_EABI_VER2: u32 = 33_554_432u32;
```

### `EF_ARM_EABI_VER3`

```rust
const EF_ARM_EABI_VER3: u32 = 50_331_648u32;
```

### `EF_ARM_EABI_VER4`

```rust
const EF_ARM_EABI_VER4: u32 = 67_108_864u32;
```

### `EF_ARM_EABI_VER5`

```rust
const EF_ARM_EABI_VER5: u32 = 83_886_080u32;
```

### `STT_ARM_TFUNC`

```rust
const STT_ARM_TFUNC: u8 = 13u8;
```

A Thumb function.

### `STT_ARM_16BIT`

```rust
const STT_ARM_16BIT: u8 = 15u8;
```

A Thumb label.

### `SHF_ARM_ENTRYSECT`

```rust
const SHF_ARM_ENTRYSECT: u32 = 268_435_456u32;
```

Section contains an entry point

### `SHF_ARM_COMDEF`

```rust
const SHF_ARM_COMDEF: u32 = 2_147_483_648u32;
```

Section may be multiply defined in the input to a link step.

### `PF_ARM_SB`

```rust
const PF_ARM_SB: u32 = 268_435_456u32;
```

Segment contains the location addressed by the static base.

### `PF_ARM_PI`

```rust
const PF_ARM_PI: u32 = 536_870_912u32;
```

Position-independent segment.

### `PF_ARM_ABS`

```rust
const PF_ARM_ABS: u32 = 1_073_741_824u32;
```

Absolute segment.

### `PT_ARM_EXIDX`

```rust
const PT_ARM_EXIDX: u32 = 1_879_048_193u32;
```

ARM unwind segment.

### `SHT_ARM_EXIDX`

```rust
const SHT_ARM_EXIDX: u32 = 1_879_048_193u32;
```

ARM unwind section.

### `SHT_ARM_PREEMPTMAP`

```rust
const SHT_ARM_PREEMPTMAP: u32 = 1_879_048_194u32;
```

Preemption details.

### `SHT_ARM_ATTRIBUTES`

```rust
const SHT_ARM_ATTRIBUTES: u32 = 1_879_048_195u32;
```

ARM attributes section.

### `SHT_AARCH64_ATTRIBUTES`

```rust
const SHT_AARCH64_ATTRIBUTES: u32 = 1_879_048_195u32;
```

AArch64 attributes section.

### `R_AARCH64_NONE`

```rust
const R_AARCH64_NONE: u32 = 0u32;
```

No relocation.

### `R_AARCH64_P32_ABS32`

```rust
const R_AARCH64_P32_ABS32: u32 = 1u32;
```

Direct 32 bit.

### `R_AARCH64_P32_COPY`

```rust
const R_AARCH64_P32_COPY: u32 = 180u32;
```

Copy symbol at runtime.

### `R_AARCH64_P32_GLOB_DAT`

```rust
const R_AARCH64_P32_GLOB_DAT: u32 = 181u32;
```

Create GOT entry.

### `R_AARCH64_P32_JUMP_SLOT`

```rust
const R_AARCH64_P32_JUMP_SLOT: u32 = 182u32;
```

Create PLT entry.

### `R_AARCH64_P32_RELATIVE`

```rust
const R_AARCH64_P32_RELATIVE: u32 = 183u32;
```

Adjust by program base.

### `R_AARCH64_P32_TLS_DTPMOD`

```rust
const R_AARCH64_P32_TLS_DTPMOD: u32 = 184u32;
```

Module number, 32 bit.

### `R_AARCH64_P32_TLS_DTPREL`

```rust
const R_AARCH64_P32_TLS_DTPREL: u32 = 185u32;
```

Module-relative offset, 32 bit.

### `R_AARCH64_P32_TLS_TPREL`

```rust
const R_AARCH64_P32_TLS_TPREL: u32 = 186u32;
```

TP-relative offset, 32 bit.

### `R_AARCH64_P32_TLSDESC`

```rust
const R_AARCH64_P32_TLSDESC: u32 = 187u32;
```

TLS Descriptor.

### `R_AARCH64_P32_IRELATIVE`

```rust
const R_AARCH64_P32_IRELATIVE: u32 = 188u32;
```

STT_GNU_IFUNC relocation.

### `R_AARCH64_ABS64`

```rust
const R_AARCH64_ABS64: u32 = 257u32;
```

Direct 64 bit.

### `R_AARCH64_ABS32`

```rust
const R_AARCH64_ABS32: u32 = 258u32;
```

Direct 32 bit.

### `R_AARCH64_ABS16`

```rust
const R_AARCH64_ABS16: u32 = 259u32;
```

Direct 16-bit.

### `R_AARCH64_PREL64`

```rust
const R_AARCH64_PREL64: u32 = 260u32;
```

PC-relative 64-bit.

### `R_AARCH64_PREL32`

```rust
const R_AARCH64_PREL32: u32 = 261u32;
```

PC-relative 32-bit.

### `R_AARCH64_PREL16`

```rust
const R_AARCH64_PREL16: u32 = 262u32;
```

PC-relative 16-bit.

### `R_AARCH64_MOVW_UABS_G0`

```rust
const R_AARCH64_MOVW_UABS_G0: u32 = 263u32;
```

Dir. MOVZ imm. from bits 15:0.

### `R_AARCH64_MOVW_UABS_G0_NC`

```rust
const R_AARCH64_MOVW_UABS_G0_NC: u32 = 264u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_UABS_G1`

```rust
const R_AARCH64_MOVW_UABS_G1: u32 = 265u32;
```

Dir. MOVZ imm. from bits 31:16.

### `R_AARCH64_MOVW_UABS_G1_NC`

```rust
const R_AARCH64_MOVW_UABS_G1_NC: u32 = 266u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_UABS_G2`

```rust
const R_AARCH64_MOVW_UABS_G2: u32 = 267u32;
```

Dir. MOVZ imm. from bits 47:32.

### `R_AARCH64_MOVW_UABS_G2_NC`

```rust
const R_AARCH64_MOVW_UABS_G2_NC: u32 = 268u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_UABS_G3`

```rust
const R_AARCH64_MOVW_UABS_G3: u32 = 269u32;
```

Dir. MOV{K,Z} imm. from 63:48.

### `R_AARCH64_MOVW_SABS_G0`

```rust
const R_AARCH64_MOVW_SABS_G0: u32 = 270u32;
```

Dir. MOV{N,Z} imm. from 15:0.

### `R_AARCH64_MOVW_SABS_G1`

```rust
const R_AARCH64_MOVW_SABS_G1: u32 = 271u32;
```

Dir. MOV{N,Z} imm. from 31:16.

### `R_AARCH64_MOVW_SABS_G2`

```rust
const R_AARCH64_MOVW_SABS_G2: u32 = 272u32;
```

Dir. MOV{N,Z} imm. from 47:32.

### `R_AARCH64_LD_PREL_LO19`

```rust
const R_AARCH64_LD_PREL_LO19: u32 = 273u32;
```

PC-rel. LD imm. from bits 20:2.

### `R_AARCH64_ADR_PREL_LO21`

```rust
const R_AARCH64_ADR_PREL_LO21: u32 = 274u32;
```

PC-rel. ADR imm. from bits 20:0.

### `R_AARCH64_ADR_PREL_PG_HI21`

```rust
const R_AARCH64_ADR_PREL_PG_HI21: u32 = 275u32;
```

Page-rel. ADRP imm. from 32:12.

### `R_AARCH64_ADR_PREL_PG_HI21_NC`

```rust
const R_AARCH64_ADR_PREL_PG_HI21_NC: u32 = 276u32;
```

Likewise; no overflow check.

### `R_AARCH64_ADD_ABS_LO12_NC`

```rust
const R_AARCH64_ADD_ABS_LO12_NC: u32 = 277u32;
```

Dir. ADD imm. from bits 11:0.

### `R_AARCH64_LDST8_ABS_LO12_NC`

```rust
const R_AARCH64_LDST8_ABS_LO12_NC: u32 = 278u32;
```

Likewise for LD/ST; no check.

### `R_AARCH64_TSTBR14`

```rust
const R_AARCH64_TSTBR14: u32 = 279u32;
```

PC-rel. TBZ/TBNZ imm. from 15:2.

### `R_AARCH64_CONDBR19`

```rust
const R_AARCH64_CONDBR19: u32 = 280u32;
```

PC-rel. cond. br. imm. from 20:2.

### `R_AARCH64_JUMP26`

```rust
const R_AARCH64_JUMP26: u32 = 282u32;
```

PC-rel. B imm. from bits 27:2.

### `R_AARCH64_CALL26`

```rust
const R_AARCH64_CALL26: u32 = 283u32;
```

Likewise for CALL.

### `R_AARCH64_LDST16_ABS_LO12_NC`

```rust
const R_AARCH64_LDST16_ABS_LO12_NC: u32 = 284u32;
```

Dir. ADD imm. from bits 11:1.

### `R_AARCH64_LDST32_ABS_LO12_NC`

```rust
const R_AARCH64_LDST32_ABS_LO12_NC: u32 = 285u32;
```

Likewise for bits 11:2.

### `R_AARCH64_LDST64_ABS_LO12_NC`

```rust
const R_AARCH64_LDST64_ABS_LO12_NC: u32 = 286u32;
```

Likewise for bits 11:3.

### `R_AARCH64_MOVW_PREL_G0`

```rust
const R_AARCH64_MOVW_PREL_G0: u32 = 287u32;
```

PC-rel. MOV{N,Z} imm. from 15:0.

### `R_AARCH64_MOVW_PREL_G0_NC`

```rust
const R_AARCH64_MOVW_PREL_G0_NC: u32 = 288u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_PREL_G1`

```rust
const R_AARCH64_MOVW_PREL_G1: u32 = 289u32;
```

PC-rel. MOV{N,Z} imm. from 31:16.

### `R_AARCH64_MOVW_PREL_G1_NC`

```rust
const R_AARCH64_MOVW_PREL_G1_NC: u32 = 290u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_PREL_G2`

```rust
const R_AARCH64_MOVW_PREL_G2: u32 = 291u32;
```

PC-rel. MOV{N,Z} imm. from 47:32.

### `R_AARCH64_MOVW_PREL_G2_NC`

```rust
const R_AARCH64_MOVW_PREL_G2_NC: u32 = 292u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_PREL_G3`

```rust
const R_AARCH64_MOVW_PREL_G3: u32 = 293u32;
```

PC-rel. MOV{N,Z} imm. from 63:48.

### `R_AARCH64_LDST128_ABS_LO12_NC`

```rust
const R_AARCH64_LDST128_ABS_LO12_NC: u32 = 299u32;
```

Dir. ADD imm. from bits 11:4.

### `R_AARCH64_MOVW_GOTOFF_G0`

```rust
const R_AARCH64_MOVW_GOTOFF_G0: u32 = 300u32;
```

GOT-rel. off. MOV{N,Z} imm. 15:0.

### `R_AARCH64_MOVW_GOTOFF_G0_NC`

```rust
const R_AARCH64_MOVW_GOTOFF_G0_NC: u32 = 301u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_GOTOFF_G1`

```rust
const R_AARCH64_MOVW_GOTOFF_G1: u32 = 302u32;
```

GOT-rel. o. MOV{N,Z} imm. 31:16.

### `R_AARCH64_MOVW_GOTOFF_G1_NC`

```rust
const R_AARCH64_MOVW_GOTOFF_G1_NC: u32 = 303u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_GOTOFF_G2`

```rust
const R_AARCH64_MOVW_GOTOFF_G2: u32 = 304u32;
```

GOT-rel. o. MOV{N,Z} imm. 47:32.

### `R_AARCH64_MOVW_GOTOFF_G2_NC`

```rust
const R_AARCH64_MOVW_GOTOFF_G2_NC: u32 = 305u32;
```

Likewise for MOVK; no check.

### `R_AARCH64_MOVW_GOTOFF_G3`

```rust
const R_AARCH64_MOVW_GOTOFF_G3: u32 = 306u32;
```

GOT-rel. o. MOV{N,Z} imm. 63:48.

### `R_AARCH64_GOTREL64`

```rust
const R_AARCH64_GOTREL64: u32 = 307u32;
```

GOT-relative 64-bit.

### `R_AARCH64_GOTREL32`

```rust
const R_AARCH64_GOTREL32: u32 = 308u32;
```

GOT-relative 32-bit.

### `R_AARCH64_GOT_LD_PREL19`

```rust
const R_AARCH64_GOT_LD_PREL19: u32 = 309u32;
```

PC-rel. GOT off. load imm. 20:2.

### `R_AARCH64_LD64_GOTOFF_LO15`

```rust
const R_AARCH64_LD64_GOTOFF_LO15: u32 = 310u32;
```

GOT-rel. off. LD/ST imm. 14:3.

### `R_AARCH64_ADR_GOT_PAGE`

```rust
const R_AARCH64_ADR_GOT_PAGE: u32 = 311u32;
```

P-page-rel. GOT off. ADRP 32:12.

### `R_AARCH64_LD64_GOT_LO12_NC`

```rust
const R_AARCH64_LD64_GOT_LO12_NC: u32 = 312u32;
```

Dir. GOT off. LD/ST imm. 11:3.

### `R_AARCH64_LD64_GOTPAGE_LO15`

```rust
const R_AARCH64_LD64_GOTPAGE_LO15: u32 = 313u32;
```

GOT-page-rel. GOT off. LD/ST 14:3

### `R_AARCH64_TLSGD_ADR_PREL21`

```rust
const R_AARCH64_TLSGD_ADR_PREL21: u32 = 512u32;
```

PC-relative ADR imm. 20:0.

### `R_AARCH64_TLSGD_ADR_PAGE21`

```rust
const R_AARCH64_TLSGD_ADR_PAGE21: u32 = 513u32;
```

page-rel. ADRP imm. 32:12.

### `R_AARCH64_TLSGD_ADD_LO12_NC`

```rust
const R_AARCH64_TLSGD_ADD_LO12_NC: u32 = 514u32;
```

direct ADD imm. from 11:0.

### `R_AARCH64_TLSGD_MOVW_G1`

```rust
const R_AARCH64_TLSGD_MOVW_G1: u32 = 515u32;
```

GOT-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSGD_MOVW_G0_NC`

```rust
const R_AARCH64_TLSGD_MOVW_G0_NC: u32 = 516u32;
```

GOT-rel. MOVK imm. 15:0.

### `R_AARCH64_TLSLD_ADR_PREL21`

```rust
const R_AARCH64_TLSLD_ADR_PREL21: u32 = 517u32;
```

Like 512; local dynamic model.

### `R_AARCH64_TLSLD_ADR_PAGE21`

```rust
const R_AARCH64_TLSLD_ADR_PAGE21: u32 = 518u32;
```

Like 513; local dynamic model.

### `R_AARCH64_TLSLD_ADD_LO12_NC`

```rust
const R_AARCH64_TLSLD_ADD_LO12_NC: u32 = 519u32;
```

Like 514; local dynamic model.

### `R_AARCH64_TLSLD_MOVW_G1`

```rust
const R_AARCH64_TLSLD_MOVW_G1: u32 = 520u32;
```

Like 515; local dynamic model.

### `R_AARCH64_TLSLD_MOVW_G0_NC`

```rust
const R_AARCH64_TLSLD_MOVW_G0_NC: u32 = 521u32;
```

Like 516; local dynamic model.

### `R_AARCH64_TLSLD_LD_PREL19`

```rust
const R_AARCH64_TLSLD_LD_PREL19: u32 = 522u32;
```

TLS PC-rel. load imm. 20:2.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G2`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G2: u32 = 523u32;
```

TLS DTP-rel. MOV{N,Z} 47:32.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G1`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G1: u32 = 524u32;
```

TLS DTP-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC: u32 = 525u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G0`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G0: u32 = 526u32;
```

TLS DTP-rel. MOV{N,Z} 15:0.

### `R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC`

```rust
const R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC: u32 = 527u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLD_ADD_DTPREL_HI12`

```rust
const R_AARCH64_TLSLD_ADD_DTPREL_HI12: u32 = 528u32;
```

DTP-rel. ADD imm. from 23:12.

### `R_AARCH64_TLSLD_ADD_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_ADD_DTPREL_LO12: u32 = 529u32;
```

DTP-rel. ADD imm. from 11:0.

### `R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC: u32 = 530u32;
```

Likewise; no ovfl. check.

### `R_AARCH64_TLSLD_LDST8_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST8_DTPREL_LO12: u32 = 531u32;
```

DTP-rel. LD/ST imm. 11:0.

### `R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC: u32 = 532u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST16_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST16_DTPREL_LO12: u32 = 533u32;
```

DTP-rel. LD/ST imm. 11:1.

### `R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC: u32 = 534u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST32_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST32_DTPREL_LO12: u32 = 535u32;
```

DTP-rel. LD/ST imm. 11:2.

### `R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC: u32 = 536u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST64_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST64_DTPREL_LO12: u32 = 537u32;
```

DTP-rel. LD/ST imm. 11:3.

### `R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC: u32 = 538u32;
```

Likewise; no check.

### `R_AARCH64_TLSIE_MOVW_GOTTPREL_G1`

```rust
const R_AARCH64_TLSIE_MOVW_GOTTPREL_G1: u32 = 539u32;
```

GOT-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC`

```rust
const R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC: u32 = 540u32;
```

GOT-rel. MOVK 15:0.

### `R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21`

```rust
const R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21: u32 = 541u32;
```

Page-rel. ADRP 32:12.

### `R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC`

```rust
const R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC: u32 = 542u32;
```

Direct LD off. 11:3.

### `R_AARCH64_TLSIE_LD_GOTTPREL_PREL19`

```rust
const R_AARCH64_TLSIE_LD_GOTTPREL_PREL19: u32 = 543u32;
```

PC-rel. load imm. 20:2.

### `R_AARCH64_TLSLE_MOVW_TPREL_G2`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G2: u32 = 544u32;
```

TLS TP-rel. MOV{N,Z} 47:32.

### `R_AARCH64_TLSLE_MOVW_TPREL_G1`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G1: u32 = 545u32;
```

TLS TP-rel. MOV{N,Z} 31:16.

### `R_AARCH64_TLSLE_MOVW_TPREL_G1_NC`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G1_NC: u32 = 546u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLE_MOVW_TPREL_G0`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G0: u32 = 547u32;
```

TLS TP-rel. MOV{N,Z} 15:0.

### `R_AARCH64_TLSLE_MOVW_TPREL_G0_NC`

```rust
const R_AARCH64_TLSLE_MOVW_TPREL_G0_NC: u32 = 548u32;
```

Likewise; MOVK; no check.

### `R_AARCH64_TLSLE_ADD_TPREL_HI12`

```rust
const R_AARCH64_TLSLE_ADD_TPREL_HI12: u32 = 549u32;
```

TP-rel. ADD imm. 23:12.

### `R_AARCH64_TLSLE_ADD_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_ADD_TPREL_LO12: u32 = 550u32;
```

TP-rel. ADD imm. 11:0.

### `R_AARCH64_TLSLE_ADD_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_ADD_TPREL_LO12_NC: u32 = 551u32;
```

Likewise; no ovfl. check.

### `R_AARCH64_TLSLE_LDST8_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST8_TPREL_LO12: u32 = 552u32;
```

TP-rel. LD/ST off. 11:0.

### `R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC: u32 = 553u32;
```

Likewise; no ovfl. check.

### `R_AARCH64_TLSLE_LDST16_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST16_TPREL_LO12: u32 = 554u32;
```

TP-rel. LD/ST off. 11:1.

### `R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC: u32 = 555u32;
```

Likewise; no check.

### `R_AARCH64_TLSLE_LDST32_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST32_TPREL_LO12: u32 = 556u32;
```

TP-rel. LD/ST off. 11:2.

### `R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC: u32 = 557u32;
```

Likewise; no check.

### `R_AARCH64_TLSLE_LDST64_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST64_TPREL_LO12: u32 = 558u32;
```

TP-rel. LD/ST off. 11:3.

### `R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC: u32 = 559u32;
```

Likewise; no check.

### `R_AARCH64_TLSDESC_LD_PREL19`

```rust
const R_AARCH64_TLSDESC_LD_PREL19: u32 = 560u32;
```

PC-rel. load immediate 20:2.

### `R_AARCH64_TLSDESC_ADR_PREL21`

```rust
const R_AARCH64_TLSDESC_ADR_PREL21: u32 = 561u32;
```

PC-rel. ADR immediate 20:0.

### `R_AARCH64_TLSDESC_ADR_PAGE21`

```rust
const R_AARCH64_TLSDESC_ADR_PAGE21: u32 = 562u32;
```

Page-rel. ADRP imm. 32:12.

### `R_AARCH64_TLSDESC_LD64_LO12`

```rust
const R_AARCH64_TLSDESC_LD64_LO12: u32 = 563u32;
```

Direct LD off. from 11:3.

### `R_AARCH64_TLSDESC_ADD_LO12`

```rust
const R_AARCH64_TLSDESC_ADD_LO12: u32 = 564u32;
```

Direct ADD imm. from 11:0.

### `R_AARCH64_TLSDESC_OFF_G1`

```rust
const R_AARCH64_TLSDESC_OFF_G1: u32 = 565u32;
```

GOT-rel. MOV{N,Z} imm. 31:16.

### `R_AARCH64_TLSDESC_OFF_G0_NC`

```rust
const R_AARCH64_TLSDESC_OFF_G0_NC: u32 = 566u32;
```

GOT-rel. MOVK imm. 15:0; no ck.

### `R_AARCH64_TLSDESC_LDR`

```rust
const R_AARCH64_TLSDESC_LDR: u32 = 567u32;
```

Relax LDR.

### `R_AARCH64_TLSDESC_ADD`

```rust
const R_AARCH64_TLSDESC_ADD: u32 = 568u32;
```

Relax ADD.

### `R_AARCH64_TLSDESC_CALL`

```rust
const R_AARCH64_TLSDESC_CALL: u32 = 569u32;
```

Relax BLR.

### `R_AARCH64_TLSLE_LDST128_TPREL_LO12`

```rust
const R_AARCH64_TLSLE_LDST128_TPREL_LO12: u32 = 570u32;
```

TP-rel. LD/ST off. 11:4.

### `R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC`

```rust
const R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC: u32 = 571u32;
```

Likewise; no check.

### `R_AARCH64_TLSLD_LDST128_DTPREL_LO12`

```rust
const R_AARCH64_TLSLD_LDST128_DTPREL_LO12: u32 = 572u32;
```

DTP-rel. LD/ST imm. 11:4.

### `R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC`

```rust
const R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC: u32 = 573u32;
```

Likewise; no check.

### `R_AARCH64_COPY`

```rust
const R_AARCH64_COPY: u32 = 1_024u32;
```

Copy symbol at runtime.

### `R_AARCH64_GLOB_DAT`

```rust
const R_AARCH64_GLOB_DAT: u32 = 1_025u32;
```

Create GOT entry.

### `R_AARCH64_JUMP_SLOT`

```rust
const R_AARCH64_JUMP_SLOT: u32 = 1_026u32;
```

Create PLT entry.

### `R_AARCH64_RELATIVE`

```rust
const R_AARCH64_RELATIVE: u32 = 1_027u32;
```

Adjust by program base.

### `R_AARCH64_TLS_DTPMOD`

```rust
const R_AARCH64_TLS_DTPMOD: u32 = 1_028u32;
```

Module number, 64 bit.

### `R_AARCH64_TLS_DTPREL`

```rust
const R_AARCH64_TLS_DTPREL: u32 = 1_029u32;
```

Module-relative offset, 64 bit.

### `R_AARCH64_TLS_TPREL`

```rust
const R_AARCH64_TLS_TPREL: u32 = 1_030u32;
```

TP-relative offset, 64 bit.

### `R_AARCH64_TLSDESC`

```rust
const R_AARCH64_TLSDESC: u32 = 1_031u32;
```

TLS Descriptor.

### `R_AARCH64_IRELATIVE`

```rust
const R_AARCH64_IRELATIVE: u32 = 1_032u32;
```

STT_GNU_IFUNC relocation.

### `EF_AVR_ARCH`

```rust
const EF_AVR_ARCH: u32 = 127u32;
```

Bitmask for `EF_AVR_ARCH_*`.

### `EF_AVR_LINKRELAX_PREPARED`

```rust
const EF_AVR_LINKRELAX_PREPARED: u32 = 128u32;
```

If set, it is assumed that the elf file uses local symbols as reference
for the relocations so that linker relaxation is possible.

### `EF_AVR_ARCH_AVR1`

```rust
const EF_AVR_ARCH_AVR1: u32 = 1u32;
```

### `EF_AVR_ARCH_AVR2`

```rust
const EF_AVR_ARCH_AVR2: u32 = 2u32;
```

### `EF_AVR_ARCH_AVR25`

```rust
const EF_AVR_ARCH_AVR25: u32 = 25u32;
```

### `EF_AVR_ARCH_AVR3`

```rust
const EF_AVR_ARCH_AVR3: u32 = 3u32;
```

### `EF_AVR_ARCH_AVR31`

```rust
const EF_AVR_ARCH_AVR31: u32 = 31u32;
```

### `EF_AVR_ARCH_AVR35`

```rust
const EF_AVR_ARCH_AVR35: u32 = 35u32;
```

### `EF_AVR_ARCH_AVR4`

```rust
const EF_AVR_ARCH_AVR4: u32 = 4u32;
```

### `EF_AVR_ARCH_AVR5`

```rust
const EF_AVR_ARCH_AVR5: u32 = 5u32;
```

### `EF_AVR_ARCH_AVR51`

```rust
const EF_AVR_ARCH_AVR51: u32 = 51u32;
```

### `EF_AVR_ARCH_AVR6`

```rust
const EF_AVR_ARCH_AVR6: u32 = 6u32;
```

### `EF_AVR_ARCH_AVRTINY`

```rust
const EF_AVR_ARCH_AVRTINY: u32 = 100u32;
```

### `EF_AVR_ARCH_XMEGA1`

```rust
const EF_AVR_ARCH_XMEGA1: u32 = 101u32;
```

### `EF_AVR_ARCH_XMEGA2`

```rust
const EF_AVR_ARCH_XMEGA2: u32 = 102u32;
```

### `EF_AVR_ARCH_XMEGA3`

```rust
const EF_AVR_ARCH_XMEGA3: u32 = 103u32;
```

### `EF_AVR_ARCH_XMEGA4`

```rust
const EF_AVR_ARCH_XMEGA4: u32 = 104u32;
```

### `EF_AVR_ARCH_XMEGA5`

```rust
const EF_AVR_ARCH_XMEGA5: u32 = 105u32;
```

### `EF_AVR_ARCH_XMEGA6`

```rust
const EF_AVR_ARCH_XMEGA6: u32 = 106u32;
```

### `EF_AVR_ARCH_XMEGA7`

```rust
const EF_AVR_ARCH_XMEGA7: u32 = 107u32;
```

### `R_AVR_NONE`

```rust
const R_AVR_NONE: u32 = 0u32;
```

### `R_AVR_32`

```rust
const R_AVR_32: u32 = 1u32;
```

Direct 32 bit

### `R_AVR_7_PCREL`

```rust
const R_AVR_7_PCREL: u32 = 2u32;
```

### `R_AVR_13_PCREL`

```rust
const R_AVR_13_PCREL: u32 = 3u32;
```

### `R_AVR_16`

```rust
const R_AVR_16: u32 = 4u32;
```

Direct 16 bit

### `R_AVR_16_PM`

```rust
const R_AVR_16_PM: u32 = 5u32;
```

### `R_AVR_LO8_LDI`

```rust
const R_AVR_LO8_LDI: u32 = 6u32;
```

### `R_AVR_HI8_LDI`

```rust
const R_AVR_HI8_LDI: u32 = 7u32;
```

### `R_AVR_HH8_LDI`

```rust
const R_AVR_HH8_LDI: u32 = 8u32;
```

### `R_AVR_LO8_LDI_NEG`

```rust
const R_AVR_LO8_LDI_NEG: u32 = 9u32;
```

### `R_AVR_HI8_LDI_NEG`

```rust
const R_AVR_HI8_LDI_NEG: u32 = 10u32;
```

### `R_AVR_HH8_LDI_NEG`

```rust
const R_AVR_HH8_LDI_NEG: u32 = 11u32;
```

### `R_AVR_LO8_LDI_PM`

```rust
const R_AVR_LO8_LDI_PM: u32 = 12u32;
```

### `R_AVR_HI8_LDI_PM`

```rust
const R_AVR_HI8_LDI_PM: u32 = 13u32;
```

### `R_AVR_HH8_LDI_PM`

```rust
const R_AVR_HH8_LDI_PM: u32 = 14u32;
```

### `R_AVR_LO8_LDI_PM_NEG`

```rust
const R_AVR_LO8_LDI_PM_NEG: u32 = 15u32;
```

### `R_AVR_HI8_LDI_PM_NEG`

```rust
const R_AVR_HI8_LDI_PM_NEG: u32 = 16u32;
```

### `R_AVR_HH8_LDI_PM_NEG`

```rust
const R_AVR_HH8_LDI_PM_NEG: u32 = 17u32;
```

### `R_AVR_CALL`

```rust
const R_AVR_CALL: u32 = 18u32;
```

### `R_AVR_LDI`

```rust
const R_AVR_LDI: u32 = 19u32;
```

### `R_AVR_6`

```rust
const R_AVR_6: u32 = 20u32;
```

### `R_AVR_6_ADIW`

```rust
const R_AVR_6_ADIW: u32 = 21u32;
```

### `R_AVR_MS8_LDI`

```rust
const R_AVR_MS8_LDI: u32 = 22u32;
```

### `R_AVR_MS8_LDI_NEG`

```rust
const R_AVR_MS8_LDI_NEG: u32 = 23u32;
```

### `R_AVR_LO8_LDI_GS`

```rust
const R_AVR_LO8_LDI_GS: u32 = 24u32;
```

### `R_AVR_HI8_LDI_GS`

```rust
const R_AVR_HI8_LDI_GS: u32 = 25u32;
```

### `R_AVR_8`

```rust
const R_AVR_8: u32 = 26u32;
```

### `R_AVR_8_LO8`

```rust
const R_AVR_8_LO8: u32 = 27u32;
```

### `R_AVR_8_HI8`

```rust
const R_AVR_8_HI8: u32 = 28u32;
```

### `R_AVR_8_HLO8`

```rust
const R_AVR_8_HLO8: u32 = 29u32;
```

### `R_AVR_DIFF8`

```rust
const R_AVR_DIFF8: u32 = 30u32;
```

### `R_AVR_DIFF16`

```rust
const R_AVR_DIFF16: u32 = 31u32;
```

### `R_AVR_DIFF32`

```rust
const R_AVR_DIFF32: u32 = 32u32;
```

### `R_AVR_LDS_STS_16`

```rust
const R_AVR_LDS_STS_16: u32 = 33u32;
```

### `R_AVR_PORT6`

```rust
const R_AVR_PORT6: u32 = 34u32;
```

### `R_AVR_PORT5`

```rust
const R_AVR_PORT5: u32 = 35u32;
```

### `R_AVR_32_PCREL`

```rust
const R_AVR_32_PCREL: u32 = 36u32;
```

### `R_MSP430_32`

```rust
const R_MSP430_32: u32 = 1u32;
```

Direct 32 bit

### `R_MSP430_16_BYTE`

```rust
const R_MSP430_16_BYTE: u32 = 5u32;
```

Direct 16 bit

### `R_HEX_32`

```rust
const R_HEX_32: u32 = 6u32;
```

Direct 32 bit

### `R_ARM_NONE`

```rust
const R_ARM_NONE: u32 = 0u32;
```

No reloc

### `R_ARM_PC24`

```rust
const R_ARM_PC24: u32 = 1u32;
```

Deprecated PC relative 26 bit branch.

### `R_ARM_ABS32`

```rust
const R_ARM_ABS32: u32 = 2u32;
```

Direct 32 bit

### `R_ARM_REL32`

```rust
const R_ARM_REL32: u32 = 3u32;
```

PC relative 32 bit

### `R_ARM_PC13`

```rust
const R_ARM_PC13: u32 = 4u32;
```

### `R_ARM_ABS16`

```rust
const R_ARM_ABS16: u32 = 5u32;
```

Direct 16 bit

### `R_ARM_ABS12`

```rust
const R_ARM_ABS12: u32 = 6u32;
```

Direct 12 bit

### `R_ARM_THM_ABS5`

```rust
const R_ARM_THM_ABS5: u32 = 7u32;
```

Direct & 0x7C (`LDR`, `STR`).

### `R_ARM_ABS8`

```rust
const R_ARM_ABS8: u32 = 8u32;
```

Direct 8 bit

### `R_ARM_SBREL32`

```rust
const R_ARM_SBREL32: u32 = 9u32;
```

### `R_ARM_THM_PC22`

```rust
const R_ARM_THM_PC22: u32 = 10u32;
```

PC relative 24 bit (Thumb32 `BL`).

### `R_ARM_THM_PC8`

```rust
const R_ARM_THM_PC8: u32 = 11u32;
```

PC relative & 0x3FC (Thumb16 `LDR`, `ADD`, `ADR`).

### `R_ARM_AMP_VCALL9`

```rust
const R_ARM_AMP_VCALL9: u32 = 12u32;
```

### `R_ARM_SWI24`

```rust
const R_ARM_SWI24: u32 = 13u32;
```

Obsolete static relocation.

### `R_ARM_TLS_DESC`

```rust
const R_ARM_TLS_DESC: u32 = 13u32;
```

Dynamic relocation.

### `R_ARM_THM_SWI8`

```rust
const R_ARM_THM_SWI8: u32 = 14u32;
```

Reserved.

### `R_ARM_XPC25`

```rust
const R_ARM_XPC25: u32 = 15u32;
```

Reserved.

### `R_ARM_THM_XPC22`

```rust
const R_ARM_THM_XPC22: u32 = 16u32;
```

Reserved.

### `R_ARM_TLS_DTPMOD32`

```rust
const R_ARM_TLS_DTPMOD32: u32 = 17u32;
```

ID of module containing symbol

### `R_ARM_TLS_DTPOFF32`

```rust
const R_ARM_TLS_DTPOFF32: u32 = 18u32;
```

Offset in TLS block

### `R_ARM_TLS_TPOFF32`

```rust
const R_ARM_TLS_TPOFF32: u32 = 19u32;
```

Offset in static TLS block

### `R_ARM_COPY`

```rust
const R_ARM_COPY: u32 = 20u32;
```

Copy symbol at runtime

### `R_ARM_GLOB_DAT`

```rust
const R_ARM_GLOB_DAT: u32 = 21u32;
```

Create GOT entry

### `R_ARM_JUMP_SLOT`

```rust
const R_ARM_JUMP_SLOT: u32 = 22u32;
```

Create PLT entry

### `R_ARM_RELATIVE`

```rust
const R_ARM_RELATIVE: u32 = 23u32;
```

Adjust by program base

### `R_ARM_GOTOFF`

```rust
const R_ARM_GOTOFF: u32 = 24u32;
```

32 bit offset to GOT

### `R_ARM_GOTPC`

```rust
const R_ARM_GOTPC: u32 = 25u32;
```

32 bit PC relative offset to GOT

### `R_ARM_GOT32`

```rust
const R_ARM_GOT32: u32 = 26u32;
```

32 bit GOT entry

### `R_ARM_PLT32`

```rust
const R_ARM_PLT32: u32 = 27u32;
```

Deprecated, 32 bit PLT address.

### `R_ARM_CALL`

```rust
const R_ARM_CALL: u32 = 28u32;
```

PC relative 24 bit (`BL`, `BLX`).

### `R_ARM_JUMP24`

```rust
const R_ARM_JUMP24: u32 = 29u32;
```

PC relative 24 bit (`B`, `BL<cond>`).

### `R_ARM_THM_JUMP24`

```rust
const R_ARM_THM_JUMP24: u32 = 30u32;
```

PC relative 24 bit (Thumb32 `B.W`).

### `R_ARM_BASE_ABS`

```rust
const R_ARM_BASE_ABS: u32 = 31u32;
```

Adjust by program base.

### `R_ARM_ALU_PCREL_7_0`

```rust
const R_ARM_ALU_PCREL_7_0: u32 = 32u32;
```

Obsolete.

### `R_ARM_ALU_PCREL_15_8`

```rust
const R_ARM_ALU_PCREL_15_8: u32 = 33u32;
```

Obsolete.

### `R_ARM_ALU_PCREL_23_15`

```rust
const R_ARM_ALU_PCREL_23_15: u32 = 34u32;
```

Obsolete.

### `R_ARM_LDR_SBREL_11_0`

```rust
const R_ARM_LDR_SBREL_11_0: u32 = 35u32;
```

Deprecated, prog. base relative.

### `R_ARM_ALU_SBREL_19_12`

```rust
const R_ARM_ALU_SBREL_19_12: u32 = 36u32;
```

Deprecated, prog. base relative.

### `R_ARM_ALU_SBREL_27_20`

```rust
const R_ARM_ALU_SBREL_27_20: u32 = 37u32;
```

Deprecated, prog. base relative.

### `R_ARM_TARGET1`

```rust
const R_ARM_TARGET1: u32 = 38u32;
```

### `R_ARM_SBREL31`

```rust
const R_ARM_SBREL31: u32 = 39u32;
```

Program base relative.

### `R_ARM_V4BX`

```rust
const R_ARM_V4BX: u32 = 40u32;
```

### `R_ARM_TARGET2`

```rust
const R_ARM_TARGET2: u32 = 41u32;
```

### `R_ARM_PREL31`

```rust
const R_ARM_PREL31: u32 = 42u32;
```

32 bit PC relative.

### `R_ARM_MOVW_ABS_NC`

```rust
const R_ARM_MOVW_ABS_NC: u32 = 43u32;
```

Direct 16-bit (`MOVW`).

### `R_ARM_MOVT_ABS`

```rust
const R_ARM_MOVT_ABS: u32 = 44u32;
```

Direct high 16-bit (`MOVT`).

### `R_ARM_MOVW_PREL_NC`

```rust
const R_ARM_MOVW_PREL_NC: u32 = 45u32;
```

PC relative 16-bit (`MOVW`).

### `R_ARM_MOVT_PREL`

```rust
const R_ARM_MOVT_PREL: u32 = 46u32;
```

PC relative (MOVT).

### `R_ARM_THM_MOVW_ABS_NC`

```rust
const R_ARM_THM_MOVW_ABS_NC: u32 = 47u32;
```

Direct 16 bit (Thumb32 `MOVW`).

### `R_ARM_THM_MOVT_ABS`

```rust
const R_ARM_THM_MOVT_ABS: u32 = 48u32;
```

Direct high 16 bit (Thumb32 `MOVT`).

### `R_ARM_THM_MOVW_PREL_NC`

```rust
const R_ARM_THM_MOVW_PREL_NC: u32 = 49u32;
```

PC relative 16 bit (Thumb32 `MOVW`).

### `R_ARM_THM_MOVT_PREL`

```rust
const R_ARM_THM_MOVT_PREL: u32 = 50u32;
```

PC relative high 16 bit (Thumb32 `MOVT`).

### `R_ARM_THM_JUMP19`

```rust
const R_ARM_THM_JUMP19: u32 = 51u32;
```

PC relative 20 bit (Thumb32 `B<cond>.W`).

### `R_ARM_THM_JUMP6`

```rust
const R_ARM_THM_JUMP6: u32 = 52u32;
```

PC relative X & 0x7E (Thumb16 `CBZ`, `CBNZ`).

### `R_ARM_THM_ALU_PREL_11_0`

```rust
const R_ARM_THM_ALU_PREL_11_0: u32 = 53u32;
```

PC relative 12 bit (Thumb32 `ADR.W`).

### `R_ARM_THM_PC12`

```rust
const R_ARM_THM_PC12: u32 = 54u32;
```

PC relative 12 bit (Thumb32 `LDR{D,SB,H,SH}`).

### `R_ARM_ABS32_NOI`

```rust
const R_ARM_ABS32_NOI: u32 = 55u32;
```

Direct 32-bit.

### `R_ARM_REL32_NOI`

```rust
const R_ARM_REL32_NOI: u32 = 56u32;
```

PC relative 32-bit.

### `R_ARM_ALU_PC_G0_NC`

```rust
const R_ARM_ALU_PC_G0_NC: u32 = 57u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G0`

```rust
const R_ARM_ALU_PC_G0: u32 = 58u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G1_NC`

```rust
const R_ARM_ALU_PC_G1_NC: u32 = 59u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G1`

```rust
const R_ARM_ALU_PC_G1: u32 = 60u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_ALU_PC_G2`

```rust
const R_ARM_ALU_PC_G2: u32 = 61u32;
```

PC relative (`ADD`, `SUB`).

### `R_ARM_LDR_PC_G1`

```rust
const R_ARM_LDR_PC_G1: u32 = 62u32;
```

PC relative (`LDR`,`STR`,`LDRB`,`STRB`).

### `R_ARM_LDR_PC_G2`

```rust
const R_ARM_LDR_PC_G2: u32 = 63u32;
```

PC relative (`LDR`,`STR`,`LDRB`,`STRB`).

### `R_ARM_LDRS_PC_G0`

```rust
const R_ARM_LDRS_PC_G0: u32 = 64u32;
```

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).

### `R_ARM_LDRS_PC_G1`

```rust
const R_ARM_LDRS_PC_G1: u32 = 65u32;
```

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).

### `R_ARM_LDRS_PC_G2`

```rust
const R_ARM_LDRS_PC_G2: u32 = 66u32;
```

PC relative (`STR{D,H}`, `LDR{D,SB,H,SH}`).

### `R_ARM_LDC_PC_G0`

```rust
const R_ARM_LDC_PC_G0: u32 = 67u32;
```

PC relative (`LDC`, `STC`).

### `R_ARM_LDC_PC_G1`

```rust
const R_ARM_LDC_PC_G1: u32 = 68u32;
```

PC relative (`LDC`, `STC`).

### `R_ARM_LDC_PC_G2`

```rust
const R_ARM_LDC_PC_G2: u32 = 69u32;
```

PC relative (`LDC`, `STC`).

### `R_ARM_ALU_SB_G0_NC`

```rust
const R_ARM_ALU_SB_G0_NC: u32 = 70u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G0`

```rust
const R_ARM_ALU_SB_G0: u32 = 71u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G1_NC`

```rust
const R_ARM_ALU_SB_G1_NC: u32 = 72u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G1`

```rust
const R_ARM_ALU_SB_G1: u32 = 73u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_ALU_SB_G2`

```rust
const R_ARM_ALU_SB_G2: u32 = 74u32;
```

Program base relative (`ADD`,`SUB`).

### `R_ARM_LDR_SB_G0`

```rust
const R_ARM_LDR_SB_G0: u32 = 75u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDR_SB_G1`

```rust
const R_ARM_LDR_SB_G1: u32 = 76u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDR_SB_G2`

```rust
const R_ARM_LDR_SB_G2: u32 = 77u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDRS_SB_G0`

```rust
const R_ARM_LDRS_SB_G0: u32 = 78u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDRS_SB_G1`

```rust
const R_ARM_LDRS_SB_G1: u32 = 79u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDRS_SB_G2`

```rust
const R_ARM_LDRS_SB_G2: u32 = 80u32;
```

Program base relative (`LDR`, `STR`, `LDRB`, `STRB`).

### `R_ARM_LDC_SB_G0`

```rust
const R_ARM_LDC_SB_G0: u32 = 81u32;
```

Program base relative (`LDC`,`STC`).

### `R_ARM_LDC_SB_G1`

```rust
const R_ARM_LDC_SB_G1: u32 = 82u32;
```

Program base relative (`LDC`,`STC`).

### `R_ARM_LDC_SB_G2`

```rust
const R_ARM_LDC_SB_G2: u32 = 83u32;
```

Program base relative (`LDC`,`STC`).

### `R_ARM_MOVW_BREL_NC`

```rust
const R_ARM_MOVW_BREL_NC: u32 = 84u32;
```

Program base relative 16 bit (`MOVW`).

### `R_ARM_MOVT_BREL`

```rust
const R_ARM_MOVT_BREL: u32 = 85u32;
```

Program base relative high 16 bit (`MOVT`).

### `R_ARM_MOVW_BREL`

```rust
const R_ARM_MOVW_BREL: u32 = 86u32;
```

Program base relative 16 bit (`MOVW`).

### `R_ARM_THM_MOVW_BREL_NC`

```rust
const R_ARM_THM_MOVW_BREL_NC: u32 = 87u32;
```

Program base relative 16 bit (Thumb32 `MOVW`).

### `R_ARM_THM_MOVT_BREL`

```rust
const R_ARM_THM_MOVT_BREL: u32 = 88u32;
```

Program base relative high 16 bit (Thumb32 `MOVT`).

### `R_ARM_THM_MOVW_BREL`

```rust
const R_ARM_THM_MOVW_BREL: u32 = 89u32;
```

Program base relative 16 bit (Thumb32 `MOVW`).

### `R_ARM_TLS_GOTDESC`

```rust
const R_ARM_TLS_GOTDESC: u32 = 90u32;
```

### `R_ARM_TLS_CALL`

```rust
const R_ARM_TLS_CALL: u32 = 91u32;
```

### `R_ARM_TLS_DESCSEQ`

```rust
const R_ARM_TLS_DESCSEQ: u32 = 92u32;
```

TLS relaxation.

### `R_ARM_THM_TLS_CALL`

```rust
const R_ARM_THM_TLS_CALL: u32 = 93u32;
```

### `R_ARM_PLT32_ABS`

```rust
const R_ARM_PLT32_ABS: u32 = 94u32;
```

### `R_ARM_GOT_ABS`

```rust
const R_ARM_GOT_ABS: u32 = 95u32;
```

GOT entry.

### `R_ARM_GOT_PREL`

```rust
const R_ARM_GOT_PREL: u32 = 96u32;
```

PC relative GOT entry.

### `R_ARM_GOT_BREL12`

```rust
const R_ARM_GOT_BREL12: u32 = 97u32;
```

GOT entry relative to GOT origin (`LDR`).

### `R_ARM_GOTOFF12`

```rust
const R_ARM_GOTOFF12: u32 = 98u32;
```

12 bit, GOT entry relative to GOT origin (`LDR`, `STR`).

### `R_ARM_GOTRELAX`

```rust
const R_ARM_GOTRELAX: u32 = 99u32;
```

### `R_ARM_GNU_VTENTRY`

```rust
const R_ARM_GNU_VTENTRY: u32 = 100u32;
```

### `R_ARM_GNU_VTINHERIT`

```rust
const R_ARM_GNU_VTINHERIT: u32 = 101u32;
```

### `R_ARM_THM_PC11`

```rust
const R_ARM_THM_PC11: u32 = 102u32;
```

PC relative & 0xFFE (Thumb16 `B`).

### `R_ARM_THM_PC9`

```rust
const R_ARM_THM_PC9: u32 = 103u32;
```

PC relative & 0x1FE (Thumb16 `B`/`B<cond>`).

### `R_ARM_TLS_GD32`

```rust
const R_ARM_TLS_GD32: u32 = 104u32;
```

PC-rel 32 bit for global dynamic thread local data

### `R_ARM_TLS_LDM32`

```rust
const R_ARM_TLS_LDM32: u32 = 105u32;
```

PC-rel 32 bit for local dynamic thread local data

### `R_ARM_TLS_LDO32`

```rust
const R_ARM_TLS_LDO32: u32 = 106u32;
```

32 bit offset relative to TLS block

### `R_ARM_TLS_IE32`

```rust
const R_ARM_TLS_IE32: u32 = 107u32;
```

PC-rel 32 bit for GOT entry of static TLS block offset

### `R_ARM_TLS_LE32`

```rust
const R_ARM_TLS_LE32: u32 = 108u32;
```

32 bit offset relative to static TLS block

### `R_ARM_TLS_LDO12`

```rust
const R_ARM_TLS_LDO12: u32 = 109u32;
```

12 bit relative to TLS block (`LDR`, `STR`).

### `R_ARM_TLS_LE12`

```rust
const R_ARM_TLS_LE12: u32 = 110u32;
```

12 bit relative to static TLS block (`LDR`, `STR`).

### `R_ARM_TLS_IE12GP`

```rust
const R_ARM_TLS_IE12GP: u32 = 111u32;
```

12 bit GOT entry relative to GOT origin (`LDR`).

### `R_ARM_ME_TOO`

```rust
const R_ARM_ME_TOO: u32 = 128u32;
```

Obsolete.

### `R_ARM_THM_TLS_DESCSEQ`

```rust
const R_ARM_THM_TLS_DESCSEQ: u32 = 129u32;
```

### `R_ARM_THM_TLS_DESCSEQ16`

```rust
const R_ARM_THM_TLS_DESCSEQ16: u32 = 129u32;
```

### `R_ARM_THM_TLS_DESCSEQ32`

```rust
const R_ARM_THM_TLS_DESCSEQ32: u32 = 130u32;
```

### `R_ARM_THM_GOT_BREL12`

```rust
const R_ARM_THM_GOT_BREL12: u32 = 131u32;
```

GOT entry relative to GOT origin, 12 bit (Thumb32 `LDR`).

### `R_ARM_IRELATIVE`

```rust
const R_ARM_IRELATIVE: u32 = 160u32;
```

### `R_ARM_RXPC25`

```rust
const R_ARM_RXPC25: u32 = 249u32;
```

### `R_ARM_RSBREL32`

```rust
const R_ARM_RSBREL32: u32 = 250u32;
```

### `R_ARM_THM_RPC22`

```rust
const R_ARM_THM_RPC22: u32 = 251u32;
```

### `R_ARM_RREL32`

```rust
const R_ARM_RREL32: u32 = 252u32;
```

### `R_ARM_RABS22`

```rust
const R_ARM_RABS22: u32 = 253u32;
```

### `R_ARM_RPC24`

```rust
const R_ARM_RPC24: u32 = 254u32;
```

### `R_ARM_RBASE`

```rust
const R_ARM_RBASE: u32 = 255u32;
```

### `R_CKCORE_NONE`

```rust
const R_CKCORE_NONE: u32 = 0u32;
```

no reloc

### `R_CKCORE_ADDR32`

```rust
const R_CKCORE_ADDR32: u32 = 1u32;
```

direct 32 bit (S + A)

### `R_CKCORE_PCRELIMM8BY4`

```rust
const R_CKCORE_PCRELIMM8BY4: u32 = 2u32;
```

disp ((S + A - P) >> 2) & 0xff

### `R_CKCORE_PCRELIMM11BY2`

```rust
const R_CKCORE_PCRELIMM11BY2: u32 = 3u32;
```

disp ((S + A - P) >> 1) & 0x7ff

### `R_CKCORE_PCREL32`

```rust
const R_CKCORE_PCREL32: u32 = 5u32;
```

32-bit rel (S + A - P)

### `R_CKCORE_PCRELJSR_IMM11BY2`

```rust
const R_CKCORE_PCRELJSR_IMM11BY2: u32 = 6u32;
```

disp ((S + A - P) >>1) & 0x7ff

### `R_CKCORE_RELATIVE`

```rust
const R_CKCORE_RELATIVE: u32 = 9u32;
```

32 bit adjust program base(B + A)

### `R_CKCORE_COPY`

```rust
const R_CKCORE_COPY: u32 = 10u32;
```

32 bit adjust by program base

### `R_CKCORE_GLOB_DAT`

```rust
const R_CKCORE_GLOB_DAT: u32 = 11u32;
```

off between got and sym (S)

### `R_CKCORE_JUMP_SLOT`

```rust
const R_CKCORE_JUMP_SLOT: u32 = 12u32;
```

PLT entry (S)

### `R_CKCORE_GOTOFF`

```rust
const R_CKCORE_GOTOFF: u32 = 13u32;
```

offset to GOT (S + A - GOT)

### `R_CKCORE_GOTPC`

```rust
const R_CKCORE_GOTPC: u32 = 14u32;
```

PC offset to GOT (GOT + A - P)

### `R_CKCORE_GOT32`

```rust
const R_CKCORE_GOT32: u32 = 15u32;
```

32 bit GOT entry (G)

### `R_CKCORE_PLT32`

```rust
const R_CKCORE_PLT32: u32 = 16u32;
```

32 bit PLT entry (G)

### `R_CKCORE_ADDRGOT`

```rust
const R_CKCORE_ADDRGOT: u32 = 17u32;
```

GOT entry in GLOB_DAT (GOT + G)

### `R_CKCORE_ADDRPLT`

```rust
const R_CKCORE_ADDRPLT: u32 = 18u32;
```

PLT entry in GLOB_DAT (GOT + G)

### `R_CKCORE_PCREL_IMM26BY2`

```rust
const R_CKCORE_PCREL_IMM26BY2: u32 = 19u32;
```

((S + A - P) >> 1) & 0x3ff_ffff

### `R_CKCORE_PCREL_IMM16BY2`

```rust
const R_CKCORE_PCREL_IMM16BY2: u32 = 20u32;
```

disp ((S + A - P) >> 1) & 0xffff

### `R_CKCORE_PCREL_IMM16BY4`

```rust
const R_CKCORE_PCREL_IMM16BY4: u32 = 21u32;
```

disp ((S + A - P) >> 2) & 0xffff

### `R_CKCORE_PCREL_IMM10BY2`

```rust
const R_CKCORE_PCREL_IMM10BY2: u32 = 22u32;
```

disp ((S + A - P) >> 1) & 0x3ff

### `R_CKCORE_PCREL_IMM10BY4`

```rust
const R_CKCORE_PCREL_IMM10BY4: u32 = 23u32;
```

disp ((S + A - P) >> 2) & 0x3ff

### `R_CKCORE_ADDR_HI16`

```rust
const R_CKCORE_ADDR_HI16: u32 = 24u32;
```

high & low 16 bit ADDR, ((S + A) >> 16) & 0xffff

### `R_CKCORE_ADDR_LO16`

```rust
const R_CKCORE_ADDR_LO16: u32 = 25u32;
```

(S + A) & 0xffff

### `R_CKCORE_GOTPC_HI16`

```rust
const R_CKCORE_GOTPC_HI16: u32 = 26u32;
```

high & low 16 bit GOTPC, ((GOT + A - P) >> 16) & 0xffff

### `R_CKCORE_GOTPC_LO16`

```rust
const R_CKCORE_GOTPC_LO16: u32 = 27u32;
```

(GOT + A - P) & 0xffff

### `R_CKCORE_GOTOFF_HI16`

```rust
const R_CKCORE_GOTOFF_HI16: u32 = 28u32;
```

high & low 16 bit GOTOFF, ((S + A - GOT) >> 16) & 0xffff

### `R_CKCORE_GOTOFF_LO16`

```rust
const R_CKCORE_GOTOFF_LO16: u32 = 29u32;
```

(S + A - GOT) & 0xffff

### `R_CKCORE_GOT12`

```rust
const R_CKCORE_GOT12: u32 = 30u32;
```

12 bit disp GOT entry (G)

### `R_CKCORE_GOT_HI16`

```rust
const R_CKCORE_GOT_HI16: u32 = 31u32;
```

high & low 16 bit GOT, (G >> 16) & 0xffff

### `R_CKCORE_GOT_LO16`

```rust
const R_CKCORE_GOT_LO16: u32 = 32u32;
```

(G & 0xffff)

### `R_CKCORE_PLT12`

```rust
const R_CKCORE_PLT12: u32 = 33u32;
```

12 bit disp PLT entry (G)

### `R_CKCORE_PLT_HI16`

```rust
const R_CKCORE_PLT_HI16: u32 = 34u32;
```

high & low 16 bit PLT, (G >> 16) & 0xffff

### `R_CKCORE_PLT_LO16`

```rust
const R_CKCORE_PLT_LO16: u32 = 35u32;
```

G & 0xffff

### `R_CKCORE_ADDRGOT_HI16`

```rust
const R_CKCORE_ADDRGOT_HI16: u32 = 36u32;
```

high & low 16 bit ADDRGOT, (GOT + G * 4) & 0xffff

### `R_CKCORE_ADDRGOT_LO16`

```rust
const R_CKCORE_ADDRGOT_LO16: u32 = 37u32;
```

(GOT + G * 4) & 0xffff

### `R_CKCORE_ADDRPLT_HI16`

```rust
const R_CKCORE_ADDRPLT_HI16: u32 = 38u32;
```

high & low 16 bit ADDRPLT, ((GOT + G * 4) >> 16) & 0xFFFF

### `R_CKCORE_ADDRPLT_LO16`

```rust
const R_CKCORE_ADDRPLT_LO16: u32 = 39u32;
```

(GOT+G*4) & 0xffff

### `R_CKCORE_PCREL_JSR_IMM26BY2`

```rust
const R_CKCORE_PCREL_JSR_IMM26BY2: u32 = 40u32;
```

disp ((S+A-P) >>1) & x3ff_ffff

### `R_CKCORE_TOFFSET_LO16`

```rust
const R_CKCORE_TOFFSET_LO16: u32 = 41u32;
```

(S+A-BTEXT) & 0xffff

### `R_CKCORE_DOFFSET_LO16`

```rust
const R_CKCORE_DOFFSET_LO16: u32 = 42u32;
```

(S+A-BTEXT) & 0xffff

### `R_CKCORE_PCREL_IMM18BY2`

```rust
const R_CKCORE_PCREL_IMM18BY2: u32 = 43u32;
```

disp ((S+A-P) >>1) & 0x3ffff

### `R_CKCORE_DOFFSET_IMM18`

```rust
const R_CKCORE_DOFFSET_IMM18: u32 = 44u32;
```

disp (S+A-BDATA) & 0x3ffff

### `R_CKCORE_DOFFSET_IMM18BY2`

```rust
const R_CKCORE_DOFFSET_IMM18BY2: u32 = 45u32;
```

disp ((S+A-BDATA)>>1) & 0x3ffff

### `R_CKCORE_DOFFSET_IMM18BY4`

```rust
const R_CKCORE_DOFFSET_IMM18BY4: u32 = 46u32;
```

disp ((S+A-BDATA)>>2) & 0x3ffff

### `R_CKCORE_GOT_IMM18BY4`

```rust
const R_CKCORE_GOT_IMM18BY4: u32 = 48u32;
```

disp (G >> 2)

### `R_CKCORE_PLT_IMM18BY4`

```rust
const R_CKCORE_PLT_IMM18BY4: u32 = 49u32;
```

disp (G >> 2)

### `R_CKCORE_PCREL_IMM7BY4`

```rust
const R_CKCORE_PCREL_IMM7BY4: u32 = 50u32;
```

disp ((S+A-P) >>2) & 0x7f

### `R_CKCORE_TLS_LE32`

```rust
const R_CKCORE_TLS_LE32: u32 = 51u32;
```

32 bit offset to TLS block

### `R_CKCORE_TLS_IE32`

```rust
const R_CKCORE_TLS_IE32: u32 = 52u32;
```

### `R_CKCORE_TLS_GD32`

```rust
const R_CKCORE_TLS_GD32: u32 = 53u32;
```

### `R_CKCORE_TLS_LDM32`

```rust
const R_CKCORE_TLS_LDM32: u32 = 54u32;
```

### `R_CKCORE_TLS_LDO32`

```rust
const R_CKCORE_TLS_LDO32: u32 = 55u32;
```

### `R_CKCORE_TLS_DTPMOD32`

```rust
const R_CKCORE_TLS_DTPMOD32: u32 = 56u32;
```

### `R_CKCORE_TLS_DTPOFF32`

```rust
const R_CKCORE_TLS_DTPOFF32: u32 = 57u32;
```

### `R_CKCORE_TLS_TPOFF32`

```rust
const R_CKCORE_TLS_TPOFF32: u32 = 58u32;
```

### `EF_CSKY_ABIMASK`

```rust
const EF_CSKY_ABIMASK: u32 = 4_026_531_840u32;
```

### `EF_CSKY_OTHER`

```rust
const EF_CSKY_OTHER: u32 = 268_369_920u32;
```

### `EF_CSKY_PROCESSOR`

```rust
const EF_CSKY_PROCESSOR: u32 = 65_535u32;
```

### `EF_CSKY_ABIV1`

```rust
const EF_CSKY_ABIV1: u32 = 268_435_456u32;
```

### `EF_CSKY_ABIV2`

```rust
const EF_CSKY_ABIV2: u32 = 536_870_912u32;
```

### `SHT_CSKY_ATTRIBUTES`

```rust
const SHT_CSKY_ATTRIBUTES: u32 = 1_879_048_193u32;
```

C-SKY attributes section.

### `EF_IA_64_MASKOS`

```rust
const EF_IA_64_MASKOS: u32 = 15u32;
```

os-specific flags

### `EF_IA_64_ABI64`

```rust
const EF_IA_64_ABI64: u32 = 16u32;
```

64-bit ABI

### `EF_IA_64_ARCH`

```rust
const EF_IA_64_ARCH: u32 = 4_278_190_080u32;
```

arch. version mask

### `PT_IA_64_ARCHEXT`

```rust
const PT_IA_64_ARCHEXT: u32 = 1_879_048_192u32;
```

arch extension bits

### `PT_IA_64_UNWIND`

```rust
const PT_IA_64_UNWIND: u32 = 1_879_048_193u32;
```

ia64 unwind bits

### `PT_IA_64_HP_OPT_ANOT`

```rust
const PT_IA_64_HP_OPT_ANOT: u32 = 1_610_612_754u32;
```

### `PT_IA_64_HP_HSL_ANOT`

```rust
const PT_IA_64_HP_HSL_ANOT: u32 = 1_610_612_755u32;
```

### `PT_IA_64_HP_STACK`

```rust
const PT_IA_64_HP_STACK: u32 = 1_610_612_756u32;
```

### `PF_IA_64_NORECOV`

```rust
const PF_IA_64_NORECOV: u32 = 2_147_483_648u32;
```

spec insns w/o recovery

### `SHT_IA_64_EXT`

```rust
const SHT_IA_64_EXT: u32 = 1_879_048_192u32;
```

extension bits

### `SHT_IA_64_UNWIND`

```rust
const SHT_IA_64_UNWIND: u32 = 1_879_048_193u32;
```

unwind bits

### `SHF_IA_64_SHORT`

```rust
const SHF_IA_64_SHORT: u32 = 268_435_456u32;
```

section near gp

### `SHF_IA_64_NORECOV`

```rust
const SHF_IA_64_NORECOV: u32 = 536_870_912u32;
```

spec insns w/o recovery

### `DT_IA_64_PLT_RESERVE`

```rust
const DT_IA_64_PLT_RESERVE: u32 = 1_879_048_192u32;
```

### `R_IA64_NONE`

```rust
const R_IA64_NONE: u32 = 0u32;
```

none

### `R_IA64_IMM14`

```rust
const R_IA64_IMM14: u32 = 33u32;
```

symbol + addend, add imm14

### `R_IA64_IMM22`

```rust
const R_IA64_IMM22: u32 = 34u32;
```

symbol + addend, add imm22

### `R_IA64_IMM64`

```rust
const R_IA64_IMM64: u32 = 35u32;
```

symbol + addend, mov imm64

### `R_IA64_DIR32MSB`

```rust
const R_IA64_DIR32MSB: u32 = 36u32;
```

symbol + addend, data4 MSB

### `R_IA64_DIR32LSB`

```rust
const R_IA64_DIR32LSB: u32 = 37u32;
```

symbol + addend, data4 LSB

### `R_IA64_DIR64MSB`

```rust
const R_IA64_DIR64MSB: u32 = 38u32;
```

symbol + addend, data8 MSB

### `R_IA64_DIR64LSB`

```rust
const R_IA64_DIR64LSB: u32 = 39u32;
```

symbol + addend, data8 LSB

### `R_IA64_GPREL22`

```rust
const R_IA64_GPREL22: u32 = 42u32;
```

@gprel(sym + add), add imm22

### `R_IA64_GPREL64I`

```rust
const R_IA64_GPREL64I: u32 = 43u32;
```

@gprel(sym + add), mov imm64

### `R_IA64_GPREL32MSB`

```rust
const R_IA64_GPREL32MSB: u32 = 44u32;
```

@gprel(sym + add), data4 MSB

### `R_IA64_GPREL32LSB`

```rust
const R_IA64_GPREL32LSB: u32 = 45u32;
```

@gprel(sym + add), data4 LSB

### `R_IA64_GPREL64MSB`

```rust
const R_IA64_GPREL64MSB: u32 = 46u32;
```

@gprel(sym + add), data8 MSB

### `R_IA64_GPREL64LSB`

```rust
const R_IA64_GPREL64LSB: u32 = 47u32;
```

@gprel(sym + add), data8 LSB

### `R_IA64_LTOFF22`

```rust
const R_IA64_LTOFF22: u32 = 50u32;
```

@ltoff(sym + add), add imm22

### `R_IA64_LTOFF64I`

```rust
const R_IA64_LTOFF64I: u32 = 51u32;
```

@ltoff(sym + add), mov imm64

### `R_IA64_PLTOFF22`

```rust
const R_IA64_PLTOFF22: u32 = 58u32;
```

@pltoff(sym + add), add imm22

### `R_IA64_PLTOFF64I`

```rust
const R_IA64_PLTOFF64I: u32 = 59u32;
```

@pltoff(sym + add), mov imm64

### `R_IA64_PLTOFF64MSB`

```rust
const R_IA64_PLTOFF64MSB: u32 = 62u32;
```

@pltoff(sym + add), data8 MSB

### `R_IA64_PLTOFF64LSB`

```rust
const R_IA64_PLTOFF64LSB: u32 = 63u32;
```

@pltoff(sym + add), data8 LSB

### `R_IA64_FPTR64I`

```rust
const R_IA64_FPTR64I: u32 = 67u32;
```

@fptr(sym + add), mov imm64

### `R_IA64_FPTR32MSB`

```rust
const R_IA64_FPTR32MSB: u32 = 68u32;
```

@fptr(sym + add), data4 MSB

### `R_IA64_FPTR32LSB`

```rust
const R_IA64_FPTR32LSB: u32 = 69u32;
```

@fptr(sym + add), data4 LSB

### `R_IA64_FPTR64MSB`

```rust
const R_IA64_FPTR64MSB: u32 = 70u32;
```

@fptr(sym + add), data8 MSB

### `R_IA64_FPTR64LSB`

```rust
const R_IA64_FPTR64LSB: u32 = 71u32;
```

@fptr(sym + add), data8 LSB

### `R_IA64_PCREL60B`

```rust
const R_IA64_PCREL60B: u32 = 72u32;
```

@pcrel(sym + add), brl

### `R_IA64_PCREL21B`

```rust
const R_IA64_PCREL21B: u32 = 73u32;
```

@pcrel(sym + add), ptb, call

### `R_IA64_PCREL21M`

```rust
const R_IA64_PCREL21M: u32 = 74u32;
```

@pcrel(sym + add), chk.s

### `R_IA64_PCREL21F`

```rust
const R_IA64_PCREL21F: u32 = 75u32;
```

@pcrel(sym + add), fchkf

### `R_IA64_PCREL32MSB`

```rust
const R_IA64_PCREL32MSB: u32 = 76u32;
```

@pcrel(sym + add), data4 MSB

### `R_IA64_PCREL32LSB`

```rust
const R_IA64_PCREL32LSB: u32 = 77u32;
```

@pcrel(sym + add), data4 LSB

### `R_IA64_PCREL64MSB`

```rust
const R_IA64_PCREL64MSB: u32 = 78u32;
```

@pcrel(sym + add), data8 MSB

### `R_IA64_PCREL64LSB`

```rust
const R_IA64_PCREL64LSB: u32 = 79u32;
```

@pcrel(sym + add), data8 LSB

### `R_IA64_LTOFF_FPTR22`

```rust
const R_IA64_LTOFF_FPTR22: u32 = 82u32;
```

@ltoff(@fptr(s+a)), imm22

### `R_IA64_LTOFF_FPTR64I`

```rust
const R_IA64_LTOFF_FPTR64I: u32 = 83u32;
```

@ltoff(@fptr(s+a)), imm64

### `R_IA64_LTOFF_FPTR32MSB`

```rust
const R_IA64_LTOFF_FPTR32MSB: u32 = 84u32;
```

@ltoff(@fptr(s+a)), data4 MSB

### `R_IA64_LTOFF_FPTR32LSB`

```rust
const R_IA64_LTOFF_FPTR32LSB: u32 = 85u32;
```

@ltoff(@fptr(s+a)), data4 LSB

### `R_IA64_LTOFF_FPTR64MSB`

```rust
const R_IA64_LTOFF_FPTR64MSB: u32 = 86u32;
```

@ltoff(@fptr(s+a)), data8 MSB

### `R_IA64_LTOFF_FPTR64LSB`

```rust
const R_IA64_LTOFF_FPTR64LSB: u32 = 87u32;
```

@ltoff(@fptr(s+a)), data8 LSB

### `R_IA64_SEGREL32MSB`

```rust
const R_IA64_SEGREL32MSB: u32 = 92u32;
```

@segrel(sym + add), data4 MSB

### `R_IA64_SEGREL32LSB`

```rust
const R_IA64_SEGREL32LSB: u32 = 93u32;
```

@segrel(sym + add), data4 LSB

### `R_IA64_SEGREL64MSB`

```rust
const R_IA64_SEGREL64MSB: u32 = 94u32;
```

@segrel(sym + add), data8 MSB

### `R_IA64_SEGREL64LSB`

```rust
const R_IA64_SEGREL64LSB: u32 = 95u32;
```

@segrel(sym + add), data8 LSB

### `R_IA64_SECREL32MSB`

```rust
const R_IA64_SECREL32MSB: u32 = 100u32;
```

@secrel(sym + add), data4 MSB

### `R_IA64_SECREL32LSB`

```rust
const R_IA64_SECREL32LSB: u32 = 101u32;
```

@secrel(sym + add), data4 LSB

### `R_IA64_SECREL64MSB`

```rust
const R_IA64_SECREL64MSB: u32 = 102u32;
```

@secrel(sym + add), data8 MSB

### `R_IA64_SECREL64LSB`

```rust
const R_IA64_SECREL64LSB: u32 = 103u32;
```

@secrel(sym + add), data8 LSB

### `R_IA64_REL32MSB`

```rust
const R_IA64_REL32MSB: u32 = 108u32;
```

data 4 + REL

### `R_IA64_REL32LSB`

```rust
const R_IA64_REL32LSB: u32 = 109u32;
```

data 4 + REL

### `R_IA64_REL64MSB`

```rust
const R_IA64_REL64MSB: u32 = 110u32;
```

data 8 + REL

### `R_IA64_REL64LSB`

```rust
const R_IA64_REL64LSB: u32 = 111u32;
```

data 8 + REL

### `R_IA64_LTV32MSB`

```rust
const R_IA64_LTV32MSB: u32 = 116u32;
```

symbol + addend, data4 MSB

### `R_IA64_LTV32LSB`

```rust
const R_IA64_LTV32LSB: u32 = 117u32;
```

symbol + addend, data4 LSB

### `R_IA64_LTV64MSB`

```rust
const R_IA64_LTV64MSB: u32 = 118u32;
```

symbol + addend, data8 MSB

### `R_IA64_LTV64LSB`

```rust
const R_IA64_LTV64LSB: u32 = 119u32;
```

symbol + addend, data8 LSB

### `R_IA64_PCREL21BI`

```rust
const R_IA64_PCREL21BI: u32 = 121u32;
```

@pcrel(sym + add), 21bit inst

### `R_IA64_PCREL22`

```rust
const R_IA64_PCREL22: u32 = 122u32;
```

@pcrel(sym + add), 22bit inst

### `R_IA64_PCREL64I`

```rust
const R_IA64_PCREL64I: u32 = 123u32;
```

@pcrel(sym + add), 64bit inst

### `R_IA64_IPLTMSB`

```rust
const R_IA64_IPLTMSB: u32 = 128u32;
```

dynamic reloc, imported PLT, MSB

### `R_IA64_IPLTLSB`

```rust
const R_IA64_IPLTLSB: u32 = 129u32;
```

dynamic reloc, imported PLT, LSB

### `R_IA64_COPY`

```rust
const R_IA64_COPY: u32 = 132u32;
```

copy relocation

### `R_IA64_SUB`

```rust
const R_IA64_SUB: u32 = 133u32;
```

Addend and symbol difference

### `R_IA64_LTOFF22X`

```rust
const R_IA64_LTOFF22X: u32 = 134u32;
```

LTOFF22, relaxable.

### `R_IA64_LDXMOV`

```rust
const R_IA64_LDXMOV: u32 = 135u32;
```

Use of LTOFF22X.

### `R_IA64_TPREL14`

```rust
const R_IA64_TPREL14: u32 = 145u32;
```

@tprel(sym + add), imm14

### `R_IA64_TPREL22`

```rust
const R_IA64_TPREL22: u32 = 146u32;
```

@tprel(sym + add), imm22

### `R_IA64_TPREL64I`

```rust
const R_IA64_TPREL64I: u32 = 147u32;
```

@tprel(sym + add), imm64

### `R_IA64_TPREL64MSB`

```rust
const R_IA64_TPREL64MSB: u32 = 150u32;
```

@tprel(sym + add), data8 MSB

### `R_IA64_TPREL64LSB`

```rust
const R_IA64_TPREL64LSB: u32 = 151u32;
```

@tprel(sym + add), data8 LSB

### `R_IA64_LTOFF_TPREL22`

```rust
const R_IA64_LTOFF_TPREL22: u32 = 154u32;
```

@ltoff(@tprel(s+a)), imm2

### `R_IA64_DTPMOD64MSB`

```rust
const R_IA64_DTPMOD64MSB: u32 = 166u32;
```

@dtpmod(sym + add), data8 MSB

### `R_IA64_DTPMOD64LSB`

```rust
const R_IA64_DTPMOD64LSB: u32 = 167u32;
```

@dtpmod(sym + add), data8 LSB

### `R_IA64_LTOFF_DTPMOD22`

```rust
const R_IA64_LTOFF_DTPMOD22: u32 = 170u32;
```

@ltoff(@dtpmod(sym + add)), imm22

### `R_IA64_DTPREL14`

```rust
const R_IA64_DTPREL14: u32 = 177u32;
```

@dtprel(sym + add), imm14

### `R_IA64_DTPREL22`

```rust
const R_IA64_DTPREL22: u32 = 178u32;
```

@dtprel(sym + add), imm22

### `R_IA64_DTPREL64I`

```rust
const R_IA64_DTPREL64I: u32 = 179u32;
```

@dtprel(sym + add), imm64

### `R_IA64_DTPREL32MSB`

```rust
const R_IA64_DTPREL32MSB: u32 = 180u32;
```

@dtprel(sym + add), data4 MSB

### `R_IA64_DTPREL32LSB`

```rust
const R_IA64_DTPREL32LSB: u32 = 181u32;
```

@dtprel(sym + add), data4 LSB

### `R_IA64_DTPREL64MSB`

```rust
const R_IA64_DTPREL64MSB: u32 = 182u32;
```

@dtprel(sym + add), data8 MSB

### `R_IA64_DTPREL64LSB`

```rust
const R_IA64_DTPREL64LSB: u32 = 183u32;
```

@dtprel(sym + add), data8 LSB

### `R_IA64_LTOFF_DTPREL22`

```rust
const R_IA64_LTOFF_DTPREL22: u32 = 186u32;
```

@ltoff(@dtprel(s+a)), imm22

### `EF_SH_MACH_MASK`

```rust
const EF_SH_MACH_MASK: u32 = 31u32;
```

### `EF_SH_UNKNOWN`

```rust
const EF_SH_UNKNOWN: u32 = 0u32;
```

### `EF_SH1`

```rust
const EF_SH1: u32 = 1u32;
```

### `EF_SH2`

```rust
const EF_SH2: u32 = 2u32;
```

### `EF_SH3`

```rust
const EF_SH3: u32 = 3u32;
```

### `EF_SH_DSP`

```rust
const EF_SH_DSP: u32 = 4u32;
```

### `EF_SH3_DSP`

```rust
const EF_SH3_DSP: u32 = 5u32;
```

### `EF_SH4AL_DSP`

```rust
const EF_SH4AL_DSP: u32 = 6u32;
```

### `EF_SH3E`

```rust
const EF_SH3E: u32 = 8u32;
```

### `EF_SH4`

```rust
const EF_SH4: u32 = 9u32;
```

### `EF_SH2E`

```rust
const EF_SH2E: u32 = 11u32;
```

### `EF_SH4A`

```rust
const EF_SH4A: u32 = 12u32;
```

### `EF_SH2A`

```rust
const EF_SH2A: u32 = 13u32;
```

### `EF_SH4_NOFPU`

```rust
const EF_SH4_NOFPU: u32 = 16u32;
```

### `EF_SH4A_NOFPU`

```rust
const EF_SH4A_NOFPU: u32 = 17u32;
```

### `EF_SH4_NOMMU_NOFPU`

```rust
const EF_SH4_NOMMU_NOFPU: u32 = 18u32;
```

### `EF_SH2A_NOFPU`

```rust
const EF_SH2A_NOFPU: u32 = 19u32;
```

### `EF_SH3_NOMMU`

```rust
const EF_SH3_NOMMU: u32 = 20u32;
```

### `EF_SH2A_SH4_NOFPU`

```rust
const EF_SH2A_SH4_NOFPU: u32 = 21u32;
```

### `EF_SH2A_SH3_NOFPU`

```rust
const EF_SH2A_SH3_NOFPU: u32 = 22u32;
```

### `EF_SH2A_SH4`

```rust
const EF_SH2A_SH4: u32 = 23u32;
```

### `EF_SH2A_SH3E`

```rust
const EF_SH2A_SH3E: u32 = 24u32;
```

### `R_SH_NONE`

```rust
const R_SH_NONE: u32 = 0u32;
```

### `R_SH_DIR32`

```rust
const R_SH_DIR32: u32 = 1u32;
```

### `R_SH_REL32`

```rust
const R_SH_REL32: u32 = 2u32;
```

### `R_SH_DIR8WPN`

```rust
const R_SH_DIR8WPN: u32 = 3u32;
```

### `R_SH_IND12W`

```rust
const R_SH_IND12W: u32 = 4u32;
```

### `R_SH_DIR8WPL`

```rust
const R_SH_DIR8WPL: u32 = 5u32;
```

### `R_SH_DIR8WPZ`

```rust
const R_SH_DIR8WPZ: u32 = 6u32;
```

### `R_SH_DIR8BP`

```rust
const R_SH_DIR8BP: u32 = 7u32;
```

### `R_SH_DIR8W`

```rust
const R_SH_DIR8W: u32 = 8u32;
```

### `R_SH_DIR8L`

```rust
const R_SH_DIR8L: u32 = 9u32;
```

### `R_SH_SWITCH16`

```rust
const R_SH_SWITCH16: u32 = 25u32;
```

### `R_SH_SWITCH32`

```rust
const R_SH_SWITCH32: u32 = 26u32;
```

### `R_SH_USES`

```rust
const R_SH_USES: u32 = 27u32;
```

### `R_SH_COUNT`

```rust
const R_SH_COUNT: u32 = 28u32;
```

### `R_SH_ALIGN`

```rust
const R_SH_ALIGN: u32 = 29u32;
```

### `R_SH_CODE`

```rust
const R_SH_CODE: u32 = 30u32;
```

### `R_SH_DATA`

```rust
const R_SH_DATA: u32 = 31u32;
```

### `R_SH_LABEL`

```rust
const R_SH_LABEL: u32 = 32u32;
```

### `R_SH_SWITCH8`

```rust
const R_SH_SWITCH8: u32 = 33u32;
```

### `R_SH_GNU_VTINHERIT`

```rust
const R_SH_GNU_VTINHERIT: u32 = 34u32;
```

### `R_SH_GNU_VTENTRY`

```rust
const R_SH_GNU_VTENTRY: u32 = 35u32;
```

### `R_SH_TLS_GD_32`

```rust
const R_SH_TLS_GD_32: u32 = 144u32;
```

### `R_SH_TLS_LD_32`

```rust
const R_SH_TLS_LD_32: u32 = 145u32;
```

### `R_SH_TLS_LDO_32`

```rust
const R_SH_TLS_LDO_32: u32 = 146u32;
```

### `R_SH_TLS_IE_32`

```rust
const R_SH_TLS_IE_32: u32 = 147u32;
```

### `R_SH_TLS_LE_32`

```rust
const R_SH_TLS_LE_32: u32 = 148u32;
```

### `R_SH_TLS_DTPMOD32`

```rust
const R_SH_TLS_DTPMOD32: u32 = 149u32;
```

### `R_SH_TLS_DTPOFF32`

```rust
const R_SH_TLS_DTPOFF32: u32 = 150u32;
```

### `R_SH_TLS_TPOFF32`

```rust
const R_SH_TLS_TPOFF32: u32 = 151u32;
```

### `R_SH_GOT32`

```rust
const R_SH_GOT32: u32 = 160u32;
```

### `R_SH_PLT32`

```rust
const R_SH_PLT32: u32 = 161u32;
```

### `R_SH_COPY`

```rust
const R_SH_COPY: u32 = 162u32;
```

### `R_SH_GLOB_DAT`

```rust
const R_SH_GLOB_DAT: u32 = 163u32;
```

### `R_SH_JMP_SLOT`

```rust
const R_SH_JMP_SLOT: u32 = 164u32;
```

### `R_SH_RELATIVE`

```rust
const R_SH_RELATIVE: u32 = 165u32;
```

### `R_SH_GOTOFF`

```rust
const R_SH_GOTOFF: u32 = 166u32;
```

### `R_SH_GOTPC`

```rust
const R_SH_GOTPC: u32 = 167u32;
```

### `EF_S390_HIGH_GPRS`

```rust
const EF_S390_HIGH_GPRS: u32 = 1u32;
```

High GPRs kernel facility needed.

### `R_390_NONE`

```rust
const R_390_NONE: u32 = 0u32;
```

No reloc.

### `R_390_8`

```rust
const R_390_8: u32 = 1u32;
```

Direct 8 bit.

### `R_390_12`

```rust
const R_390_12: u32 = 2u32;
```

Direct 12 bit.

### `R_390_16`

```rust
const R_390_16: u32 = 3u32;
```

Direct 16 bit.

### `R_390_32`

```rust
const R_390_32: u32 = 4u32;
```

Direct 32 bit.

### `R_390_PC32`

```rust
const R_390_PC32: u32 = 5u32;
```

PC relative 32 bit.

### `R_390_GOT12`

```rust
const R_390_GOT12: u32 = 6u32;
```

12 bit GOT offset.

### `R_390_GOT32`

```rust
const R_390_GOT32: u32 = 7u32;
```

32 bit GOT offset.

### `R_390_PLT32`

```rust
const R_390_PLT32: u32 = 8u32;
```

32 bit PC relative PLT address.

### `R_390_COPY`

```rust
const R_390_COPY: u32 = 9u32;
```

Copy symbol at runtime.

### `R_390_GLOB_DAT`

```rust
const R_390_GLOB_DAT: u32 = 10u32;
```

Create GOT entry.

### `R_390_JMP_SLOT`

```rust
const R_390_JMP_SLOT: u32 = 11u32;
```

Create PLT entry.

### `R_390_RELATIVE`

```rust
const R_390_RELATIVE: u32 = 12u32;
```

Adjust by program base.

### `R_390_GOTOFF32`

```rust
const R_390_GOTOFF32: u32 = 13u32;
```

32 bit offset to GOT.

### `R_390_GOTPC`

```rust
const R_390_GOTPC: u32 = 14u32;
```

32 bit PC relative offset to GOT.

### `R_390_GOT16`

```rust
const R_390_GOT16: u32 = 15u32;
```

16 bit GOT offset.

### `R_390_PC16`

```rust
const R_390_PC16: u32 = 16u32;
```

PC relative 16 bit.

### `R_390_PC16DBL`

```rust
const R_390_PC16DBL: u32 = 17u32;
```

PC relative 16 bit shifted by 1.

### `R_390_PLT16DBL`

```rust
const R_390_PLT16DBL: u32 = 18u32;
```

16 bit PC rel. PLT shifted by 1.

### `R_390_PC32DBL`

```rust
const R_390_PC32DBL: u32 = 19u32;
```

PC relative 32 bit shifted by 1.

### `R_390_PLT32DBL`

```rust
const R_390_PLT32DBL: u32 = 20u32;
```

32 bit PC rel. PLT shifted by 1.

### `R_390_GOTPCDBL`

```rust
const R_390_GOTPCDBL: u32 = 21u32;
```

32 bit PC rel. GOT shifted by 1.

### `R_390_64`

```rust
const R_390_64: u32 = 22u32;
```

Direct 64 bit.

### `R_390_PC64`

```rust
const R_390_PC64: u32 = 23u32;
```

PC relative 64 bit.

### `R_390_GOT64`

```rust
const R_390_GOT64: u32 = 24u32;
```

64 bit GOT offset.

### `R_390_PLT64`

```rust
const R_390_PLT64: u32 = 25u32;
```

64 bit PC relative PLT address.

### `R_390_GOTENT`

```rust
const R_390_GOTENT: u32 = 26u32;
```

32 bit PC rel. to GOT entry >> 1.

### `R_390_GOTOFF16`

```rust
const R_390_GOTOFF16: u32 = 27u32;
```

16 bit offset to GOT.

### `R_390_GOTOFF64`

```rust
const R_390_GOTOFF64: u32 = 28u32;
```

64 bit offset to GOT.

### `R_390_GOTPLT12`

```rust
const R_390_GOTPLT12: u32 = 29u32;
```

12 bit offset to jump slot.

### `R_390_GOTPLT16`

```rust
const R_390_GOTPLT16: u32 = 30u32;
```

16 bit offset to jump slot.

### `R_390_GOTPLT32`

```rust
const R_390_GOTPLT32: u32 = 31u32;
```

32 bit offset to jump slot.

### `R_390_GOTPLT64`

```rust
const R_390_GOTPLT64: u32 = 32u32;
```

64 bit offset to jump slot.

### `R_390_GOTPLTENT`

```rust
const R_390_GOTPLTENT: u32 = 33u32;
```

32 bit rel. offset to jump slot.

### `R_390_PLTOFF16`

```rust
const R_390_PLTOFF16: u32 = 34u32;
```

16 bit offset from GOT to PLT.

### `R_390_PLTOFF32`

```rust
const R_390_PLTOFF32: u32 = 35u32;
```

32 bit offset from GOT to PLT.

### `R_390_PLTOFF64`

```rust
const R_390_PLTOFF64: u32 = 36u32;
```

16 bit offset from GOT to PLT.

### `R_390_TLS_LOAD`

```rust
const R_390_TLS_LOAD: u32 = 37u32;
```

Tag for load insn in TLS code.

### `R_390_TLS_GDCALL`

```rust
const R_390_TLS_GDCALL: u32 = 38u32;
```

Tag for function call in general dynamic TLS code.

### `R_390_TLS_LDCALL`

```rust
const R_390_TLS_LDCALL: u32 = 39u32;
```

Tag for function call in local dynamic TLS code.

### `R_390_TLS_GD32`

```rust
const R_390_TLS_GD32: u32 = 40u32;
```

Direct 32 bit for general dynamic thread local data.

### `R_390_TLS_GD64`

```rust
const R_390_TLS_GD64: u32 = 41u32;
```

Direct 64 bit for general dynamic thread local data.

### `R_390_TLS_GOTIE12`

```rust
const R_390_TLS_GOTIE12: u32 = 42u32;
```

12 bit GOT offset for static TLS block offset.

### `R_390_TLS_GOTIE32`

```rust
const R_390_TLS_GOTIE32: u32 = 43u32;
```

32 bit GOT offset for static TLS block offset.

### `R_390_TLS_GOTIE64`

```rust
const R_390_TLS_GOTIE64: u32 = 44u32;
```

64 bit GOT offset for static TLS block offset.

### `R_390_TLS_LDM32`

```rust
const R_390_TLS_LDM32: u32 = 45u32;
```

Direct 32 bit for local dynamic thread local data in LE code.

### `R_390_TLS_LDM64`

```rust
const R_390_TLS_LDM64: u32 = 46u32;
```

Direct 64 bit for local dynamic thread local data in LE code.

### `R_390_TLS_IE32`

```rust
const R_390_TLS_IE32: u32 = 47u32;
```

32 bit address of GOT entry for negated static TLS block offset.

### `R_390_TLS_IE64`

```rust
const R_390_TLS_IE64: u32 = 48u32;
```

64 bit address of GOT entry for negated static TLS block offset.

### `R_390_TLS_IEENT`

```rust
const R_390_TLS_IEENT: u32 = 49u32;
```

32 bit rel. offset to GOT entry for negated static TLS block offset.

### `R_390_TLS_LE32`

```rust
const R_390_TLS_LE32: u32 = 50u32;
```

32 bit negated offset relative to static TLS block.

### `R_390_TLS_LE64`

```rust
const R_390_TLS_LE64: u32 = 51u32;
```

64 bit negated offset relative to static TLS block.

### `R_390_TLS_LDO32`

```rust
const R_390_TLS_LDO32: u32 = 52u32;
```

32 bit offset relative to TLS block.

### `R_390_TLS_LDO64`

```rust
const R_390_TLS_LDO64: u32 = 53u32;
```

64 bit offset relative to TLS block.

### `R_390_TLS_DTPMOD`

```rust
const R_390_TLS_DTPMOD: u32 = 54u32;
```

ID of module containing symbol.

### `R_390_TLS_DTPOFF`

```rust
const R_390_TLS_DTPOFF: u32 = 55u32;
```

Offset in TLS block.

### `R_390_TLS_TPOFF`

```rust
const R_390_TLS_TPOFF: u32 = 56u32;
```

Negated offset in static TLS block.

### `R_390_20`

```rust
const R_390_20: u32 = 57u32;
```

Direct 20 bit.

### `R_390_GOT20`

```rust
const R_390_GOT20: u32 = 58u32;
```

20 bit GOT offset.

### `R_390_GOTPLT20`

```rust
const R_390_GOTPLT20: u32 = 59u32;
```

20 bit offset to jump slot.

### `R_390_TLS_GOTIE20`

```rust
const R_390_TLS_GOTIE20: u32 = 60u32;
```

20 bit GOT offset for static TLS block offset.

### `R_390_IRELATIVE`

```rust
const R_390_IRELATIVE: u32 = 61u32;
```

STT_GNU_IFUNC relocation.

### `R_CRIS_NONE`

```rust
const R_CRIS_NONE: u32 = 0u32;
```

### `R_CRIS_8`

```rust
const R_CRIS_8: u32 = 1u32;
```

### `R_CRIS_16`

```rust
const R_CRIS_16: u32 = 2u32;
```

### `R_CRIS_32`

```rust
const R_CRIS_32: u32 = 3u32;
```

### `R_CRIS_8_PCREL`

```rust
const R_CRIS_8_PCREL: u32 = 4u32;
```

### `R_CRIS_16_PCREL`

```rust
const R_CRIS_16_PCREL: u32 = 5u32;
```

### `R_CRIS_32_PCREL`

```rust
const R_CRIS_32_PCREL: u32 = 6u32;
```

### `R_CRIS_GNU_VTINHERIT`

```rust
const R_CRIS_GNU_VTINHERIT: u32 = 7u32;
```

### `R_CRIS_GNU_VTENTRY`

```rust
const R_CRIS_GNU_VTENTRY: u32 = 8u32;
```

### `R_CRIS_COPY`

```rust
const R_CRIS_COPY: u32 = 9u32;
```

### `R_CRIS_GLOB_DAT`

```rust
const R_CRIS_GLOB_DAT: u32 = 10u32;
```

### `R_CRIS_JUMP_SLOT`

```rust
const R_CRIS_JUMP_SLOT: u32 = 11u32;
```

### `R_CRIS_RELATIVE`

```rust
const R_CRIS_RELATIVE: u32 = 12u32;
```

### `R_CRIS_16_GOT`

```rust
const R_CRIS_16_GOT: u32 = 13u32;
```

### `R_CRIS_32_GOT`

```rust
const R_CRIS_32_GOT: u32 = 14u32;
```

### `R_CRIS_16_GOTPLT`

```rust
const R_CRIS_16_GOTPLT: u32 = 15u32;
```

### `R_CRIS_32_GOTPLT`

```rust
const R_CRIS_32_GOTPLT: u32 = 16u32;
```

### `R_CRIS_32_GOTREL`

```rust
const R_CRIS_32_GOTREL: u32 = 17u32;
```

### `R_CRIS_32_PLT_GOTREL`

```rust
const R_CRIS_32_PLT_GOTREL: u32 = 18u32;
```

### `R_CRIS_32_PLT_PCREL`

```rust
const R_CRIS_32_PLT_PCREL: u32 = 19u32;
```

### `R_X86_64_NONE`

```rust
const R_X86_64_NONE: u32 = 0u32;
```

No reloc

### `R_X86_64_64`

```rust
const R_X86_64_64: u32 = 1u32;
```

Direct 64 bit

### `R_X86_64_PC32`

```rust
const R_X86_64_PC32: u32 = 2u32;
```

PC relative 32 bit signed

### `R_X86_64_GOT32`

```rust
const R_X86_64_GOT32: u32 = 3u32;
```

32 bit GOT entry

### `R_X86_64_PLT32`

```rust
const R_X86_64_PLT32: u32 = 4u32;
```

32 bit PLT address

### `R_X86_64_COPY`

```rust
const R_X86_64_COPY: u32 = 5u32;
```

Copy symbol at runtime

### `R_X86_64_GLOB_DAT`

```rust
const R_X86_64_GLOB_DAT: u32 = 6u32;
```

Create GOT entry

### `R_X86_64_JUMP_SLOT`

```rust
const R_X86_64_JUMP_SLOT: u32 = 7u32;
```

Create PLT entry

### `R_X86_64_RELATIVE`

```rust
const R_X86_64_RELATIVE: u32 = 8u32;
```

Adjust by program base

### `R_X86_64_GOTPCREL`

```rust
const R_X86_64_GOTPCREL: u32 = 9u32;
```

32 bit signed PC relative offset to GOT

### `R_X86_64_32`

```rust
const R_X86_64_32: u32 = 10u32;
```

Direct 32 bit zero extended

### `R_X86_64_32S`

```rust
const R_X86_64_32S: u32 = 11u32;
```

Direct 32 bit sign extended

### `R_X86_64_16`

```rust
const R_X86_64_16: u32 = 12u32;
```

Direct 16 bit zero extended

### `R_X86_64_PC16`

```rust
const R_X86_64_PC16: u32 = 13u32;
```

16 bit sign extended pc relative

### `R_X86_64_8`

```rust
const R_X86_64_8: u32 = 14u32;
```

Direct 8 bit sign extended

### `R_X86_64_PC8`

```rust
const R_X86_64_PC8: u32 = 15u32;
```

8 bit sign extended pc relative

### `R_X86_64_DTPMOD64`

```rust
const R_X86_64_DTPMOD64: u32 = 16u32;
```

ID of module containing symbol

### `R_X86_64_DTPOFF64`

```rust
const R_X86_64_DTPOFF64: u32 = 17u32;
```

Offset in module's TLS block

### `R_X86_64_TPOFF64`

```rust
const R_X86_64_TPOFF64: u32 = 18u32;
```

Offset in initial TLS block

### `R_X86_64_TLSGD`

```rust
const R_X86_64_TLSGD: u32 = 19u32;
```

32 bit signed PC relative offset to two GOT entries for GD symbol

### `R_X86_64_TLSLD`

```rust
const R_X86_64_TLSLD: u32 = 20u32;
```

32 bit signed PC relative offset to two GOT entries for LD symbol

### `R_X86_64_DTPOFF32`

```rust
const R_X86_64_DTPOFF32: u32 = 21u32;
```

Offset in TLS block

### `R_X86_64_GOTTPOFF`

```rust
const R_X86_64_GOTTPOFF: u32 = 22u32;
```

32 bit signed PC relative offset to GOT entry for IE symbol

### `R_X86_64_TPOFF32`

```rust
const R_X86_64_TPOFF32: u32 = 23u32;
```

Offset in initial TLS block

### `R_X86_64_PC64`

```rust
const R_X86_64_PC64: u32 = 24u32;
```

PC relative 64 bit

### `R_X86_64_GOTOFF64`

```rust
const R_X86_64_GOTOFF64: u32 = 25u32;
```

64 bit offset to GOT

### `R_X86_64_GOTPC32`

```rust
const R_X86_64_GOTPC32: u32 = 26u32;
```

32 bit signed pc relative offset to GOT

### `R_X86_64_GOT64`

```rust
const R_X86_64_GOT64: u32 = 27u32;
```

64-bit GOT entry offset

### `R_X86_64_GOTPCREL64`

```rust
const R_X86_64_GOTPCREL64: u32 = 28u32;
```

64-bit PC relative offset to GOT entry

### `R_X86_64_GOTPC64`

```rust
const R_X86_64_GOTPC64: u32 = 29u32;
```

64-bit PC relative offset to GOT

### `R_X86_64_GOTPLT64`

```rust
const R_X86_64_GOTPLT64: u32 = 30u32;
```

like GOT64, says PLT entry needed

### `R_X86_64_PLTOFF64`

```rust
const R_X86_64_PLTOFF64: u32 = 31u32;
```

64-bit GOT relative offset to PLT entry

### `R_X86_64_SIZE32`

```rust
const R_X86_64_SIZE32: u32 = 32u32;
```

Size of symbol plus 32-bit addend

### `R_X86_64_SIZE64`

```rust
const R_X86_64_SIZE64: u32 = 33u32;
```

Size of symbol plus 64-bit addend

### `R_X86_64_GOTPC32_TLSDESC`

```rust
const R_X86_64_GOTPC32_TLSDESC: u32 = 34u32;
```

GOT offset for TLS descriptor.

### `R_X86_64_TLSDESC_CALL`

```rust
const R_X86_64_TLSDESC_CALL: u32 = 35u32;
```

Marker for call through TLS descriptor.

### `R_X86_64_TLSDESC`

```rust
const R_X86_64_TLSDESC: u32 = 36u32;
```

TLS descriptor.

### `R_X86_64_IRELATIVE`

```rust
const R_X86_64_IRELATIVE: u32 = 37u32;
```

Adjust indirectly by program base

### `R_X86_64_RELATIVE64`

```rust
const R_X86_64_RELATIVE64: u32 = 38u32;
```

64-bit adjust by program base

### `R_X86_64_GOTPCRELX`

```rust
const R_X86_64_GOTPCRELX: u32 = 41u32;
```

Load from 32 bit signed pc relative offset to GOT entry without REX prefix, relaxable.

### `R_X86_64_REX_GOTPCRELX`

```rust
const R_X86_64_REX_GOTPCRELX: u32 = 42u32;
```

Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable.

### `SHT_X86_64_UNWIND`

```rust
const SHT_X86_64_UNWIND: u32 = 1_879_048_193u32;
```

Unwind information.

### `R_MN10300_NONE`

```rust
const R_MN10300_NONE: u32 = 0u32;
```

No reloc.

### `R_MN10300_32`

```rust
const R_MN10300_32: u32 = 1u32;
```

Direct 32 bit.

### `R_MN10300_16`

```rust
const R_MN10300_16: u32 = 2u32;
```

Direct 16 bit.

### `R_MN10300_8`

```rust
const R_MN10300_8: u32 = 3u32;
```

Direct 8 bit.

### `R_MN10300_PCREL32`

```rust
const R_MN10300_PCREL32: u32 = 4u32;
```

PC-relative 32-bit.

### `R_MN10300_PCREL16`

```rust
const R_MN10300_PCREL16: u32 = 5u32;
```

PC-relative 16-bit signed.

### `R_MN10300_PCREL8`

```rust
const R_MN10300_PCREL8: u32 = 6u32;
```

PC-relative 8-bit signed.

### `R_MN10300_GNU_VTINHERIT`

```rust
const R_MN10300_GNU_VTINHERIT: u32 = 7u32;
```

Ancient C++ vtable garbage...

### `R_MN10300_GNU_VTENTRY`

```rust
const R_MN10300_GNU_VTENTRY: u32 = 8u32;
```

... collection annotation.

### `R_MN10300_24`

```rust
const R_MN10300_24: u32 = 9u32;
```

Direct 24 bit.

### `R_MN10300_GOTPC32`

```rust
const R_MN10300_GOTPC32: u32 = 10u32;
```

32-bit PCrel offset to GOT.

### `R_MN10300_GOTPC16`

```rust
const R_MN10300_GOTPC16: u32 = 11u32;
```

16-bit PCrel offset to GOT.

### `R_MN10300_GOTOFF32`

```rust
const R_MN10300_GOTOFF32: u32 = 12u32;
```

32-bit offset from GOT.

### `R_MN10300_GOTOFF24`

```rust
const R_MN10300_GOTOFF24: u32 = 13u32;
```

24-bit offset from GOT.

### `R_MN10300_GOTOFF16`

```rust
const R_MN10300_GOTOFF16: u32 = 14u32;
```

16-bit offset from GOT.

### `R_MN10300_PLT32`

```rust
const R_MN10300_PLT32: u32 = 15u32;
```

32-bit PCrel to PLT entry.

### `R_MN10300_PLT16`

```rust
const R_MN10300_PLT16: u32 = 16u32;
```

16-bit PCrel to PLT entry.

### `R_MN10300_GOT32`

```rust
const R_MN10300_GOT32: u32 = 17u32;
```

32-bit offset to GOT entry.

### `R_MN10300_GOT24`

```rust
const R_MN10300_GOT24: u32 = 18u32;
```

24-bit offset to GOT entry.

### `R_MN10300_GOT16`

```rust
const R_MN10300_GOT16: u32 = 19u32;
```

16-bit offset to GOT entry.

### `R_MN10300_COPY`

```rust
const R_MN10300_COPY: u32 = 20u32;
```

Copy symbol at runtime.

### `R_MN10300_GLOB_DAT`

```rust
const R_MN10300_GLOB_DAT: u32 = 21u32;
```

Create GOT entry.

### `R_MN10300_JMP_SLOT`

```rust
const R_MN10300_JMP_SLOT: u32 = 22u32;
```

Create PLT entry.

### `R_MN10300_RELATIVE`

```rust
const R_MN10300_RELATIVE: u32 = 23u32;
```

Adjust by program base.

### `R_MN10300_TLS_GD`

```rust
const R_MN10300_TLS_GD: u32 = 24u32;
```

32-bit offset for global dynamic.

### `R_MN10300_TLS_LD`

```rust
const R_MN10300_TLS_LD: u32 = 25u32;
```

32-bit offset for local dynamic.

### `R_MN10300_TLS_LDO`

```rust
const R_MN10300_TLS_LDO: u32 = 26u32;
```

Module-relative offset.

### `R_MN10300_TLS_GOTIE`

```rust
const R_MN10300_TLS_GOTIE: u32 = 27u32;
```

GOT offset for static TLS block offset.

### `R_MN10300_TLS_IE`

```rust
const R_MN10300_TLS_IE: u32 = 28u32;
```

GOT address for static TLS block offset.

### `R_MN10300_TLS_LE`

```rust
const R_MN10300_TLS_LE: u32 = 29u32;
```

Offset relative to static TLS block.

### `R_MN10300_TLS_DTPMOD`

```rust
const R_MN10300_TLS_DTPMOD: u32 = 30u32;
```

ID of module containing symbol.

### `R_MN10300_TLS_DTPOFF`

```rust
const R_MN10300_TLS_DTPOFF: u32 = 31u32;
```

Offset in module TLS block.

### `R_MN10300_TLS_TPOFF`

```rust
const R_MN10300_TLS_TPOFF: u32 = 32u32;
```

Offset in static TLS block.

### `R_MN10300_SYM_DIFF`

```rust
const R_MN10300_SYM_DIFF: u32 = 33u32;
```

Adjustment for next reloc as needed by linker relaxation.

### `R_MN10300_ALIGN`

```rust
const R_MN10300_ALIGN: u32 = 34u32;
```

Alignment requirement for linker relaxation.

### `R_M32R_NONE`

```rust
const R_M32R_NONE: u32 = 0u32;
```

No reloc.

### `R_M32R_16`

```rust
const R_M32R_16: u32 = 1u32;
```

Direct 16 bit.

### `R_M32R_32`

```rust
const R_M32R_32: u32 = 2u32;
```

Direct 32 bit.

### `R_M32R_24`

```rust
const R_M32R_24: u32 = 3u32;
```

Direct 24 bit.

### `R_M32R_10_PCREL`

```rust
const R_M32R_10_PCREL: u32 = 4u32;
```

PC relative 10 bit shifted.

### `R_M32R_18_PCREL`

```rust
const R_M32R_18_PCREL: u32 = 5u32;
```

PC relative 18 bit shifted.

### `R_M32R_26_PCREL`

```rust
const R_M32R_26_PCREL: u32 = 6u32;
```

PC relative 26 bit shifted.

### `R_M32R_HI16_ULO`

```rust
const R_M32R_HI16_ULO: u32 = 7u32;
```

High 16 bit with unsigned low.

### `R_M32R_HI16_SLO`

```rust
const R_M32R_HI16_SLO: u32 = 8u32;
```

High 16 bit with signed low.

### `R_M32R_LO16`

```rust
const R_M32R_LO16: u32 = 9u32;
```

Low 16 bit.

### `R_M32R_SDA16`

```rust
const R_M32R_SDA16: u32 = 10u32;
```

16 bit offset in SDA.

### `R_M32R_GNU_VTINHERIT`

```rust
const R_M32R_GNU_VTINHERIT: u32 = 11u32;
```

### `R_M32R_GNU_VTENTRY`

```rust
const R_M32R_GNU_VTENTRY: u32 = 12u32;
```

### `R_M32R_16_RELA`

```rust
const R_M32R_16_RELA: u32 = 33u32;
```

Direct 16 bit.

### `R_M32R_32_RELA`

```rust
const R_M32R_32_RELA: u32 = 34u32;
```

Direct 32 bit.

### `R_M32R_24_RELA`

```rust
const R_M32R_24_RELA: u32 = 35u32;
```

Direct 24 bit.

### `R_M32R_10_PCREL_RELA`

```rust
const R_M32R_10_PCREL_RELA: u32 = 36u32;
```

PC relative 10 bit shifted.

### `R_M32R_18_PCREL_RELA`

```rust
const R_M32R_18_PCREL_RELA: u32 = 37u32;
```

PC relative 18 bit shifted.

### `R_M32R_26_PCREL_RELA`

```rust
const R_M32R_26_PCREL_RELA: u32 = 38u32;
```

PC relative 26 bit shifted.

### `R_M32R_HI16_ULO_RELA`

```rust
const R_M32R_HI16_ULO_RELA: u32 = 39u32;
```

High 16 bit with unsigned low

### `R_M32R_HI16_SLO_RELA`

```rust
const R_M32R_HI16_SLO_RELA: u32 = 40u32;
```

High 16 bit with signed low

### `R_M32R_LO16_RELA`

```rust
const R_M32R_LO16_RELA: u32 = 41u32;
```

Low 16 bit

### `R_M32R_SDA16_RELA`

```rust
const R_M32R_SDA16_RELA: u32 = 42u32;
```

16 bit offset in SDA

### `R_M32R_RELA_GNU_VTINHERIT`

```rust
const R_M32R_RELA_GNU_VTINHERIT: u32 = 43u32;
```

### `R_M32R_RELA_GNU_VTENTRY`

```rust
const R_M32R_RELA_GNU_VTENTRY: u32 = 44u32;
```

### `R_M32R_REL32`

```rust
const R_M32R_REL32: u32 = 45u32;
```

PC relative 32 bit.

### `R_M32R_GOT24`

```rust
const R_M32R_GOT24: u32 = 48u32;
```

24 bit GOT entry

### `R_M32R_26_PLTREL`

```rust
const R_M32R_26_PLTREL: u32 = 49u32;
```

26 bit PC relative to PLT shifted

### `R_M32R_COPY`

```rust
const R_M32R_COPY: u32 = 50u32;
```

Copy symbol at runtime

### `R_M32R_GLOB_DAT`

```rust
const R_M32R_GLOB_DAT: u32 = 51u32;
```

Create GOT entry

### `R_M32R_JMP_SLOT`

```rust
const R_M32R_JMP_SLOT: u32 = 52u32;
```

Create PLT entry

### `R_M32R_RELATIVE`

```rust
const R_M32R_RELATIVE: u32 = 53u32;
```

Adjust by program base

### `R_M32R_GOTOFF`

```rust
const R_M32R_GOTOFF: u32 = 54u32;
```

24 bit offset to GOT

### `R_M32R_GOTPC24`

```rust
const R_M32R_GOTPC24: u32 = 55u32;
```

24 bit PC relative offset to GOT

### `R_M32R_GOT16_HI_ULO`

```rust
const R_M32R_GOT16_HI_ULO: u32 = 56u32;
```

High 16 bit GOT entry with unsigned low

### `R_M32R_GOT16_HI_SLO`

```rust
const R_M32R_GOT16_HI_SLO: u32 = 57u32;
```

High 16 bit GOT entry with signed low

### `R_M32R_GOT16_LO`

```rust
const R_M32R_GOT16_LO: u32 = 58u32;
```

Low 16 bit GOT entry

### `R_M32R_GOTPC_HI_ULO`

```rust
const R_M32R_GOTPC_HI_ULO: u32 = 59u32;
```

High 16 bit PC relative offset to GOT with unsigned low

### `R_M32R_GOTPC_HI_SLO`

```rust
const R_M32R_GOTPC_HI_SLO: u32 = 60u32;
```

High 16 bit PC relative offset to GOT with signed low

### `R_M32R_GOTPC_LO`

```rust
const R_M32R_GOTPC_LO: u32 = 61u32;
```

Low 16 bit PC relative offset to GOT

### `R_M32R_GOTOFF_HI_ULO`

```rust
const R_M32R_GOTOFF_HI_ULO: u32 = 62u32;
```

High 16 bit offset to GOT with unsigned low

### `R_M32R_GOTOFF_HI_SLO`

```rust
const R_M32R_GOTOFF_HI_SLO: u32 = 63u32;
```

High 16 bit offset to GOT with signed low

### `R_M32R_GOTOFF_LO`

```rust
const R_M32R_GOTOFF_LO: u32 = 64u32;
```

Low 16 bit offset to GOT

### `R_M32R_NUM`

```rust
const R_M32R_NUM: u32 = 256u32;
```

Keep this the last entry.

### `R_MICROBLAZE_NONE`

```rust
const R_MICROBLAZE_NONE: u32 = 0u32;
```

No reloc.

### `R_MICROBLAZE_32`

```rust
const R_MICROBLAZE_32: u32 = 1u32;
```

Direct 32 bit.

### `R_MICROBLAZE_32_PCREL`

```rust
const R_MICROBLAZE_32_PCREL: u32 = 2u32;
```

PC relative 32 bit.

### `R_MICROBLAZE_64_PCREL`

```rust
const R_MICROBLAZE_64_PCREL: u32 = 3u32;
```

PC relative 64 bit.

### `R_MICROBLAZE_32_PCREL_LO`

```rust
const R_MICROBLAZE_32_PCREL_LO: u32 = 4u32;
```

Low 16 bits of PCREL32.

### `R_MICROBLAZE_64`

```rust
const R_MICROBLAZE_64: u32 = 5u32;
```

Direct 64 bit.

### `R_MICROBLAZE_32_LO`

```rust
const R_MICROBLAZE_32_LO: u32 = 6u32;
```

Low 16 bit.

### `R_MICROBLAZE_SRO32`

```rust
const R_MICROBLAZE_SRO32: u32 = 7u32;
```

Read-only small data area.

### `R_MICROBLAZE_SRW32`

```rust
const R_MICROBLAZE_SRW32: u32 = 8u32;
```

Read-write small data area.

### `R_MICROBLAZE_64_NONE`

```rust
const R_MICROBLAZE_64_NONE: u32 = 9u32;
```

No reloc.

### `R_MICROBLAZE_32_SYM_OP_SYM`

```rust
const R_MICROBLAZE_32_SYM_OP_SYM: u32 = 10u32;
```

Symbol Op Symbol relocation.

### `R_MICROBLAZE_GNU_VTINHERIT`

```rust
const R_MICROBLAZE_GNU_VTINHERIT: u32 = 11u32;
```

GNU C++ vtable hierarchy.

### `R_MICROBLAZE_GNU_VTENTRY`

```rust
const R_MICROBLAZE_GNU_VTENTRY: u32 = 12u32;
```

GNU C++ vtable member usage.

### `R_MICROBLAZE_GOTPC_64`

```rust
const R_MICROBLAZE_GOTPC_64: u32 = 13u32;
```

PC-relative GOT offset.

### `R_MICROBLAZE_GOT_64`

```rust
const R_MICROBLAZE_GOT_64: u32 = 14u32;
```

GOT entry offset.

### `R_MICROBLAZE_PLT_64`

```rust
const R_MICROBLAZE_PLT_64: u32 = 15u32;
```

PLT offset (PC-relative).

### `R_MICROBLAZE_REL`

```rust
const R_MICROBLAZE_REL: u32 = 16u32;
```

Adjust by program base.

### `R_MICROBLAZE_JUMP_SLOT`

```rust
const R_MICROBLAZE_JUMP_SLOT: u32 = 17u32;
```

Create PLT entry.

### `R_MICROBLAZE_GLOB_DAT`

```rust
const R_MICROBLAZE_GLOB_DAT: u32 = 18u32;
```

Create GOT entry.

### `R_MICROBLAZE_GOTOFF_64`

```rust
const R_MICROBLAZE_GOTOFF_64: u32 = 19u32;
```

64 bit offset to GOT.

### `R_MICROBLAZE_GOTOFF_32`

```rust
const R_MICROBLAZE_GOTOFF_32: u32 = 20u32;
```

32 bit offset to GOT.

### `R_MICROBLAZE_COPY`

```rust
const R_MICROBLAZE_COPY: u32 = 21u32;
```

Runtime copy.

### `R_MICROBLAZE_TLS`

```rust
const R_MICROBLAZE_TLS: u32 = 22u32;
```

TLS Reloc.

### `R_MICROBLAZE_TLSGD`

```rust
const R_MICROBLAZE_TLSGD: u32 = 23u32;
```

TLS General Dynamic.

### `R_MICROBLAZE_TLSLD`

```rust
const R_MICROBLAZE_TLSLD: u32 = 24u32;
```

TLS Local Dynamic.

### `R_MICROBLAZE_TLSDTPMOD32`

```rust
const R_MICROBLAZE_TLSDTPMOD32: u32 = 25u32;
```

TLS Module ID.

### `R_MICROBLAZE_TLSDTPREL32`

```rust
const R_MICROBLAZE_TLSDTPREL32: u32 = 26u32;
```

TLS Offset Within TLS Block.

### `R_MICROBLAZE_TLSDTPREL64`

```rust
const R_MICROBLAZE_TLSDTPREL64: u32 = 27u32;
```

TLS Offset Within TLS Block.

### `R_MICROBLAZE_TLSGOTTPREL32`

```rust
const R_MICROBLAZE_TLSGOTTPREL32: u32 = 28u32;
```

TLS Offset From Thread Pointer.

### `R_MICROBLAZE_TLSTPREL32`

```rust
const R_MICROBLAZE_TLSTPREL32: u32 = 29u32;
```

TLS Offset From Thread Pointer.

### `DT_NIOS2_GP`

```rust
const DT_NIOS2_GP: u32 = 1_879_048_194u32;
```

Address of _gp.

### `R_NIOS2_NONE`

```rust
const R_NIOS2_NONE: u32 = 0u32;
```

No reloc.

### `R_NIOS2_S16`

```rust
const R_NIOS2_S16: u32 = 1u32;
```

Direct signed 16 bit.

### `R_NIOS2_U16`

```rust
const R_NIOS2_U16: u32 = 2u32;
```

Direct unsigned 16 bit.

### `R_NIOS2_PCREL16`

```rust
const R_NIOS2_PCREL16: u32 = 3u32;
```

PC relative 16 bit.

### `R_NIOS2_CALL26`

```rust
const R_NIOS2_CALL26: u32 = 4u32;
```

Direct call.

### `R_NIOS2_IMM5`

```rust
const R_NIOS2_IMM5: u32 = 5u32;
```

5 bit constant expression.

### `R_NIOS2_CACHE_OPX`

```rust
const R_NIOS2_CACHE_OPX: u32 = 6u32;
```

5 bit expression, shift 22.

### `R_NIOS2_IMM6`

```rust
const R_NIOS2_IMM6: u32 = 7u32;
```

6 bit constant expression.

### `R_NIOS2_IMM8`

```rust
const R_NIOS2_IMM8: u32 = 8u32;
```

8 bit constant expression.

### `R_NIOS2_HI16`

```rust
const R_NIOS2_HI16: u32 = 9u32;
```

High 16 bit.

### `R_NIOS2_LO16`

```rust
const R_NIOS2_LO16: u32 = 10u32;
```

Low 16 bit.

### `R_NIOS2_HIADJ16`

```rust
const R_NIOS2_HIADJ16: u32 = 11u32;
```

High 16 bit, adjusted.

### `R_NIOS2_BFD_RELOC_32`

```rust
const R_NIOS2_BFD_RELOC_32: u32 = 12u32;
```

32 bit symbol value + addend.

### `R_NIOS2_BFD_RELOC_16`

```rust
const R_NIOS2_BFD_RELOC_16: u32 = 13u32;
```

16 bit symbol value + addend.

### `R_NIOS2_BFD_RELOC_8`

```rust
const R_NIOS2_BFD_RELOC_8: u32 = 14u32;
```

8 bit symbol value + addend.

### `R_NIOS2_GPREL`

```rust
const R_NIOS2_GPREL: u32 = 15u32;
```

16 bit GP pointer offset.

### `R_NIOS2_GNU_VTINHERIT`

```rust
const R_NIOS2_GNU_VTINHERIT: u32 = 16u32;
```

GNU C++ vtable hierarchy.

### `R_NIOS2_GNU_VTENTRY`

```rust
const R_NIOS2_GNU_VTENTRY: u32 = 17u32;
```

GNU C++ vtable member usage.

### `R_NIOS2_UJMP`

```rust
const R_NIOS2_UJMP: u32 = 18u32;
```

Unconditional branch.

### `R_NIOS2_CJMP`

```rust
const R_NIOS2_CJMP: u32 = 19u32;
```

Conditional branch.

### `R_NIOS2_CALLR`

```rust
const R_NIOS2_CALLR: u32 = 20u32;
```

Indirect call through register.

### `R_NIOS2_ALIGN`

```rust
const R_NIOS2_ALIGN: u32 = 21u32;
```

Alignment requirement for linker relaxation.

### `R_NIOS2_GOT16`

```rust
const R_NIOS2_GOT16: u32 = 22u32;
```

16 bit GOT entry.

### `R_NIOS2_CALL16`

```rust
const R_NIOS2_CALL16: u32 = 23u32;
```

16 bit GOT entry for function.

### `R_NIOS2_GOTOFF_LO`

```rust
const R_NIOS2_GOTOFF_LO: u32 = 24u32;
```

%lo of offset to GOT pointer.

### `R_NIOS2_GOTOFF_HA`

```rust
const R_NIOS2_GOTOFF_HA: u32 = 25u32;
```

%hiadj of offset to GOT pointer.

### `R_NIOS2_PCREL_LO`

```rust
const R_NIOS2_PCREL_LO: u32 = 26u32;
```

%lo of PC relative offset.

### `R_NIOS2_PCREL_HA`

```rust
const R_NIOS2_PCREL_HA: u32 = 27u32;
```

%hiadj of PC relative offset.

### `R_NIOS2_TLS_GD16`

```rust
const R_NIOS2_TLS_GD16: u32 = 28u32;
```

16 bit GOT offset for TLS GD.

### `R_NIOS2_TLS_LDM16`

```rust
const R_NIOS2_TLS_LDM16: u32 = 29u32;
```

16 bit GOT offset for TLS LDM.

### `R_NIOS2_TLS_LDO16`

```rust
const R_NIOS2_TLS_LDO16: u32 = 30u32;
```

16 bit module relative offset.

### `R_NIOS2_TLS_IE16`

```rust
const R_NIOS2_TLS_IE16: u32 = 31u32;
```

16 bit GOT offset for TLS IE.

### `R_NIOS2_TLS_LE16`

```rust
const R_NIOS2_TLS_LE16: u32 = 32u32;
```

16 bit LE TP-relative offset.

### `R_NIOS2_TLS_DTPMOD`

```rust
const R_NIOS2_TLS_DTPMOD: u32 = 33u32;
```

Module number.

### `R_NIOS2_TLS_DTPREL`

```rust
const R_NIOS2_TLS_DTPREL: u32 = 34u32;
```

Module-relative offset.

### `R_NIOS2_TLS_TPREL`

```rust
const R_NIOS2_TLS_TPREL: u32 = 35u32;
```

TP-relative offset.

### `R_NIOS2_COPY`

```rust
const R_NIOS2_COPY: u32 = 36u32;
```

Copy symbol at runtime.

### `R_NIOS2_GLOB_DAT`

```rust
const R_NIOS2_GLOB_DAT: u32 = 37u32;
```

Create GOT entry.

### `R_NIOS2_JUMP_SLOT`

```rust
const R_NIOS2_JUMP_SLOT: u32 = 38u32;
```

Create PLT entry.

### `R_NIOS2_RELATIVE`

```rust
const R_NIOS2_RELATIVE: u32 = 39u32;
```

Adjust by program base.

### `R_NIOS2_GOTOFF`

```rust
const R_NIOS2_GOTOFF: u32 = 40u32;
```

16 bit offset to GOT pointer.

### `R_NIOS2_CALL26_NOAT`

```rust
const R_NIOS2_CALL26_NOAT: u32 = 41u32;
```

Direct call in .noat section.

### `R_NIOS2_GOT_LO`

```rust
const R_NIOS2_GOT_LO: u32 = 42u32;
```

%lo() of GOT entry.

### `R_NIOS2_GOT_HA`

```rust
const R_NIOS2_GOT_HA: u32 = 43u32;
```

%hiadj() of GOT entry.

### `R_NIOS2_CALL_LO`

```rust
const R_NIOS2_CALL_LO: u32 = 44u32;
```

%lo() of function GOT entry.

### `R_NIOS2_CALL_HA`

```rust
const R_NIOS2_CALL_HA: u32 = 45u32;
```

%hiadj() of function GOT entry.

### `R_TILEPRO_NONE`

```rust
const R_TILEPRO_NONE: u32 = 0u32;
```

No reloc

### `R_TILEPRO_32`

```rust
const R_TILEPRO_32: u32 = 1u32;
```

Direct 32 bit

### `R_TILEPRO_16`

```rust
const R_TILEPRO_16: u32 = 2u32;
```

Direct 16 bit

### `R_TILEPRO_8`

```rust
const R_TILEPRO_8: u32 = 3u32;
```

Direct 8 bit

### `R_TILEPRO_32_PCREL`

```rust
const R_TILEPRO_32_PCREL: u32 = 4u32;
```

PC relative 32 bit

### `R_TILEPRO_16_PCREL`

```rust
const R_TILEPRO_16_PCREL: u32 = 5u32;
```

PC relative 16 bit

### `R_TILEPRO_8_PCREL`

```rust
const R_TILEPRO_8_PCREL: u32 = 6u32;
```

PC relative 8 bit

### `R_TILEPRO_LO16`

```rust
const R_TILEPRO_LO16: u32 = 7u32;
```

Low 16 bit

### `R_TILEPRO_HI16`

```rust
const R_TILEPRO_HI16: u32 = 8u32;
```

High 16 bit

### `R_TILEPRO_HA16`

```rust
const R_TILEPRO_HA16: u32 = 9u32;
```

High 16 bit, adjusted

### `R_TILEPRO_COPY`

```rust
const R_TILEPRO_COPY: u32 = 10u32;
```

Copy relocation

### `R_TILEPRO_GLOB_DAT`

```rust
const R_TILEPRO_GLOB_DAT: u32 = 11u32;
```

Create GOT entry

### `R_TILEPRO_JMP_SLOT`

```rust
const R_TILEPRO_JMP_SLOT: u32 = 12u32;
```

Create PLT entry

### `R_TILEPRO_RELATIVE`

```rust
const R_TILEPRO_RELATIVE: u32 = 13u32;
```

Adjust by program base

### `R_TILEPRO_BROFF_X1`

```rust
const R_TILEPRO_BROFF_X1: u32 = 14u32;
```

X1 pipe branch offset

### `R_TILEPRO_JOFFLONG_X1`

```rust
const R_TILEPRO_JOFFLONG_X1: u32 = 15u32;
```

X1 pipe jump offset

### `R_TILEPRO_JOFFLONG_X1_PLT`

```rust
const R_TILEPRO_JOFFLONG_X1_PLT: u32 = 16u32;
```

X1 pipe jump offset to PLT

### `R_TILEPRO_IMM8_X0`

```rust
const R_TILEPRO_IMM8_X0: u32 = 17u32;
```

X0 pipe 8-bit

### `R_TILEPRO_IMM8_Y0`

```rust
const R_TILEPRO_IMM8_Y0: u32 = 18u32;
```

Y0 pipe 8-bit

### `R_TILEPRO_IMM8_X1`

```rust
const R_TILEPRO_IMM8_X1: u32 = 19u32;
```

X1 pipe 8-bit

### `R_TILEPRO_IMM8_Y1`

```rust
const R_TILEPRO_IMM8_Y1: u32 = 20u32;
```

Y1 pipe 8-bit

### `R_TILEPRO_MT_IMM15_X1`

```rust
const R_TILEPRO_MT_IMM15_X1: u32 = 21u32;
```

X1 pipe mtspr

### `R_TILEPRO_MF_IMM15_X1`

```rust
const R_TILEPRO_MF_IMM15_X1: u32 = 22u32;
```

X1 pipe mfspr

### `R_TILEPRO_IMM16_X0`

```rust
const R_TILEPRO_IMM16_X0: u32 = 23u32;
```

X0 pipe 16-bit

### `R_TILEPRO_IMM16_X1`

```rust
const R_TILEPRO_IMM16_X1: u32 = 24u32;
```

X1 pipe 16-bit

### `R_TILEPRO_IMM16_X0_LO`

```rust
const R_TILEPRO_IMM16_X0_LO: u32 = 25u32;
```

X0 pipe low 16-bit

### `R_TILEPRO_IMM16_X1_LO`

```rust
const R_TILEPRO_IMM16_X1_LO: u32 = 26u32;
```

X1 pipe low 16-bit

### `R_TILEPRO_IMM16_X0_HI`

```rust
const R_TILEPRO_IMM16_X0_HI: u32 = 27u32;
```

X0 pipe high 16-bit

### `R_TILEPRO_IMM16_X1_HI`

```rust
const R_TILEPRO_IMM16_X1_HI: u32 = 28u32;
```

X1 pipe high 16-bit

### `R_TILEPRO_IMM16_X0_HA`

```rust
const R_TILEPRO_IMM16_X0_HA: u32 = 29u32;
```

X0 pipe high 16-bit, adjusted

### `R_TILEPRO_IMM16_X1_HA`

```rust
const R_TILEPRO_IMM16_X1_HA: u32 = 30u32;
```

X1 pipe high 16-bit, adjusted

### `R_TILEPRO_IMM16_X0_PCREL`

```rust
const R_TILEPRO_IMM16_X0_PCREL: u32 = 31u32;
```

X0 pipe PC relative 16 bit

### `R_TILEPRO_IMM16_X1_PCREL`

```rust
const R_TILEPRO_IMM16_X1_PCREL: u32 = 32u32;
```

X1 pipe PC relative 16 bit

### `R_TILEPRO_IMM16_X0_LO_PCREL`

```rust
const R_TILEPRO_IMM16_X0_LO_PCREL: u32 = 33u32;
```

X0 pipe PC relative low 16 bit

### `R_TILEPRO_IMM16_X1_LO_PCREL`

```rust
const R_TILEPRO_IMM16_X1_LO_PCREL: u32 = 34u32;
```

X1 pipe PC relative low 16 bit

### `R_TILEPRO_IMM16_X0_HI_PCREL`

```rust
const R_TILEPRO_IMM16_X0_HI_PCREL: u32 = 35u32;
```

X0 pipe PC relative high 16 bit

### `R_TILEPRO_IMM16_X1_HI_PCREL`

```rust
const R_TILEPRO_IMM16_X1_HI_PCREL: u32 = 36u32;
```

X1 pipe PC relative high 16 bit

### `R_TILEPRO_IMM16_X0_HA_PCREL`

```rust
const R_TILEPRO_IMM16_X0_HA_PCREL: u32 = 37u32;
```

X0 pipe PC relative ha() 16 bit

### `R_TILEPRO_IMM16_X1_HA_PCREL`

```rust
const R_TILEPRO_IMM16_X1_HA_PCREL: u32 = 38u32;
```

X1 pipe PC relative ha() 16 bit

### `R_TILEPRO_IMM16_X0_GOT`

```rust
const R_TILEPRO_IMM16_X0_GOT: u32 = 39u32;
```

X0 pipe 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT`

```rust
const R_TILEPRO_IMM16_X1_GOT: u32 = 40u32;
```

X1 pipe 16-bit GOT offset

### `R_TILEPRO_IMM16_X0_GOT_LO`

```rust
const R_TILEPRO_IMM16_X0_GOT_LO: u32 = 41u32;
```

X0 pipe low 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT_LO`

```rust
const R_TILEPRO_IMM16_X1_GOT_LO: u32 = 42u32;
```

X1 pipe low 16-bit GOT offset

### `R_TILEPRO_IMM16_X0_GOT_HI`

```rust
const R_TILEPRO_IMM16_X0_GOT_HI: u32 = 43u32;
```

X0 pipe high 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT_HI`

```rust
const R_TILEPRO_IMM16_X1_GOT_HI: u32 = 44u32;
```

X1 pipe high 16-bit GOT offset

### `R_TILEPRO_IMM16_X0_GOT_HA`

```rust
const R_TILEPRO_IMM16_X0_GOT_HA: u32 = 45u32;
```

X0 pipe ha() 16-bit GOT offset

### `R_TILEPRO_IMM16_X1_GOT_HA`

```rust
const R_TILEPRO_IMM16_X1_GOT_HA: u32 = 46u32;
```

X1 pipe ha() 16-bit GOT offset

### `R_TILEPRO_MMSTART_X0`

```rust
const R_TILEPRO_MMSTART_X0: u32 = 47u32;
```

X0 pipe mm "start"

### `R_TILEPRO_MMEND_X0`

```rust
const R_TILEPRO_MMEND_X0: u32 = 48u32;
```

X0 pipe mm "end"

### `R_TILEPRO_MMSTART_X1`

```rust
const R_TILEPRO_MMSTART_X1: u32 = 49u32;
```

X1 pipe mm "start"

### `R_TILEPRO_MMEND_X1`

```rust
const R_TILEPRO_MMEND_X1: u32 = 50u32;
```

X1 pipe mm "end"

### `R_TILEPRO_SHAMT_X0`

```rust
const R_TILEPRO_SHAMT_X0: u32 = 51u32;
```

X0 pipe shift amount

### `R_TILEPRO_SHAMT_X1`

```rust
const R_TILEPRO_SHAMT_X1: u32 = 52u32;
```

X1 pipe shift amount

### `R_TILEPRO_SHAMT_Y0`

```rust
const R_TILEPRO_SHAMT_Y0: u32 = 53u32;
```

Y0 pipe shift amount

### `R_TILEPRO_SHAMT_Y1`

```rust
const R_TILEPRO_SHAMT_Y1: u32 = 54u32;
```

Y1 pipe shift amount

### `R_TILEPRO_DEST_IMM8_X1`

```rust
const R_TILEPRO_DEST_IMM8_X1: u32 = 55u32;
```

X1 pipe destination 8-bit

### `R_TILEPRO_TLS_GD_CALL`

```rust
const R_TILEPRO_TLS_GD_CALL: u32 = 60u32;
```

"jal" for TLS GD

### `R_TILEPRO_IMM8_X0_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_X0_TLS_GD_ADD: u32 = 61u32;
```

X0 pipe "addi" for TLS GD

### `R_TILEPRO_IMM8_X1_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_X1_TLS_GD_ADD: u32 = 62u32;
```

X1 pipe "addi" for TLS GD

### `R_TILEPRO_IMM8_Y0_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_Y0_TLS_GD_ADD: u32 = 63u32;
```

Y0 pipe "addi" for TLS GD

### `R_TILEPRO_IMM8_Y1_TLS_GD_ADD`

```rust
const R_TILEPRO_IMM8_Y1_TLS_GD_ADD: u32 = 64u32;
```

Y1 pipe "addi" for TLS GD

### `R_TILEPRO_TLS_IE_LOAD`

```rust
const R_TILEPRO_TLS_IE_LOAD: u32 = 65u32;
```

"lw_tls" for TLS IE

### `R_TILEPRO_IMM16_X0_TLS_GD`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD: u32 = 66u32;
```

X0 pipe 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD: u32 = 67u32;
```

X1 pipe 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_GD_LO`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD_LO: u32 = 68u32;
```

X0 pipe low 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD_LO`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD_LO: u32 = 69u32;
```

X1 pipe low 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_GD_HI`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD_HI: u32 = 70u32;
```

X0 pipe high 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD_HI`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD_HI: u32 = 71u32;
```

X1 pipe high 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_GD_HA`

```rust
const R_TILEPRO_IMM16_X0_TLS_GD_HA: u32 = 72u32;
```

X0 pipe ha() 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X1_TLS_GD_HA`

```rust
const R_TILEPRO_IMM16_X1_TLS_GD_HA: u32 = 73u32;
```

X1 pipe ha() 16-bit TLS GD offset

### `R_TILEPRO_IMM16_X0_TLS_IE`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE: u32 = 74u32;
```

X0 pipe 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE: u32 = 75u32;
```

X1 pipe 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X0_TLS_IE_LO`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE_LO: u32 = 76u32;
```

X0 pipe low 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE_LO`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE_LO: u32 = 77u32;
```

X1 pipe low 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X0_TLS_IE_HI`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE_HI: u32 = 78u32;
```

X0 pipe high 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE_HI`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE_HI: u32 = 79u32;
```

X1 pipe high 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X0_TLS_IE_HA`

```rust
const R_TILEPRO_IMM16_X0_TLS_IE_HA: u32 = 80u32;
```

X0 pipe ha() 16-bit TLS IE offset

### `R_TILEPRO_IMM16_X1_TLS_IE_HA`

```rust
const R_TILEPRO_IMM16_X1_TLS_IE_HA: u32 = 81u32;
```

X1 pipe ha() 16-bit TLS IE offset

### `R_TILEPRO_TLS_DTPMOD32`

```rust
const R_TILEPRO_TLS_DTPMOD32: u32 = 82u32;
```

ID of module containing symbol

### `R_TILEPRO_TLS_DTPOFF32`

```rust
const R_TILEPRO_TLS_DTPOFF32: u32 = 83u32;
```

Offset in TLS block

### `R_TILEPRO_TLS_TPOFF32`

```rust
const R_TILEPRO_TLS_TPOFF32: u32 = 84u32;
```

Offset in static TLS block

### `R_TILEPRO_IMM16_X0_TLS_LE`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE: u32 = 85u32;
```

X0 pipe 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE: u32 = 86u32;
```

X1 pipe 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X0_TLS_LE_LO`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE_LO: u32 = 87u32;
```

X0 pipe low 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE_LO`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE_LO: u32 = 88u32;
```

X1 pipe low 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X0_TLS_LE_HI`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE_HI: u32 = 89u32;
```

X0 pipe high 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE_HI`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE_HI: u32 = 90u32;
```

X1 pipe high 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X0_TLS_LE_HA`

```rust
const R_TILEPRO_IMM16_X0_TLS_LE_HA: u32 = 91u32;
```

X0 pipe ha() 16-bit TLS LE offset

### `R_TILEPRO_IMM16_X1_TLS_LE_HA`

```rust
const R_TILEPRO_IMM16_X1_TLS_LE_HA: u32 = 92u32;
```

X1 pipe ha() 16-bit TLS LE offset

### `R_TILEPRO_GNU_VTINHERIT`

```rust
const R_TILEPRO_GNU_VTINHERIT: u32 = 128u32;
```

GNU C++ vtable hierarchy

### `R_TILEPRO_GNU_VTENTRY`

```rust
const R_TILEPRO_GNU_VTENTRY: u32 = 129u32;
```

GNU C++ vtable member usage

### `R_TILEGX_NONE`

```rust
const R_TILEGX_NONE: u32 = 0u32;
```

No reloc

### `R_TILEGX_64`

```rust
const R_TILEGX_64: u32 = 1u32;
```

Direct 64 bit

### `R_TILEGX_32`

```rust
const R_TILEGX_32: u32 = 2u32;
```

Direct 32 bit

### `R_TILEGX_16`

```rust
const R_TILEGX_16: u32 = 3u32;
```

Direct 16 bit

### `R_TILEGX_8`

```rust
const R_TILEGX_8: u32 = 4u32;
```

Direct 8 bit

### `R_TILEGX_64_PCREL`

```rust
const R_TILEGX_64_PCREL: u32 = 5u32;
```

PC relative 64 bit

### `R_TILEGX_32_PCREL`

```rust
const R_TILEGX_32_PCREL: u32 = 6u32;
```

PC relative 32 bit

### `R_TILEGX_16_PCREL`

```rust
const R_TILEGX_16_PCREL: u32 = 7u32;
```

PC relative 16 bit

### `R_TILEGX_8_PCREL`

```rust
const R_TILEGX_8_PCREL: u32 = 8u32;
```

PC relative 8 bit

### `R_TILEGX_HW0`

```rust
const R_TILEGX_HW0: u32 = 9u32;
```

hword 0 16-bit

### `R_TILEGX_HW1`

```rust
const R_TILEGX_HW1: u32 = 10u32;
```

hword 1 16-bit

### `R_TILEGX_HW2`

```rust
const R_TILEGX_HW2: u32 = 11u32;
```

hword 2 16-bit

### `R_TILEGX_HW3`

```rust
const R_TILEGX_HW3: u32 = 12u32;
```

hword 3 16-bit

### `R_TILEGX_HW0_LAST`

```rust
const R_TILEGX_HW0_LAST: u32 = 13u32;
```

last hword 0 16-bit

### `R_TILEGX_HW1_LAST`

```rust
const R_TILEGX_HW1_LAST: u32 = 14u32;
```

last hword 1 16-bit

### `R_TILEGX_HW2_LAST`

```rust
const R_TILEGX_HW2_LAST: u32 = 15u32;
```

last hword 2 16-bit

### `R_TILEGX_COPY`

```rust
const R_TILEGX_COPY: u32 = 16u32;
```

Copy relocation

### `R_TILEGX_GLOB_DAT`

```rust
const R_TILEGX_GLOB_DAT: u32 = 17u32;
```

Create GOT entry

### `R_TILEGX_JMP_SLOT`

```rust
const R_TILEGX_JMP_SLOT: u32 = 18u32;
```

Create PLT entry

### `R_TILEGX_RELATIVE`

```rust
const R_TILEGX_RELATIVE: u32 = 19u32;
```

Adjust by program base

### `R_TILEGX_BROFF_X1`

```rust
const R_TILEGX_BROFF_X1: u32 = 20u32;
```

X1 pipe branch offset

### `R_TILEGX_JUMPOFF_X1`

```rust
const R_TILEGX_JUMPOFF_X1: u32 = 21u32;
```

X1 pipe jump offset

### `R_TILEGX_JUMPOFF_X1_PLT`

```rust
const R_TILEGX_JUMPOFF_X1_PLT: u32 = 22u32;
```

X1 pipe jump offset to PLT

### `R_TILEGX_IMM8_X0`

```rust
const R_TILEGX_IMM8_X0: u32 = 23u32;
```

X0 pipe 8-bit

### `R_TILEGX_IMM8_Y0`

```rust
const R_TILEGX_IMM8_Y0: u32 = 24u32;
```

Y0 pipe 8-bit

### `R_TILEGX_IMM8_X1`

```rust
const R_TILEGX_IMM8_X1: u32 = 25u32;
```

X1 pipe 8-bit

### `R_TILEGX_IMM8_Y1`

```rust
const R_TILEGX_IMM8_Y1: u32 = 26u32;
```

Y1 pipe 8-bit

### `R_TILEGX_DEST_IMM8_X1`

```rust
const R_TILEGX_DEST_IMM8_X1: u32 = 27u32;
```

X1 pipe destination 8-bit

### `R_TILEGX_MT_IMM14_X1`

```rust
const R_TILEGX_MT_IMM14_X1: u32 = 28u32;
```

X1 pipe mtspr

### `R_TILEGX_MF_IMM14_X1`

```rust
const R_TILEGX_MF_IMM14_X1: u32 = 29u32;
```

X1 pipe mfspr

### `R_TILEGX_MMSTART_X0`

```rust
const R_TILEGX_MMSTART_X0: u32 = 30u32;
```

X0 pipe mm "start"

### `R_TILEGX_MMEND_X0`

```rust
const R_TILEGX_MMEND_X0: u32 = 31u32;
```

X0 pipe mm "end"

### `R_TILEGX_SHAMT_X0`

```rust
const R_TILEGX_SHAMT_X0: u32 = 32u32;
```

X0 pipe shift amount

### `R_TILEGX_SHAMT_X1`

```rust
const R_TILEGX_SHAMT_X1: u32 = 33u32;
```

X1 pipe shift amount

### `R_TILEGX_SHAMT_Y0`

```rust
const R_TILEGX_SHAMT_Y0: u32 = 34u32;
```

Y0 pipe shift amount

### `R_TILEGX_SHAMT_Y1`

```rust
const R_TILEGX_SHAMT_Y1: u32 = 35u32;
```

Y1 pipe shift amount

### `R_TILEGX_IMM16_X0_HW0`

```rust
const R_TILEGX_IMM16_X0_HW0: u32 = 36u32;
```

X0 pipe hword 0

### `R_TILEGX_IMM16_X1_HW0`

```rust
const R_TILEGX_IMM16_X1_HW0: u32 = 37u32;
```

X1 pipe hword 0

### `R_TILEGX_IMM16_X0_HW1`

```rust
const R_TILEGX_IMM16_X0_HW1: u32 = 38u32;
```

X0 pipe hword 1

### `R_TILEGX_IMM16_X1_HW1`

```rust
const R_TILEGX_IMM16_X1_HW1: u32 = 39u32;
```

X1 pipe hword 1

### `R_TILEGX_IMM16_X0_HW2`

```rust
const R_TILEGX_IMM16_X0_HW2: u32 = 40u32;
```

X0 pipe hword 2

### `R_TILEGX_IMM16_X1_HW2`

```rust
const R_TILEGX_IMM16_X1_HW2: u32 = 41u32;
```

X1 pipe hword 2

### `R_TILEGX_IMM16_X0_HW3`

```rust
const R_TILEGX_IMM16_X0_HW3: u32 = 42u32;
```

X0 pipe hword 3

### `R_TILEGX_IMM16_X1_HW3`

```rust
const R_TILEGX_IMM16_X1_HW3: u32 = 43u32;
```

X1 pipe hword 3

### `R_TILEGX_IMM16_X0_HW0_LAST`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST: u32 = 44u32;
```

X0 pipe last hword 0

### `R_TILEGX_IMM16_X1_HW0_LAST`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST: u32 = 45u32;
```

X1 pipe last hword 0

### `R_TILEGX_IMM16_X0_HW1_LAST`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST: u32 = 46u32;
```

X0 pipe last hword 1

### `R_TILEGX_IMM16_X1_HW1_LAST`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST: u32 = 47u32;
```

X1 pipe last hword 1

### `R_TILEGX_IMM16_X0_HW2_LAST`

```rust
const R_TILEGX_IMM16_X0_HW2_LAST: u32 = 48u32;
```

X0 pipe last hword 2

### `R_TILEGX_IMM16_X1_HW2_LAST`

```rust
const R_TILEGX_IMM16_X1_HW2_LAST: u32 = 49u32;
```

X1 pipe last hword 2

### `R_TILEGX_IMM16_X0_HW0_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_PCREL: u32 = 50u32;
```

X0 pipe PC relative hword 0

### `R_TILEGX_IMM16_X1_HW0_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_PCREL: u32 = 51u32;
```

X1 pipe PC relative hword 0

### `R_TILEGX_IMM16_X0_HW1_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_PCREL: u32 = 52u32;
```

X0 pipe PC relative hword 1

### `R_TILEGX_IMM16_X1_HW1_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_PCREL: u32 = 53u32;
```

X1 pipe PC relative hword 1

### `R_TILEGX_IMM16_X0_HW2_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_PCREL: u32 = 54u32;
```

X0 pipe PC relative hword 2

### `R_TILEGX_IMM16_X1_HW2_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_PCREL: u32 = 55u32;
```

X1 pipe PC relative hword 2

### `R_TILEGX_IMM16_X0_HW3_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW3_PCREL: u32 = 56u32;
```

X0 pipe PC relative hword 3

### `R_TILEGX_IMM16_X1_HW3_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW3_PCREL: u32 = 57u32;
```

X1 pipe PC relative hword 3

### `R_TILEGX_IMM16_X0_HW0_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_PCREL: u32 = 58u32;
```

X0 pipe PC-rel last hword 0

### `R_TILEGX_IMM16_X1_HW0_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_PCREL: u32 = 59u32;
```

X1 pipe PC-rel last hword 0

### `R_TILEGX_IMM16_X0_HW1_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_PCREL: u32 = 60u32;
```

X0 pipe PC-rel last hword 1

### `R_TILEGX_IMM16_X1_HW1_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_PCREL: u32 = 61u32;
```

X1 pipe PC-rel last hword 1

### `R_TILEGX_IMM16_X0_HW2_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_LAST_PCREL: u32 = 62u32;
```

X0 pipe PC-rel last hword 2

### `R_TILEGX_IMM16_X1_HW2_LAST_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_LAST_PCREL: u32 = 63u32;
```

X1 pipe PC-rel last hword 2

### `R_TILEGX_IMM16_X0_HW0_GOT`

```rust
const R_TILEGX_IMM16_X0_HW0_GOT: u32 = 64u32;
```

X0 pipe hword 0 GOT offset

### `R_TILEGX_IMM16_X1_HW0_GOT`

```rust
const R_TILEGX_IMM16_X1_HW0_GOT: u32 = 65u32;
```

X1 pipe hword 0 GOT offset

### `R_TILEGX_IMM16_X0_HW0_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_PLT_PCREL: u32 = 66u32;
```

X0 pipe PC-rel PLT hword 0

### `R_TILEGX_IMM16_X1_HW0_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_PLT_PCREL: u32 = 67u32;
```

X1 pipe PC-rel PLT hword 0

### `R_TILEGX_IMM16_X0_HW1_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_PLT_PCREL: u32 = 68u32;
```

X0 pipe PC-rel PLT hword 1

### `R_TILEGX_IMM16_X1_HW1_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_PLT_PCREL: u32 = 69u32;
```

X1 pipe PC-rel PLT hword 1

### `R_TILEGX_IMM16_X0_HW2_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_PLT_PCREL: u32 = 70u32;
```

X0 pipe PC-rel PLT hword 2

### `R_TILEGX_IMM16_X1_HW2_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_PLT_PCREL: u32 = 71u32;
```

X1 pipe PC-rel PLT hword 2

### `R_TILEGX_IMM16_X0_HW0_LAST_GOT`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_GOT: u32 = 72u32;
```

X0 pipe last hword 0 GOT offset

### `R_TILEGX_IMM16_X1_HW0_LAST_GOT`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_GOT: u32 = 73u32;
```

X1 pipe last hword 0 GOT offset

### `R_TILEGX_IMM16_X0_HW1_LAST_GOT`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_GOT: u32 = 74u32;
```

X0 pipe last hword 1 GOT offset

### `R_TILEGX_IMM16_X1_HW1_LAST_GOT`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_GOT: u32 = 75u32;
```

X1 pipe last hword 1 GOT offset

### `R_TILEGX_IMM16_X0_HW3_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW3_PLT_PCREL: u32 = 76u32;
```

X0 pipe PC-rel PLT hword 3

### `R_TILEGX_IMM16_X1_HW3_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW3_PLT_PCREL: u32 = 77u32;
```

X1 pipe PC-rel PLT hword 3

### `R_TILEGX_IMM16_X0_HW0_TLS_GD`

```rust
const R_TILEGX_IMM16_X0_HW0_TLS_GD: u32 = 78u32;
```

X0 pipe hword 0 TLS GD offset

### `R_TILEGX_IMM16_X1_HW0_TLS_GD`

```rust
const R_TILEGX_IMM16_X1_HW0_TLS_GD: u32 = 79u32;
```

X1 pipe hword 0 TLS GD offset

### `R_TILEGX_IMM16_X0_HW0_TLS_LE`

```rust
const R_TILEGX_IMM16_X0_HW0_TLS_LE: u32 = 80u32;
```

X0 pipe hword 0 TLS LE offset

### `R_TILEGX_IMM16_X1_HW0_TLS_LE`

```rust
const R_TILEGX_IMM16_X1_HW0_TLS_LE: u32 = 81u32;
```

X1 pipe hword 0 TLS LE offset

### `R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE: u32 = 82u32;
```

X0 pipe last hword 0 LE off

### `R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE: u32 = 83u32;
```

X1 pipe last hword 0 LE off

### `R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE: u32 = 84u32;
```

X0 pipe last hword 1 LE off

### `R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE: u32 = 85u32;
```

X1 pipe last hword 1 LE off

### `R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD: u32 = 86u32;
```

X0 pipe last hword 0 GD off

### `R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD: u32 = 87u32;
```

X1 pipe last hword 0 GD off

### `R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD: u32 = 88u32;
```

X0 pipe last hword 1 GD off

### `R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD: u32 = 89u32;
```

X1 pipe last hword 1 GD off

### `R_TILEGX_IMM16_X0_HW0_TLS_IE`

```rust
const R_TILEGX_IMM16_X0_HW0_TLS_IE: u32 = 92u32;
```

X0 pipe hword 0 TLS IE offset

### `R_TILEGX_IMM16_X1_HW0_TLS_IE`

```rust
const R_TILEGX_IMM16_X1_HW0_TLS_IE: u32 = 93u32;
```

X1 pipe hword 0 TLS IE offset

### `R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL: u32 = 94u32;
```

X0 pipe PC-rel PLT last hword 0

### `R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL: u32 = 95u32;
```

X1 pipe PC-rel PLT last hword 0

### `R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL: u32 = 96u32;
```

X0 pipe PC-rel PLT last hword 1

### `R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL: u32 = 97u32;
```

X1 pipe PC-rel PLT last hword 1

### `R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL: u32 = 98u32;
```

X0 pipe PC-rel PLT last hword 2

### `R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL`

```rust
const R_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL: u32 = 99u32;
```

X1 pipe PC-rel PLT last hword 2

### `R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE: u32 = 100u32;
```

X0 pipe last hword 0 IE off

### `R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE: u32 = 101u32;
```

X1 pipe last hword 0 IE off

### `R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE: u32 = 102u32;
```

X0 pipe last hword 1 IE off

### `R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE`

```rust
const R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE: u32 = 103u32;
```

X1 pipe last hword 1 IE off

### `R_TILEGX_TLS_DTPMOD64`

```rust
const R_TILEGX_TLS_DTPMOD64: u32 = 106u32;
```

64-bit ID of symbol's module

### `R_TILEGX_TLS_DTPOFF64`

```rust
const R_TILEGX_TLS_DTPOFF64: u32 = 107u32;
```

64-bit offset in TLS block

### `R_TILEGX_TLS_TPOFF64`

```rust
const R_TILEGX_TLS_TPOFF64: u32 = 108u32;
```

64-bit offset in static TLS block

### `R_TILEGX_TLS_DTPMOD32`

```rust
const R_TILEGX_TLS_DTPMOD32: u32 = 109u32;
```

32-bit ID of symbol's module

### `R_TILEGX_TLS_DTPOFF32`

```rust
const R_TILEGX_TLS_DTPOFF32: u32 = 110u32;
```

32-bit offset in TLS block

### `R_TILEGX_TLS_TPOFF32`

```rust
const R_TILEGX_TLS_TPOFF32: u32 = 111u32;
```

32-bit offset in static TLS block

### `R_TILEGX_TLS_GD_CALL`

```rust
const R_TILEGX_TLS_GD_CALL: u32 = 112u32;
```

"jal" for TLS GD

### `R_TILEGX_IMM8_X0_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_X0_TLS_GD_ADD: u32 = 113u32;
```

X0 pipe "addi" for TLS GD

### `R_TILEGX_IMM8_X1_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_X1_TLS_GD_ADD: u32 = 114u32;
```

X1 pipe "addi" for TLS GD

### `R_TILEGX_IMM8_Y0_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_Y0_TLS_GD_ADD: u32 = 115u32;
```

Y0 pipe "addi" for TLS GD

### `R_TILEGX_IMM8_Y1_TLS_GD_ADD`

```rust
const R_TILEGX_IMM8_Y1_TLS_GD_ADD: u32 = 116u32;
```

Y1 pipe "addi" for TLS GD

### `R_TILEGX_TLS_IE_LOAD`

```rust
const R_TILEGX_TLS_IE_LOAD: u32 = 117u32;
```

"ld_tls" for TLS IE

### `R_TILEGX_IMM8_X0_TLS_ADD`

```rust
const R_TILEGX_IMM8_X0_TLS_ADD: u32 = 118u32;
```

X0 pipe "addi" for TLS GD/IE

### `R_TILEGX_IMM8_X1_TLS_ADD`

```rust
const R_TILEGX_IMM8_X1_TLS_ADD: u32 = 119u32;
```

X1 pipe "addi" for TLS GD/IE

### `R_TILEGX_IMM8_Y0_TLS_ADD`

```rust
const R_TILEGX_IMM8_Y0_TLS_ADD: u32 = 120u32;
```

Y0 pipe "addi" for TLS GD/IE

### `R_TILEGX_IMM8_Y1_TLS_ADD`

```rust
const R_TILEGX_IMM8_Y1_TLS_ADD: u32 = 121u32;
```

Y1 pipe "addi" for TLS GD/IE

### `R_TILEGX_GNU_VTINHERIT`

```rust
const R_TILEGX_GNU_VTINHERIT: u32 = 128u32;
```

GNU C++ vtable hierarchy

### `R_TILEGX_GNU_VTENTRY`

```rust
const R_TILEGX_GNU_VTENTRY: u32 = 129u32;
```

GNU C++ vtable member usage

### `EF_RISCV_RVC`

```rust
const EF_RISCV_RVC: u32 = 1u32;
```

### `EF_RISCV_FLOAT_ABI`

```rust
const EF_RISCV_FLOAT_ABI: u32 = 6u32;
```

### `EF_RISCV_FLOAT_ABI_SOFT`

```rust
const EF_RISCV_FLOAT_ABI_SOFT: u32 = 0u32;
```

### `EF_RISCV_FLOAT_ABI_SINGLE`

```rust
const EF_RISCV_FLOAT_ABI_SINGLE: u32 = 2u32;
```

### `EF_RISCV_FLOAT_ABI_DOUBLE`

```rust
const EF_RISCV_FLOAT_ABI_DOUBLE: u32 = 4u32;
```

### `EF_RISCV_FLOAT_ABI_QUAD`

```rust
const EF_RISCV_FLOAT_ABI_QUAD: u32 = 6u32;
```

### `EF_RISCV_RVE`

```rust
const EF_RISCV_RVE: u32 = 8u32;
```

### `EF_RISCV_TSO`

```rust
const EF_RISCV_TSO: u32 = 16u32;
```

### `EF_RISCV_RV64ILP32`

```rust
const EF_RISCV_RV64ILP32: u32 = 32u32;
```

### `SHT_RISCV_ATTRIBUTES`

```rust
const SHT_RISCV_ATTRIBUTES: u32 = 1_879_048_195u32;
```

RISC-V attributes section.

### `R_RISCV_NONE`

```rust
const R_RISCV_NONE: u32 = 0u32;
```

### `R_RISCV_32`

```rust
const R_RISCV_32: u32 = 1u32;
```

### `R_RISCV_64`

```rust
const R_RISCV_64: u32 = 2u32;
```

### `R_RISCV_RELATIVE`

```rust
const R_RISCV_RELATIVE: u32 = 3u32;
```

### `R_RISCV_COPY`

```rust
const R_RISCV_COPY: u32 = 4u32;
```

### `R_RISCV_JUMP_SLOT`

```rust
const R_RISCV_JUMP_SLOT: u32 = 5u32;
```

### `R_RISCV_TLS_DTPMOD32`

```rust
const R_RISCV_TLS_DTPMOD32: u32 = 6u32;
```

### `R_RISCV_TLS_DTPMOD64`

```rust
const R_RISCV_TLS_DTPMOD64: u32 = 7u32;
```

### `R_RISCV_TLS_DTPREL32`

```rust
const R_RISCV_TLS_DTPREL32: u32 = 8u32;
```

### `R_RISCV_TLS_DTPREL64`

```rust
const R_RISCV_TLS_DTPREL64: u32 = 9u32;
```

### `R_RISCV_TLS_TPREL32`

```rust
const R_RISCV_TLS_TPREL32: u32 = 10u32;
```

### `R_RISCV_TLS_TPREL64`

```rust
const R_RISCV_TLS_TPREL64: u32 = 11u32;
```

### `R_RISCV_TLSDESC`

```rust
const R_RISCV_TLSDESC: u32 = 12u32;
```

### `R_RISCV_BRANCH`

```rust
const R_RISCV_BRANCH: u32 = 16u32;
```

### `R_RISCV_JAL`

```rust
const R_RISCV_JAL: u32 = 17u32;
```

### `R_RISCV_CALL`

```rust
const R_RISCV_CALL: u32 = 18u32;
```

### `R_RISCV_CALL_PLT`

```rust
const R_RISCV_CALL_PLT: u32 = 19u32;
```

### `R_RISCV_GOT_HI20`

```rust
const R_RISCV_GOT_HI20: u32 = 20u32;
```

### `R_RISCV_TLS_GOT_HI20`

```rust
const R_RISCV_TLS_GOT_HI20: u32 = 21u32;
```

### `R_RISCV_TLS_GD_HI20`

```rust
const R_RISCV_TLS_GD_HI20: u32 = 22u32;
```

### `R_RISCV_PCREL_HI20`

```rust
const R_RISCV_PCREL_HI20: u32 = 23u32;
```

### `R_RISCV_PCREL_LO12_I`

```rust
const R_RISCV_PCREL_LO12_I: u32 = 24u32;
```

### `R_RISCV_PCREL_LO12_S`

```rust
const R_RISCV_PCREL_LO12_S: u32 = 25u32;
```

### `R_RISCV_HI20`

```rust
const R_RISCV_HI20: u32 = 26u32;
```

### `R_RISCV_LO12_I`

```rust
const R_RISCV_LO12_I: u32 = 27u32;
```

### `R_RISCV_LO12_S`

```rust
const R_RISCV_LO12_S: u32 = 28u32;
```

### `R_RISCV_TPREL_HI20`

```rust
const R_RISCV_TPREL_HI20: u32 = 29u32;
```

### `R_RISCV_TPREL_LO12_I`

```rust
const R_RISCV_TPREL_LO12_I: u32 = 30u32;
```

### `R_RISCV_TPREL_LO12_S`

```rust
const R_RISCV_TPREL_LO12_S: u32 = 31u32;
```

### `R_RISCV_TPREL_ADD`

```rust
const R_RISCV_TPREL_ADD: u32 = 32u32;
```

### `R_RISCV_ADD8`

```rust
const R_RISCV_ADD8: u32 = 33u32;
```

### `R_RISCV_ADD16`

```rust
const R_RISCV_ADD16: u32 = 34u32;
```

### `R_RISCV_ADD32`

```rust
const R_RISCV_ADD32: u32 = 35u32;
```

### `R_RISCV_ADD64`

```rust
const R_RISCV_ADD64: u32 = 36u32;
```

### `R_RISCV_SUB8`

```rust
const R_RISCV_SUB8: u32 = 37u32;
```

### `R_RISCV_SUB16`

```rust
const R_RISCV_SUB16: u32 = 38u32;
```

### `R_RISCV_SUB32`

```rust
const R_RISCV_SUB32: u32 = 39u32;
```

### `R_RISCV_SUB64`

```rust
const R_RISCV_SUB64: u32 = 40u32;
```

### `R_RISCV_GOT32_PCREL`

```rust
const R_RISCV_GOT32_PCREL: u32 = 41u32;
```

### `R_RISCV_ALIGN`

```rust
const R_RISCV_ALIGN: u32 = 43u32;
```

### `R_RISCV_RVC_BRANCH`

```rust
const R_RISCV_RVC_BRANCH: u32 = 44u32;
```

### `R_RISCV_RVC_JUMP`

```rust
const R_RISCV_RVC_JUMP: u32 = 45u32;
```

### `R_RISCV_RVC_LUI`

```rust
const R_RISCV_RVC_LUI: u32 = 46u32;
```

### `R_RISCV_GPREL_I`

```rust
const R_RISCV_GPREL_I: u32 = 47u32;
```

### `R_RISCV_GPREL_S`

```rust
const R_RISCV_GPREL_S: u32 = 48u32;
```

### `R_RISCV_TPREL_I`

```rust
const R_RISCV_TPREL_I: u32 = 49u32;
```

### `R_RISCV_TPREL_S`

```rust
const R_RISCV_TPREL_S: u32 = 50u32;
```

### `R_RISCV_RELAX`

```rust
const R_RISCV_RELAX: u32 = 51u32;
```

### `R_RISCV_SUB6`

```rust
const R_RISCV_SUB6: u32 = 52u32;
```

### `R_RISCV_SET6`

```rust
const R_RISCV_SET6: u32 = 53u32;
```

### `R_RISCV_SET8`

```rust
const R_RISCV_SET8: u32 = 54u32;
```

### `R_RISCV_SET16`

```rust
const R_RISCV_SET16: u32 = 55u32;
```

### `R_RISCV_SET32`

```rust
const R_RISCV_SET32: u32 = 56u32;
```

### `R_RISCV_32_PCREL`

```rust
const R_RISCV_32_PCREL: u32 = 57u32;
```

### `R_RISCV_IRELATIVE`

```rust
const R_RISCV_IRELATIVE: u32 = 58u32;
```

### `R_RISCV_PLT32`

```rust
const R_RISCV_PLT32: u32 = 59u32;
```

### `R_RISCV_SET_ULEB128`

```rust
const R_RISCV_SET_ULEB128: u32 = 60u32;
```

### `R_RISCV_SUB_ULEB128`

```rust
const R_RISCV_SUB_ULEB128: u32 = 61u32;
```

### `R_RISCV_TLSDESC_HI20`

```rust
const R_RISCV_TLSDESC_HI20: u32 = 62u32;
```

### `R_RISCV_TLSDESC_LOAD_LO12`

```rust
const R_RISCV_TLSDESC_LOAD_LO12: u32 = 63u32;
```

### `R_RISCV_TLSDESC_ADD_LO12`

```rust
const R_RISCV_TLSDESC_ADD_LO12: u32 = 64u32;
```

### `R_RISCV_TLSDESC_CALL`

```rust
const R_RISCV_TLSDESC_CALL: u32 = 65u32;
```

### `R_BPF_NONE`

```rust
const R_BPF_NONE: u32 = 0u32;
```

No reloc

### `R_BPF_64_64`

```rust
const R_BPF_64_64: u32 = 1u32;
```

### `R_BPF_64_32`

```rust
const R_BPF_64_32: u32 = 10u32;
```

### `R_SBF_NONE`

```rust
const R_SBF_NONE: u32 = 0u32;
```

No reloc

### `R_SBF_64_64`

```rust
const R_SBF_64_64: u32 = 1u32;
```

### `R_SBF_64_32`

```rust
const R_SBF_64_32: u32 = 10u32;
```

### `R_METAG_HIADDR16`

```rust
const R_METAG_HIADDR16: u32 = 0u32;
```

### `R_METAG_LOADDR16`

```rust
const R_METAG_LOADDR16: u32 = 1u32;
```

### `R_METAG_ADDR32`

```rust
const R_METAG_ADDR32: u32 = 2u32;
```

32bit absolute address

### `R_METAG_NONE`

```rust
const R_METAG_NONE: u32 = 3u32;
```

No reloc

### `R_METAG_RELBRANCH`

```rust
const R_METAG_RELBRANCH: u32 = 4u32;
```

### `R_METAG_GETSETOFF`

```rust
const R_METAG_GETSETOFF: u32 = 5u32;
```

### `R_METAG_REG32OP1`

```rust
const R_METAG_REG32OP1: u32 = 6u32;
```

### `R_METAG_REG32OP2`

```rust
const R_METAG_REG32OP2: u32 = 7u32;
```

### `R_METAG_REG32OP3`

```rust
const R_METAG_REG32OP3: u32 = 8u32;
```

### `R_METAG_REG16OP1`

```rust
const R_METAG_REG16OP1: u32 = 9u32;
```

### `R_METAG_REG16OP2`

```rust
const R_METAG_REG16OP2: u32 = 10u32;
```

### `R_METAG_REG16OP3`

```rust
const R_METAG_REG16OP3: u32 = 11u32;
```

### `R_METAG_REG32OP4`

```rust
const R_METAG_REG32OP4: u32 = 12u32;
```

### `R_METAG_HIOG`

```rust
const R_METAG_HIOG: u32 = 13u32;
```

### `R_METAG_LOOG`

```rust
const R_METAG_LOOG: u32 = 14u32;
```

### `R_METAG_REL8`

```rust
const R_METAG_REL8: u32 = 15u32;
```

### `R_METAG_REL16`

```rust
const R_METAG_REL16: u32 = 16u32;
```

### `R_METAG_GNU_VTINHERIT`

```rust
const R_METAG_GNU_VTINHERIT: u32 = 30u32;
```

### `R_METAG_GNU_VTENTRY`

```rust
const R_METAG_GNU_VTENTRY: u32 = 31u32;
```

### `R_METAG_HI16_GOTOFF`

```rust
const R_METAG_HI16_GOTOFF: u32 = 32u32;
```

### `R_METAG_LO16_GOTOFF`

```rust
const R_METAG_LO16_GOTOFF: u32 = 33u32;
```

### `R_METAG_GETSET_GOTOFF`

```rust
const R_METAG_GETSET_GOTOFF: u32 = 34u32;
```

### `R_METAG_GETSET_GOT`

```rust
const R_METAG_GETSET_GOT: u32 = 35u32;
```

### `R_METAG_HI16_GOTPC`

```rust
const R_METAG_HI16_GOTPC: u32 = 36u32;
```

### `R_METAG_LO16_GOTPC`

```rust
const R_METAG_LO16_GOTPC: u32 = 37u32;
```

### `R_METAG_HI16_PLT`

```rust
const R_METAG_HI16_PLT: u32 = 38u32;
```

### `R_METAG_LO16_PLT`

```rust
const R_METAG_LO16_PLT: u32 = 39u32;
```

### `R_METAG_RELBRANCH_PLT`

```rust
const R_METAG_RELBRANCH_PLT: u32 = 40u32;
```

### `R_METAG_GOTOFF`

```rust
const R_METAG_GOTOFF: u32 = 41u32;
```

### `R_METAG_PLT`

```rust
const R_METAG_PLT: u32 = 42u32;
```

### `R_METAG_COPY`

```rust
const R_METAG_COPY: u32 = 43u32;
```

### `R_METAG_JMP_SLOT`

```rust
const R_METAG_JMP_SLOT: u32 = 44u32;
```

### `R_METAG_RELATIVE`

```rust
const R_METAG_RELATIVE: u32 = 45u32;
```

### `R_METAG_GLOB_DAT`

```rust
const R_METAG_GLOB_DAT: u32 = 46u32;
```

### `R_METAG_TLS_GD`

```rust
const R_METAG_TLS_GD: u32 = 47u32;
```

### `R_METAG_TLS_LDM`

```rust
const R_METAG_TLS_LDM: u32 = 48u32;
```

### `R_METAG_TLS_LDO_HI16`

```rust
const R_METAG_TLS_LDO_HI16: u32 = 49u32;
```

### `R_METAG_TLS_LDO_LO16`

```rust
const R_METAG_TLS_LDO_LO16: u32 = 50u32;
```

### `R_METAG_TLS_LDO`

```rust
const R_METAG_TLS_LDO: u32 = 51u32;
```

### `R_METAG_TLS_IE`

```rust
const R_METAG_TLS_IE: u32 = 52u32;
```

### `R_METAG_TLS_IENONPIC`

```rust
const R_METAG_TLS_IENONPIC: u32 = 53u32;
```

### `R_METAG_TLS_IENONPIC_HI16`

```rust
const R_METAG_TLS_IENONPIC_HI16: u32 = 54u32;
```

### `R_METAG_TLS_IENONPIC_LO16`

```rust
const R_METAG_TLS_IENONPIC_LO16: u32 = 55u32;
```

### `R_METAG_TLS_TPOFF`

```rust
const R_METAG_TLS_TPOFF: u32 = 56u32;
```

### `R_METAG_TLS_DTPMOD`

```rust
const R_METAG_TLS_DTPMOD: u32 = 57u32;
```

### `R_METAG_TLS_DTPOFF`

```rust
const R_METAG_TLS_DTPOFF: u32 = 58u32;
```

### `R_METAG_TLS_LE`

```rust
const R_METAG_TLS_LE: u32 = 59u32;
```

### `R_METAG_TLS_LE_HI16`

```rust
const R_METAG_TLS_LE_HI16: u32 = 60u32;
```

### `R_METAG_TLS_LE_LO16`

```rust
const R_METAG_TLS_LE_LO16: u32 = 61u32;
```

### `R_NDS32_NONE`

```rust
const R_NDS32_NONE: u32 = 0u32;
```

### `R_NDS32_32_RELA`

```rust
const R_NDS32_32_RELA: u32 = 20u32;
```

### `R_NDS32_COPY`

```rust
const R_NDS32_COPY: u32 = 39u32;
```

### `R_NDS32_GLOB_DAT`

```rust
const R_NDS32_GLOB_DAT: u32 = 40u32;
```

### `R_NDS32_JMP_SLOT`

```rust
const R_NDS32_JMP_SLOT: u32 = 41u32;
```

### `R_NDS32_RELATIVE`

```rust
const R_NDS32_RELATIVE: u32 = 42u32;
```

### `R_NDS32_TLS_TPOFF`

```rust
const R_NDS32_TLS_TPOFF: u32 = 102u32;
```

### `R_NDS32_TLS_DESC`

```rust
const R_NDS32_TLS_DESC: u32 = 119u32;
```

### `EF_LARCH_ABI_MODIFIER_MASK`

```rust
const EF_LARCH_ABI_MODIFIER_MASK: u32 = 7u32;
```

Additional properties of the base ABI type, including the FP calling
convention.

### `EF_LARCH_ABI_SOFT_FLOAT`

```rust
const EF_LARCH_ABI_SOFT_FLOAT: u32 = 1u32;
```

Uses GPRs and the stack for parameter passing

### `EF_LARCH_ABI_SINGLE_FLOAT`

```rust
const EF_LARCH_ABI_SINGLE_FLOAT: u32 = 2u32;
```

Uses GPRs, 32-bit FPRs and the stack for parameter passing

### `EF_LARCH_ABI_DOUBLE_FLOAT`

```rust
const EF_LARCH_ABI_DOUBLE_FLOAT: u32 = 3u32;
```

Uses GPRs, 64-bit FPRs and the stack for parameter passing

### `EF_LARCH_OBJABI_V1`

```rust
const EF_LARCH_OBJABI_V1: u32 = 64u32;
```

Uses relocation types directly writing to immediate slots

### `R_LARCH_NONE`

```rust
const R_LARCH_NONE: u32 = 0u32;
```

No reloc

### `R_LARCH_32`

```rust
const R_LARCH_32: u32 = 1u32;
```

Runtime address resolving

### `R_LARCH_64`

```rust
const R_LARCH_64: u32 = 2u32;
```

Runtime address resolving

### `R_LARCH_RELATIVE`

```rust
const R_LARCH_RELATIVE: u32 = 3u32;
```

Runtime fixup for load-address

### `R_LARCH_COPY`

```rust
const R_LARCH_COPY: u32 = 4u32;
```

Runtime memory copy in executable

### `R_LARCH_JUMP_SLOT`

```rust
const R_LARCH_JUMP_SLOT: u32 = 5u32;
```

Runtime PLT supporting

### `R_LARCH_TLS_DTPMOD32`

```rust
const R_LARCH_TLS_DTPMOD32: u32 = 6u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_DTPMOD64`

```rust
const R_LARCH_TLS_DTPMOD64: u32 = 7u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_DTPREL32`

```rust
const R_LARCH_TLS_DTPREL32: u32 = 8u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_DTPREL64`

```rust
const R_LARCH_TLS_DTPREL64: u32 = 9u32;
```

Runtime relocation for TLS-GD

### `R_LARCH_TLS_TPREL32`

```rust
const R_LARCH_TLS_TPREL32: u32 = 10u32;
```

Runtime relocation for TLE-IE

### `R_LARCH_TLS_TPREL64`

```rust
const R_LARCH_TLS_TPREL64: u32 = 11u32;
```

Runtime relocation for TLE-IE

### `R_LARCH_IRELATIVE`

```rust
const R_LARCH_IRELATIVE: u32 = 12u32;
```

Runtime local indirect function resolving

### `R_LARCH_MARK_LA`

```rust
const R_LARCH_MARK_LA: u32 = 20u32;
```

Mark la.abs: load absolute address for static link.

### `R_LARCH_MARK_PCREL`

```rust
const R_LARCH_MARK_PCREL: u32 = 21u32;
```

Mark external label branch: access PC relative address for static link.

### `R_LARCH_SOP_PUSH_PCREL`

```rust
const R_LARCH_SOP_PUSH_PCREL: u32 = 22u32;
```

Push PC-relative offset

### `R_LARCH_SOP_PUSH_ABSOLUTE`

```rust
const R_LARCH_SOP_PUSH_ABSOLUTE: u32 = 23u32;
```

Push constant or absolute address

### `R_LARCH_SOP_PUSH_DUP`

```rust
const R_LARCH_SOP_PUSH_DUP: u32 = 24u32;
```

Duplicate stack top

### `R_LARCH_SOP_PUSH_GPREL`

```rust
const R_LARCH_SOP_PUSH_GPREL: u32 = 25u32;
```

Push for access GOT entry

### `R_LARCH_SOP_PUSH_TLS_TPREL`

```rust
const R_LARCH_SOP_PUSH_TLS_TPREL: u32 = 26u32;
```

Push for TLS-LE

### `R_LARCH_SOP_PUSH_TLS_GOT`

```rust
const R_LARCH_SOP_PUSH_TLS_GOT: u32 = 27u32;
```

Push for TLS-IE

### `R_LARCH_SOP_PUSH_TLS_GD`

```rust
const R_LARCH_SOP_PUSH_TLS_GD: u32 = 28u32;
```

Push for TLS-GD

### `R_LARCH_SOP_PUSH_PLT_PCREL`

```rust
const R_LARCH_SOP_PUSH_PLT_PCREL: u32 = 29u32;
```

Push for external function calling

### `R_LARCH_SOP_ASSERT`

```rust
const R_LARCH_SOP_ASSERT: u32 = 30u32;
```

Assert stack top

### `R_LARCH_SOP_NOT`

```rust
const R_LARCH_SOP_NOT: u32 = 31u32;
```

Stack top logical not (unary)

### `R_LARCH_SOP_SUB`

```rust
const R_LARCH_SOP_SUB: u32 = 32u32;
```

Stack top subtraction (binary)

### `R_LARCH_SOP_SL`

```rust
const R_LARCH_SOP_SL: u32 = 33u32;
```

Stack top left shift (binary)

### `R_LARCH_SOP_SR`

```rust
const R_LARCH_SOP_SR: u32 = 34u32;
```

Stack top right shift (binary)

### `R_LARCH_SOP_ADD`

```rust
const R_LARCH_SOP_ADD: u32 = 35u32;
```

Stack top addition (binary)

### `R_LARCH_SOP_AND`

```rust
const R_LARCH_SOP_AND: u32 = 36u32;
```

Stack top bitwise and (binary)

### `R_LARCH_SOP_IF_ELSE`

```rust
const R_LARCH_SOP_IF_ELSE: u32 = 37u32;
```

Stack top selection (tertiary)

### `R_LARCH_SOP_POP_32_S_10_5`

```rust
const R_LARCH_SOP_POP_32_S_10_5: u32 = 38u32;
```

Pop stack top to fill 5-bit signed immediate operand

### `R_LARCH_SOP_POP_32_U_10_12`

```rust
const R_LARCH_SOP_POP_32_U_10_12: u32 = 39u32;
```

Pop stack top to fill 12-bit unsigned immediate operand

### `R_LARCH_SOP_POP_32_S_10_12`

```rust
const R_LARCH_SOP_POP_32_S_10_12: u32 = 40u32;
```

Pop stack top to fill 12-bit signed immediate operand

### `R_LARCH_SOP_POP_32_S_10_16`

```rust
const R_LARCH_SOP_POP_32_S_10_16: u32 = 41u32;
```

Pop stack top to fill 16-bit signed immediate operand

### `R_LARCH_SOP_POP_32_S_10_16_S2`

```rust
const R_LARCH_SOP_POP_32_S_10_16_S2: u32 = 42u32;
```

Pop stack top to fill 18-bit signed immediate operand with two trailing
zeros implied

### `R_LARCH_SOP_POP_32_S_5_20`

```rust
const R_LARCH_SOP_POP_32_S_5_20: u32 = 43u32;
```

Pop stack top to fill 20-bit signed immediate operand

### `R_LARCH_SOP_POP_32_S_0_5_10_16_S2`

```rust
const R_LARCH_SOP_POP_32_S_0_5_10_16_S2: u32 = 44u32;
```

Pop stack top to fill 23-bit signed immediate operand with two trailing
zeros implied

### `R_LARCH_SOP_POP_32_S_0_10_10_16_S2`

```rust
const R_LARCH_SOP_POP_32_S_0_10_10_16_S2: u32 = 45u32;
```

Pop stack top to fill 28-bit signed immediate operand with two trailing
zeros implied

### `R_LARCH_SOP_POP_32_U`

```rust
const R_LARCH_SOP_POP_32_U: u32 = 46u32;
```

Pop stack top to fill an instruction

### `R_LARCH_ADD8`

```rust
const R_LARCH_ADD8: u32 = 47u32;
```

8-bit in-place addition

### `R_LARCH_ADD16`

```rust
const R_LARCH_ADD16: u32 = 48u32;
```

16-bit in-place addition

### `R_LARCH_ADD24`

```rust
const R_LARCH_ADD24: u32 = 49u32;
```

24-bit in-place addition

### `R_LARCH_ADD32`

```rust
const R_LARCH_ADD32: u32 = 50u32;
```

32-bit in-place addition

### `R_LARCH_ADD64`

```rust
const R_LARCH_ADD64: u32 = 51u32;
```

64-bit in-place addition

### `R_LARCH_SUB8`

```rust
const R_LARCH_SUB8: u32 = 52u32;
```

8-bit in-place subtraction

### `R_LARCH_SUB16`

```rust
const R_LARCH_SUB16: u32 = 53u32;
```

16-bit in-place subtraction

### `R_LARCH_SUB24`

```rust
const R_LARCH_SUB24: u32 = 54u32;
```

24-bit in-place subtraction

### `R_LARCH_SUB32`

```rust
const R_LARCH_SUB32: u32 = 55u32;
```

32-bit in-place subtraction

### `R_LARCH_SUB64`

```rust
const R_LARCH_SUB64: u32 = 56u32;
```

64-bit in-place subtraction

### `R_LARCH_GNU_VTINHERIT`

```rust
const R_LARCH_GNU_VTINHERIT: u32 = 57u32;
```

GNU C++ vtable hierarchy

### `R_LARCH_GNU_VTENTRY`

```rust
const R_LARCH_GNU_VTENTRY: u32 = 58u32;
```

GNU C++ vtable member usage

### `R_LARCH_B16`

```rust
const R_LARCH_B16: u32 = 64u32;
```

18-bit PC-relative jump offset with two trailing zeros

### `R_LARCH_B21`

```rust
const R_LARCH_B21: u32 = 65u32;
```

23-bit PC-relative jump offset with two trailing zeros

### `R_LARCH_B26`

```rust
const R_LARCH_B26: u32 = 66u32;
```

28-bit PC-relative jump offset with two trailing zeros

### `R_LARCH_ABS_HI20`

```rust
const R_LARCH_ABS_HI20: u32 = 67u32;
```

12..=31 bits of 32/64-bit absolute address

### `R_LARCH_ABS_LO12`

```rust
const R_LARCH_ABS_LO12: u32 = 68u32;
```

0..=11 bits of 32/64-bit absolute address

### `R_LARCH_ABS64_LO20`

```rust
const R_LARCH_ABS64_LO20: u32 = 69u32;
```

32..=51 bits of 64-bit absolute address

### `R_LARCH_ABS64_HI12`

```rust
const R_LARCH_ABS64_HI12: u32 = 70u32;
```

52..=63 bits of 64-bit absolute address

### `R_LARCH_PCALA_HI20`

```rust
const R_LARCH_PCALA_HI20: u32 = 71u32;
```

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(S + A + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for `S + A` as `PC + offs` (`offs`
is sign-extended to VA bits).

### `R_LARCH_PCALA_LO12`

```rust
const R_LARCH_PCALA_LO12: u32 = 72u32;
```

Same as R_LARCH_ABS_LO12.  0..=11 bits of the 32/64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].

### `R_LARCH_PCALA64_LO20`

```rust
const R_LARCH_PCALA64_LO20: u32 = 73u32;
```

32..=51 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].

### `R_LARCH_PCALA64_HI12`

```rust
const R_LARCH_PCALA64_HI12: u32 = 74u32;
```

52..=63 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_PCALA_HI20].

### `R_LARCH_GOT_PC_HI20`

```rust
const R_LARCH_GOT_PC_HI20: u32 = 75u32;
```

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(GP + G + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for the GOT entry at `GP + G` as
`PC + offs` (`offs` is sign-extended to VA bits).

### `R_LARCH_GOT_PC_LO12`

```rust
const R_LARCH_GOT_PC_LO12: u32 = 76u32;
```

0..=11 bits of the 32/64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.

### `R_LARCH_GOT64_PC_LO20`

```rust
const R_LARCH_GOT64_PC_LO20: u32 = 77u32;
```

32..=51 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.

### `R_LARCH_GOT64_PC_HI12`

```rust
const R_LARCH_GOT64_PC_HI12: u32 = 78u32;
```

52..=63 bits of the 64-bit offset from the
[PC relative anchor][R_LARCH_GOT_PC_HI20] to the GOT entry.

### `R_LARCH_GOT_HI20`

```rust
const R_LARCH_GOT_HI20: u32 = 79u32;
```

12..=31 bits of 32/64-bit GOT entry absolute address

### `R_LARCH_GOT_LO12`

```rust
const R_LARCH_GOT_LO12: u32 = 80u32;
```

0..=11 bits of 32/64-bit GOT entry absolute address

### `R_LARCH_GOT64_LO20`

```rust
const R_LARCH_GOT64_LO20: u32 = 81u32;
```

32..=51 bits of 64-bit GOT entry absolute address

### `R_LARCH_GOT64_HI12`

```rust
const R_LARCH_GOT64_HI12: u32 = 82u32;
```

52..=63 bits of 64-bit GOT entry absolute address

### `R_LARCH_TLS_LE_HI20`

```rust
const R_LARCH_TLS_LE_HI20: u32 = 83u32;
```

12..=31 bits of TLS LE 32/64-bit offset from thread pointer

### `R_LARCH_TLS_LE_LO12`

```rust
const R_LARCH_TLS_LE_LO12: u32 = 84u32;
```

0..=11 bits of TLS LE 32/64-bit offset from thread pointer

### `R_LARCH_TLS_LE64_LO20`

```rust
const R_LARCH_TLS_LE64_LO20: u32 = 85u32;
```

32..=51 bits of TLS LE 64-bit offset from thread pointer

### `R_LARCH_TLS_LE64_HI12`

```rust
const R_LARCH_TLS_LE64_HI12: u32 = 86u32;
```

52..=63 bits of TLS LE 64-bit offset from thread pointer

### `R_LARCH_TLS_IE_PC_HI20`

```rust
const R_LARCH_TLS_IE_PC_HI20: u32 = 87u32;
```

The signed 32-bit offset `offs` from `PC & 0xfffff000` to
`(GP + IE + 0x800) & 0xfffff000`, with 12 trailing zeros removed.

We define the *PC relative anchor* for the TLS IE GOT entry at
`GP + IE` as `PC + offs` (`offs` is sign-extended to VA bits).

### `R_LARCH_TLS_IE_PC_LO12`

```rust
const R_LARCH_TLS_IE_PC_LO12: u32 = 88u32;
```

0..=12 bits of the 32/64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.

### `R_LARCH_TLS_IE64_PC_LO20`

```rust
const R_LARCH_TLS_IE64_PC_LO20: u32 = 89u32;
```

32..=51 bits of the 64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.

### `R_LARCH_TLS_IE64_PC_HI12`

```rust
const R_LARCH_TLS_IE64_PC_HI12: u32 = 90u32;
```

52..=63 bits of the 64-bit offset from the
[PC-relative anchor][R_LARCH_TLS_IE_PC_HI20] to the TLS IE GOT entry.

### `R_LARCH_TLS_IE_HI20`

```rust
const R_LARCH_TLS_IE_HI20: u32 = 91u32;
```

12..=31 bits of TLS IE GOT entry 32/64-bit absolute address

### `R_LARCH_TLS_IE_LO12`

```rust
const R_LARCH_TLS_IE_LO12: u32 = 92u32;
```

0..=11 bits of TLS IE GOT entry 32/64-bit absolute address

### `R_LARCH_TLS_IE64_LO20`

```rust
const R_LARCH_TLS_IE64_LO20: u32 = 93u32;
```

32..=51 bits of TLS IE GOT entry 64-bit absolute address

### `R_LARCH_TLS_IE64_HI12`

```rust
const R_LARCH_TLS_IE64_HI12: u32 = 94u32;
```

51..=63 bits of TLS IE GOT entry 64-bit absolute address

### `R_LARCH_TLS_LD_PC_HI20`

```rust
const R_LARCH_TLS_LD_PC_HI20: u32 = 95u32;
```

12..=31 bits of the offset from `PC` to `GP + GD + 0x800`, where
`GP + GD` is a TLS LD GOT entry

### `R_LARCH_TLS_LD_HI20`

```rust
const R_LARCH_TLS_LD_HI20: u32 = 96u32;
```

12..=31 bits of TLS LD GOT entry 32/64-bit absolute address

### `R_LARCH_TLS_GD_PC_HI20`

```rust
const R_LARCH_TLS_GD_PC_HI20: u32 = 97u32;
```

12..=31 bits of the 32/64-bit PC-relative offset to the PC-relative
anchor for the TLE GD GOT entry.

### `R_LARCH_TLS_GD_HI20`

```rust
const R_LARCH_TLS_GD_HI20: u32 = 98u32;
```

12..=31 bits of TLS GD GOT entry 32/64-bit absolute address

### `R_LARCH_32_PCREL`

```rust
const R_LARCH_32_PCREL: u32 = 99u32;
```

32-bit PC relative

### `R_LARCH_RELAX`

```rust
const R_LARCH_RELAX: u32 = 100u32;
```

Paired with a normal relocation at the same address to indicate the
instruction can be relaxed

### `R_LARCH_DELETE`

```rust
const R_LARCH_DELETE: u32 = 101u32;
```

Reserved

### `R_LARCH_ALIGN`

```rust
const R_LARCH_ALIGN: u32 = 102u32;
```

Delete some bytes to ensure the instruction at PC + A aligned to
`A.next_power_of_two()`-byte boundary

### `R_LARCH_PCREL20_S2`

```rust
const R_LARCH_PCREL20_S2: u32 = 103u32;
```

22-bit PC-relative offset with two trailing zeros

### `R_LARCH_CFA`

```rust
const R_LARCH_CFA: u32 = 104u32;
```

Reserved

### `R_LARCH_ADD6`

```rust
const R_LARCH_ADD6: u32 = 105u32;
```

6-bit in-place addition

### `R_LARCH_SUB6`

```rust
const R_LARCH_SUB6: u32 = 106u32;
```

6-bit in-place subtraction

### `R_LARCH_ADD_ULEB128`

```rust
const R_LARCH_ADD_ULEB128: u32 = 107u32;
```

LEB128 in-place addition

### `R_LARCH_SUB_ULEB128`

```rust
const R_LARCH_SUB_ULEB128: u32 = 108u32;
```

LEB128 in-place subtraction

### `R_LARCH_64_PCREL`

```rust
const R_LARCH_64_PCREL: u32 = 109u32;
```

64-bit PC relative

### `R_LARCH_CALL36`

```rust
const R_LARCH_CALL36: u32 = 110u32;
```

18..=37 bits of `S + A - PC` into the `pcaddu18i` instruction at `PC`,
and 2..=17 bits of `S + A - PC` into the `jirl` instruction at `PC + 4`

### `R_LARCH_TLS_DESC_PC_HI20`

```rust
const R_LARCH_TLS_DESC_PC_HI20: u32 = 111u32;
```

12..=31 bits of 32/64-bit PC-relative offset to TLS DESC GOT entry

### `R_LARCH_TLS_DESC_PC_LO12`

```rust
const R_LARCH_TLS_DESC_PC_LO12: u32 = 112u32;
```

0..=11 bits of 32/64-bit TLS DESC GOT entry address

### `R_LARCH_TLS_DESC64_PC_LO20`

```rust
const R_LARCH_TLS_DESC64_PC_LO20: u32 = 113u32;
```

32..=51 bits of 64-bit PC-relative offset to TLS DESC GOT entry

### `R_LARCH_TLS_DESC64_PC_HI12`

```rust
const R_LARCH_TLS_DESC64_PC_HI12: u32 = 114u32;
```

52..=63 bits of 64-bit PC-relative offset to TLS DESC GOT entry

### `R_LARCH_TLS_DESC_HI20`

```rust
const R_LARCH_TLS_DESC_HI20: u32 = 115u32;
```

12..=31 bits of 32/64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC_LO12`

```rust
const R_LARCH_TLS_DESC_LO12: u32 = 116u32;
```

0..=11 bits of 32/64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC64_LO20`

```rust
const R_LARCH_TLS_DESC64_LO20: u32 = 117u32;
```

32..=51 bits of 64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC64_HI12`

```rust
const R_LARCH_TLS_DESC64_HI12: u32 = 118u32;
```

52..=63 bits of 64-bit TLS DESC GOT entry absolute address

### `R_LARCH_TLS_DESC_LD`

```rust
const R_LARCH_TLS_DESC_LD: u32 = 119u32;
```

Used on ld.{w,d} for TLS DESC to get the resolve function address
from GOT entry

### `R_LARCH_TLS_DESC_CALL`

```rust
const R_LARCH_TLS_DESC_CALL: u32 = 120u32;
```

Used on jirl for TLS DESC to call the resolve function

### `R_LARCH_TLS_LE_HI20_R`

```rust
const R_LARCH_TLS_LE_HI20_R: u32 = 121u32;
```

12..=31 bits of TLS LE 32/64-bit offset from TP register, can be relaxed

### `R_LARCH_TLS_LE_ADD_R`

```rust
const R_LARCH_TLS_LE_ADD_R: u32 = 122u32;
```

TLS LE thread pointer usage, can be relaxed

### `R_LARCH_TLS_LE_LO12_R`

```rust
const R_LARCH_TLS_LE_LO12_R: u32 = 123u32;
```

0..=11 bits of TLS LE 32/64-bit offset from TP register, sign-extended,
can be relaxed.

### `R_LARCH_TLS_LD_PCREL20_S2`

```rust
const R_LARCH_TLS_LD_PCREL20_S2: u32 = 124u32;
```

22-bit PC-relative offset to TLS LD GOT entry

### `R_LARCH_TLS_GD_PCREL20_S2`

```rust
const R_LARCH_TLS_GD_PCREL20_S2: u32 = 125u32;
```

22-bit PC-relative offset to TLS GD GOT entry

### `R_LARCH_TLS_DESC_PCREL20_S2`

```rust
const R_LARCH_TLS_DESC_PCREL20_S2: u32 = 126u32;
```

22-bit PC-relative offset to TLS DESC GOT entry

### `R_XTENSA_NONE`

```rust
const R_XTENSA_NONE: u32 = 0u32;
```

### `R_XTENSA_32`

```rust
const R_XTENSA_32: u32 = 1u32;
```

### `R_XTENSA_RTLD`

```rust
const R_XTENSA_RTLD: u32 = 2u32;
```

### `R_XTENSA_GLOB_DAT`

```rust
const R_XTENSA_GLOB_DAT: u32 = 3u32;
```

### `R_XTENSA_JMP_SLOT`

```rust
const R_XTENSA_JMP_SLOT: u32 = 4u32;
```

### `R_XTENSA_RELATIVE`

```rust
const R_XTENSA_RELATIVE: u32 = 5u32;
```

### `R_XTENSA_PLT`

```rust
const R_XTENSA_PLT: u32 = 6u32;
```

### `R_XTENSA_OP0`

```rust
const R_XTENSA_OP0: u32 = 8u32;
```

### `R_XTENSA_OP1`

```rust
const R_XTENSA_OP1: u32 = 9u32;
```

### `R_XTENSA_OP2`

```rust
const R_XTENSA_OP2: u32 = 10u32;
```

### `R_XTENSA_ASM_EXPAND`

```rust
const R_XTENSA_ASM_EXPAND: u32 = 11u32;
```

### `R_XTENSA_ASM_SIMPLIFY`

```rust
const R_XTENSA_ASM_SIMPLIFY: u32 = 12u32;
```

### `R_XTENSA_32_PCREL`

```rust
const R_XTENSA_32_PCREL: u32 = 14u32;
```

### `R_XTENSA_GNU_VTINHERIT`

```rust
const R_XTENSA_GNU_VTINHERIT: u32 = 15u32;
```

### `R_XTENSA_GNU_VTENTRY`

```rust
const R_XTENSA_GNU_VTENTRY: u32 = 16u32;
```

### `R_XTENSA_DIFF8`

```rust
const R_XTENSA_DIFF8: u32 = 17u32;
```

### `R_XTENSA_DIFF16`

```rust
const R_XTENSA_DIFF16: u32 = 18u32;
```

### `R_XTENSA_DIFF32`

```rust
const R_XTENSA_DIFF32: u32 = 19u32;
```

### `R_XTENSA_SLOT0_OP`

```rust
const R_XTENSA_SLOT0_OP: u32 = 20u32;
```

### `R_XTENSA_SLOT1_OP`

```rust
const R_XTENSA_SLOT1_OP: u32 = 21u32;
```

### `R_XTENSA_SLOT2_OP`

```rust
const R_XTENSA_SLOT2_OP: u32 = 22u32;
```

### `R_XTENSA_SLOT3_OP`

```rust
const R_XTENSA_SLOT3_OP: u32 = 23u32;
```

### `R_XTENSA_SLOT4_OP`

```rust
const R_XTENSA_SLOT4_OP: u32 = 24u32;
```

### `R_XTENSA_SLOT5_OP`

```rust
const R_XTENSA_SLOT5_OP: u32 = 25u32;
```

### `R_XTENSA_SLOT6_OP`

```rust
const R_XTENSA_SLOT6_OP: u32 = 26u32;
```

### `R_XTENSA_SLOT7_OP`

```rust
const R_XTENSA_SLOT7_OP: u32 = 27u32;
```

### `R_XTENSA_SLOT8_OP`

```rust
const R_XTENSA_SLOT8_OP: u32 = 28u32;
```

### `R_XTENSA_SLOT9_OP`

```rust
const R_XTENSA_SLOT9_OP: u32 = 29u32;
```

### `R_XTENSA_SLOT10_OP`

```rust
const R_XTENSA_SLOT10_OP: u32 = 30u32;
```

### `R_XTENSA_SLOT11_OP`

```rust
const R_XTENSA_SLOT11_OP: u32 = 31u32;
```

### `R_XTENSA_SLOT12_OP`

```rust
const R_XTENSA_SLOT12_OP: u32 = 32u32;
```

### `R_XTENSA_SLOT13_OP`

```rust
const R_XTENSA_SLOT13_OP: u32 = 33u32;
```

### `R_XTENSA_SLOT14_OP`

```rust
const R_XTENSA_SLOT14_OP: u32 = 34u32;
```

### `R_XTENSA_SLOT0_ALT`

```rust
const R_XTENSA_SLOT0_ALT: u32 = 35u32;
```

### `R_XTENSA_SLOT1_ALT`

```rust
const R_XTENSA_SLOT1_ALT: u32 = 36u32;
```

### `R_XTENSA_SLOT2_ALT`

```rust
const R_XTENSA_SLOT2_ALT: u32 = 37u32;
```

### `R_XTENSA_SLOT3_ALT`

```rust
const R_XTENSA_SLOT3_ALT: u32 = 38u32;
```

### `R_XTENSA_SLOT4_ALT`

```rust
const R_XTENSA_SLOT4_ALT: u32 = 39u32;
```

### `R_XTENSA_SLOT5_ALT`

```rust
const R_XTENSA_SLOT5_ALT: u32 = 40u32;
```

### `R_XTENSA_SLOT6_ALT`

```rust
const R_XTENSA_SLOT6_ALT: u32 = 41u32;
```

### `R_XTENSA_SLOT7_ALT`

```rust
const R_XTENSA_SLOT7_ALT: u32 = 42u32;
```

### `R_XTENSA_SLOT8_ALT`

```rust
const R_XTENSA_SLOT8_ALT: u32 = 43u32;
```

### `R_XTENSA_SLOT9_ALT`

```rust
const R_XTENSA_SLOT9_ALT: u32 = 44u32;
```

### `R_XTENSA_SLOT10_ALT`

```rust
const R_XTENSA_SLOT10_ALT: u32 = 45u32;
```

### `R_XTENSA_SLOT11_ALT`

```rust
const R_XTENSA_SLOT11_ALT: u32 = 46u32;
```

### `R_XTENSA_SLOT12_ALT`

```rust
const R_XTENSA_SLOT12_ALT: u32 = 47u32;
```

### `R_XTENSA_SLOT13_ALT`

```rust
const R_XTENSA_SLOT13_ALT: u32 = 48u32;
```

### `R_XTENSA_SLOT14_ALT`

```rust
const R_XTENSA_SLOT14_ALT: u32 = 49u32;
```

### `R_XTENSA_TLSDESC_FN`

```rust
const R_XTENSA_TLSDESC_FN: u32 = 50u32;
```

### `R_XTENSA_TLSDESC_ARG`

```rust
const R_XTENSA_TLSDESC_ARG: u32 = 51u32;
```

### `R_XTENSA_TLS_DTPOFF`

```rust
const R_XTENSA_TLS_DTPOFF: u32 = 52u32;
```

### `R_XTENSA_TLS_TPOFF`

```rust
const R_XTENSA_TLS_TPOFF: u32 = 53u32;
```

### `R_XTENSA_TLS_FUNC`

```rust
const R_XTENSA_TLS_FUNC: u32 = 54u32;
```

### `R_XTENSA_TLS_ARG`

```rust
const R_XTENSA_TLS_ARG: u32 = 55u32;
```

### `R_XTENSA_TLS_CALL`

```rust
const R_XTENSA_TLS_CALL: u32 = 56u32;
```

### `R_XTENSA_PDIFF8`

```rust
const R_XTENSA_PDIFF8: u32 = 57u32;
```

### `R_XTENSA_PDIFF16`

```rust
const R_XTENSA_PDIFF16: u32 = 58u32;
```

### `R_XTENSA_PDIFF32`

```rust
const R_XTENSA_PDIFF32: u32 = 59u32;
```

### `R_XTENSA_NDIFF8`

```rust
const R_XTENSA_NDIFF8: u32 = 60u32;
```

### `R_XTENSA_NDIFF16`

```rust
const R_XTENSA_NDIFF16: u32 = 61u32;
```

### `R_XTENSA_NDIFF32`

```rust
const R_XTENSA_NDIFF32: u32 = 62u32;
```

### `EF_E2K_IPD`

```rust
const EF_E2K_IPD: u32 = 3u32;
```

### `EF_E2K_X86APP`

```rust
const EF_E2K_X86APP: u32 = 4u32;
```

### `EF_E2K_4MB_PAGES`

```rust
const EF_E2K_4MB_PAGES: u32 = 8u32;
```

### `EF_E2K_INCOMPAT`

```rust
const EF_E2K_INCOMPAT: u32 = 16u32;
```

### `EF_E2K_PM`

```rust
const EF_E2K_PM: u32 = 32u32;
```

### `EF_E2K_PACK_SEGMENTS`

```rust
const EF_E2K_PACK_SEGMENTS: u32 = 64u32;
```

### `E_E2K_MACH_BASE`

```rust
const E_E2K_MACH_BASE: u32 = 0u32;
```

-march=generic code.

Legacy. Shouldn't be created nowadays.

### `E_E2K_MACH_EV1`

```rust
const E_E2K_MACH_EV1: u32 = 1u32;
```

-march=elbrus-v1 code.

Legacy. Shouldn't be created nowadays.

### `E_E2K_MACH_EV2`

```rust
const E_E2K_MACH_EV2: u32 = 2u32;
```

-march=elbrus-v2 code.

### `E_E2K_MACH_EV3`

```rust
const E_E2K_MACH_EV3: u32 = 3u32;
```

-march=elbrus-v3 code.

### `E_E2K_MACH_EV4`

```rust
const E_E2K_MACH_EV4: u32 = 4u32;
```

-march=elbrus-v4 code.

### `E_E2K_MACH_EV5`

```rust
const E_E2K_MACH_EV5: u32 = 5u32;
```

-march=elbrus-v5 code.

### `E_E2K_MACH_EV6`

```rust
const E_E2K_MACH_EV6: u32 = 6u32;
```

-march=elbrus-v6 code.

### `E_E2K_MACH_EV7`

```rust
const E_E2K_MACH_EV7: u32 = 7u32;
```

-march=elbrus-v7 code.

### `E_E2K_MACH_8C`

```rust
const E_E2K_MACH_8C: u32 = 19u32;
```

-mtune=elbrus-8c code.

### `E_E2K_MACH_1CPLUS`

```rust
const E_E2K_MACH_1CPLUS: u32 = 20u32;
```

-mtune=elbrus-1c+ code.

### `E_E2K_MACH_12C`

```rust
const E_E2K_MACH_12C: u32 = 21u32;
```

-mtune=elbrus-12c code.

### `E_E2K_MACH_16C`

```rust
const E_E2K_MACH_16C: u32 = 22u32;
```

-mtune=elbrus-16c code.

### `E_E2K_MACH_2C3`

```rust
const E_E2K_MACH_2C3: u32 = 23u32;
```

-mtune=elbrus-2c3 code.

### `E_E2K_MACH_48C`

```rust
const E_E2K_MACH_48C: u32 = 24u32;
```

-mtune=elbrus-48c code.

### `E_E2K_MACH_8V7`

```rust
const E_E2K_MACH_8V7: u32 = 25u32;
```

-mtune=elbrus-8v7 code.

### `R_E2K_32_ABS`

```rust
const R_E2K_32_ABS: u32 = 0u32;
```

Direct 32 bit.

### `R_E2K_32_PC`

```rust
const R_E2K_32_PC: u32 = 2u32;
```

PC relative 32 bit.

### `R_E2K_AP_GOT`

```rust
const R_E2K_AP_GOT: u32 = 3u32;
```

32-bit offset of AP GOT entry.

### `R_E2K_PL_GOT`

```rust
const R_E2K_PL_GOT: u32 = 4u32;
```

32-bit offset of PL GOT entry.

### `R_E2K_32_JMP_SLOT`

```rust
const R_E2K_32_JMP_SLOT: u32 = 8u32;
```

Create PLT entry.

### `R_E2K_32_COPY`

```rust
const R_E2K_32_COPY: u32 = 9u32;
```

Copy relocation, 32-bit case.

### `R_E2K_32_RELATIVE`

```rust
const R_E2K_32_RELATIVE: u32 = 10u32;
```

Adjust by program base, 32-bit case.

### `R_E2K_32_IRELATIVE`

```rust
const R_E2K_32_IRELATIVE: u32 = 11u32;
```

Adjust indirectly by program base, 32-bit case.

### `R_E2K_32_SIZE`

```rust
const R_E2K_32_SIZE: u32 = 12u32;
```

Size of symbol plus 32-bit addend.

### `R_E2K_32_DYNOPT`

```rust
const R_E2K_32_DYNOPT: u32 = 13u32;
```

Symbol value if resolved by the definition in the same
compilation unit or NULL otherwise, 32-bit case.

### `R_E2K_64_ABS`

```rust
const R_E2K_64_ABS: u32 = 50u32;
```

Direct 64 bit.

### `R_E2K_64_ABS_LIT`

```rust
const R_E2K_64_ABS_LIT: u32 = 51u32;
```

Direct 64 bit for literal.

### `R_E2K_64_PC_LIT`

```rust
const R_E2K_64_PC_LIT: u32 = 54u32;
```

PC relative 64 bit for literal.

### `R_E2K_64_JMP_SLOT`

```rust
const R_E2K_64_JMP_SLOT: u32 = 63u32;
```

Create PLT entry, 64-bit case.

### `R_E2K_64_COPY`

```rust
const R_E2K_64_COPY: u32 = 64u32;
```

Copy relocation, 64-bit case.

### `R_E2K_64_RELATIVE`

```rust
const R_E2K_64_RELATIVE: u32 = 65u32;
```

Adjust by program base, 64-bit case.

### `R_E2K_64_RELATIVE_LIT`

```rust
const R_E2K_64_RELATIVE_LIT: u32 = 66u32;
```

Adjust by program base for literal, 64-bit case.

### `R_E2K_64_IRELATIVE`

```rust
const R_E2K_64_IRELATIVE: u32 = 67u32;
```

Adjust indirectly by program base, 64-bit case.

### `R_E2K_64_SIZE`

```rust
const R_E2K_64_SIZE: u32 = 68u32;
```

Size of symbol plus 64-bit addend.

### `R_E2K_64_GOTOFF`

```rust
const R_E2K_64_GOTOFF: u32 = 69u32;
```

64-bit offset of the symbol from GOT.

### `R_E2K_TLS_GDMOD`

```rust
const R_E2K_TLS_GDMOD: u32 = 70u32;
```

GOT entry for ID of module containing symbol.

### `R_E2K_TLS_GDREL`

```rust
const R_E2K_TLS_GDREL: u32 = 71u32;
```

GOT entry for offset in module TLS block.

### `R_E2K_TLS_IE`

```rust
const R_E2K_TLS_IE: u32 = 74u32;
```

Static TLS block offset GOT entry.

### `R_E2K_32_TLS_LE`

```rust
const R_E2K_32_TLS_LE: u32 = 75u32;
```

Offset relative to static TLS block, 32-bit case.

### `R_E2K_64_TLS_LE`

```rust
const R_E2K_64_TLS_LE: u32 = 76u32;
```

Offset relative to static TLS block, 64-bit case.

### `R_E2K_TLS_32_DTPMOD`

```rust
const R_E2K_TLS_32_DTPMOD: u32 = 80u32;
```

ID of module containing symbol, 32-bit case.

### `R_E2K_TLS_32_DTPREL`

```rust
const R_E2K_TLS_32_DTPREL: u32 = 81u32;
```

Offset in module TLS block, 32-bit case.

### `R_E2K_TLS_64_DTPMOD`

```rust
const R_E2K_TLS_64_DTPMOD: u32 = 82u32;
```

ID of module containing symbol, 64-bit case.

### `R_E2K_TLS_64_DTPREL`

```rust
const R_E2K_TLS_64_DTPREL: u32 = 83u32;
```

Offset in module TLS block, 64-bit case.

### `R_E2K_TLS_32_TPREL`

```rust
const R_E2K_TLS_32_TPREL: u32 = 84u32;
```

Offset in static TLS block, 32-bit case.

### `R_E2K_TLS_64_TPREL`

```rust
const R_E2K_TLS_64_TPREL: u32 = 85u32;
```

Offset in static TLS block, 64-bit case.

### `R_E2K_AP`

```rust
const R_E2K_AP: u32 = 100u32;
```

Direct AP.

### `R_E2K_PL`

```rust
const R_E2K_PL: u32 = 101u32;
```

Direct PL.

### `R_E2K_GOT`

```rust
const R_E2K_GOT: u32 = 108u32;
```

32-bit offset of the symbol's entry in GOT.

### `R_E2K_GOTOFF`

```rust
const R_E2K_GOTOFF: u32 = 109u32;
```

32-bit offset of the symbol from GOT.

### `R_E2K_DISP`

```rust
const R_E2K_DISP: u32 = 110u32;
```

PC relative 28 bit for DISP.

### `R_E2K_PREF`

```rust
const R_E2K_PREF: u32 = 111u32;
```

Prefetch insn line containing the label (symbol).

### `R_E2K_NONE`

```rust
const R_E2K_NONE: u32 = 112u32;
```

No reloc.

### `R_E2K_GOTPLT`

```rust
const R_E2K_GOTPLT: u32 = 114u32;
```

32-bit offset of the symbol's entry in .got.plt.

### `R_E2K_ISLOCAL`

```rust
const R_E2K_ISLOCAL: u32 = 115u32;
```

Is symbol resolved locally during the link.
The result is encoded in 5-bit ALS.src1.

### `R_E2K_ISLOCAL32`

```rust
const R_E2K_ISLOCAL32: u32 = 118u32;
```

Is symbol resloved locally during the link.
The result is encoded in a long 32-bit LTS.

### `R_E2K_64_GOTOFF_LIT`

```rust
const R_E2K_64_GOTOFF_LIT: u32 = 256u32;
```

The symbol's offset from GOT encoded within a 64-bit literal.

### `R_E2K_64_DYNOPT`

```rust
const R_E2K_64_DYNOPT: u32 = 257u32;
```

Symbol value if resolved by the definition in the same
compilation unit or NULL otherwise, 64-bit case.

### `R_E2K_64_PC`

```rust
const R_E2K_64_PC: u32 = 258u32;
```

PC relative 64 bit in data.

### `DT_E2K_LAZY`

```rust
const DT_E2K_LAZY: u32 = 1_879_048_193u32;
```

### `DT_E2K_LAZY_GOT`

```rust
const DT_E2K_LAZY_GOT: u32 = 1_879_048_195u32;
```

### `DT_E2K_INIT_GOT`

```rust
const DT_E2K_INIT_GOT: u32 = 1_879_052_316u32;
```

### `DT_E2K_EXPORT_PL`

```rust
const DT_E2K_EXPORT_PL: u32 = 1_879_052_317u32;
```

### `DT_E2K_EXPORT_PLSZ`

```rust
const DT_E2K_EXPORT_PLSZ: u32 = 1_879_052_318u32;
```

### `DT_E2K_REAL_PLTGOT`

```rust
const DT_E2K_REAL_PLTGOT: u32 = 1_879_052_319u32;
```

### `DT_E2K_NO_SELFINIT`

```rust
const DT_E2K_NO_SELFINIT: u32 = 1_879_052_320u32;
```

### `DT_E2K_NUM`

```rust
const DT_E2K_NUM: u32 = 4_129u32;
```

### `Tag_File`

```rust
const Tag_File: u8 = 1u8;
```

### `Tag_Section`

```rust
const Tag_Section: u8 = 2u8;
```

### `Tag_Symbol`

```rust
const Tag_Symbol: u8 = 3u8;
```

