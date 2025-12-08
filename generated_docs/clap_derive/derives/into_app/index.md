*[clap_derive](../../index.md) / [derives](../index.md) / [into_app](index.md)*

---

# Module `into_app`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`gen_for_struct`](#gen_for_struct) | fn |  |
| [`gen_for_enum`](#gen_for_enum) | fn |  |

## Functions

### `gen_for_struct`

```rust
fn gen_for_struct(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics) -> Result<proc_macro2::TokenStream, syn::Error>
```

