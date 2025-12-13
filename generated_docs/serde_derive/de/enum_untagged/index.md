*[serde_derive](../../index.md) / [de](../index.md) / [enum_untagged](index.md)*

---

# Module `enum_untagged`

Deserialization for untagged enums:

```ignore
#[serde(untagged)]
enum Enum {}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for an `enum Enum {...}` with `#[serde(untagged)]` attribute |
| [`deserialize_variant`](#deserialize-variant) | fn |  |
| [`deserialize_newtype_variant`](#deserialize-newtype-variant) | fn |  |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container, first_attempt: Option<proc_macro2::TokenStream>) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_untagged.rs:22-59`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_untagged.rs#L22-L59)*

Generates `Deserialize::deserialize` body for an `enum Enum {...}` with `#[serde(untagged)]` attribute

### `deserialize_variant`

```rust
fn deserialize_variant(params: &crate::de::Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_untagged.rs:62-109`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_untagged.rs#L62-L109)*

### `deserialize_newtype_variant`

```rust
fn deserialize_newtype_variant(variant_ident: &syn::Ident, params: &crate::de::Parameters, field: &crate::internals::ast::Field<'_>) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_untagged.rs:113-135`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_untagged.rs#L113-L135)*

