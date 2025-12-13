*[syn](../../index.md) / [ty](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ambig_ty`](#ambig-ty) | fn |  |
| [`parse_bare_fn_arg`](#parse-bare-fn-arg) | fn |  |
| [`parse_bare_variadic`](#parse-bare-variadic) | fn |  |

## Functions

### `ambig_ty`

```rust
fn ambig_ty(input: crate::parse::ParseStream<'_>, allow_plus: bool, allow_group_generic: bool) -> crate::error::Result<crate::ty::Type>
```

*Defined in [`syn-2.0.111/src/ty.rs:318-611`](../../../../.source_1765633015/syn-2.0.111/src/ty.rs#L318-L611)*

### `parse_bare_fn_arg`

```rust
fn parse_bare_fn_arg(input: crate::parse::ParseStream<'_>, allow_self: bool) -> crate::error::Result<crate::ty::BareFnArg>
```

*Defined in [`syn-2.0.111/src/ty.rs:986-1032`](../../../../.source_1765633015/syn-2.0.111/src/ty.rs#L986-L1032)*

### `parse_bare_variadic`

```rust
fn parse_bare_variadic(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>) -> crate::error::Result<crate::ty::BareVariadic>
```

*Defined in [`syn-2.0.111/src/ty.rs:1034-1047`](../../../../.source_1765633015/syn-2.0.111/src/ty.rs#L1034-L1047)*

