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

*Defined in [`syn-2.0.111/src/expr.rs:1225`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1225)*

#### Trait Implementations

##### `impl Clone for AllowStruct`

- <span id="allowstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for AllowStruct`

## Functions

### `parse_with_earlier_boundary_rule`

```rust
fn parse_with_earlier_boundary_rule(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1239-1297`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1239-L1297)*

### `parse_expr`

```rust
fn parse_expr(input: crate::parse::ParseStream<'_>, lhs: crate::expr::Expr, allow_struct: AllowStruct, base: crate::precedence::Precedence) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1310-1388`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1310-L1388)*

### `parse_binop_rhs`

```rust
fn parse_binop_rhs(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct, precedence: crate::precedence::Precedence) -> crate::error::Result<Box<crate::expr::Expr>>
```

*Defined in [`syn-2.0.111/src/expr.rs:1433-1466`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1433-L1466)*

### `peek_precedence`

```rust
fn peek_precedence(input: crate::parse::ParseStream<'_>) -> crate::precedence::Precedence
```

*Defined in [`syn-2.0.111/src/expr.rs:1468-1480`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1468-L1480)*

### `ambiguous_expr`

```rust
fn ambiguous_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1483-1499`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1483-L1499)*

### `expr_attrs`

```rust
fn expr_attrs(input: crate::parse::ParseStream<'_>) -> crate::error::Result<Vec<crate::attr::Attribute>>
```

*Defined in [`syn-2.0.111/src/expr.rs:1502-1508`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1502-L1508)*

### `unary_expr`

```rust
fn unary_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1515-1562`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1515-L1562)*

### `trailer_expr`

```rust
fn trailer_expr(begin: crate::parse::ParseBuffer<'_>, attrs: Vec<crate::attr::Attribute>, input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1591-1620`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1591-L1620)*

### `trailer_helper`

```rust
fn trailer_helper(input: crate::parse::ParseStream<'_>, e: crate::expr::Expr) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1623-1713`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1623-L1713)*

### `atom_expr`

```rust
fn atom_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1792-1866`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1792-L1866)*

### `atom_labeled`

```rust
fn atom_labeled(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1869-1890`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1869-L1890)*

### `expr_builtin`

```rust
fn expr_builtin(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1927-1939`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1927-L1939)*

### `path_or_macro_or_struct`

```rust
fn path_or_macro_or_struct(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1941-1954`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1941-L1954)*

### `rest_of_path_or_macro_or_struct`

```rust
fn rest_of_path_or_macro_or_struct(qself: Option<crate::path::QSelf>, path: crate::path::Path, input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:1956-1991`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L1956-L1991)*

### `paren_or_tuple`

```rust
fn paren_or_tuple(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:2003-2039`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2003-L2039)*

### `array_or_repeat`

```rust
fn array_or_repeat(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:2042-2084`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2042-L2084)*

### `continue_parsing_early`

```rust
fn continue_parsing_early(expr: &crate::expr::Expr) -> bool
```

*Defined in [`syn-2.0.111/src/expr.rs:2128-2144`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2128-L2144)*

### `expr_group`

```rust
fn expr_group(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:2156-2188`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2156-L2188)*

### `expr_let`

```rust
fn expr_let(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprLet>
```

*Defined in [`syn-2.0.111/src/expr.rs:2213-2224`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2213-L2224)*

### `expr_unary`

```rust
fn expr_unary(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprUnary>
```

*Defined in [`syn-2.0.111/src/expr.rs:2417-2427`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2417-L2427)*

### `expr_become`

```rust
fn expr_become(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/expr.rs:2495-2500`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2495-L2500)*

### `expr_closure`

```rust
fn expr_closure(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprClosure>
```

*Defined in [`syn-2.0.111/src/expr.rs:2533-2586`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2533-L2586)*

### `closure_arg`

```rust
fn closure_arg(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

*Defined in [`syn-2.0.111/src/expr.rs:2602-2635`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2602-L2635)*

### `expr_break`

```rust
fn expr_break(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprBreak>
```

*Defined in [`syn-2.0.111/src/expr.rs:2716-2747`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2716-L2747)*

### `expr_struct_helper`

```rust
fn expr_struct_helper(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::expr::ExprStruct>
```

*Defined in [`syn-2.0.111/src/expr.rs:2787-2830`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2787-L2830)*

### `expr_range`

```rust
fn expr_range(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprRange>
```

*Defined in [`syn-2.0.111/src/expr.rs:2872-2881`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2872-L2881)*

### `parse_range_end`

```rust
fn parse_range_end(input: crate::parse::ParseStream<'_>, limits: &crate::expr::RangeLimits, allow_struct: AllowStruct) -> crate::error::Result<Option<Box<crate::expr::Expr>>>
```

*Defined in [`syn-2.0.111/src/expr.rs:2884-2917`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2884-L2917)*

### `multi_index`

```rust
fn multi_index(e: &mut crate::expr::Expr, dot_token: &mut token::Dot, float: crate::lit::LitFloat) -> crate::error::Result<bool>
```

*Defined in [`syn-2.0.111/src/expr.rs:3048-3080`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L3048-L3080)*

### `check_cast`

```rust
fn check_cast(input: crate::parse::ParseStream<'_>) -> crate::error::Result<()>
```

*Defined in [`syn-2.0.111/src/expr.rs:3097-3117`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L3097-L3117)*

## Macros

### `impl_by_parsing_expr!`

*Defined in [`syn-2.0.111/src/expr.rs:2367-2390`](../../../../.source_1765210505/syn-2.0.111/src/expr.rs#L2367-L2390)*

