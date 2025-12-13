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

*Defined in [`object-0.37.3/src/read/coff/import.rs:20-26`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/import.rs#L20-L26)*

A Windows short form description of a symbol to import.

Used in Windows import libraries to provide a mapping from
a symbol name to a DLL export. This is not an object file.

This is a file that starts with [`pe::ImportObjectHeader`](../../../pe/index.md), and corresponds
to [`crate::FileKind::CoffImport`](../../../index.md).

#### Implementations

- <span id="importfile-parse"></span>`fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse it.

- <span id="importfile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

  Get the machine type.

- <span id="importfile-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md#subarchitecture)

  Get the sub machine type, if available.

- <span id="importfile-symbol"></span>`fn symbol(&self) -> &'data [u8]`

  The public symbol name.

- <span id="importfile-dll"></span>`fn dll(&self) -> &'data [u8]`

  The name of the DLL to import the symbol from.

- <span id="importfile-import"></span>`fn import(&self) -> ImportName<'data>` — [`ImportName`](../index.md#importname)

  The name exported from the DLL.

- <span id="importfile-import-type"></span>`fn import_type(&self) -> ImportType` — [`ImportType`](../index.md#importtype)

  The type of import. Usually either a function or data.

#### Trait Implementations

##### `impl Any for ImportFile<'data>`

- <span id="importfile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportFile<'data>`

- <span id="importfile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportFile<'data>`

- <span id="importfile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportFile<'data>`

- <span id="importfile-clone"></span>`fn clone(&self) -> ImportFile<'data>` — [`ImportFile`](../index.md#importfile)

##### `impl CloneToUninit for ImportFile<'data>`

- <span id="importfile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportFile<'data>`

- <span id="importfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportFile<'data>`

- <span id="importfile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportFile<'data>`

- <span id="importfile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ImportFile<'data>`

- <span id="importfile-toowned-type-owned"></span>`type Owned = T`

- <span id="importfile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importfile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportFile<'data>`

- <span id="importfile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importfile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportFile<'data>`

- <span id="importfile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importfile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImportObjectData<'data>`

```rust
struct ImportObjectData<'data> {
    symbol: crate::read::ByteString<'data>,
    dll: crate::read::ByteString<'data>,
    export: Option<crate::read::ByteString<'data>>,
}
```

*Defined in [`object-0.37.3/src/read/coff/import.rs:200-204`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/import.rs#L200-L204)*

The data following [`pe::ImportObjectHeader`](../../../pe/index.md).

#### Implementations

- <span id="importobjectdata-symbol"></span>`fn symbol(&self) -> &'data [u8]`

  The public symbol name.

- <span id="importobjectdata-dll"></span>`fn dll(&self) -> &'data [u8]`

  The name of the DLL to import the symbol from.

- <span id="importobjectdata-export"></span>`fn export(&self) -> Option<&'data [u8]>`

  The name exported from the DLL.

  

  This is only set if the name is not derived from the symbol name.

#### Trait Implementations

##### `impl Any for ImportObjectData<'data>`

- <span id="importobjectdata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportObjectData<'data>`

- <span id="importobjectdata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportObjectData<'data>`

- <span id="importobjectdata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportObjectData<'data>`

- <span id="importobjectdata-clone"></span>`fn clone(&self) -> ImportObjectData<'data>` — [`ImportObjectData`](../index.md#importobjectdata)

##### `impl CloneToUninit for ImportObjectData<'data>`

- <span id="importobjectdata-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportObjectData<'data>`

- <span id="importobjectdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportObjectData<'data>`

- <span id="importobjectdata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportObjectData<'data>`

- <span id="importobjectdata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ImportObjectData<'data>`

- <span id="importobjectdata-toowned-type-owned"></span>`type Owned = T`

- <span id="importobjectdata-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importobjectdata-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportObjectData<'data>`

- <span id="importobjectdata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importobjectdata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportObjectData<'data>`

- <span id="importobjectdata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importobjectdata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ImportName<'data>`

```rust
enum ImportName<'data> {
    Ordinal(u16),
    Name(&'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/coff/import.rs:114-119`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/import.rs#L114-L119)*

The name or ordinal to import from a DLL.

#### Variants

- **`Ordinal`**

  Import by ordinal. Ordinarily this is a 1-based index.

- **`Name`**

  Import by name.

#### Trait Implementations

##### `impl Any for ImportName<'data>`

- <span id="importname-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportName<'data>`

- <span id="importname-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportName<'data>`

- <span id="importname-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportName<'data>`

- <span id="importname-clone"></span>`fn clone(&self) -> ImportName<'data>` — [`ImportName`](../index.md#importname)

##### `impl CloneToUninit for ImportName<'data>`

- <span id="importname-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ImportName<'data>`

##### `impl Debug for ImportName<'data>`

- <span id="importname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImportName<'data>`

##### `impl<T> From for ImportName<'data>`

- <span id="importname-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportName<'data>`

- <span id="importname-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ImportName<'data>`

- <span id="importname-partialeq-eq"></span>`fn eq(&self, other: &ImportName<'data>) -> bool` — [`ImportName`](../index.md#importname)

##### `impl StructuralPartialEq for ImportName<'data>`

##### `impl ToOwned for ImportName<'data>`

- <span id="importname-toowned-type-owned"></span>`type Owned = T`

- <span id="importname-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importname-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportName<'data>`

- <span id="importname-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importname-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportName<'data>`

- <span id="importname-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importname-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImportType`

```rust
enum ImportType {
    Code,
    Data,
    Const,
}
```

*Defined in [`object-0.37.3/src/read/coff/import.rs:123-130`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/import.rs#L123-L130)*

The kind of import symbol.

#### Variants

- **`Code`**

  An executable code symbol.

- **`Data`**

  A data symbol.

- **`Const`**

  A constant value.

#### Trait Implementations

##### `impl Any for ImportType`

- <span id="importtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportType`

- <span id="importtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportType`

- <span id="importtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportType`

- <span id="importtype-clone"></span>`fn clone(&self) -> ImportType` — [`ImportType`](../index.md#importtype)

##### `impl CloneToUninit for ImportType`

- <span id="importtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ImportType`

##### `impl Debug for ImportType`

- <span id="importtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImportType`

##### `impl<T> From for ImportType`

- <span id="importtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ImportType`

- <span id="importtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ImportType`

- <span id="importtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ImportType`

- <span id="importtype-partialeq-eq"></span>`fn eq(&self, other: &ImportType) -> bool` — [`ImportType`](../index.md#importtype)

##### `impl StructuralPartialEq for ImportType`

##### `impl ToOwned for ImportType`

- <span id="importtype-toowned-type-owned"></span>`type Owned = T`

- <span id="importtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportType`

- <span id="importtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportType`

- <span id="importtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

