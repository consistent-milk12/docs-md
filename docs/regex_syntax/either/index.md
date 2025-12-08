*[regex_syntax](../index.md) / [either](index.md)*

---

# Module `either`

## Enums

### `Either<Left, Right>`

```rust
enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}
```

A simple binary sum type.

This is occasionally useful in an ad hoc fashion.

#### Trait Implementations

##### `impl<Left: $crate::clone::Clone, Right: $crate::clone::Clone> Clone for Either<Left, Right>`

- `fn clone(self: &Self) -> Either<Left, Right>` — [`Either`](#either)

##### `impl<Left: $crate::fmt::Debug, Right: $crate::fmt::Debug> Debug for Either<Left, Right>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Left: $crate::cmp::Eq, Right: $crate::cmp::Eq> Eq for Either<Left, Right>`

##### `impl<Left: $crate::cmp::PartialEq, Right: $crate::cmp::PartialEq> PartialEq for Either<Left, Right>`

- `fn eq(self: &Self, other: &Either<Left, Right>) -> bool` — [`Either`](#either)

##### `impl<Left, Right> StructuralPartialEq for Either<Left, Right>`

