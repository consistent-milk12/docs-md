*[syn](../index.md) / [precedence](index.md)*

---

# Module `precedence`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Precedence`](#precedence) | enum |  |

## Enums

### `Precedence`

```rust
enum Precedence {
    Jump,
    Assign,
    Range,
    Or,
    And,
    Let,
    Compare,
    BitOr,
    BitXor,
    BitAnd,
    Shift,
    Sum,
    Product,
    Cast,
    Prefix,
    Unambiguous,
}
```

*Defined in [`syn-2.0.111/src/precedence.rs:18-54`](../../../.source_1765521767/syn-2.0.111/src/precedence.rs#L18-L54)*

#### Implementations

- <span id="precedence-const-min"></span>`const MIN: Self`

- <span id="precedence-of-binop"></span>`fn of_binop(op: &BinOp) -> Self` — [`BinOp`](../op/index.md#binop)

- <span id="precedence-of"></span>`fn of(e: &Expr) -> Self` — [`Expr`](../expr/index.md#expr)

#### Trait Implementations

##### `impl Clone for Precedence`

- <span id="precedence-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Precedence`

##### `impl PartialEq for Precedence`

- <span id="precedence-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for Precedence`

- <span id="precedence-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

