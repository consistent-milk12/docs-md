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

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:12-16`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/load_command.rs#L12-L16)*

An iterator for the load commands from a [`MachHeader`](../index.md).

#### Implementations

- <span id="loadcommanditerator-new"></span>`fn new(endian: E, data: &'data [u8], ncmds: u32) -> Self`

- <span id="loadcommanditerator-next"></span>`fn next(&mut self) -> Result<Option<LoadCommandData<'data, E>>>` — [`Result`](../../../index.md#result), [`LoadCommandData`](../index.md#loadcommanddata)

  Return the next load command.

- <span id="loadcommanditerator-parse"></span>`fn parse(&mut self) -> Result<LoadCommandData<'data, E>>` — [`Result`](../../../index.md#result), [`LoadCommandData`](../index.md#loadcommanddata)

#### Trait Implementations

##### `impl Any for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-clone"></span>`fn clone(&self) -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](../index.md#loadcommanditerator)

##### `impl CloneToUninit for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for LoadCommandIterator<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-default"></span>`fn default() -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](../index.md#loadcommanditerator)

##### `impl<T> From for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="loadcommanditerator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="loadcommanditerator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E: Endian> Iterator for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-iterator-type-item"></span>`type Item = Result<LoadCommandData<'data, E>, Error>`

- <span id="loadcommanditerator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-toowned-type-owned"></span>`type Owned = T`

- <span id="loadcommanditerator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="loadcommanditerator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loadcommanditerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loadcommanditerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LoadCommandData<'data, E: Endian>`

```rust
struct LoadCommandData<'data, E: Endian> {
    cmd: u32,
    data: crate::read::Bytes<'data>,
    marker: core::marker::PhantomData<E>,
}
```

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:74-79`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/load_command.rs#L74-L79)*

The data for a [`macho::LoadCommand`](../../../macho/index.md).

#### Implementations

- <span id="loadcommanddata-cmd"></span>`fn cmd(&self) -> u32`

  Return the `cmd` field of the [`macho::LoadCommand`](../../../macho/index.md).

  

  This is one of the `LC_` constants.

- <span id="loadcommanddata-cmdsize"></span>`fn cmdsize(&self) -> u32`

  Return the `cmdsize` field of the [`macho::LoadCommand`](../../../macho/index.md).

- <span id="loadcommanddata-data"></span>`fn data<T: Pod>(&self) -> Result<&'data T>` — [`Result`](../../../index.md#result)

  Parse the data as the given type.

- <span id="loadcommanddata-raw-data"></span>`fn raw_data(&self) -> &'data [u8]`

  Raw bytes of this [`macho::LoadCommand`](../../../macho/index.md) structure.

- <span id="loadcommanddata-string"></span>`fn string(&self, endian: E, s: macho::LcStr<E>) -> Result<&'data [u8]>` — [`LcStr`](../../../macho/index.md#lcstr), [`Result`](../../../index.md#result)

  Parse a load command string value.

  

  Strings used by load commands are specified by offsets that are

  relative to the load command header.

- <span id="loadcommanddata-variant"></span>`fn variant(&self) -> Result<LoadCommandVariant<'data, E>>` — [`Result`](../../../index.md#result), [`LoadCommandVariant`](../index.md#loadcommandvariant)

  Parse the command data according to the `cmd` field.

- <span id="loadcommanddata-segment-32"></span>`fn segment_32(self) -> Result<Option<(&'data macho::SegmentCommand32<E>, &'data [u8])>>` — [`Result`](../../../index.md#result), [`SegmentCommand32`](../../../macho/index.md#segmentcommand32)

  Try to parse this command as a [`macho::SegmentCommand32`](../../../macho/index.md).

  

  Returns the segment command and the data containing the sections.

- <span id="loadcommanddata-symtab"></span>`fn symtab(self) -> Result<Option<&'data macho::SymtabCommand<E>>>` — [`Result`](../../../index.md#result), [`SymtabCommand`](../../../macho/index.md#symtabcommand)

  Try to parse this command as a [`macho::SymtabCommand`](../../../macho/index.md).

- <span id="loadcommanddata-dysymtab"></span>`fn dysymtab(self) -> Result<Option<&'data macho::DysymtabCommand<E>>>` — [`Result`](../../../index.md#result), [`DysymtabCommand`](../../../macho/index.md#dysymtabcommand)

  Try to parse this command as a [`macho::DysymtabCommand`](../../../macho/index.md).

- <span id="loadcommanddata-dylib"></span>`fn dylib(self) -> Result<Option<&'data macho::DylibCommand<E>>>` — [`Result`](../../../index.md#result), [`DylibCommand`](../../../macho/index.md#dylibcommand)

  Try to parse this command as a [`macho::DylibCommand`](../../../macho/index.md).

- <span id="loadcommanddata-uuid"></span>`fn uuid(self) -> Result<Option<&'data macho::UuidCommand<E>>>` — [`Result`](../../../index.md#result), [`UuidCommand`](../../../macho/index.md#uuidcommand)

  Try to parse this command as a [`macho::UuidCommand`](../../../macho/index.md).

- <span id="loadcommanddata-segment-64"></span>`fn segment_64(self) -> Result<Option<(&'data macho::SegmentCommand64<E>, &'data [u8])>>` — [`Result`](../../../index.md#result), [`SegmentCommand64`](../../../macho/index.md#segmentcommand64)

  Try to parse this command as a [`macho::SegmentCommand64`](../../../macho/index.md).

- <span id="loadcommanddata-dyld-info"></span>`fn dyld_info(self) -> Result<Option<&'data macho::DyldInfoCommand<E>>>` — [`Result`](../../../index.md#result), [`DyldInfoCommand`](../../../macho/index.md#dyldinfocommand)

  Try to parse this command as a [`macho::DyldInfoCommand`](../../../macho/index.md).

- <span id="loadcommanddata-entry-point"></span>`fn entry_point(self) -> Result<Option<&'data macho::EntryPointCommand<E>>>` — [`Result`](../../../index.md#result), [`EntryPointCommand`](../../../macho/index.md#entrypointcommand)

  Try to parse this command as an [`macho::EntryPointCommand`](../../../macho/index.md).

- <span id="loadcommanddata-build-version"></span>`fn build_version(self) -> Result<Option<&'data macho::BuildVersionCommand<E>>>` — [`Result`](../../../index.md#result), [`BuildVersionCommand`](../../../macho/index.md#buildversioncommand)

  Try to parse this command as a [`macho::BuildVersionCommand`](../../../macho/index.md).

#### Trait Implementations

##### `impl Any for LoadCommandData<'data, E>`

- <span id="loadcommanddata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LoadCommandData<'data, E>`

- <span id="loadcommanddata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LoadCommandData<'data, E>`

- <span id="loadcommanddata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for LoadCommandData<'data, E>`

- <span id="loadcommanddata-clone"></span>`fn clone(&self) -> LoadCommandData<'data, E>` — [`LoadCommandData`](../index.md#loadcommanddata)

##### `impl CloneToUninit for LoadCommandData<'data, E>`

- <span id="loadcommanddata-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for LoadCommandData<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for LoadCommandData<'data, E>`

- <span id="loadcommanddata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LoadCommandData<'data, E>`

- <span id="loadcommanddata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LoadCommandData<'data, E>`

- <span id="loadcommanddata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LoadCommandData<'data, E>`

- <span id="loadcommanddata-toowned-type-owned"></span>`type Owned = T`

- <span id="loadcommanddata-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="loadcommanddata-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LoadCommandData<'data, E>`

- <span id="loadcommanddata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loadcommanddata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LoadCommandData<'data, E>`

- <span id="loadcommanddata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loadcommanddata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:280-360`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/load_command.rs#L280-L360)*

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

##### `impl Any for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-clone"></span>`fn clone(&self) -> LoadCommandVariant<'data, E>` — [`LoadCommandVariant`](../index.md#loadcommandvariant)

##### `impl CloneToUninit for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for LoadCommandVariant<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-toowned-type-owned"></span>`type Owned = T`

- <span id="loadcommandvariant-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="loadcommandvariant-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loadcommandvariant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loadcommandvariant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

