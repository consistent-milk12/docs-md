*[syn](../index.md) / [classify](index.md)*

---

# Module `classify`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`requires_semi_to_be_stmt`](#requires-semi-to-be-stmt) | fn |  |
| [`requires_comma_to_be_match_arm`](#requires-comma-to-be-match-arm) | fn |  |
| [`trailing_unparameterized_path`](#trailing-unparameterized-path) | fn |  |
| [`expr_leading_label`](#expr-leading-label) | fn | Whether the expression's first token is the label of a loop/block. |
| [`expr_trailing_brace`](#expr-trailing-brace) | fn | Whether the expression's last token is `}`. |

## Functions

### `requires_semi_to_be_stmt`

```rust
fn requires_semi_to_be_stmt(expr: &crate::expr::Expr) -> bool
```

*Defined in [`syn-2.0.111/src/classify.rs:17-22`](../../../.source_1765633015/syn-2.0.111/src/classify.rs#L17-L22)*

### `requires_comma_to_be_match_arm`

```rust
fn requires_comma_to_be_match_arm(expr: &crate::expr::Expr) -> bool
```

*Defined in [`syn-2.0.111/src/classify.rs:25-68`](../../../.source_1765633015/syn-2.0.111/src/classify.rs#L25-L68)*

### `trailing_unparameterized_path`

```rust
fn trailing_unparameterized_path(ty: &crate::ty::Type) -> bool
```

*Defined in [`syn-2.0.111/src/classify.rs:71-126`](../../../.source_1765633015/syn-2.0.111/src/classify.rs#L71-L126)*

### `expr_leading_label`

```rust
fn expr_leading_label(expr: &crate::expr::Expr) -> bool
```

*Defined in [`syn-2.0.111/src/classify.rs:130-180`](../../../.source_1765633015/syn-2.0.111/src/classify.rs#L130-L180)*

Whether the expression's first token is the label of a loop/block.

### `expr_trailing_brace`

```rust
fn expr_trailing_brace(expr: &crate::expr::Expr) -> bool
```

*Defined in [`syn-2.0.111/src/classify.rs:184-311`](../../../.source_1765633015/syn-2.0.111/src/classify.rs#L184-L311)*

Whether the expression's last token is `}`.

