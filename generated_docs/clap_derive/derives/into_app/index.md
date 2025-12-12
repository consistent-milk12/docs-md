*[clap_derive](../../index.md) / [derives](../index.md) / [into_app](index.md)*

---

# Module `into_app`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`gen_for_struct`](#gen-for-struct) | fn |  |
| [`gen_for_enum`](#gen-for-enum) | fn |  |

## Functions

### `gen_for_struct`

```rust
fn gen_for_struct(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/into_app.rs:21-67`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/into_app.rs#L21-L67)*

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/into_app.rs:69-117`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/into_app.rs#L69-L117)*

