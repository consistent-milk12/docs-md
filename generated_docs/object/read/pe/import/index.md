*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [import](index.md)*

---

# Module `import`

## Structs

### `ImportTable<'data>`

```rust
struct ImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

Information for parsing a PE import table.

Returned by [`DataDirectories::import_table`](super::DataDirectories::import_table).

#### Implementations

- `fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- `fn descriptors(self: &Self) -> Result<ImportDescriptorIterator<'data>>` — [`Result`](../../../index.md), [`ImportDescriptorIterator`](../index.md)

- `fn name(self: &Self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn thunks(self: &Self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../../index.md), [`ImportThunkList`](../index.md)

- `fn import<Pe: ImageNtHeaders>(self: &Self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](../index.md), [`Result`](../../../index.md), [`Import`](../index.md)

- `fn hint_name(self: &Self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ImportTable<'data>`

- `fn clone(self: &Self) -> ImportTable<'data>` — [`ImportTable`](../index.md)

##### `impl<'data> Debug for ImportTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ImportDescriptorIterator<'data>`

```rust
struct ImportDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

A fallible iterator for the descriptors in the import data directory.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<&'data pe::ImageImportDescriptor>>` — [`Result`](../../../index.md), [`ImageImportDescriptor`](../../../pe/index.md)

#### Trait Implementations

##### `impl<'data> Clone for ImportDescriptorIterator<'data>`

- `fn clone(self: &Self) -> ImportDescriptorIterator<'data>` — [`ImportDescriptorIterator`](../index.md)

##### `impl<'data> Debug for ImportDescriptorIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ImportDescriptorIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for ImportDescriptorIterator<'data>`

- `type Item = Result<&'data ImageImportDescriptor, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ImportThunkList<'data>`

```rust
struct ImportThunkList<'data> {
    data: crate::read::Bytes<'data>,
}
```

A list of import thunks.

These may be in the import lookup table, or the import address table.

#### Implementations

- `fn get<Pe: ImageNtHeaders>(self: &Self, index: usize) -> Result<<Pe as >::ImageThunkData>` — [`Result`](../../../index.md), [`ImageNtHeaders`](../index.md)

- `fn next<Pe: ImageNtHeaders>(self: &mut Self) -> Result<Option<<Pe as >::ImageThunkData>>` — [`Result`](../../../index.md), [`ImageNtHeaders`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ImportThunkList<'data>`

- `fn clone(self: &Self) -> ImportThunkList<'data>` — [`ImportThunkList`](../index.md)

##### `impl<'data> Debug for ImportThunkList<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DelayLoadImportTable<'data>`

```rust
struct DelayLoadImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

Information for parsing a PE delay-load import table.

Returned by
[`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table).

#### Implementations

- `fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- `fn descriptors(self: &Self) -> Result<DelayLoadDescriptorIterator<'data>>` — [`Result`](../../../index.md), [`DelayLoadDescriptorIterator`](../index.md)

- `fn name(self: &Self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn thunks(self: &Self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../../index.md), [`ImportThunkList`](../index.md)

- `fn import<Pe: ImageNtHeaders>(self: &Self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](../index.md), [`Result`](../../../index.md), [`Import`](../index.md)

- `fn hint_name(self: &Self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for DelayLoadImportTable<'data>`

- `fn clone(self: &Self) -> DelayLoadImportTable<'data>` — [`DelayLoadImportTable`](../index.md)

##### `impl<'data> Debug for DelayLoadImportTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DelayLoadDescriptorIterator<'data>`

```rust
struct DelayLoadDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

A fallible iterator for the descriptors in the delay-load data directory.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<&'data pe::ImageDelayloadDescriptor>>` — [`Result`](../../../index.md), [`ImageDelayloadDescriptor`](../../../pe/index.md)

#### Trait Implementations

##### `impl<'data> Clone for DelayLoadDescriptorIterator<'data>`

- `fn clone(self: &Self) -> DelayLoadDescriptorIterator<'data>` — [`DelayLoadDescriptorIterator`](../index.md)

##### `impl<'data> Debug for DelayLoadDescriptorIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for DelayLoadDescriptorIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for DelayLoadDescriptorIterator<'data>`

- `type Item = Result<&'data ImageDelayloadDescriptor, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Enums

### `Import<'data>`

```rust
enum Import<'data> {
    Ordinal(u16),
    Name(u16, &'data [u8]),
}
```

A parsed import thunk.

#### Variants

- **`Ordinal`**

  Import by ordinal.

- **`Name`**

  Import by name.
  
  Includes a hint for the index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl<'data> Clone for Import<'data>`

- `fn clone(self: &Self) -> Import<'data>` — [`Import`](../index.md)

##### `impl<'data> Copy for Import<'data>`

##### `impl<'data> Debug for Import<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `ImageThunkData`

```rust
trait ImageThunkData: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageThunkData32`](../../../pe/index.md) and [`pe::ImageThunkData64`](../../../pe/index.md).

#### Required Methods

- `fn raw(self: Self) -> u64`

  Return the raw thunk value.

- `fn is_ordinal(self: Self) -> bool`

  Returns true if the ordinal flag is set.

- `fn ordinal(self: Self) -> u16`

  Return the ordinal portion of the thunk.

- `fn address(self: Self) -> u32`

  Return the RVA portion of the thunk.

