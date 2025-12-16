*[either](../index.md) / [into_either](index.md)*

---

# Module `into_either`

The trait [`IntoEither`](#intoeither) provides methods for converting a type `Self`, whose
size is constant and known at compile-time, into an [`Either`](../index.md) variant.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoEither`](#intoeither) | trait | Provides methods for converting a type `Self` into either a [`Left`] or [`Right`] variant of [`Either<Self, Self>`](Either). |

## Traits

### `IntoEither`

```rust
trait IntoEither: Sized { ... }
```

*Defined in [`either-1.15.0/src/into_either.rs:14-62`](../../../.source_1765894658/either-1.15.0/src/into_either.rs#L14-L62)*

Provides methods for converting a type `Self` into either a [`Left`](../index.md) or [`Right`](../index.md)
variant of [`Either<Self, Self>`](Either).

The [`into_either`](IntoEither::into_either) method takes a `bool` to determine
whether to convert to [`Left`](../index.md) or [`Right`](../index.md).

The [`into_either_with`](IntoEither::into_either_with) method takes a
[predicate function](FnOnce) to determine whether to convert to [`Left`](../index.md) or [`Right`](../index.md).

#### Provided Methods

- `fn into_either(self, into_left: bool) -> Either<Self, Self>`

  Converts `self` into a [`Left`](../index.md) variant of [`Either<Self, Self>`](Either)
  if `into_left` is `true`.
  Converts `self` into a [`Right`](../index.md) variant of [`Either<Self, Self>`](Either)
  otherwise.
  
  # Examples
  
  ```rust
  use either::{IntoEither, Left, Right};
  
  let x = 0;
  assert_eq!(x.into_either(true), Left(x));
  assert_eq!(x.into_either(false), Right(x));
  ```

- `fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>`

  Converts `self` into a [`Left`](../index.md) variant of [`Either<Self, Self>`](Either)
  if `into_left(&self)` returns `true`.
  Converts `self` into a [`Right`](../index.md) variant of [`Either<Self, Self>`](Either)
  otherwise.
  
  # Examples
  
  ```rust
  use either::{IntoEither, Left, Right};
  
  fn is_even(x: &u8) -> bool {
      x % 2 == 0
  }
  
  let x = 0;
  assert_eq!(x.into_either_with(is_even), Left(x));
  assert_eq!(x.into_either_with(|x| !is_even(x)), Right(x));
  ```

#### Implementors

- `T`

