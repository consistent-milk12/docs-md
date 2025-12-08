*[syn](../../index.md) / [expr](../index.md) / [printing](index.md)*

---

# Module `printing`

## Contents

- [Functions](#functions)
  - [`outer_attrs_to_tokens`](#outer_attrs_to_tokens)
  - [`inner_attrs_to_tokens`](#inner_attrs_to_tokens)
  - [`print_subexpression`](#print_subexpression)
  - [`print_expr`](#print_expr)
  - [`print_expr_assign`](#print_expr_assign)
  - [`print_expr_await`](#print_expr_await)
  - [`print_expr_binary`](#print_expr_binary)
  - [`print_expr_break`](#print_expr_break)
  - [`print_expr_call`](#print_expr_call)
  - [`print_expr_cast`](#print_expr_cast)
  - [`print_expr_closure`](#print_expr_closure)
  - [`print_expr_field`](#print_expr_field)
  - [`print_expr_index`](#print_expr_index)
  - [`print_expr_let`](#print_expr_let)
  - [`print_expr_method_call`](#print_expr_method_call)
  - [`print_expr_range`](#print_expr_range)
  - [`print_expr_raw_addr`](#print_expr_raw_addr)
  - [`print_expr_reference`](#print_expr_reference)
  - [`print_expr_return`](#print_expr_return)
  - [`print_expr_try`](#print_expr_try)
  - [`print_expr_unary`](#print_expr_unary)
  - [`print_expr_yield`](#print_expr_yield)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`outer_attrs_to_tokens`](#outer_attrs_to_tokens) | fn |  |
| [`inner_attrs_to_tokens`](#inner_attrs_to_tokens) | fn |  |
| [`print_subexpression`](#print_subexpression) | fn |  |
| [`print_expr`](#print_expr) | fn |  |
| [`print_expr_assign`](#print_expr_assign) | fn |  |
| [`print_expr_await`](#print_expr_await) | fn |  |
| [`print_expr_binary`](#print_expr_binary) | fn |  |
| [`print_expr_break`](#print_expr_break) | fn |  |
| [`print_expr_call`](#print_expr_call) | fn |  |
| [`print_expr_cast`](#print_expr_cast) | fn |  |
| [`print_expr_closure`](#print_expr_closure) | fn |  |
| [`print_expr_field`](#print_expr_field) | fn |  |
| [`print_expr_index`](#print_expr_index) | fn |  |
| [`print_expr_let`](#print_expr_let) | fn |  |
| [`print_expr_method_call`](#print_expr_method_call) | fn |  |
| [`print_expr_range`](#print_expr_range) | fn |  |
| [`print_expr_raw_addr`](#print_expr_raw_addr) | fn |  |
| [`print_expr_reference`](#print_expr_reference) | fn |  |
| [`print_expr_return`](#print_expr_return) | fn |  |
| [`print_expr_try`](#print_expr_try) | fn |  |
| [`print_expr_unary`](#print_expr_unary) | fn |  |
| [`print_expr_yield`](#print_expr_yield) | fn |  |

## Functions

### `outer_attrs_to_tokens`

```rust
fn outer_attrs_to_tokens(attrs: &[crate::attr::Attribute], tokens: &mut proc_macro2::TokenStream)
```

### `inner_attrs_to_tokens`

```rust
fn inner_attrs_to_tokens(attrs: &[crate::attr::Attribute], tokens: &mut proc_macro2::TokenStream)
```

### `print_subexpression`

```rust
fn print_subexpression(expr: &crate::expr::Expr, needs_group: bool, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr`

```rust
fn print_expr(expr: &crate::expr::Expr, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_assign`

```rust
fn print_expr_assign(e: &crate::expr::ExprAssign, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_await`

```rust
fn print_expr_await(e: &crate::expr::ExprAwait, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_binary`

```rust
fn print_expr_binary(e: &crate::expr::ExprBinary, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_break`

```rust
fn print_expr_break(e: &crate::expr::ExprBreak, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_call`

```rust
fn print_expr_call(e: &crate::expr::ExprCall, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_cast`

```rust
fn print_expr_cast(e: &crate::expr::ExprCast, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_closure`

```rust
fn print_expr_closure(e: &crate::expr::ExprClosure, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_field`

```rust
fn print_expr_field(e: &crate::expr::ExprField, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_index`

```rust
fn print_expr_index(e: &crate::expr::ExprIndex, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_let`

```rust
fn print_expr_let(e: &crate::expr::ExprLet, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_method_call`

```rust
fn print_expr_method_call(e: &crate::expr::ExprMethodCall, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_range`

```rust
fn print_expr_range(e: &crate::expr::ExprRange, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_raw_addr`

```rust
fn print_expr_raw_addr(e: &crate::expr::ExprRawAddr, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_reference`

```rust
fn print_expr_reference(e: &crate::expr::ExprReference, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_return`

```rust
fn print_expr_return(e: &crate::expr::ExprReturn, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_try`

```rust
fn print_expr_try(e: &crate::expr::ExprTry, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_unary`

```rust
fn print_expr_unary(e: &crate::expr::ExprUnary, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

### `print_expr_yield`

```rust
fn print_expr_yield(e: &crate::expr::ExprYield, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

