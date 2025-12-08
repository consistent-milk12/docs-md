*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [import](index.md)*

---

# Module `import`

Support for reading short import files.

These are used by some Windows linkers as a more compact way to describe
dynamically imported symbols.

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

- `fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md)

- `fn symbol(self: &Self) -> &'data [u8]`

- `fn dll(self: &Self) -> &'data [u8]`

- `fn import(self: &Self) -> ImportName<'data>` — [`ImportName`](../index.md)

- `fn import_type(self: &Self) -> ImportType` — [`ImportType`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ImportFile<'data>`

- `fn clone(self: &Self) -> ImportFile<'data>` — [`ImportFile`](../index.md)

##### `impl<'data> Debug for ImportFile<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn symbol(self: &Self) -> &'data [u8]`

- `fn dll(self: &Self) -> &'data [u8]`

- `fn export(self: &Self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for ImportObjectData<'data>`

- `fn clone(self: &Self) -> ImportObjectData<'data>` — [`ImportObjectData`](../index.md)

##### `impl<'data> Debug for ImportObjectData<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> ImportName<'data>` — [`ImportName`](../index.md)

##### `impl<'data> Copy for ImportName<'data>`

##### `impl<'data> Debug for ImportName<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for ImportName<'data>`

##### `impl<'data> PartialEq for ImportName<'data>`

- `fn eq(self: &Self, other: &ImportName<'data>) -> bool` — [`ImportName`](../index.md)

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

- `fn clone(self: &Self) -> ImportType` — [`ImportType`](../index.md)

##### `impl Copy for ImportType`

##### `impl Debug for ImportType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ImportType`

##### `impl Hash for ImportType`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ImportType`

- `fn eq(self: &Self, other: &ImportType) -> bool` — [`ImportType`](../index.md)

##### `impl StructuralPartialEq for ImportType`

