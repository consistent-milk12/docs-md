*[thiserror_impl](../index.md) / [ast](index.md)*

---

# Module `ast`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Struct`](#struct) | struct |  |
| [`Enum`](#enum) | struct |  |
| [`Variant`](#variant) | struct |  |
| [`Field`](#field) | struct |  |
| [`Input`](#input) | enum |  |
| [`ContainerKind`](#containerkind) | enum |  |

## Structs

### `Struct<'a>`

```rust
struct Struct<'a> {
    pub attrs: crate::attr::Attrs<'a>,
    pub ident: syn::Ident,
    pub generics: &'a syn::Generics,
    pub fields: Vec<Field<'a>>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/ast.rs:15-20`](../../../.source_1765521767/thiserror-impl-2.0.17/src/ast.rs#L15-L20)*

#### Implementations

- <span id="struct-from-syn"></span>`fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Self>`

#### Trait Implementations

##### `impl Any for Struct<'a>`

- <span id="struct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Struct<'a>`

- <span id="struct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Struct<'a>`

- <span id="struct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Struct<'a>`

- <span id="struct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Struct<'a>`

- <span id="struct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Struct<'a>`

- <span id="struct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="struct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Struct<'a>`

- <span id="struct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="struct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Enum<'a>`

```rust
struct Enum<'a> {
    pub attrs: crate::attr::Attrs<'a>,
    pub ident: syn::Ident,
    pub generics: &'a syn::Generics,
    pub variants: Vec<Variant<'a>>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/ast.rs:22-27`](../../../.source_1765521767/thiserror-impl-2.0.17/src/ast.rs#L22-L27)*

#### Implementations

- <span id="enum-from-syn"></span>`fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Result<Self>`

#### Trait Implementations

##### `impl Any for Enum<'a>`

- <span id="enum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Enum<'a>`

- <span id="enum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Enum<'a>`

- <span id="enum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Enum<'a>`

- <span id="enum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Enum<'a>`

- <span id="enum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Enum<'a>`

- <span id="enum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Enum<'a>`

- <span id="enum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Variant<'a>`

```rust
struct Variant<'a> {
    pub original: &'a syn::Variant,
    pub attrs: crate::attr::Attrs<'a>,
    pub ident: syn::Ident,
    pub fields: Vec<Field<'a>>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/ast.rs:29-34`](../../../.source_1765521767/thiserror-impl-2.0.17/src/ast.rs#L29-L34)*

#### Implementations

- <span id="variant-from-syn"></span>`fn from_syn(node: &'a syn::Variant, scope: &ParamsInScope<'a>) -> Result<Self>` — [`ParamsInScope`](../generics/index.md#paramsinscope)

#### Trait Implementations

##### `impl Any for Variant<'a>`

- <span id="variant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Variant<'a>`

- <span id="variant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Variant<'a>`

- <span id="variant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Variant<'a>`

- <span id="variant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Variant<'a>`

- <span id="variant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Variant<'a>`

- <span id="variant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Variant<'a>`

- <span id="variant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Field<'a>`

```rust
struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: crate::attr::Attrs<'a>,
    pub member: crate::unraw::MemberUnraw,
    pub ty: &'a syn::Type,
    pub contains_generic: bool,
}
```

*Defined in [`thiserror-impl-2.0.17/src/ast.rs:36-42`](../../../.source_1765521767/thiserror-impl-2.0.17/src/ast.rs#L36-L42)*

#### Implementations

- <span id="field-multiple-from-syn"></span>`fn multiple_from_syn(fields: &'a Fields, scope: &ParamsInScope<'a>) -> Result<Vec<Self>>` — [`ParamsInScope`](../generics/index.md#paramsinscope)

- <span id="field-from-syn"></span>`fn from_syn(i: usize, node: &'a syn::Field, scope: &ParamsInScope<'a>) -> Result<Self>` — [`ParamsInScope`](../generics/index.md#paramsinscope)

#### Trait Implementations

##### `impl Any for Field<'a>`

- <span id="field-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Field<'a>`

- <span id="field-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Field<'a>`

- <span id="field-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Field<'a>`

- <span id="field-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Field<'a>`

- <span id="field-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Field<'a>`

- <span id="field-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="field-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Field<'a>`

- <span id="field-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="field-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Input<'a>`

```rust
enum Input<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
}
```

*Defined in [`thiserror-impl-2.0.17/src/ast.rs:10-13`](../../../.source_1765521767/thiserror-impl-2.0.17/src/ast.rs#L10-L13)*

#### Implementations

- <span id="input-from-syn"></span>`fn from_syn(node: &'a DeriveInput) -> Result<Self>`

#### Trait Implementations

##### `impl Any for Input<'a>`

- <span id="input-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Input<'a>`

- <span id="input-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Input<'a>`

- <span id="input-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Input<'a>`

- <span id="input-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Input<'a>`

- <span id="input-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Input<'a>`

- <span id="input-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="input-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Input<'a>`

- <span id="input-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="input-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ContainerKind`

```rust
enum ContainerKind {
    Struct,
    TupleStruct,
    UnitStruct,
    StructVariant,
    TupleVariant,
    UnitVariant,
}
```

*Defined in [`thiserror-impl-2.0.17/src/ast.rs:45-52`](../../../.source_1765521767/thiserror-impl-2.0.17/src/ast.rs#L45-L52)*

#### Implementations

- <span id="containerkind-from-struct"></span>`fn from_struct(node: &DataStruct) -> Self`

- <span id="containerkind-from-variant"></span>`fn from_variant(node: &syn::Variant) -> Self`

#### Trait Implementations

##### `impl Any for ContainerKind`

- <span id="containerkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ContainerKind`

- <span id="containerkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ContainerKind`

- <span id="containerkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ContainerKind`

- <span id="containerkind-clone"></span>`fn clone(&self) -> ContainerKind` — [`ContainerKind`](#containerkind)

##### `impl CloneToUninit for ContainerKind`

- <span id="containerkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ContainerKind`

##### `impl Display for ContainerKind`

- <span id="containerkind-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ContainerKind`

- <span id="containerkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ContainerKind`

- <span id="containerkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ContainerKind`

- <span id="containerkind-toowned-type-owned"></span>`type Owned = T`

- <span id="containerkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="containerkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ContainerKind`

- <span id="containerkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ContainerKind`

- <span id="containerkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="containerkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ContainerKind`

- <span id="containerkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="containerkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

