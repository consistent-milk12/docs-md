*[clap_derive](../../index.md) / [derives](../index.md) / [subcommand](index.md)*

---

# Module `subcommand`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_subcommand`](#derive_subcommand) | fn |  |
| [`gen_for_enum`](#gen_for_enum) | fn |  |
| [`gen_augment`](#gen_augment) | fn |  |
| [`gen_has_subcommand`](#gen_has_subcommand) | fn |  |
| [`gen_from_arg_matches`](#gen_from_arg_matches) | fn |  |
| [`gen_update_from_arg_matches`](#gen_update_from_arg_matches) | fn |  |

## Functions

### `derive_subcommand`

```rust
fn derive_subcommand(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &proc_macro2::Ident, generics: &syn::Generics, variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_augment`

```rust
fn gen_augment(variants: &[(&syn::Variant, crate::item::Item)], parent_item: &crate::item::Item, override_required: bool) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_has_subcommand`

```rust
fn gen_has_subcommand(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_from_arg_matches`

```rust
fn gen_from_arg_matches(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_update_from_arg_matches`

```rust
fn gen_update_from_arg_matches(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

