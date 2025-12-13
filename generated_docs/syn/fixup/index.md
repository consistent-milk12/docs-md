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

  Create the initial fixup for printing an expression in statement

  position.

- <span id="fixupcontext-new-match-arm"></span>`fn new_match_arm() -> Self`

  Create the initial fixup for printing an expression as the right-hand

  side of a match arm.

- <span id="fixupcontext-new-condition"></span>`fn new_condition() -> Self`

  Create the initial fixup for printing an expression as the "condition"

  of an `if` or `while`. There are a few other positions which are

  grammatically equivalent and also use this, such as the iterator

  expression in `for` and the scrutinee in `match`.

- <span id="fixupcontext-leftmost-subexpression-with-operator"></span>`fn leftmost_subexpression_with_operator(self, expr: &Expr, next_operator_can_begin_expr: bool, next_operator_can_begin_generics: bool, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Transform this fixup into the one that should apply when printing the

  leftmost subexpression of the current expression.

  

  The leftmost subexpression is any subexpression that has the same first

  token as the current expression, but has a different last token.

  

  For example in `$a + $b` and `$a.method()`, the subexpression `$a` is a

  leftmost subexpression.

  

  Not every expression has a leftmost subexpression. For example neither

  `-$a` nor `[$a]` have one.

- <span id="fixupcontext-leftmost-subexpression-with-dot"></span>`fn leftmost_subexpression_with_dot(self, expr: &Expr) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Transform this fixup into the one that should apply when printing a

  leftmost subexpression followed by a `.` or `?` token, which confer

  different statement boundary rules compared to other leftmost

  subexpressions.

- <span id="fixupcontext-leftmost-subexpression-precedence"></span>`fn leftmost_subexpression_precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-rightmost-subexpression"></span>`fn rightmost_subexpression(self, expr: &Expr, precedence: Precedence) -> (Precedence, Self)` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Transform this fixup into the one that should apply when printing the

  rightmost subexpression of the current expression.

  

  The rightmost subexpression is any subexpression that has a different

  first token than the current expression, but has the same last token.

  

  For example in `$a + $b` and `-$b`, the subexpression `$b` is a

  rightmost subexpression.

  

  Not every expression has a rightmost subexpression. For example neither

  `[$b]` nor `$a.f($b)` have one.

- <span id="fixupcontext-rightmost-subexpression-fixup"></span>`fn rightmost_subexpression_fixup(self, reset_allow_struct: bool, optional_operand: bool, precedence: Precedence) -> Self` — [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-rightmost-subexpression-precedence"></span>`fn rightmost_subexpression_precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

- <span id="fixupcontext-parenthesize"></span>`fn parenthesize(self, expr: &Expr) -> bool` — [`Expr`](../expr/index.md#expr)

  Determine whether parentheses are needed around the given expression to

  head off the early termination of a statement or condition.

- <span id="fixupcontext-precedence"></span>`fn precedence(self, expr: &Expr) -> Precedence` — [`Expr`](../expr/index.md#expr), [`Precedence`](../precedence/index.md#precedence)

  Determines the effective precedence of a subexpression. Some expressions

  have higher or lower precedence when adjacent to particular operators.

#### Trait Implementations

##### `impl Any for FixupContext`

- <span id="fixupcontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FixupContext`

- <span id="fixupcontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FixupContext`

- <span id="fixupcontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FixupContext`

- <span id="fixupcontext-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FixupContext`

- <span id="fixupcontext-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FixupContext`

##### `impl<T> From for FixupContext`

- <span id="fixupcontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FixupContext`

- <span id="fixupcontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for FixupContext`

- <span id="fixupcontext-toowned-type-owned"></span>`type Owned = T`

- <span id="fixupcontext-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fixupcontext-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FixupContext`

- <span id="fixupcontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fixupcontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FixupContext`

- <span id="fixupcontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fixupcontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Scan`

- <span id="scan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Scan`

- <span id="scan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Scan`

- <span id="scan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Scan`

- <span id="scan-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Scan`

- <span id="scan-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Scan`

##### `impl<T> From for Scan`

- <span id="scan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Scan`

- <span id="scan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Scan`

- <span id="scan-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for Scan`

- <span id="scan-toowned-type-owned"></span>`type Owned = T`

- <span id="scan-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="scan-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Scan`

- <span id="scan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Scan`

- <span id="scan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

