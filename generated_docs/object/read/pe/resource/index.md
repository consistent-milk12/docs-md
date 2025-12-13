*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [resource](index.md)*

---

# Module `resource`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ResourceDirectory`](#resourcedirectory) | struct | The `.rsrc` section of a PE file. |
| [`ResourceDirectoryTable`](#resourcedirectorytable) | struct | A table of resource entries. |
| [`ResourceName`](#resourcename) | struct | A resource name. |
| [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata) | enum | Data associated with a resource directory entry. |
| [`ResourceNameOrId`](#resourcenameorid) | enum | A resource name or ID. |

## Structs

### `ResourceDirectory<'data>`

```rust
struct ResourceDirectory<'data> {
    data: &'data [u8],
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:12-14`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/resource.rs#L12-L14)*

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

#### Implementations

- <span id="resourcedirectory-new"></span>`fn new(data: &'data [u8]) -> Self`

  Construct from the data of the `.rsrc` section.

- <span id="resourcedirectory-root"></span>`fn root(&self) -> Result<ResourceDirectoryTable<'data>>` — [`Result`](../../../index.md#result), [`ResourceDirectoryTable`](../index.md#resourcedirectorytable)

  Parses the root resource directory.

#### Trait Implementations

##### `impl Any for ResourceDirectory<'data>`

- <span id="resourcedirectory-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceDirectory<'data>`

- <span id="resourcedirectory-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceDirectory<'data>`

- <span id="resourcedirectory-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceDirectory<'data>`

- <span id="resourcedirectory-clone"></span>`fn clone(&self) -> ResourceDirectory<'data>` — [`ResourceDirectory`](../index.md#resourcedirectory)

##### `impl CloneToUninit for ResourceDirectory<'data>`

- <span id="resourcedirectory-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ResourceDirectory<'data>`

##### `impl Debug for ResourceDirectory<'data>`

- <span id="resourcedirectory-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceDirectory<'data>`

- <span id="resourcedirectory-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceDirectory<'data>`

- <span id="resourcedirectory-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceDirectory<'data>`

- <span id="resourcedirectory-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcedirectory-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcedirectory-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceDirectory<'data>`

- <span id="resourcedirectory-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcedirectory-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceDirectory<'data>`

- <span id="resourcedirectory-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcedirectory-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceDirectoryTable<'data>`

```rust
struct ResourceDirectoryTable<'data> {
    pub header: &'data pe::ImageResourceDirectory,
    pub entries: &'data [pe::ImageResourceDirectoryEntry],
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:30-35`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/resource.rs#L30-L35)*

A table of resource entries.

#### Fields

- **`header`**: `&'data pe::ImageResourceDirectory`

  The table header.

- **`entries`**: `&'data [pe::ImageResourceDirectoryEntry]`

  The table entries.

#### Implementations

- <span id="resourcedirectorytable-parse"></span>`fn parse(data: &'data [u8], offset: u32) -> Result<Self>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Any for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-clone"></span>`fn clone(&self) -> ResourceDirectoryTable<'data>` — [`ResourceDirectoryTable`](../index.md#resourcedirectorytable)

##### `impl CloneToUninit for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcedirectorytable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcedirectorytable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcedirectorytable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcedirectorytable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceName`

```rust
struct ResourceName {
    offset: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:143-145`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/resource.rs#L143-L145)*

A resource name.

#### Implementations

- <span id="resourcename-to-string-lossy"></span>`fn to_string_lossy(&self, directory: ResourceDirectory<'_>) -> Result<String>` — [`ResourceDirectory`](../index.md#resourcedirectory), [`Result`](../../../index.md#result)

  Converts to a `String`.

- <span id="resourcename-data"></span>`fn data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` — [`ResourceDirectory`](../index.md#resourcedirectory), [`Result`](../../../index.md#result), [`U16Bytes`](../../../index.md#u16bytes), [`LittleEndian`](../../../index.md#littleendian)

  Returns the string unicode buffer.

- <span id="resourcename-raw-data"></span>`fn raw_data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` — [`ResourceDirectory`](../index.md#resourcedirectory), [`Result`](../../../index.md#result)

  Returns the string buffer as raw bytes.

#### Trait Implementations

##### `impl Any for ResourceName`

- <span id="resourcename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceName`

- <span id="resourcename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceName`

- <span id="resourcename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceName`

- <span id="resourcename-clone"></span>`fn clone(&self) -> ResourceName` — [`ResourceName`](../index.md#resourcename)

##### `impl CloneToUninit for ResourceName`

- <span id="resourcename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ResourceName`

##### `impl Debug for ResourceName`

- <span id="resourcename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceName`

- <span id="resourcename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceName`

- <span id="resourcename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceName`

- <span id="resourcename-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceName`

- <span id="resourcename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceName`

- <span id="resourcename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ResourceDirectoryEntryData<'data>`

```rust
enum ResourceDirectoryEntryData<'data> {
    Table(ResourceDirectoryTable<'data>),
    Data(&'data pe::ImageResourceDataEntry),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:112-117`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/resource.rs#L112-L117)*

Data associated with a resource directory entry.

#### Variants

- **`Table`**

  A subtable entry.

- **`Data`**

  A resource data entry.

#### Implementations

- <span id="resourcedirectoryentrydata-table"></span>`fn table(self) -> Option<ResourceDirectoryTable<'data>>` — [`ResourceDirectoryTable`](../index.md#resourcedirectorytable)

  Converts to an option of table.

  

  Helper for iterator filtering.

- <span id="resourcedirectoryentrydata-data"></span>`fn data(self) -> Option<&'data pe::ImageResourceDataEntry>` — [`ImageResourceDataEntry`](../../../pe/index.md#imageresourcedataentry)

  Converts to an option of data entry.

  

  Helper for iterator filtering.

#### Trait Implementations

##### `impl Any for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-clone"></span>`fn clone(&self) -> ResourceDirectoryEntryData<'data>` — [`ResourceDirectoryEntryData`](../index.md#resourcedirectoryentrydata)

##### `impl CloneToUninit for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcedirectoryentrydata-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcedirectoryentrydata-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcedirectoryentrydata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcedirectoryentrydata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceNameOrId`

```rust
enum ResourceNameOrId {
    Name(ResourceName),
    Id(u16),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:183-188`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/resource.rs#L183-L188)*

A resource name or ID.

Can be either a string or a numeric ID.

#### Variants

- **`Name`**

  A resource name.

- **`Id`**

  A resource ID.

#### Implementations

- <span id="resourcenameorid-name"></span>`fn name(self) -> Option<ResourceName>` — [`ResourceName`](../index.md#resourcename)

  Converts to an option of name.

  

  Helper for iterator filtering.

- <span id="resourcenameorid-id"></span>`fn id(self) -> Option<u16>`

  Converts to an option of ID.

  

  Helper for iterator filtering.

#### Trait Implementations

##### `impl Any for ResourceNameOrId`

- <span id="resourcenameorid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceNameOrId`

- <span id="resourcenameorid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceNameOrId`

- <span id="resourcenameorid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ResourceNameOrId`

- <span id="resourcenameorid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceNameOrId`

- <span id="resourcenameorid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceNameOrId`

- <span id="resourcenameorid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ResourceNameOrId`

- <span id="resourcenameorid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcenameorid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceNameOrId`

- <span id="resourcenameorid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcenameorid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

