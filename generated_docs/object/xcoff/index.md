*[object](../index.md) / [xcoff](index.md)*

---

# Module `xcoff`

XCOFF definitions

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is the equivalent of /usr/include/xcoff.h, and is based heavily on it.

## Structs

### `FileHeader32`

```rust
struct FileHeader32 {
    pub f_magic: crate::endian::U16<crate::endian::BigEndian>,
    pub f_nscns: crate::endian::U16<crate::endian::BigEndian>,
    pub f_timdat: crate::endian::U32<crate::endian::BigEndian>,
    pub f_symptr: crate::endian::U32<crate::endian::BigEndian>,
    pub f_nsyms: crate::endian::U32<crate::endian::BigEndian>,
    pub f_opthdr: crate::endian::U16<crate::endian::BigEndian>,
    pub f_flags: crate::endian::U16<crate::endian::BigEndian>,
}
```

The header at the start of every 32-bit XCOFF file.

#### Fields

- **`f_magic`**: `crate::endian::U16<crate::endian::BigEndian>`

  Magic number. Must be 0x01DF.

- **`f_nscns`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of sections.

- **`f_timdat`**: `crate::endian::U32<crate::endian::BigEndian>`

  Time and date of file creation.

- **`f_symptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Byte offset to symbol table start.

- **`f_nsyms`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of entries in symbol table.

- **`f_opthdr`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of bytes in optional header

- **`f_flags`**: `crate::endian::U16<crate::endian::BigEndian>`

  Extra flags.

#### Trait Implementations

##### `impl Clone for FileHeader32`

- `fn clone(self: &Self) -> FileHeader32` — [`FileHeader32`](#fileheader32)

##### `impl Copy for FileHeader32`

##### `impl Debug for FileHeader32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FileHeader for xcoff::FileHeader32`

- `type Word = u32`

- `type AuxHeader = AuxHeader32`

- `type SectionHeader = SectionHeader32`

- `type Symbol = Symbol32`

- `type FileAux = FileAux32`

- `type CsectAux = CsectAux32`

- `type Rel = Rel32`

- `fn is_type_64(self: &Self) -> bool`

- `fn f_magic(self: &Self) -> u16`

- `fn f_nscns(self: &Self) -> u16`

- `fn f_timdat(self: &Self) -> u32`

- `fn f_symptr(self: &Self) -> <Self as >::Word` — [`FileHeader`](../read/xcoff/index.md)

- `fn f_nsyms(self: &Self) -> u32`

- `fn f_opthdr(self: &Self) -> u16`

- `fn f_flags(self: &Self) -> u16`

##### `impl Pod for FileHeader32`

### `FileHeader64`

```rust
struct FileHeader64 {
    pub f_magic: crate::endian::U16<crate::endian::BigEndian>,
    pub f_nscns: crate::endian::U16<crate::endian::BigEndian>,
    pub f_timdat: crate::endian::U32<crate::endian::BigEndian>,
    pub f_symptr: crate::endian::U64<crate::endian::BigEndian>,
    pub f_opthdr: crate::endian::U16<crate::endian::BigEndian>,
    pub f_flags: crate::endian::U16<crate::endian::BigEndian>,
    pub f_nsyms: crate::endian::U32<crate::endian::BigEndian>,
}
```

The header at the start of every 64-bit XCOFF file.

#### Fields

- **`f_magic`**: `crate::endian::U16<crate::endian::BigEndian>`

  Magic number. Must be 0x01F7.

- **`f_nscns`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of sections.

- **`f_timdat`**: `crate::endian::U32<crate::endian::BigEndian>`

  Time and date of file creation

- **`f_symptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Byte offset to symbol table start.

- **`f_opthdr`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of bytes in optional header

- **`f_flags`**: `crate::endian::U16<crate::endian::BigEndian>`

  Extra flags.

- **`f_nsyms`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of entries in symbol table.

#### Trait Implementations

##### `impl Clone for FileHeader64`

