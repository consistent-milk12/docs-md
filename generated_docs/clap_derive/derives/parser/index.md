*[clap_derive](../../index.md) / [derives](../index.md) / [parser](index.md)*

---

# Module `parser`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_parser`](#derive_parser) | fn |  |
| [`gen_for_struct`](#gen_for_struct) | fn |  |
| [`gen_for_enum`](#gen_for_enum) | fn |  |

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

