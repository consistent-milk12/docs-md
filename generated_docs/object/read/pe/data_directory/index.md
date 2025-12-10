*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [data_directory](index.md)*

---

# Module `data_directory`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DataDirectories`](#datadirectories) | struct | The table of data directories in a PE file. |

## Structs

### `DataDirectories<'data>`

```rust
struct DataDirectories<'data> {
    entries: &'data [pe::ImageDataDirectory],
}
```

*Defined in [`object-0.37.3/src/read/pe/data_directory.rs:16-18`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/data_directory.rs#L16-L18)*

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

#### Implementations

- <span id="datadirectories-parse"></span>`fn parse(data: &'data [u8], number: u32) -> Result<Self>` — [`Result`](../../../index.md#result)

- <span id="datadirectories-len"></span>`fn len(&self) -> usize`

- <span id="datadirectories-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

- <span id="datadirectories-enumerate"></span>`fn enumerate(&self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

- <span id="datadirectories-get"></span>`fn get(&self, index: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

- <span id="datadirectories-export-directory"></span>`fn export_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ImageExportDirectory`](../../../pe/index.md#imageexportdirectory)

- <span id="datadirectories-export-table"></span>`fn export_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ExportTable`](../index.md#exporttable)

- <span id="datadirectories-import-table"></span>`fn import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ImportTable`](../index.md#importtable)

- <span id="datadirectories-delay-load-import-table"></span>`fn delay_load_import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`DelayLoadImportTable`](../index.md#delayloadimporttable)

- <span id="datadirectories-relocation-blocks"></span>`fn relocation_blocks<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`RelocationBlockIterator`](../index.md#relocationblockiterator)

- <span id="datadirectories-resource-directory"></span>`fn resource_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ResourceDirectory`](../index.md#resourcedirectory)

#### Trait Implementations

##### `impl Clone for DataDirectories<'data>`

- <span id="datadirectories-clone"></span>`fn clone(&self) -> DataDirectories<'data>` — [`DataDirectories`](../index.md#datadirectories)

##### `impl Copy for DataDirectories<'data>`

##### `impl Debug for DataDirectories<'data>`

- <span id="datadirectories-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

