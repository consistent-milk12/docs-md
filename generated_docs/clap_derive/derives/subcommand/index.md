*[clap_derive](../../index.md) / [derives](../index.md) / [subcommand](index.md)*

---

# Module `subcommand`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_subcommand`](#derive-subcommand) | fn |  |
| [`gen_for_enum`](#gen-for-enum) | fn |  |
| [`gen_augment`](#gen-augment) | fn |  |
| [`gen_has_subcommand`](#gen-has-subcommand) | fn |  |
| [`gen_from_arg_matches`](#gen-from-arg-matches) | fn |  |
| [`gen_update_from_arg_matches`](#gen-update-from-arg-matches) | fn |  |

## Functions

### `derive_subcommand`

```rust
fn derive_subcommand(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/subcommand.rs:24-44`](../../../../.source_1765633015/clap_derive-4.5.49/src/derives/subcommand.rs#L24-L44)*

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &proc_macro2::Ident, generics: &syn::Generics, variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/subcommand.rs:46-136`](../../../../.source_1765633015/clap_derive-4.5.49/src/derives/subcommand.rs#L46-L136)*

### `gen_augment`

```rust
fn gen_augment(variants: &[(&syn::Variant, crate::item::Item)], parent_item: &crate::item::Item, override_required: bool) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/subcommand.rs:138-357`](../../../../.source_1765633015/clap_derive-4.5.49/src/derives/subcommand.rs#L138-L357)*

### `gen_has_subcommand`

```rust
fn gen_has_subcommand(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/subcommand.rs:359-421`](../../../../.source_1765633015/clap_derive-4.5.49/src/derives/subcommand.rs#L359-L421)*

### `gen_from_arg_matches`

```rust
fn gen_from_arg_matches(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/subcommand.rs:423-572`](../../../../.source_1765633015/clap_derive-4.5.49/src/derives/subcommand.rs#L423-L572)*

### `gen_update_from_arg_matches`

```rust
fn gen_update_from_arg_matches(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/subcommand.rs:574-674`](../../../../.source_1765633015/clap_derive-4.5.49/src/derives/subcommand.rs#L574-L674)*

