*[syn](../index.md) / [fixup](index.md)*

---

# Module `fixup`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FixupContext`](#fixupcontext) | struct |  |
| [`Scan`](#scan) | enum |  |
| [`scan_left`](#scan-left) | fn |  |
| [`scan_right`](#scan-right) | fn |  |

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

*Defined in [`syn-2.0.111/src/fixup.rs:11-149`](../../../.source_1765521767/syn-2.0.111/src/fixup.rs#L11-L149)*

#### Implementations

- <span id="fixupcontext-const-none"></span>`const NONE: Self`

- <span id="fixupcontext-new-stmt"></span>`fn new_stmt() -> Self`

- <span id="fixupcontext-new-match-arm"></span>`fn new_match_arm() -> Self`

- <span id="fixupcontext-new-condition"></span>`fn new_condition() -> Self`

- <span id="fixupcontext-leftmost-subexpression-with-operator"></span>`fn leftmost_subexpression_with_operator(self, expr: &Expr, next_operator_can_begin_expr: bool, next_operator_can_begin_generics: bool, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-leftmost-subexpression-with-dot"></span>`fn leftmost_subexpression_with_dot(self, expr: &Expr) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-leftmost-subexpression-precedence"></span>`fn leftmost_subexpression_precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-rightmost-subexpression"></span>`fn rightmost_subexpression(self, expr: &Expr, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-rightmost-subexpression-fixup"></span>`fn rightmost_subexpression_fixup(self, reset_allow_struct: bool, optional_operand: bool, precedence: Precedence) -> Self` — [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-rightmost-subexpression-precedence"></span>`fn rightmost_subexpression_precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-parenthesize"></span>`fn parenthesize(self, expr: &Expr) -> bool` — [`Expr`](../expr/index.md#expr)

- <span id="fixupcontext-precedence"></span>`fn precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

#### Trait Implementations

##### `impl Clone for FixupContext`

- <span id="fixupcontext-clone"></span>`fn clone(&self) -> Self`

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

*Defined in [`syn-2.0.111/src/fixup.rs:455-459`](../../../.source_1765521767/syn-2.0.111/src/fixup.rs#L455-L459)*

#### Trait Implementations

##### `impl Clone for Scan`

- <span id="scan-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Scan`

##### `impl PartialEq for Scan`

- <span id="scan-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Functions

### `scan_left`

```rust
fn scan_left(expr: &crate::expr::Expr, fixup: FixupContext) -> bool
```

*Defined in [`syn-2.0.111/src/fixup.rs:479-490`](../../../.source_1765521767/syn-2.0.111/src/fixup.rs#L479-L490)*

### `scan_right`

```rust
fn scan_right(expr: &crate::expr::Expr, fixup: FixupContext, precedence: crate::precedence::Precedence, fail_offset: u8, bailout_offset: u8) -> Scan
```

*Defined in [`syn-2.0.111/src/fixup.rs:493-773`](../../../.source_1765521767/syn-2.0.111/src/fixup.rs#L493-L773)*

