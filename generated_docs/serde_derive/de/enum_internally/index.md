*[serde_derive](../../index.md) / [de](../index.md) / [enum_internally](index.md)*

---

# Module `enum_internally`

Deserialization for internally tagged enums:

```ignore
#[serde(tag = "...")]
enum Enum {}
```

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container, tag: &str) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for an `enum Enum {...}` with `#[serde(tag)]` attribute

### `deserialize_internally_tagged_variant`

```rust
fn deserialize_internally_tagged_variant(params: &crate::de::Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

