*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [load_command](index.md)*

---

# Module `load_command`

## Structs

### `LoadCommandIterator<'data, E: Endian>`

```rust
struct LoadCommandIterator<'data, E: Endian> {
    endian: E,
    data: crate::read::Bytes<'data>,
    ncmds: u32,
}
```

An iterator for the load commands from a [`MachHeader`](../index.md).

#### Implementations

- `fn new(endian: E, data: &'data [u8], ncmds: u32) -> Self`

- `fn next(self: &mut Self) -> Result<Option<LoadCommandData<'data, E>>>` — [`Result`](../../../index.md), [`LoadCommandData`](../index.md)

- `fn parse(self: &mut Self) -> Result<LoadCommandData<'data, E>>` — [`Result`](../../../index.md), [`LoadCommandData`](../index.md)

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for LoadCommandIterator<'data, E>`

- `fn clone(self: &Self) -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](../index.md)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for LoadCommandIterator<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for LoadCommandIterator<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, E: $crate::default::Default + Endian> Default for LoadCommandIterator<'data, E>`

- `fn default() -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](../index.md)

##### `impl<I> IntoIterator for LoadCommandIterator<'data, E>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, E: Endian> Iterator for LoadCommandIterator<'data, E>`

- `type Item = Result<LoadCommandData<'data, E>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `LoadCommandData<'data, E: Endian>`

```rust
struct LoadCommandData<'data, E: Endian> {
    cmd: u32,
    data: crate::read::Bytes<'data>,
    marker: core::marker::PhantomData<E>,
}
```

The data for a [`macho::LoadCommand`](../../../macho/index.md).

#### Implementations

- `fn cmd(self: &Self) -> u32`

- `fn cmdsize(self: &Self) -> u32`

- `fn data<T: Pod>(self: &Self) -> Result<&'data T>` — [`Result`](../../../index.md)

- `fn raw_data(self: &Self) -> &'data [u8]`

- `fn string(self: &Self, endian: E, s: macho::LcStr<E>) -> Result<&'data [u8]>` — [`LcStr`](../../../macho/index.md), [`Result`](../../../index.md)

- `fn variant(self: &Self) -> Result<LoadCommandVariant<'data, E>>` — [`Result`](../../../index.md), [`LoadCommandVariant`](../index.md)

- `fn segment_32(self: Self) -> Result<Option<(&'data macho::SegmentCommand32<E>, &'data [u8])>>` — [`Result`](../../../index.md), [`SegmentCommand32`](../../../macho/index.md)

- `fn symtab(self: Self) -> Result<Option<&'data macho::SymtabCommand<E>>>` — [`Result`](../../../index.md), [`SymtabCommand`](../../../macho/index.md)

- `fn dysymtab(self: Self) -> Result<Option<&'data macho::DysymtabCommand<E>>>` — [`Result`](../../../index.md), [`DysymtabCommand`](../../../macho/index.md)

- `fn dylib(self: Self) -> Result<Option<&'data macho::DylibCommand<E>>>` — [`Result`](../../../index.md), [`DylibCommand`](../../../macho/index.md)

- `fn uuid(self: Self) -> Result<Option<&'data macho::UuidCommand<E>>>` — [`Result`](../../../index.md), [`UuidCommand`](../../../macho/index.md)

- `fn segment_64(self: Self) -> Result<Option<(&'data macho::SegmentCommand64<E>, &'data [u8])>>` — [`Result`](../../../index.md), [`SegmentCommand64`](../../../macho/index.md)

- `fn dyld_info(self: Self) -> Result<Option<&'data macho::DyldInfoCommand<E>>>` — [`Result`](../../../index.md), [`DyldInfoCommand`](../../../macho/index.md)

- `fn entry_point(self: Self) -> Result<Option<&'data macho::EntryPointCommand<E>>>` — [`Result`](../../../index.md), [`EntryPointCommand`](../../../macho/index.md)

- `fn build_version(self: Self) -> Result<Option<&'data macho::BuildVersionCommand<E>>>` — [`Result`](../../../index.md), [`BuildVersionCommand`](../../../macho/index.md)

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for LoadCommandData<'data, E>`

- `fn clone(self: &Self) -> LoadCommandData<'data, E>` — [`LoadCommandData`](../index.md)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for LoadCommandData<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for LoadCommandData<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for LoadCommandVariant<'data, E>`

- `fn clone(self: &Self) -> LoadCommandVariant<'data, E>` — [`LoadCommandVariant`](../index.md)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for LoadCommandVariant<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for LoadCommandVariant<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

