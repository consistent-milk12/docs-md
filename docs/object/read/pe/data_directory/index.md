*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [data_directory](index.md)*

---

# Module `data_directory`

## Structs

### `DataDirectories<'data>`

```rust
struct DataDirectories<'data> {
    entries: &'data [pe::ImageDataDirectory],
}
```

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

#### Implementations

- `fn parse(data: &'data [u8], number: u32) -> Result<Self>` — [`Result`](../../../index.md)

- `fn len(self: &Self) -> usize`

- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md)

- `fn enumerate(self: &Self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` — [`ImageDataDirectory`](../../../pe/index.md)

- `fn get(self: &Self, index: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md)

- `fn export_directory<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` — [`SectionTable`](../index.md), [`Result`](../../../index.md), [`ImageExportDirectory`](../../../pe/index.md)

- `fn export_table<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` — [`SectionTable`](../index.md), [`Result`](../../../index.md), [`ExportTable`](../index.md)

- `fn import_table<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` — [`SectionTable`](../index.md), [`Result`](../../../index.md), [`ImportTable`](../index.md)

- `fn delay_load_import_table<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` — [`SectionTable`](../index.md), [`Result`](../../../index.md), [`DelayLoadImportTable`](../index.md)

- `fn relocation_blocks<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` — [`SectionTable`](../index.md), [`Result`](../../../index.md), [`RelocationBlockIterator`](../index.md)

- `fn resource_directory<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` — [`SectionTable`](../index.md), [`Result`](../../../index.md), [`ResourceDirectory`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for DataDirectories<'data>`

- `fn clone(self: &Self) -> DataDirectories<'data>` — [`DataDirectories`](../index.md)

##### `impl<'data> Copy for DataDirectories<'data>`

##### `impl<'data> Debug for DataDirectories<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

