*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [export](index.md)*

---

# Module `export`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Export`](#export) | struct | An export from a PE file. |
| [`ExportTable`](#exporttable) | struct | A partially parsed PE export table. |
| [`ExportTarget`](#exporttarget) | enum | Where an export is pointing to. |
| [`parse_ordinal`](#parse_ordinal) | fn |  |

## Structs

### `Export<'data>`

```rust
struct Export<'data> {
    pub ordinal: u32,
    pub name: Option<&'data [u8]>,
    pub target: ExportTarget<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:42-51`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L42-L51)*

An export from a PE file.

There are multiple kinds of PE exports (with or without a name, and local or forwarded).

#### Fields

- **`ordinal`**: `u32`

  The ordinal of the export.
  
  These are sequential, starting at a base specified in the DLL.

- **`name`**: `Option<&'data [u8]>`

  The name of the export, if known.

- **`target`**: `ExportTarget<'data>`

  The target of this export.

#### Trait Implementations

##### `impl Clone for Export<'data>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'data>` — [`Export`](../index.md#export)

##### `impl Copy for Export<'data>`

##### `impl Debug for Export<'a>`

- <span id="export-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

### `ExportTable<'data>`

```rust
struct ExportTable<'data> {
    data: crate::read::Bytes<'data>,
    virtual_address: u32,
    directory: &'data pe::ImageExportDirectory,
    addresses: &'data [crate::endian::U32Bytes<crate::endian::LittleEndian>],
    names: &'data [crate::endian::U32Bytes<crate::endian::LittleEndian>],
    name_ordinals: &'data [crate::endian::U16Bytes<crate::endian::LittleEndian>],
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:87-94`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L87-L94)*

A partially parsed PE export table.

Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table).

#### Implementations

- <span id="exporttable-parse"></span>`fn parse(data: &'data [u8], virtual_address: u32) -> Result<Self>` — [`Result`](../../../index.md#result)

- <span id="exporttable-parse-directory"></span>`fn parse_directory(data: &'data [u8]) -> Result<&'data pe::ImageExportDirectory>` — [`Result`](../../../index.md#result), [`ImageExportDirectory`](../../../pe/index.md#imageexportdirectory)

- <span id="exporttable-directory"></span>`fn directory(&self) -> &'data pe::ImageExportDirectory` — [`ImageExportDirectory`](../../../pe/index.md#imageexportdirectory)

- <span id="exporttable-ordinal-base"></span>`fn ordinal_base(&self) -> u32`

- <span id="exporttable-addresses"></span>`fn addresses(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../../index.md#u32bytes), [`LittleEndian`](../../../index.md#littleendian)

- <span id="exporttable-name-pointers"></span>`fn name_pointers(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../../index.md#u32bytes), [`LittleEndian`](../../../index.md#littleendian)

- <span id="exporttable-name-ordinals"></span>`fn name_ordinals(&self) -> &'data [U16Bytes<LE>]` — [`U16Bytes`](../../../index.md#u16bytes), [`LittleEndian`](../../../index.md#littleendian)

- <span id="exporttable-name-iter"></span>`fn name_iter(&self) -> impl Iterator<Item = (u32, u16)> + 'data`

- <span id="exporttable-address-by-index"></span>`fn address_by_index(&self, index: u32) -> Result<u32>` — [`Result`](../../../index.md#result)

- <span id="exporttable-address-by-ordinal"></span>`fn address_by_ordinal(&self, ordinal: u32) -> Result<u32>` — [`Result`](../../../index.md#result)

- <span id="exporttable-target-by-index"></span>`fn target_by_index(&self, index: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md#result), [`ExportTarget`](../index.md#exporttarget)

- <span id="exporttable-target-by-ordinal"></span>`fn target_by_ordinal(&self, ordinal: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md#result), [`ExportTarget`](../index.md#exporttarget)

- <span id="exporttable-target-from-address"></span>`fn target_from_address(&self, address: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md#result), [`ExportTarget`](../index.md#exporttarget)

- <span id="exporttable-forward-offset"></span>`fn forward_offset(&self, address: u32) -> Option<usize>`

- <span id="exporttable-is-forward"></span>`fn is_forward(&self, address: u32) -> bool`

- <span id="exporttable-forward-string"></span>`fn forward_string(&self, address: u32) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="exporttable-name-from-pointer"></span>`fn name_from_pointer(&self, name_pointer: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="exporttable-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md#result), [`Export`](../index.md#export)

#### Trait Implementations

##### `impl Clone for ExportTable<'data>`

- <span id="exporttable-clone"></span>`fn clone(&self) -> ExportTable<'data>` — [`ExportTable`](../index.md#exporttable)

##### `impl Debug for ExportTable<'data>`

- <span id="exporttable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ExportTarget<'data>`

```rust
enum ExportTarget<'data> {
    Address(u32),
    ForwardByOrdinal(&'data [u8], u32),
    ForwardByName(&'data [u8], &'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:10-21`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L10-L21)*

Where an export is pointing to.

#### Variants

- **`Address`**

  The address of the export, relative to the image base.

- **`ForwardByOrdinal`**

  Forwarded to an export ordinal in another DLL.
  
  This gives the name of the DLL, and the ordinal.

- **`ForwardByName`**

  Forwarded to an export name in another DLL.
  
  This gives the name of the DLL, and the export name.

#### Implementations

- <span id="exporttarget-is-address"></span>`fn is_address(&self) -> bool`

- <span id="exporttarget-is-forward"></span>`fn is_forward(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ExportTarget<'data>`

- <span id="exporttarget-clone"></span>`fn clone(&self) -> ExportTarget<'data>` — [`ExportTarget`](../index.md#exporttarget)

##### `impl Copy for ExportTarget<'data>`

##### `impl Debug for ExportTarget<'a>`

- <span id="exporttarget-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

## Functions

### `parse_ordinal`

```rust
fn parse_ordinal(digits: &[u8]) -> Option<u32>
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:324-334`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L324-L334)*

