*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [import](index.md)*

---

# Module `import`

## Contents

- [Structs](#structs)
  - [`ImportTable`](#importtable)
  - [`ImportDescriptorIterator`](#importdescriptoriterator)
  - [`ImportThunkList`](#importthunklist)
  - [`DelayLoadImportTable`](#delayloadimporttable)
  - [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)
- [Enums](#enums)
  - [`Import`](#import)
- [Traits](#traits)
  - [`ImageThunkData`](#imagethunkdata)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImportTable`](#importtable) | struct | Information for parsing a PE import table. |
| [`ImportDescriptorIterator`](#importdescriptoriterator) | struct | A fallible iterator for the descriptors in the import data directory. |
| [`ImportThunkList`](#importthunklist) | struct | A list of import thunks. |
| [`DelayLoadImportTable`](#delayloadimporttable) | struct | Information for parsing a PE delay-load import table. |
| [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator) | struct | A fallible iterator for the descriptors in the delay-load data directory. |
| [`Import`](#import) | enum | A parsed import thunk. |
| [`ImageThunkData`](#imagethunkdata) | trait | A trait for generic access to [`pe::ImageThunkData32`] and [`pe::ImageThunkData64`]. |

## Structs

### `ImportTable<'data>`

```rust
struct ImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:15-19`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/import.rs#L15-L19)*

Information for parsing a PE import table.

Returned by [`DataDirectories::import_table`](super::DataDirectories::import_table).

#### Implementations

- <span id="importtable-new"></span>`fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- <span id="importtable-descriptors"></span>`fn descriptors(&self) -> Result<ImportDescriptorIterator<'data>>` — [`Result`](../../../index.md#result), [`ImportDescriptorIterator`](../index.md#importdescriptoriterator)

- <span id="importtable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="importtable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../../index.md#result), [`ImportThunkList`](../index.md#importthunklist)

- <span id="importtable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](../index.md#imagentheaders), [`Result`](../../../index.md#result), [`Import`](../index.md#import)

- <span id="importtable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Clone for ImportTable<'data>`

- <span id="importtable-clone"></span>`fn clone(&self) -> ImportTable<'data>` — [`ImportTable`](../index.md#importtable)

##### `impl Debug for ImportTable<'data>`

- <span id="importtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportDescriptorIterator<'data>`

```rust
struct ImportDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:102-105`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/import.rs#L102-L105)*

A fallible iterator for the descriptors in the import data directory.

#### Implementations

- <span id="importdescriptoriterator-next"></span>`fn next(&mut self) -> Result<Option<&'data pe::ImageImportDescriptor>>` — [`Result`](../../../index.md#result), [`ImageImportDescriptor`](../../../pe/index.md#imageimportdescriptor)

#### Trait Implementations

##### `impl Clone for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-clone"></span>`fn clone(&self) -> ImportDescriptorIterator<'data>` — [`ImportDescriptorIterator`](../index.md#importdescriptoriterator)

##### `impl Debug for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="importdescriptoriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="importdescriptoriterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-iterator-type-item"></span>`type Item = Result<&'data ImageImportDescriptor, Error>`

- <span id="importdescriptoriterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ImportThunkList<'data>`

```rust
struct ImportThunkList<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:148-150`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/import.rs#L148-L150)*

A list of import thunks.

These may be in the import lookup table, or the import address table.

#### Implementations

- <span id="importthunklist-get"></span>`fn get<Pe: ImageNtHeaders>(&self, index: usize) -> Result<<Pe as >::ImageThunkData>` — [`Result`](../../../index.md#result), [`ImageNtHeaders`](../index.md#imagentheaders)

- <span id="importthunklist-next"></span>`fn next<Pe: ImageNtHeaders>(&mut self) -> Result<Option<<Pe as >::ImageThunkData>>` — [`Result`](../../../index.md#result), [`ImageNtHeaders`](../index.md#imagentheaders)

#### Trait Implementations

##### `impl Clone for ImportThunkList<'data>`

- <span id="importthunklist-clone"></span>`fn clone(&self) -> ImportThunkList<'data>` — [`ImportThunkList`](../index.md#importthunklist)

##### `impl Debug for ImportThunkList<'data>`

- <span id="importthunklist-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DelayLoadImportTable<'data>`

```rust
struct DelayLoadImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:250-254`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/import.rs#L250-L254)*

Information for parsing a PE delay-load import table.

Returned by
[`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table).

#### Implementations

- <span id="delayloadimporttable-new"></span>`fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- <span id="delayloadimporttable-descriptors"></span>`fn descriptors(&self) -> Result<DelayLoadDescriptorIterator<'data>>` — [`Result`](../../../index.md#result), [`DelayLoadDescriptorIterator`](../index.md#delayloaddescriptoriterator)

- <span id="delayloadimporttable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="delayloadimporttable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../../index.md#result), [`ImportThunkList`](../index.md#importthunklist)

- <span id="delayloadimporttable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](../index.md#imagentheaders), [`Result`](../../../index.md#result), [`Import`](../index.md#import)

- <span id="delayloadimporttable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Clone for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-clone"></span>`fn clone(&self) -> DelayLoadImportTable<'data>` — [`DelayLoadImportTable`](../index.md#delayloadimporttable)

##### `impl Debug for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DelayLoadDescriptorIterator<'data>`

```rust
struct DelayLoadDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:341-344`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/import.rs#L341-L344)*

A fallible iterator for the descriptors in the delay-load data directory.

#### Implementations

- <span id="delayloaddescriptoriterator-next"></span>`fn next(&mut self) -> Result<Option<&'data pe::ImageDelayloadDescriptor>>` — [`Result`](../../../index.md#result), [`ImageDelayloadDescriptor`](../../../pe/index.md#imagedelayloaddescriptor)

#### Trait Implementations

##### `impl Clone for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-clone"></span>`fn clone(&self) -> DelayLoadDescriptorIterator<'data>` — [`DelayLoadDescriptorIterator`](../index.md#delayloaddescriptoriterator)

##### `impl Debug for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="delayloaddescriptoriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="delayloaddescriptoriterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-iterator-type-item"></span>`type Item = Result<&'data ImageDelayloadDescriptor, Error>`

- <span id="delayloaddescriptoriterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `Import<'data>`

```rust
enum Import<'data> {
    Ordinal(u16),
    Name(u16, &'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:180-187`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/import.rs#L180-L187)*

A parsed import thunk.

#### Variants

- **`Ordinal`**

  Import by ordinal.

- **`Name`**

  Import by name.
  
  Includes a hint for the index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl Clone for Import<'data>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'data>` — [`Import`](../index.md#import)

##### `impl Copy for Import<'data>`

##### `impl Debug for Import<'data>`

- <span id="import-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `ImageThunkData`

```rust
trait ImageThunkData: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:191-207`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/import.rs#L191-L207)*

A trait for generic access to [`pe::ImageThunkData32`](../../../pe/index.md) and [`pe::ImageThunkData64`](../../../pe/index.md).

#### Required Methods

- `fn raw(self) -> u64`

  Return the raw thunk value.

- `fn is_ordinal(self) -> bool`

  Returns true if the ordinal flag is set.

- `fn ordinal(self) -> u16`

  Return the ordinal portion of the thunk.

- `fn address(self) -> u32`

  Return the RVA portion of the thunk.

#### Implementors

- [`ImageThunkData32`](../../../pe/index.md#imagethunkdata32)
- [`ImageThunkData64`](../../../pe/index.md#imagethunkdata64)

