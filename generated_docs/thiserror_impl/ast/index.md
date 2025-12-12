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

##### `impl Clone for ContainerKind`

- <span id="containerkind-clone"></span>`fn clone(&self) -> ContainerKind` — [`ContainerKind`](#containerkind)

##### `impl Copy for ContainerKind`

##### `impl Display for ContainerKind`

- <span id="containerkind-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ContainerKind`

- <span id="containerkind-to-string"></span>`fn to_string(&self) -> String`

