*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [export](index.md)*

---

# Module `export`

## Structs

### `Export<'data>`

```rust
struct Export<'data> {
    pub ordinal: u32,
    pub name: Option<&'data [u8]>,
    pub target: ExportTarget<'data>,
}
```

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

##### `impl<'data> Clone for Export<'data>`

- `fn clone(self: &Self) -> Export<'data>` — [`Export`](../index.md)

##### `impl<'data> Copy for Export<'data>`

##### `impl<'a> Debug for Export<'a>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

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

A partially parsed PE export table.

Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table).

#### Implementations

- `fn parse(data: &'data [u8], virtual_address: u32) -> Result<Self>` — [`Result`](../../../index.md)

- `fn parse_directory(data: &'data [u8]) -> Result<&'data pe::ImageExportDirectory>` — [`Result`](../../../index.md), [`ImageExportDirectory`](../../../pe/index.md)

- `fn directory(self: &Self) -> &'data pe::ImageExportDirectory` — [`ImageExportDirectory`](../../../pe/index.md)

- `fn ordinal_base(self: &Self) -> u32`

- `fn addresses(self: &Self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../../index.md), [`LittleEndian`](../../../index.md)

- `fn name_pointers(self: &Self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../../index.md), [`LittleEndian`](../../../index.md)

- `fn name_ordinals(self: &Self) -> &'data [U16Bytes<LE>]` — [`U16Bytes`](../../../index.md), [`LittleEndian`](../../../index.md)

- `fn name_iter(self: &Self) -> impl Iterator<Item = (u32, u16)> + 'data`

- `fn address_by_index(self: &Self, index: u32) -> Result<u32>` — [`Result`](../../../index.md)

- `fn address_by_ordinal(self: &Self, ordinal: u32) -> Result<u32>` — [`Result`](../../../index.md)

- `fn target_by_index(self: &Self, index: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md), [`ExportTarget`](../index.md)

- `fn target_by_ordinal(self: &Self, ordinal: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md), [`ExportTarget`](../index.md)

- `fn target_from_address(self: &Self, address: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md), [`ExportTarget`](../index.md)

- `fn forward_offset(self: &Self, address: u32) -> Option<usize>`

- `fn is_forward(self: &Self, address: u32) -> bool`

- `fn forward_string(self: &Self, address: u32) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn name_from_pointer(self: &Self, name_pointer: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ExportTable<'data>`

- `fn clone(self: &Self) -> ExportTable<'data>` — [`ExportTable`](../index.md)

##### `impl<'data> Debug for ExportTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `ExportTarget<'data>`

```rust
enum ExportTarget<'data> {
    Address(u32),
    ForwardByOrdinal(&'data [u8], u32),
    ForwardByName(&'data [u8], &'data [u8]),
}
```

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

- `fn is_address(self: &Self) -> bool`

- `fn is_forward(self: &Self) -> bool`

#### Trait Implementations

##### `impl<'data> Clone for ExportTarget<'data>`

- `fn clone(self: &Self) -> ExportTarget<'data>` — [`ExportTarget`](../index.md)

##### `impl<'data> Copy for ExportTarget<'data>`

##### `impl<'a> Debug for ExportTarget<'a>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

## Functions

### `parse_ordinal`

```rust
fn parse_ordinal(digits: &[u8]) -> Option<u32>
```

