*[serde_derive](../../index.md) / [de](../index.md) / [enum_](index.md)*

---

# Module `enum_`

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for an `enum Enum {...}`

### `deserialize_homogeneous_enum`

```rust
fn deserialize_homogeneous_enum(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `prepare_enum_variant_enum`

```rust
fn prepare_enum_variant_enum(variants: &[crate::internals::ast::Variant<'_>]) -> (proc_macro2::TokenStream, crate::fragment::Stmts)
```

