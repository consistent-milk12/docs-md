*[object](../index.md) / [pod](index.md)*

---

# Module `pod`

Tools for converting file format structures to and from bytes.

This module should be replaced once rust provides safe transmutes.

## Contents

- [Traits](#traits)
  - [`Pod`](#pod)
- [Functions](#functions)
  - [`from_bytes`](#from-bytes)
  - [`from_bytes_mut`](#from-bytes-mut)
  - [`slice_from_bytes`](#slice-from-bytes)
  - [`slice_from_bytes_mut`](#slice-from-bytes-mut)
  - [`slice_from_all_bytes`](#slice-from-all-bytes)
  - [`slice_from_all_bytes_mut`](#slice-from-all-bytes-mut)
  - [`bytes_of`](#bytes-of)
  - [`bytes_of_mut`](#bytes-of-mut)
  - [`bytes_of_slice`](#bytes-of-slice)
  - [`bytes_of_slice_mut`](#bytes-of-slice-mut)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Macros](#macros)
  - [`unsafe_impl_pod!`](#unsafe-impl-pod)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pod`](#pod) | trait | A trait for types that can safely be converted from and to byte slices. |
| [`from_bytes`](#from-bytes) | fn | Cast the head of a byte slice to a `Pod` type. |
| [`from_bytes_mut`](#from-bytes-mut) | fn | Cast the head of a mutable byte slice to a `Pod` type. |
| [`slice_from_bytes`](#slice-from-bytes) | fn | Cast the head of a byte slice to a slice of a `Pod` type. |
| [`slice_from_bytes_mut`](#slice-from-bytes-mut) | fn | Cast the head of a mutable byte slice to a slice of a `Pod` type. |
| [`slice_from_all_bytes`](#slice-from-all-bytes) | fn | Cast all of a byte slice to a slice of a `Pod` type. |
| [`slice_from_all_bytes_mut`](#slice-from-all-bytes-mut) | fn | Cast all of a byte slice to a slice of a `Pod` type. |
| [`bytes_of`](#bytes-of) | fn | Cast a `Pod` type to a byte slice. |
| [`bytes_of_mut`](#bytes-of-mut) | fn | Cast a `Pod` type to a mutable byte slice. |
| [`bytes_of_slice`](#bytes-of-slice) | fn | Cast a slice of a `Pod` type to a byte slice. |
| [`bytes_of_slice_mut`](#bytes-of-slice-mut) | fn | Cast a slice of a `Pod` type to a mutable byte slice. |
| [`Result`](#result) | type |  |
| [`unsafe_impl_pod!`](#unsafe-impl-pod) | macro |  |

## Traits

### `Pod`

```rust
trait Pod: Copy + 'static { ... }
```

*Defined in [`object-0.37.3/src/pod.rs:22`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L22)*

A trait for types that can safely be converted from and to byte slices.

# Safety
A type that is `Pod` must:
- be `#[repr(C)]` or `#[repr(transparent)]`
- have no invalid byte values
- have no padding

#### Implementors

- [`AixFileHeader`](../archive/index.md#aixfileheader)
- [`AixHeader`](../archive/index.md#aixheader)
- [`AixMemberOffset`](../archive/index.md#aixmemberoffset)
- [`AnonObjectHeaderBigobj`](../pe/index.md#anonobjectheaderbigobj)
- [`AnonObjectHeaderV2`](../pe/index.md#anonobjectheaderv2)
- [`AnonObjectHeader`](../pe/index.md#anonobjectheader)
- [`AuxHeader32`](../xcoff/index.md#auxheader32)
- [`AuxHeader64`](../xcoff/index.md#auxheader64)
- [`BlockAux32`](../xcoff/index.md#blockaux32)
- [`BlockAux64`](../xcoff/index.md#blockaux64)
- [`BuildToolVersion`](../macho/index.md#buildtoolversion)
- [`BuildVersionCommand`](../macho/index.md#buildversioncommand)
- [`CompressionHeader32`](../elf/index.md#compressionheader32)
- [`CompressionHeader64`](../elf/index.md#compressionheader64)
- [`CsectAux32`](../xcoff/index.md#csectaux32)
- [`CsectAux64`](../xcoff/index.md#csectaux64)
- [`DataInCodeEntry`](../macho/index.md#dataincodeentry)
- [`DwarfAux32`](../xcoff/index.md#dwarfaux32)
- [`DwarfAux64`](../xcoff/index.md#dwarfaux64)
- [`DyldCacheHeader`](../macho/index.md#dyldcacheheader)
- [`DyldCacheImageInfo`](../macho/index.md#dyldcacheimageinfo)
- [`DyldCacheMappingAndSlideInfo`](../macho/index.md#dyldcachemappingandslideinfo)
- [`DyldCacheMappingInfo`](../macho/index.md#dyldcachemappinginfo)
- [`DyldCacheSlideInfo2`](../macho/index.md#dyldcacheslideinfo2)
- [`DyldCacheSlideInfo3`](../macho/index.md#dyldcacheslideinfo3)
- [`DyldCacheSlideInfo5`](../macho/index.md#dyldcacheslideinfo5)
- [`DyldInfoCommand`](../macho/index.md#dyldinfocommand)
- [`DyldSubCacheEntryV1`](../macho/index.md#dyldsubcacheentryv1)
- [`DyldSubCacheEntryV2`](../macho/index.md#dyldsubcacheentryv2)
- [`DylibCommand`](../macho/index.md#dylibcommand)
- [`DylibModule32`](../macho/index.md#dylibmodule32)
- [`DylibModule64`](../macho/index.md#dylibmodule64)
- [`DylibReference`](../macho/index.md#dylibreference)
- [`DylibTableOfContents`](../macho/index.md#dylibtableofcontents)
- [`Dylib`](../macho/index.md#dylib)
- [`DylinkerCommand`](../macho/index.md#dylinkercommand)
- [`Dyn32`](../elf/index.md#dyn32)
- [`Dyn64`](../elf/index.md#dyn64)
- [`DysymtabCommand`](../macho/index.md#dysymtabcommand)
- [`EncryptionInfoCommand32`](../macho/index.md#encryptioninfocommand32)
- [`EncryptionInfoCommand64`](../macho/index.md#encryptioninfocommand64)
- [`EntryPointCommand`](../macho/index.md#entrypointcommand)
- [`ExpAux`](../xcoff/index.md#expaux)
- [`FatArch32`](../macho/index.md#fatarch32)
- [`FatArch64`](../macho/index.md#fatarch64)
- [`FatHeader`](../macho/index.md#fatheader)
- [`FileAux32`](../xcoff/index.md#fileaux32)
- [`FileAux64`](../xcoff/index.md#fileaux64)
- [`FileHeader32`](../elf/index.md#fileheader32)
- [`FileHeader32`](../xcoff/index.md#fileheader32)
- [`FileHeader64`](../elf/index.md#fileheader64)
- [`FileHeader64`](../xcoff/index.md#fileheader64)
- [`FilesetEntryCommand`](../macho/index.md#filesetentrycommand)
- [`FunAux32`](../xcoff/index.md#funaux32)
- [`FunAux64`](../xcoff/index.md#funaux64)
- [`FvmfileCommand`](../macho/index.md#fvmfilecommand)
- [`FvmlibCommand`](../macho/index.md#fvmlibcommand)
- [`Fvmlib`](../macho/index.md#fvmlib)
- [`GnuHashHeader`](../elf/index.md#gnuhashheader)
- [`Guid`](../pe/index.md#guid)
- [`HashHeader`](../elf/index.md#hashheader)
- [`Header`](../archive/index.md#header)
- [`I16Bytes`](../index.md#i16bytes)
- [`I32Bytes`](../index.md#i32bytes)
- [`I64Bytes`](../index.md#i64bytes)
- [`IdentCommand`](../macho/index.md#identcommand)
- [`ImageAlpha64RuntimeFunctionEntry`](../pe/index.md#imagealpha64runtimefunctionentry)
- [`ImageAlphaRuntimeFunctionEntry`](../pe/index.md#imagealpharuntimefunctionentry)
- [`ImageArchitectureEntry`](../pe/index.md#imagearchitectureentry)
- [`ImageArchiveMemberHeader`](../pe/index.md#imagearchivememberheader)
- [`ImageArm64RuntimeFunctionEntry`](../pe/index.md#imagearm64runtimefunctionentry)
- [`ImageArmRuntimeFunctionEntry`](../pe/index.md#imagearmruntimefunctionentry)
- [`ImageAuxSymbolCrc`](../pe/index.md#imageauxsymbolcrc)
- [`ImageAuxSymbolFunctionBeginEnd`](../pe/index.md#imageauxsymbolfunctionbeginend)
- [`ImageAuxSymbolFunction`](../pe/index.md#imageauxsymbolfunction)
- [`ImageAuxSymbolSection`](../pe/index.md#imageauxsymbolsection)
- [`ImageAuxSymbolTokenDef`](../pe/index.md#imageauxsymboltokendef)
- [`ImageAuxSymbolWeak`](../pe/index.md#imageauxsymbolweak)
- [`ImageBaseRelocation`](../pe/index.md#imagebaserelocation)
- [`ImageBoundForwarderRef`](../pe/index.md#imageboundforwarderref)
- [`ImageBoundImportDescriptor`](../pe/index.md#imageboundimportdescriptor)
- [`ImageCoffSymbolsHeader`](../pe/index.md#imagecoffsymbolsheader)
- [`ImageCor20Header`](../pe/index.md#imagecor20header)
- [`ImageDataDirectory`](../pe/index.md#imagedatadirectory)
- [`ImageDebugDirectory`](../pe/index.md#imagedebugdirectory)
- [`ImageDebugMisc`](../pe/index.md#imagedebugmisc)
- [`ImageDelayloadDescriptor`](../pe/index.md#imagedelayloaddescriptor)
- [`ImageDosHeader`](../pe/index.md#imagedosheader)
- [`ImageDynamicRelocation32V2`](../pe/index.md#imagedynamicrelocation32v2)
- [`ImageDynamicRelocation32`](../pe/index.md#imagedynamicrelocation32)
- [`ImageDynamicRelocation64V2`](../pe/index.md#imagedynamicrelocation64v2)
- [`ImageDynamicRelocation64`](../pe/index.md#imagedynamicrelocation64)
- [`ImageDynamicRelocationTable`](../pe/index.md#imagedynamicrelocationtable)
- [`ImageEnclaveConfig32`](../pe/index.md#imageenclaveconfig32)
- [`ImageEnclaveConfig64`](../pe/index.md#imageenclaveconfig64)
- [`ImageEnclaveImport`](../pe/index.md#imageenclaveimport)
- [`ImageEpilogueDynamicRelocationHeader`](../pe/index.md#imageepiloguedynamicrelocationheader)
- [`ImageExportDirectory`](../pe/index.md#imageexportdirectory)
- [`ImageFileHeader`](../pe/index.md#imagefileheader)
- [`ImageFunctionEntry64`](../pe/index.md#imagefunctionentry64)
- [`ImageFunctionEntry`](../pe/index.md#imagefunctionentry)
- [`ImageHotPatchBase`](../pe/index.md#imagehotpatchbase)
- [`ImageHotPatchHashes`](../pe/index.md#imagehotpatchhashes)
- [`ImageHotPatchInfo`](../pe/index.md#imagehotpatchinfo)
- [`ImageImportByName`](../pe/index.md#imageimportbyname)
- [`ImageImportDescriptor`](../pe/index.md#imageimportdescriptor)
- [`ImageLinenumber`](../pe/index.md#imagelinenumber)
- [`ImageLoadConfigCodeIntegrity`](../pe/index.md#imageloadconfigcodeintegrity)
- [`ImageLoadConfigDirectory32`](../pe/index.md#imageloadconfigdirectory32)
- [`ImageLoadConfigDirectory64`](../pe/index.md#imageloadconfigdirectory64)
- [`ImageNtHeaders32`](../pe/index.md#imagentheaders32)
- [`ImageNtHeaders64`](../pe/index.md#imagentheaders64)
- [`ImageOptionalHeader32`](../pe/index.md#imageoptionalheader32)
- [`ImageOptionalHeader64`](../pe/index.md#imageoptionalheader64)
- [`ImageOs2Header`](../pe/index.md#imageos2header)
- [`ImagePrologueDynamicRelocationHeader`](../pe/index.md#imageprologuedynamicrelocationheader)
- [`ImageRelocation`](../pe/index.md#imagerelocation)
- [`ImageResourceDataEntry`](../pe/index.md#imageresourcedataentry)
- [`ImageResourceDirStringU`](../pe/index.md#imageresourcedirstringu)
- [`ImageResourceDirectoryEntry`](../pe/index.md#imageresourcedirectoryentry)
- [`ImageResourceDirectoryString`](../pe/index.md#imageresourcedirectorystring)
- [`ImageResourceDirectory`](../pe/index.md#imageresourcedirectory)
- [`ImageRomHeaders`](../pe/index.md#imageromheaders)
- [`ImageRomOptionalHeader`](../pe/index.md#imageromoptionalheader)
- [`ImageRuntimeFunctionEntry`](../pe/index.md#imageruntimefunctionentry)
- [`ImageSectionHeader`](../pe/index.md#imagesectionheader)
- [`ImageSeparateDebugHeader`](../pe/index.md#imageseparatedebugheader)
- [`ImageSymbolBytes`](../pe/index.md#imagesymbolbytes)
- [`ImageSymbolExBytes`](../pe/index.md#imagesymbolexbytes)
- [`ImageSymbolEx`](../pe/index.md#imagesymbolex)
- [`ImageSymbol`](../pe/index.md#imagesymbol)
- [`ImageThunkData32`](../pe/index.md#imagethunkdata32)
- [`ImageThunkData64`](../pe/index.md#imagethunkdata64)
- [`ImageTlsDirectory32`](../pe/index.md#imagetlsdirectory32)
- [`ImageTlsDirectory64`](../pe/index.md#imagetlsdirectory64)
- [`ImageVxdHeader`](../pe/index.md#imagevxdheader)
- [`ImportObjectHeader`](../pe/index.md#importobjectheader)
- [`LcStr`](../macho/index.md#lcstr)
- [`LinkeditDataCommand`](../macho/index.md#linkeditdatacommand)
- [`LinkerOptionCommand`](../macho/index.md#linkeroptioncommand)
- [`LoadCommand`](../macho/index.md#loadcommand)
- [`MachHeader32`](../macho/index.md#machheader32)
- [`MachHeader64`](../macho/index.md#machheader64)
- [`MaskedRichHeaderEntry`](../pe/index.md#maskedrichheaderentry)
- [`Nlist32`](../macho/index.md#nlist32)
- [`Nlist64`](../macho/index.md#nlist64)
- [`NonPagedDebugInfo`](../pe/index.md#nonpageddebuginfo)
- [`NoteCommand`](../macho/index.md#notecommand)
- [`NoteHeader32`](../elf/index.md#noteheader32)
- [`NoteHeader64`](../elf/index.md#noteheader64)
- [`PrebindCksumCommand`](../macho/index.md#prebindcksumcommand)
- [`PreboundDylibCommand`](../macho/index.md#prebounddylibcommand)
- [`ProgramHeader32`](../elf/index.md#programheader32)
- [`ProgramHeader64`](../elf/index.md#programheader64)
- [`Rel32`](../elf/index.md#rel32)
- [`Rel32`](../xcoff/index.md#rel32)
- [`Rel64`](../elf/index.md#rel64)
- [`Rel64`](../xcoff/index.md#rel64)
- [`Rela32`](../elf/index.md#rela32)
- [`Rela64`](../elf/index.md#rela64)
- [`Relocation`](../macho/index.md#relocation)
- [`Relr32`](../elf/index.md#relr32)
- [`Relr64`](../elf/index.md#relr64)
- [`RoutinesCommand32`](../macho/index.md#routinescommand32)
- [`RoutinesCommand64`](../macho/index.md#routinescommand64)
- [`RpathCommand`](../macho/index.md#rpathcommand)
- [`Section32`](../macho/index.md#section32)
- [`Section64`](../macho/index.md#section64)
- [`SectionHeader32`](../elf/index.md#sectionheader32)
- [`SectionHeader32`](../xcoff/index.md#sectionheader32)
- [`SectionHeader64`](../elf/index.md#sectionheader64)
- [`SectionHeader64`](../xcoff/index.md#sectionheader64)
- [`SegmentCommand32`](../macho/index.md#segmentcommand32)
- [`SegmentCommand64`](../macho/index.md#segmentcommand64)
- [`SourceVersionCommand`](../macho/index.md#sourceversioncommand)
- [`StatAux`](../xcoff/index.md#stataux)
- [`SubClientCommand`](../macho/index.md#subclientcommand)
- [`SubFrameworkCommand`](../macho/index.md#subframeworkcommand)
- [`SubLibraryCommand`](../macho/index.md#sublibrarycommand)
- [`SubUmbrellaCommand`](../macho/index.md#subumbrellacommand)
- [`Sym32`](../elf/index.md#sym32)
- [`Sym64`](../elf/index.md#sym64)
- [`Symbol32`](../xcoff/index.md#symbol32)
- [`Symbol64`](../xcoff/index.md#symbol64)
- [`SymbolBytes`](../xcoff/index.md#symbolbytes)
- [`Syminfo32`](../elf/index.md#syminfo32)
- [`Syminfo64`](../elf/index.md#syminfo64)
- [`SymsegCommand`](../macho/index.md#symsegcommand)
- [`SymtabCommand`](../macho/index.md#symtabcommand)
- [`ThreadCommand`](../macho/index.md#threadcommand)
- [`TwolevelHint`](../macho/index.md#twolevelhint)
- [`TwolevelHintsCommand`](../macho/index.md#twolevelhintscommand)
- [`U16Bytes`](../index.md#u16bytes)
- [`U32Bytes`](../index.md#u32bytes)
- [`U64Bytes`](../index.md#u64bytes)
- [`UuidCommand`](../macho/index.md#uuidcommand)
- [`Verdaux`](../elf/index.md#verdaux)
- [`Verdef`](../elf/index.md#verdef)
- [`Vernaux`](../elf/index.md#vernaux)
- [`Verneed`](../elf/index.md#verneed)
- [`VersionMinCommand`](../macho/index.md#versionmincommand)
- [`Versym`](../elf/index.md#versym)
- `[T; N]`
- `u16`
- `u32`
- `u64`
- `u8`

## Functions

### `from_bytes`

```rust
fn from_bytes<T: Pod>(data: &[u8]) -> result::Result<(&T, &[u8]), ()>
```

*Defined in [`object-0.37.3/src/pod.rs:30-42`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L30-L42)*

Cast the head of a byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `from_bytes_mut`

```rust
fn from_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<(&mut T, &mut [u8]), ()>
```

*Defined in [`object-0.37.3/src/pod.rs:50-65`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L50-L65)*

Cast the head of a mutable byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes`

```rust
fn slice_from_bytes<T: Pod>(data: &[u8], count: usize) -> result::Result<(&[T], &[u8]), ()>
```

*Defined in [`object-0.37.3/src/pod.rs:73-85`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L73-L85)*

Cast the head of a byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes_mut`

```rust
fn slice_from_bytes_mut<T: Pod>(data: &mut [u8], count: usize) -> result::Result<(&mut [T], &mut [u8]), ()>
```

*Defined in [`object-0.37.3/src/pod.rs:93-111`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L93-L111)*

Cast the head of a mutable byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_all_bytes`

```rust
fn slice_from_all_bytes<T: Pod>(data: &[u8]) -> result::Result<&[T], ()>
```

*Defined in [`object-0.37.3/src/pod.rs:120-127`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L120-L127)*

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `slice_from_all_bytes_mut`

```rust
fn slice_from_all_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<&mut [T], ()>
```

*Defined in [`object-0.37.3/src/pod.rs:136-143`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L136-L143)*

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `bytes_of`

```rust
fn bytes_of<T: Pod>(val: &T) -> &[u8]
```

*Defined in [`object-0.37.3/src/pod.rs:147-154`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L147-L154)*

Cast a `Pod` type to a byte slice.

### `bytes_of_mut`

```rust
fn bytes_of_mut<T: Pod>(val: &mut T) -> &mut [u8]
```

*Defined in [`object-0.37.3/src/pod.rs:158-165`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L158-L165)*

Cast a `Pod` type to a mutable byte slice.

### `bytes_of_slice`

```rust
fn bytes_of_slice<T: Pod>(val: &[T]) -> &[u8]
```

*Defined in [`object-0.37.3/src/pod.rs:169-176`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L169-L176)*

Cast a slice of a `Pod` type to a byte slice.

### `bytes_of_slice_mut`

```rust
fn bytes_of_slice_mut<T: Pod>(val: &mut [T]) -> &mut [u8]
```

*Defined in [`object-0.37.3/src/pod.rs:180-187`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L180-L187)*

Cast a slice of a `Pod` type to a mutable byte slice.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, ()>;
```

*Defined in [`object-0.37.3/src/pod.rs:13`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L13)*

## Macros

### `unsafe_impl_pod!`

*Defined in [`object-0.37.3/src/pod.rs:189-195`](../../../.source_1765521767/object-0.37.3/src/pod.rs#L189-L195)*

