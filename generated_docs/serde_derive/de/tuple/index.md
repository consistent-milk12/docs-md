*[serde_derive](../../index.md) / [de](../index.md) / [tuple](index.md)*

---

# Module `tuple`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for a `struct Tuple(...);` including `struct Newtype(T);` |
| [`deserialize_newtype_struct`](#deserialize_newtype_struct) | fn |  |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container, form: crate::de::TupleForm<'_>) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for a `struct Tuple(...);` including `struct Newtype(T);`

### `deserialize_newtype_struct`

```rust
fn deserialize_newtype_struct(type_path: &proc_macro2::TokenStream, params: &crate::de::Parameters, field: &crate::internals::ast::Field<'_>) -> proc_macro2::TokenStream
```

