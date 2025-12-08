*[syn](../index.md) / [classify](index.md)*

---

# Module `classify`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`requires_semi_to_be_stmt`](#requires_semi_to_be_stmt) | fn |  |
| [`requires_comma_to_be_match_arm`](#requires_comma_to_be_match_arm) | fn |  |
| [`trailing_unparameterized_path`](#trailing_unparameterized_path) | fn |  |
| [`expr_leading_label`](#expr_leading_label) | fn | Whether the expression's first token is the label of a loop/block. |
| [`expr_trailing_brace`](#expr_trailing_brace) | fn | Whether the expression's last token is `}`. |

## Functions

### `requires_semi_to_be_stmt`

```rust
fn requires_semi_to_be_stmt(expr: &crate::expr::Expr) -> bool
```

### `requires_comma_to_be_match_arm`

```rust
fn requires_comma_to_be_match_arm(expr: &crate::expr::Expr) -> bool
```

### `trailing_unparameterized_path`

```rust
fn trailing_unparameterized_path(ty: &crate::ty::Type) -> bool
```

### `expr_leading_label`

```rust
fn expr_leading_label(expr: &crate::expr::Expr) -> bool
```

Whether the expression's first token is the label of a loop/block.

### `expr_trailing_brace`

```rust
fn expr_trailing_brace(expr: &crate::expr::Expr) -> bool
```

Whether the expression's last token is `}`.

