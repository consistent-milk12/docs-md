*[clap_derive](../../index.md) / [derives](../index.md) / [subcommand](index.md)*

---

# Module `subcommand`

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

