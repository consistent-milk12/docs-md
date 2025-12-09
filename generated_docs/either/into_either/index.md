*[either](../index.md) / [into_either](index.md)*

---

# Module `into_either`

The trait [`IntoEither`](../index.md) provides methods for converting a type `Self`, whose
size is constant and known at compile-time, into an [`Either`](../index.md) variant.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoEither`](#intoeither) | trait | Provides methods for converting a type `Self` into either a [`Left`] or [`Right`] |

## Traits

### `IntoEither`

```rust
trait IntoEither: Sized { ... }
```

Provides methods for converting a type `Self` into either a [`Left`](../index.md) or [`Right`](../index.md)
variant of [`Either<Self, Self>`](Either).

The [`into_either`](IntoEither::into_either) method takes a `bool` to determine
whether to convert to [`Left`](../index.md) or [`Right`](../index.md).

The [`into_either_with`](IntoEither::into_either_with) method takes a
[predicate function](FnOnce) to determine whether to convert to [`Left`](../index.md) or [`Right`](../index.md).

#### Provided Methods

- `fn into_either(self, into_left: bool) -> Either<Self, Self>`

  Converts `self` into a [`Left`](../index.md) variant of [`Either<Self, Self>`](Either)

- `fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>`

  Converts `self` into a [`Left`](../index.md) variant of [`Either<Self, Self>`](Either)

#### Implementors

- [`Either`](../index.md)
- [`IterEither`](../index.md)
- `T`

