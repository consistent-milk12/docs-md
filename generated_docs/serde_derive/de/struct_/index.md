*[serde_derive](../../index.md) / [de](../index.md) / [struct_](index.md)*

---

# Module `struct_`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for a `struct Struct {...}` |
| [`deserialize_map`](#deserialize-map) | fn |  |
| [`deserialize_field_identifier`](#deserialize-field-identifier) | fn | Generates enum and its `Deserialize` implementation that represents each non-skipped field of the struct |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container, form: crate::de::StructForm<'_>) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/struct_.rs:17-197`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/struct_.rs#L17-L197)*

Generates `Deserialize::deserialize` body for a `struct Struct {...}`

### `deserialize_map`

```rust
fn deserialize_map(struct_path: &proc_macro2::TokenStream, params: &crate::de::Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container, has_flatten: bool) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/struct_.rs:199-419`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/struct_.rs#L199-L419)*

### `deserialize_field_identifier`

```rust
fn deserialize_field_identifier(deserialized_fields: &[crate::de::FieldWithAliases<'_>], cattrs: &attr::Container, has_flatten: bool) -> crate::fragment::Stmts
```

*Defined in [`serde_derive-1.0.228/src/de/struct_.rs:673-697`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/struct_.rs#L673-L697)*

Generates enum and its `Deserialize` implementation that represents each
non-skipped field of the struct

