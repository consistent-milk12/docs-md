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

*Defined in [`object-0.37.3/src/read/pe/resource.rs:12-14`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L12-L14)*

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

#### Implementations

- <span id="resourcedirectory-new"></span>`fn new(data: &'data [u8]) -> Self`

- <span id="resourcedirectory-root"></span>`fn root(&self) -> Result<ResourceDirectoryTable<'data>>` — [`Result`](../../../index.md), [`ResourceDirectoryTable`](../index.md)

#### Trait Implementations

##### `impl Clone for ResourceDirectory<'data>`

- <span id="resourcedirectory-clone"></span>`fn clone(&self) -> ResourceDirectory<'data>` — [`ResourceDirectory`](../index.md)

##### `impl Copy for ResourceDirectory<'data>`

##### `impl Debug for ResourceDirectory<'data>`

- <span id="resourcedirectory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceDirectoryTable<'data>`

```rust
struct ResourceDirectoryTable<'data> {
    pub header: &'data pe::ImageResourceDirectory,
    pub entries: &'data [pe::ImageResourceDirectoryEntry],
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:30-35`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L30-L35)*

A table of resource entries.

#### Fields

- **`header`**: `&'data pe::ImageResourceDirectory`

  The table header.

- **`entries`**: `&'data [pe::ImageResourceDirectoryEntry]`

  The table entries.

#### Implementations

- <span id="resourcedirectorytable-parse"></span>`fn parse(data: &'data [u8], offset: u32) -> Result<Self>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl Clone for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-clone"></span>`fn clone(&self) -> ResourceDirectoryTable<'data>` — [`ResourceDirectoryTable`](../index.md)

##### `impl Debug for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceName`

```rust
struct ResourceName {
    offset: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:143-145`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L143-L145)*

A resource name.

#### Implementations

- <span id="resourcename-to-string-lossy"></span>`fn to_string_lossy(&self, directory: ResourceDirectory<'_>) -> Result<String>` — [`ResourceDirectory`](../index.md), [`Result`](../../../index.md)

- <span id="resourcename-data"></span>`fn data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` — [`ResourceDirectory`](../index.md), [`Result`](../../../index.md), [`U16Bytes`](../../../index.md), [`LittleEndian`](../../../index.md)

- <span id="resourcename-raw-data"></span>`fn raw_data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` — [`ResourceDirectory`](../index.md), [`Result`](../../../index.md)

#### Trait Implementations

##### `impl Clone for ResourceName`

- <span id="resourcename-clone"></span>`fn clone(&self) -> ResourceName` — [`ResourceName`](../index.md)

##### `impl Copy for ResourceName`

##### `impl Debug for ResourceName`

- <span id="resourcename-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ResourceDirectoryEntryData<'data>`

```rust
enum ResourceDirectoryEntryData<'data> {
    Table(ResourceDirectoryTable<'data>),
    Data(&'data pe::ImageResourceDataEntry),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:112-117`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L112-L117)*

Data associated with a resource directory entry.

#### Variants

- **`Table`**

  A subtable entry.

- **`Data`**

  A resource data entry.

#### Implementations

- <span id="resourcedirectoryentrydata-table"></span>`fn table(self) -> Option<ResourceDirectoryTable<'data>>` — [`ResourceDirectoryTable`](../index.md)

- <span id="resourcedirectoryentrydata-data"></span>`fn data(self) -> Option<&'data pe::ImageResourceDataEntry>` — [`ImageResourceDataEntry`](../../../pe/index.md)

#### Trait Implementations

##### `impl Clone for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-clone"></span>`fn clone(&self) -> ResourceDirectoryEntryData<'data>` — [`ResourceDirectoryEntryData`](../index.md)

##### `impl Debug for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceNameOrId`

```rust
enum ResourceNameOrId {
    Name(ResourceName),
    Id(u16),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:183-188`](../../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L183-L188)*

A resource name or ID.

Can be either a string or a numeric ID.

#### Variants

- **`Name`**

  A resource name.

- **`Id`**

  A resource ID.

#### Implementations

- <span id="resourcenameorid-name"></span>`fn name(self) -> Option<ResourceName>` — [`ResourceName`](../index.md)

- <span id="resourcenameorid-id"></span>`fn id(self) -> Option<u16>`

#### Trait Implementations

##### `impl Debug for ResourceNameOrId`

- <span id="resourcenameorid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

