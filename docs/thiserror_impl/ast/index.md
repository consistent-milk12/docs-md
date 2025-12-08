*[thiserror_impl](../index.md) / [ast](index.md)*

---

# Module `ast`

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

#### Implementations

- `fn from_field(self: &Self) -> Option<&Field<'_>>` — [`Field`](#field)

- `fn source_field(self: &Self) -> Option<&Field<'_>>` — [`Field`](#field)

- `fn backtrace_field(self: &Self) -> Option<&Field<'_>>` — [`Field`](#field)

- `fn distinct_backtrace_field(self: &Self) -> Option<&Field<'_>>` — [`Field`](#field)

### `Enum<'a>`

```rust
struct Enum<'a> {
    pub attrs: crate::attr::Attrs<'a>,
    pub ident: syn::Ident,
    pub generics: &'a syn::Generics,
    pub variants: Vec<Variant<'a>>,
}
```

#### Implementations

- `fn has_source(self: &Self) -> bool`

- `fn has_backtrace(self: &Self) -> bool`

- `fn has_display(self: &Self) -> bool`

### `Variant<'a>`

```rust
struct Variant<'a> {
    pub original: &'a syn::Variant,
    pub attrs: crate::attr::Attrs<'a>,
    pub ident: syn::Ident,
    pub fields: Vec<Field<'a>>,
}
```

#### Implementations

- `fn from_syn(node: &'a syn::Variant, scope: &ParamsInScope<'a>) -> Result<Self>` — [`ParamsInScope`](../generics/index.md)

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

#### Implementations

- `fn is_backtrace(self: &Self) -> bool`

- `fn source_span(self: &Self) -> Span`

## Enums

### `Input<'a>`

```rust
enum Input<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
}
```

#### Implementations

- `fn from_syn(node: &'a DeriveInput) -> Result<Self>`

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

#### Implementations

- `fn from_struct(node: &DataStruct) -> Self`

- `fn from_variant(node: &syn::Variant) -> Self`

#### Trait Implementations

##### `impl Clone for ContainerKind`

- `fn clone(self: &Self) -> ContainerKind` — [`ContainerKind`](#containerkind)

##### `impl Copy for ContainerKind`

##### `impl Display for ContainerKind`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for ContainerKind`

- `fn to_string(self: &Self) -> String`

