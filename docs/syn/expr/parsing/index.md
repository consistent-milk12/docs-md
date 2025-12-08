*[syn](../../index.md) / [expr](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Structs

### `AllowStruct`

```rust
struct AllowStruct(bool);
```

#### Trait Implementations

##### `impl Clone for AllowStruct`

- `fn clone(self: &Self) -> Self`

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
fn multi_index(e: &mut crate::expr::Expr, dot_token: &mut $crate::token::Dot, float: crate::lit::LitFloat) -> crate::error::Result<bool>
```

### `check_cast`

```rust
fn check_cast(input: crate::parse::ParseStream<'_>) -> crate::error::Result<()>
```

## Macros

### `impl_by_parsing_expr!`

