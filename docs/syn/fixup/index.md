*[syn](../index.md) / [fixup](index.md)*

---

# Module `fixup`

## Structs

### `FixupContext`

```rust
struct FixupContext {
    previous_operator: crate::precedence::Precedence,
    next_operator: crate::precedence::Precedence,
    stmt: bool,
    leftmost_subexpression_in_stmt: bool,
    match_arm: bool,
    leftmost_subexpression_in_match_arm: bool,
    condition: bool,
    rightmost_subexpression_in_condition: bool,
    leftmost_subexpression_in_optional_operand: bool,
    next_operator_can_begin_expr: bool,
    next_operator_can_continue_expr: bool,
    next_operator_can_begin_generics: bool,
}
```

#### Implementations

- `const NONE: Self`

- `fn new_stmt() -> Self`

- `fn new_match_arm() -> Self`

- `fn new_condition() -> Self`

- `fn leftmost_subexpression_with_operator(self: Self, expr: &Expr, next_operator_can_begin_expr: bool, next_operator_can_begin_generics: bool, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md), [`Precedence`](../precedence/index.md)

- `fn leftmost_subexpression_with_dot(self: Self, expr: &Expr) -> (Precedence, Self)` — [`Expr`](../expr/index.md), [`Precedence`](../precedence/index.md)

- `fn leftmost_subexpression_precedence(self: Self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md), [`Precedence`](../precedence/index.md)

- `fn rightmost_subexpression(self: Self, expr: &Expr, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md), [`Precedence`](../precedence/index.md)

- `fn rightmost_subexpression_fixup(self: Self, reset_allow_struct: bool, optional_operand: bool, precedence: Precedence) -> Self` — [`Precedence`](../precedence/index.md)

- `fn rightmost_subexpression_precedence(self: Self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md), [`Precedence`](../precedence/index.md)

- `fn parenthesize(self: Self, expr: &Expr) -> bool` — [`Expr`](../expr/index.md)

- `fn precedence(self: Self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md), [`Precedence`](../precedence/index.md)

#### Trait Implementations

##### `impl Clone for FixupContext`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for FixupContext`

## Enums

### `Scan`

```rust
enum Scan {
    Fail,
    Bailout,
    Consume,
}
```

#### Trait Implementations

##### `impl Clone for Scan`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Scan`

##### `impl PartialEq for Scan`

- `fn eq(self: &Self, other: &Self) -> bool`

## Functions

### `scan_left`

```rust
fn scan_left(expr: &crate::expr::Expr, fixup: FixupContext) -> bool
```

### `scan_right`

```rust
fn scan_right(expr: &crate::expr::Expr, fixup: FixupContext, precedence: crate::precedence::Precedence, fail_offset: u8, bailout_offset: u8) -> Scan
```

