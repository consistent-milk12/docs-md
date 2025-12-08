*[serde_derive](../../index.md) / [internals](../index.md) / [check](index.md)*

---

# Module `check`

## Contents

- [Functions](#functions)
  - [`check`](#check)
  - [`check_default_on_tuple`](#check_default_on_tuple)
  - [`check_remote_generic`](#check_remote_generic)
  - [`check_getter`](#check_getter)
  - [`check_flatten`](#check_flatten)
  - [`check_flatten_field`](#check_flatten_field)
  - [`check_identifier`](#check_identifier)
  - [`check_variant_skip_attrs`](#check_variant_skip_attrs)
  - [`check_internal_tag_field_name_conflict`](#check_internal_tag_field_name_conflict)
  - [`check_adjacent_tag_conflict`](#check_adjacent_tag_conflict)
  - [`check_transparent`](#check_transparent)
  - [`member_message`](#member_message)
  - [`allow_transparent`](#allow_transparent)
  - [`check_from_and_try_from`](#check_from_and_try_from)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`check`](#check) | fn |  |
| [`check_default_on_tuple`](#check_default_on_tuple) | fn |  |
| [`check_remote_generic`](#check_remote_generic) | fn |  |
| [`check_getter`](#check_getter) | fn |  |
| [`check_flatten`](#check_flatten) | fn |  |
| [`check_flatten_field`](#check_flatten_field) | fn |  |
| [`check_identifier`](#check_identifier) | fn |  |
| [`check_variant_skip_attrs`](#check_variant_skip_attrs) | fn |  |
| [`check_internal_tag_field_name_conflict`](#check_internal_tag_field_name_conflict) | fn |  |
| [`check_adjacent_tag_conflict`](#check_adjacent_tag_conflict) | fn |  |
| [`check_transparent`](#check_transparent) | fn |  |
| [`member_message`](#member_message) | fn |  |
| [`allow_transparent`](#allow_transparent) | fn |  |
| [`check_from_and_try_from`](#check_from_and_try_from) | fn |  |

## Functions

### `check`

```rust
fn check(cx: &crate::internals::Ctxt, cont: &mut crate::internals::ast::Container<'_>, derive: crate::internals::Derive)
```

### `check_default_on_tuple`

```rust
fn check_default_on_tuple(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_remote_generic`

```rust
fn check_remote_generic(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_getter`

```rust
fn check_getter(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_flatten`

```rust
fn check_flatten(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_flatten_field`

```rust
fn check_flatten_field(cx: &crate::internals::Ctxt, style: crate::internals::ast::Style, field: &crate::internals::ast::Field<'_>)
```

### `check_identifier`

```rust
fn check_identifier(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_variant_skip_attrs`

```rust
fn check_variant_skip_attrs(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_internal_tag_field_name_conflict`

```rust
fn check_internal_tag_field_name_conflict(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_adjacent_tag_conflict`

```rust
fn check_adjacent_tag_conflict(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `check_transparent`

```rust
fn check_transparent(cx: &crate::internals::Ctxt, cont: &mut crate::internals::ast::Container<'_>, derive: crate::internals::Derive)
```

### `member_message`

```rust
fn member_message(member: &syn::Member) -> String
```

### `allow_transparent`

```rust
fn allow_transparent(field: &crate::internals::ast::Field<'_>, derive: crate::internals::Derive) -> bool
```

### `check_from_and_try_from`

```rust
fn check_from_and_try_from(cx: &crate::internals::Ctxt, cont: &mut crate::internals::ast::Container<'_>)
```