- `fn clone(self: &Self) -> FileHeader64` — [`FileHeader64`](#fileheader64)

##### `impl Copy for FileHeader64`

##### `impl Debug for FileHeader64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FileHeader for xcoff::FileHeader64`

- `type Word = u64`

- `type AuxHeader = AuxHeader64`

- `type SectionHeader = SectionHeader64`

- `type Symbol = Symbol64`

- `type FileAux = FileAux64`

- `type CsectAux = CsectAux64`

- `type Rel = Rel64`

- `fn is_type_64(self: &Self) -> bool`

- `fn f_magic(self: &Self) -> u16`

- `fn f_nscns(self: &Self) -> u16`

- `fn f_timdat(self: &Self) -> u32`

- `fn f_symptr(self: &Self) -> <Self as >::Word` — [`FileHeader`](../read/xcoff/index.md)

- `fn f_nsyms(self: &Self) -> u32`

- `fn f_opthdr(self: &Self) -> u16`

- `fn f_flags(self: &Self) -> u16`

##### `impl Pod for FileHeader64`

### `AuxHeader32`

```rust
struct AuxHeader32 {
    pub o_mflag: crate::endian::U16<crate::endian::BigEndian>,
    pub o_vstamp: crate::endian::U16<crate::endian::BigEndian>,
    pub o_tsize: crate::endian::U32<crate::endian::BigEndian>,
    pub o_dsize: crate::endian::U32<crate::endian::BigEndian>,
    pub o_bsize: crate::endian::U32<crate::endian::BigEndian>,
    pub o_entry: crate::endian::U32<crate::endian::BigEndian>,
    pub o_text_start: crate::endian::U32<crate::endian::BigEndian>,
    pub o_data_start: crate::endian::U32<crate::endian::BigEndian>,
    pub o_toc: crate::endian::U32<crate::endian::BigEndian>,
    pub o_snentry: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntoc: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snloader: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snbss: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_modtype: crate::endian::U16<crate::endian::BigEndian>,
    pub o_cpuflag: u8,
    pub o_cputype: u8,
    pub o_maxstack: crate::endian::U32<crate::endian::BigEndian>,
    pub o_maxdata: crate::endian::U32<crate::endian::BigEndian>,
    pub o_debugger: crate::endian::U32<crate::endian::BigEndian>,
    pub o_textpsize: u8,
    pub o_datapsize: u8,
    pub o_stackpsize: u8,
    pub o_flags: u8,
    pub o_sntdata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntbss: crate::endian::U16<crate::endian::BigEndian>,
}
```

The auxiliary header immediately following file header. If the value of the
f_opthdr field in the file header is 0, the auxiliary header does not exist.

#### Fields

- **`o_mflag`**: `crate::endian::U16<crate::endian::BigEndian>`

  Flags.

- **`o_vstamp`**: `crate::endian::U16<crate::endian::BigEndian>`

  Version.

- **`o_tsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Text size in bytes.

- **`o_dsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Initialized data size in bytes.

- **`o_bsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Uninitialized data size in bytes.

- **`o_entry`**: `crate::endian::U32<crate::endian::BigEndian>`

  Entry point descriptor (virtual address).

- **`o_text_start`**: `crate::endian::U32<crate::endian::BigEndian>`

  Base address of text (virtual address).

- **`o_data_start`**: `crate::endian::U32<crate::endian::BigEndian>`

  Base address of data (virtual address).

- **`o_toc`**: `crate::endian::U32<crate::endian::BigEndian>`

  Address of TOC anchor.

- **`o_snentry`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for entry point.

- **`o_sntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .text.

- **`o_sndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .data.

- **`o_sntoc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for TOC.

- **`o_snloader`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for loader data.

- **`o_snbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .bss.

- **`o_algntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .text.

- **`o_algndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .data.

- **`o_modtype`**: `crate::endian::U16<crate::endian::BigEndian>`

  Module type field.

- **`o_cpuflag`**: `u8`

  Bit flags - cpu types of objects.

- **`o_cputype`**: `u8`

  Reserved for CPU type.

- **`o_maxstack`**: `crate::endian::U32<crate::endian::BigEndian>`

  Maximum stack size allowed (bytes).

- **`o_maxdata`**: `crate::endian::U32<crate::endian::BigEndian>`

  Maximum data size allowed (bytes).

- **`o_debugger`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved for debuggers.

- **`o_textpsize`**: `u8`

  Requested text page size.

- **`o_datapsize`**: `u8`

  Requested data page size.

- **`o_stackpsize`**: `u8`

  Requested stack page size.

- **`o_flags`**: `u8`

  Flags and thread-local storage alignment.

- **`o_sntdata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tdata.

- **`o_sntbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tbss.

#### Trait Implementations

##### `impl AuxHeader for xcoff::AuxHeader32`

- `type Word = u32`

- `fn o_mflag(self: &Self) -> u16`

- `fn o_vstamp(self: &Self) -> u16`

- `fn o_tsize(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_dsize(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_bsize(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_entry(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_text_start(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_data_start(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_toc(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_snentry(self: &Self) -> u16`

- `fn o_sntext(self: &Self) -> u16`

- `fn o_sndata(self: &Self) -> u16`

- `fn o_sntoc(self: &Self) -> u16`

- `fn o_snloader(self: &Self) -> u16`

- `fn o_snbss(self: &Self) -> u16`

- `fn o_algntext(self: &Self) -> u16`

- `fn o_algndata(self: &Self) -> u16`

- `fn o_modtype(self: &Self) -> u16`

- `fn o_cpuflag(self: &Self) -> u8`

- `fn o_cputype(self: &Self) -> u8`

- `fn o_maxstack(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_maxdata(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_debugger(self: &Self) -> u32`

- `fn o_textpsize(self: &Self) -> u8`

- `fn o_datapsize(self: &Self) -> u8`

- `fn o_stackpsize(self: &Self) -> u8`

- `fn o_flags(self: &Self) -> u8`

- `fn o_sntdata(self: &Self) -> u16`

- `fn o_sntbss(self: &Self) -> u16`

- `fn o_x64flags(self: &Self) -> Option<u16>`

##### `impl Clone for AuxHeader32`

- `fn clone(self: &Self) -> AuxHeader32` — [`AuxHeader32`](#auxheader32)

##### `impl Copy for AuxHeader32`

##### `impl Debug for AuxHeader32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for AuxHeader32`

### `AuxHeader64`

```rust
struct AuxHeader64 {
    pub o_mflag: crate::endian::U16<crate::endian::BigEndian>,
    pub o_vstamp: crate::endian::U16<crate::endian::BigEndian>,
    pub o_debugger: crate::endian::U32<crate::endian::BigEndian>,
    pub o_text_start: crate::endian::U64<crate::endian::BigEndian>,
    pub o_data_start: crate::endian::U64<crate::endian::BigEndian>,
    pub o_toc: crate::endian::U64<crate::endian::BigEndian>,
    pub o_snentry: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntoc: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snloader: crate::endian::U16<crate::endian::BigEndian>,
    pub o_snbss: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algntext: crate::endian::U16<crate::endian::BigEndian>,
    pub o_algndata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_modtype: crate::endian::U16<crate::endian::BigEndian>,
    pub o_cpuflag: u8,
    pub o_cputype: u8,
    pub o_textpsize: u8,
    pub o_datapsize: u8,
    pub o_stackpsize: u8,
    pub o_flags: u8,
    pub o_tsize: crate::endian::U64<crate::endian::BigEndian>,
    pub o_dsize: crate::endian::U64<crate::endian::BigEndian>,
    pub o_bsize: crate::endian::U64<crate::endian::BigEndian>,
    pub o_entry: crate::endian::U64<crate::endian::BigEndian>,
    pub o_maxstack: crate::endian::U64<crate::endian::BigEndian>,
    pub o_maxdata: crate::endian::U64<crate::endian::BigEndian>,
    pub o_sntdata: crate::endian::U16<crate::endian::BigEndian>,
    pub o_sntbss: crate::endian::U16<crate::endian::BigEndian>,
    pub o_x64flags: crate::endian::U16<crate::endian::BigEndian>,
    pub o_resv3a: crate::endian::U16<crate::endian::BigEndian>,
    pub o_resv3: [crate::endian::U32<crate::endian::BigEndian>; 2],
}
```

The auxiliary header immediately following file header. If the value of the
f_opthdr field in the file header is 0, the auxiliary header does not exist.

#### Fields

- **`o_mflag`**: `crate::endian::U16<crate::endian::BigEndian>`

  Flags.

- **`o_vstamp`**: `crate::endian::U16<crate::endian::BigEndian>`

  Version.

- **`o_debugger`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved for debuggers.

- **`o_text_start`**: `crate::endian::U64<crate::endian::BigEndian>`

  Base address of text (virtual address).

- **`o_data_start`**: `crate::endian::U64<crate::endian::BigEndian>`

  Base address of data (virtual address).

- **`o_toc`**: `crate::endian::U64<crate::endian::BigEndian>`

  Address of TOC anchor.

- **`o_snentry`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for entry point.

- **`o_sntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .text.

- **`o_sndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .data.

- **`o_sntoc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for TOC.

- **`o_snloader`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for loader data.

- **`o_snbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .bss.

- **`o_algntext`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .text.

- **`o_algndata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Maximum alignment for .data.

- **`o_modtype`**: `crate::endian::U16<crate::endian::BigEndian>`

  Module type field.

- **`o_cpuflag`**: `u8`

  Bit flags - cpu types of objects.

- **`o_cputype`**: `u8`

  Reserved for CPU type.

- **`o_textpsize`**: `u8`

  Requested text page size.

- **`o_datapsize`**: `u8`

  Requested data page size.

- **`o_stackpsize`**: `u8`

  Requested stack page size.

- **`o_flags`**: `u8`

  Flags and thread-local storage alignment.

- **`o_tsize`**: `crate::endian::U64<crate::endian::BigEndian>`

  Text size in bytes.

- **`o_dsize`**: `crate::endian::U64<crate::endian::BigEndian>`

  Initialized data size in bytes.

- **`o_bsize`**: `crate::endian::U64<crate::endian::BigEndian>`

  Uninitialized data size in bytes.

- **`o_entry`**: `crate::endian::U64<crate::endian::BigEndian>`

  Entry point descriptor (virtual address).

- **`o_maxstack`**: `crate::endian::U64<crate::endian::BigEndian>`

  Maximum stack size allowed (bytes).

- **`o_maxdata`**: `crate::endian::U64<crate::endian::BigEndian>`

  Maximum data size allowed (bytes).

- **`o_sntdata`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tdata.

- **`o_sntbss`**: `crate::endian::U16<crate::endian::BigEndian>`

  Section number for .tbss.

- **`o_x64flags`**: `crate::endian::U16<crate::endian::BigEndian>`

  XCOFF64 flags.

- **`o_resv3a`**: `crate::endian::U16<crate::endian::BigEndian>`

  Reserved.

- **`o_resv3`**: `[crate::endian::U32<crate::endian::BigEndian>; 2]`

  Reserved.

#### Trait Implementations

##### `impl AuxHeader for xcoff::AuxHeader64`

- `type Word = u64`

- `fn o_mflag(self: &Self) -> u16`

- `fn o_vstamp(self: &Self) -> u16`

- `fn o_tsize(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_dsize(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_bsize(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_entry(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_text_start(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_data_start(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_toc(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_snentry(self: &Self) -> u16`

- `fn o_sntext(self: &Self) -> u16`

- `fn o_sndata(self: &Self) -> u16`

- `fn o_sntoc(self: &Self) -> u16`

- `fn o_snloader(self: &Self) -> u16`

- `fn o_snbss(self: &Self) -> u16`

- `fn o_algntext(self: &Self) -> u16`

- `fn o_algndata(self: &Self) -> u16`

- `fn o_modtype(self: &Self) -> u16`

- `fn o_cpuflag(self: &Self) -> u8`

- `fn o_cputype(self: &Self) -> u8`

- `fn o_maxstack(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_maxdata(self: &Self) -> <Self as >::Word` — [`AuxHeader`](../read/xcoff/index.md)

- `fn o_debugger(self: &Self) -> u32`

- `fn o_textpsize(self: &Self) -> u8`

- `fn o_datapsize(self: &Self) -> u8`

- `fn o_stackpsize(self: &Self) -> u8`

- `fn o_flags(self: &Self) -> u8`

- `fn o_sntdata(self: &Self) -> u16`

- `fn o_sntbss(self: &Self) -> u16`

- `fn o_x64flags(self: &Self) -> Option<u16>`

##### `impl Clone for AuxHeader64`

- `fn clone(self: &Self) -> AuxHeader64` — [`AuxHeader64`](#auxheader64)

##### `impl Copy for AuxHeader64`

##### `impl Debug for AuxHeader64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for AuxHeader64`

### `SectionHeader32`

```rust
struct SectionHeader32 {
    pub s_name: [u8; 8],
    pub s_paddr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_vaddr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_size: crate::endian::U32<crate::endian::BigEndian>,
    pub s_scnptr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_relptr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_lnnoptr: crate::endian::U32<crate::endian::BigEndian>,
    pub s_nreloc: crate::endian::U16<crate::endian::BigEndian>,
    pub s_nlnno: crate::endian::U16<crate::endian::BigEndian>,
    pub s_flags: crate::endian::U32<crate::endian::BigEndian>,
}
```

Section header.

#### Fields

- **`s_name`**: `[u8; 8]`

  Section name.

- **`s_paddr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Physical address.

- **`s_vaddr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Virtual address (same as physical address).

- **`s_size`**: `crate::endian::U32<crate::endian::BigEndian>`

  Section size.

- **`s_scnptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset in file to raw data for section.

- **`s_relptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset in file to relocation entries for section.

- **`s_lnnoptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset in file to line number entries for section.

- **`s_nreloc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of relocation entries.

- **`s_nlnno`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of line number entries.

- **`s_flags`**: `crate::endian::U32<crate::endian::BigEndian>`

  Flags to define the section type.

#### Trait Implementations

##### `impl Clone for SectionHeader32`

- `fn clone(self: &Self) -> SectionHeader32` — [`SectionHeader32`](#sectionheader32)

##### `impl Copy for SectionHeader32`

##### `impl Debug for SectionHeader32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for SectionHeader32`

##### `impl SectionHeader for xcoff::SectionHeader32`

- `type Word = u32`

- `type HalfWord = u16`

- `type Xcoff = FileHeader32`

- `type Rel = Rel32`

- `fn s_name(self: &Self) -> &[u8; 8]`

- `fn s_paddr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_vaddr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_size(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_scnptr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_relptr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_lnnoptr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_nreloc(self: &Self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_nlnno(self: &Self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_flags(self: &Self) -> u32`

- `fn relocations<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<&'data [<Self as >::Rel]>` — [`Result`](../index.md), [`SectionHeader`](../read/xcoff/index.md)

### `SectionHeader64`

```rust
struct SectionHeader64 {
    pub s_name: [u8; 8],
    pub s_paddr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_vaddr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_size: crate::endian::U64<crate::endian::BigEndian>,
    pub s_scnptr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_relptr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_lnnoptr: crate::endian::U64<crate::endian::BigEndian>,
    pub s_nreloc: crate::endian::U32<crate::endian::BigEndian>,
    pub s_nlnno: crate::endian::U32<crate::endian::BigEndian>,
    pub s_flags: crate::endian::U32<crate::endian::BigEndian>,
    pub s_reserve: crate::endian::U32<crate::endian::BigEndian>,
}
```

Section header.

#### Fields

- **`s_name`**: `[u8; 8]`

  Section name.

- **`s_paddr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Physical address.

- **`s_vaddr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Virtual address (same as physical address).

- **`s_size`**: `crate::endian::U64<crate::endian::BigEndian>`

  Section size.

- **`s_scnptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Offset in file to raw data for section.

- **`s_relptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Offset in file to relocation entries for section.

- **`s_lnnoptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Offset in file to line number entries for section.

- **`s_nreloc`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of relocation entries.

- **`s_nlnno`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of line number entries.

- **`s_flags`**: `crate::endian::U32<crate::endian::BigEndian>`

  Flags to define the section type.

- **`s_reserve`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved.

#### Trait Implementations

##### `impl Clone for SectionHeader64`

- `fn clone(self: &Self) -> SectionHeader64` — [`SectionHeader64`](#sectionheader64)

##### `impl Copy for SectionHeader64`

##### `impl Debug for SectionHeader64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for SectionHeader64`

##### `impl SectionHeader for xcoff::SectionHeader64`

- `type Word = u64`

- `type HalfWord = u32`

- `type Xcoff = FileHeader64`

- `type Rel = Rel64`

- `fn s_name(self: &Self) -> &[u8; 8]`

- `fn s_paddr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_vaddr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_size(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_scnptr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_relptr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_lnnoptr(self: &Self) -> <Self as >::Word` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_nreloc(self: &Self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_nlnno(self: &Self) -> <Self as >::HalfWord` — [`SectionHeader`](../read/xcoff/index.md)

- `fn s_flags(self: &Self) -> u32`

- `fn relocations<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<&'data [<Self as >::Rel]>` — [`Result`](../index.md), [`SectionHeader`](../read/xcoff/index.md)

### `SymbolBytes`

```rust
struct SymbolBytes([u8; 18]);
```

#### Trait Implementations

##### `impl Clone for SymbolBytes`

- `fn clone(self: &Self) -> SymbolBytes` — [`SymbolBytes`](#symbolbytes)

##### `impl Copy for SymbolBytes`

##### `impl Debug for SymbolBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for SymbolBytes`

### `Symbol32`

```rust
struct Symbol32 {
    pub n_name: [u8; 8],
    pub n_value: crate::endian::U32<crate::endian::BigEndian>,
    pub n_scnum: crate::endian::I16<crate::endian::BigEndian>,
    pub n_type: crate::endian::U16<crate::endian::BigEndian>,
    pub n_sclass: u8,
    pub n_numaux: u8,
}
```

Symbol table entry.

#### Fields

- **`n_name`**: `[u8; 8]`

  Symbol name.
  
  If first 4 bytes are 0, then second 4 bytes are offset into string table.

- **`n_value`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol value; storage class-dependent.

- **`n_scnum`**: `crate::endian::I16<crate::endian::BigEndian>`

  Section number of symbol.

- **`n_type`**: `crate::endian::U16<crate::endian::BigEndian>`

  Basic and derived type specification.

- **`n_sclass`**: `u8`

  Storage class of symbol.

- **`n_numaux`**: `u8`

  Number of auxiliary entries.

#### Trait Implementations

##### `impl Clone for Symbol32`

- `fn clone(self: &Self) -> Symbol32` — [`Symbol32`](#symbol32)

##### `impl Copy for Symbol32`

##### `impl Debug for Symbol32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for Symbol32`

##### `impl Symbol for xcoff::Symbol32`

- `type Word = u32`

- `fn n_value(self: &Self) -> <Self as >::Word` — [`Symbol`](../read/xcoff/index.md)

- `fn n_scnum(self: &Self) -> i16`

- `fn n_type(self: &Self) -> u16`

- `fn n_sclass(self: &Self) -> u8`

- `fn n_numaux(self: &Self) -> u8`

- `fn name_offset(self: &Self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>` — [`StringTable`](../read/index.md), [`Result`](../index.md)

### `Symbol64`

```rust
struct Symbol64 {
    pub n_value: crate::endian::U64<crate::endian::BigEndian>,
    pub n_offset: crate::endian::U32<crate::endian::BigEndian>,
    pub n_scnum: crate::endian::I16<crate::endian::BigEndian>,
    pub n_type: crate::endian::U16<crate::endian::BigEndian>,
    pub n_sclass: u8,
    pub n_numaux: u8,
}
```

Symbol table entry.

#### Fields

- **`n_value`**: `crate::endian::U64<crate::endian::BigEndian>`

  Symbol value; storage class-dependent.

- **`n_offset`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset of the name in string table or .debug section.

- **`n_scnum`**: `crate::endian::I16<crate::endian::BigEndian>`

  Section number of symbol.

- **`n_type`**: `crate::endian::U16<crate::endian::BigEndian>`

  Basic and derived type specification.

- **`n_sclass`**: `u8`

  Storage class of symbol.

- **`n_numaux`**: `u8`

  Number of auxiliary entries.

#### Trait Implementations

##### `impl Clone for Symbol64`

- `fn clone(self: &Self) -> Symbol64` — [`Symbol64`](#symbol64)

##### `impl Copy for Symbol64`

##### `impl Debug for Symbol64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for Symbol64`

##### `impl Symbol for xcoff::Symbol64`

- `type Word = u64`

- `fn n_value(self: &Self) -> <Self as >::Word` — [`Symbol`](../read/xcoff/index.md)

- `fn n_scnum(self: &Self) -> i16`

- `fn n_type(self: &Self) -> u16`

- `fn n_sclass(self: &Self) -> u8`

- `fn n_numaux(self: &Self) -> u8`

- `fn name_offset(self: &Self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>` — [`StringTable`](../read/index.md), [`Result`](../index.md)

### `FileAux32`

```rust
struct FileAux32 {
    pub x_fname: [u8; 8],
    pub x_fpad: [u8; 6],
    pub x_ftype: u8,
    pub x_freserve: [u8; 3],
}
```

File Auxiliary Entry for C_FILE Symbols.

#### Fields

- **`x_fname`**: `[u8; 8]`

  The source file name or compiler-related string.
  
  If first 4 bytes are 0, then second 4 bytes are offset into string table.

- **`x_fpad`**: `[u8; 6]`

  Pad size for file name.

- **`x_ftype`**: `u8`

  The source-file string type.

- **`x_freserve`**: `[u8; 3]`

  Reserved.

#### Trait Implementations

##### `impl Clone for FileAux32`

- `fn clone(self: &Self) -> FileAux32` — [`FileAux32`](#fileaux32)

##### `impl Copy for FileAux32`

##### `impl Debug for FileAux32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FileAux for xcoff::FileAux32`

- `fn x_fname(self: &Self) -> &[u8; 8]`

- `fn x_ftype(self: &Self) -> u8`

- `fn x_auxtype(self: &Self) -> Option<u8>`

##### `impl Pod for FileAux32`

### `FileAux64`

```rust
struct FileAux64 {
    pub x_fname: [u8; 8],
    pub x_fpad: [u8; 6],
    pub x_ftype: u8,
    pub x_freserve: [u8; 2],
    pub x_auxtype: u8,
}
```

File Auxiliary Entry for C_FILE Symbols.

#### Fields

- **`x_fname`**: `[u8; 8]`

  The source file name or compiler-related string.
  
  If first 4 bytes are 0, then second 4 bytes are offset into string table.

- **`x_fpad`**: `[u8; 6]`

  Pad size for file name.

- **`x_ftype`**: `u8`

  The source-file string type.

- **`x_freserve`**: `[u8; 2]`

  Reserved.

- **`x_auxtype`**: `u8`

  Specifies the type of auxiliary entry. Contains _AUX_FILE for this auxiliary entry.

#### Trait Implementations

##### `impl Clone for FileAux64`

- `fn clone(self: &Self) -> FileAux64` — [`FileAux64`](#fileaux64)

##### `impl Copy for FileAux64`

##### `impl Debug for FileAux64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FileAux for xcoff::FileAux64`

- `fn x_fname(self: &Self) -> &[u8; 8]`

- `fn x_ftype(self: &Self) -> u8`

- `fn x_auxtype(self: &Self) -> Option<u8>`

##### `impl Pod for FileAux64`

### `CsectAux32`

```rust
struct CsectAux32 {
    pub x_scnlen: crate::endian::U32<crate::endian::BigEndian>,
    pub x_parmhash: crate::endian::U32<crate::endian::BigEndian>,
    pub x_snhash: crate::endian::U16<crate::endian::BigEndian>,
    pub x_smtyp: u8,
    pub x_smclas: u8,
    pub x_stab: crate::endian::U32<crate::endian::BigEndian>,
    pub x_snstab: crate::endian::U16<crate::endian::BigEndian>,
}
```

Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.

#### Fields

- **`x_scnlen`**: `crate::endian::U32<crate::endian::BigEndian>`

  Section length.

- **`x_parmhash`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset of parameter type-check hash in .typchk section.

- **`x_snhash`**: `crate::endian::U16<crate::endian::BigEndian>`

  .typchk section number.

- **`x_smtyp`**: `u8`

  Symbol alignment and type.

- **`x_smclas`**: `u8`

  Storage mapping class.

- **`x_stab`**: `crate::endian::U32<crate::endian::BigEndian>`

  Reserved.

- **`x_snstab`**: `crate::endian::U16<crate::endian::BigEndian>`

  x_snstab.

#### Trait Implementations

##### `impl Clone for CsectAux32`

- `fn clone(self: &Self) -> CsectAux32` — [`CsectAux32`](#csectaux32)

##### `impl Copy for CsectAux32`

##### `impl CsectAux for xcoff::CsectAux32`

- `fn x_scnlen(self: &Self) -> u64`

- `fn x_parmhash(self: &Self) -> u32`

- `fn x_snhash(self: &Self) -> u16`

- `fn x_smtyp(self: &Self) -> u8`

- `fn x_smclas(self: &Self) -> u8`

- `fn x_stab(self: &Self) -> Option<u32>`

- `fn x_snstab(self: &Self) -> Option<u16>`

- `fn x_auxtype(self: &Self) -> Option<u8>`

##### `impl Debug for CsectAux32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for CsectAux32`

### `CsectAux64`

```rust
struct CsectAux64 {
    pub x_scnlen_lo: crate::endian::U32<crate::endian::BigEndian>,
    pub x_parmhash: crate::endian::U32<crate::endian::BigEndian>,
    pub x_snhash: crate::endian::U16<crate::endian::BigEndian>,
    pub x_smtyp: u8,
    pub x_smclas: u8,
    pub x_scnlen_hi: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

Csect auxiliary entry for C_EXT, C_WEAKEXT, and C_HIDEXT symbols.

#### Fields

- **`x_scnlen_lo`**: `crate::endian::U32<crate::endian::BigEndian>`

  Low 4 bytes of section length.

- **`x_parmhash`**: `crate::endian::U32<crate::endian::BigEndian>`

  Offset of parameter type-check hash in .typchk section.

- **`x_snhash`**: `crate::endian::U16<crate::endian::BigEndian>`

  .typchk section number.

- **`x_smtyp`**: `u8`

  Symbol alignment and type.

- **`x_smclas`**: `u8`

  Storage mapping class.

- **`x_scnlen_hi`**: `crate::endian::U32<crate::endian::BigEndian>`

  High 4 bytes of section length.

- **`pad`**: `u8`

  Reserved.

- **`x_auxtype`**: `u8`

  Contains _AUX_CSECT; indicates type of auxiliary entry.

#### Trait Implementations

##### `impl Clone for CsectAux64`

- `fn clone(self: &Self) -> CsectAux64` — [`CsectAux64`](#csectaux64)

##### `impl Copy for CsectAux64`

##### `impl CsectAux for xcoff::CsectAux64`

- `fn x_scnlen(self: &Self) -> u64`

- `fn x_parmhash(self: &Self) -> u32`

- `fn x_snhash(self: &Self) -> u16`

- `fn x_smtyp(self: &Self) -> u8`

- `fn x_smclas(self: &Self) -> u8`

- `fn x_stab(self: &Self) -> Option<u32>`

- `fn x_snstab(self: &Self) -> Option<u16>`

- `fn x_auxtype(self: &Self) -> Option<u8>`

##### `impl Debug for CsectAux64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for CsectAux64`

### `FunAux32`

```rust
struct FunAux32 {
    pub x_exptr: crate::endian::U32<crate::endian::BigEndian>,
    pub x_fsize: crate::endian::U32<crate::endian::BigEndian>,
    pub x_lnnoptr: crate::endian::U32<crate::endian::BigEndian>,
    pub x_endndx: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: crate::endian::U16<crate::endian::BigEndian>,
}
```

Function auxiliary entry.

#### Fields

- **`x_exptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  File offset to exception table entry.

- **`x_fsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Size of function in bytes.

- **`x_lnnoptr`**: `crate::endian::U32<crate::endian::BigEndian>`

  File pointer to line number

- **`x_endndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of next entry beyond this function.

- **`pad`**: `crate::endian::U16<crate::endian::BigEndian>`

  Pad

#### Trait Implementations

##### `impl Clone for FunAux32`

- `fn clone(self: &Self) -> FunAux32` — [`FunAux32`](#funaux32)

##### `impl Copy for FunAux32`

##### `impl Debug for FunAux32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for FunAux32`

### `FunAux64`

```rust
struct FunAux64 {
    pub x_lnnoptr: crate::endian::U64<crate::endian::BigEndian>,
    pub x_fsize: crate::endian::U32<crate::endian::BigEndian>,
    pub x_endndx: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

Function auxiliary entry.

#### Fields

- **`x_lnnoptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  File pointer to line number

- **`x_fsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Size of function in bytes.

- **`x_endndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of next entry beyond this function.

- **`pad`**: `u8`

  Pad

- **`x_auxtype`**: `u8`

  Contains _AUX_FCN; Type of auxiliary entry.

#### Trait Implementations

##### `impl Clone for FunAux64`

- `fn clone(self: &Self) -> FunAux64` — [`FunAux64`](#funaux64)

##### `impl Copy for FunAux64`

##### `impl Debug for FunAux64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for FunAux64`

### `ExpAux`

```rust
struct ExpAux {
    pub x_exptr: crate::endian::U64<crate::endian::BigEndian>,
    pub x_fsize: crate::endian::U32<crate::endian::BigEndian>,
    pub x_endndx: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

Exception auxiliary entry. (XCOFF64 only)

#### Fields

- **`x_exptr`**: `crate::endian::U64<crate::endian::BigEndian>`

  File offset to exception table entry.

- **`x_fsize`**: `crate::endian::U32<crate::endian::BigEndian>`

  Size of function in bytes.

- **`x_endndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of next entry beyond this function.

- **`pad`**: `u8`

  Pad

- **`x_auxtype`**: `u8`

  Contains _AUX_EXCEPT; Type of auxiliary entry

#### Trait Implementations

##### `impl Clone for ExpAux`

- `fn clone(self: &Self) -> ExpAux` — [`ExpAux`](#expaux)

##### `impl Copy for ExpAux`

##### `impl Debug for ExpAux`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ExpAux`

### `BlockAux32`

```rust
struct BlockAux32 {
    pub pad: [u8; 2],
    pub x_lnnohi: crate::endian::U16<crate::endian::BigEndian>,
    pub x_lnnolo: crate::endian::U16<crate::endian::BigEndian>,
    pub pad2: [u8; 12],
}
```

Block auxiliary entry for the C_BLOCK and C_FCN Symbols.

#### Fields

- **`pad`**: `[u8; 2]`

  Reserved.

- **`x_lnnohi`**: `crate::endian::U16<crate::endian::BigEndian>`

  High-order 2 bytes of the source line number.

- **`x_lnnolo`**: `crate::endian::U16<crate::endian::BigEndian>`

  Low-order 2 bytes of the source line number.

- **`pad2`**: `[u8; 12]`

  Reserved.

#### Trait Implementations

##### `impl Clone for BlockAux32`

- `fn clone(self: &Self) -> BlockAux32` — [`BlockAux32`](#blockaux32)

##### `impl Copy for BlockAux32`

##### `impl Debug for BlockAux32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for BlockAux32`

### `BlockAux64`

```rust
struct BlockAux64 {
    pub x_lnno: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: [u8; 13],
    pub x_auxtype: u8,
}
```

Block auxiliary entry for the C_BLOCK and C_FCN Symbols.

#### Fields

- **`x_lnno`**: `crate::endian::U32<crate::endian::BigEndian>`

  Source line number.

- **`pad`**: `[u8; 13]`

  Reserved.

- **`x_auxtype`**: `u8`

  Contains _AUX_SYM; Type of auxiliary entry.

#### Trait Implementations

##### `impl Clone for BlockAux64`

- `fn clone(self: &Self) -> BlockAux64` — [`BlockAux64`](#blockaux64)

##### `impl Copy for BlockAux64`

##### `impl Debug for BlockAux64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for BlockAux64`

### `StatAux`

```rust
struct StatAux {
    pub x_scnlen: crate::endian::U32<crate::endian::BigEndian>,
    pub x_nreloc: crate::endian::U16<crate::endian::BigEndian>,
    pub x_nlinno: crate::endian::U16<crate::endian::BigEndian>,
    pub pad: [u8; 10],
}
```

Section auxiliary entry for the C_STAT Symbol. (XCOFF32 Only)

#### Fields

- **`x_scnlen`**: `crate::endian::U32<crate::endian::BigEndian>`

  Section length.

- **`x_nreloc`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of relocation entries.

- **`x_nlinno`**: `crate::endian::U16<crate::endian::BigEndian>`

  Number of line numbers.

- **`pad`**: `[u8; 10]`

  Reserved.

#### Trait Implementations

##### `impl Clone for StatAux`

- `fn clone(self: &Self) -> StatAux` — [`StatAux`](#stataux)

##### `impl Copy for StatAux`

##### `impl Debug for StatAux`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for StatAux`

### `DwarfAux32`

```rust
struct DwarfAux32 {
    pub x_scnlen: crate::endian::U32<crate::endian::BigEndian>,
    pub pad: [u8; 4],
    pub x_nreloc: crate::endian::U32<crate::endian::BigEndian>,
    pub pad2: [u8; 6],
}
```

Section auxiliary entry Format for C_DWARF symbols.

#### Fields

- **`x_scnlen`**: `crate::endian::U32<crate::endian::BigEndian>`

  Length of portion of section represented by symbol.

- **`pad`**: `[u8; 4]`

  Reserved.

- **`x_nreloc`**: `crate::endian::U32<crate::endian::BigEndian>`

  Number of relocation entries in section.

- **`pad2`**: `[u8; 6]`

  Reserved.

#### Trait Implementations

##### `impl Clone for DwarfAux32`

- `fn clone(self: &Self) -> DwarfAux32` — [`DwarfAux32`](#dwarfaux32)

##### `impl Copy for DwarfAux32`

##### `impl Debug for DwarfAux32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for DwarfAux32`

### `DwarfAux64`

```rust
struct DwarfAux64 {
    pub x_scnlen: crate::endian::U64<crate::endian::BigEndian>,
    pub x_nreloc: crate::endian::U64<crate::endian::BigEndian>,
    pub pad: u8,
    pub x_auxtype: u8,
}
```

Section auxiliary entry Format for C_DWARF symbols.

#### Fields

- **`x_scnlen`**: `crate::endian::U64<crate::endian::BigEndian>`

  Length of portion of section represented by symbol.

- **`x_nreloc`**: `crate::endian::U64<crate::endian::BigEndian>`

  Number of relocation entries in section.

- **`pad`**: `u8`

  Reserved.

- **`x_auxtype`**: `u8`

  Contains _AUX_SECT; Type of Auxiliary entry.

#### Trait Implementations

##### `impl Clone for DwarfAux64`

- `fn clone(self: &Self) -> DwarfAux64` — [`DwarfAux64`](#dwarfaux64)

##### `impl Copy for DwarfAux64`

##### `impl Debug for DwarfAux64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for DwarfAux64`

### `Rel32`

```rust
struct Rel32 {
    pub r_vaddr: crate::endian::U32<crate::endian::BigEndian>,
    pub r_symndx: crate::endian::U32<crate::endian::BigEndian>,
    pub r_rsize: u8,
    pub r_rtype: u8,
}
```

Relocation table entry

#### Fields

- **`r_vaddr`**: `crate::endian::U32<crate::endian::BigEndian>`

  Virtual address (position) in section to be relocated.

- **`r_symndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of item that is referenced.

- **`r_rsize`**: `u8`

  Relocation size and information.

- **`r_rtype`**: `u8`

  Relocation type.

#### Trait Implementations

##### `impl Clone for Rel32`

- `fn clone(self: &Self) -> Rel32` — [`Rel32`](#rel32)

##### `impl Copy for Rel32`

##### `impl Debug for Rel32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for Rel32`

##### `impl Rel for xcoff::Rel32`

- `type Word = u32`

- `fn r_vaddr(self: &Self) -> <Self as >::Word` — [`Rel`](../read/xcoff/index.md)

- `fn r_symndx(self: &Self) -> u32`

- `fn r_rsize(self: &Self) -> u8`

- `fn r_rtype(self: &Self) -> u8`

### `Rel64`

```rust
struct Rel64 {
    pub r_vaddr: crate::endian::U64<crate::endian::BigEndian>,
    pub r_symndx: crate::endian::U32<crate::endian::BigEndian>,
    pub r_rsize: u8,
    pub r_rtype: u8,
}
```

Relocation table entry

#### Fields

- **`r_vaddr`**: `crate::endian::U64<crate::endian::BigEndian>`

  Virtual address (position) in section to be relocated.

- **`r_symndx`**: `crate::endian::U32<crate::endian::BigEndian>`

  Symbol table index of item that is referenced.

- **`r_rsize`**: `u8`

  Relocation size and information.

- **`r_rtype`**: `u8`

  Relocation type.

#### Trait Implementations

##### `impl Clone for Rel64`

- `fn clone(self: &Self) -> Rel64` — [`Rel64`](#rel64)

##### `impl Copy for Rel64`

##### `impl Debug for Rel64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for Rel64`

##### `impl Rel for xcoff::Rel64`

- `type Word = u64`

- `fn r_vaddr(self: &Self) -> <Self as >::Word` — [`Rel`](../read/xcoff/index.md)

- `fn r_symndx(self: &Self) -> u32`

- `fn r_rsize(self: &Self) -> u8`

- `fn r_rtype(self: &Self) -> u8`

## Constants

### `MAGIC_64`

```rust
const MAGIC_64: u16 = 503u16;
```

the 64-bit mach magic number

### `MAGIC_32`

```rust
const MAGIC_32: u16 = 479u16;
```

the 32-bit mach magic number

### `F_RELFLG`

```rust
const F_RELFLG: u16 = 1u16;
```

Indicates that the relocation information for binding has been removed from
the file.

### `F_EXEC`

```rust
const F_EXEC: u16 = 2u16;
```

Indicates that the file is executable. No unresolved external references exist.

### `F_LNNO`

```rust
const F_LNNO: u16 = 4u16;
```

Indicates that line numbers have been stripped from the file by a utility program.

### `F_FDPR_PROF`

```rust
const F_FDPR_PROF: u16 = 16u16;
```

Indicates that the file was profiled with the fdpr command.

### `F_FDPR_OPTI`

```rust
const F_FDPR_OPTI: u16 = 32u16;
```

Indicates that the file was reordered with the fdpr command.

### `F_DSA`

```rust
const F_DSA: u16 = 64u16;
```

Indicates that the file uses Very Large Program Support.

### `F_VARPG`

```rust
const F_VARPG: u16 = 256u16;
```

Indicates that one of the members of the auxiliary header specifying the
medium page sizes is non-zero.

### `F_DYNLOAD`

```rust
const F_DYNLOAD: u16 = 4_096u16;
```

Indicates the file is dynamically loadable and executable. External references
are resolved by way of imports, and the file might contain exports and loader
relocation.

### `F_SHROBJ`

```rust
const F_SHROBJ: u16 = 8_192u16;
```

Indicates the file is a shared object (shared library). The file is separately
loadable. That is, it is not normally bound with other objects, and its loader
exports symbols are used as automatic import symbols for other object files.

### `F_LOADONLY`

```rust
const F_LOADONLY: u16 = 16_384u16;
```

If the object file is a member of an archive, it can be loaded by the system
loader, but the member is ignored by the binder. If the object file is not in
an archive, this flag has no effect.

### `AOUTHSZ_SHORT`

```rust
const AOUTHSZ_SHORT: u16 = 28u16;
```

Some AIX programs generate auxiliary headers for 32-bit object files that
end after the data_start field.

### `STYP_REG`

```rust
const STYP_REG: u16 = 0u16;
```

"regular" section

### `STYP_PAD`

```rust
const STYP_PAD: u16 = 8u16;
```

Specifies a pad section. A section of this type is used to provide alignment
padding between sections within an XCOFF executable object file. This section
header type is obsolete since padding is allowed in an XCOFF file without a
corresponding pad section header.

### `STYP_DWARF`

```rust
const STYP_DWARF: u16 = 16u16;
```

Specifies a DWARF debugging section, which provide source file and symbol
information for the symbolic debugger.

### `STYP_TEXT`

```rust
const STYP_TEXT: u16 = 32u16;
```

Specifies an executable text (code) section. A section of this type contains
the executable instructions of a program.

### `STYP_DATA`

```rust
const STYP_DATA: u16 = 64u16;
```

Specifies an initialized data section. A section of this type contains the
initialized data and the TOC of a program.

### `STYP_BSS`

```rust
const STYP_BSS: u16 = 128u16;
```

Specifies an uninitialized data section. A section header of this type
defines the uninitialized data of a program.

### `STYP_EXCEPT`

```rust
const STYP_EXCEPT: u16 = 256u16;
```

Specifies an exception section. A section of this type provides information
to identify the reason that a trap or exception occurred within an executable
object program.

### `STYP_INFO`

```rust
const STYP_INFO: u16 = 512u16;
```

Specifies a comment section. A section of this type provides comments or data
to special processing utility programs.

### `STYP_TDATA`

```rust
const STYP_TDATA: u16 = 1_024u16;
```

Specifies an initialized thread-local data section.

### `STYP_TBSS`

```rust
const STYP_TBSS: u16 = 2_048u16;
```

Specifies an uninitialized thread-local data section.

### `STYP_LOADER`

```rust
const STYP_LOADER: u16 = 4_096u16;
```

Specifies a loader section. A section of this type contains object file
information for the system loader to load an XCOFF executable. The information
includes imported symbols, exported symbols, relocation data, type-check
information, and shared object names.

### `STYP_DEBUG`

```rust
const STYP_DEBUG: u16 = 8_192u16;
```

Specifies a debug section. A section of this type contains stabstring
information used by the symbolic debugger.

### `STYP_TYPCHK`

```rust
const STYP_TYPCHK: u16 = 16_384u16;
```

Specifies a type-check section. A section of this type contains
parameter/argument type-check strings used by the binder.

### `STYP_OVRFLO`

```rust
const STYP_OVRFLO: u16 = 32_768u16;
```

Specifies a relocation or line-number field overflow section. A section
header of this type contains the count of relocation entries and line
number entries for some other section. This section header is required
when either of the counts exceeds 65,534.

### `SSUBTYP_DWINFO`

```rust
const SSUBTYP_DWINFO: u32 = 65_536u32;
```

### `SSUBTYP_DWLINE`

```rust
const SSUBTYP_DWLINE: u32 = 131_072u32;
```

### `SSUBTYP_DWPBNMS`

```rust
const SSUBTYP_DWPBNMS: u32 = 196_608u32;
```

### `SSUBTYP_DWPBTYP`

```rust
const SSUBTYP_DWPBTYP: u32 = 262_144u32;
```

### `SSUBTYP_DWARNGE`

```rust
const SSUBTYP_DWARNGE: u32 = 327_680u32;
```

### `SSUBTYP_DWABREV`

```rust
const SSUBTYP_DWABREV: u32 = 393_216u32;
```

### `SSUBTYP_DWSTR`

```rust
const SSUBTYP_DWSTR: u32 = 458_752u32;
```

### `SSUBTYP_DWRNGES`

```rust
const SSUBTYP_DWRNGES: u32 = 524_288u32;
```

### `SSUBTYP_DWLOC`

```rust
const SSUBTYP_DWLOC: u32 = 589_824u32;
```

### `SSUBTYP_DWFRAME`

```rust
const SSUBTYP_DWFRAME: u32 = 655_360u32;
```

### `SSUBTYP_DWMAC`

```rust
const SSUBTYP_DWMAC: u32 = 720_896u32;
```

### `SIZEOF_SYMBOL`

```rust
const SIZEOF_SYMBOL: usize = 18usize;
```

### `N_DEBUG`

```rust
const N_DEBUG: i16 = -2i16;
```

A special symbolic debugging symbol.

### `N_ABS`

```rust
const N_ABS: i16 = -1i16;
```

An absolute symbol. The symbol has a value but is not relocatable.

### `N_UNDEF`

```rust
const N_UNDEF: i16 = 0i16;
```

An undefined external symbol.

### `SYM_V_MASK`

```rust
const SYM_V_MASK: u16 = 61_440u16;
```

Values for visibility as they would appear when encoded in the high 4 bits
of the 16-bit unsigned n_type field of symbol table entries. Valid for
32-bit XCOFF only when the o_vstamp in the auxiliary header is greater than 1.

### `SYM_V_INTERNAL`

```rust
const SYM_V_INTERNAL: u16 = 4_096u16;
```

### `SYM_V_HIDDEN`

```rust
const SYM_V_HIDDEN: u16 = 8_192u16;
```

### `SYM_V_PROTECTED`

```rust
const SYM_V_PROTECTED: u16 = 12_288u16;
```

### `SYM_V_EXPORTED`

```rust
const SYM_V_EXPORTED: u16 = 16_384u16;
```

### `C_FILE`

```rust
const C_FILE: u8 = 103u8;
```

Source file name and compiler information.

### `C_BINCL`

```rust
const C_BINCL: u8 = 108u8;
```

Beginning of include file.

### `C_EINCL`

```rust
const C_EINCL: u8 = 109u8;
```

Ending of include file.

### `C_GSYM`

```rust
const C_GSYM: u8 = 128u8;
```

Global variable.

### `C_STSYM`

```rust
const C_STSYM: u8 = 133u8;
```

Statically allocated symbol.

### `C_BCOMM`

```rust
const C_BCOMM: u8 = 135u8;
```

Beginning of common block.

### `C_ECOMM`

```rust
const C_ECOMM: u8 = 137u8;
```

End of common block.

### `C_ENTRY`

```rust
const C_ENTRY: u8 = 141u8;
```

Alternate entry.

### `C_BSTAT`

```rust
const C_BSTAT: u8 = 143u8;
```

Beginning of static block.

### `C_ESTAT`

```rust
const C_ESTAT: u8 = 144u8;
```

End of static block.

### `C_GTLS`

```rust
const C_GTLS: u8 = 145u8;
```

Global thread-local variable.

### `C_STTLS`

```rust
const C_STTLS: u8 = 146u8;
```

Static thread-local variable.

### `C_DWARF`

```rust
const C_DWARF: u8 = 112u8;
```

DWARF section symbol.

### `C_LSYM`

```rust
const C_LSYM: u8 = 129u8;
```

Automatic variable allocated on stack.

### `C_PSYM`

```rust
const C_PSYM: u8 = 130u8;
```

Argument to subroutine allocated on stack.

### `C_RSYM`

```rust
const C_RSYM: u8 = 131u8;
```

Register variable.

### `C_RPSYM`

```rust
const C_RPSYM: u8 = 132u8;
```

Argument to function or procedure stored in register.

### `C_ECOML`

```rust
const C_ECOML: u8 = 136u8;
```

Local member of common block.

### `C_FUN`

```rust
const C_FUN: u8 = 142u8;
```

Function or procedure.

### `C_EXT`

```rust
const C_EXT: u8 = 2u8;
```

External symbol.

### `C_WEAKEXT`

```rust
const C_WEAKEXT: u8 = 111u8;
```

Weak external symbol.

### `C_NULL`

```rust
const C_NULL: u8 = 0u8;
```

Symbol table entry marked for deletion.

### `C_STAT`

```rust
const C_STAT: u8 = 3u8;
```

Static.

### `C_BLOCK`

```rust
const C_BLOCK: u8 = 100u8;
```

Beginning or end of inner block.

### `C_FCN`

```rust
const C_FCN: u8 = 101u8;
```

Beginning or end of function.

### `C_HIDEXT`

```rust
const C_HIDEXT: u8 = 107u8;
```

Un-named external symbol.

### `C_INFO`

```rust
const C_INFO: u8 = 110u8;
```

Comment string in .info section.

### `C_DECL`

```rust
const C_DECL: u8 = 140u8;
```

Declaration of object (type).

### `C_AUTO`

```rust
const C_AUTO: u8 = 1u8;
```

Automatic variable.

### `C_REG`

```rust
const C_REG: u8 = 4u8;
```

Register variable.

### `C_EXTDEF`

```rust
const C_EXTDEF: u8 = 5u8;
```

External definition.

### `C_LABEL`

```rust
const C_LABEL: u8 = 6u8;
```

Label.

### `C_ULABEL`

```rust
const C_ULABEL: u8 = 7u8;
```

Undefined label.

### `C_MOS`

```rust
const C_MOS: u8 = 8u8;
```

Member of structure.

### `C_ARG`

```rust
const C_ARG: u8 = 9u8;
```

Function argument.

### `C_STRTAG`

```rust
const C_STRTAG: u8 = 10u8;
```

Structure tag.

### `C_MOU`

```rust
const C_MOU: u8 = 11u8;
```

Member of union.

### `C_UNTAG`

```rust
const C_UNTAG: u8 = 12u8;
```

Union tag.

### `C_TPDEF`

```rust
const C_TPDEF: u8 = 13u8;
```

Type definition.

### `C_USTATIC`

```rust
const C_USTATIC: u8 = 14u8;
```

Undefined static.

### `C_ENTAG`

```rust
const C_ENTAG: u8 = 15u8;
```

Enumeration tag.

### `C_MOE`

```rust
const C_MOE: u8 = 16u8;
```

Member of enumeration.

### `C_REGPARM`

```rust
const C_REGPARM: u8 = 17u8;
```

Register parameter.

### `C_FIELD`

```rust
const C_FIELD: u8 = 18u8;
```

Bit field.

### `C_EOS`

```rust
const C_EOS: u8 = 102u8;
```

End of structure.

### `C_ALIAS`

```rust
const C_ALIAS: u8 = 105u8;
```

Duplicate tag.

### `C_HIDDEN`

```rust
const C_HIDDEN: u8 = 106u8;
```

Special storage class for external.

### `C_EFCN`

```rust
const C_EFCN: u8 = 255u8;
```

Physical end of function.

### `C_TCSYM`

```rust
const C_TCSYM: u8 = 134u8;
```

Reserved.

### `XFT_FN`

```rust
const XFT_FN: u8 = 0u8;
```

Specifies the source-file name.

### `XFT_CT`

```rust
const XFT_CT: u8 = 1u8;
```

Specifies the compiler time stamp.

### `XFT_CV`

```rust
const XFT_CV: u8 = 2u8;
```

Specifies the compiler version number.

### `XFT_CD`

```rust
const XFT_CD: u8 = 128u8;
```

Specifies compiler-defined information.

### `XTY_ER`

```rust
const XTY_ER: u8 = 0u8;
```

External reference.

### `XTY_SD`

```rust
const XTY_SD: u8 = 1u8;
```

Csect definition for initialized storage.

### `XTY_LD`

```rust
const XTY_LD: u8 = 2u8;
```

Defines an entry point to an initialized csect.

### `XTY_CM`

```rust
const XTY_CM: u8 = 3u8;
```

Common csect definition. For uninitialized storage.

### `XMC_PR`

```rust
const XMC_PR: u8 = 0u8;
```

Program Code

### `XMC_RO`

```rust
const XMC_RO: u8 = 1u8;
```

Read Only Constant

### `XMC_DB`

```rust
const XMC_DB: u8 = 2u8;
```

Debug Dictionary Table

### `XMC_GL`

```rust
const XMC_GL: u8 = 6u8;
```

Global Linkage (Interfile Interface Code)

### `XMC_XO`

```rust
const XMC_XO: u8 = 7u8;
```

Extended Operation (Pseudo Machine Instruction)

### `XMC_SV`

```rust
const XMC_SV: u8 = 8u8;
```

Supervisor Call (32-bit process only)

### `XMC_SV64`

```rust
const XMC_SV64: u8 = 17u8;
```

Supervisor Call for 64-bit process

### `XMC_SV3264`

```rust
const XMC_SV3264: u8 = 18u8;
```

Supervisor Call for both 32- and 64-bit processes

### `XMC_TI`

```rust
const XMC_TI: u8 = 12u8;
```

Traceback Index csect

### `XMC_TB`

```rust
const XMC_TB: u8 = 13u8;
```

Traceback Table csect

### `XMC_RW`

```rust
const XMC_RW: u8 = 5u8;
```

Read Write Data

### `XMC_TC0`

```rust
const XMC_TC0: u8 = 15u8;
```

TOC Anchor for TOC Addressability

### `XMC_TC`

```rust
const XMC_TC: u8 = 3u8;
```

General TOC item

### `XMC_TD`

```rust
const XMC_TD: u8 = 16u8;
```

Scalar data item in the TOC

### `XMC_DS`

```rust
const XMC_DS: u8 = 10u8;
```

Descriptor csect

### `XMC_UA`

```rust
const XMC_UA: u8 = 4u8;
```

Unclassified - Treated as Read Write

### `XMC_BS`

```rust
const XMC_BS: u8 = 9u8;
```

BSS class (uninitialized static internal)

### `XMC_UC`

```rust
const XMC_UC: u8 = 11u8;
```

Un-named Fortran Common

### `XMC_TL`

```rust
const XMC_TL: u8 = 20u8;
```

Initialized thread-local variable

### `XMC_UL`

```rust
const XMC_UL: u8 = 21u8;
```

Uninitialized thread-local variable

### `XMC_TE`

```rust
const XMC_TE: u8 = 22u8;
```

Symbol mapped at the end of TOC

### `AUX_EXCEPT`

```rust
const AUX_EXCEPT: u8 = 255u8;
```

Identifies an exception auxiliary entry.

### `AUX_FCN`

```rust
const AUX_FCN: u8 = 254u8;
```

Identifies a function auxiliary entry.

### `AUX_SYM`

```rust
const AUX_SYM: u8 = 253u8;
```

Identifies a symbol auxiliary entry.

### `AUX_FILE`

```rust
const AUX_FILE: u8 = 252u8;
```

Identifies a file auxiliary entry.

### `AUX_CSECT`

```rust
const AUX_CSECT: u8 = 251u8;
```

Identifies a csect auxiliary entry.

### `AUX_SECT`

```rust
const AUX_SECT: u8 = 250u8;
```

Identifies a SECT auxiliary entry.

### `R_POS`

```rust
const R_POS: u8 = 0u8;
```

Positive relocation.

### `R_RL`

```rust
const R_RL: u8 = 12u8;
```

Positive indirect load relocation.

### `R_RLA`

```rust
const R_RLA: u8 = 13u8;
```

Positive load address relocation. Modifiable instruction.

### `R_NEG`

```rust
const R_NEG: u8 = 1u8;
```

Negative relocation.

### `R_REL`

```rust
const R_REL: u8 = 2u8;
```

Relative to self relocation.

### `R_TOC`

```rust
const R_TOC: u8 = 3u8;
```

Relative to the TOC relocation.

### `R_TRL`

```rust
const R_TRL: u8 = 18u8;
```

TOC relative indirect load relocation.

### `R_TRLA`

```rust
const R_TRLA: u8 = 19u8;
```

Relative to the TOC or to the thread-local storage base relocation.

### `R_GL`

```rust
const R_GL: u8 = 5u8;
```

Global linkage-external TOC address relocation.

### `R_TCL`

```rust
const R_TCL: u8 = 6u8;
```

Local object TOC address relocation.

### `R_REF`

```rust
const R_REF: u8 = 15u8;
```

A non-relocating relocation.

### `R_BA`

```rust
const R_BA: u8 = 8u8;
```

Branch absolute relocation. References a non-modifiable instruction.

### `R_BR`

```rust
const R_BR: u8 = 10u8;
```

Branch relative to self relocation. References a non-modifiable instruction.

### `R_RBA`

```rust
const R_RBA: u8 = 24u8;
```

Branch absolute relocation. References a modifiable instruction.

### `R_RBR`

```rust
const R_RBR: u8 = 26u8;
```

Branch relative to self relocation. References a modifiable instruction.

### `R_TLS`

```rust
const R_TLS: u8 = 32u8;
```

General-dynamic reference to TLS symbol.

### `R_TLS_IE`

```rust
const R_TLS_IE: u8 = 33u8;
```

Initial-exec reference to TLS symbol.

### `R_TLS_LD`

```rust
const R_TLS_LD: u8 = 34u8;
```

Local-dynamic reference to TLS symbol.

### `R_TLS_LE`

```rust
const R_TLS_LE: u8 = 35u8;
```

Local-exec reference to TLS symbol.

### `R_TLSM`

```rust
const R_TLSM: u8 = 36u8;
```

Module reference to TLS.

### `R_TLSML`

```rust
const R_TLSML: u8 = 37u8;
```

Module reference to the local TLS storage.

### `R_TOCU`

```rust
const R_TOCU: u8 = 48u8;
```

Relative to TOC upper.

### `R_TOCL`

```rust
const R_TOCL: u8 = 49u8;
```

Relative to TOC lower.

