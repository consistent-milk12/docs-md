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

*Defined in [`object-0.37.3/src/read/pe/data_directory.rs:16-18`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/data_directory.rs#L16-L18)*

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

#### Implementations

- <span id="datadirectories-parse"></span>`fn parse(data: &'data [u8], number: u32) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the data directory table.

  

  `data` must be the remaining optional data following the

  [optional header](pe::ImageOptionalHeader64).  `number` must be from the

  [`number_of_rva_and_sizes`](pe::ImageOptionalHeader64::number_of_rva_and_sizes)

  field of the optional header.

- <span id="datadirectories-len"></span>`fn len(&self) -> usize`

  The number of data directories.

- <span id="datadirectories-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

  Iterator over the data directories.

- <span id="datadirectories-enumerate"></span>`fn enumerate(&self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

  Iterator which gives the directories as well as their index (one of the IMAGE_DIRECTORY_ENTRY_* constants).

- <span id="datadirectories-get"></span>`fn get(&self, index: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

  Returns the data directory at the given index.

  

  Index should be one of the `IMAGE_DIRECTORY_ENTRY_*` constants.

  

  Returns `None` if the index is larger than the table size,

  or if the entry at the index has a zero virtual address.

- <span id="datadirectories-export-directory"></span>`fn export_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ImageExportDirectory`](../../../pe/index.md#imageexportdirectory)

  Returns the unparsed export directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-export-table"></span>`fn export_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ExportTable`](../index.md#exporttable)

  Returns the partially parsed export directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-import-table"></span>`fn import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ImportTable`](../index.md#importtable)

  Returns the partially parsed import directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-delay-load-import-table"></span>`fn delay_load_import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`DelayLoadImportTable`](../index.md#delayloadimporttable)

  Returns the partially parsed delay-load import directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-relocation-blocks"></span>`fn relocation_blocks<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`RelocationBlockIterator`](../index.md#relocationblockiterator)

  Returns the blocks in the base relocation directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-resource-directory"></span>`fn resource_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` — [`SectionTable`](../../coff/index.md#sectiontable), [`Result`](../../../index.md#result), [`ResourceDirectory`](../index.md#resourcedirectory)

  Returns the resource directory.

  

  `data` must be the entire file data.

#### Trait Implementations

##### `impl Any for DataDirectories<'data>`

- <span id="datadirectories-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataDirectories<'data>`

- <span id="datadirectories-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataDirectories<'data>`

- <span id="datadirectories-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DataDirectories<'data>`

- <span id="datadirectories-clone"></span>`fn clone(&self) -> DataDirectories<'data>` — [`DataDirectories`](../index.md#datadirectories)

##### `impl CloneToUninit for DataDirectories<'data>`

- <span id="datadirectories-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DataDirectories<'data>`

##### `impl Debug for DataDirectories<'data>`

- <span id="datadirectories-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DataDirectories<'data>`

- <span id="datadirectories-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DataDirectories<'data>`

- <span id="datadirectories-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DataDirectories<'data>`

- <span id="datadirectories-toowned-type-owned"></span>`type Owned = T`

- <span id="datadirectories-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="datadirectories-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataDirectories<'data>`

- <span id="datadirectories-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="datadirectories-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataDirectories<'data>`

- <span id="datadirectories-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="datadirectories-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

