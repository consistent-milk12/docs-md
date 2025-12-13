*[serde_derive](../../index.md) / [de](../index.md) / [enum_](index.md)*

---

# Module `enum_`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deserialize`](#deserialize) | fn | Generates `Deserialize::deserialize` body for an `enum Enum {...}` |
| [`deserialize_homogeneous_enum`](#deserialize-homogeneous-enum) | fn |  |
| [`prepare_enum_variant_enum`](#prepare-enum-variant-enum) | fn |  |

## Functions

### `deserialize`

```rust
fn deserialize(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_.rs:15-37`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_.rs#L15-L37)*

Generates `Deserialize::deserialize` body for an `enum Enum {...}`

### `deserialize_homogeneous_enum`

```rust
fn deserialize_homogeneous_enum(params: &crate::de::Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de/enum_.rs:39-54`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_.rs#L39-L54)*

### `prepare_enum_variant_enum`

```rust
fn prepare_enum_variant_enum(variants: &[crate::internals::ast::Variant<'_>]) -> (proc_macro2::TokenStream, crate::fragment::Stmts)
```

*Defined in [`serde_derive-1.0.228/src/de/enum_.rs:56-96`](../../../../.source_1765633015/serde_derive-1.0.228/src/de/enum_.rs#L56-L96)*

