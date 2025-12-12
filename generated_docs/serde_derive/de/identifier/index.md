*[serde_derive](../../index.md) / [de](../index.md) / [identifier](index.md)*

---

# Module `identifier`

Deserialization of struct field identifiers and enum variant identifiers by
way of a Rust enum.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize_custom`](#deserialize-custom) | fn |  |
| [`deserialize_generated`](#deserialize-generated) | fn |  |
| [`deserialize_identifier`](#deserialize-identifier) | fn |  |

## Functions

### `deserialize_custom`

```rust
fn deserialize_custom(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/identifier.rs:14-123`](../../../../.source_1765210505/serde_derive-1.0.228/src/de/identifier.rs#L14-L123)*

### `deserialize_generated`

```rust
fn deserialize_generated(deserialized_fields: &[crate::de::FieldWithAliases<'_>], has_flatten: bool, is_variant: bool, ignore_variant: Option<proc_macro2::TokenStream>, fallthrough: Option<proc_macro2::TokenStream>) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/identifier.rs:125-183`](../../../../.source_1765210505/serde_derive-1.0.228/src/de/identifier.rs#L125-L183)*

### `deserialize_identifier`

```rust
fn deserialize_identifier(this_value: &proc_macro2::TokenStream, deserialized_fields: &[crate::de::FieldWithAliases<'_>], is_variant: bool, fallthrough: Option<proc_macro2::TokenStream>, fallthrough_borrowed: Option<proc_macro2::TokenStream>, collect_other_fields: bool, expecting: Option<&str>) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/identifier.rs:185-477`](../../../../.source_1765210505/serde_derive-1.0.228/src/de/identifier.rs#L185-L477)*

