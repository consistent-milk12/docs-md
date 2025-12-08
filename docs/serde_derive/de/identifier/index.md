*[serde_derive](../../index.md) / [de](../index.md) / [identifier](index.md)*

---

# Module `identifier`

Deserialization of struct field identifiers and enum variant identifiers by
way of a Rust enum.

## Functions

### `deserialize_custom`

```rust
fn deserialize_custom(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `deserialize_generated`

```rust
fn deserialize_generated(deserialized_fields: &[crate::de::FieldWithAliases<'_>], has_flatten: bool, is_variant: bool, ignore_variant: Option<proc_macro2::TokenStream>, fallthrough: Option<proc_macro2::TokenStream>) -> crate::fragment::Fragment
```

### `deserialize_identifier`

```rust
fn deserialize_identifier(this_value: &proc_macro2::TokenStream, deserialized_fields: &[crate::de::FieldWithAliases<'_>], is_variant: bool, fallthrough: Option<proc_macro2::TokenStream>, fallthrough_borrowed: Option<proc_macro2::TokenStream>, collect_other_fields: bool, expecting: Option<&str>) -> crate::fragment::Fragment
```

