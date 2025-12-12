*[clap_derive](../../index.md) / [derives](../index.md) / [parser](index.md)*

---

# Module `parser`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_parser`](#derive-parser) | fn |  |
| [`gen_for_struct`](#gen-for-struct) | fn |  |
| [`gen_for_enum`](#gen-for-enum) | fn |  |

## Functions

### `derive_parser`

```rust
fn derive_parser(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/parser.rs:29-75`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/parser.rs#L29-L75)*

### `gen_for_struct`

```rust
fn gen_for_struct(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics, fields: &[(&syn::Field, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/parser.rs:77-99`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/parser.rs#L77-L99)*

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &syn::Ident, generics: &syn::Generics, variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/parser.rs:101-119`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/parser.rs#L101-L119)*

