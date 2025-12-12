*[clap_derive](../../index.md) / [derives](../index.md) / [value_enum](index.md)*

---

# Module `value_enum`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_value_enum`](#derive-value-enum) | fn |  |
| [`gen_for_enum`](#gen-for-enum) | fn |  |
| [`lits`](#lits) | fn |  |
| [`gen_value_variants`](#gen-value-variants) | fn |  |
| [`gen_to_possible_value`](#gen-to-possible-value) | fn |  |

## Functions

### `derive_value_enum`

```rust
fn derive_value_enum(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/value_enum.rs:18-35`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/value_enum.rs#L18-L35)*

### `gen_for_enum`

```rust
fn gen_for_enum(item: &crate::item::Item, item_name: &syn::Ident, variants: &[(&syn::Variant, crate::item::Item)]) -> Result<proc_macro2::TokenStream, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/value_enum.rs:37-80`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/value_enum.rs#L37-L80)*

### `lits`

```rust
fn lits(variants: &[(&syn::Variant, crate::item::Item)]) -> Result<Vec<(proc_macro2::TokenStream, syn::Ident)>, syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/derives/value_enum.rs:82-104`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/value_enum.rs#L82-L104)*

### `gen_value_variants`

```rust
fn gen_value_variants(lits: &[(proc_macro2::TokenStream, syn::Ident)]) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/derives/value_enum.rs:106-114`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/value_enum.rs#L106-L114)*

### `gen_to_possible_value`

```rust
fn gen_to_possible_value(item: &crate::item::Item, lits: &[(proc_macro2::TokenStream, syn::Ident)]) -> proc_macro2::TokenStream
```

*Defined in [`clap_derive-4.5.49/src/derives/value_enum.rs:116-130`](../../../../.source_1765210505/clap_derive-4.5.49/src/derives/value_enum.rs#L116-L130)*

