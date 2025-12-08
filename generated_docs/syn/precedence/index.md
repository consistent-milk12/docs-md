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

#### Implementations

- <span id="precedence-min"></span>`const MIN: Self`

- <span id="precedence-of-binop"></span>`fn of_binop(op: &BinOp) -> Self` — [`BinOp`](../index.md)

- <span id="precedence-of"></span>`fn of(e: &Expr) -> Self` — [`Expr`](../index.md)

#### Trait Implementations

##### `impl Clone for Precedence`

- <span id="precedence-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Precedence`

##### `impl PartialEq for Precedence`

- <span id="precedence-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for Precedence`

- <span id="precedence-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

