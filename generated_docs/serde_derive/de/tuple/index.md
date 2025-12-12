*[serde_derive](../../index.md) / [de](../index.md) / [tuple](index.md)*

---

# Module `tuple`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for a `struct Tuple(...);` including `struct Newtype(T);` |
| [`deserialize_newtype_struct`](#deserialize-newtype-struct) | fn |  |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container, form: crate::de::TupleForm<'_>) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/tuple.rs:13-133`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/tuple.rs#L13-L133)*

Generates `Deserialize::deserialize` body for a `struct Tuple(...);` including `struct Newtype(T);`

### `deserialize_newtype_struct`

```rust
fn deserialize_newtype_struct(type_path: &proc_macro2::TokenStream, params: &crate::de::Parameters, field: &crate::internals::ast::Field<'_>) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/de/tuple.rs:135-182`](../../../../.source_1765521767/serde_derive-1.0.228/src/de/tuple.rs#L135-L182)*

