*[clap_derive](../../index.md) / [derives](../index.md) / [parser](index.md)*

---

# Module `parser`

## Functions

### `derive_parser`

```rust
fn derive_parser(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_for_struct`

```rust
fn gen_for_struct(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics, fields: &[(&syn::Field, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics, variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

