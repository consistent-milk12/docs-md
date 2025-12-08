*[clap_derive](../../index.md) / [derives](../index.md) / [value_enum](index.md)*

---

# Module `value_enum`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_value_enum`](#derive_value_enum) | fn |  |
| [`gen_for_enum`](#gen_for_enum) | fn |  |
| [`lits`](#lits) | fn |  |
| [`gen_value_variants`](#gen_value_variants) | fn |  |
| [`gen_to_possible_value`](#gen_to_possible_value) | fn |  |

## Functions

### `derive_value_enum`

```rust
fn derive_value_enum(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &syn::Ident, variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `lits`

```rust
fn lits(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<Vec<(proc_macro2::TokenStream, syn::Ident)>, syn::Error>
```

### `gen_value_variants`

```rust
fn gen_value_variants(lits: &[(proc_macro2::TokenStream, syn::Ident)]) -> proc_macro2::TokenStream
```

### `gen_to_possible_value`

```rust
fn gen_to_possible_value(item: &crate::item::Item, lits: &[(proc_macro2::TokenStream, syn::Ident)]) -> proc_macro2::TokenStream
```

