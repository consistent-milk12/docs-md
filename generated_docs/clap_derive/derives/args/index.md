*[clap_derive](../../index.md) / [derives](../index.md) / [args](index.md)*

---

# Module `args`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_args`](#derive_args) | fn |  |
| [`gen_for_struct`](#gen_for_struct) | fn |  |
| [`gen_augment`](#gen_augment) | fn | Generate a block of code to add arguments/subcommands corresponding to the `fields` to an cmd. |
| [`gen_constructor`](#gen_constructor) | fn |  |
| [`gen_updater`](#gen_updater) | fn |  |
| [`gen_parsers`](#gen_parsers) | fn |  |
| [`raw_deprecated`](#raw_deprecated) | fn |  |
| [`collect_args_fields`](#collect_args_fields) | fn |  |

## Functions

### `derive_args`

```rust
fn derive_args(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:25-56`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L25-L56)*

### `gen_for_struct`

```rust
fn gen_for_struct(item: &crate::item::Item, item_name: &proc_macro2::Ident, generics: &syn::Generics, fields: &[(&syn::Field, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:58-165`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L58-L165)*

### `gen_augment`

```rust
fn gen_augment(fields: &[(&syn::Field, crate::item::Item)], app_var: &proc_macro2::Ident, parent_item: &crate::item::Item, override_required: bool) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:169-441`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L169-L441)*

Generate a block of code to add arguments/subcommands corresponding to
the `fields` to an cmd.

### `gen_constructor`

```rust
fn gen_constructor(fields: &[(&syn::Field, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:443-552`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L443-L552)*

### `gen_updater`

```rust
fn gen_updater(fields: &[(&syn::Field, crate::item::Item)], use_self: bool) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:554-658`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L554-L658)*

### `gen_parsers`

```rust
fn gen_parsers(item: &crate::item::Item, ty: &self::spanned::Sp<self::ty::Ty>, field_name: &proc_macro2::Ident, field: &syn::Field, update: Option<&proc_macro2::TokenStream>) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:660-762`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L660-L762)*

### `raw_deprecated`

```rust
fn raw_deprecated() -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:770-775`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L770-L775)*

### `collect_args_fields`

```rust
fn collect_args_fields<'a>(item: &'a crate::item::Item, fields: &'a syn::FieldsNamed) -> Result<Vec<(&'a syn::Field, crate::item::Item)>, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/args.rs:777-789`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/args.rs#L777-L789)*

