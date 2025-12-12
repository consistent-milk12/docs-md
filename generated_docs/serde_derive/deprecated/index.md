*[serde_derive](../index.md) / [deprecated](index.md)*

---

# Module `deprecated`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`allow_deprecated`](#allow-deprecated) | fn |  |
| [`should_allow_deprecated`](#should-allow-deprecated) | fn | Determine if an `#[allow(deprecated)]` should be added to the derived impl. |
| [`contains_deprecated`](#contains-deprecated) | fn | Check whether the given attributes contains one of: - `#[deprecated]` - `#[allow(deprecated)]` |

## Functions

### `allow_deprecated`

```rust
fn allow_deprecated(input: &syn::DeriveInput) -> Option<proc_macro2::TokenStream>
```

*Defined in [`serde_derive-1.0.228/src/deprecated.rs:4-10`](../../../.source_1765210505/serde_derive-1.0.228/src/deprecated.rs#L4-L10)*

### `should_allow_deprecated`

```rust
fn should_allow_deprecated(input: &syn::DeriveInput) -> bool
```

*Defined in [`serde_derive-1.0.228/src/deprecated.rs:18-30`](../../../.source_1765210505/serde_derive-1.0.228/src/deprecated.rs#L18-L30)*

Determine if an `#[allow(deprecated)]` should be added to the derived impl.

This should happen if the derive input or an enum variant it contains has
one of:
  - `#[deprecated]`
  - `#[allow(deprecated)]`

### `contains_deprecated`

```rust
fn contains_deprecated(attrs: &[syn::Attribute]) -> bool
```

*Defined in [`serde_derive-1.0.228/src/deprecated.rs:35-56`](../../../.source_1765210505/serde_derive-1.0.228/src/deprecated.rs#L35-L56)*

Check whether the given attributes contains one of:
  - `#[deprecated]`
  - `#[allow(deprecated)]`

