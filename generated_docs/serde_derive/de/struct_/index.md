*[serde_derive](../../index.md) / [de](../index.md) / [struct_](index.md)*

---

# Module `struct_`

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container, form: crate::de::StructForm<'_>) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for a `struct Struct {...}`

### `deserialize_map`

```rust
fn deserialize_map(struct_path: &proc_macro2::TokenStream, params: &crate::de::Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container, has_flatten: bool) -> crate::fragment::Fragment
```

### `deserialize_field_identifier`

```rust
fn deserialize_field_identifier(deserialized_fields: &[crate::de::FieldWithAliases<'_>], cattrs: &attr::Container, has_flatten: bool) -> crate::fragment::Stmts
```

Generates enum and its `Deserialize` implementation that represents each
non-skipped field of the struct

