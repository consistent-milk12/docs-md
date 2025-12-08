*[syn](../../index.md) / [ty](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ambig_ty`](#ambig_ty) | fn |  |
| [`parse_bare_fn_arg`](#parse_bare_fn_arg) | fn |  |
| [`parse_bare_variadic`](#parse_bare_variadic) | fn |  |

## Functions

### `ambig_ty`

```rust
fn ambig_ty(input: crate::parse::ParseStream<'_>, allow_plus: bool, allow_group_generic: bool) -> crate::error::Result<crate::ty::Type>
```

### `parse_bare_fn_arg`

```rust
fn parse_bare_fn_arg(input: crate::parse::ParseStream<'_>, allow_self: bool) -> crate::error::Result<crate::ty::BareFnArg>
```

### `parse_bare_variadic`

```rust
fn parse_bare_variadic(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::ty::BareVariadic>
```

