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

#### Implementations

- <span id="crateaststruct-from-field"></span>`fn from_field(&self) -> Option<&Field<'_>>` — [`Field`](#field)

- <span id="crateaststruct-source-field"></span>`fn source_field(&self) -> Option<&Field<'_>>` — [`Field`](#field)

- <span id="crateaststruct-backtrace-field"></span>`fn backtrace_field(&self) -> Option<&Field<'_>>` — [`Field`](#field)

- <span id="crateaststruct-distinct-backtrace-field"></span>`fn distinct_backtrace_field(&self) -> Option<&Field<'_>>` — [`Field`](#field)

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

- <span id="crateastenum-validate"></span>`fn validate(&self) -> Result<()>`

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

- <span id="crateastvariant-validate"></span>`fn validate(&self) -> Result<()>`

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

- <span id="crateastfield-is-backtrace"></span>`fn is_backtrace(&self) -> bool`

- <span id="crateastfield-source-span"></span>`fn source_span(&self) -> Span`

## Enums

### `Input<'a>`

```rust
enum Input<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
}
```

#### Implementations

- <span id="crateastinput-validate"></span>`fn validate(&self) -> Result<()>`

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

- <span id="containerkind-from-struct"></span>`fn from_struct(node: &DataStruct) -> Self`

- <span id="containerkind-from-variant"></span>`fn from_variant(node: &syn::Variant) -> Self`

#### Trait Implementations

##### `impl Clone for ContainerKind`

- <span id="containerkind-clone"></span>`fn clone(&self) -> ContainerKind` — [`ContainerKind`](#containerkind)

##### `impl Copy for ContainerKind`

##### `impl Display for ContainerKind`

- <span id="containerkind-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for ContainerKind`

- <span id="containerkind-to-string"></span>`fn to_string(&self) -> String`

