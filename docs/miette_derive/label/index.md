*[miette_derive](../index.md) / [label](index.md)*

---

# Module `label`

## Structs

### `Labels`

```rust
struct Labels(Vec<Label>);
```

#### Implementations

- `fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- `fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- `fn gen_struct(self: &Self, fields: &syn::Fields) -> Option<TokenStream>`

- `fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

### `Label`

```rust
struct Label {
    label: Option<crate::fmt::Display>,
    ty: syn::Type,
    span: syn::Member,
    lbl_ty: LabelType,
}
```

### `LabelAttr`

```rust
struct LabelAttr {
    label: Option<crate::fmt::Display>,
    lbl_ty: LabelType,
}
```

#### Trait Implementations

##### `impl Parse for LabelAttr`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

## Enums

### `LabelType`

```rust
enum LabelType {
    Default,
    Primary,
    Collection,
}
```

#### Trait Implementations

##### `impl Eq for LabelType`

##### `impl PartialEq for LabelType`

- `fn eq(self: &Self, other: &LabelType) -> bool` — [`LabelType`](#labeltype)

##### `impl StructuralPartialEq for LabelType`

