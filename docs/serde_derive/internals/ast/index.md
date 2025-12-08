*[serde_derive](../../index.md) / [internals](../index.md) / [ast](index.md)*

---

# Module `ast`

A Serde ast, parsed from the Syn ast and ready to generate Rust code.

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

- `fn from_ast(cx: &Ctxt, item: &'a syn::DeriveInput, derive: Derive, private: &Ident) -> Option<Container<'a>>` — [`Ctxt`](../index.md), [`Derive`](../index.md), [`Container`](#container)

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

A field of a struct.

## Enums

### `Data<'a>`

```rust
enum Data<'a> {
    Enum(Vec<Variant<'a>>),
    Struct(Style, Vec<Field<'a>>),
}
```

The fields of a struct or enum.

Analogous to `syn::Data`.

#### Implementations

- `fn all_fields(self: &'a Self) -> Box<dyn Iterator<Item = &'a Field<'a>>>` — [`Field`](#field)

- `fn has_getter(self: &Self) -> bool`

### `Style`

```rust
enum Style {
    Struct,
    Tuple,
    Newtype,
    Unit,
}
```

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

- `fn clone(self: &Self) -> Style` — [`Style`](#style)

##### `impl Copy for Style`

## Functions

### `enum_from_ast`

```rust
fn enum_from_ast<'a>(cx: &crate::internals::Ctxt, variants: &'a syn::punctuated::Punctuated<syn::Variant, $crate::token::Comma>, container_default: &attr::Default, private: &proc_macro2::Ident) -> Vec<Variant<'a>>
```

### `struct_from_ast`

```rust
fn struct_from_ast<'a>(cx: &crate::internals::Ctxt, fields: &'a syn::Fields, attrs: Option<&attr::Variant>, container_default: &attr::Default, private: &proc_macro2::Ident) -> (Style, Vec<Field<'a>>)
```

### `fields_from_ast`

```rust
fn fields_from_ast<'a>(cx: &crate::internals::Ctxt, fields: &'a syn::punctuated::Punctuated<syn::Field, $crate::token::Comma>, attrs: Option<&attr::Variant>, container_default: &attr::Default, private: &proc_macro2::Ident) -> Vec<Field<'a>>
```

