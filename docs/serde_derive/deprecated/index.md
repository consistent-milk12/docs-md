*[serde_derive](../index.md) / [deprecated](index.md)*

---

# Module `deprecated`

## Functions

### `allow_deprecated`

```rust
fn allow_deprecated(input: &syn::DeriveInput) -> Option<proc_macro2::TokenStream>
```

### `should_allow_deprecated`

```rust
fn should_allow_deprecated(input: &syn::DeriveInput) -> bool
```

Determine if an `#[allow(deprecated)]` should be added to the derived impl.

This should happen if the derive input or an enum variant it contains has
one of:
  - `#[deprecated]`
  - `#[allow(deprecated)]`

### `contains_deprecated`

```rust
fn contains_deprecated(attrs: &[syn::Attribute]) -> bool
```

Check whether the given attributes contains one of:
  - `#[deprecated]`
  - `#[allow(deprecated)]`

