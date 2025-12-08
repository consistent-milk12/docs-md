*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [import](index.md)*

---

# Module `import`

Support for reading short import files.

These are used by some Windows linkers as a more compact way to describe
dynamically imported symbols.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImportFile`](#importfile) | struct | A Windows short form description of a symbol to import. |
| [`ImportObjectData`](#importobjectdata) | struct | The data following [`pe::ImportObjectHeader`]. |
| [`ImportName`](#importname) | enum | The name or ordinal to import from a DLL. |
| [`ImportType`](#importtype) | enum | The kind of import symbol. |

## Structs

### `ImportFile<'data>`

```rust
struct ImportFile<'data> {
    header: &'data pe::ImportObjectHeader,
    kind: ImportType,
    dll: crate::read::ByteString<'data>,
    symbol: crate::read::ByteString<'data>,
    import: Option<crate::read::ByteString<'data>>,
}
```

A Windows short form description of a symbol to import.

Used in Windows import libraries to provide a mapping from
a symbol name to a DLL export. This is not an object file.

This is a file that starts with [`pe::ImportObjectHeader`](../../../pe/index.md), and corresponds
to [`crate::FileKind::CoffImport`](../../../index.md).

#### Implementations

- <span id="importfile-parse"></span>`fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- <span id="importfile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md)

- <span id="importfile-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md)

- <span id="importfile-symbol"></span>`fn symbol(&self) -> &'data [u8]`

- <span id="importfile-dll"></span>`fn dll(&self) -> &'data [u8]`

- <span id="importfile-import"></span>`fn import(&self) -> ImportName<'data>` — [`ImportName`](../index.md)

- <span id="importfile-import-type"></span>`fn import_type(&self) -> ImportType` — [`ImportType`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ImportFile<'data>`

- <span id="importfile-clone"></span>`fn clone(&self) -> ImportFile<'data>` — [`ImportFile`](../index.md)

##### `impl<'data> Debug for ImportFile<'data>`

- <span id="importfile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportObjectData<'data>`

```rust
struct ImportObjectData<'data> {
    symbol: crate::read::ByteString<'data>,
    dll: crate::read::ByteString<'data>,
    export: Option<crate::read::ByteString<'data>>,
}
```

The data following [`pe::ImportObjectHeader`](../../../pe/index.md).

#### Implementations

- <span id="importobjectdata-symbol"></span>`fn symbol(&self) -> &'data [u8]`

- <span id="importobjectdata-dll"></span>`fn dll(&self) -> &'data [u8]`

- <span id="importobjectdata-export"></span>`fn export(&self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for ImportObjectData<'data>`

- <span id="importobjectdata-clone"></span>`fn clone(&self) -> ImportObjectData<'data>` — [`ImportObjectData`](../index.md)

##### `impl<'data> Debug for ImportObjectData<'data>`

- <span id="importobjectdata-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ImportName<'data>`

```rust
enum ImportName<'data> {
    Ordinal(u16),
    Name(&'data [u8]),
}
```

The name or ordinal to import from a DLL.

#### Variants

- **`Ordinal`**

  Import by ordinal. Ordinarily this is a 1-based index.

- **`Name`**

  Import by name.

#### Trait Implementations

##### `impl<'data> Clone for ImportName<'data>`

- <span id="importname-clone"></span>`fn clone(&self) -> ImportName<'data>` — [`ImportName`](../index.md)

##### `impl<'data> Copy for ImportName<'data>`

##### `impl<'data> Debug for ImportName<'data>`

- <span id="importname-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data> Eq for ImportName<'data>`

##### `impl<'data> PartialEq for ImportName<'data>`

- <span id="importname-eq"></span>`fn eq(&self, other: &ImportName<'data>) -> bool` — [`ImportName`](../index.md)

##### `impl<'data> StructuralPartialEq for ImportName<'data>`

### `ImportType`

```rust
enum ImportType {
    Code,
    Data,
    Const,
}
```

The kind of import symbol.

#### Variants

- **`Code`**

  An executable code symbol.

- **`Data`**

  A data symbol.

- **`Const`**

  A constant value.

#### Trait Implementations

##### `impl Clone for ImportType`

- <span id="importtype-clone"></span>`fn clone(&self) -> ImportType` — [`ImportType`](../index.md)

##### `impl Copy for ImportType`

##### `impl Debug for ImportType`

- <span id="importtype-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImportType`

##### `impl Hash for ImportType`

- <span id="importtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ImportType`

- <span id="importtype-eq"></span>`fn eq(&self, other: &ImportType) -> bool` — [`ImportType`](../index.md)

##### `impl StructuralPartialEq for ImportType`

