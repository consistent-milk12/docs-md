*[serde_derive](../../index.md) / [de](../index.md) / [enum_externally](index.md)*

---

# Module `enum_externally`

Deserialization for externally tagged enums:

```ignore
enum Enum {}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for an `enum Enum {...}` without additional attributes |
| [`deserialize_externally_tagged_variant`](#deserialize_externally_tagged_variant) | fn |  |
| [`wrap_deserialize_variant_with`](#wrap_deserialize_variant_with) | fn |  |
| [`deserialize_externally_tagged_newtype_variant`](#deserialize_externally_tagged_newtype_variant) | fn |  |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for an `enum Enum {...}` without additional attributes

### `deserialize_externally_tagged_variant`

```rust
fn deserialize_externally_tagged_variant(params: &crate::de::Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `wrap_deserialize_variant_with`

```rust
fn wrap_deserialize_variant_with(params: &crate::de::Parameters, variant: &crate::internals::ast::Variant<'_>, deserialize_with: &syn::ExprPath) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream)
```

### `deserialize_externally_tagged_newtype_variant`

```rust
fn deserialize_externally_tagged_newtype_variant(variant_ident: &syn::Ident, params: &crate::de::Parameters, field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

