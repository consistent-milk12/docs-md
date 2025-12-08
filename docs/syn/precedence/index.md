*[syn](../index.md) / [precedence](index.md)*

---

# Module `precedence`

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

- `const MIN: Self`

- `fn of_binop(op: &BinOp) -> Self` — [`BinOp`](../op/index.md)

- `fn of(e: &Expr) -> Self` — [`Expr`](../expr/index.md)

#### Trait Implementations

##### `impl Clone for Precedence`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Precedence`

##### `impl PartialEq for Precedence`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl PartialOrd for Precedence`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

