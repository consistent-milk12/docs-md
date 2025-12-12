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

*Defined in [`miette-derive-7.6.0/src/label.rs:17`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L17)*

#### Implementations

- <span id="labels-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="labels-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="labels-gen-struct"></span>`fn gen_struct(&self, fields: &syn::Fields) -> Option<TokenStream>`

- <span id="labels-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

### `Label`

```rust
struct Label {
    label: Option<crate::fmt::Display>,
    ty: syn::Type,
    span: syn::Member,
    lbl_ty: LabelType,
}
```

*Defined in [`miette-derive-7.6.0/src/label.rs:26-31`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L26-L31)*

### `LabelAttr`

```rust
struct LabelAttr {
    label: Option<crate::fmt::Display>,
    lbl_ty: LabelType,
}
```

*Defined in [`miette-derive-7.6.0/src/label.rs:33-36`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L33-L36)*

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

*Defined in [`miette-derive-7.6.0/src/label.rs:20-24`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L20-L24)*

#### Trait Implementations

##### `impl Eq for LabelType`

##### `impl PartialEq for LabelType`

- <span id="labeltype-eq"></span>`fn eq(&self, other: &LabelType) -> bool` — [`LabelType`](#labeltype)

##### `impl StructuralPartialEq for LabelType`

