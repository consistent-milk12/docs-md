*[object](../index.md) / [pod](index.md)*

---

# Module `pod`

Tools for converting file format structures to and from bytes.

This module should be replaced once rust provides safe transmutes.

## Contents

- [Traits](#traits)
  - [`Pod`](#pod)
- [Functions](#functions)
  - [`from_bytes`](#from_bytes)
  - [`from_bytes_mut`](#from_bytes_mut)
  - [`slice_from_bytes`](#slice_from_bytes)
  - [`slice_from_bytes_mut`](#slice_from_bytes_mut)
  - [`slice_from_all_bytes`](#slice_from_all_bytes)
  - [`slice_from_all_bytes_mut`](#slice_from_all_bytes_mut)
  - [`bytes_of`](#bytes_of)
  - [`bytes_of_mut`](#bytes_of_mut)
  - [`bytes_of_slice`](#bytes_of_slice)
  - [`bytes_of_slice_mut`](#bytes_of_slice_mut)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Macros](#macros)
  - [`unsafe_impl_pod!`](#unsafe_impl_pod)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pod`](#pod) | trait | A trait for types that can safely be converted from and to byte slices. |
| [`from_bytes`](#from_bytes) | fn | Cast the head of a byte slice to a `Pod` type. |
| [`from_bytes_mut`](#from_bytes_mut) | fn | Cast the head of a mutable byte slice to a `Pod` type. |
| [`slice_from_bytes`](#slice_from_bytes) | fn | Cast the head of a byte slice to a slice of a `Pod` type. |
| [`slice_from_bytes_mut`](#slice_from_bytes_mut) | fn | Cast the head of a mutable byte slice to a slice of a `Pod` type. |
| [`slice_from_all_bytes`](#slice_from_all_bytes) | fn | Cast all of a byte slice to a slice of a `Pod` type. |
| [`slice_from_all_bytes_mut`](#slice_from_all_bytes_mut) | fn | Cast all of a byte slice to a slice of a `Pod` type. |
| [`bytes_of`](#bytes_of) | fn | Cast a `Pod` type to a byte slice. |
| [`bytes_of_mut`](#bytes_of_mut) | fn | Cast a `Pod` type to a mutable byte slice. |
| [`bytes_of_slice`](#bytes_of_slice) | fn | Cast a slice of a `Pod` type to a byte slice. |
| [`bytes_of_slice_mut`](#bytes_of_slice_mut) | fn | Cast a slice of a `Pod` type to a mutable byte slice. |
| [`Result`](#result) | type |  |
| [`unsafe_impl_pod!`](#unsafe_impl_pod) | macro |  |

## Traits

### `Pod`

```rust
trait Pod: Copy + 'static { ... }
```

*Defined in [`object-0.37.3/src/pod.rs:22`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L22)*

A trait for types that can safely be converted from and to byte slices.

# Safety
A type that is `Pod` must:
- be `#[repr(C)]` or `#[repr(transparent)]`
- have no invalid byte values
- have no padding

#### Implementors

- [`AixFileHeader`](../archive/index.md)
- [`AixHeader`](../archive/index.md)
- [`AixMemberOffset`](../archive/index.md)
- [`AnonObjectHeaderBigobj`](../pe/index.md)
- [`AnonObjectHeaderV2`](../pe/index.md)
- [`AnonObjectHeader`](../pe/index.md)
- [`AuxHeader32`](../xcoff/index.md)
- [`AuxHeader64`](../xcoff/index.md)
- [`BlockAux32`](../xcoff/index.md)
- [`BlockAux64`](../xcoff/index.md)
- [`BuildToolVersion`](../macho/index.md)
- [`BuildVersionCommand`](../macho/index.md)
- [`CompressionHeader32`](../elf/index.md)
- [`CompressionHeader64`](../elf/index.md)
- [`CsectAux32`](../xcoff/index.md)
- [`CsectAux64`](../xcoff/index.md)
- [`DataInCodeEntry`](../macho/index.md)
- [`DwarfAux32`](../xcoff/index.md)
- [`DwarfAux64`](../xcoff/index.md)
- [`DyldCacheHeader`](../macho/index.md)
- [`DyldCacheImageInfo`](../macho/index.md)
- [`DyldCacheMappingAndSlideInfo`](../macho/index.md)
- [`DyldCacheMappingInfo`](../macho/index.md)
- [`DyldCacheSlideInfo2`](../macho/index.md)
- [`DyldCacheSlideInfo3`](../macho/index.md)
- [`DyldCacheSlideInfo5`](../macho/index.md)
- [`DyldInfoCommand`](../macho/index.md)
- [`DyldSubCacheEntryV1`](../macho/index.md)
- [`DyldSubCacheEntryV2`](../macho/index.md)
- [`DylibCommand`](../macho/index.md)
- [`DylibModule32`](../macho/index.md)
- [`DylibModule64`](../macho/index.md)
- [`DylibReference`](../macho/index.md)
- [`DylibTableOfContents`](../macho/index.md)
- [`Dylib`](../macho/index.md)
- [`DylinkerCommand`](../macho/index.md)
- [`Dyn32`](../elf/index.md)
- [`Dyn64`](../elf/index.md)
- [`DysymtabCommand`](../macho/index.md)
- [`EncryptionInfoCommand32`](../macho/index.md)
- [`EncryptionInfoCommand64`](../macho/index.md)
- [`EntryPointCommand`](../macho/index.md)
- [`ExpAux`](../xcoff/index.md)
- [`FatArch32`](../macho/index.md)
- [`FatArch64`](../macho/index.md)
- [`FatHeader`](../macho/index.md)
- [`FileAux32`](../xcoff/index.md)
- [`FileAux64`](../xcoff/index.md)
- [`FileHeader32`](../elf/index.md)
- [`FileHeader32`](../xcoff/index.md)
- [`FileHeader64`](../elf/index.md)
- [`FileHeader64`](../xcoff/index.md)
- [`FilesetEntryCommand`](../macho/index.md)
- [`FunAux32`](../xcoff/index.md)
- [`FunAux64`](../xcoff/index.md)
- [`FvmfileCommand`](../macho/index.md)
- [`FvmlibCommand`](../macho/index.md)
- [`Fvmlib`](../macho/index.md)
- [`GnuHashHeader`](../elf/index.md)
- [`Guid`](../pe/index.md)
- [`HashHeader`](../elf/index.md)
- [`Header`](../archive/index.md)
- [`I16Bytes`](../index.md)
- [`I32Bytes`](../index.md)
- [`I64Bytes`](../index.md)
- [`IdentCommand`](../macho/index.md)
- [`ImageAlpha64RuntimeFunctionEntry`](../pe/index.md)
- [`ImageAlphaRuntimeFunctionEntry`](../pe/index.md)
- [`ImageArchitectureEntry`](../pe/index.md)
- [`ImageArchiveMemberHeader`](../pe/index.md)
- [`ImageArm64RuntimeFunctionEntry`](../pe/index.md)
- [`ImageArmRuntimeFunctionEntry`](../pe/index.md)
- [`ImageAuxSymbolCrc`](../pe/index.md)
- [`ImageAuxSymbolFunctionBeginEnd`](../pe/index.md)
- [`ImageAuxSymbolFunction`](../pe/index.md)
- [`ImageAuxSymbolSection`](../pe/index.md)
- [`ImageAuxSymbolTokenDef`](../pe/index.md)
- [`ImageAuxSymbolWeak`](../pe/index.md)
- [`ImageBaseRelocation`](../pe/index.md)
- [`ImageBoundForwarderRef`](../pe/index.md)
- [`ImageBoundImportDescriptor`](../pe/index.md)
- [`ImageCoffSymbolsHeader`](../pe/index.md)
- [`ImageCor20Header`](../pe/index.md)
- [`ImageDataDirectory`](../pe/index.md)
- [`ImageDebugDirectory`](../pe/index.md)
- [`ImageDebugMisc`](../pe/index.md)
- [`ImageDelayloadDescriptor`](../pe/index.md)
- [`ImageDosHeader`](../pe/index.md)
- [`ImageDynamicRelocation32V2`](../pe/index.md)
- [`ImageDynamicRelocation32`](../pe/index.md)
- [`ImageDynamicRelocation64V2`](../pe/index.md)
- [`ImageDynamicRelocation64`](../pe/index.md)
- [`ImageDynamicRelocationTable`](../pe/index.md)
- [`ImageEnclaveConfig32`](../pe/index.md)
- [`ImageEnclaveConfig64`](../pe/index.md)
- [`ImageEnclaveImport`](../pe/index.md)
- [`ImageEpilogueDynamicRelocationHeader`](../pe/index.md)
- [`ImageExportDirectory`](../pe/index.md)
- [`ImageFileHeader`](../pe/index.md)
- [`ImageFunctionEntry64`](../pe/index.md)
- [`ImageFunctionEntry`](../pe/index.md)
- [`ImageHotPatchBase`](../pe/index.md)
- [`ImageHotPatchHashes`](../pe/index.md)
- [`ImageHotPatchInfo`](../pe/index.md)
- [`ImageImportByName`](../pe/index.md)
- [`ImageImportDescriptor`](../pe/index.md)
- [`ImageLinenumber`](../pe/index.md)
- [`ImageLoadConfigCodeIntegrity`](../pe/index.md)
- [`ImageLoadConfigDirectory32`](../pe/index.md)
- [`ImageLoadConfigDirectory64`](../pe/index.md)
- [`ImageNtHeaders32`](../pe/index.md)
- [`ImageNtHeaders64`](../pe/index.md)
- [`ImageOptionalHeader32`](../pe/index.md)
- [`ImageOptionalHeader64`](../pe/index.md)
- [`ImageOs2Header`](../pe/index.md)
- [`ImagePrologueDynamicRelocationHeader`](../pe/index.md)
- [`ImageRelocation`](../pe/index.md)
- [`ImageResourceDataEntry`](../pe/index.md)
- [`ImageResourceDirStringU`](../pe/index.md)
- [`ImageResourceDirectoryEntry`](../pe/index.md)
- [`ImageResourceDirectoryString`](../pe/index.md)
- [`ImageResourceDirectory`](../pe/index.md)
- [`ImageRomHeaders`](../pe/index.md)
- [`ImageRomOptionalHeader`](../pe/index.md)
- [`ImageRuntimeFunctionEntry`](../pe/index.md)
- [`ImageSectionHeader`](../pe/index.md)
- [`ImageSeparateDebugHeader`](../pe/index.md)
- [`ImageSymbolBytes`](../pe/index.md)
- [`ImageSymbolExBytes`](../pe/index.md)
- [`ImageSymbolEx`](../pe/index.md)
- [`ImageSymbol`](../pe/index.md)
- [`ImageThunkData32`](../pe/index.md)
- [`ImageThunkData64`](../pe/index.md)
- [`ImageTlsDirectory32`](../pe/index.md)
- [`ImageTlsDirectory64`](../pe/index.md)
- [`ImageVxdHeader`](../pe/index.md)
- [`ImportObjectHeader`](../pe/index.md)
- [`LcStr`](../macho/index.md)
- [`LinkeditDataCommand`](../macho/index.md)
- [`LinkerOptionCommand`](../macho/index.md)
- [`LoadCommand`](../macho/index.md)
- [`MachHeader32`](../macho/index.md)
- [`MachHeader64`](../macho/index.md)
- [`MaskedRichHeaderEntry`](../pe/index.md)
- [`Nlist32`](../macho/index.md)
- [`Nlist64`](../macho/index.md)
- [`NonPagedDebugInfo`](../pe/index.md)
- [`NoteCommand`](../macho/index.md)
- [`NoteHeader32`](../elf/index.md)
- [`NoteHeader64`](../elf/index.md)
- [`PrebindCksumCommand`](../macho/index.md)
- [`PreboundDylibCommand`](../macho/index.md)
- [`ProgramHeader32`](../elf/index.md)
- [`ProgramHeader64`](../elf/index.md)
- [`Rel32`](../elf/index.md)
- [`Rel32`](../xcoff/index.md)
- [`Rel64`](../elf/index.md)
- [`Rel64`](../xcoff/index.md)
- [`Rela32`](../elf/index.md)
- [`Rela64`](../elf/index.md)
- [`Relocation`](../macho/index.md)
- [`Relr32`](../elf/index.md)
- [`Relr64`](../elf/index.md)
- [`RoutinesCommand32`](../macho/index.md)
- [`RoutinesCommand64`](../macho/index.md)
- [`RpathCommand`](../macho/index.md)
- [`Section32`](../macho/index.md)
- [`Section64`](../macho/index.md)
- [`SectionHeader32`](../elf/index.md)
- [`SectionHeader32`](../xcoff/index.md)
- [`SectionHeader64`](../elf/index.md)
- [`SectionHeader64`](../xcoff/index.md)
- [`SegmentCommand32`](../macho/index.md)
- [`SegmentCommand64`](../macho/index.md)
- [`SourceVersionCommand`](../macho/index.md)
- [`StatAux`](../xcoff/index.md)
- [`SubClientCommand`](../macho/index.md)
- [`SubFrameworkCommand`](../macho/index.md)
- [`SubLibraryCommand`](../macho/index.md)
- [`SubUmbrellaCommand`](../macho/index.md)
- [`Sym32`](../elf/index.md)
- [`Sym64`](../elf/index.md)
- [`Symbol32`](../xcoff/index.md)
- [`Symbol64`](../xcoff/index.md)
- [`SymbolBytes`](../xcoff/index.md)
- [`Syminfo32`](../elf/index.md)
- [`Syminfo64`](../elf/index.md)
- [`SymsegCommand`](../macho/index.md)
- [`SymtabCommand`](../macho/index.md)
- [`ThreadCommand`](../macho/index.md)
- [`TwolevelHint`](../macho/index.md)
- [`TwolevelHintsCommand`](../macho/index.md)
- [`U16Bytes`](../index.md)
- [`U32Bytes`](../index.md)
- [`U64Bytes`](../index.md)
- [`UuidCommand`](../macho/index.md)
- [`Verdaux`](../elf/index.md)
- [`Verdef`](../elf/index.md)
- [`Vernaux`](../elf/index.md)
- [`Verneed`](../elf/index.md)
- [`VersionMinCommand`](../macho/index.md)
- [`Versym`](../elf/index.md)
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

*Defined in [`object-0.37.3/src/pod.rs:30-42`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L30-L42)*

Cast the head of a byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `from_bytes_mut`

```rust
fn from_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<(&mut T, &mut [u8]), ()>
```

*Defined in [`object-0.37.3/src/pod.rs:50-65`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L50-L65)*

Cast the head of a mutable byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes`

```rust
fn slice_from_bytes<T: Pod>(data: &[u8], count: usize) -> result::Result<(&[T], &[u8]), ()>
```

*Defined in [`object-0.37.3/src/pod.rs:73-85`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L73-L85)*

Cast the head of a byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes_mut`

```rust
fn slice_from_bytes_mut<T: Pod>(data: &mut [u8], count: usize) -> result::Result<(&mut [T], &mut [u8]), ()>
```

*Defined in [`object-0.37.3/src/pod.rs:93-111`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L93-L111)*

Cast the head of a mutable byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_all_bytes`

```rust
fn slice_from_all_bytes<T: Pod>(data: &[u8]) -> result::Result<&[T], ()>
```

*Defined in [`object-0.37.3/src/pod.rs:120-127`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L120-L127)*

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `slice_from_all_bytes_mut`

```rust
fn slice_from_all_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<&mut [T], ()>
```

*Defined in [`object-0.37.3/src/pod.rs:136-143`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L136-L143)*

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `bytes_of`

```rust
fn bytes_of<T: Pod>(val: &T) -> &[u8]
```

*Defined in [`object-0.37.3/src/pod.rs:147-154`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L147-L154)*

Cast a `Pod` type to a byte slice.

### `bytes_of_mut`

```rust
fn bytes_of_mut<T: Pod>(val: &mut T) -> &mut [u8]
```

*Defined in [`object-0.37.3/src/pod.rs:158-165`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L158-L165)*

Cast a `Pod` type to a mutable byte slice.

### `bytes_of_slice`

```rust
fn bytes_of_slice<T: Pod>(val: &[T]) -> &[u8]
```

*Defined in [`object-0.37.3/src/pod.rs:169-176`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L169-L176)*

Cast a slice of a `Pod` type to a byte slice.

### `bytes_of_slice_mut`

```rust
fn bytes_of_slice_mut<T: Pod>(val: &mut [T]) -> &mut [u8]
```

*Defined in [`object-0.37.3/src/pod.rs:180-187`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L180-L187)*

Cast a slice of a `Pod` type to a mutable byte slice.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, ()>;
```

*Defined in [`object-0.37.3/src/pod.rs:13`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L13)*

## Macros

### `unsafe_impl_pod!`

*Defined in [`object-0.37.3/src/pod.rs:189-195`](../../../.source_1765210505/object-0.37.3/src/pod.rs#L189-L195)*

