*[object](../index.md) / [pe](index.md)*

---

# Module `pe`

PE/COFF definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is based heavily on "winnt.h" (10.0.17763.0).

## Structs

### `ImageDosHeader`

```rust
struct ImageDosHeader {
    pub e_magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cblp: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cp: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_crlc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cparhdr: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_minalloc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_maxalloc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_ss: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_sp: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_csum: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_ip: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_cs: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_lfarlc: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_ovno: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_res: [crate::endian::U16<crate::endian::LittleEndian>; 4],
    pub e_oemid: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_oeminfo: crate::endian::U16<crate::endian::LittleEndian>,
    pub e_res2: [crate::endian::U16<crate::endian::LittleEndian>; 10],
    pub e_lfanew: crate::endian::U32<crate::endian::LittleEndian>,
}
```

DOS .EXE header

#### Fields

- **`e_magic`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Magic number

- **`e_cblp`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Bytes on last page of file

- **`e_cp`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Pages in file

- **`e_crlc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Relocations

- **`e_cparhdr`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Size of header in paragraphs

- **`e_minalloc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Minimum extra paragraphs needed

- **`e_maxalloc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Maximum extra paragraphs needed

- **`e_ss`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial (relative) SS value

- **`e_sp`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial SP value

- **`e_csum`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Checksum

- **`e_ip`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial IP value

- **`e_cs`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial (relative) CS value

- **`e_lfarlc`**: `crate::endian::U16<crate::endian::LittleEndian>`

  File address of relocation table

- **`e_ovno`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Overlay number

- **`e_res`**: `[crate::endian::U16<crate::endian::LittleEndian>; 4]`

  Reserved words

- **`e_oemid`**: `crate::endian::U16<crate::endian::LittleEndian>`

  OEM identifier (for e_oeminfo)

- **`e_oeminfo`**: `crate::endian::U16<crate::endian::LittleEndian>`

  OEM information; e_oemid specific

- **`e_res2`**: `[crate::endian::U16<crate::endian::LittleEndian>; 10]`

  Reserved words

- **`e_lfanew`**: `crate::endian::U32<crate::endian::LittleEndian>`

  File address of new exe header

#### Implementations

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> read::Result<&'data Self>` — [`Result`](../index.md)

- `fn nt_headers_offset(self: &Self) -> u32`

#### Trait Implementations

##### `impl Clone for ImageDosHeader`

