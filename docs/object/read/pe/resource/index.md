*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [resource](index.md)*

---

# Module `resource`

## Structs

### `ResourceDirectory<'data>`

```rust
struct ResourceDirectory<'data> {
    data: &'data [u8],
}
```

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

#### Implementations

- `fn new(data: &'data [u8]) -> Self`

- `fn root(self: &Self) -> Result<ResourceDirectoryTable<'data>>` — [`Result`](../../../index.md), [`ResourceDirectoryTable`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ResourceDirectory<'data>`

- `fn clone(self: &Self) -> ResourceDirectory<'data>` — [`ResourceDirectory`](../index.md)

##### `impl<'data> Copy for ResourceDirectory<'data>`

##### `impl<'data> Debug for ResourceDirectory<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ResourceDirectoryTable<'data>`

```rust
struct ResourceDirectoryTable<'data> {
    pub header: &'data pe::ImageResourceDirectory,
    pub entries: &'data [pe::ImageResourceDirectoryEntry],
}
```

A table of resource entries.

#### Fields

- **`header`**: `&'data pe::ImageResourceDirectory`

  The table header.

- **`entries`**: `&'data [pe::ImageResourceDirectoryEntry]`

  The table entries.

#### Implementations

- `fn parse(data: &'data [u8], offset: u32) -> Result<Self>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ResourceDirectoryTable<'data>`

- `fn clone(self: &Self) -> ResourceDirectoryTable<'data>` — [`ResourceDirectoryTable`](../index.md)

##### `impl<'data> Debug for ResourceDirectoryTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ResourceName`

```rust
struct ResourceName {
    offset: u32,
}
```

A resource name.

#### Implementations

- `fn to_string_lossy(self: &Self, directory: ResourceDirectory<'_>) -> Result<String>` — [`ResourceDirectory`](../index.md), [`Result`](../../../index.md)

- `fn data<'data>(self: &Self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` — [`ResourceDirectory`](../index.md), [`Result`](../../../index.md), [`U16Bytes`](../../../index.md), [`LittleEndian`](../../../index.md)

- `fn raw_data<'data>(self: &Self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` — [`ResourceDirectory`](../index.md), [`Result`](../../../index.md)

#### Trait Implementations

##### `impl Clone for ResourceName`

- `fn clone(self: &Self) -> ResourceName` — [`ResourceName`](../index.md)

##### `impl Copy for ResourceName`

##### `impl Debug for ResourceName`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `ResourceDirectoryEntryData<'data>`

```rust
enum ResourceDirectoryEntryData<'data> {
    Table(ResourceDirectoryTable<'data>),
    Data(&'data pe::ImageResourceDataEntry),
}
```

Data associated with a resource directory entry.

#### Variants

- **`Table`**

  A subtable entry.

- **`Data`**

  A resource data entry.

#### Implementations

- `fn table(self: Self) -> Option<ResourceDirectoryTable<'data>>` — [`ResourceDirectoryTable`](../index.md)

- `fn data(self: Self) -> Option<&'data pe::ImageResourceDataEntry>` — [`ImageResourceDataEntry`](../../../pe/index.md)

#### Trait Implementations

##### `impl<'data> Clone for ResourceDirectoryEntryData<'data>`

- `fn clone(self: &Self) -> ResourceDirectoryEntryData<'data>` — [`ResourceDirectoryEntryData`](../index.md)

##### `impl<'data> Debug for ResourceDirectoryEntryData<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ResourceNameOrId`

```rust
enum ResourceNameOrId {
    Name(ResourceName),
    Id(u16),
}
```

A resource name or ID.

Can be either a string or a numeric ID.

#### Variants

- **`Name`**

  A resource name.

- **`Id`**

  A resource ID.

#### Implementations

- `fn name(self: Self) -> Option<ResourceName>` — [`ResourceName`](../index.md)

- `fn id(self: Self) -> Option<u16>`

#### Trait Implementations

##### `impl Debug for ResourceNameOrId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

