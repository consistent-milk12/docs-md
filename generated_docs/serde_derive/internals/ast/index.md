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

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:10-21`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L10-L21)*

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

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:32-38`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L32-L38)*

A variant of an enum.

### `Field<'a>`

```rust
struct Field<'a> {
    pub member: syn::Member,
    pub attrs: attr::Field,
    pub ty: &'a syn::Type,
    pub original: &'a syn::Field,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:41-46`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L41-L46)*

A field of a struct.

## Enums

### `Data<'a>`

```rust
enum Data<'a> {
    Enum(Vec<Variant<'a>>),
    Struct(Style, Vec<Field<'a>>),
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:26-29`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L26-L29)*

The fields of a struct or enum.

Analogous to `syn::Data`.

#### Implementations

- <span id="data-all-fields"></span>`fn all_fields(self: &'a Self) -> Box<dyn Iterator<Item = &'a Field<'a>>>` — [`Field`](#field)

- <span id="data-has-getter"></span>`fn has_getter(&self) -> bool`

### `Style`

```rust
enum Style {
    Struct,
    Tuple,
    Newtype,
    Unit,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:49-58`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L49-L58)*

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

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](#style)

##### `impl Copy for Style`

## Functions

### `enum_from_ast`

```rust
fn enum_from_ast<'a>(cx: &crate::internals::Ctxt, variants: &'a syn::punctuated::Punctuated<syn::Variant, token::Comma>, container_default: &attr::Default, private: &proc_macro2::Ident) -> Vec<Variant<'a>>
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:133-172`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L133-L172)*

### `struct_from_ast`

```rust
fn struct_from_ast<'a>(cx: &crate::internals::Ctxt, fields: &'a syn::Fields, attrs: Option<&attr::Variant>, container_default: &attr::Default, private: &proc_macro2::Ident) -> (Style, Vec<Field<'a>>)
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:174-196`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L174-L196)*

### `fields_from_ast`

```rust
fn fields_from_ast<'a>(cx: &crate::internals::Ctxt, fields: &'a syn::punctuated::Punctuated<syn::Field, token::Comma>, attrs: Option<&attr::Variant>, container_default: &attr::Default, private: &proc_macro2::Ident) -> Vec<Field<'a>>
```

*Defined in [`serde_derive-1.0.228/src/internals/ast.rs:198-218`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ast.rs#L198-L218)*