- `fn clone(self: &Self) -> ImageDosHeader` — [`ImageDosHeader`](#imagedosheader)

##### `impl Copy for ImageDosHeader`

##### `impl Debug for ImageDosHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDosHeader`

### `ImageOs2Header`

```rust
struct ImageOs2Header {
    pub ne_magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_ver: i8,
    pub ne_rev: i8,
    pub ne_enttab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cbenttab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_crc: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_autodata: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_heap: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_stack: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_csip: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_sssp: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_cseg: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cmod: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cbnrestab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_segtab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_rsrctab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_restab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_modtab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_imptab: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_nrestab: crate::endian::I32<crate::endian::LittleEndian>,
    pub ne_cmovent: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_align: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_cres: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_exetyp: u8,
    pub ne_flagsothers: u8,
    pub ne_pretthunks: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_psegrefbytes: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_swaparea: crate::endian::U16<crate::endian::LittleEndian>,
    pub ne_expver: crate::endian::U16<crate::endian::LittleEndian>,
}
```

OS/2 .EXE header

#### Fields

- **`ne_magic`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Magic number

- **`ne_ver`**: `i8`

  Version number

- **`ne_rev`**: `i8`

  Revision number

- **`ne_enttab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Entry Table

- **`ne_cbenttab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Number of bytes in Entry Table

- **`ne_crc`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Checksum of whole file

- **`ne_flags`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Flag word

- **`ne_autodata`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Automatic data segment number

- **`ne_heap`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial heap allocation

- **`ne_stack`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Initial stack allocation

- **`ne_csip`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Initial CS:IP setting

- **`ne_sssp`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Initial SS:SP setting

- **`ne_cseg`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Count of file segments

- **`ne_cmod`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Entries in Module Reference Table

- **`ne_cbnrestab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Size of non-resident name table

- **`ne_segtab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Segment Table

- **`ne_rsrctab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Resource Table

- **`ne_restab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of resident name table

- **`ne_modtab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Module Reference Table

- **`ne_imptab`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Offset of Imported Names Table

- **`ne_nrestab`**: `crate::endian::I32<crate::endian::LittleEndian>`

  Offset of Non-resident Names Table

- **`ne_cmovent`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Count of movable entries

- **`ne_align`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Segment alignment shift count

- **`ne_cres`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Count of resource segments

- **`ne_exetyp`**: `u8`

  Target Operating system

- **`ne_flagsothers`**: `u8`

  Other .EXE flags

- **`ne_pretthunks`**: `crate::endian::U16<crate::endian::LittleEndian>`

  offset to return thunks

- **`ne_psegrefbytes`**: `crate::endian::U16<crate::endian::LittleEndian>`

  offset to segment ref. bytes

- **`ne_swaparea`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Minimum code swap area size

- **`ne_expver`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Expected Windows version number

#### Trait Implementations

##### `impl Clone for ImageOs2Header`

- `fn clone(self: &Self) -> ImageOs2Header` — [`ImageOs2Header`](#imageos2header)

##### `impl Copy for ImageOs2Header`

##### `impl Debug for ImageOs2Header`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageOs2Header`

### `ImageVxdHeader`

```rust
struct ImageVxdHeader {
    pub e32_magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_border: u8,
    pub e32_worder: u8,
    pub e32_level: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_cpu: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_os: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_ver: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_mflags: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_mpages: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_startobj: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_eip: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_stackobj: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_esp: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_pagesize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_lastpagesize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_fixupsize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_fixupsum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_ldrsize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_ldrsum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_objtab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_objcnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_objmap: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_itermap: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_rsrctab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_rsrccnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_restab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_enttab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_dirtab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_dircnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_fpagetab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_frectab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_impmod: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_impmodcnt: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_impproc: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_pagesum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_datapage: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_preload: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_nrestab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_cbnrestab: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_nressum: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_autodata: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_debuginfo: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_debuglen: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_instpreload: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_instdemand: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_heapsize: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_res3: [u8; 12],
    pub e32_winresoff: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_winreslen: crate::endian::U32<crate::endian::LittleEndian>,
    pub e32_devid: crate::endian::U16<crate::endian::LittleEndian>,
    pub e32_ddkver: crate::endian::U16<crate::endian::LittleEndian>,
}
```

Windows VXD header

#### Fields

- **`e32_magic`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Magic number

- **`e32_border`**: `u8`

  The byte ordering for the VXD

- **`e32_worder`**: `u8`

  The word ordering for the VXD

- **`e32_level`**: `crate::endian::U32<crate::endian::LittleEndian>`

  The EXE format level for now = 0

- **`e32_cpu`**: `crate::endian::U16<crate::endian::LittleEndian>`

  The CPU type

- **`e32_os`**: `crate::endian::U16<crate::endian::LittleEndian>`

  The OS type

- **`e32_ver`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Module version

- **`e32_mflags`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Module flags

- **`e32_mpages`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Module # pages

- **`e32_startobj`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object # for instruction pointer

- **`e32_eip`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Extended instruction pointer

- **`e32_stackobj`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object # for stack pointer

- **`e32_esp`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Extended stack pointer

- **`e32_pagesize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VXD page size

- **`e32_lastpagesize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Last page size in VXD

- **`e32_fixupsize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Fixup section size

- **`e32_fixupsum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Fixup section checksum

- **`e32_ldrsize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Loader section size

- **`e32_ldrsum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Loader section checksum

- **`e32_objtab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object table offset

- **`e32_objcnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of objects in module

- **`e32_objmap`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object page map offset

- **`e32_itermap`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object iterated data map offset

- **`e32_rsrctab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Resource Table

- **`e32_rsrccnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of resource entries

- **`e32_restab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of resident name table

- **`e32_enttab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Entry Table

- **`e32_dirtab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Module Directive Table

- **`e32_dircnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of module directives

- **`e32_fpagetab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Fixup Page Table

- **`e32_frectab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Fixup Record Table

- **`e32_impmod`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Import Module Name Table

- **`e32_impmodcnt`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of entries in Import Module Name Table

- **`e32_impproc`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Import Procedure Name Table

- **`e32_pagesum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Per-Page Checksum Table

- **`e32_datapage`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Enumerated Data Pages

- **`e32_preload`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of preload pages

- **`e32_nrestab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of Non-resident Names Table

- **`e32_cbnrestab`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of Non-resident Name Table

- **`e32_nressum`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Non-resident Name Table Checksum

- **`e32_autodata`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Object # for automatic data object

- **`e32_debuginfo`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of the debugging information

- **`e32_debuglen`**: `crate::endian::U32<crate::endian::LittleEndian>`

  The length of the debugging info. in bytes

- **`e32_instpreload`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of instance pages in preload section of VXD file

- **`e32_instdemand`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Number of instance pages in demand load section of VXD file

- **`e32_heapsize`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of heap - for 16-bit apps

- **`e32_res3`**: `[u8; 12]`

  Reserved words

- **`e32_devid`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Device ID for VxD

- **`e32_ddkver`**: `crate::endian::U16<crate::endian::LittleEndian>`

  DDK version for VxD

#### Trait Implementations

##### `impl Clone for ImageVxdHeader`

- `fn clone(self: &Self) -> ImageVxdHeader` — [`ImageVxdHeader`](#imagevxdheader)

##### `impl Copy for ImageVxdHeader`

##### `impl Debug for ImageVxdHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageVxdHeader`

### `MaskedRichHeaderEntry`

```rust
struct MaskedRichHeaderEntry {
    pub masked_comp_id: crate::endian::U32<crate::endian::LittleEndian>,
    pub masked_count: crate::endian::U32<crate::endian::LittleEndian>,
}
```

A PE rich header entry.

Rich headers have no official documentation, but have been heavily
reversed-engineered and documented in the wild, e.g.:
* `http://www.ntcore.com/files/richsign.htm`
* `https://www.researchgate.net/figure/Structure-of-the-Rich-Header_fig1_318145388`

This data is "masked", i.e. XORed with a checksum derived from the file data.

#### Trait Implementations

##### `impl Clone for MaskedRichHeaderEntry`

- `fn clone(self: &Self) -> MaskedRichHeaderEntry` — [`MaskedRichHeaderEntry`](#maskedrichheaderentry)

##### `impl Copy for MaskedRichHeaderEntry`

##### `impl Debug for MaskedRichHeaderEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for MaskedRichHeaderEntry`

### `ImageFileHeader`

```rust
struct ImageFileHeader {
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_sections: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_symbol_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_optional_header: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageFileHeader`

- `fn clone(self: &Self) -> ImageFileHeader` — [`ImageFileHeader`](#imagefileheader)

##### `impl CoffHeader for pe::ImageFileHeader`

- `type ImageSymbol = ImageSymbol`

- `type ImageSymbolBytes = ImageSymbolBytes`

- `fn is_type_bigobj() -> bool`

- `fn machine(self: &Self) -> u16`

- `fn number_of_sections(self: &Self) -> u32`

- `fn pointer_to_symbol_table(self: &Self) -> u32`

- `fn number_of_symbols(self: &Self) -> u32`

- `fn characteristics(self: &Self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>` — [`Result`](../index.md)

##### `impl Copy for ImageFileHeader`

##### `impl Debug for ImageFileHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageFileHeader`

### `ImageDataDirectory`

```rust
struct ImageDataDirectory {
    pub virtual_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Implementations

- `fn address_range(self: &Self) -> (u32, u32)`

- `fn file_range(self: &Self, sections: &SectionTable<'_>) -> Result<(u32, u32)>` — [`SectionTable`](../read/pe/index.md), [`Result`](../index.md)

- `fn data<'data, R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<&'data [u8]>` — [`SectionTable`](../read/pe/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for ImageDataDirectory`

- `fn clone(self: &Self) -> ImageDataDirectory` — [`ImageDataDirectory`](#imagedatadirectory)

##### `impl Copy for ImageDataDirectory`

##### `impl Debug for ImageDataDirectory`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDataDirectory`

### `ImageOptionalHeader32`

```rust
struct ImageOptionalHeader32 {
    pub magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U32<crate::endian::LittleEndian>,
    pub section_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub win32_version_value: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_headers: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub subsystem: crate::endian::U16<crate::endian::LittleEndian>,
    pub dll_characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub size_of_stack_reserve: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_stack_commit: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_heap_reserve: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_heap_commit: crate::endian::U32<crate::endian::LittleEndian>,
    pub loader_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_rva_and_sizes: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageOptionalHeader32`

- `fn clone(self: &Self) -> ImageOptionalHeader32` — [`ImageOptionalHeader32`](#imageoptionalheader32)

##### `impl Copy for ImageOptionalHeader32`

##### `impl Debug for ImageOptionalHeader32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageOptionalHeader for pe::ImageOptionalHeader32`

- `fn magic(self: &Self) -> u16`

- `fn major_linker_version(self: &Self) -> u8`

- `fn minor_linker_version(self: &Self) -> u8`

- `fn size_of_code(self: &Self) -> u32`

- `fn size_of_initialized_data(self: &Self) -> u32`

- `fn size_of_uninitialized_data(self: &Self) -> u32`

- `fn address_of_entry_point(self: &Self) -> u32`

- `fn base_of_code(self: &Self) -> u32`

- `fn base_of_data(self: &Self) -> Option<u32>`

- `fn image_base(self: &Self) -> u64`

- `fn section_alignment(self: &Self) -> u32`

- `fn file_alignment(self: &Self) -> u32`

- `fn major_operating_system_version(self: &Self) -> u16`

- `fn minor_operating_system_version(self: &Self) -> u16`

- `fn major_image_version(self: &Self) -> u16`

- `fn minor_image_version(self: &Self) -> u16`

- `fn major_subsystem_version(self: &Self) -> u16`

- `fn minor_subsystem_version(self: &Self) -> u16`

- `fn win32_version_value(self: &Self) -> u32`

- `fn size_of_image(self: &Self) -> u32`

- `fn size_of_headers(self: &Self) -> u32`

- `fn check_sum(self: &Self) -> u32`

- `fn subsystem(self: &Self) -> u16`

- `fn dll_characteristics(self: &Self) -> u16`

- `fn size_of_stack_reserve(self: &Self) -> u64`

- `fn size_of_stack_commit(self: &Self) -> u64`

- `fn size_of_heap_reserve(self: &Self) -> u64`

- `fn size_of_heap_commit(self: &Self) -> u64`

- `fn loader_flags(self: &Self) -> u32`

- `fn number_of_rva_and_sizes(self: &Self) -> u32`

##### `impl Pod for ImageOptionalHeader32`

### `ImageRomOptionalHeader`

```rust
struct ImageRomOptionalHeader {
    pub magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_bss: crate::endian::U32<crate::endian::LittleEndian>,
    pub gpr_mask: crate::endian::U32<crate::endian::LittleEndian>,
    pub cpr_mask: [crate::endian::U32<crate::endian::LittleEndian>; 4],
    pub gp_value: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageRomOptionalHeader`

- `fn clone(self: &Self) -> ImageRomOptionalHeader` — [`ImageRomOptionalHeader`](#imageromoptionalheader)

##### `impl Copy for ImageRomOptionalHeader`

##### `impl Debug for ImageRomOptionalHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageRomOptionalHeader`

### `ImageOptionalHeader64`

```rust
struct ImageOptionalHeader64 {
    pub magic: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_initialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_uninitialized_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_entry_point: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U64<crate::endian::LittleEndian>,
    pub section_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_operating_system_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_image_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub major_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_subsystem_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub win32_version_value: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_headers: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub subsystem: crate::endian::U16<crate::endian::LittleEndian>,
    pub dll_characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub size_of_stack_reserve: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_stack_commit: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_heap_reserve: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_heap_commit: crate::endian::U64<crate::endian::LittleEndian>,
    pub loader_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_rva_and_sizes: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageOptionalHeader64`

- `fn clone(self: &Self) -> ImageOptionalHeader64` — [`ImageOptionalHeader64`](#imageoptionalheader64)

##### `impl Copy for ImageOptionalHeader64`

##### `impl Debug for ImageOptionalHeader64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageOptionalHeader for pe::ImageOptionalHeader64`

- `fn magic(self: &Self) -> u16`

- `fn major_linker_version(self: &Self) -> u8`

- `fn minor_linker_version(self: &Self) -> u8`

- `fn size_of_code(self: &Self) -> u32`

- `fn size_of_initialized_data(self: &Self) -> u32`

- `fn size_of_uninitialized_data(self: &Self) -> u32`

- `fn address_of_entry_point(self: &Self) -> u32`

- `fn base_of_code(self: &Self) -> u32`

- `fn base_of_data(self: &Self) -> Option<u32>`

- `fn image_base(self: &Self) -> u64`

- `fn section_alignment(self: &Self) -> u32`

- `fn file_alignment(self: &Self) -> u32`

- `fn major_operating_system_version(self: &Self) -> u16`

- `fn minor_operating_system_version(self: &Self) -> u16`

- `fn major_image_version(self: &Self) -> u16`

- `fn minor_image_version(self: &Self) -> u16`

- `fn major_subsystem_version(self: &Self) -> u16`

- `fn minor_subsystem_version(self: &Self) -> u16`

- `fn win32_version_value(self: &Self) -> u32`

- `fn size_of_image(self: &Self) -> u32`

- `fn size_of_headers(self: &Self) -> u32`

- `fn check_sum(self: &Self) -> u32`

- `fn subsystem(self: &Self) -> u16`

- `fn dll_characteristics(self: &Self) -> u16`

- `fn size_of_stack_reserve(self: &Self) -> u64`

- `fn size_of_stack_commit(self: &Self) -> u64`

- `fn size_of_heap_reserve(self: &Self) -> u64`

- `fn size_of_heap_commit(self: &Self) -> u64`

- `fn loader_flags(self: &Self) -> u32`

- `fn number_of_rva_and_sizes(self: &Self) -> u32`

##### `impl Pod for ImageOptionalHeader64`

### `ImageNtHeaders64`

```rust
struct ImageNtHeaders64 {
    pub signature: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader64,
}
```

#### Trait Implementations

##### `impl Clone for ImageNtHeaders64`

- `fn clone(self: &Self) -> ImageNtHeaders64` — [`ImageNtHeaders64`](#imagentheaders64)

##### `impl Copy for ImageNtHeaders64`

##### `impl Debug for ImageNtHeaders64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageNtHeaders for pe::ImageNtHeaders64`

- `type ImageOptionalHeader = ImageOptionalHeader64`

- `type ImageThunkData = ImageThunkData64`

- `fn is_type_64(self: &Self) -> bool`

- `fn is_valid_optional_magic(self: &Self) -> bool`

- `fn signature(self: &Self) -> u32`

- `fn file_header(self: &Self) -> &pe::ImageFileHeader` — [`ImageFileHeader`](#imagefileheader)

- `fn optional_header(self: &Self) -> &<Self as >::ImageOptionalHeader` — [`ImageNtHeaders`](../read/pe/index.md)

##### `impl Pod for ImageNtHeaders64`

### `ImageNtHeaders32`

```rust
struct ImageNtHeaders32 {
    pub signature: crate::endian::U32<crate::endian::LittleEndian>,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader32,
}
```

#### Trait Implementations

##### `impl Clone for ImageNtHeaders32`

- `fn clone(self: &Self) -> ImageNtHeaders32` — [`ImageNtHeaders32`](#imagentheaders32)

##### `impl Copy for ImageNtHeaders32`

##### `impl Debug for ImageNtHeaders32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageNtHeaders for pe::ImageNtHeaders32`

- `type ImageOptionalHeader = ImageOptionalHeader32`

- `type ImageThunkData = ImageThunkData32`

- `fn is_type_64(self: &Self) -> bool`

- `fn is_valid_optional_magic(self: &Self) -> bool`

- `fn signature(self: &Self) -> u32`

- `fn file_header(self: &Self) -> &pe::ImageFileHeader` — [`ImageFileHeader`](#imagefileheader)

- `fn optional_header(self: &Self) -> &<Self as >::ImageOptionalHeader` — [`ImageNtHeaders`](../read/pe/index.md)

##### `impl Pod for ImageNtHeaders32`

### `ImageRomHeaders`

```rust
struct ImageRomHeaders {
    pub file_header: ImageFileHeader,
    pub optional_header: ImageRomOptionalHeader,
}
```

#### Trait Implementations

##### `impl Clone for ImageRomHeaders`

- `fn clone(self: &Self) -> ImageRomHeaders` — [`ImageRomHeaders`](#imageromheaders)

##### `impl Copy for ImageRomHeaders`

##### `impl Debug for ImageRomHeaders`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageRomHeaders`

### `Guid`

```rust
struct Guid([u8; 16]);
```

#### Implementations

- `fn data1(self: Self) -> U32<LE>` — [`U32`](../index.md), [`LittleEndian`](../index.md)

- `fn data2(self: Self) -> U16<LE>` — [`U16`](../index.md), [`LittleEndian`](../index.md)

- `fn data3(self: Self) -> U16<LE>` — [`U16`](../index.md), [`LittleEndian`](../index.md)

- `fn data4(self: Self) -> [u8; 8]`

#### Trait Implementations

##### `impl Clone for Guid`

- `fn clone(self: &Self) -> Guid` — [`ClsId`](#clsid)

##### `impl Copy for Guid`

##### `impl Debug for Guid`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Guid`

##### `impl PartialEq for Guid`

- `fn eq(self: &Self, other: &Guid) -> bool` — [`ClsId`](#clsid)

##### `impl Pod for Guid`

##### `impl StructuralPartialEq for Guid`

### `ClsId`

```rust
struct ClsId([u8; 16]);
```

#### Implementations

- `fn data1(self: Self) -> U32<LE>` — [`U32`](../index.md), [`LittleEndian`](../index.md)

- `fn data2(self: Self) -> U16<LE>` — [`U16`](../index.md), [`LittleEndian`](../index.md)

- `fn data3(self: Self) -> U16<LE>` — [`U16`](../index.md), [`LittleEndian`](../index.md)

- `fn data4(self: Self) -> [u8; 8]`

#### Trait Implementations

##### `impl Clone for Guid`

- `fn clone(self: &Self) -> Guid` — [`ClsId`](#clsid)

##### `impl Copy for Guid`

##### `impl Debug for Guid`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Guid`

##### `impl PartialEq for Guid`

- `fn eq(self: &Self, other: &Guid) -> bool` — [`ClsId`](#clsid)

##### `impl Pod for Guid`

##### `impl StructuralPartialEq for Guid`

### `AnonObjectHeader`

```rust
struct AnonObjectHeader {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub class_id: ClsId,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

Non-COFF Object file header

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be 0xffff

- **`version`**: `crate::endian::U16<crate::endian::LittleEndian>`

  >= 1 (implies the ClsId field is present)

- **`class_id`**: `ClsId`

  Used to invoke CoCreateInstance

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of data that follows the header

#### Trait Implementations

##### `impl Clone for AnonObjectHeader`

- `fn clone(self: &Self) -> AnonObjectHeader` — [`AnonObjectHeader`](#anonobjectheader)

##### `impl Copy for AnonObjectHeader`

##### `impl Debug for AnonObjectHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for AnonObjectHeader`

### `AnonObjectHeaderV2`

```rust
struct AnonObjectHeaderV2 {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub class_id: ClsId,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_offset: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be 0xffff

- **`version`**: `crate::endian::U16<crate::endian::LittleEndian>`

  >= 2 (implies the Flags field is present - otherwise V1)

- **`class_id`**: `ClsId`

  Used to invoke CoCreateInstance

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of data that follows the header

- **`flags`**: `crate::endian::U32<crate::endian::LittleEndian>`

  0x1 -> contains metadata

- **`meta_data_size`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of CLR metadata

- **`meta_data_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of CLR metadata

#### Trait Implementations

##### `impl Clone for AnonObjectHeaderV2`

- `fn clone(self: &Self) -> AnonObjectHeaderV2` — [`AnonObjectHeaderV2`](#anonobjectheaderv2)

##### `impl Copy for AnonObjectHeaderV2`

##### `impl Debug for AnonObjectHeaderV2`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for AnonObjectHeaderV2`

### `AnonObjectHeaderBigobj`

```rust
struct AnonObjectHeaderBigobj {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub class_id: ClsId,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub meta_data_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_sections: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_symbol_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be 0xffff

- **`version`**: `crate::endian::U16<crate::endian::LittleEndian>`

  >= 2 (implies the Flags field is present)

- **`machine`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Actual machine - IMAGE_FILE_MACHINE_xxx

- **`class_id`**: `ClsId`

  Must be `ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`.

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of data that follows the header

- **`flags`**: `crate::endian::U32<crate::endian::LittleEndian>`

  0x1 -> contains metadata

- **`meta_data_size`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Size of CLR metadata

- **`meta_data_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Offset of CLR metadata

- **`number_of_sections`**: `crate::endian::U32<crate::endian::LittleEndian>`

  extended from WORD

#### Trait Implementations

##### `impl Clone for AnonObjectHeaderBigobj`

- `fn clone(self: &Self) -> AnonObjectHeaderBigobj` — [`AnonObjectHeaderBigobj`](#anonobjectheaderbigobj)

##### `impl CoffHeader for pe::AnonObjectHeaderBigobj`

- `type ImageSymbol = ImageSymbolEx`

- `type ImageSymbolBytes = ImageSymbolExBytes`

- `fn is_type_bigobj() -> bool`

- `fn machine(self: &Self) -> u16`

- `fn number_of_sections(self: &Self) -> u32`

- `fn pointer_to_symbol_table(self: &Self) -> u32`

- `fn number_of_symbols(self: &Self) -> u32`

- `fn characteristics(self: &Self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>` — [`Result`](../index.md)

##### `impl Copy for AnonObjectHeaderBigobj`

##### `impl Debug for AnonObjectHeaderBigobj`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for AnonObjectHeaderBigobj`

### `ImageSectionHeader`

```rust
struct ImageSectionHeader {
    pub name: [u8; 8],
    pub virtual_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub virtual_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_relocations: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_linenumbers: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_relocations: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_linenumbers: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Implementations

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../index.md)

#### Trait Implementations

##### `impl Clone for ImageSectionHeader`

- `fn clone(self: &Self) -> ImageSectionHeader` — [`ImageSectionHeader`](#imagesectionheader)

##### `impl Copy for ImageSectionHeader`

##### `impl Debug for ImageSectionHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ImageSectionHeader`

- `fn default() -> ImageSectionHeader` — [`ImageSectionHeader`](#imagesectionheader)

##### `impl Pod for ImageSectionHeader`

### `ImageSymbol`

```rust
struct ImageSymbol {
    pub name: [u8; 8],
    pub value: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub section_number: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub typ: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub storage_class: u8,
    pub number_of_aux_symbols: u8,
}
```

#### Fields

- **`name`**: `[u8; 8]`

  If first 4 bytes are 0, then second 4 bytes are offset into string table.

#### Trait Implementations

##### `impl Clone for ImageSymbol`

- `fn clone(self: &Self) -> ImageSymbol` — [`ImageSymbol`](#imagesymbol)

##### `impl Copy for ImageSymbol`

##### `impl Debug for ImageSymbol`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageSymbol for pe::ImageSymbol`

- `fn raw_name(self: &Self) -> &[u8; 8]`

- `fn value(self: &Self) -> u32`

- `fn section_number(self: &Self) -> i32`

- `fn typ(self: &Self) -> u16`

- `fn storage_class(self: &Self) -> u8`

- `fn number_of_aux_symbols(self: &Self) -> u8`

##### `impl Pod for ImageSymbol`

### `ImageSymbolBytes`

```rust
struct ImageSymbolBytes([u8; 18]);
```

#### Trait Implementations

##### `impl Clone for ImageSymbolBytes`

- `fn clone(self: &Self) -> ImageSymbolBytes` — [`ImageSymbolBytes`](#imagesymbolbytes)

##### `impl Copy for ImageSymbolBytes`

##### `impl Debug for ImageSymbolBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageSymbolBytes`

### `ImageSymbolEx`

```rust
struct ImageSymbolEx {
    pub name: [u8; 8],
    pub value: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub section_number: crate::endian::I32Bytes<crate::endian::LittleEndian>,
    pub typ: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub storage_class: u8,
    pub number_of_aux_symbols: u8,
}
```

#### Fields

- **`name`**: `[u8; 8]`

  If first 4 bytes are 0, then second 4 bytes are offset into string table.

#### Trait Implementations

##### `impl Clone for ImageSymbolEx`

- `fn clone(self: &Self) -> ImageSymbolEx` — [`ImageSymbolEx`](#imagesymbolex)

##### `impl Copy for ImageSymbolEx`

##### `impl Debug for ImageSymbolEx`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageSymbol for pe::ImageSymbolEx`

- `fn raw_name(self: &Self) -> &[u8; 8]`

- `fn value(self: &Self) -> u32`

- `fn section_number(self: &Self) -> i32`

- `fn typ(self: &Self) -> u16`

- `fn storage_class(self: &Self) -> u8`

- `fn number_of_aux_symbols(self: &Self) -> u8`

##### `impl Pod for ImageSymbolEx`

### `ImageSymbolExBytes`

```rust
struct ImageSymbolExBytes([u8; 20]);
```

#### Trait Implementations

##### `impl Clone for ImageSymbolExBytes`

- `fn clone(self: &Self) -> ImageSymbolExBytes` — [`ImageSymbolExBytes`](#imagesymbolexbytes)

##### `impl Copy for ImageSymbolExBytes`

##### `impl Debug for ImageSymbolExBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageSymbolExBytes`

### `ImageAuxSymbolTokenDef`

```rust
struct ImageAuxSymbolTokenDef {
    pub aux_type: u8,
    pub reserved1: u8,
    pub symbol_table_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub reserved2: [u8; 12],
}
```

#### Fields

- **`aux_type`**: `u8`

  IMAGE_AUX_SYMBOL_TYPE

- **`reserved1`**: `u8`

  Must be 0

- **`reserved2`**: `[u8; 12]`

  Must be 0

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolTokenDef`

- `fn clone(self: &Self) -> ImageAuxSymbolTokenDef` — [`ImageAuxSymbolTokenDef`](#imageauxsymboltokendef)

##### `impl Copy for ImageAuxSymbolTokenDef`

##### `impl Debug for ImageAuxSymbolTokenDef`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAuxSymbolTokenDef`

### `ImageAuxSymbolFunction`

```rust
struct ImageAuxSymbolFunction {
    pub tag_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub total_size: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub pointer_to_linenumber: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub pointer_to_next_function: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub unused: [u8; 2],
}
```

Auxiliary symbol format 1: function definitions.

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolFunction`

- `fn clone(self: &Self) -> ImageAuxSymbolFunction` — [`ImageAuxSymbolFunction`](#imageauxsymbolfunction)

##### `impl Copy for ImageAuxSymbolFunction`

##### `impl Debug for ImageAuxSymbolFunction`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAuxSymbolFunction`

### `ImageAuxSymbolFunctionBeginEnd`

```rust
struct ImageAuxSymbolFunctionBeginEnd {
    pub unused1: [u8; 4],
    pub linenumber: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub unused2: [u8; 6],
    pub pointer_to_next_function: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub unused3: [u8; 2],
}
```

Auxiliary symbol format 2: .bf and .ef symbols.

#### Fields

- **`linenumber`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  declaration line number

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolFunctionBeginEnd`

- `fn clone(self: &Self) -> ImageAuxSymbolFunctionBeginEnd` — [`ImageAuxSymbolFunctionBeginEnd`](#imageauxsymbolfunctionbeginend)

##### `impl Copy for ImageAuxSymbolFunctionBeginEnd`

##### `impl Debug for ImageAuxSymbolFunctionBeginEnd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAuxSymbolFunctionBeginEnd`

### `ImageAuxSymbolWeak`

```rust
struct ImageAuxSymbolWeak {
    pub weak_default_sym_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub weak_search_type: crate::endian::U32Bytes<crate::endian::LittleEndian>,
}
```

Auxiliary symbol format 3: weak externals.

Used for both `ImageSymbol` and `ImageSymbolEx` (both with padding).

#### Fields

- **`weak_default_sym_index`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  the weak extern default symbol index

#### Implementations

- `fn default_symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolWeak`

- `fn clone(self: &Self) -> ImageAuxSymbolWeak` — [`ImageAuxSymbolWeak`](#imageauxsymbolweak)

##### `impl Copy for ImageAuxSymbolWeak`

##### `impl Debug for ImageAuxSymbolWeak`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAuxSymbolWeak`

### `ImageAuxSymbolSection`

```rust
struct ImageAuxSymbolSection {
    pub length: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub number_of_relocations: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub number_of_linenumbers: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub number: crate::endian::U16Bytes<crate::endian::LittleEndian>,
    pub selection: u8,
    pub reserved: u8,
    pub high_number: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

Auxiliary symbol format 5: sections.

Used for both `ImageSymbol` and `ImageSymbolEx` (with padding).

#### Fields

- **`length`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  section length

- **`number_of_relocations`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  number of relocation entries

- **`number_of_linenumbers`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  number of line numbers

- **`check_sum`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  checksum for communal

- **`number`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  section number to associate with

- **`selection`**: `u8`

  communal selection type

- **`high_number`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  high bits of the section number

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolSection`

- `fn clone(self: &Self) -> ImageAuxSymbolSection` — [`ImageAuxSymbolSection`](#imageauxsymbolsection)

##### `impl Copy for ImageAuxSymbolSection`

##### `impl Debug for ImageAuxSymbolSection`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAuxSymbolSection`

### `ImageAuxSymbolCrc`

```rust
struct ImageAuxSymbolCrc {
    pub crc: crate::endian::U32Bytes<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageAuxSymbolCrc`

- `fn clone(self: &Self) -> ImageAuxSymbolCrc` — [`ImageAuxSymbolCrc`](#imageauxsymbolcrc)

##### `impl Copy for ImageAuxSymbolCrc`

##### `impl Debug for ImageAuxSymbolCrc`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAuxSymbolCrc`

### `ImageRelocation`

```rust
struct ImageRelocation {
    pub virtual_address: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub symbol_table_index: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub typ: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

#### Fields

- **`virtual_address`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  Also `RelocCount` when IMAGE_SCN_LNK_NRELOC_OVFL is set

#### Implementations

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

#### Trait Implementations

##### `impl Clone for ImageRelocation`

- `fn clone(self: &Self) -> ImageRelocation` — [`ImageRelocation`](#imagerelocation)

##### `impl Copy for ImageRelocation`

##### `impl Debug for ImageRelocation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageRelocation`

### `ImageLinenumber`

```rust
struct ImageLinenumber {
    pub symbol_table_index_or_virtual_address: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub linenumber: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

#### Fields

- **`symbol_table_index_or_virtual_address`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  Symbol table index of function name if Linenumber is 0.
  Otherwise virtual address of line number.

- **`linenumber`**: `crate::endian::U16Bytes<crate::endian::LittleEndian>`

  Line number.

#### Trait Implementations

##### `impl Clone for ImageLinenumber`

- `fn clone(self: &Self) -> ImageLinenumber` — [`ImageLinenumber`](#imagelinenumber)

##### `impl Copy for ImageLinenumber`

##### `impl Debug for ImageLinenumber`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageLinenumber`

### `ImageBaseRelocation`

```rust
struct ImageBaseRelocation {
    pub virtual_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_block: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageBaseRelocation`

- `fn clone(self: &Self) -> ImageBaseRelocation` — [`ImageBaseRelocation`](#imagebaserelocation)

##### `impl Copy for ImageBaseRelocation`

##### `impl Debug for ImageBaseRelocation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageBaseRelocation`

### `ImageArchiveMemberHeader`

```rust
struct ImageArchiveMemberHeader {
    pub name: [u8; 16],
    pub date: [u8; 12],
    pub user_id: [u8; 6],
    pub group_id: [u8; 6],
    pub mode: [u8; 8],
    pub size: [u8; 10],
    pub end_header: [u8; 2],
}
```

#### Fields

- **`name`**: `[u8; 16]`

  File member name - `/' terminated.

- **`date`**: `[u8; 12]`

  File member date - decimal.

- **`user_id`**: `[u8; 6]`

  File member user id - decimal.

- **`group_id`**: `[u8; 6]`

  File member group id - decimal.

- **`mode`**: `[u8; 8]`

  File member mode - octal.

- **`size`**: `[u8; 10]`

  File member size - decimal.

- **`end_header`**: `[u8; 2]`

  String to end header.

#### Trait Implementations

##### `impl Clone for ImageArchiveMemberHeader`

- `fn clone(self: &Self) -> ImageArchiveMemberHeader` — [`ImageArchiveMemberHeader`](#imagearchivememberheader)

##### `impl Copy for ImageArchiveMemberHeader`

##### `impl Debug for ImageArchiveMemberHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageArchiveMemberHeader`

### `ImageExportDirectory`

```rust
struct ImageExportDirectory {
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub name: crate::endian::U32<crate::endian::LittleEndian>,
    pub base: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_functions: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_names: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_functions: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_names: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_name_ordinals: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`address_of_functions`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA from base of image

- **`address_of_names`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA from base of image

- **`address_of_name_ordinals`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA from base of image

#### Trait Implementations

##### `impl Clone for ImageExportDirectory`

- `fn clone(self: &Self) -> ImageExportDirectory` — [`ImageExportDirectory`](#imageexportdirectory)

##### `impl Copy for ImageExportDirectory`

##### `impl Debug for ImageExportDirectory`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageExportDirectory`

### `ImageImportByName`

```rust
struct ImageImportByName {
    pub hint: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageImportByName`

- `fn clone(self: &Self) -> ImageImportByName` — [`ImageImportByName`](#imageimportbyname)

##### `impl Copy for ImageImportByName`

##### `impl Debug for ImageImportByName`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageImportByName`

### `ImageThunkData64`

```rust
struct ImageThunkData64(crate::endian::U64<crate::endian::LittleEndian>);
```

#### Trait Implementations

##### `impl Clone for ImageThunkData64`

- `fn clone(self: &Self) -> ImageThunkData64` — [`ImageThunkData64`](#imagethunkdata64)

##### `impl Copy for ImageThunkData64`

##### `impl Debug for ImageThunkData64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageThunkData for pe::ImageThunkData64`

- `fn raw(self: Self) -> u64`

- `fn is_ordinal(self: Self) -> bool`

- `fn ordinal(self: Self) -> u16`

- `fn address(self: Self) -> u32`

##### `impl Pod for ImageThunkData64`

### `ImageThunkData32`

```rust
struct ImageThunkData32(crate::endian::U32<crate::endian::LittleEndian>);
```

#### Trait Implementations

##### `impl Clone for ImageThunkData32`

- `fn clone(self: &Self) -> ImageThunkData32` — [`ImageThunkData32`](#imagethunkdata32)

##### `impl Copy for ImageThunkData32`

##### `impl Debug for ImageThunkData32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl ImageThunkData for pe::ImageThunkData32`

- `fn raw(self: Self) -> u64`

- `fn is_ordinal(self: Self) -> bool`

- `fn ordinal(self: Self) -> u16`

- `fn address(self: Self) -> u32`

##### `impl Pod for ImageThunkData32`

### `ImageTlsDirectory64`

```rust
struct ImageTlsDirectory64 {
    pub start_address_of_raw_data: crate::endian::U64<crate::endian::LittleEndian>,
    pub end_address_of_raw_data: crate::endian::U64<crate::endian::LittleEndian>,
    pub address_of_index: crate::endian::U64<crate::endian::LittleEndian>,
    pub address_of_call_backs: crate::endian::U64<crate::endian::LittleEndian>,
    pub size_of_zero_fill: crate::endian::U32<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`address_of_index`**: `crate::endian::U64<crate::endian::LittleEndian>`

  PDWORD

- **`address_of_call_backs`**: `crate::endian::U64<crate::endian::LittleEndian>`

  PIMAGE_TLS_CALLBACK *;

#### Trait Implementations

##### `impl Clone for ImageTlsDirectory64`

- `fn clone(self: &Self) -> ImageTlsDirectory64` — [`ImageTlsDirectory64`](#imagetlsdirectory64)

##### `impl Copy for ImageTlsDirectory64`

##### `impl Debug for ImageTlsDirectory64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageTlsDirectory64`

### `ImageTlsDirectory32`

```rust
struct ImageTlsDirectory32 {
    pub start_address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_index: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_call_backs: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_zero_fill: crate::endian::U32<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`address_of_index`**: `crate::endian::U32<crate::endian::LittleEndian>`

  PDWORD

- **`address_of_call_backs`**: `crate::endian::U32<crate::endian::LittleEndian>`

  PIMAGE_TLS_CALLBACK *

#### Trait Implementations

##### `impl Clone for ImageTlsDirectory32`

- `fn clone(self: &Self) -> ImageTlsDirectory32` — [`ImageTlsDirectory32`](#imagetlsdirectory32)

##### `impl Copy for ImageTlsDirectory32`

##### `impl Debug for ImageTlsDirectory32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageTlsDirectory32`

### `ImageImportDescriptor`

```rust
struct ImageImportDescriptor {
    pub original_first_thunk: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub forwarder_chain: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub name: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub first_thunk: crate::endian::U32Bytes<crate::endian::LittleEndian>,
}
```

#### Fields

- **`original_first_thunk`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  RVA to original unbound IAT (`ImageThunkData32`/`ImageThunkData64`)
  0 for terminating null import descriptor

- **`time_date_stamp`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  0 if not bound,
  -1 if bound, and real date\time stamp
      in IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT (new BIND)
  O.W. date/time stamp of DLL bound to (Old BIND)

- **`forwarder_chain`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  -1 if no forwarders

- **`first_thunk`**: `crate::endian::U32Bytes<crate::endian::LittleEndian>`

  RVA to IAT (if bound this IAT has actual addresses)

#### Implementations

- `fn is_null(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for ImageImportDescriptor`

- `fn clone(self: &Self) -> ImageImportDescriptor` — [`ImageImportDescriptor`](#imageimportdescriptor)

##### `impl Copy for ImageImportDescriptor`

##### `impl Debug for ImageImportDescriptor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageImportDescriptor`

### `ImageBoundImportDescriptor`

```rust
struct ImageBoundImportDescriptor {
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub offset_module_name: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_module_forwarder_refs: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageBoundImportDescriptor`

- `fn clone(self: &Self) -> ImageBoundImportDescriptor` — [`ImageBoundImportDescriptor`](#imageboundimportdescriptor)

##### `impl Copy for ImageBoundImportDescriptor`

##### `impl Debug for ImageBoundImportDescriptor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageBoundImportDescriptor`

### `ImageBoundForwarderRef`

```rust
struct ImageBoundForwarderRef {
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub offset_module_name: crate::endian::U16<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageBoundForwarderRef`

- `fn clone(self: &Self) -> ImageBoundForwarderRef` — [`ImageBoundForwarderRef`](#imageboundforwarderref)

##### `impl Copy for ImageBoundForwarderRef`

##### `impl Debug for ImageBoundForwarderRef`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageBoundForwarderRef`

### `ImageDelayloadDescriptor`

```rust
struct ImageDelayloadDescriptor {
    pub attributes: crate::endian::U32<crate::endian::LittleEndian>,
    pub dll_name_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub module_handle_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_address_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_name_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub bound_import_address_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub unload_information_table_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`dll_name_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the name of the target library (NULL-terminate ASCII string)

- **`module_handle_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the HMODULE caching location (PHMODULE)

- **`import_address_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the start of the IAT (PIMAGE_THUNK_DATA)

- **`import_name_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to the start of the name table (PIMAGE_THUNK_DATA::AddressOfData)

- **`bound_import_address_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to an optional bound IAT

- **`unload_information_table_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA to an optional unload info table

- **`time_date_stamp`**: `crate::endian::U32<crate::endian::LittleEndian>`

  0 if not bound, otherwise, date/time of the target DLL

#### Implementations

- `fn is_null(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for ImageDelayloadDescriptor`

- `fn clone(self: &Self) -> ImageDelayloadDescriptor` — [`ImageDelayloadDescriptor`](#imagedelayloaddescriptor)

##### `impl Copy for ImageDelayloadDescriptor`

##### `impl Debug for ImageDelayloadDescriptor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDelayloadDescriptor`

### `ImageResourceDirectory`

```rust
struct ImageResourceDirectory {
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_named_entries: crate::endian::U16<crate::endian::LittleEndian>,
    pub number_of_id_entries: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageResourceDirectory`

- `fn clone(self: &Self) -> ImageResourceDirectory` — [`ImageResourceDirectory`](#imageresourcedirectory)

##### `impl Copy for ImageResourceDirectory`

##### `impl Debug for ImageResourceDirectory`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageResourceDirectory`

### `ImageResourceDirectoryEntry`

```rust
struct ImageResourceDirectoryEntry {
    pub name_or_id: crate::endian::U32<crate::endian::LittleEndian>,
    pub offset_to_data_or_directory: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Implementations

- `fn has_name(self: &Self) -> bool`

- `fn name(self: &Self) -> ResourceName` — [`ResourceName`](../read/pe/index.md)

- `fn id(self: &Self) -> u16`

- `fn name_or_id(self: &Self) -> ResourceNameOrId` — [`ResourceNameOrId`](../read/pe/index.md)

- `fn is_table(self: &Self) -> bool`

- `fn data_offset(self: &Self) -> u32`

- `fn data<'data>(self: &Self, section: ResourceDirectory<'data>) -> Result<ResourceDirectoryEntryData<'data>>` — [`ResourceDirectory`](../read/pe/index.md), [`Result`](../index.md), [`ResourceDirectoryEntryData`](../read/pe/index.md)

#### Trait Implementations

##### `impl Clone for ImageResourceDirectoryEntry`

- `fn clone(self: &Self) -> ImageResourceDirectoryEntry` — [`ImageResourceDirectoryEntry`](#imageresourcedirectoryentry)

##### `impl Copy for ImageResourceDirectoryEntry`

##### `impl Debug for ImageResourceDirectoryEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageResourceDirectoryEntry`

### `ImageResourceDirectoryString`

```rust
struct ImageResourceDirectoryString {
    pub length: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageResourceDirectoryString`

- `fn clone(self: &Self) -> ImageResourceDirectoryString` — [`ImageResourceDirectoryString`](#imageresourcedirectorystring)

##### `impl Copy for ImageResourceDirectoryString`

##### `impl Debug for ImageResourceDirectoryString`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageResourceDirectoryString`

### `ImageResourceDirStringU`

```rust
struct ImageResourceDirStringU {
    pub length: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageResourceDirStringU`

- `fn clone(self: &Self) -> ImageResourceDirStringU` — [`ImageResourceDirStringU`](#imageresourcedirstringu)

##### `impl Copy for ImageResourceDirStringU`

##### `impl Debug for ImageResourceDirStringU`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageResourceDirStringU`

### `ImageResourceDataEntry`

```rust
struct ImageResourceDataEntry {
    pub offset_to_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_page: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`offset_to_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA of the data.

#### Trait Implementations

##### `impl Clone for ImageResourceDataEntry`

- `fn clone(self: &Self) -> ImageResourceDataEntry` — [`ImageResourceDataEntry`](#imageresourcedataentry)

##### `impl Copy for ImageResourceDataEntry`

##### `impl Debug for ImageResourceDataEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageResourceDataEntry`

### `ImageLoadConfigCodeIntegrity`

```rust
struct ImageLoadConfigCodeIntegrity {
    pub flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub catalog: crate::endian::U16<crate::endian::LittleEndian>,
    pub catalog_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`flags`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Flags to indicate if CI information is available, etc.

- **`catalog`**: `crate::endian::U16<crate::endian::LittleEndian>`

  0xFFFF means not available

- **`reserved`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Additional bitmask to be defined later

#### Trait Implementations

##### `impl Clone for ImageLoadConfigCodeIntegrity`

- `fn clone(self: &Self) -> ImageLoadConfigCodeIntegrity` — [`ImageLoadConfigCodeIntegrity`](#imageloadconfigcodeintegrity)

##### `impl Copy for ImageLoadConfigCodeIntegrity`

##### `impl Debug for ImageLoadConfigCodeIntegrity`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageLoadConfigCodeIntegrity`

### `ImageDynamicRelocationTable`

```rust
struct ImageDynamicRelocationTable {
    pub version: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocationTable`

- `fn clone(self: &Self) -> ImageDynamicRelocationTable` — [`ImageDynamicRelocationTable`](#imagedynamicrelocationtable)

##### `impl Copy for ImageDynamicRelocationTable`

##### `impl Debug for ImageDynamicRelocationTable`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDynamicRelocationTable`

### `ImageDynamicRelocation32`

```rust
struct ImageDynamicRelocation32 {
    pub symbol: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_reloc_size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation32`

- `fn clone(self: &Self) -> ImageDynamicRelocation32` — [`ImageDynamicRelocation32`](#imagedynamicrelocation32)

##### `impl Copy for ImageDynamicRelocation32`

##### `impl Debug for ImageDynamicRelocation32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDynamicRelocation32`

### `ImageDynamicRelocation64`

```rust
struct ImageDynamicRelocation64 {
    pub symbol: crate::endian::U64<crate::endian::LittleEndian>,
    pub base_reloc_size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation64`

- `fn clone(self: &Self) -> ImageDynamicRelocation64` — [`ImageDynamicRelocation64`](#imagedynamicrelocation64)

##### `impl Copy for ImageDynamicRelocation64`

##### `impl Debug for ImageDynamicRelocation64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDynamicRelocation64`

### `ImageDynamicRelocation32V2`

```rust
struct ImageDynamicRelocation32V2 {
    pub header_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub fixup_info_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub symbol: crate::endian::U32<crate::endian::LittleEndian>,
    pub symbol_group: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation32V2`

- `fn clone(self: &Self) -> ImageDynamicRelocation32V2` — [`ImageDynamicRelocation32V2`](#imagedynamicrelocation32v2)

##### `impl Copy for ImageDynamicRelocation32V2`

##### `impl Debug for ImageDynamicRelocation32V2`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDynamicRelocation32V2`

### `ImageDynamicRelocation64V2`

```rust
struct ImageDynamicRelocation64V2 {
    pub header_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub fixup_info_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub symbol: crate::endian::U64<crate::endian::LittleEndian>,
    pub symbol_group: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageDynamicRelocation64V2`

- `fn clone(self: &Self) -> ImageDynamicRelocation64V2` — [`ImageDynamicRelocation64V2`](#imagedynamicrelocation64v2)

##### `impl Copy for ImageDynamicRelocation64V2`

##### `impl Debug for ImageDynamicRelocation64V2`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDynamicRelocation64V2`

### `ImagePrologueDynamicRelocationHeader`

```rust
struct ImagePrologueDynamicRelocationHeader {
    pub prologue_byte_count: u8,
}
```

#### Trait Implementations

##### `impl Clone for ImagePrologueDynamicRelocationHeader`

- `fn clone(self: &Self) -> ImagePrologueDynamicRelocationHeader` — [`ImagePrologueDynamicRelocationHeader`](#imageprologuedynamicrelocationheader)

##### `impl Copy for ImagePrologueDynamicRelocationHeader`

##### `impl Debug for ImagePrologueDynamicRelocationHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImagePrologueDynamicRelocationHeader`

### `ImageEpilogueDynamicRelocationHeader`

```rust
struct ImageEpilogueDynamicRelocationHeader {
    pub epilogue_count: crate::endian::U32Bytes<crate::endian::LittleEndian>,
    pub epilogue_byte_count: u8,
    pub branch_descriptor_element_size: u8,
    pub branch_descriptor_count: crate::endian::U16Bytes<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageEpilogueDynamicRelocationHeader`

- `fn clone(self: &Self) -> ImageEpilogueDynamicRelocationHeader` — [`ImageEpilogueDynamicRelocationHeader`](#imageepiloguedynamicrelocationheader)

##### `impl Copy for ImageEpilogueDynamicRelocationHeader`

##### `impl Debug for ImageEpilogueDynamicRelocationHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageEpilogueDynamicRelocationHeader`

### `ImageLoadConfigDirectory32`

```rust
struct ImageLoadConfigDirectory32 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub global_flags_clear: crate::endian::U32<crate::endian::LittleEndian>,
    pub global_flags_set: crate::endian::U32<crate::endian::LittleEndian>,
    pub critical_section_default_timeout: crate::endian::U32<crate::endian::LittleEndian>,
    pub de_commit_free_block_threshold: crate::endian::U32<crate::endian::LittleEndian>,
    pub de_commit_total_free_threshold: crate::endian::U32<crate::endian::LittleEndian>,
    pub lock_prefix_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub maximum_allocation_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub virtual_memory_threshold: crate::endian::U32<crate::endian::LittleEndian>,
    pub process_heap_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub process_affinity_mask: crate::endian::U32<crate::endian::LittleEndian>,
    pub csd_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub dependent_load_flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub edit_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub security_cookie: crate::endian::U32<crate::endian::LittleEndian>,
    pub sehandler_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub sehandler_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_check_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_dispatch_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_function_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_cf_function_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity: ImageLoadConfigCodeIntegrity,
    pub guard_address_taken_iat_entry_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_address_taken_iat_entry_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_long_jump_target_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_long_jump_target_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub chpe_metadata_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine: crate::endian::U32<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_section: crate::endian::U16<crate::endian::LittleEndian>,
    pub reserved2: crate::endian::U16<crate::endian::LittleEndian>,
    pub guard_rf_verify_stack_pointer_function_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub hot_patch_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved3: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_configuration_pointer: crate::endian::U32<crate::endian::LittleEndian>,
    pub volatile_metadata_pointer: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`lock_prefix_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`edit_list`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`security_cookie`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`sehandler_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_cf_check_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_cf_dispatch_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_cf_function_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_address_taken_iat_entry_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_long_jump_target_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`dynamic_value_reloc_table`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`guard_rf_verify_stack_pointer_function_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`enclave_configuration_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

- **`volatile_metadata_pointer`**: `crate::endian::U32<crate::endian::LittleEndian>`

  VA

#### Trait Implementations

##### `impl Clone for ImageLoadConfigDirectory32`

- `fn clone(self: &Self) -> ImageLoadConfigDirectory32` — [`ImageLoadConfigDirectory32`](#imageloadconfigdirectory32)

##### `impl Copy for ImageLoadConfigDirectory32`

##### `impl Debug for ImageLoadConfigDirectory32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageLoadConfigDirectory32`

### `ImageLoadConfigDirectory64`

```rust
struct ImageLoadConfigDirectory64 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub global_flags_clear: crate::endian::U32<crate::endian::LittleEndian>,
    pub global_flags_set: crate::endian::U32<crate::endian::LittleEndian>,
    pub critical_section_default_timeout: crate::endian::U32<crate::endian::LittleEndian>,
    pub de_commit_free_block_threshold: crate::endian::U64<crate::endian::LittleEndian>,
    pub de_commit_total_free_threshold: crate::endian::U64<crate::endian::LittleEndian>,
    pub lock_prefix_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub maximum_allocation_size: crate::endian::U64<crate::endian::LittleEndian>,
    pub virtual_memory_threshold: crate::endian::U64<crate::endian::LittleEndian>,
    pub process_affinity_mask: crate::endian::U64<crate::endian::LittleEndian>,
    pub process_heap_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub csd_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub dependent_load_flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub edit_list: crate::endian::U64<crate::endian::LittleEndian>,
    pub security_cookie: crate::endian::U64<crate::endian::LittleEndian>,
    pub sehandler_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub sehandler_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_check_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_dispatch_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_function_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_cf_function_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity: ImageLoadConfigCodeIntegrity,
    pub guard_address_taken_iat_entry_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_address_taken_iat_entry_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_long_jump_target_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_long_jump_target_count: crate::endian::U64<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table: crate::endian::U64<crate::endian::LittleEndian>,
    pub chpe_metadata_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine: crate::endian::U64<crate::endian::LittleEndian>,
    pub guard_rf_failure_routine_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub dynamic_value_reloc_table_section: crate::endian::U16<crate::endian::LittleEndian>,
    pub reserved2: crate::endian::U16<crate::endian::LittleEndian>,
    pub guard_rf_verify_stack_pointer_function_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub hot_patch_table_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved3: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_configuration_pointer: crate::endian::U64<crate::endian::LittleEndian>,
    pub volatile_metadata_pointer: crate::endian::U64<crate::endian::LittleEndian>,
}
```

#### Fields

- **`lock_prefix_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`edit_list`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`security_cookie`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`sehandler_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_cf_check_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_cf_dispatch_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_cf_function_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_address_taken_iat_entry_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_long_jump_target_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`dynamic_value_reloc_table`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`chpe_metadata_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_rf_failure_routine_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`guard_rf_verify_stack_pointer_function_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`enclave_configuration_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

- **`volatile_metadata_pointer`**: `crate::endian::U64<crate::endian::LittleEndian>`

  VA

#### Trait Implementations

##### `impl Clone for ImageLoadConfigDirectory64`

- `fn clone(self: &Self) -> ImageLoadConfigDirectory64` — [`ImageLoadConfigDirectory64`](#imageloadconfigdirectory64)

##### `impl Copy for ImageLoadConfigDirectory64`

##### `impl Debug for ImageLoadConfigDirectory64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageLoadConfigDirectory64`

### `ImageHotPatchInfo`

```rust
struct ImageHotPatchInfo {
    pub version: crate::endian::U32<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub sequence_number: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_image_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub base_image_count: crate::endian::U32<crate::endian::LittleEndian>,
    pub buffer_offset: crate::endian::U32<crate::endian::LittleEndian>,
    pub extra_patch_size: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`buffer_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Version 2 and later

- **`extra_patch_size`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Version 3 and later

#### Trait Implementations

##### `impl Clone for ImageHotPatchInfo`

- `fn clone(self: &Self) -> ImageHotPatchInfo` — [`ImageHotPatchInfo`](#imagehotpatchinfo)

##### `impl Copy for ImageHotPatchInfo`

##### `impl Debug for ImageHotPatchInfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageHotPatchInfo`

### `ImageHotPatchBase`

```rust
struct ImageHotPatchBase {
    pub sequence_number: crate::endian::U32<crate::endian::LittleEndian>,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub original_time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub original_check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity_info: crate::endian::U32<crate::endian::LittleEndian>,
    pub code_integrity_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub patch_table: crate::endian::U32<crate::endian::LittleEndian>,
    pub buffer_offset: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`buffer_offset`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Version 2 and later

#### Trait Implementations

##### `impl Clone for ImageHotPatchBase`

- `fn clone(self: &Self) -> ImageHotPatchBase` — [`ImageHotPatchBase`](#imagehotpatchbase)

##### `impl Copy for ImageHotPatchBase`

##### `impl Debug for ImageHotPatchBase`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageHotPatchBase`

### `ImageHotPatchHashes`

```rust
struct ImageHotPatchHashes {
    pub sha256: [u8; 32],
    pub sha1: [u8; 20],
}
```

#### Trait Implementations

##### `impl Clone for ImageHotPatchHashes`

- `fn clone(self: &Self) -> ImageHotPatchHashes` — [`ImageHotPatchHashes`](#imagehotpatchhashes)

##### `impl Copy for ImageHotPatchHashes`

##### `impl Debug for ImageHotPatchHashes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageHotPatchHashes`

### `ImageArmRuntimeFunctionEntry`

```rust
struct ImageArmRuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub unwind_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageArmRuntimeFunctionEntry`

- `fn clone(self: &Self) -> ImageArmRuntimeFunctionEntry` — [`ImageArmRuntimeFunctionEntry`](#imagearmruntimefunctionentry)

##### `impl Copy for ImageArmRuntimeFunctionEntry`

##### `impl Debug for ImageArmRuntimeFunctionEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageArmRuntimeFunctionEntry`

### `ImageArm64RuntimeFunctionEntry`

```rust
struct ImageArm64RuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub unwind_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageArm64RuntimeFunctionEntry`

- `fn clone(self: &Self) -> ImageArm64RuntimeFunctionEntry` — [`ImageArm64RuntimeFunctionEntry`](#imagearm64runtimefunctionentry)

##### `impl Copy for ImageArm64RuntimeFunctionEntry`

##### `impl Debug for ImageArm64RuntimeFunctionEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageArm64RuntimeFunctionEntry`

### `ImageAlpha64RuntimeFunctionEntry`

```rust
struct ImageAlpha64RuntimeFunctionEntry {
    pub begin_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub end_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub exception_handler: crate::endian::U64<crate::endian::LittleEndian>,
    pub handler_data: crate::endian::U64<crate::endian::LittleEndian>,
    pub prolog_end_address: crate::endian::U64<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageAlpha64RuntimeFunctionEntry`

- `fn clone(self: &Self) -> ImageAlpha64RuntimeFunctionEntry` — [`ImageAlpha64RuntimeFunctionEntry`](#imagealpha64runtimefunctionentry)

##### `impl Copy for ImageAlpha64RuntimeFunctionEntry`

##### `impl Debug for ImageAlpha64RuntimeFunctionEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAlpha64RuntimeFunctionEntry`

### `ImageAlphaRuntimeFunctionEntry`

```rust
struct ImageAlphaRuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub exception_handler: crate::endian::U32<crate::endian::LittleEndian>,
    pub handler_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub prolog_end_address: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageAlphaRuntimeFunctionEntry`

- `fn clone(self: &Self) -> ImageAlphaRuntimeFunctionEntry` — [`ImageAlphaRuntimeFunctionEntry`](#imagealpharuntimefunctionentry)

##### `impl Copy for ImageAlphaRuntimeFunctionEntry`

##### `impl Debug for ImageAlphaRuntimeFunctionEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageAlphaRuntimeFunctionEntry`

### `ImageRuntimeFunctionEntry`

```rust
struct ImageRuntimeFunctionEntry {
    pub begin_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub unwind_info_address_or_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageRuntimeFunctionEntry`

- `fn clone(self: &Self) -> ImageRuntimeFunctionEntry` — [`ImageRuntimeFunctionEntry`](#imageruntimefunctionentry)

##### `impl Copy for ImageRuntimeFunctionEntry`

##### `impl Debug for ImageRuntimeFunctionEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageRuntimeFunctionEntry`

### `ImageEnclaveConfig32`

```rust
struct ImageEnclaveConfig32 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub minimum_required_config_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub policy_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_imports: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_entry_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub family_id: [u8; 16],
    pub image_id: [u8; 16],
    pub image_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub security_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_threads: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageEnclaveConfig32`

- `fn clone(self: &Self) -> ImageEnclaveConfig32` — [`ImageEnclaveConfig32`](#imageenclaveconfig32)

##### `impl Copy for ImageEnclaveConfig32`

##### `impl Debug for ImageEnclaveConfig32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageEnclaveConfig32`

### `ImageEnclaveConfig64`

```rust
struct ImageEnclaveConfig64 {
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub minimum_required_config_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub policy_flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_imports: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_list: crate::endian::U32<crate::endian::LittleEndian>,
    pub import_entry_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub family_id: [u8; 16],
    pub image_id: [u8; 16],
    pub image_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub security_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_size: crate::endian::U64<crate::endian::LittleEndian>,
    pub number_of_threads: crate::endian::U32<crate::endian::LittleEndian>,
    pub enclave_flags: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageEnclaveConfig64`

- `fn clone(self: &Self) -> ImageEnclaveConfig64` — [`ImageEnclaveConfig64`](#imageenclaveconfig64)

##### `impl Copy for ImageEnclaveConfig64`

##### `impl Debug for ImageEnclaveConfig64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageEnclaveConfig64`

### `ImageEnclaveImport`

```rust
struct ImageEnclaveImport {
    pub match_type: crate::endian::U32<crate::endian::LittleEndian>,
    pub minimum_security_version: crate::endian::U32<crate::endian::LittleEndian>,
    pub unique_or_author_id: [u8; 32],
    pub family_id: [u8; 16],
    pub image_id: [u8; 16],
    pub import_name: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageEnclaveImport`

- `fn clone(self: &Self) -> ImageEnclaveImport` — [`ImageEnclaveImport`](#imageenclaveimport)

##### `impl Copy for ImageEnclaveImport`

##### `impl Debug for ImageEnclaveImport`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageEnclaveImport`

### `ImageDebugDirectory`

```rust
struct ImageDebugDirectory {
    pub characteristics: crate::endian::U32<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub typ: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub address_of_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub pointer_to_raw_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageDebugDirectory`

- `fn clone(self: &Self) -> ImageDebugDirectory` — [`ImageDebugDirectory`](#imagedebugdirectory)

##### `impl Copy for ImageDebugDirectory`

##### `impl Debug for ImageDebugDirectory`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDebugDirectory`

### `ImageCoffSymbolsHeader`

```rust
struct ImageCoffSymbolsHeader {
    pub number_of_symbols: crate::endian::U32<crate::endian::LittleEndian>,
    pub lva_to_first_symbol: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_linenumbers: crate::endian::U32<crate::endian::LittleEndian>,
    pub lva_to_first_linenumber: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_first_byte_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_last_byte_of_code: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_first_byte_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub rva_to_last_byte_of_data: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageCoffSymbolsHeader`

- `fn clone(self: &Self) -> ImageCoffSymbolsHeader` — [`ImageCoffSymbolsHeader`](#imagecoffsymbolsheader)

##### `impl Copy for ImageCoffSymbolsHeader`

##### `impl Debug for ImageCoffSymbolsHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageCoffSymbolsHeader`

### `ImageDebugMisc`

```rust
struct ImageDebugMisc {
    pub data_type: crate::endian::U32<crate::endian::LittleEndian>,
    pub length: crate::endian::U32<crate::endian::LittleEndian>,
    pub unicode: u8,
    pub reserved: [u8; 3],
}
```

#### Fields

- **`data_type`**: `crate::endian::U32<crate::endian::LittleEndian>`

  type of misc data, see defines

- **`length`**: `crate::endian::U32<crate::endian::LittleEndian>`

  total length of record, rounded to four byte multiple.

- **`unicode`**: `u8`

  TRUE if data is unicode string

#### Trait Implementations

##### `impl Clone for ImageDebugMisc`

- `fn clone(self: &Self) -> ImageDebugMisc` — [`ImageDebugMisc`](#imagedebugmisc)

##### `impl Copy for ImageDebugMisc`

##### `impl Debug for ImageDebugMisc`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageDebugMisc`

### `ImageFunctionEntry`

```rust
struct ImageFunctionEntry {
    pub starting_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub ending_address: crate::endian::U32<crate::endian::LittleEndian>,
    pub end_of_prologue: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageFunctionEntry`

- `fn clone(self: &Self) -> ImageFunctionEntry` — [`ImageFunctionEntry`](#imagefunctionentry)

##### `impl Copy for ImageFunctionEntry`

##### `impl Debug for ImageFunctionEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageFunctionEntry`

### `ImageFunctionEntry64`

```rust
struct ImageFunctionEntry64 {
    pub starting_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub ending_address: crate::endian::U64<crate::endian::LittleEndian>,
    pub end_of_prologue_or_unwind_info_address: crate::endian::U64<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for ImageFunctionEntry64`

- `fn clone(self: &Self) -> ImageFunctionEntry64` — [`ImageFunctionEntry64`](#imagefunctionentry64)

##### `impl Copy for ImageFunctionEntry64`

##### `impl Debug for ImageFunctionEntry64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageFunctionEntry64`

### `ImageSeparateDebugHeader`

```rust
struct ImageSeparateDebugHeader {
    pub signature: crate::endian::U16<crate::endian::LittleEndian>,
    pub flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub number_of_sections: crate::endian::U32<crate::endian::LittleEndian>,
    pub exported_names_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub debug_directory_size: crate::endian::U32<crate::endian::LittleEndian>,
    pub section_alignment: crate::endian::U32<crate::endian::LittleEndian>,
    pub reserved: [crate::endian::U32<crate::endian::LittleEndian>; 2],
}
```

#### Trait Implementations

##### `impl Clone for ImageSeparateDebugHeader`

- `fn clone(self: &Self) -> ImageSeparateDebugHeader` — [`ImageSeparateDebugHeader`](#imageseparatedebugheader)

##### `impl Copy for ImageSeparateDebugHeader`

##### `impl Debug for ImageSeparateDebugHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageSeparateDebugHeader`

### `NonPagedDebugInfo`

```rust
struct NonPagedDebugInfo {
    pub signature: crate::endian::U16<crate::endian::LittleEndian>,
    pub flags: crate::endian::U16<crate::endian::LittleEndian>,
    pub size: crate::endian::U32<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub characteristics: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub check_sum: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_image: crate::endian::U32<crate::endian::LittleEndian>,
    pub image_base: crate::endian::U64<crate::endian::LittleEndian>,
}
```

#### Trait Implementations

##### `impl Clone for NonPagedDebugInfo`

- `fn clone(self: &Self) -> NonPagedDebugInfo` — [`NonPagedDebugInfo`](#nonpageddebuginfo)

##### `impl Copy for NonPagedDebugInfo`

##### `impl Debug for NonPagedDebugInfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for NonPagedDebugInfo`

### `ImageArchitectureEntry`

```rust
struct ImageArchitectureEntry {
    pub fixup_inst_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub new_inst: crate::endian::U32<crate::endian::LittleEndian>,
}
```

#### Fields

- **`fixup_inst_rva`**: `crate::endian::U32<crate::endian::LittleEndian>`

  RVA of instruction to fixup

- **`new_inst`**: `crate::endian::U32<crate::endian::LittleEndian>`

  fixup instruction (see alphaops.h)

#### Trait Implementations

##### `impl Clone for ImageArchitectureEntry`

- `fn clone(self: &Self) -> ImageArchitectureEntry` — [`ImageArchitectureEntry`](#imagearchitectureentry)

##### `impl Copy for ImageArchitectureEntry`

##### `impl Debug for ImageArchitectureEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageArchitectureEntry`

### `ImportObjectHeader`

```rust
struct ImportObjectHeader {
    pub sig1: crate::endian::U16<crate::endian::LittleEndian>,
    pub sig2: crate::endian::U16<crate::endian::LittleEndian>,
    pub version: crate::endian::U16<crate::endian::LittleEndian>,
    pub machine: crate::endian::U16<crate::endian::LittleEndian>,
    pub time_date_stamp: crate::endian::U32<crate::endian::LittleEndian>,
    pub size_of_data: crate::endian::U32<crate::endian::LittleEndian>,
    pub ordinal_or_hint: crate::endian::U16<crate::endian::LittleEndian>,
    pub name_type: crate::endian::U16<crate::endian::LittleEndian>,
}
```

#### Fields

- **`sig1`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMAGE_FILE_MACHINE_UNKNOWN

- **`sig2`**: `crate::endian::U16<crate::endian::LittleEndian>`

  Must be IMPORT_OBJECT_HDR_SIG2.

- **`time_date_stamp`**: `crate::endian::U32<crate::endian::LittleEndian>`

  Time/date stamp

- **`size_of_data`**: `crate::endian::U32<crate::endian::LittleEndian>`

  particularly useful for incremental links

- **`ordinal_or_hint`**: `crate::endian::U16<crate::endian::LittleEndian>`

  if grf & IMPORT_OBJECT_ORDINAL

#### Implementations

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>` — [`Result`](../index.md)

- `fn parse_data<'data, R: ReadRef<'data>>(self: &Self, data: R, offset: &mut u64) -> Result<ImportObjectData<'data>>` — [`Result`](../index.md), [`ImportObjectData`](../read/coff/index.md)

- `fn import_type(self: &Self) -> u16`

- `fn name_type(self: &Self) -> u16`

#### Trait Implementations

##### `impl Clone for ImportObjectHeader`

- `fn clone(self: &Self) -> ImportObjectHeader` — [`ImportObjectHeader`](#importobjectheader)

##### `impl Copy for ImportObjectHeader`

##### `impl Debug for ImportObjectHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImportObjectHeader`

### `ImageCor20Header`

```rust
struct ImageCor20Header {
    pub cb: crate::endian::U32<crate::endian::LittleEndian>,
    pub major_runtime_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub minor_runtime_version: crate::endian::U16<crate::endian::LittleEndian>,
    pub meta_data: ImageDataDirectory,
    pub flags: crate::endian::U32<crate::endian::LittleEndian>,
    pub entry_point_token_or_rva: crate::endian::U32<crate::endian::LittleEndian>,
    pub resources: ImageDataDirectory,
    pub strong_name_signature: ImageDataDirectory,
    pub code_manager_table: ImageDataDirectory,
    pub vtable_fixups: ImageDataDirectory,
    pub export_address_table_jumps: ImageDataDirectory,
    pub managed_native_header: ImageDataDirectory,
}
```

#### Trait Implementations

##### `impl Clone for ImageCor20Header`

- `fn clone(self: &Self) -> ImageCor20Header` — [`ImageCor20Header`](#imagecor20header)

##### `impl Copy for ImageCor20Header`

##### `impl Debug for ImageCor20Header`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for ImageCor20Header`

## Constants

### `IMAGE_DOS_SIGNATURE`

```rust
const IMAGE_DOS_SIGNATURE: u16 = 23_117u16;
```

MZ

### `IMAGE_OS2_SIGNATURE`

```rust
const IMAGE_OS2_SIGNATURE: u16 = 17_742u16;
```

NE

### `IMAGE_OS2_SIGNATURE_LE`

```rust
const IMAGE_OS2_SIGNATURE_LE: u16 = 17_740u16;
```

LE

### `IMAGE_VXD_SIGNATURE`

```rust
const IMAGE_VXD_SIGNATURE: u16 = 17_740u16;
```

LE

### `IMAGE_NT_SIGNATURE`

```rust
const IMAGE_NT_SIGNATURE: u32 = 17_744u32;
```

PE00

### `IMAGE_SIZEOF_FILE_HEADER`

```rust
const IMAGE_SIZEOF_FILE_HEADER: usize = 20usize;
```

### `IMAGE_FILE_RELOCS_STRIPPED`

```rust
const IMAGE_FILE_RELOCS_STRIPPED: u16 = 1u16;
```

Relocation info stripped from file.

### `IMAGE_FILE_EXECUTABLE_IMAGE`

```rust
const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 2u16;
```

File is executable  (i.e. no unresolved external references).

### `IMAGE_FILE_LINE_NUMS_STRIPPED`

```rust
const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 4u16;
```

Line numbers stripped from file.

### `IMAGE_FILE_LOCAL_SYMS_STRIPPED`

```rust
const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 8u16;
```

Local symbols stripped from file.

### `IMAGE_FILE_AGGRESIVE_WS_TRIM`

```rust
const IMAGE_FILE_AGGRESIVE_WS_TRIM: u16 = 16u16;
```

Aggressively trim working set

### `IMAGE_FILE_LARGE_ADDRESS_AWARE`

```rust
const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 32u16;
```

App can handle >2gb addresses

### `IMAGE_FILE_BYTES_REVERSED_LO`

```rust
const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 128u16;
```

Bytes of machine word are reversed.

### `IMAGE_FILE_32BIT_MACHINE`

```rust
const IMAGE_FILE_32BIT_MACHINE: u16 = 256u16;
```

32 bit word machine.

### `IMAGE_FILE_DEBUG_STRIPPED`

```rust
const IMAGE_FILE_DEBUG_STRIPPED: u16 = 512u16;
```

Debugging info stripped from file in .DBG file

### `IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP`

```rust
const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 1_024u16;
```

If Image is on removable media, copy and run from the swap file.

### `IMAGE_FILE_NET_RUN_FROM_SWAP`

```rust
const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 2_048u16;
```

If Image is on Net, copy and run from the swap file.

### `IMAGE_FILE_SYSTEM`

```rust
const IMAGE_FILE_SYSTEM: u16 = 4_096u16;
```

System File.

### `IMAGE_FILE_DLL`

```rust
const IMAGE_FILE_DLL: u16 = 8_192u16;
```

File is a DLL.

### `IMAGE_FILE_UP_SYSTEM_ONLY`

```rust
const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 16_384u16;
```

File should only be run on a UP machine

### `IMAGE_FILE_BYTES_REVERSED_HI`

```rust
const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 32_768u16;
```

Bytes of machine word are reversed.

### `IMAGE_FILE_MACHINE_UNKNOWN`

```rust
const IMAGE_FILE_MACHINE_UNKNOWN: u16 = 0u16;
```

### `IMAGE_FILE_MACHINE_TARGET_HOST`

```rust
const IMAGE_FILE_MACHINE_TARGET_HOST: u16 = 1u16;
```

Useful for indicating we want to interact with the host and not a WoW guest.

### `IMAGE_FILE_MACHINE_I386`

```rust
const IMAGE_FILE_MACHINE_I386: u16 = 332u16;
```

Intel 386.

### `IMAGE_FILE_MACHINE_R3000`

```rust
const IMAGE_FILE_MACHINE_R3000: u16 = 354u16;
```

MIPS little-endian, 0x160 big-endian

### `IMAGE_FILE_MACHINE_R4000`

```rust
const IMAGE_FILE_MACHINE_R4000: u16 = 358u16;
```

MIPS little-endian

### `IMAGE_FILE_MACHINE_R10000`

```rust
const IMAGE_FILE_MACHINE_R10000: u16 = 360u16;
```

MIPS little-endian

### `IMAGE_FILE_MACHINE_WCEMIPSV2`

```rust
const IMAGE_FILE_MACHINE_WCEMIPSV2: u16 = 361u16;
```

MIPS little-endian WCE v2

### `IMAGE_FILE_MACHINE_ALPHA`

```rust
const IMAGE_FILE_MACHINE_ALPHA: u16 = 388u16;
```

Alpha_AXP

### `IMAGE_FILE_MACHINE_SH3`

```rust
const IMAGE_FILE_MACHINE_SH3: u16 = 418u16;
```

SH3 little-endian

### `IMAGE_FILE_MACHINE_SH3DSP`

```rust
const IMAGE_FILE_MACHINE_SH3DSP: u16 = 419u16;
```

### `IMAGE_FILE_MACHINE_SH3E`

```rust
const IMAGE_FILE_MACHINE_SH3E: u16 = 420u16;
```

SH3E little-endian

### `IMAGE_FILE_MACHINE_SH4`

```rust
const IMAGE_FILE_MACHINE_SH4: u16 = 422u16;
```

SH4 little-endian

### `IMAGE_FILE_MACHINE_SH5`

```rust
const IMAGE_FILE_MACHINE_SH5: u16 = 424u16;
```

SH5

### `IMAGE_FILE_MACHINE_ARM`

```rust
const IMAGE_FILE_MACHINE_ARM: u16 = 448u16;
```

ARM Little-Endian

### `IMAGE_FILE_MACHINE_THUMB`

```rust
const IMAGE_FILE_MACHINE_THUMB: u16 = 450u16;
```

ARM Thumb/Thumb-2 Little-Endian

### `IMAGE_FILE_MACHINE_ARMNT`

```rust
const IMAGE_FILE_MACHINE_ARMNT: u16 = 452u16;
```

ARM Thumb-2 Little-Endian

### `IMAGE_FILE_MACHINE_AM33`

```rust
const IMAGE_FILE_MACHINE_AM33: u16 = 467u16;
```

### `IMAGE_FILE_MACHINE_POWERPC`

```rust
const IMAGE_FILE_MACHINE_POWERPC: u16 = 496u16;
```

IBM PowerPC Little-Endian

### `IMAGE_FILE_MACHINE_POWERPCFP`

```rust
const IMAGE_FILE_MACHINE_POWERPCFP: u16 = 497u16;
```

### `IMAGE_FILE_MACHINE_POWERPCBE`

```rust
const IMAGE_FILE_MACHINE_POWERPCBE: u16 = 498u16;
```

IBM PowerPC Big-Endian

### `IMAGE_FILE_MACHINE_IA64`

```rust
const IMAGE_FILE_MACHINE_IA64: u16 = 512u16;
```

Intel 64

### `IMAGE_FILE_MACHINE_MIPS16`

```rust
const IMAGE_FILE_MACHINE_MIPS16: u16 = 614u16;
```

MIPS

### `IMAGE_FILE_MACHINE_ALPHA64`

```rust
const IMAGE_FILE_MACHINE_ALPHA64: u16 = 644u16;
```

ALPHA64

### `IMAGE_FILE_MACHINE_MIPSFPU`

```rust
const IMAGE_FILE_MACHINE_MIPSFPU: u16 = 870u16;
```

MIPS

### `IMAGE_FILE_MACHINE_MIPSFPU16`

```rust
const IMAGE_FILE_MACHINE_MIPSFPU16: u16 = 1_126u16;
```

MIPS

### `IMAGE_FILE_MACHINE_AXP64`

```rust
const IMAGE_FILE_MACHINE_AXP64: u16 = 644u16;
```

### `IMAGE_FILE_MACHINE_TRICORE`

```rust
const IMAGE_FILE_MACHINE_TRICORE: u16 = 1_312u16;
```

Infineon

### `IMAGE_FILE_MACHINE_CEF`

```rust
const IMAGE_FILE_MACHINE_CEF: u16 = 3_311u16;
```

### `IMAGE_FILE_MACHINE_EBC`

```rust
const IMAGE_FILE_MACHINE_EBC: u16 = 3_772u16;
```

EFI Byte Code

### `IMAGE_FILE_MACHINE_AMD64`

```rust
const IMAGE_FILE_MACHINE_AMD64: u16 = 34_404u16;
```

AMD64 (K8)

### `IMAGE_FILE_MACHINE_M32R`

```rust
const IMAGE_FILE_MACHINE_M32R: u16 = 36_929u16;
```

M32R little-endian

### `IMAGE_FILE_MACHINE_ARM64`

```rust
const IMAGE_FILE_MACHINE_ARM64: u16 = 43_620u16;
```

ARM64 Little-Endian

### `IMAGE_FILE_MACHINE_ARM64EC`

```rust
const IMAGE_FILE_MACHINE_ARM64EC: u16 = 42_561u16;
```

ARM64EC ("Emulation Compatible")

### `IMAGE_FILE_MACHINE_CEE`

```rust
const IMAGE_FILE_MACHINE_CEE: u16 = 49_390u16;
```

### `IMAGE_FILE_MACHINE_RISCV32`

```rust
const IMAGE_FILE_MACHINE_RISCV32: u16 = 20_530u16;
```

RISCV32

### `IMAGE_FILE_MACHINE_RISCV64`

```rust
const IMAGE_FILE_MACHINE_RISCV64: u16 = 20_580u16;
```

RISCV64

### `IMAGE_FILE_MACHINE_RISCV128`

```rust
const IMAGE_FILE_MACHINE_RISCV128: u16 = 20_776u16;
```

RISCV128

### `IMAGE_FILE_MACHINE_ARM64X`

```rust
const IMAGE_FILE_MACHINE_ARM64X: u16 = 42_574u16;
```

ARM64X (Mixed ARM64 and ARM64EC)

### `IMAGE_FILE_MACHINE_CHPE_X86`

```rust
const IMAGE_FILE_MACHINE_CHPE_X86: u16 = 14_948u16;
```

CHPE x86 ("Compiled Hybrid Portable Executable")

### `IMAGE_NUMBEROF_DIRECTORY_ENTRIES`

```rust
const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16usize;
```

### `IMAGE_NT_OPTIONAL_HDR32_MAGIC`

```rust
const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 267u16;
```

### `IMAGE_NT_OPTIONAL_HDR64_MAGIC`

```rust
const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 523u16;
```

### `IMAGE_ROM_OPTIONAL_HDR_MAGIC`

```rust
const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16 = 263u16;
```

### `IMAGE_SUBSYSTEM_UNKNOWN`

```rust
const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0u16;
```

Unknown subsystem.

### `IMAGE_SUBSYSTEM_NATIVE`

```rust
const IMAGE_SUBSYSTEM_NATIVE: u16 = 1u16;
```

Image doesn't require a subsystem.

### `IMAGE_SUBSYSTEM_WINDOWS_GUI`

```rust
const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2u16;
```

Image runs in the Windows GUI subsystem.

### `IMAGE_SUBSYSTEM_WINDOWS_CUI`

```rust
const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3u16;
```

Image runs in the Windows character subsystem.

### `IMAGE_SUBSYSTEM_OS2_CUI`

```rust
const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 5u16;
```

image runs in the OS/2 character subsystem.

### `IMAGE_SUBSYSTEM_POSIX_CUI`

```rust
const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 7u16;
```

image runs in the Posix character subsystem.

### `IMAGE_SUBSYSTEM_NATIVE_WINDOWS`

```rust
const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16 = 8u16;
```

image is a native Win9x driver.

### `IMAGE_SUBSYSTEM_WINDOWS_CE_GUI`

```rust
const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16 = 9u16;
```

Image runs in the Windows CE subsystem.

### `IMAGE_SUBSYSTEM_EFI_APPLICATION`

```rust
const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16 = 10u16;
```

### `IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER`

```rust
const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16 = 11u16;
```

### `IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER`

```rust
const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16 = 12u16;
```

### `IMAGE_SUBSYSTEM_EFI_ROM`

```rust
const IMAGE_SUBSYSTEM_EFI_ROM: u16 = 13u16;
```

### `IMAGE_SUBSYSTEM_XBOX`

```rust
const IMAGE_SUBSYSTEM_XBOX: u16 = 14u16;
```

### `IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION`

```rust
const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16u16;
```

### `IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG`

```rust
const IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG: u16 = 17u16;
```

### `IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA`

```rust
const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16 = 32u16;
```

Image can handle a high entropy 64-bit virtual address space.

### `IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE`

```rust
const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 64u16;
```

DLL can move.

### `IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY`

```rust
const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16 = 128u16;
```

Code Integrity Image

### `IMAGE_DLLCHARACTERISTICS_NX_COMPAT`

```rust
const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 256u16;
```

Image is NX compatible

### `IMAGE_DLLCHARACTERISTICS_NO_ISOLATION`

```rust
const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16 = 512u16;
```

Image understands isolation and doesn't want it

### `IMAGE_DLLCHARACTERISTICS_NO_SEH`

```rust
const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16 = 1_024u16;
```

Image does not use SEH.  No SE handler may reside in this image

### `IMAGE_DLLCHARACTERISTICS_NO_BIND`

```rust
const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16 = 2_048u16;
```

Do not bind this image.

### `IMAGE_DLLCHARACTERISTICS_APPCONTAINER`

```rust
const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u16 = 4_096u16;
```

Image should execute in an AppContainer

### `IMAGE_DLLCHARACTERISTICS_WDM_DRIVER`

```rust
const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16 = 8_192u16;
```

Driver uses WDM model

### `IMAGE_DLLCHARACTERISTICS_GUARD_CF`

```rust
const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u16 = 16_384u16;
```

Image supports Control Flow Guard.

### `IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE`

```rust
const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 32_768u16;
```

### `IMAGE_DIRECTORY_ENTRY_EXPORT`

```rust
const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0usize;
```

Export Directory

### `IMAGE_DIRECTORY_ENTRY_IMPORT`

```rust
const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1usize;
```

Import Directory

### `IMAGE_DIRECTORY_ENTRY_RESOURCE`

```rust
const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2usize;
```

Resource Directory

### `IMAGE_DIRECTORY_ENTRY_EXCEPTION`

```rust
const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize = 3usize;
```

Exception Directory

### `IMAGE_DIRECTORY_ENTRY_SECURITY`

```rust
const IMAGE_DIRECTORY_ENTRY_SECURITY: usize = 4usize;
```

Security Directory

### `IMAGE_DIRECTORY_ENTRY_BASERELOC`

```rust
const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5usize;
```

Base Relocation Table

### `IMAGE_DIRECTORY_ENTRY_DEBUG`

```rust
const IMAGE_DIRECTORY_ENTRY_DEBUG: usize = 6usize;
```

Debug Directory

### `IMAGE_DIRECTORY_ENTRY_ARCHITECTURE`

```rust
const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: usize = 7usize;
```

Architecture Specific Data

### `IMAGE_DIRECTORY_ENTRY_GLOBALPTR`

```rust
const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: usize = 8usize;
```

RVA of GP

### `IMAGE_DIRECTORY_ENTRY_TLS`

```rust
const IMAGE_DIRECTORY_ENTRY_TLS: usize = 9usize;
```

TLS Directory

### `IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG`

```rust
const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize = 10usize;
```

Load Configuration Directory

### `IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT`

```rust
const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: usize = 11usize;
```

Bound Import Directory in headers

### `IMAGE_DIRECTORY_ENTRY_IAT`

```rust
const IMAGE_DIRECTORY_ENTRY_IAT: usize = 12usize;
```

Import Address Table

### `IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT`

```rust
const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: usize = 13usize;
```

Delay Load Import Descriptors

### `IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR`

```rust
const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14usize;
```

COM Runtime descriptor

### `ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`

```rust
const ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID: ClsId;
```

The required value of `AnonObjectHeaderBigobj::class_id`.

### `IMAGE_SIZEOF_SHORT_NAME`

```rust
const IMAGE_SIZEOF_SHORT_NAME: usize = 8usize;
```

### `IMAGE_SIZEOF_SECTION_HEADER`

```rust
const IMAGE_SIZEOF_SECTION_HEADER: usize = 40usize;
```

### `IMAGE_SCN_TYPE_NO_PAD`

```rust
const IMAGE_SCN_TYPE_NO_PAD: u32 = 8u32;
```

Reserved.

### `IMAGE_SCN_CNT_CODE`

```rust
const IMAGE_SCN_CNT_CODE: u32 = 32u32;
```

Section contains code.

### `IMAGE_SCN_CNT_INITIALIZED_DATA`

```rust
const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 64u32;
```

Section contains initialized data.

### `IMAGE_SCN_CNT_UNINITIALIZED_DATA`

```rust
const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 128u32;
```

Section contains uninitialized data.

### `IMAGE_SCN_LNK_OTHER`

```rust
const IMAGE_SCN_LNK_OTHER: u32 = 256u32;
```

Reserved.

### `IMAGE_SCN_LNK_INFO`

```rust
const IMAGE_SCN_LNK_INFO: u32 = 512u32;
```

Section contains comments or some other type of information.

### `IMAGE_SCN_LNK_REMOVE`

```rust
const IMAGE_SCN_LNK_REMOVE: u32 = 2_048u32;
```

Section contents will not become part of image.

### `IMAGE_SCN_LNK_COMDAT`

```rust
const IMAGE_SCN_LNK_COMDAT: u32 = 4_096u32;
```

Section contents comdat.

### `IMAGE_SCN_NO_DEFER_SPEC_EXC`

```rust
const IMAGE_SCN_NO_DEFER_SPEC_EXC: u32 = 16_384u32;
```

Reset speculative exceptions handling bits in the TLB entries for this section.

### `IMAGE_SCN_GPREL`

```rust
const IMAGE_SCN_GPREL: u32 = 32_768u32;
```

Section content can be accessed relative to GP

### `IMAGE_SCN_MEM_FARDATA`

```rust
const IMAGE_SCN_MEM_FARDATA: u32 = 32_768u32;
```

### `IMAGE_SCN_MEM_PURGEABLE`

```rust
const IMAGE_SCN_MEM_PURGEABLE: u32 = 131_072u32;
```

### `IMAGE_SCN_MEM_16BIT`

```rust
const IMAGE_SCN_MEM_16BIT: u32 = 131_072u32;
```

### `IMAGE_SCN_MEM_LOCKED`

```rust
const IMAGE_SCN_MEM_LOCKED: u32 = 262_144u32;
```

### `IMAGE_SCN_MEM_PRELOAD`

```rust
const IMAGE_SCN_MEM_PRELOAD: u32 = 524_288u32;
```

### `IMAGE_SCN_ALIGN_1BYTES`

```rust
const IMAGE_SCN_ALIGN_1BYTES: u32 = 1_048_576u32;
```

### `IMAGE_SCN_ALIGN_2BYTES`

```rust
const IMAGE_SCN_ALIGN_2BYTES: u32 = 2_097_152u32;
```

### `IMAGE_SCN_ALIGN_4BYTES`

```rust
const IMAGE_SCN_ALIGN_4BYTES: u32 = 3_145_728u32;
```

### `IMAGE_SCN_ALIGN_8BYTES`

```rust
const IMAGE_SCN_ALIGN_8BYTES: u32 = 4_194_304u32;
```

### `IMAGE_SCN_ALIGN_16BYTES`

```rust
const IMAGE_SCN_ALIGN_16BYTES: u32 = 5_242_880u32;
```

Default alignment if no others are specified.

### `IMAGE_SCN_ALIGN_32BYTES`

```rust
const IMAGE_SCN_ALIGN_32BYTES: u32 = 6_291_456u32;
```

### `IMAGE_SCN_ALIGN_64BYTES`

```rust
const IMAGE_SCN_ALIGN_64BYTES: u32 = 7_340_032u32;
```

### `IMAGE_SCN_ALIGN_128BYTES`

```rust
const IMAGE_SCN_ALIGN_128BYTES: u32 = 8_388_608u32;
```

### `IMAGE_SCN_ALIGN_256BYTES`

```rust
const IMAGE_SCN_ALIGN_256BYTES: u32 = 9_437_184u32;
```

### `IMAGE_SCN_ALIGN_512BYTES`

```rust
const IMAGE_SCN_ALIGN_512BYTES: u32 = 10_485_760u32;
```

### `IMAGE_SCN_ALIGN_1024BYTES`

```rust
const IMAGE_SCN_ALIGN_1024BYTES: u32 = 11_534_336u32;
```

### `IMAGE_SCN_ALIGN_2048BYTES`

```rust
const IMAGE_SCN_ALIGN_2048BYTES: u32 = 12_582_912u32;
```

### `IMAGE_SCN_ALIGN_4096BYTES`

```rust
const IMAGE_SCN_ALIGN_4096BYTES: u32 = 13_631_488u32;
```

### `IMAGE_SCN_ALIGN_8192BYTES`

```rust
const IMAGE_SCN_ALIGN_8192BYTES: u32 = 14_680_064u32;
```

### `IMAGE_SCN_ALIGN_MASK`

```rust
const IMAGE_SCN_ALIGN_MASK: u32 = 15_728_640u32;
```

### `IMAGE_SCN_LNK_NRELOC_OVFL`

```rust
const IMAGE_SCN_LNK_NRELOC_OVFL: u32 = 16_777_216u32;
```

Section contains extended relocations.

### `IMAGE_SCN_MEM_DISCARDABLE`

```rust
const IMAGE_SCN_MEM_DISCARDABLE: u32 = 33_554_432u32;
```

Section can be discarded.

### `IMAGE_SCN_MEM_NOT_CACHED`

```rust
const IMAGE_SCN_MEM_NOT_CACHED: u32 = 67_108_864u32;
```

Section is not cacheable.

### `IMAGE_SCN_MEM_NOT_PAGED`

```rust
const IMAGE_SCN_MEM_NOT_PAGED: u32 = 134_217_728u32;
```

Section is not pageable.

### `IMAGE_SCN_MEM_SHARED`

```rust
const IMAGE_SCN_MEM_SHARED: u32 = 268_435_456u32;
```

Section is shareable.

### `IMAGE_SCN_MEM_EXECUTE`

```rust
const IMAGE_SCN_MEM_EXECUTE: u32 = 536_870_912u32;
```

Section is executable.

### `IMAGE_SCN_MEM_READ`

```rust
const IMAGE_SCN_MEM_READ: u32 = 1_073_741_824u32;
```

Section is readable.

### `IMAGE_SCN_MEM_WRITE`

```rust
const IMAGE_SCN_MEM_WRITE: u32 = 2_147_483_648u32;
```

Section is writeable.

### `IMAGE_SCN_SCALE_INDEX`

```rust
const IMAGE_SCN_SCALE_INDEX: u32 = 1u32;
```

Tls index is scaled

### `IMAGE_SIZEOF_SYMBOL`

```rust
const IMAGE_SIZEOF_SYMBOL: usize = 18usize;
```

### `IMAGE_SIZEOF_SYMBOL_EX`

```rust
const IMAGE_SIZEOF_SYMBOL_EX: usize = 20usize;
```

### `IMAGE_SYM_UNDEFINED`

```rust
const IMAGE_SYM_UNDEFINED: i32 = 0i32;
```

Symbol is undefined or is common.

### `IMAGE_SYM_ABSOLUTE`

```rust
const IMAGE_SYM_ABSOLUTE: i32 = -1i32;
```

Symbol is an absolute value.

### `IMAGE_SYM_DEBUG`

```rust
const IMAGE_SYM_DEBUG: i32 = -2i32;
```

Symbol is a special debug item.

### `IMAGE_SYM_SECTION_MAX`

```rust
const IMAGE_SYM_SECTION_MAX: u16 = 65_279u16;
```

Values 0xFF00-0xFFFF are special

### `IMAGE_SYM_SECTION_MAX_EX`

```rust
const IMAGE_SYM_SECTION_MAX_EX: u32 = 2_147_483_647u32;
```

### `IMAGE_SYM_TYPE_NULL`

```rust
const IMAGE_SYM_TYPE_NULL: u16 = 0u16;
```

no type.

### `IMAGE_SYM_TYPE_VOID`

```rust
const IMAGE_SYM_TYPE_VOID: u16 = 1u16;
```

### `IMAGE_SYM_TYPE_CHAR`

```rust
const IMAGE_SYM_TYPE_CHAR: u16 = 2u16;
```

type character.

### `IMAGE_SYM_TYPE_SHORT`

```rust
const IMAGE_SYM_TYPE_SHORT: u16 = 3u16;
```

type short integer.

### `IMAGE_SYM_TYPE_INT`

```rust
const IMAGE_SYM_TYPE_INT: u16 = 4u16;
```

### `IMAGE_SYM_TYPE_LONG`

```rust
const IMAGE_SYM_TYPE_LONG: u16 = 5u16;
```

### `IMAGE_SYM_TYPE_FLOAT`

```rust
const IMAGE_SYM_TYPE_FLOAT: u16 = 6u16;
```

### `IMAGE_SYM_TYPE_DOUBLE`

```rust
const IMAGE_SYM_TYPE_DOUBLE: u16 = 7u16;
```

### `IMAGE_SYM_TYPE_STRUCT`

```rust
const IMAGE_SYM_TYPE_STRUCT: u16 = 8u16;
```

### `IMAGE_SYM_TYPE_UNION`

```rust
const IMAGE_SYM_TYPE_UNION: u16 = 9u16;
```

### `IMAGE_SYM_TYPE_ENUM`

```rust
const IMAGE_SYM_TYPE_ENUM: u16 = 10u16;
```

enumeration.

### `IMAGE_SYM_TYPE_MOE`

```rust
const IMAGE_SYM_TYPE_MOE: u16 = 11u16;
```

member of enumeration.

### `IMAGE_SYM_TYPE_BYTE`

```rust
const IMAGE_SYM_TYPE_BYTE: u16 = 12u16;
```

### `IMAGE_SYM_TYPE_WORD`

```rust
const IMAGE_SYM_TYPE_WORD: u16 = 13u16;
```

### `IMAGE_SYM_TYPE_UINT`

```rust
const IMAGE_SYM_TYPE_UINT: u16 = 14u16;
```

### `IMAGE_SYM_TYPE_DWORD`

```rust
const IMAGE_SYM_TYPE_DWORD: u16 = 15u16;
```

### `IMAGE_SYM_TYPE_PCODE`

```rust
const IMAGE_SYM_TYPE_PCODE: u16 = 32_768u16;
```

### `IMAGE_SYM_DTYPE_NULL`

```rust
const IMAGE_SYM_DTYPE_NULL: u16 = 0u16;
```

no derived type.

### `IMAGE_SYM_DTYPE_POINTER`

```rust
const IMAGE_SYM_DTYPE_POINTER: u16 = 1u16;
```

pointer.

### `IMAGE_SYM_DTYPE_FUNCTION`

```rust
const IMAGE_SYM_DTYPE_FUNCTION: u16 = 2u16;
```

function.

### `IMAGE_SYM_DTYPE_ARRAY`

```rust
const IMAGE_SYM_DTYPE_ARRAY: u16 = 3u16;
```

array.

### `IMAGE_SYM_CLASS_END_OF_FUNCTION`

```rust
const IMAGE_SYM_CLASS_END_OF_FUNCTION: u8 = 255u8;
```

### `IMAGE_SYM_CLASS_NULL`

```rust
const IMAGE_SYM_CLASS_NULL: u8 = 0u8;
```

### `IMAGE_SYM_CLASS_AUTOMATIC`

```rust
const IMAGE_SYM_CLASS_AUTOMATIC: u8 = 1u8;
```

### `IMAGE_SYM_CLASS_EXTERNAL`

```rust
const IMAGE_SYM_CLASS_EXTERNAL: u8 = 2u8;
```

### `IMAGE_SYM_CLASS_STATIC`

```rust
const IMAGE_SYM_CLASS_STATIC: u8 = 3u8;
```

### `IMAGE_SYM_CLASS_REGISTER`

```rust
const IMAGE_SYM_CLASS_REGISTER: u8 = 4u8;
```

### `IMAGE_SYM_CLASS_EXTERNAL_DEF`

```rust
const IMAGE_SYM_CLASS_EXTERNAL_DEF: u8 = 5u8;
```

### `IMAGE_SYM_CLASS_LABEL`

```rust
const IMAGE_SYM_CLASS_LABEL: u8 = 6u8;
```

### `IMAGE_SYM_CLASS_UNDEFINED_LABEL`

```rust
const IMAGE_SYM_CLASS_UNDEFINED_LABEL: u8 = 7u8;
```

### `IMAGE_SYM_CLASS_MEMBER_OF_STRUCT`

```rust
const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: u8 = 8u8;
```

### `IMAGE_SYM_CLASS_ARGUMENT`

```rust
const IMAGE_SYM_CLASS_ARGUMENT: u8 = 9u8;
```

### `IMAGE_SYM_CLASS_STRUCT_TAG`

```rust
const IMAGE_SYM_CLASS_STRUCT_TAG: u8 = 10u8;
```

### `IMAGE_SYM_CLASS_MEMBER_OF_UNION`

```rust
const IMAGE_SYM_CLASS_MEMBER_OF_UNION: u8 = 11u8;
```

### `IMAGE_SYM_CLASS_UNION_TAG`

```rust
const IMAGE_SYM_CLASS_UNION_TAG: u8 = 12u8;
```

### `IMAGE_SYM_CLASS_TYPE_DEFINITION`

```rust
const IMAGE_SYM_CLASS_TYPE_DEFINITION: u8 = 13u8;
```

### `IMAGE_SYM_CLASS_UNDEFINED_STATIC`

```rust
const IMAGE_SYM_CLASS_UNDEFINED_STATIC: u8 = 14u8;
```

### `IMAGE_SYM_CLASS_ENUM_TAG`

```rust
const IMAGE_SYM_CLASS_ENUM_TAG: u8 = 15u8;
```

### `IMAGE_SYM_CLASS_MEMBER_OF_ENUM`

```rust
const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: u8 = 16u8;
```

### `IMAGE_SYM_CLASS_REGISTER_PARAM`

```rust
const IMAGE_SYM_CLASS_REGISTER_PARAM: u8 = 17u8;
```

### `IMAGE_SYM_CLASS_BIT_FIELD`

```rust
const IMAGE_SYM_CLASS_BIT_FIELD: u8 = 18u8;
```

### `IMAGE_SYM_CLASS_FAR_EXTERNAL`

```rust
const IMAGE_SYM_CLASS_FAR_EXTERNAL: u8 = 68u8;
```

### `IMAGE_SYM_CLASS_BLOCK`

```rust
const IMAGE_SYM_CLASS_BLOCK: u8 = 100u8;
```

### `IMAGE_SYM_CLASS_FUNCTION`

```rust
const IMAGE_SYM_CLASS_FUNCTION: u8 = 101u8;
```

### `IMAGE_SYM_CLASS_END_OF_STRUCT`

```rust
const IMAGE_SYM_CLASS_END_OF_STRUCT: u8 = 102u8;
```

### `IMAGE_SYM_CLASS_FILE`

```rust
const IMAGE_SYM_CLASS_FILE: u8 = 103u8;
```

### `IMAGE_SYM_CLASS_SECTION`

```rust
const IMAGE_SYM_CLASS_SECTION: u8 = 104u8;
```

### `IMAGE_SYM_CLASS_WEAK_EXTERNAL`

```rust
const IMAGE_SYM_CLASS_WEAK_EXTERNAL: u8 = 105u8;
```

### `IMAGE_SYM_CLASS_CLR_TOKEN`

```rust
const IMAGE_SYM_CLASS_CLR_TOKEN: u8 = 107u8;
```

### `N_BTMASK`

```rust
const N_BTMASK: u16 = 15u16;
```

### `N_TMASK`

```rust
const N_TMASK: u16 = 48u16;
```

### `N_TMASK1`

```rust
const N_TMASK1: u16 = 192u16;
```

### `N_TMASK2`

```rust
const N_TMASK2: u16 = 240u16;
```

### `N_BTSHFT`

```rust
const N_BTSHFT: usize = 4usize;
```

### `N_TSHIFT`

```rust
const N_TSHIFT: usize = 2usize;
```

### `IMAGE_SYM_DTYPE_SHIFT`

```rust
const IMAGE_SYM_DTYPE_SHIFT: usize = 4usize;
```

### `IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF`

```rust
const IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF: u16 = 1u16;
```

### `IMAGE_COMDAT_SELECT_NODUPLICATES`

```rust
const IMAGE_COMDAT_SELECT_NODUPLICATES: u8 = 1u8;
```

### `IMAGE_COMDAT_SELECT_ANY`

```rust
const IMAGE_COMDAT_SELECT_ANY: u8 = 2u8;
```

### `IMAGE_COMDAT_SELECT_SAME_SIZE`

```rust
const IMAGE_COMDAT_SELECT_SAME_SIZE: u8 = 3u8;
```

### `IMAGE_COMDAT_SELECT_EXACT_MATCH`

```rust
const IMAGE_COMDAT_SELECT_EXACT_MATCH: u8 = 4u8;
```

### `IMAGE_COMDAT_SELECT_ASSOCIATIVE`

```rust
const IMAGE_COMDAT_SELECT_ASSOCIATIVE: u8 = 5u8;
```

### `IMAGE_COMDAT_SELECT_LARGEST`

```rust
const IMAGE_COMDAT_SELECT_LARGEST: u8 = 6u8;
```

### `IMAGE_COMDAT_SELECT_NEWEST`

```rust
const IMAGE_COMDAT_SELECT_NEWEST: u8 = 7u8;
```

### `IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY`

```rust
const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: u32 = 1u32;
```

### `IMAGE_WEAK_EXTERN_SEARCH_LIBRARY`

```rust
const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: u32 = 2u32;
```

### `IMAGE_WEAK_EXTERN_SEARCH_ALIAS`

```rust
const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: u32 = 3u32;
```

### `IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY`

```rust
const IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY: u32 = 4u32;
```

### `IMAGE_REL_I386_ABSOLUTE`

```rust
const IMAGE_REL_I386_ABSOLUTE: u16 = 0u16;
```

Reference is absolute, no relocation is necessary

### `IMAGE_REL_I386_DIR16`

```rust
const IMAGE_REL_I386_DIR16: u16 = 1u16;
```

Direct 16-bit reference to the symbols virtual address

### `IMAGE_REL_I386_REL16`

```rust
const IMAGE_REL_I386_REL16: u16 = 2u16;
```

PC-relative 16-bit reference to the symbols virtual address

### `IMAGE_REL_I386_DIR32`

```rust
const IMAGE_REL_I386_DIR32: u16 = 6u16;
```

Direct 32-bit reference to the symbols virtual address

### `IMAGE_REL_I386_DIR32NB`

```rust
const IMAGE_REL_I386_DIR32NB: u16 = 7u16;
```

Direct 32-bit reference to the symbols virtual address, base not included

### `IMAGE_REL_I386_SEG12`

```rust
const IMAGE_REL_I386_SEG12: u16 = 9u16;
```

Direct 16-bit reference to the segment-selector bits of a 32-bit virtual address

### `IMAGE_REL_I386_SECTION`

```rust
const IMAGE_REL_I386_SECTION: u16 = 10u16;
```

### `IMAGE_REL_I386_SECREL`

```rust
const IMAGE_REL_I386_SECREL: u16 = 11u16;
```

### `IMAGE_REL_I386_TOKEN`

```rust
const IMAGE_REL_I386_TOKEN: u16 = 12u16;
```

clr token

### `IMAGE_REL_I386_SECREL7`

```rust
const IMAGE_REL_I386_SECREL7: u16 = 13u16;
```

7 bit offset from base of section containing target

### `IMAGE_REL_I386_REL32`

```rust
const IMAGE_REL_I386_REL32: u16 = 20u16;
```

PC-relative 32-bit reference to the symbols virtual address

### `IMAGE_REL_MIPS_ABSOLUTE`

```rust
const IMAGE_REL_MIPS_ABSOLUTE: u16 = 0u16;
```

Reference is absolute, no relocation is necessary

### `IMAGE_REL_MIPS_REFHALF`

```rust
const IMAGE_REL_MIPS_REFHALF: u16 = 1u16;
```

### `IMAGE_REL_MIPS_REFWORD`

```rust
const IMAGE_REL_MIPS_REFWORD: u16 = 2u16;
```

### `IMAGE_REL_MIPS_JMPADDR`

```rust
const IMAGE_REL_MIPS_JMPADDR: u16 = 3u16;
```

### `IMAGE_REL_MIPS_REFHI`

```rust
const IMAGE_REL_MIPS_REFHI: u16 = 4u16;
```

### `IMAGE_REL_MIPS_REFLO`

```rust
const IMAGE_REL_MIPS_REFLO: u16 = 5u16;
```

### `IMAGE_REL_MIPS_GPREL`

```rust
const IMAGE_REL_MIPS_GPREL: u16 = 6u16;
```

### `IMAGE_REL_MIPS_LITERAL`

```rust
const IMAGE_REL_MIPS_LITERAL: u16 = 7u16;
```

### `IMAGE_REL_MIPS_SECTION`

```rust
const IMAGE_REL_MIPS_SECTION: u16 = 10u16;
```

### `IMAGE_REL_MIPS_SECREL`

```rust
const IMAGE_REL_MIPS_SECREL: u16 = 11u16;
```

### `IMAGE_REL_MIPS_SECRELLO`

```rust
const IMAGE_REL_MIPS_SECRELLO: u16 = 12u16;
```

Low 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_MIPS_SECRELHI`

```rust
const IMAGE_REL_MIPS_SECRELHI: u16 = 13u16;
```

High 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_MIPS_TOKEN`

```rust
const IMAGE_REL_MIPS_TOKEN: u16 = 14u16;
```

clr token

### `IMAGE_REL_MIPS_JMPADDR16`

```rust
const IMAGE_REL_MIPS_JMPADDR16: u16 = 16u16;
```

### `IMAGE_REL_MIPS_REFWORDNB`

```rust
const IMAGE_REL_MIPS_REFWORDNB: u16 = 34u16;
```

### `IMAGE_REL_MIPS_PAIR`

```rust
const IMAGE_REL_MIPS_PAIR: u16 = 37u16;
```

### `IMAGE_REL_ALPHA_ABSOLUTE`

```rust
const IMAGE_REL_ALPHA_ABSOLUTE: u16 = 0u16;
```

### `IMAGE_REL_ALPHA_REFLONG`

```rust
const IMAGE_REL_ALPHA_REFLONG: u16 = 1u16;
```

### `IMAGE_REL_ALPHA_REFQUAD`

```rust
const IMAGE_REL_ALPHA_REFQUAD: u16 = 2u16;
```

### `IMAGE_REL_ALPHA_GPREL32`

```rust
const IMAGE_REL_ALPHA_GPREL32: u16 = 3u16;
```

### `IMAGE_REL_ALPHA_LITERAL`

```rust
const IMAGE_REL_ALPHA_LITERAL: u16 = 4u16;
```

### `IMAGE_REL_ALPHA_LITUSE`

```rust
const IMAGE_REL_ALPHA_LITUSE: u16 = 5u16;
```

### `IMAGE_REL_ALPHA_GPDISP`

```rust
const IMAGE_REL_ALPHA_GPDISP: u16 = 6u16;
```

### `IMAGE_REL_ALPHA_BRADDR`

```rust
const IMAGE_REL_ALPHA_BRADDR: u16 = 7u16;
```

### `IMAGE_REL_ALPHA_HINT`

```rust
const IMAGE_REL_ALPHA_HINT: u16 = 8u16;
```

### `IMAGE_REL_ALPHA_INLINE_REFLONG`

```rust
const IMAGE_REL_ALPHA_INLINE_REFLONG: u16 = 9u16;
```

### `IMAGE_REL_ALPHA_REFHI`

```rust
const IMAGE_REL_ALPHA_REFHI: u16 = 10u16;
```

### `IMAGE_REL_ALPHA_REFLO`

```rust
const IMAGE_REL_ALPHA_REFLO: u16 = 11u16;
```

### `IMAGE_REL_ALPHA_PAIR`

```rust
const IMAGE_REL_ALPHA_PAIR: u16 = 12u16;
```

### `IMAGE_REL_ALPHA_MATCH`

```rust
const IMAGE_REL_ALPHA_MATCH: u16 = 13u16;
```

### `IMAGE_REL_ALPHA_SECTION`

```rust
const IMAGE_REL_ALPHA_SECTION: u16 = 14u16;
```

### `IMAGE_REL_ALPHA_SECREL`

```rust
const IMAGE_REL_ALPHA_SECREL: u16 = 15u16;
```

### `IMAGE_REL_ALPHA_REFLONGNB`

```rust
const IMAGE_REL_ALPHA_REFLONGNB: u16 = 16u16;
```

### `IMAGE_REL_ALPHA_SECRELLO`

```rust
const IMAGE_REL_ALPHA_SECRELLO: u16 = 17u16;
```

Low 16-bit section relative reference

### `IMAGE_REL_ALPHA_SECRELHI`

```rust
const IMAGE_REL_ALPHA_SECRELHI: u16 = 18u16;
```

High 16-bit section relative reference

### `IMAGE_REL_ALPHA_REFQ3`

```rust
const IMAGE_REL_ALPHA_REFQ3: u16 = 19u16;
```

High 16 bits of 48 bit reference

### `IMAGE_REL_ALPHA_REFQ2`

```rust
const IMAGE_REL_ALPHA_REFQ2: u16 = 20u16;
```

Middle 16 bits of 48 bit reference

### `IMAGE_REL_ALPHA_REFQ1`

```rust
const IMAGE_REL_ALPHA_REFQ1: u16 = 21u16;
```

Low 16 bits of 48 bit reference

### `IMAGE_REL_ALPHA_GPRELLO`

```rust
const IMAGE_REL_ALPHA_GPRELLO: u16 = 22u16;
```

Low 16-bit GP relative reference

### `IMAGE_REL_ALPHA_GPRELHI`

```rust
const IMAGE_REL_ALPHA_GPRELHI: u16 = 23u16;
```

High 16-bit GP relative reference

### `IMAGE_REL_PPC_ABSOLUTE`

```rust
const IMAGE_REL_PPC_ABSOLUTE: u16 = 0u16;
```

NOP

### `IMAGE_REL_PPC_ADDR64`

```rust
const IMAGE_REL_PPC_ADDR64: u16 = 1u16;
```

64-bit address

### `IMAGE_REL_PPC_ADDR32`

```rust
const IMAGE_REL_PPC_ADDR32: u16 = 2u16;
```

32-bit address

### `IMAGE_REL_PPC_ADDR24`

```rust
const IMAGE_REL_PPC_ADDR24: u16 = 3u16;
```

26-bit address, shifted left 2 (branch absolute)

### `IMAGE_REL_PPC_ADDR16`

```rust
const IMAGE_REL_PPC_ADDR16: u16 = 4u16;
```

16-bit address

### `IMAGE_REL_PPC_ADDR14`

```rust
const IMAGE_REL_PPC_ADDR14: u16 = 5u16;
```

16-bit address, shifted left 2 (load doubleword)

### `IMAGE_REL_PPC_REL24`

```rust
const IMAGE_REL_PPC_REL24: u16 = 6u16;
```

26-bit PC-relative offset, shifted left 2 (branch relative)

### `IMAGE_REL_PPC_REL14`

```rust
const IMAGE_REL_PPC_REL14: u16 = 7u16;
```

16-bit PC-relative offset, shifted left 2 (br cond relative)

### `IMAGE_REL_PPC_TOCREL16`

```rust
const IMAGE_REL_PPC_TOCREL16: u16 = 8u16;
```

16-bit offset from TOC base

### `IMAGE_REL_PPC_TOCREL14`

```rust
const IMAGE_REL_PPC_TOCREL14: u16 = 9u16;
```

16-bit offset from TOC base, shifted left 2 (load doubleword)

### `IMAGE_REL_PPC_ADDR32NB`

```rust
const IMAGE_REL_PPC_ADDR32NB: u16 = 10u16;
```

32-bit addr w/o image base

### `IMAGE_REL_PPC_SECREL`

```rust
const IMAGE_REL_PPC_SECREL: u16 = 11u16;
```

va of containing section (as in an image sectionhdr)

### `IMAGE_REL_PPC_SECTION`

```rust
const IMAGE_REL_PPC_SECTION: u16 = 12u16;
```

sectionheader number

### `IMAGE_REL_PPC_IFGLUE`

```rust
const IMAGE_REL_PPC_IFGLUE: u16 = 13u16;
```

substitute TOC restore instruction iff symbol is glue code

### `IMAGE_REL_PPC_IMGLUE`

```rust
const IMAGE_REL_PPC_IMGLUE: u16 = 14u16;
```

symbol is glue code; virtual address is TOC restore instruction

### `IMAGE_REL_PPC_SECREL16`

```rust
const IMAGE_REL_PPC_SECREL16: u16 = 15u16;
```

va of containing section (limited to 16 bits)

### `IMAGE_REL_PPC_REFHI`

```rust
const IMAGE_REL_PPC_REFHI: u16 = 16u16;
```

### `IMAGE_REL_PPC_REFLO`

```rust
const IMAGE_REL_PPC_REFLO: u16 = 17u16;
```

### `IMAGE_REL_PPC_PAIR`

```rust
const IMAGE_REL_PPC_PAIR: u16 = 18u16;
```

### `IMAGE_REL_PPC_SECRELLO`

```rust
const IMAGE_REL_PPC_SECRELLO: u16 = 19u16;
```

Low 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_PPC_SECRELHI`

```rust
const IMAGE_REL_PPC_SECRELHI: u16 = 20u16;
```

High 16-bit section relative reference (used for >32k TLS)

### `IMAGE_REL_PPC_GPREL`

```rust
const IMAGE_REL_PPC_GPREL: u16 = 21u16;
```

### `IMAGE_REL_PPC_TOKEN`

```rust
const IMAGE_REL_PPC_TOKEN: u16 = 22u16;
```

clr token

### `IMAGE_REL_PPC_TYPEMASK`

```rust
const IMAGE_REL_PPC_TYPEMASK: u16 = 255u16;
```

mask to isolate above values in IMAGE_RELOCATION.Type

### `IMAGE_REL_PPC_NEG`

```rust
const IMAGE_REL_PPC_NEG: u16 = 256u16;
```

subtract reloc value rather than adding it

### `IMAGE_REL_PPC_BRTAKEN`

```rust
const IMAGE_REL_PPC_BRTAKEN: u16 = 512u16;
```

fix branch prediction bit to predict branch taken

### `IMAGE_REL_PPC_BRNTAKEN`

```rust
const IMAGE_REL_PPC_BRNTAKEN: u16 = 1_024u16;
```

fix branch prediction bit to predict branch not taken

### `IMAGE_REL_PPC_TOCDEFN`

```rust
const IMAGE_REL_PPC_TOCDEFN: u16 = 2_048u16;
```

toc slot defined in file (or, data in toc)

### `IMAGE_REL_SH3_ABSOLUTE`

```rust
const IMAGE_REL_SH3_ABSOLUTE: u16 = 0u16;
```

No relocation

### `IMAGE_REL_SH3_DIRECT16`

```rust
const IMAGE_REL_SH3_DIRECT16: u16 = 1u16;
```

16 bit direct

### `IMAGE_REL_SH3_DIRECT32`

```rust
const IMAGE_REL_SH3_DIRECT32: u16 = 2u16;
```

32 bit direct

### `IMAGE_REL_SH3_DIRECT8`

```rust
const IMAGE_REL_SH3_DIRECT8: u16 = 3u16;
```

8 bit direct, -128..255

### `IMAGE_REL_SH3_DIRECT8_WORD`

```rust
const IMAGE_REL_SH3_DIRECT8_WORD: u16 = 4u16;
```

8 bit direct .W (0 ext.)

### `IMAGE_REL_SH3_DIRECT8_LONG`

```rust
const IMAGE_REL_SH3_DIRECT8_LONG: u16 = 5u16;
```

8 bit direct .L (0 ext.)

### `IMAGE_REL_SH3_DIRECT4`

```rust
const IMAGE_REL_SH3_DIRECT4: u16 = 6u16;
```

4 bit direct (0 ext.)

### `IMAGE_REL_SH3_DIRECT4_WORD`

```rust
const IMAGE_REL_SH3_DIRECT4_WORD: u16 = 7u16;
```

4 bit direct .W (0 ext.)

### `IMAGE_REL_SH3_DIRECT4_LONG`

```rust
const IMAGE_REL_SH3_DIRECT4_LONG: u16 = 8u16;
```

4 bit direct .L (0 ext.)

### `IMAGE_REL_SH3_PCREL8_WORD`

```rust
const IMAGE_REL_SH3_PCREL8_WORD: u16 = 9u16;
```

8 bit PC relative .W

### `IMAGE_REL_SH3_PCREL8_LONG`

```rust
const IMAGE_REL_SH3_PCREL8_LONG: u16 = 10u16;
```

8 bit PC relative .L

### `IMAGE_REL_SH3_PCREL12_WORD`

```rust
const IMAGE_REL_SH3_PCREL12_WORD: u16 = 11u16;
```

12 LSB PC relative .W

### `IMAGE_REL_SH3_STARTOF_SECTION`

```rust
const IMAGE_REL_SH3_STARTOF_SECTION: u16 = 12u16;
```

Start of EXE section

### `IMAGE_REL_SH3_SIZEOF_SECTION`

```rust
const IMAGE_REL_SH3_SIZEOF_SECTION: u16 = 13u16;
```

Size of EXE section

### `IMAGE_REL_SH3_SECTION`

```rust
const IMAGE_REL_SH3_SECTION: u16 = 14u16;
```

Section table index

### `IMAGE_REL_SH3_SECREL`

```rust
const IMAGE_REL_SH3_SECREL: u16 = 15u16;
```

Offset within section

### `IMAGE_REL_SH3_DIRECT32_NB`

```rust
const IMAGE_REL_SH3_DIRECT32_NB: u16 = 16u16;
```

32 bit direct not based

### `IMAGE_REL_SH3_GPREL4_LONG`

```rust
const IMAGE_REL_SH3_GPREL4_LONG: u16 = 17u16;
```

GP-relative addressing

### `IMAGE_REL_SH3_TOKEN`

```rust
const IMAGE_REL_SH3_TOKEN: u16 = 18u16;
```

clr token

### `IMAGE_REL_SHM_PCRELPT`

```rust
const IMAGE_REL_SHM_PCRELPT: u16 = 19u16;
```

Offset from current instruction in longwords
if not NOMODE, insert the inverse of the low bit at bit 32 to select PTA/PTB

### `IMAGE_REL_SHM_REFLO`

```rust
const IMAGE_REL_SHM_REFLO: u16 = 20u16;
```

Low bits of 32-bit address

### `IMAGE_REL_SHM_REFHALF`

```rust
const IMAGE_REL_SHM_REFHALF: u16 = 21u16;
```

High bits of 32-bit address

### `IMAGE_REL_SHM_RELLO`

```rust
const IMAGE_REL_SHM_RELLO: u16 = 22u16;
```

Low bits of relative reference

### `IMAGE_REL_SHM_RELHALF`

```rust
const IMAGE_REL_SHM_RELHALF: u16 = 23u16;
```

High bits of relative reference

### `IMAGE_REL_SHM_PAIR`

```rust
const IMAGE_REL_SHM_PAIR: u16 = 24u16;
```

offset operand for relocation

### `IMAGE_REL_SH_NOMODE`

```rust
const IMAGE_REL_SH_NOMODE: u16 = 32_768u16;
```

relocation ignores section mode

### `IMAGE_REL_ARM_ABSOLUTE`

```rust
const IMAGE_REL_ARM_ABSOLUTE: u16 = 0u16;
```

No relocation required

### `IMAGE_REL_ARM_ADDR32`

```rust
const IMAGE_REL_ARM_ADDR32: u16 = 1u16;
```

32 bit address

### `IMAGE_REL_ARM_ADDR32NB`

```rust
const IMAGE_REL_ARM_ADDR32NB: u16 = 2u16;
```

32 bit address w/o image base

### `IMAGE_REL_ARM_BRANCH24`

```rust
const IMAGE_REL_ARM_BRANCH24: u16 = 3u16;
```

24 bit offset << 2 & sign ext.

### `IMAGE_REL_ARM_BRANCH11`

```rust
const IMAGE_REL_ARM_BRANCH11: u16 = 4u16;
```

Thumb: 2 11 bit offsets

### `IMAGE_REL_ARM_TOKEN`

```rust
const IMAGE_REL_ARM_TOKEN: u16 = 5u16;
```

clr token

### `IMAGE_REL_ARM_GPREL12`

```rust
const IMAGE_REL_ARM_GPREL12: u16 = 6u16;
```

GP-relative addressing (ARM)

### `IMAGE_REL_ARM_GPREL7`

```rust
const IMAGE_REL_ARM_GPREL7: u16 = 7u16;
```

GP-relative addressing (Thumb)

### `IMAGE_REL_ARM_BLX24`

```rust
const IMAGE_REL_ARM_BLX24: u16 = 8u16;
```

### `IMAGE_REL_ARM_BLX11`

```rust
const IMAGE_REL_ARM_BLX11: u16 = 9u16;
```

### `IMAGE_REL_ARM_REL32`

```rust
const IMAGE_REL_ARM_REL32: u16 = 10u16;
```

32-bit relative address from byte following reloc

### `IMAGE_REL_ARM_SECTION`

```rust
const IMAGE_REL_ARM_SECTION: u16 = 14u16;
```

Section table index

### `IMAGE_REL_ARM_SECREL`

```rust
const IMAGE_REL_ARM_SECREL: u16 = 15u16;
```

Offset within section

### `IMAGE_REL_ARM_MOV32A`

```rust
const IMAGE_REL_ARM_MOV32A: u16 = 16u16;
```

ARM: MOVW/MOVT

### `IMAGE_REL_ARM_MOV32`

```rust
const IMAGE_REL_ARM_MOV32: u16 = 16u16;
```

ARM: MOVW/MOVT (deprecated)

### `IMAGE_REL_ARM_MOV32T`

```rust
const IMAGE_REL_ARM_MOV32T: u16 = 17u16;
```

Thumb: MOVW/MOVT

### `IMAGE_REL_THUMB_MOV32`

```rust
const IMAGE_REL_THUMB_MOV32: u16 = 17u16;
```

Thumb: MOVW/MOVT (deprecated)

### `IMAGE_REL_ARM_BRANCH20T`

```rust
const IMAGE_REL_ARM_BRANCH20T: u16 = 18u16;
```

Thumb: 32-bit conditional B

### `IMAGE_REL_THUMB_BRANCH20`

```rust
const IMAGE_REL_THUMB_BRANCH20: u16 = 18u16;
```

Thumb: 32-bit conditional B (deprecated)

### `IMAGE_REL_ARM_BRANCH24T`

```rust
const IMAGE_REL_ARM_BRANCH24T: u16 = 20u16;
```

Thumb: 32-bit B or BL

### `IMAGE_REL_THUMB_BRANCH24`

```rust
const IMAGE_REL_THUMB_BRANCH24: u16 = 20u16;
```

Thumb: 32-bit B or BL (deprecated)

### `IMAGE_REL_ARM_BLX23T`

```rust
const IMAGE_REL_ARM_BLX23T: u16 = 21u16;
```

Thumb: BLX immediate

### `IMAGE_REL_THUMB_BLX23`

```rust
const IMAGE_REL_THUMB_BLX23: u16 = 21u16;
```

Thumb: BLX immediate (deprecated)

### `IMAGE_REL_AM_ABSOLUTE`

```rust
const IMAGE_REL_AM_ABSOLUTE: u16 = 0u16;
```

### `IMAGE_REL_AM_ADDR32`

```rust
const IMAGE_REL_AM_ADDR32: u16 = 1u16;
```

### `IMAGE_REL_AM_ADDR32NB`

```rust
const IMAGE_REL_AM_ADDR32NB: u16 = 2u16;
```

### `IMAGE_REL_AM_CALL32`

```rust
const IMAGE_REL_AM_CALL32: u16 = 3u16;
```

### `IMAGE_REL_AM_FUNCINFO`

```rust
const IMAGE_REL_AM_FUNCINFO: u16 = 4u16;
```

### `IMAGE_REL_AM_REL32_1`

```rust
const IMAGE_REL_AM_REL32_1: u16 = 5u16;
```

### `IMAGE_REL_AM_REL32_2`

```rust
const IMAGE_REL_AM_REL32_2: u16 = 6u16;
```

### `IMAGE_REL_AM_SECREL`

```rust
const IMAGE_REL_AM_SECREL: u16 = 7u16;
```

### `IMAGE_REL_AM_SECTION`

```rust
const IMAGE_REL_AM_SECTION: u16 = 8u16;
```

### `IMAGE_REL_AM_TOKEN`

```rust
const IMAGE_REL_AM_TOKEN: u16 = 9u16;
```

### `IMAGE_REL_ARM64_ABSOLUTE`

```rust
const IMAGE_REL_ARM64_ABSOLUTE: u16 = 0u16;
```

No relocation required

### `IMAGE_REL_ARM64_ADDR32`

```rust
const IMAGE_REL_ARM64_ADDR32: u16 = 1u16;
```

32 bit address. Review! do we need it?

### `IMAGE_REL_ARM64_ADDR32NB`

```rust
const IMAGE_REL_ARM64_ADDR32NB: u16 = 2u16;
```

32 bit address w/o image base (RVA: for Data/PData/XData)

### `IMAGE_REL_ARM64_BRANCH26`

```rust
const IMAGE_REL_ARM64_BRANCH26: u16 = 3u16;
```

26 bit offset << 2 & sign ext. for B & BL

### `IMAGE_REL_ARM64_PAGEBASE_REL21`

```rust
const IMAGE_REL_ARM64_PAGEBASE_REL21: u16 = 4u16;
```

ADRP

### `IMAGE_REL_ARM64_REL21`

```rust
const IMAGE_REL_ARM64_REL21: u16 = 5u16;
```

ADR

### `IMAGE_REL_ARM64_PAGEOFFSET_12A`

```rust
const IMAGE_REL_ARM64_PAGEOFFSET_12A: u16 = 6u16;
```

ADD/ADDS (immediate) with zero shift, for page offset

### `IMAGE_REL_ARM64_PAGEOFFSET_12L`

```rust
const IMAGE_REL_ARM64_PAGEOFFSET_12L: u16 = 7u16;
```

LDR (indexed, unsigned immediate), for page offset

### `IMAGE_REL_ARM64_SECREL`

```rust
const IMAGE_REL_ARM64_SECREL: u16 = 8u16;
```

Offset within section

### `IMAGE_REL_ARM64_SECREL_LOW12A`

```rust
const IMAGE_REL_ARM64_SECREL_LOW12A: u16 = 9u16;
```

ADD/ADDS (immediate) with zero shift, for bit 0:11 of section offset

### `IMAGE_REL_ARM64_SECREL_HIGH12A`

```rust
const IMAGE_REL_ARM64_SECREL_HIGH12A: u16 = 10u16;
```

ADD/ADDS (immediate) with zero shift, for bit 12:23 of section offset

### `IMAGE_REL_ARM64_SECREL_LOW12L`

```rust
const IMAGE_REL_ARM64_SECREL_LOW12L: u16 = 11u16;
```

LDR (indexed, unsigned immediate), for bit 0:11 of section offset

### `IMAGE_REL_ARM64_TOKEN`

```rust
const IMAGE_REL_ARM64_TOKEN: u16 = 12u16;
```

### `IMAGE_REL_ARM64_SECTION`

```rust
const IMAGE_REL_ARM64_SECTION: u16 = 13u16;
```

Section table index

### `IMAGE_REL_ARM64_ADDR64`

```rust
const IMAGE_REL_ARM64_ADDR64: u16 = 14u16;
```

64 bit address

### `IMAGE_REL_ARM64_BRANCH19`

```rust
const IMAGE_REL_ARM64_BRANCH19: u16 = 15u16;
```

19 bit offset << 2 & sign ext. for conditional B

### `IMAGE_REL_ARM64_BRANCH14`

```rust
const IMAGE_REL_ARM64_BRANCH14: u16 = 16u16;
```

TBZ/TBNZ

### `IMAGE_REL_ARM64_REL32`

```rust
const IMAGE_REL_ARM64_REL32: u16 = 17u16;
```

32-bit relative address from byte following reloc

### `IMAGE_REL_AMD64_ABSOLUTE`

```rust
const IMAGE_REL_AMD64_ABSOLUTE: u16 = 0u16;
```

Reference is absolute, no relocation is necessary

### `IMAGE_REL_AMD64_ADDR64`

```rust
const IMAGE_REL_AMD64_ADDR64: u16 = 1u16;
```

64-bit address (VA).

### `IMAGE_REL_AMD64_ADDR32`

```rust
const IMAGE_REL_AMD64_ADDR32: u16 = 2u16;
```

32-bit address (VA).

### `IMAGE_REL_AMD64_ADDR32NB`

```rust
const IMAGE_REL_AMD64_ADDR32NB: u16 = 3u16;
```

32-bit address w/o image base (RVA).

### `IMAGE_REL_AMD64_REL32`

```rust
const IMAGE_REL_AMD64_REL32: u16 = 4u16;
```

32-bit relative address from byte following reloc

### `IMAGE_REL_AMD64_REL32_1`

```rust
const IMAGE_REL_AMD64_REL32_1: u16 = 5u16;
```

32-bit relative address from byte distance 1 from reloc

### `IMAGE_REL_AMD64_REL32_2`

```rust
const IMAGE_REL_AMD64_REL32_2: u16 = 6u16;
```

32-bit relative address from byte distance 2 from reloc

### `IMAGE_REL_AMD64_REL32_3`

```rust
const IMAGE_REL_AMD64_REL32_3: u16 = 7u16;
```

32-bit relative address from byte distance 3 from reloc

### `IMAGE_REL_AMD64_REL32_4`

```rust
const IMAGE_REL_AMD64_REL32_4: u16 = 8u16;
```

32-bit relative address from byte distance 4 from reloc

### `IMAGE_REL_AMD64_REL32_5`

```rust
const IMAGE_REL_AMD64_REL32_5: u16 = 9u16;
```

32-bit relative address from byte distance 5 from reloc

### `IMAGE_REL_AMD64_SECTION`

```rust
const IMAGE_REL_AMD64_SECTION: u16 = 10u16;
```

Section index

### `IMAGE_REL_AMD64_SECREL`

```rust
const IMAGE_REL_AMD64_SECREL: u16 = 11u16;
```

32 bit offset from base of section containing target

### `IMAGE_REL_AMD64_SECREL7`

```rust
const IMAGE_REL_AMD64_SECREL7: u16 = 12u16;
```

7 bit unsigned offset from base of section containing target

### `IMAGE_REL_AMD64_TOKEN`

```rust
const IMAGE_REL_AMD64_TOKEN: u16 = 13u16;
```

32 bit metadata token

### `IMAGE_REL_AMD64_SREL32`

```rust
const IMAGE_REL_AMD64_SREL32: u16 = 14u16;
```

32 bit signed span-dependent value emitted into object

### `IMAGE_REL_AMD64_PAIR`

```rust
const IMAGE_REL_AMD64_PAIR: u16 = 15u16;
```

### `IMAGE_REL_AMD64_SSPAN32`

```rust
const IMAGE_REL_AMD64_SSPAN32: u16 = 16u16;
```

32 bit signed span-dependent value applied at link time

### `IMAGE_REL_AMD64_EHANDLER`

```rust
const IMAGE_REL_AMD64_EHANDLER: u16 = 17u16;
```

### `IMAGE_REL_AMD64_IMPORT_BR`

```rust
const IMAGE_REL_AMD64_IMPORT_BR: u16 = 18u16;
```

Indirect branch to an import

### `IMAGE_REL_AMD64_IMPORT_CALL`

```rust
const IMAGE_REL_AMD64_IMPORT_CALL: u16 = 19u16;
```

Indirect call to an import

### `IMAGE_REL_AMD64_CFG_BR`

```rust
const IMAGE_REL_AMD64_CFG_BR: u16 = 20u16;
```

Indirect branch to a CFG check

### `IMAGE_REL_AMD64_CFG_BR_REX`

```rust
const IMAGE_REL_AMD64_CFG_BR_REX: u16 = 21u16;
```

Indirect branch to a CFG check, with REX.W prefix

### `IMAGE_REL_AMD64_CFG_CALL`

```rust
const IMAGE_REL_AMD64_CFG_CALL: u16 = 22u16;
```

Indirect call to a CFG check

### `IMAGE_REL_AMD64_INDIR_BR`

```rust
const IMAGE_REL_AMD64_INDIR_BR: u16 = 23u16;
```

Indirect branch to a target in RAX (no CFG)

### `IMAGE_REL_AMD64_INDIR_BR_REX`

```rust
const IMAGE_REL_AMD64_INDIR_BR_REX: u16 = 24u16;
```

Indirect branch to a target in RAX, with REX.W prefix (no CFG)

### `IMAGE_REL_AMD64_INDIR_CALL`

```rust
const IMAGE_REL_AMD64_INDIR_CALL: u16 = 25u16;
```

Indirect call to a target in RAX (no CFG)

### `IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST`

```rust
const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST: u16 = 32u16;
```

Indirect branch for a switch table using Reg 0 (RAX)

### `IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST`

```rust
const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST: u16 = 47u16;
```

Indirect branch for a switch table using Reg 15 (R15)

### `IMAGE_REL_IA64_ABSOLUTE`

```rust
const IMAGE_REL_IA64_ABSOLUTE: u16 = 0u16;
```

### `IMAGE_REL_IA64_IMM14`

```rust
const IMAGE_REL_IA64_IMM14: u16 = 1u16;
```

### `IMAGE_REL_IA64_IMM22`

```rust
const IMAGE_REL_IA64_IMM22: u16 = 2u16;
```

### `IMAGE_REL_IA64_IMM64`

```rust
const IMAGE_REL_IA64_IMM64: u16 = 3u16;
```

### `IMAGE_REL_IA64_DIR32`

```rust
const IMAGE_REL_IA64_DIR32: u16 = 4u16;
```

### `IMAGE_REL_IA64_DIR64`

```rust
const IMAGE_REL_IA64_DIR64: u16 = 5u16;
```

### `IMAGE_REL_IA64_PCREL21B`

```rust
const IMAGE_REL_IA64_PCREL21B: u16 = 6u16;
```

### `IMAGE_REL_IA64_PCREL21M`

```rust
const IMAGE_REL_IA64_PCREL21M: u16 = 7u16;
```

### `IMAGE_REL_IA64_PCREL21F`

```rust
const IMAGE_REL_IA64_PCREL21F: u16 = 8u16;
```

### `IMAGE_REL_IA64_GPREL22`

```rust
const IMAGE_REL_IA64_GPREL22: u16 = 9u16;
```

### `IMAGE_REL_IA64_LTOFF22`

```rust
const IMAGE_REL_IA64_LTOFF22: u16 = 10u16;
```

### `IMAGE_REL_IA64_SECTION`

```rust
const IMAGE_REL_IA64_SECTION: u16 = 11u16;
```

### `IMAGE_REL_IA64_SECREL22`

```rust
const IMAGE_REL_IA64_SECREL22: u16 = 12u16;
```

### `IMAGE_REL_IA64_SECREL64I`

```rust
const IMAGE_REL_IA64_SECREL64I: u16 = 13u16;
```

### `IMAGE_REL_IA64_SECREL32`

```rust
const IMAGE_REL_IA64_SECREL32: u16 = 14u16;
```

### `IMAGE_REL_IA64_DIR32NB`

```rust
const IMAGE_REL_IA64_DIR32NB: u16 = 16u16;
```

### `IMAGE_REL_IA64_SREL14`

```rust
const IMAGE_REL_IA64_SREL14: u16 = 17u16;
```

### `IMAGE_REL_IA64_SREL22`

```rust
const IMAGE_REL_IA64_SREL22: u16 = 18u16;
```

### `IMAGE_REL_IA64_SREL32`

```rust
const IMAGE_REL_IA64_SREL32: u16 = 19u16;
```

### `IMAGE_REL_IA64_UREL32`

```rust
const IMAGE_REL_IA64_UREL32: u16 = 20u16;
```

### `IMAGE_REL_IA64_PCREL60X`

```rust
const IMAGE_REL_IA64_PCREL60X: u16 = 21u16;
```

This is always a BRL and never converted

### `IMAGE_REL_IA64_PCREL60B`

```rust
const IMAGE_REL_IA64_PCREL60B: u16 = 22u16;
```

If possible, convert to MBB bundle with NOP.B in slot 1

### `IMAGE_REL_IA64_PCREL60F`

```rust
const IMAGE_REL_IA64_PCREL60F: u16 = 23u16;
```

If possible, convert to MFB bundle with NOP.F in slot 1

### `IMAGE_REL_IA64_PCREL60I`

```rust
const IMAGE_REL_IA64_PCREL60I: u16 = 24u16;
```

If possible, convert to MIB bundle with NOP.I in slot 1

### `IMAGE_REL_IA64_PCREL60M`

```rust
const IMAGE_REL_IA64_PCREL60M: u16 = 25u16;
```

If possible, convert to MMB bundle with NOP.M in slot 1

### `IMAGE_REL_IA64_IMMGPREL64`

```rust
const IMAGE_REL_IA64_IMMGPREL64: u16 = 26u16;
```

### `IMAGE_REL_IA64_TOKEN`

```rust
const IMAGE_REL_IA64_TOKEN: u16 = 27u16;
```

clr token

### `IMAGE_REL_IA64_GPREL32`

```rust
const IMAGE_REL_IA64_GPREL32: u16 = 28u16;
```

### `IMAGE_REL_IA64_ADDEND`

```rust
const IMAGE_REL_IA64_ADDEND: u16 = 31u16;
```

### `IMAGE_REL_CEF_ABSOLUTE`

```rust
const IMAGE_REL_CEF_ABSOLUTE: u16 = 0u16;
```

Reference is absolute, no relocation is necessary

### `IMAGE_REL_CEF_ADDR32`

```rust
const IMAGE_REL_CEF_ADDR32: u16 = 1u16;
```

32-bit address (VA).

### `IMAGE_REL_CEF_ADDR64`

```rust
const IMAGE_REL_CEF_ADDR64: u16 = 2u16;
```

64-bit address (VA).

### `IMAGE_REL_CEF_ADDR32NB`

```rust
const IMAGE_REL_CEF_ADDR32NB: u16 = 3u16;
```

32-bit address w/o image base (RVA).

### `IMAGE_REL_CEF_SECTION`

```rust
const IMAGE_REL_CEF_SECTION: u16 = 4u16;
```

Section index

### `IMAGE_REL_CEF_SECREL`

```rust
const IMAGE_REL_CEF_SECREL: u16 = 5u16;
```

32 bit offset from base of section containing target

### `IMAGE_REL_CEF_TOKEN`

```rust
const IMAGE_REL_CEF_TOKEN: u16 = 6u16;
```

32 bit metadata token

### `IMAGE_REL_CEE_ABSOLUTE`

```rust
const IMAGE_REL_CEE_ABSOLUTE: u16 = 0u16;
```

Reference is absolute, no relocation is necessary

### `IMAGE_REL_CEE_ADDR32`

```rust
const IMAGE_REL_CEE_ADDR32: u16 = 1u16;
```

32-bit address (VA).

### `IMAGE_REL_CEE_ADDR64`

```rust
const IMAGE_REL_CEE_ADDR64: u16 = 2u16;
```

64-bit address (VA).

### `IMAGE_REL_CEE_ADDR32NB`

```rust
const IMAGE_REL_CEE_ADDR32NB: u16 = 3u16;
```

32-bit address w/o image base (RVA).

### `IMAGE_REL_CEE_SECTION`

```rust
const IMAGE_REL_CEE_SECTION: u16 = 4u16;
```

Section index

### `IMAGE_REL_CEE_SECREL`

```rust
const IMAGE_REL_CEE_SECREL: u16 = 5u16;
```

32 bit offset from base of section containing target

### `IMAGE_REL_CEE_TOKEN`

```rust
const IMAGE_REL_CEE_TOKEN: u16 = 6u16;
```

32 bit metadata token

### `IMAGE_REL_M32R_ABSOLUTE`

```rust
const IMAGE_REL_M32R_ABSOLUTE: u16 = 0u16;
```

No relocation required

### `IMAGE_REL_M32R_ADDR32`

```rust
const IMAGE_REL_M32R_ADDR32: u16 = 1u16;
```

32 bit address

### `IMAGE_REL_M32R_ADDR32NB`

```rust
const IMAGE_REL_M32R_ADDR32NB: u16 = 2u16;
```

32 bit address w/o image base

### `IMAGE_REL_M32R_ADDR24`

```rust
const IMAGE_REL_M32R_ADDR24: u16 = 3u16;
```

24 bit address

### `IMAGE_REL_M32R_GPREL16`

```rust
const IMAGE_REL_M32R_GPREL16: u16 = 4u16;
```

GP relative addressing

### `IMAGE_REL_M32R_PCREL24`

```rust
const IMAGE_REL_M32R_PCREL24: u16 = 5u16;
```

24 bit offset << 2 & sign ext.

### `IMAGE_REL_M32R_PCREL16`

```rust
const IMAGE_REL_M32R_PCREL16: u16 = 6u16;
```

16 bit offset << 2 & sign ext.

### `IMAGE_REL_M32R_PCREL8`

```rust
const IMAGE_REL_M32R_PCREL8: u16 = 7u16;
```

8 bit offset << 2 & sign ext.

### `IMAGE_REL_M32R_REFHALF`

```rust
const IMAGE_REL_M32R_REFHALF: u16 = 8u16;
```

16 MSBs

### `IMAGE_REL_M32R_REFHI`

```rust
const IMAGE_REL_M32R_REFHI: u16 = 9u16;
```

16 MSBs; adj for LSB sign ext.

### `IMAGE_REL_M32R_REFLO`

```rust
const IMAGE_REL_M32R_REFLO: u16 = 10u16;
```

16 LSBs

### `IMAGE_REL_M32R_PAIR`

```rust
const IMAGE_REL_M32R_PAIR: u16 = 11u16;
```

Link HI and LO

### `IMAGE_REL_M32R_SECTION`

```rust
const IMAGE_REL_M32R_SECTION: u16 = 12u16;
```

Section table index

### `IMAGE_REL_M32R_SECREL32`

```rust
const IMAGE_REL_M32R_SECREL32: u16 = 13u16;
```

32 bit section relative reference

### `IMAGE_REL_M32R_TOKEN`

```rust
const IMAGE_REL_M32R_TOKEN: u16 = 14u16;
```

clr token

### `IMAGE_REL_EBC_ABSOLUTE`

```rust
const IMAGE_REL_EBC_ABSOLUTE: u16 = 0u16;
```

No relocation required

### `IMAGE_REL_EBC_ADDR32NB`

```rust
const IMAGE_REL_EBC_ADDR32NB: u16 = 1u16;
```

32 bit address w/o image base

### `IMAGE_REL_EBC_REL32`

```rust
const IMAGE_REL_EBC_REL32: u16 = 2u16;
```

32-bit relative address from byte following reloc

### `IMAGE_REL_EBC_SECTION`

```rust
const IMAGE_REL_EBC_SECTION: u16 = 3u16;
```

Section table index

### `IMAGE_REL_EBC_SECREL`

```rust
const IMAGE_REL_EBC_SECREL: u16 = 4u16;
```

Offset within section

### `EMARCH_ENC_I17_IMM7B_INST_WORD_X`

```rust
const EMARCH_ENC_I17_IMM7B_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM7B_SIZE_X`

```rust
const EMARCH_ENC_I17_IMM7B_SIZE_X: u16 = 7u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X: u16 = 4u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM7B_VAL_POS_X`

```rust
const EMARCH_ENC_I17_IMM7B_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_INST_WORD_X`

```rust
const EMARCH_ENC_I17_IMM9D_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_SIZE_X`

```rust
const EMARCH_ENC_I17_IMM9D_SIZE_X: u16 = 9u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X: u16 = 18u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM9D_VAL_POS_X`

```rust
const EMARCH_ENC_I17_IMM9D_VAL_POS_X: u16 = 7u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_INST_WORD_X`

```rust
const EMARCH_ENC_I17_IMM5C_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_SIZE_X`

```rust
const EMARCH_ENC_I17_IMM5C_SIZE_X: u16 = 5u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X: u16 = 13u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM5C_VAL_POS_X`

```rust
const EMARCH_ENC_I17_IMM5C_VAL_POS_X: u16 = 16u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_INST_WORD_X`

```rust
const EMARCH_ENC_I17_IC_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_SIZE_X`

```rust
const EMARCH_ENC_I17_IC_SIZE_X: u16 = 1u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_IC_INST_WORD_POS_X: u16 = 12u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IC_VAL_POS_X`

```rust
const EMARCH_ENC_I17_IC_VAL_POS_X: u16 = 21u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_INST_WORD_X`

```rust
const EMARCH_ENC_I17_IMM41A_INST_WORD_X: u16 = 1u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_SIZE_X`

```rust
const EMARCH_ENC_I17_IMM41A_SIZE_X: u16 = 10u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_IMM41A_INST_WORD_POS_X: u16 = 14u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41A_VAL_POS_X`

```rust
const EMARCH_ENC_I17_IMM41A_VAL_POS_X: u16 = 22u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_INST_WORD_X`

```rust
const EMARCH_ENC_I17_IMM41B_INST_WORD_X: u16 = 1u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_SIZE_X`

```rust
const EMARCH_ENC_I17_IMM41B_SIZE_X: u16 = 8u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_IMM41B_INST_WORD_POS_X: u16 = 24u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41B_VAL_POS_X`

```rust
const EMARCH_ENC_I17_IMM41B_VAL_POS_X: u16 = 32u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_INST_WORD_X`

```rust
const EMARCH_ENC_I17_IMM41C_INST_WORD_X: u16 = 2u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_SIZE_X`

```rust
const EMARCH_ENC_I17_IMM41C_SIZE_X: u16 = 23u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_IMM41C_INST_WORD_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_IMM41C_VAL_POS_X`

```rust
const EMARCH_ENC_I17_IMM41C_VAL_POS_X: u16 = 40u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_INST_WORD_X`

```rust
const EMARCH_ENC_I17_SIGN_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_SIZE_X`

```rust
const EMARCH_ENC_I17_SIGN_SIZE_X: u16 = 1u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_INST_WORD_POS_X`

```rust
const EMARCH_ENC_I17_SIGN_INST_WORD_POS_X: u16 = 27u16;
```

Intel-IA64-Filler

### `EMARCH_ENC_I17_SIGN_VAL_POS_X`

```rust
const EMARCH_ENC_I17_SIGN_VAL_POS_X: u16 = 63u16;
```

Intel-IA64-Filler

### `X3_OPCODE_INST_WORD_X`

```rust
const X3_OPCODE_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `X3_OPCODE_SIZE_X`

```rust
const X3_OPCODE_SIZE_X: u16 = 4u16;
```

Intel-IA64-Filler

### `X3_OPCODE_INST_WORD_POS_X`

```rust
const X3_OPCODE_INST_WORD_POS_X: u16 = 28u16;
```

Intel-IA64-Filler

### `X3_OPCODE_SIGN_VAL_POS_X`

```rust
const X3_OPCODE_SIGN_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_I_INST_WORD_X`

```rust
const X3_I_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `X3_I_SIZE_X`

```rust
const X3_I_SIZE_X: u16 = 1u16;
```

Intel-IA64-Filler

### `X3_I_INST_WORD_POS_X`

```rust
const X3_I_INST_WORD_POS_X: u16 = 27u16;
```

Intel-IA64-Filler

### `X3_I_SIGN_VAL_POS_X`

```rust
const X3_I_SIGN_VAL_POS_X: u16 = 59u16;
```

Intel-IA64-Filler

### `X3_D_WH_INST_WORD_X`

```rust
const X3_D_WH_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `X3_D_WH_SIZE_X`

```rust
const X3_D_WH_SIZE_X: u16 = 3u16;
```

Intel-IA64-Filler

### `X3_D_WH_INST_WORD_POS_X`

```rust
const X3_D_WH_INST_WORD_POS_X: u16 = 24u16;
```

Intel-IA64-Filler

### `X3_D_WH_SIGN_VAL_POS_X`

```rust
const X3_D_WH_SIGN_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_IMM20_INST_WORD_X`

```rust
const X3_IMM20_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `X3_IMM20_SIZE_X`

```rust
const X3_IMM20_SIZE_X: u16 = 20u16;
```

Intel-IA64-Filler

### `X3_IMM20_INST_WORD_POS_X`

```rust
const X3_IMM20_INST_WORD_POS_X: u16 = 4u16;
```

Intel-IA64-Filler

### `X3_IMM20_SIGN_VAL_POS_X`

```rust
const X3_IMM20_SIGN_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_IMM39_1_INST_WORD_X`

```rust
const X3_IMM39_1_INST_WORD_X: u16 = 2u16;
```

Intel-IA64-Filler

### `X3_IMM39_1_SIZE_X`

```rust
const X3_IMM39_1_SIZE_X: u16 = 23u16;
```

Intel-IA64-Filler

### `X3_IMM39_1_INST_WORD_POS_X`

```rust
const X3_IMM39_1_INST_WORD_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_IMM39_1_SIGN_VAL_POS_X`

```rust
const X3_IMM39_1_SIGN_VAL_POS_X: u16 = 36u16;
```

Intel-IA64-Filler

### `X3_IMM39_2_INST_WORD_X`

```rust
const X3_IMM39_2_INST_WORD_X: u16 = 1u16;
```

Intel-IA64-Filler

### `X3_IMM39_2_SIZE_X`

```rust
const X3_IMM39_2_SIZE_X: u16 = 16u16;
```

Intel-IA64-Filler

### `X3_IMM39_2_INST_WORD_POS_X`

```rust
const X3_IMM39_2_INST_WORD_POS_X: u16 = 16u16;
```

Intel-IA64-Filler

### `X3_IMM39_2_SIGN_VAL_POS_X`

```rust
const X3_IMM39_2_SIGN_VAL_POS_X: u16 = 20u16;
```

Intel-IA64-Filler

### `X3_P_INST_WORD_X`

```rust
const X3_P_INST_WORD_X: u16 = 3u16;
```

Intel-IA64-Filler

### `X3_P_SIZE_X`

```rust
const X3_P_SIZE_X: u16 = 4u16;
```

Intel-IA64-Filler

### `X3_P_INST_WORD_POS_X`

```rust
const X3_P_INST_WORD_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_P_SIGN_VAL_POS_X`

```rust
const X3_P_SIGN_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_TMPLT_INST_WORD_X`

```rust
const X3_TMPLT_INST_WORD_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_TMPLT_SIZE_X`

```rust
const X3_TMPLT_SIZE_X: u16 = 4u16;
```

Intel-IA64-Filler

### `X3_TMPLT_INST_WORD_POS_X`

```rust
const X3_TMPLT_INST_WORD_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_TMPLT_SIGN_VAL_POS_X`

```rust
const X3_TMPLT_SIGN_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_BTYPE_QP_INST_WORD_X`

```rust
const X3_BTYPE_QP_INST_WORD_X: u16 = 2u16;
```

Intel-IA64-Filler

### `X3_BTYPE_QP_SIZE_X`

```rust
const X3_BTYPE_QP_SIZE_X: u16 = 9u16;
```

Intel-IA64-Filler

### `X3_BTYPE_QP_INST_WORD_POS_X`

```rust
const X3_BTYPE_QP_INST_WORD_POS_X: u16 = 23u16;
```

Intel-IA64-Filler

### `X3_BTYPE_QP_INST_VAL_POS_X`

```rust
const X3_BTYPE_QP_INST_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `X3_EMPTY_INST_WORD_X`

```rust
const X3_EMPTY_INST_WORD_X: u16 = 1u16;
```

Intel-IA64-Filler

### `X3_EMPTY_SIZE_X`

```rust
const X3_EMPTY_SIZE_X: u16 = 2u16;
```

Intel-IA64-Filler

### `X3_EMPTY_INST_WORD_POS_X`

```rust
const X3_EMPTY_INST_WORD_POS_X: u16 = 14u16;
```

Intel-IA64-Filler

### `X3_EMPTY_INST_VAL_POS_X`

```rust
const X3_EMPTY_INST_VAL_POS_X: u16 = 0u16;
```

Intel-IA64-Filler

### `IMAGE_REL_BASED_ABSOLUTE`

```rust
const IMAGE_REL_BASED_ABSOLUTE: u16 = 0u16;
```

### `IMAGE_REL_BASED_HIGH`

```rust
const IMAGE_REL_BASED_HIGH: u16 = 1u16;
```

### `IMAGE_REL_BASED_LOW`

```rust
const IMAGE_REL_BASED_LOW: u16 = 2u16;
```

### `IMAGE_REL_BASED_HIGHLOW`

```rust
const IMAGE_REL_BASED_HIGHLOW: u16 = 3u16;
```

### `IMAGE_REL_BASED_HIGHADJ`

```rust
const IMAGE_REL_BASED_HIGHADJ: u16 = 4u16;
```

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_5`

```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_5: u16 = 5u16;
```

### `IMAGE_REL_BASED_RESERVED`

```rust
const IMAGE_REL_BASED_RESERVED: u16 = 6u16;
```

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_7`

```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_7: u16 = 7u16;
```

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_8`

```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_8: u16 = 8u16;
```

### `IMAGE_REL_BASED_MACHINE_SPECIFIC_9`

```rust
const IMAGE_REL_BASED_MACHINE_SPECIFIC_9: u16 = 9u16;
```

### `IMAGE_REL_BASED_DIR64`

```rust
const IMAGE_REL_BASED_DIR64: u16 = 10u16;
```

### `IMAGE_REL_BASED_IA64_IMM64`

```rust
const IMAGE_REL_BASED_IA64_IMM64: u16 = 9u16;
```

### `IMAGE_REL_BASED_MIPS_JMPADDR`

```rust
const IMAGE_REL_BASED_MIPS_JMPADDR: u16 = 5u16;
```

### `IMAGE_REL_BASED_MIPS_JMPADDR16`

```rust
const IMAGE_REL_BASED_MIPS_JMPADDR16: u16 = 9u16;
```

### `IMAGE_REL_BASED_ARM_MOV32`

```rust
const IMAGE_REL_BASED_ARM_MOV32: u16 = 5u16;
```

### `IMAGE_REL_BASED_THUMB_MOV32`

```rust
const IMAGE_REL_BASED_THUMB_MOV32: u16 = 7u16;
```

### `IMAGE_REL_BASED_RISCV_HIGH20`

```rust
const IMAGE_REL_BASED_RISCV_HIGH20: u16 = 5u16;
```

### `IMAGE_REL_BASED_RISCV_LOW12I`

```rust
const IMAGE_REL_BASED_RISCV_LOW12I: u16 = 7u16;
```

### `IMAGE_REL_BASED_RISCV_LOW12S`

```rust
const IMAGE_REL_BASED_RISCV_LOW12S: u16 = 8u16;
```

### `IMAGE_ARCHIVE_START_SIZE`

```rust
const IMAGE_ARCHIVE_START_SIZE: usize = 8usize;
```

### `IMAGE_ARCHIVE_START`

```rust
const IMAGE_ARCHIVE_START: &[u8; 8];
```

### `IMAGE_ARCHIVE_END`

```rust
const IMAGE_ARCHIVE_END: &[u8];
```

### `IMAGE_ARCHIVE_PAD`

```rust
const IMAGE_ARCHIVE_PAD: &[u8];
```

### `IMAGE_ARCHIVE_LINKER_MEMBER`

```rust
const IMAGE_ARCHIVE_LINKER_MEMBER: &[u8; 16];
```

### `IMAGE_ARCHIVE_LONGNAMES_MEMBER`

```rust
const IMAGE_ARCHIVE_LONGNAMES_MEMBER: &[u8; 16];
```

### `IMAGE_ARCHIVE_HYBRIDMAP_MEMBER`

```rust
const IMAGE_ARCHIVE_HYBRIDMAP_MEMBER: &[u8; 16];
```

### `IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR`

```rust
const IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR: u16 = 60u16;
```

### `IMAGE_ORDINAL_FLAG64`

```rust
const IMAGE_ORDINAL_FLAG64: u64 = 9_223_372_036_854_775_808u64;
```

### `IMAGE_ORDINAL_FLAG32`

```rust
const IMAGE_ORDINAL_FLAG32: u32 = 2_147_483_648u32;
```

### `IMAGE_DELAYLOAD_RVA_BASED`

```rust
const IMAGE_DELAYLOAD_RVA_BASED: u32 = 2_147_483_648u32;
```

Delay load version 2 flag for `ImageDelayloadDescriptor::attributes`.

### `IMAGE_RESOURCE_NAME_IS_STRING`

```rust
const IMAGE_RESOURCE_NAME_IS_STRING: u32 = 2_147_483_648u32;
```

### `IMAGE_RESOURCE_DATA_IS_DIRECTORY`

```rust
const IMAGE_RESOURCE_DATA_IS_DIRECTORY: u32 = 2_147_483_648u32;
```

### `RT_CURSOR`

```rust
const RT_CURSOR: u16 = 1u16;
```

ID for: Hardware-dependent cursor resource.

### `RT_BITMAP`

```rust
const RT_BITMAP: u16 = 2u16;
```

ID for: Bitmap resource.

### `RT_ICON`

```rust
const RT_ICON: u16 = 3u16;
```

ID for: Hardware-dependent icon resource.

### `RT_MENU`

```rust
const RT_MENU: u16 = 4u16;
```

ID for: Menu resource.

### `RT_DIALOG`

```rust
const RT_DIALOG: u16 = 5u16;
```

ID for: Dialog box.

### `RT_STRING`

```rust
const RT_STRING: u16 = 6u16;
```

ID for: String-table entry.

### `RT_FONTDIR`

```rust
const RT_FONTDIR: u16 = 7u16;
```

ID for: Font directory resource.

### `RT_FONT`

```rust
const RT_FONT: u16 = 8u16;
```

ID for: Font resource.

### `RT_ACCELERATOR`

```rust
const RT_ACCELERATOR: u16 = 9u16;
```

ID for: Accelerator table.

### `RT_RCDATA`

```rust
const RT_RCDATA: u16 = 10u16;
```

ID for: Application-defined resource (raw data).

### `RT_MESSAGETABLE`

```rust
const RT_MESSAGETABLE: u16 = 11u16;
```

ID for: Message-table entry.

### `RT_GROUP_CURSOR`

```rust
const RT_GROUP_CURSOR: u16 = 12u16;
```

ID for: Hardware-independent cursor resource.

### `RT_GROUP_ICON`

```rust
const RT_GROUP_ICON: u16 = 14u16;
```

ID for: Hardware-independent icon resource.

### `RT_VERSION`

```rust
const RT_VERSION: u16 = 16u16;
```

ID for: Version resource.

### `RT_DLGINCLUDE`

```rust
const RT_DLGINCLUDE: u16 = 17u16;
```

ID for: Allows a resource editing tool to associate a string with an .rc file.

### `RT_PLUGPLAY`

```rust
const RT_PLUGPLAY: u16 = 19u16;
```

ID for: Plug and Play resource.

### `RT_VXD`

```rust
const RT_VXD: u16 = 20u16;
```

ID for: VXD.

### `RT_ANICURSOR`

```rust
const RT_ANICURSOR: u16 = 21u16;
```

ID for: Animated cursor.

### `RT_ANIICON`

```rust
const RT_ANIICON: u16 = 22u16;
```

ID for: Animated icon.

### `RT_HTML`

```rust
const RT_HTML: u16 = 23u16;
```

ID for: HTML resource.

### `RT_MANIFEST`

```rust
const RT_MANIFEST: u16 = 24u16;
```

ID for: Side-by-Side Assembly Manifest.

### `IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE`

```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE: u32 = 1u32;
```

### `IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE`

```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE: u32 = 2u32;
```

### `IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER`

```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER: u32 = 3u32;
```

### `IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER`

```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER: u32 = 4u32;
```

### `IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH`

```rust
const IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH: u32 = 5u32;
```

### `IMAGE_HOT_PATCH_BASE_OBLIGATORY`

```rust
const IMAGE_HOT_PATCH_BASE_OBLIGATORY: u32 = 1u32;
```

### `IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK`

```rust
const IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK: u32 = 2u32;
```

### `IMAGE_HOT_PATCH_CHUNK_INVERSE`

```rust
const IMAGE_HOT_PATCH_CHUNK_INVERSE: u32 = 2_147_483_648u32;
```

### `IMAGE_HOT_PATCH_CHUNK_OBLIGATORY`

```rust
const IMAGE_HOT_PATCH_CHUNK_OBLIGATORY: u32 = 1_073_741_824u32;
```

### `IMAGE_HOT_PATCH_CHUNK_RESERVED`

```rust
const IMAGE_HOT_PATCH_CHUNK_RESERVED: u32 = 1_072_705_536u32;
```

### `IMAGE_HOT_PATCH_CHUNK_TYPE`

```rust
const IMAGE_HOT_PATCH_CHUNK_TYPE: u32 = 1_032_192u32;
```

### `IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA`

```rust
const IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA: u32 = 32_768u32;
```

### `IMAGE_HOT_PATCH_CHUNK_TARGET_RVA`

```rust
const IMAGE_HOT_PATCH_CHUNK_TARGET_RVA: u32 = 16_384u32;
```

### `IMAGE_HOT_PATCH_CHUNK_SIZE`

```rust
const IMAGE_HOT_PATCH_CHUNK_SIZE: u32 = 4_095u32;
```

### `IMAGE_HOT_PATCH_NONE`

```rust
const IMAGE_HOT_PATCH_NONE: u32 = 0u32;
```

### `IMAGE_HOT_PATCH_FUNCTION`

```rust
const IMAGE_HOT_PATCH_FUNCTION: u32 = 114_688u32;
```

### `IMAGE_HOT_PATCH_ABSOLUTE`

```rust
const IMAGE_HOT_PATCH_ABSOLUTE: u32 = 180_224u32;
```

### `IMAGE_HOT_PATCH_REL32`

```rust
const IMAGE_HOT_PATCH_REL32: u32 = 245_760u32;
```

### `IMAGE_HOT_PATCH_CALL_TARGET`

```rust
const IMAGE_HOT_PATCH_CALL_TARGET: u32 = 278_528u32;
```

### `IMAGE_HOT_PATCH_INDIRECT`

```rust
const IMAGE_HOT_PATCH_INDIRECT: u32 = 376_832u32;
```

### `IMAGE_HOT_PATCH_NO_CALL_TARGET`

```rust
const IMAGE_HOT_PATCH_NO_CALL_TARGET: u32 = 409_600u32;
```

### `IMAGE_HOT_PATCH_DYNAMIC_VALUE`

```rust
const IMAGE_HOT_PATCH_DYNAMIC_VALUE: u32 = 491_520u32;
```

### `IMAGE_GUARD_CF_INSTRUMENTED`

```rust
const IMAGE_GUARD_CF_INSTRUMENTED: u32 = 256u32;
```

Module performs control flow integrity checks using system-supplied support

### `IMAGE_GUARD_CFW_INSTRUMENTED`

```rust
const IMAGE_GUARD_CFW_INSTRUMENTED: u32 = 512u32;
```

Module performs control flow and write integrity checks

### `IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT`

```rust
const IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 1_024u32;
```

Module contains valid control flow target metadata

### `IMAGE_GUARD_SECURITY_COOKIE_UNUSED`

```rust
const IMAGE_GUARD_SECURITY_COOKIE_UNUSED: u32 = 2_048u32;
```

Module does not make use of the /GS security cookie

### `IMAGE_GUARD_PROTECT_DELAYLOAD_IAT`

```rust
const IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: u32 = 4_096u32;
```

Module supports read only delay load IAT

### `IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION`

```rust
const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: u32 = 8_192u32;
```

Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected

### `IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT`

```rust
const IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT: u32 = 16_384u32;
```

Module contains suppressed export information.

This also infers that the address taken taken IAT table is also present in the load config.

### `IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION`

```rust
const IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION: u32 = 32_768u32;
```

Module enables suppression of exports

### `IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT`

```rust
const IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 65_536u32;
```

Module contains longjmp target information

### `IMAGE_GUARD_RF_INSTRUMENTED`

```rust
const IMAGE_GUARD_RF_INSTRUMENTED: u32 = 131_072u32;
```

Module contains return flow instrumentation and metadata

### `IMAGE_GUARD_RF_ENABLE`

```rust
const IMAGE_GUARD_RF_ENABLE: u32 = 262_144u32;
```

Module requests that the OS enable return flow protection

### `IMAGE_GUARD_RF_STRICT`

```rust
const IMAGE_GUARD_RF_STRICT: u32 = 524_288u32;
```

Module requests that the OS enable return flow protection in strict mode

### `IMAGE_GUARD_RETPOLINE_PRESENT`

```rust
const IMAGE_GUARD_RETPOLINE_PRESENT: u32 = 1_048_576u32;
```

Module was built with retpoline support

### `IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK`

```rust
const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK: u32 = 4_026_531_840u32;
```

Stride of Guard CF function table encoded in these bits (additional count of bytes per element)

### `IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT`

```rust
const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: u32 = 28u32;
```

Shift to right-justify Guard CF function table stride

### `IMAGE_GUARD_FLAG_FID_SUPPRESSED`

```rust
const IMAGE_GUARD_FLAG_FID_SUPPRESSED: u16 = 1u16;
```

The containing GFID entry is suppressed

### `IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED`

```rust
const IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED: u16 = 2u16;
```

The containing GFID entry is export suppressed

### `IMAGE_ENCLAVE_LONG_ID_LENGTH`

```rust
const IMAGE_ENCLAVE_LONG_ID_LENGTH: usize = 32usize;
```

### `IMAGE_ENCLAVE_SHORT_ID_LENGTH`

```rust
const IMAGE_ENCLAVE_SHORT_ID_LENGTH: usize = 16usize;
```

### `IMAGE_ENCLAVE_POLICY_DEBUGGABLE`

```rust
const IMAGE_ENCLAVE_POLICY_DEBUGGABLE: u32 = 1u32;
```

### `IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE`

```rust
const IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE: u32 = 1u32;
```

### `IMAGE_ENCLAVE_IMPORT_MATCH_NONE`

```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_NONE: u32 = 0u32;
```

### `IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID`

```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID: u32 = 1u32;
```

### `IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID`

```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID: u32 = 2u32;
```

### `IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID`

```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID: u32 = 3u32;
```

### `IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID`

```rust
const IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID: u32 = 4u32;
```

### `IMAGE_DEBUG_TYPE_UNKNOWN`

```rust
const IMAGE_DEBUG_TYPE_UNKNOWN: u32 = 0u32;
```

### `IMAGE_DEBUG_TYPE_COFF`

```rust
const IMAGE_DEBUG_TYPE_COFF: u32 = 1u32;
```

### `IMAGE_DEBUG_TYPE_CODEVIEW`

```rust
const IMAGE_DEBUG_TYPE_CODEVIEW: u32 = 2u32;
```

### `IMAGE_DEBUG_TYPE_FPO`

```rust
const IMAGE_DEBUG_TYPE_FPO: u32 = 3u32;
```

### `IMAGE_DEBUG_TYPE_MISC`

```rust
const IMAGE_DEBUG_TYPE_MISC: u32 = 4u32;
```

### `IMAGE_DEBUG_TYPE_EXCEPTION`

```rust
const IMAGE_DEBUG_TYPE_EXCEPTION: u32 = 5u32;
```

### `IMAGE_DEBUG_TYPE_FIXUP`

```rust
const IMAGE_DEBUG_TYPE_FIXUP: u32 = 6u32;
```

### `IMAGE_DEBUG_TYPE_OMAP_TO_SRC`

```rust
const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: u32 = 7u32;
```

### `IMAGE_DEBUG_TYPE_OMAP_FROM_SRC`

```rust
const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: u32 = 8u32;
```

### `IMAGE_DEBUG_TYPE_BORLAND`

```rust
const IMAGE_DEBUG_TYPE_BORLAND: u32 = 9u32;
```

### `IMAGE_DEBUG_TYPE_RESERVED10`

```rust
const IMAGE_DEBUG_TYPE_RESERVED10: u32 = 10u32;
```

### `IMAGE_DEBUG_TYPE_CLSID`

```rust
const IMAGE_DEBUG_TYPE_CLSID: u32 = 11u32;
```

### `IMAGE_DEBUG_TYPE_VC_FEATURE`

```rust
const IMAGE_DEBUG_TYPE_VC_FEATURE: u32 = 12u32;
```

### `IMAGE_DEBUG_TYPE_POGO`

```rust
const IMAGE_DEBUG_TYPE_POGO: u32 = 13u32;
```

### `IMAGE_DEBUG_TYPE_ILTCG`

```rust
const IMAGE_DEBUG_TYPE_ILTCG: u32 = 14u32;
```

### `IMAGE_DEBUG_TYPE_MPX`

```rust
const IMAGE_DEBUG_TYPE_MPX: u32 = 15u32;
```

### `IMAGE_DEBUG_TYPE_REPRO`

```rust
const IMAGE_DEBUG_TYPE_REPRO: u32 = 16u32;
```

### `FRAME_FPO`

```rust
const FRAME_FPO: u16 = 0u16;
```

### `FRAME_TRAP`

```rust
const FRAME_TRAP: u16 = 1u16;
```

### `FRAME_TSS`

```rust
const FRAME_TSS: u16 = 2u16;
```

### `FRAME_NONFPO`

```rust
const FRAME_NONFPO: u16 = 3u16;
```

### `IMAGE_DEBUG_MISC_EXENAME`

```rust
const IMAGE_DEBUG_MISC_EXENAME: u16 = 1u16;
```

### `IMAGE_SEPARATE_DEBUG_SIGNATURE`

```rust
const IMAGE_SEPARATE_DEBUG_SIGNATURE: u16 = 18_756u16;
```

### `NON_PAGED_DEBUG_SIGNATURE`

```rust
const NON_PAGED_DEBUG_SIGNATURE: u16 = 18_766u16;
```

### `IMAGE_SEPARATE_DEBUG_FLAGS_MASK`

```rust
const IMAGE_SEPARATE_DEBUG_FLAGS_MASK: u16 = 32_768u16;
```

### `IMAGE_SEPARATE_DEBUG_MISMATCH`

```rust
const IMAGE_SEPARATE_DEBUG_MISMATCH: u16 = 32_768u16;
```

when DBG was updated, the old checksum didn't match.

### `IMPORT_OBJECT_HDR_SIG2`

```rust
const IMPORT_OBJECT_HDR_SIG2: u16 = 65_535u16;
```

### `IMPORT_OBJECT_TYPE_MASK`

```rust
const IMPORT_OBJECT_TYPE_MASK: u16 = 3u16;
```

### `IMPORT_OBJECT_TYPE_SHIFT`

```rust
const IMPORT_OBJECT_TYPE_SHIFT: u16 = 0u16;
```

### `IMPORT_OBJECT_CODE`

```rust
const IMPORT_OBJECT_CODE: u16 = 0u16;
```

### `IMPORT_OBJECT_DATA`

```rust
const IMPORT_OBJECT_DATA: u16 = 1u16;
```

### `IMPORT_OBJECT_CONST`

```rust
const IMPORT_OBJECT_CONST: u16 = 2u16;
```

### `IMPORT_OBJECT_NAME_MASK`

```rust
const IMPORT_OBJECT_NAME_MASK: u16 = 7u16;
```

### `IMPORT_OBJECT_NAME_SHIFT`

```rust
const IMPORT_OBJECT_NAME_SHIFT: u16 = 2u16;
```

### `IMPORT_OBJECT_ORDINAL`

```rust
const IMPORT_OBJECT_ORDINAL: u16 = 0u16;
```

Import by ordinal

### `IMPORT_OBJECT_NAME`

```rust
const IMPORT_OBJECT_NAME: u16 = 1u16;
```

Import name == public symbol name.

### `IMPORT_OBJECT_NAME_NO_PREFIX`

```rust
const IMPORT_OBJECT_NAME_NO_PREFIX: u16 = 2u16;
```

Import name == public symbol name skipping leading ?, @, or optionally _.

### `IMPORT_OBJECT_NAME_UNDECORATE`

```rust
const IMPORT_OBJECT_NAME_UNDECORATE: u16 = 3u16;
```

Import name == public symbol name skipping leading ?, @, or optionally _ and truncating at first @.

### `IMPORT_OBJECT_NAME_EXPORTAS`

```rust
const IMPORT_OBJECT_NAME_EXPORTAS: u16 = 4u16;
```

Import name == a name is explicitly provided after the DLL name.

### `COMIMAGE_FLAGS_ILONLY`

```rust
const COMIMAGE_FLAGS_ILONLY: u32 = 1u32;
```

### `COMIMAGE_FLAGS_32BITREQUIRED`

```rust
const COMIMAGE_FLAGS_32BITREQUIRED: u32 = 2u32;
```

### `COMIMAGE_FLAGS_IL_LIBRARY`

```rust
const COMIMAGE_FLAGS_IL_LIBRARY: u32 = 4u32;
```

### `COMIMAGE_FLAGS_STRONGNAMESIGNED`

```rust
const COMIMAGE_FLAGS_STRONGNAMESIGNED: u32 = 8u32;
```

### `COMIMAGE_FLAGS_NATIVE_ENTRYPOINT`

```rust
const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT: u32 = 16u32;
```

### `COMIMAGE_FLAGS_TRACKDEBUGDATA`

```rust
const COMIMAGE_FLAGS_TRACKDEBUGDATA: u32 = 65_536u32;
```

### `COMIMAGE_FLAGS_32BITPREFERRED`

```rust
const COMIMAGE_FLAGS_32BITPREFERRED: u32 = 131_072u32;
```

### `COR_VERSION_MAJOR_V2`

```rust
const COR_VERSION_MAJOR_V2: u16 = 2u16;
```

### `COR_VERSION_MAJOR`

```rust
const COR_VERSION_MAJOR: u16 = 2u16;
```

### `COR_VERSION_MINOR`

```rust
const COR_VERSION_MINOR: u16 = 5u16;
```

### `COR_DELETED_NAME_LENGTH`

```rust
const COR_DELETED_NAME_LENGTH: usize = 8usize;
```

### `COR_VTABLEGAP_NAME_LENGTH`

```rust
const COR_VTABLEGAP_NAME_LENGTH: usize = 8usize;
```

### `NATIVE_TYPE_MAX_CB`

```rust
const NATIVE_TYPE_MAX_CB: u16 = 1u16;
```

### `COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE`

```rust
const COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE: u16 = 255u16;
```

### `IMAGE_COR_MIH_METHODRVA`

```rust
const IMAGE_COR_MIH_METHODRVA: u16 = 1u16;
```

### `IMAGE_COR_MIH_EHRVA`

```rust
const IMAGE_COR_MIH_EHRVA: u16 = 2u16;
```

### `IMAGE_COR_MIH_BASICBLOCK`

```rust
const IMAGE_COR_MIH_BASICBLOCK: u16 = 8u16;
```

### `COR_VTABLE_32BIT`

```rust
const COR_VTABLE_32BIT: u16 = 1u16;
```

V-table slots are 32-bits in size.

### `COR_VTABLE_64BIT`

```rust
const COR_VTABLE_64BIT: u16 = 2u16;
```

V-table slots are 64-bits in size.

### `COR_VTABLE_FROM_UNMANAGED`

```rust
const COR_VTABLE_FROM_UNMANAGED: u16 = 4u16;
```

If set, transition from unmanaged.

### `COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN`

```rust
const COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN: u16 = 8u16;
```

If set, transition from unmanaged with keeping the current appdomain.

### `COR_VTABLE_CALL_MOST_DERIVED`

```rust
const COR_VTABLE_CALL_MOST_DERIVED: u16 = 16u16;
```

Call most derived method described by

### `IMAGE_COR_EATJ_THUNK_SIZE`

```rust
const IMAGE_COR_EATJ_THUNK_SIZE: usize = 32usize;
```

Size of a jump thunk reserved range.

### `MAX_CLASS_NAME`

```rust
const MAX_CLASS_NAME: usize = 1_024usize;
```

### `MAX_PACKAGE_NAME`

```rust
const MAX_PACKAGE_NAME: usize = 1_024usize;
```

