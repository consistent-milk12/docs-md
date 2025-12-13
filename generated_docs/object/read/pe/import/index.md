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

  Create a new import table parser.

  

  The import descriptors start at `import_address`.

  The size declared in the `IMAGE_DIRECTORY_ENTRY_IMPORT` data directory is

  ignored by the Windows loader, and so descriptors will be parsed until a null entry.

  

  `section_data` should be from the section containing `import_address`, and

  `section_address` should be the address of that section. Pointers within the

  descriptors and thunks may point to anywhere within the section data.

- <span id="importtable-descriptors"></span>`fn descriptors(&self) -> Result<ImportDescriptorIterator<'data>>` — [`Result`](../../../index.md#result), [`ImportDescriptorIterator`](../index.md#importdescriptoriterator)

  Return an iterator for the import descriptors.

- <span id="importtable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

  Return a library name given its address.

  

  This address may be from `pe::ImageImportDescriptor::name`.

- <span id="importtable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../../index.md#result), [`ImportThunkList`](../index.md#importthunklist)

  Return a list of thunks given its address.

  

  This address may be from `pe::ImageImportDescriptor::original_first_thunk`

  or `pe::ImageImportDescriptor::first_thunk`.

- <span id="importtable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](../index.md#imagentheaders), [`Result`](../../../index.md#result), [`Import`](../index.md#import)

  Parse a thunk.

- <span id="importtable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../../index.md#result)

  Return the hint and name at the given address.

  

  This address may be from [`pe::ImageThunkData32`](../../../pe/index.md) or [`pe::ImageThunkData64`](../../../pe/index.md).

  

  The hint is an index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl Any for ImportTable<'data>`

- <span id="importtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportTable<'data>`

- <span id="importtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportTable<'data>`

- <span id="importtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportTable<'data>`

- <span id="importtable-clone"></span>`fn clone(&self) -> ImportTable<'data>` — [`ImportTable`](../index.md#importtable)

##### `impl CloneToUninit for ImportTable<'data>`

- <span id="importtable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportTable<'data>`

- <span id="importtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportTable<'data>`

- <span id="importtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportTable<'data>`

- <span id="importtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ImportTable<'data>`

- <span id="importtable-toowned-type-owned"></span>`type Owned = T`

- <span id="importtable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importtable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportTable<'data>`

- <span id="importtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportTable<'data>`

- <span id="importtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Return the next descriptor.

  

  Returns `Ok(None)` when a null descriptor is found.

#### Trait Implementations

##### `impl Any for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-clone"></span>`fn clone(&self) -> ImportDescriptorIterator<'data>` — [`ImportDescriptorIterator`](../index.md#importdescriptoriterator)

##### `impl CloneToUninit for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="importdescriptoriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="importdescriptoriterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-iterator-type-item"></span>`type Item = Result<&'data ImageImportDescriptor, Error>`

- <span id="importdescriptoriterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-toowned-type-owned"></span>`type Owned = T`

- <span id="importdescriptoriterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importdescriptoriterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importdescriptoriterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importdescriptoriterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Get the thunk at the given index.

- <span id="importthunklist-next"></span>`fn next<Pe: ImageNtHeaders>(&mut self) -> Result<Option<<Pe as >::ImageThunkData>>` — [`Result`](../../../index.md#result), [`ImageNtHeaders`](../index.md#imagentheaders)

  Return the first thunk in the list, and update `self` to point after it.

  

  Returns `Ok(None)` when a null thunk is found.

#### Trait Implementations

##### `impl Any for ImportThunkList<'data>`

- <span id="importthunklist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportThunkList<'data>`

- <span id="importthunklist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportThunkList<'data>`

- <span id="importthunklist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportThunkList<'data>`

- <span id="importthunklist-clone"></span>`fn clone(&self) -> ImportThunkList<'data>` — [`ImportThunkList`](../index.md#importthunklist)

##### `impl CloneToUninit for ImportThunkList<'data>`

- <span id="importthunklist-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportThunkList<'data>`

- <span id="importthunklist-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportThunkList<'data>`

- <span id="importthunklist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportThunkList<'data>`

- <span id="importthunklist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ImportThunkList<'data>`

- <span id="importthunklist-toowned-type-owned"></span>`type Owned = T`

- <span id="importthunklist-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importthunklist-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportThunkList<'data>`

- <span id="importthunklist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importthunklist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportThunkList<'data>`

- <span id="importthunklist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importthunklist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new delay load import table parser.

  

  The import descriptors start at `import_address`.

  This table works in the same way the import table does: descriptors will be

  parsed until a null entry.

  

  `section_data` should be from the section containing `import_address`, and

  `section_address` should be the address of that section. Pointers within the

  descriptors and thunks may point to anywhere within the section data.

- <span id="delayloadimporttable-descriptors"></span>`fn descriptors(&self) -> Result<DelayLoadDescriptorIterator<'data>>` — [`Result`](../../../index.md#result), [`DelayLoadDescriptorIterator`](../index.md#delayloaddescriptoriterator)

  Return an iterator for the import descriptors.

- <span id="delayloadimporttable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

  Return a library name given its address.

  

  This address may be from `pe::ImageDelayloadDescriptor::dll_name_rva`.

- <span id="delayloadimporttable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../../index.md#result), [`ImportThunkList`](../index.md#importthunklist)

  Return a list of thunks given its address.

  

  This address may be from the INT, i.e. from

  `pe::ImageDelayloadDescriptor::import_name_table_rva`.

  

  Please note that others RVA values from [`pe::ImageDelayloadDescriptor`](../../../pe/index.md) are used

  by the delay loader at runtime to store values, and thus do not point inside the same

  section as the INT. Calling this function on those addresses will fail.

- <span id="delayloadimporttable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](../index.md#imagentheaders), [`Result`](../../../index.md#result), [`Import`](../index.md#import)

  Parse a thunk.

- <span id="delayloadimporttable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../../index.md#result)

  Return the hint and name at the given address.

  

  This address may be from [`pe::ImageThunkData32`](../../../pe/index.md) or [`pe::ImageThunkData64`](../../../pe/index.md).

  

  The hint is an index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl Any for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-clone"></span>`fn clone(&self) -> DelayLoadImportTable<'data>` — [`DelayLoadImportTable`](../index.md#delayloadimporttable)

##### `impl CloneToUninit for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-toowned-type-owned"></span>`type Owned = T`

- <span id="delayloadimporttable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="delayloadimporttable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="delayloadimporttable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="delayloadimporttable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Return the next descriptor.

  

  Returns `Ok(None)` when a null descriptor is found.

#### Trait Implementations

##### `impl Any for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-clone"></span>`fn clone(&self) -> DelayLoadDescriptorIterator<'data>` — [`DelayLoadDescriptorIterator`](../index.md#delayloaddescriptoriterator)

##### `impl CloneToUninit for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="delayloaddescriptoriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="delayloaddescriptoriterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-iterator-type-item"></span>`type Item = Result<&'data ImageDelayloadDescriptor, Error>`

- <span id="delayloaddescriptoriterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-toowned-type-owned"></span>`type Owned = T`

- <span id="delayloaddescriptoriterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="delayloaddescriptoriterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="delayloaddescriptoriterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="delayloaddescriptoriterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Import<'data>`

- <span id="import-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Import<'data>`

- <span id="import-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Import<'data>`

- <span id="import-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Import<'data>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'data>` — [`Import`](../index.md#import)

##### `impl CloneToUninit for Import<'data>`

- <span id="import-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Import<'data>`

##### `impl Debug for Import<'data>`

- <span id="import-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Import<'data>`

- <span id="import-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Import<'data>`

- <span id="import-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Import<'data>`

- <span id="import-toowned-type-owned"></span>`type Owned = T`

- <span id="import-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="import-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Import<'data>`

- <span id="import-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="import-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Import<'data>`

- <span id="import-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="import-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

