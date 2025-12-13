*[serde_derive](../../index.md) / [internals](../index.md) / [ast](index.md)*

---

# Module `ast`

A Serde ast, parsed from the Syn ast and ready to generate Rust code.

## Contents

- [Structs](#structs)
  - [`Container`](#container)
  - [`Variant`](#variant)
  - [`Field`](#field)
- [Enums](#enums)
  - [`Data`](#data)
  - [`Style`](#style)
- [Functions](#functions)
  - [`enum_from_ast`](#enum-from-ast)
  - [`struct_from_ast`](#struct-from-ast)
  - [`fields_from_ast`](#fields-from-ast)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Container`](#container) | struct | A source data structure annotated with `#[derive(Serialize)]` and/or `#[derive(Deserialize)]`, parsed into an internal representation. |
| [`Variant`](#variant) | struct | A variant of an enum. |
| [`Field`](#field) | struct | A field of a struct. |
| [`Data`](#data) | enum | The fields of a struct or enum. |
| [`Style`](#style) | enum |  |
| [`enum_from_ast`](#enum-from-ast) | fn |  |
| [`struct_from_ast`](#struct-from-ast) | fn |  |
| [`fields_from_ast`](#fields-from-ast) | fn |  |

## Structs

### `Container<'a>`

```rust
struct Container<'a> {
    pub ident: syn::Ident,
    pub attrs: attr::Container,
    pub data: Data<'a>,
    pub generics: &'a syn::Generics,
    pub original: &'a syn::DeriveInput,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:10-21`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L10-L21)*

A source data structure annotated with `#[derive(Serialize)]` and/or `#[derive(Deserialize)]`,
parsed into an internal representation.

#### Fields

- **`ident`**: `syn::Ident`

  The struct or enum name (without generics).

- **`attrs`**: `attr::Container`

  Attributes on the structure, parsed for Serde.

- **`data`**: `Data<'a>`

  The contents of the struct or enum.

- **`generics`**: `&'a syn::Generics`

  Any generics on the struct or enum.

- **`original`**: `&'a syn::DeriveInput`

  Original input.

#### Implementations

- <span id="container-from-ast"></span>`fn from_ast(cx: &Ctxt, item: &'a syn::DeriveInput, derive: Derive, private: &Ident) -> Option<Container<'a>>` — [`Ctxt`](../ctxt/index.md#ctxt), [`Derive`](../index.md#derive), [`Container`](#container)

  Convert the raw Syn ast into a parsed container object, collecting errors in `cx`.

#### Trait Implementations

##### `impl Any for Container<'a>`

- <span id="container-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Container<'a>`

- <span id="container-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Container<'a>`

- <span id="container-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Container<'a>`

- <span id="container-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Container<'a>`

- <span id="container-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Container<'a>`

- <span id="container-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="container-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Container<'a>`

- <span id="container-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="container-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Variant<'a>`

```rust
struct Variant<'a> {
    pub ident: syn::Ident,
    pub attrs: attr::Variant,
    pub style: Style,
    pub fields: Vec<Field<'a>>,
    pub original: &'a syn::Variant,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:32-38`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L32-L38)*

A variant of an enum.

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
    pub member: syn::Member,
    pub attrs: attr::Field,
    pub ty: &'a syn::Type,
    pub original: &'a syn::Field,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:41-46`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L41-L46)*

A field of a struct.

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

### `Data<'a>`

```rust
enum Data<'a> {
    Enum(Vec<Variant<'a>>),
    Struct(Style, Vec<Field<'a>>),
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:26-29`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L26-L29)*

The fields of a struct or enum.

Analogous to `syn::Data`.

#### Implementations

- <span id="data-all-fields"></span>`fn all_fields(self: &'a Self) -> Box<dyn Iterator<Item = &'a Field<'a>>>` — [`Field`](#field)

- <span id="data-has-getter"></span>`fn has_getter(&self) -> bool`

#### Trait Implementations

##### `impl Any for Data<'a>`

- <span id="data-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Data<'a>`

- <span id="data-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Data<'a>`

- <span id="data-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Data<'a>`

- <span id="data-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Data<'a>`

- <span id="data-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Data<'a>`

- <span id="data-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="data-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Data<'a>`

- <span id="data-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="data-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Style`

```rust
enum Style {
    Struct,
    Tuple,
    Newtype,
    Unit,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:49-58`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L49-L58)*

#### Variants

- **`Struct`**

  Named fields.

- **`Tuple`**

  Many unnamed fields.

- **`Newtype`**

  One unnamed field.

- **`Unit`**

  No fields.

#### Trait Implementations

##### `impl Any for Style`

- <span id="style-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Style`

- <span id="style-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Style`

- <span id="style-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](#style)

##### `impl CloneToUninit for Style`

- <span id="style-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Style`

##### `impl<T> From for Style`

- <span id="style-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Style`

- <span id="style-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Style`

- <span id="style-toowned-type-owned"></span>`type Owned = T`

- <span id="style-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="style-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Style`

- <span id="style-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="style-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Style`

- <span id="style-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="style-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `enum_from_ast`

```rust
fn enum_from_ast<'a>(cx: &crate::internals::Ctxt, variants: &'a syn::punctuated::Punctuated<syn::Variant, token::Comma>, container_default: &attr::Default, private: &proc_macro2::Ident) -> Vec<Variant<'a>>
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:133-172`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L133-L172)*

### `struct_from_ast`

```rust
fn struct_from_ast<'a>(cx: &crate::internals::Ctxt, fields: &'a syn::Fields, attrs: Option<&attr::Variant>, container_default: &attr::Default, private: &proc_macro2::Ident) -> (Style, Vec<Field<'a>>)
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:174-196`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L174-L196)*

### `fields_from_ast`

```rust
fn fields_from_ast<'a>(cx: &crate::internals::Ctxt, fields: &'a syn::punctuated::Punctuated<syn::Field, token::Comma>, attrs: Option<&attr::Variant>, container_default: &attr::Default, private: &proc_macro2::Ident) -> Vec<Field<'a>>
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:198-218`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ast.rs#L198-L218)*

