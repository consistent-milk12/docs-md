*[clap_derive](../../index.md) / [derives](../index.md) / [args](index.md)*

---

# Module `args`

## Functions

### `derive_args`

```rust
fn derive_args(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_for_struct`

```rust
fn gen_for_struct(item: &crate::item::Item, item_name: &proc_macro2::Ident, generics: &syn::Generics, fields: &[(&syn::Field, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_augment`

```rust
fn gen_augment(fields: &[(&syn::Field, crate::item::Item)], app_var: &proc_macro2::Ident, parent_item: &crate::item::Item, override_required: bool) -> Result<proc_macro2::TokenStream, syn::Error>
```

Generate a block of code to add arguments/subcommands corresponding to
the `fields` to an cmd.

### `gen_constructor`

```rust
fn gen_constructor(fields: &[(&syn::Field, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_updater`

```rust
fn gen_updater(fields: &[(&syn::Field, crate::item::Item)], use_self: bool) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `gen_parsers`

```rust
fn gen_parsers(item: &crate::item::Item, ty: &self::spanned::Sp<self::ty::Ty>, field_name: &proc_macro2::Ident, field: &syn::Field, update: Option<&proc_macro2::TokenStream>) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `raw_deprecated`

```rust
fn raw_deprecated() -> proc_macro2::TokenStream
```

### `collect_args_fields`

```rust
fn collect_args_fields<'a>(item: &'a crate::item::Item, fields: &'a syn::FieldsNamed) -> Result<Vec<(&'a syn::Field, crate::item::Item)>, syn::Error>
```

