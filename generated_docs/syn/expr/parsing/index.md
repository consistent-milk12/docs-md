*[syn](../../index.md) / [expr](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Structs](#structs)
  - [`AllowStruct`](#allowstruct)
- [Functions](#functions)
  - [`parse_with_earlier_boundary_rule`](#parse_with_earlier_boundary_rule)
  - [`parse_expr`](#parse_expr)
  - [`parse_binop_rhs`](#parse_binop_rhs)
  - [`peek_precedence`](#peek_precedence)
  - [`ambiguous_expr`](#ambiguous_expr)
  - [`expr_attrs`](#expr_attrs)
  - [`unary_expr`](#unary_expr)
  - [`trailer_expr`](#trailer_expr)
  - [`trailer_helper`](#trailer_helper)
  - [`atom_expr`](#atom_expr)
  - [`atom_labeled`](#atom_labeled)
  - [`expr_builtin`](#expr_builtin)
  - [`path_or_macro_or_struct`](#path_or_macro_or_struct)
  - [`rest_of_path_or_macro_or_struct`](#rest_of_path_or_macro_or_struct)
  - [`paren_or_tuple`](#paren_or_tuple)
  - [`array_or_repeat`](#array_or_repeat)
  - [`continue_parsing_early`](#continue_parsing_early)
  - [`expr_group`](#expr_group)
  - [`expr_let`](#expr_let)
  - [`expr_unary`](#expr_unary)
  - [`expr_become`](#expr_become)
  - [`expr_closure`](#expr_closure)
  - [`closure_arg`](#closure_arg)
  - [`expr_break`](#expr_break)
  - [`expr_struct_helper`](#expr_struct_helper)
  - [`expr_range`](#expr_range)
  - [`parse_range_end`](#parse_range_end)
  - [`multi_index`](#multi_index)
  - [`check_cast`](#check_cast)
- [Macros](#macros)
  - [`impl_by_parsing_expr!`](#impl_by_parsing_expr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AllowStruct`](#allowstruct) | struct |  |
| [`parse_with_earlier_boundary_rule`](#parse_with_earlier_boundary_rule) | fn |  |
| [`parse_expr`](#parse_expr) | fn |  |
| [`parse_binop_rhs`](#parse_binop_rhs) | fn |  |
| [`peek_precedence`](#peek_precedence) | fn |  |
| [`ambiguous_expr`](#ambiguous_expr) | fn |  |
| [`expr_attrs`](#expr_attrs) | fn |  |
| [`unary_expr`](#unary_expr) | fn |  |
| [`trailer_expr`](#trailer_expr) | fn |  |
| [`trailer_helper`](#trailer_helper) | fn |  |
| [`atom_expr`](#atom_expr) | fn |  |
| [`atom_labeled`](#atom_labeled) | fn |  |
| [`expr_builtin`](#expr_builtin) | fn |  |
| [`path_or_macro_or_struct`](#path_or_macro_or_struct) | fn |  |
| [`rest_of_path_or_macro_or_struct`](#rest_of_path_or_macro_or_struct) | fn |  |
| [`paren_or_tuple`](#paren_or_tuple) | fn |  |
| [`array_or_repeat`](#array_or_repeat) | fn |  |
| [`continue_parsing_early`](#continue_parsing_early) | fn |  |
| [`expr_group`](#expr_group) | fn |  |
| [`expr_let`](#expr_let) | fn |  |
| [`expr_unary`](#expr_unary) | fn |  |
| [`expr_become`](#expr_become) | fn |  |
| [`expr_closure`](#expr_closure) | fn |  |
| [`closure_arg`](#closure_arg) | fn |  |
| [`expr_break`](#expr_break) | fn |  |
| [`expr_struct_helper`](#expr_struct_helper) | fn |  |
| [`expr_range`](#expr_range) | fn |  |
| [`parse_range_end`](#parse_range_end) | fn |  |
| [`multi_index`](#multi_index) | fn |  |
| [`check_cast`](#check_cast) | fn |  |
| [`impl_by_parsing_expr!`](#impl_by_parsing_expr) | macro |  |

## Structs

### `AllowStruct`

```rust
struct AllowStruct(bool);
```

#### Trait Implementations

##### `impl Clone for AllowStruct`

- <span id="allowstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for AllowStruct`

## Functions

### `parse_with_earlier_boundary_rule`

```rust
fn parse_with_earlier_boundary_rule(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `parse_expr`

```rust
fn parse_expr(input: crate::parse::ParseStream<'_>, lhs: crate::expr::Expr, allow_struct: AllowStruct, base: crate::precedence::Precedence) -> crate::error::Result<crate::expr::Expr>
```

### `parse_binop_rhs`

```rust
fn parse_binop_rhs(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct, precedence: crate::precedence::Precedence) -> crate::error::Result<Box<crate::expr::Expr>>
```

### `peek_precedence`

```rust
fn peek_precedence(input: crate::parse::ParseStream<'_>) -> crate::precedence::Precedence
```

### `ambiguous_expr`

```rust
fn ambiguous_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `expr_attrs`

```rust
fn expr_attrs(input: crate::parse::ParseStream<'_>) -> crate::error::Result<Vec<crate::attr::Attribute>>
```

### `unary_expr`

```rust
fn unary_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `trailer_expr`

```rust
fn trailer_expr(begin: crate::parse::ParseBuffer<'_>, attrs: Vec<crate::attr::Attribute>, input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `trailer_helper`

```rust
fn trailer_helper(input: crate::parse::ParseStream<'_>, e: crate::expr::Expr) -> crate::error::Result<crate::expr::Expr>
```

### `atom_expr`

```rust
fn atom_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `atom_labeled`

```rust
fn atom_labeled(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `expr_builtin`

```rust
fn expr_builtin(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `path_or_macro_or_struct`

```rust
fn path_or_macro_or_struct(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `rest_of_path_or_macro_or_struct`

```rust
fn rest_of_path_or_macro_or_struct(qself: Option<crate::path::QSelf>, path: crate::path::Path, input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `paren_or_tuple`

```rust
fn paren_or_tuple(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `array_or_repeat`

```rust
fn array_or_repeat(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `continue_parsing_early`

```rust
fn continue_parsing_early(expr: &crate::expr::Expr) -> bool
```

### `expr_group`

```rust
fn expr_group(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `expr_let`

```rust
fn expr_let(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprLet>
```

### `expr_unary`

```rust
fn expr_unary(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprUnary>
```

### `expr_become`

```rust
fn expr_become(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `expr_closure`

```rust
fn expr_closure(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprClosure>
```

### `closure_arg`

```rust
fn closure_arg(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

### `expr_break`

```rust
fn expr_break(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprBreak>
```

### `expr_struct_helper`

```rust
fn expr_struct_helper(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::expr::ExprStruct>
```

### `expr_range`

```rust
fn expr_range(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprRange>
```

### `parse_range_end`

```rust
fn parse_range_end(input: crate::parse::ParseStream<'_>, limits: &crate::expr::RangeLimits, allow_struct: AllowStruct) -> crate::error::Result<Option<Box<crate::expr::Expr>>>
```

### `multi_index`

```rust
fn multi_index(e: &mut crate::expr::Expr, dot_token: &mut token::Dot, float: crate::lit::LitFloat) -> crate::error::Result<bool>
```

### `check_cast`

```rust
fn check_cast(input: crate::parse::ParseStream<'_>) -> crate::error::Result<()>
```

## Macros

### `impl_by_parsing_expr!`

