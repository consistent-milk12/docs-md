*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [load_command](index.md)*

---

# Module `load_command`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LoadCommandIterator`](#loadcommanditerator) | struct | An iterator for the load commands from a [`MachHeader`]. |
| [`LoadCommandData`](#loadcommanddata) | struct | The data for a [`macho::LoadCommand`]. |
| [`LoadCommandVariant`](#loadcommandvariant) | enum | A [`macho::LoadCommand`] that has been interpreted according to its `cmd` field. |

## Structs

### `LoadCommandIterator<'data, E: Endian>`

```rust
struct LoadCommandIterator<'data, E: Endian> {
    endian: E,
    data: crate::read::Bytes<'data>,
    ncmds: u32,
}
```

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:12-16`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/load_command.rs#L12-L16)*

An iterator for the load commands from a [`MachHeader`](../index.md).

#### Implementations

- <span id="loadcommanditerator-new"></span>`fn new(endian: E, data: &'data [u8], ncmds: u32) -> Self`

- <span id="loadcommanditerator-next"></span>`fn next(&mut self) -> Result<Option<LoadCommandData<'data, E>>>` — [`Result`](../../../index.md#result), [`LoadCommandData`](../index.md#loadcommanddata)

- <span id="loadcommanditerator-parse"></span>`fn parse(&mut self) -> Result<LoadCommandData<'data, E>>` — [`Result`](../../../index.md#result), [`LoadCommandData`](../index.md#loadcommanddata)

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-clone"></span>`fn clone(&self) -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](../index.md#loadcommanditerator)

##### `impl<E: marker::Copy + Endian> Copy for LoadCommandIterator<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-default"></span>`fn default() -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](../index.md#loadcommanditerator)

##### `impl IntoIterator for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="loadcommanditerator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="loadcommanditerator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E: Endian> Iterator for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-iterator-type-item"></span>`type Item = Result<LoadCommandData<'data, E>, Error>`

- <span id="loadcommanditerator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LoadCommandData<'data, E: Endian>`

```rust
struct LoadCommandData<'data, E: Endian> {
    cmd: u32,
    data: crate::read::Bytes<'data>,
    marker: core::marker::PhantomData<E>,
}
```

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:74-79`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/load_command.rs#L74-L79)*

The data for a [`macho::LoadCommand`](../../../macho/index.md).

#### Implementations

- <span id="loadcommanddata-cmd"></span>`fn cmd(&self) -> u32`

- <span id="loadcommanddata-cmdsize"></span>`fn cmdsize(&self) -> u32`

- <span id="loadcommanddata-data"></span>`fn data<T: Pod>(&self) -> Result<&'data T>` — [`Result`](../../../index.md#result)

- <span id="loadcommanddata-raw-data"></span>`fn raw_data(&self) -> &'data [u8]`

- <span id="loadcommanddata-string"></span>`fn string(&self, endian: E, s: macho::LcStr<E>) -> Result<&'data [u8]>` — [`LcStr`](../../../macho/index.md#lcstr), [`Result`](../../../index.md#result)

- <span id="loadcommanddata-variant"></span>`fn variant(&self) -> Result<LoadCommandVariant<'data, E>>` — [`Result`](../../../index.md#result), [`LoadCommandVariant`](../index.md#loadcommandvariant)

- <span id="loadcommanddata-segment-32"></span>`fn segment_32(self) -> Result<Option<(&'data macho::SegmentCommand32<E>, &'data [u8])>>` — [`Result`](../../../index.md#result), [`SegmentCommand32`](../../../macho/index.md#segmentcommand32)

- <span id="loadcommanddata-symtab"></span>`fn symtab(self) -> Result<Option<&'data macho::SymtabCommand<E>>>` — [`Result`](../../../index.md#result), [`SymtabCommand`](../../../macho/index.md#symtabcommand)

- <span id="loadcommanddata-dysymtab"></span>`fn dysymtab(self) -> Result<Option<&'data macho::DysymtabCommand<E>>>` — [`Result`](../../../index.md#result), [`DysymtabCommand`](../../../macho/index.md#dysymtabcommand)

- <span id="loadcommanddata-dylib"></span>`fn dylib(self) -> Result<Option<&'data macho::DylibCommand<E>>>` — [`Result`](../../../index.md#result), [`DylibCommand`](../../../macho/index.md#dylibcommand)

- <span id="loadcommanddata-uuid"></span>`fn uuid(self) -> Result<Option<&'data macho::UuidCommand<E>>>` — [`Result`](../../../index.md#result), [`UuidCommand`](../../../macho/index.md#uuidcommand)

- <span id="loadcommanddata-segment-64"></span>`fn segment_64(self) -> Result<Option<(&'data macho::SegmentCommand64<E>, &'data [u8])>>` — [`Result`](../../../index.md#result), [`SegmentCommand64`](../../../macho/index.md#segmentcommand64)

- <span id="loadcommanddata-dyld-info"></span>`fn dyld_info(self) -> Result<Option<&'data macho::DyldInfoCommand<E>>>` — [`Result`](../../../index.md#result), [`DyldInfoCommand`](../../../macho/index.md#dyldinfocommand)

- <span id="loadcommanddata-entry-point"></span>`fn entry_point(self) -> Result<Option<&'data macho::EntryPointCommand<E>>>` — [`Result`](../../../index.md#result), [`EntryPointCommand`](../../../macho/index.md#entrypointcommand)

- <span id="loadcommanddata-build-version"></span>`fn build_version(self) -> Result<Option<&'data macho::BuildVersionCommand<E>>>` — [`Result`](../../../index.md#result), [`BuildVersionCommand`](../../../macho/index.md#buildversioncommand)

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for LoadCommandData<'data, E>`

- <span id="loadcommanddata-clone"></span>`fn clone(&self) -> LoadCommandData<'data, E>` — [`LoadCommandData`](../index.md#loadcommanddata)

##### `impl<E: marker::Copy + Endian> Copy for LoadCommandData<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for LoadCommandData<'data, E>`

- <span id="loadcommanddata-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `LoadCommandVariant<'data, E: Endian>`

```rust
enum LoadCommandVariant<'data, E: Endian> {
    Segment32(&'data macho::SegmentCommand32<E>, &'data [u8]),
    Symtab(&'data macho::SymtabCommand<E>),
    Thread(&'data macho::ThreadCommand<E>, &'data [u8]),
    Dysymtab(&'data macho::DysymtabCommand<E>),
    Dylib(&'data macho::DylibCommand<E>),
    IdDylib(&'data macho::DylibCommand<E>),
    LoadDylinker(&'data macho::DylinkerCommand<E>),
    IdDylinker(&'data macho::DylinkerCommand<E>),
    PreboundDylib(&'data macho::PreboundDylibCommand<E>),
    Routines32(&'data macho::RoutinesCommand32<E>),
    SubFramework(&'data macho::SubFrameworkCommand<E>),
    SubUmbrella(&'data macho::SubUmbrellaCommand<E>),
    SubClient(&'data macho::SubClientCommand<E>),
    SubLibrary(&'data macho::SubLibraryCommand<E>),
    TwolevelHints(&'data macho::TwolevelHintsCommand<E>),
    PrebindCksum(&'data macho::PrebindCksumCommand<E>),
    Segment64(&'data macho::SegmentCommand64<E>, &'data [u8]),
    Routines64(&'data macho::RoutinesCommand64<E>),
    Uuid(&'data macho::UuidCommand<E>),
    Rpath(&'data macho::RpathCommand<E>),
    LinkeditData(&'data macho::LinkeditDataCommand<E>),
    EncryptionInfo32(&'data macho::EncryptionInfoCommand32<E>),
    DyldInfo(&'data macho::DyldInfoCommand<E>),
    VersionMin(&'data macho::VersionMinCommand<E>),
    DyldEnvironment(&'data macho::DylinkerCommand<E>),
    EntryPoint(&'data macho::EntryPointCommand<E>),
    SourceVersion(&'data macho::SourceVersionCommand<E>),
    EncryptionInfo64(&'data macho::EncryptionInfoCommand64<E>),
    LinkerOption(&'data macho::LinkerOptionCommand<E>),
    Note(&'data macho::NoteCommand<E>),
    BuildVersion(&'data macho::BuildVersionCommand<E>),
    FilesetEntry(&'data macho::FilesetEntryCommand<E>),
    Other,
}
```

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:280-360`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/load_command.rs#L280-L360)*

A [`macho::LoadCommand`](../../../macho/index.md) that has been interpreted according to its `cmd` field.

#### Variants

- **`Segment32`**

  `LC_SEGMENT`

- **`Symtab`**

  `LC_SYMTAB`

- **`Thread`**

  `LC_THREAD` or `LC_UNIXTHREAD`

- **`Dysymtab`**

  `LC_DYSYMTAB`

- **`Dylib`**

  `LC_LOAD_DYLIB`, `LC_LOAD_WEAK_DYLIB`, `LC_REEXPORT_DYLIB`,
  `LC_LAZY_LOAD_DYLIB`, or `LC_LOAD_UPWARD_DYLIB`

- **`IdDylib`**

  `LC_ID_DYLIB`

- **`LoadDylinker`**

  `LC_LOAD_DYLINKER`

- **`IdDylinker`**

  `LC_ID_DYLINKER`

- **`PreboundDylib`**

  `LC_PREBOUND_DYLIB`

- **`Routines32`**

  `LC_ROUTINES`

- **`SubFramework`**

  `LC_SUB_FRAMEWORK`

- **`SubUmbrella`**

  `LC_SUB_UMBRELLA`

- **`SubClient`**

  `LC_SUB_CLIENT`

- **`SubLibrary`**

  `LC_SUB_LIBRARY`

- **`TwolevelHints`**

  `LC_TWOLEVEL_HINTS`

- **`PrebindCksum`**

  `LC_PREBIND_CKSUM`

- **`Segment64`**

  `LC_SEGMENT_64`

- **`Routines64`**

  `LC_ROUTINES_64`

- **`Uuid`**

  `LC_UUID`

- **`Rpath`**

  `LC_RPATH`

- **`LinkeditData`**

  `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,
  `LC_DATA_IN_CODE`, `LC_DYLIB_CODE_SIGN_DRS`, `LC_LINKER_OPTIMIZATION_HINT`,
  `LC_DYLD_EXPORTS_TRIE`, or `LC_DYLD_CHAINED_FIXUPS`.

- **`EncryptionInfo32`**

  `LC_ENCRYPTION_INFO`

- **`DyldInfo`**

  `LC_DYLD_INFO` or `LC_DYLD_INFO_ONLY`

- **`VersionMin`**

  `LC_VERSION_MIN_MACOSX`, `LC_VERSION_MIN_IPHONEOS`, `LC_VERSION_MIN_WATCHOS`,
  or `LC_VERSION_MIN_TVOS`

- **`DyldEnvironment`**

  `LC_DYLD_ENVIRONMENT`

- **`EntryPoint`**

  `LC_MAIN`

- **`SourceVersion`**

  `LC_SOURCE_VERSION`

- **`EncryptionInfo64`**

  `LC_ENCRYPTION_INFO_64`

- **`LinkerOption`**

  `LC_LINKER_OPTION`

- **`Note`**

  `LC_NOTE`

- **`BuildVersion`**

  `LC_BUILD_VERSION`

- **`FilesetEntry`**

  `LC_FILESET_ENTRY`

- **`Other`**

  An unrecognized or obsolete load command.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-clone"></span>`fn clone(&self) -> LoadCommandVariant<'data, E>` — [`LoadCommandVariant`](../index.md#loadcommandvariant)

##### `impl<E: marker::Copy + Endian> Copy for LoadCommandVariant<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

