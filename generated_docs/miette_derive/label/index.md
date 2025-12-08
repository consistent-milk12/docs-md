*[miette_derive](../index.md) / [label](index.md)*

---

# Module `label`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Labels`](#labels) | struct |  |
| [`Label`](#label) | struct |  |
| [`LabelAttr`](#labelattr) | struct |  |
| [`LabelType`](#labeltype) | enum |  |

## Structs

### `Labels`

```rust
struct Labels(Vec<Label>);
```

#### Implementations

- <span id="labels-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="labels-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="labels-gen-struct"></span>`fn gen_struct(&self, fields: &syn::Fields) -> Option<TokenStream>`

- <span id="labels-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

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

- <span id="labelattr-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

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

- <span id="labeltype-eq"></span>`fn eq(&self, other: &LabelType) -> bool` — [`LabelType`](#labeltype)

##### `impl StructuralPartialEq for LabelType`

