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
| [`deserialize_externally_tagged_variant`](#deserialize-externally-tagged-variant) | fn |  |
| [`wrap_deserialize_variant_with`](#wrap-deserialize-variant-with) | fn |  |
| [`deserialize_externally_tagged_newtype_variant`](#deserialize-externally-tagged-newtype-variant) | fn |  |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_externally.rs:23-115`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/enum_externally.rs#L23-L115)*

Generates `Deserialize::deserialize` body for an `enum Enum {...}` without additional attributes

### `deserialize_externally_tagged_variant`

```rust
fn deserialize_externally_tagged_variant(params: &crate::de::Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_externally.rs:117-160`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/enum_externally.rs#L117-L160)*

### `wrap_deserialize_variant_with`

```rust
fn wrap_deserialize_variant_with(params: &crate::de::Parameters, variant: &crate::internals::ast::Variant<'_>, deserialize_with: &syn::ExprPath) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream)
```

*Defined in [`serde_derive-1.0.228/src/de/enum_externally.rs:162-174`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/enum_externally.rs#L162-L174)*

### `deserialize_externally_tagged_newtype_variant`

```rust
fn deserialize_externally_tagged_newtype_variant(variant_ident: &syn::Ident, params: &crate::de::Parameters, field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_externally.rs:176-212`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/enum_externally.rs#L176-L212)*

