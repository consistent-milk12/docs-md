*[regex_syntax](../index.md) / [either](index.md)*

---

# Module `either`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Either`](#either) | enum | A simple binary sum type. |

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

##### `impl<Left: clone::Clone, Right: clone::Clone> Clone for Either<Left, Right>`

- <span id="either-clone"></span>`fn clone(&self) -> Either<Left, Right>` — [`Either`](#either)

##### `impl<Left: fmt::Debug, Right: fmt::Debug> Debug for Either<Left, Right>`

- <span id="either-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Left: cmp::Eq, Right: cmp::Eq> Eq for Either<Left, Right>`

##### `impl<Left: cmp::PartialEq, Right: cmp::PartialEq> PartialEq for Either<Left, Right>`

- <span id="either-eq"></span>`fn eq(&self, other: &Either<Left, Right>) -> bool` — [`Either`](#either)

##### `impl<Left, Right> StructuralPartialEq for Either<Left, Right>`

