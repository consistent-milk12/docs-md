*[serde_derive](../../index.md) / [de](../index.md) / [enum_adjacently](index.md)*

---

# Module `enum_adjacently`

Deserialization for adjacently tagged enums:

```ignore
#[serde(tag = "...", content = "...")]
enum Enum {}
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for an `enum Enum {...}` with `#[serde(tag, content)]` attributes |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container, tag: &str, content: &str) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_adjacently.rs:19-323`](../../../../.source_1765210505/serde_derive-1.0.228/src/de/enum_adjacently.rs#L19-L323)*

Generates `Deserialize::deserialize` body for an `enum Enum {...}` with `#[serde(tag, content)]` attributes

