*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [export](index.md)*

---

# Module `export`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Export`](#export) | struct | An export from a PE file. |
| [`ExportTable`](#exporttable) | struct | A partially parsed PE export table. |
| [`ExportTarget`](#exporttarget) | enum | Where an export is pointing to. |
| [`parse_ordinal`](#parse-ordinal) | fn |  |

## Structs

### `Export<'data>`

```rust
struct Export<'data> {
    pub ordinal: u32,
    pub name: Option<&'data [u8]>,
    pub target: ExportTarget<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:42-51`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/export.rs#L42-L51)*

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

##### `impl Any for Export<'data>`

- <span id="export-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Export<'data>`

- <span id="export-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Export<'data>`

- <span id="export-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Export<'data>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'data>` — [`Export`](../index.md#export)

##### `impl CloneToUninit for Export<'data>`

- <span id="export-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Export<'data>`

##### `impl Debug for Export<'a>`

- <span id="export-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl<T> From for Export<'data>`

- <span id="export-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Export<'data>`

- <span id="export-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Export<'data>`

- <span id="export-toowned-type-owned"></span>`type Owned = T`

- <span id="export-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="export-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Export<'data>`

- <span id="export-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="export-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Export<'data>`

- <span id="export-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="export-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/export.rs:87-94`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/export.rs#L87-L94)*

A partially parsed PE export table.

Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table).

#### Implementations

- <span id="exporttable-parse"></span>`fn parse(data: &'data [u8], virtual_address: u32) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the export table given its section data and address.

- <span id="exporttable-parse-directory"></span>`fn parse_directory(data: &'data [u8]) -> Result<&'data pe::ImageExportDirectory>` — [`Result`](../../../index.md#result), [`ImageExportDirectory`](../../../pe/index.md#imageexportdirectory)

  Parse the export directory given its section data.

- <span id="exporttable-directory"></span>`fn directory(&self) -> &'data pe::ImageExportDirectory` — [`ImageExportDirectory`](../../../pe/index.md#imageexportdirectory)

  Returns the header of the export table.

- <span id="exporttable-ordinal-base"></span>`fn ordinal_base(&self) -> u32`

  Returns the base value of ordinals.

  

  Adding this to an address index will give an ordinal.

- <span id="exporttable-addresses"></span>`fn addresses(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../../index.md#u32bytes), [`LittleEndian`](../../../index.md#littleendian)

  Returns the unparsed address table.

  

  An address table entry may be a local address, or the address of a forwarded export entry.

  See `Self::is_forward` and `Self::target_from_address`.

- <span id="exporttable-name-pointers"></span>`fn name_pointers(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../../index.md#u32bytes), [`LittleEndian`](../../../index.md#littleendian)

  Returns the unparsed name pointer table.

  

  A name pointer table entry can be used with `Self::name_from_pointer`.

- <span id="exporttable-name-ordinals"></span>`fn name_ordinals(&self) -> &'data [U16Bytes<LE>]` — [`U16Bytes`](../../../index.md#u16bytes), [`LittleEndian`](../../../index.md#littleendian)

  Returns the unparsed ordinal table.

  

  An ordinal table entry is a 0-based index into the address table.

  See `Self::address_by_index` and `Self::target_by_index`.

- <span id="exporttable-name-iter"></span>`fn name_iter(&self) -> impl Iterator<Item = (u32, u16)> + 'data`

  Returns an iterator for the entries in the name pointer table and ordinal table.

  

  A name pointer table entry can be used with `Self::name_from_pointer`.

  

  An ordinal table entry is a 0-based index into the address table.

  See `Self::address_by_index` and `Self::target_by_index`.

- <span id="exporttable-address-by-index"></span>`fn address_by_index(&self, index: u32) -> Result<u32>` — [`Result`](../../../index.md#result)

  Returns the export address table entry at the given address index.

  

  This may be a local address, or the address of a forwarded export entry.

  See `Self::is_forward` and `Self::target_from_address`.

  

  `index` is a 0-based index into the export address table.

- <span id="exporttable-address-by-ordinal"></span>`fn address_by_ordinal(&self, ordinal: u32) -> Result<u32>` — [`Result`](../../../index.md#result)

  Returns the export address table entry at the given ordinal.

  

  This may be a local address, or the address of a forwarded export entry.

  See `Self::is_forward` and `Self::target_from_address`.

- <span id="exporttable-target-by-index"></span>`fn target_by_index(&self, index: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md#result), [`ExportTarget`](../index.md#exporttarget)

  Returns the target of the export at the given address index.

  

  `index` is a 0-based index into the export address table.

- <span id="exporttable-target-by-ordinal"></span>`fn target_by_ordinal(&self, ordinal: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md#result), [`ExportTarget`](../index.md#exporttarget)

  Returns the target of the export at the given ordinal.

- <span id="exporttable-target-from-address"></span>`fn target_from_address(&self, address: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../../index.md#result), [`ExportTarget`](../index.md#exporttarget)

  Convert an export address table entry into a target.

- <span id="exporttable-forward-offset"></span>`fn forward_offset(&self, address: u32) -> Option<usize>`

- <span id="exporttable-is-forward"></span>`fn is_forward(&self, address: u32) -> bool`

  Return true if the export address table entry is a forward.

- <span id="exporttable-forward-string"></span>`fn forward_string(&self, address: u32) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

  Return the forward string if the export address table entry is a forward.

- <span id="exporttable-name-from-pointer"></span>`fn name_from_pointer(&self, name_pointer: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

  Convert an export name pointer table entry into a name.

- <span id="exporttable-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md#result), [`Export`](../index.md#export)

  Returns the parsed exports in this table.

#### Trait Implementations

##### `impl Any for ExportTable<'data>`

- <span id="exporttable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExportTable<'data>`

- <span id="exporttable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExportTable<'data>`

- <span id="exporttable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ExportTable<'data>`

- <span id="exporttable-clone"></span>`fn clone(&self) -> ExportTable<'data>` — [`ExportTable`](../index.md#exporttable)

##### `impl CloneToUninit for ExportTable<'data>`

- <span id="exporttable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ExportTable<'data>`

- <span id="exporttable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ExportTable<'data>`

- <span id="exporttable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExportTable<'data>`

- <span id="exporttable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ExportTable<'data>`

- <span id="exporttable-toowned-type-owned"></span>`type Owned = T`

- <span id="exporttable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exporttable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExportTable<'data>`

- <span id="exporttable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exporttable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExportTable<'data>`

- <span id="exporttable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exporttable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ExportTarget<'data>`

```rust
enum ExportTarget<'data> {
    Address(u32),
    ForwardByOrdinal(&'data [u8], u32),
    ForwardByName(&'data [u8], &'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:10-21`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/export.rs#L10-L21)*

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

  Returns true if the target is an address.

- <span id="exporttarget-is-forward"></span>`fn is_forward(&self) -> bool`

  Returns true if the export is forwarded to another DLL.

#### Trait Implementations

##### `impl Any for ExportTarget<'data>`

- <span id="exporttarget-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExportTarget<'data>`

- <span id="exporttarget-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExportTarget<'data>`

- <span id="exporttarget-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ExportTarget<'data>`

- <span id="exporttarget-clone"></span>`fn clone(&self) -> ExportTarget<'data>` — [`ExportTarget`](../index.md#exporttarget)

##### `impl CloneToUninit for ExportTarget<'data>`

- <span id="exporttarget-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ExportTarget<'data>`

##### `impl Debug for ExportTarget<'a>`

- <span id="exporttarget-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl<T> From for ExportTarget<'data>`

- <span id="exporttarget-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExportTarget<'data>`

- <span id="exporttarget-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ExportTarget<'data>`

- <span id="exporttarget-toowned-type-owned"></span>`type Owned = T`

- <span id="exporttarget-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exporttarget-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExportTarget<'data>`

- <span id="exporttarget-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exporttarget-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExportTarget<'data>`

- <span id="exporttarget-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exporttarget-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_ordinal`

```rust
fn parse_ordinal(digits: &[u8]) -> Option<u32>
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:324-334`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/export.rs#L324-L334)*

