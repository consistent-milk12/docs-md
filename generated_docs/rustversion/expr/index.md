*[rustversion](../index.md) / [expr](index.md)*

---

# Module `expr`

## Contents

- [Enums](#enums)
  - [`Expr`](#expr)
- [Functions](#functions)
  - [`parse`](#parse)
  - [`parse_nightly`](#parse-nightly)
  - [`parse_stable`](#parse-stable)
  - [`parse_since`](#parse-since)
  - [`parse_before`](#parse-before)
  - [`parse_not`](#parse-not)
  - [`parse_any`](#parse-any)
  - [`parse_all`](#parse-all)
  - [`parse_comma_separated`](#parse-comma-separated)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Expr`](#expr) | enum |  |
| [`parse`](#parse) | fn |  |
| [`parse_nightly`](#parse-nightly) | fn |  |
| [`parse_stable`](#parse-stable) | fn |  |
| [`parse_since`](#parse-since) | fn |  |
| [`parse_before`](#parse-before) | fn |  |
| [`parse_not`](#parse-not) | fn |  |
| [`parse_any`](#parse-any) | fn |  |
| [`parse_all`](#parse-all) | fn |  |
| [`parse_comma_separated`](#parse-comma-separated) | fn |  |

## Enums

### `Expr`

```rust
enum Expr {
    Stable,
    Beta,
    Nightly,
    Date(crate::date::Date),
    Since(crate::bound::Bound),
    Before(crate::bound::Bound),
    Release(crate::release::Release),
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
}
```

*Defined in [`rustversion-1.0.22/src/expr.rs:10-21`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L10-L21)*

#### Implementations

- <span id="expr-eval"></span>`fn eval(&self, rustc: Version) -> bool` — [`Version`](../version/index.md#version)

#### Trait Implementations

##### `impl Any for Expr`

- <span id="expr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Expr`

- <span id="expr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Expr`

- <span id="expr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Expr`

- <span id="expr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Expr`

- <span id="expr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Expr`

- <span id="expr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="expr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Expr`

- <span id="expr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="expr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:52-69`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L52-L69)*

### `parse_nightly`

```rust
fn parse_nightly(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:71-83`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L71-L83)*

### `parse_stable`

```rust
fn parse_stable(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:85-97`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L85-L97)*

### `parse_since`

```rust
fn parse_since(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:99-108`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L99-L108)*

### `parse_before`

```rust
fn parse_before(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:110-119`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L110-L119)*

### `parse_not`

```rust
fn parse_not(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:121-130`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L121-L130)*

### `parse_any`

```rust
fn parse_any(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:132-139`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L132-L139)*

### `parse_all`

```rust
fn parse_all(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:141-148`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L141-L148)*

### `parse_comma_separated`

```rust
fn parse_comma_separated(iter: &'_ mut IterImpl) -> std::result::Result<Vec<Expr>, Error>
```

*Defined in [`rustversion-1.0.22/src/expr.rs:150-163`](../../../.source_1765633015/rustversion-1.0.22/src/expr.rs#L150-L163)*

