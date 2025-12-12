*[syn](../../index.md) / [expr](../index.md) / [printing](index.md)*

---

# Module `printing`

## Contents

- [Functions](#functions)
  - [`outer_attrs_to_tokens`](#outer-attrs-to-tokens)
  - [`inner_attrs_to_tokens`](#inner-attrs-to-tokens)
  - [`print_subexpression`](#print-subexpression)
  - [`print_expr`](#print-expr)
  - [`print_expr_assign`](#print-expr-assign)
  - [`print_expr_await`](#print-expr-await)
  - [`print_expr_binary`](#print-expr-binary)
  - [`print_expr_break`](#print-expr-break)
  - [`print_expr_call`](#print-expr-call)
  - [`print_expr_cast`](#print-expr-cast)
  - [`print_expr_closure`](#print-expr-closure)
  - [`print_expr_field`](#print-expr-field)
  - [`print_expr_index`](#print-expr-index)
  - [`print_expr_let`](#print-expr-let)
  - [`print_expr_method_call`](#print-expr-method-call)
  - [`print_expr_range`](#print-expr-range)
  - [`print_expr_raw_addr`](#print-expr-raw-addr)
  - [`print_expr_reference`](#print-expr-reference)
  - [`print_expr_return`](#print-expr-return)
  - [`print_expr_try`](#print-expr-try)
  - [`print_expr_unary`](#print-expr-unary)
  - [`print_expr_yield`](#print-expr-yield)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`outer_attrs_to_tokens`](#outer-attrs-to-tokens) | fn |  |
| [`inner_attrs_to_tokens`](#inner-attrs-to-tokens) | fn |  |
| [`print_subexpression`](#print-subexpression) | fn |  |
| [`print_expr`](#print-expr) | fn |  |
| [`print_expr_assign`](#print-expr-assign) | fn |  |
| [`print_expr_await`](#print-expr-await) | fn |  |
| [`print_expr_binary`](#print-expr-binary) | fn |  |
| [`print_expr_break`](#print-expr-break) | fn |  |
| [`print_expr_call`](#print-expr-call) | fn |  |
| [`print_expr_cast`](#print-expr-cast) | fn |  |
| [`print_expr_closure`](#print-expr-closure) | fn |  |
| [`print_expr_field`](#print-expr-field) | fn |  |
| [`print_expr_index`](#print-expr-index) | fn |  |
| [`print_expr_let`](#print-expr-let) | fn |  |
| [`print_expr_method_call`](#print-expr-method-call) | fn |  |
| [`print_expr_range`](#print-expr-range) | fn |  |
| [`print_expr_raw_addr`](#print-expr-raw-addr) | fn |  |
| [`print_expr_reference`](#print-expr-reference) | fn |  |
| [`print_expr_return`](#print-expr-return) | fn |  |
| [`print_expr_try`](#print-expr-try) | fn |  |
| [`print_expr_unary`](#print-expr-unary) | fn |  |
| [`print_expr_yield`](#print-expr-yield) | fn |  |

## Functions

### `outer_attrs_to_tokens`

```rust
fn outer_attrs_to_tokens(attrs: &[crate::attr::Attribute], tokens: &mut proc_macro2::TokenStream)
```

*Defined in [`syn-2.0.111/src/expr.rs:3151-3153`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3151-L3153)*

### `inner_attrs_to_tokens`

```rust
fn inner_attrs_to_tokens(attrs: &[crate::attr::Attribute], tokens: &mut proc_macro2::TokenStream)
```

*Defined in [`syn-2.0.111/src/expr.rs:3156-3158`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3156-L3158)*

### `print_subexpression`

```rust
fn print_subexpression(expr: &crate::expr::Expr, needs_group: bool, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3163-3191`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3163-L3191)*

### `print_expr`

```rust
fn print_expr(expr: &crate::expr::Expr, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3193-3278`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3193-L3278)*

### `print_expr_assign`

```rust
fn print_expr_assign(e: &crate::expr::ExprAssign, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3300-3329`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3300-L3329)*

### `print_expr_await`

```rust
fn print_expr_await(e: &crate::expr::ExprAwait, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3351-3362`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3351-L3362)*

### `print_expr_binary`

```rust
fn print_expr_binary(e: &crate::expr::ExprBinary, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3371-3429`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3371-L3429)*

### `print_expr_break`

```rust
fn print_expr_break(e: &crate::expr::ExprBreak, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3453-3467`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3453-L3467)*

### `print_expr_call`

```rust
fn print_expr_call(e: &crate::expr::ExprCall, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3476-3497`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3476-L3497)*

### `print_expr_cast`

```rust
fn print_expr_cast(e: &crate::expr::ExprCast, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3506-3533`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3506-L3533)*

### `print_expr_closure`

```rust
fn print_expr_closure(e: &crate::expr::ExprClosure, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3544-3568`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3544-L3568)*

### `print_expr_field`

```rust
fn print_expr_field(e: &crate::expr::ExprField, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3600-3611`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3600-L3611)*

### `print_expr_index`

```rust
fn print_expr_index(e: &crate::expr::ExprIndex, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3686-3705`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3686-L3705)*

### `print_expr_let`

```rust
fn print_expr_let(e: &crate::expr::ExprLet, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3725-3732`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3725-L3732)*

### `print_expr_method_call`

```rust
fn print_expr_method_call(e: &crate::expr::ExprMethodCall, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3796-3817`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3796-L3817)*

### `print_expr_range`

```rust
fn print_expr_range(e: &crate::expr::ExprRange, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3846-3878`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3846-L3878)*

### `print_expr_raw_addr`

```rust
fn print_expr_raw_addr(e: &crate::expr::ExprRawAddr, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3889-3901`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3889-L3901)*

### `print_expr_reference`

```rust
fn print_expr_reference(e: &crate::expr::ExprReference, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3910-3925`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3910-L3925)*

### `print_expr_return`

```rust
fn print_expr_return(e: &crate::expr::ExprReturn, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3949-3959`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3949-L3959)*

### `print_expr_try`

```rust
fn print_expr_try(e: &crate::expr::ExprTry, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:3987-3997`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L3987-L3997)*

### `print_expr_unary`

```rust
fn print_expr_unary(e: &crate::expr::ExprUnary, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:4031-4045`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L4031-L4045)*

### `print_expr_yield`

```rust
fn print_expr_yield(e: &crate::expr::ExprYield, tokens: &mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```

*Defined in [`syn-2.0.111/src/expr.rs:4084-4094`](../../../../.source_1765521767/syn-2.0.111/src/expr.rs#L4084-L4094)*

