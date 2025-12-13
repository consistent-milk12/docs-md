*[serde_derive](../../index.md) / [de](../index.md) / [enum_internally](index.md)*

---

# Module `enum_internally`

Deserialization for internally tagged enums:

```ignore
#[serde(tag = "...")]
enum Enum {}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for an `enum Enum {...}` with `#[serde(tag)]` attribute |
| [`deserialize_internally_tagged_variant`](#deserialize-internally-tagged-variant) | fn |  |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container, tag: &str) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_internally.rs:21-63`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_internally.rs#L21-L63)*

Generates `Deserialize::deserialize` body for an `enum Enum {...}` with `#[serde(tag)]` attribute

### `deserialize_internally_tagged_variant`

```rust
fn deserialize_internally_tagged_variant(params: &crate::de::Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_internally.rs:67-106`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_internally.rs#L67-L106)*

