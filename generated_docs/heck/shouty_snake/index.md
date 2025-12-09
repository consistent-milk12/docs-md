*[heck](../index.md) / [shouty_snake](index.md)*

---

# Module `shouty_snake`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsShoutySnakeCase`](#asshoutysnakecase) | struct | This wrapper performs a shouty snake  case conversion in [`fmt::Display`]. |
| [`ToShoutySnakeCase`](#toshoutysnakecase) | trait | This trait defines a shouty snake case conversion. |
| [`ToShoutySnekCase`](#toshoutysnekcase) | trait | Oh heck, `ToShoutySnekCase` is an alias for [`ToShoutySnakeCase`]. |

## Structs

### `AsShoutySnakeCase<T: AsRef<str>>`

```rust
struct AsShoutySnakeCase<T: AsRef<str>>(T);
```

This wrapper performs a shouty snake  case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(format!("{}", AsShoutySnakeCase(sentence)), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToShoutySnakeCase`

```rust
trait ToShoutySnakeCase: ToOwned { ... }
```

This trait defines a shouty snake case conversion.

In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all
words are in uppercase.

## Example:

```rust
use heck::ToShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(sentence.to_shouty_snake_case(), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Required Methods

- `fn to_shouty_snake_case(&self) -> <Self as >::Owned`

  Convert this type to shouty snake case.

#### Implementors

- `str`

### `ToShoutySnekCase`

```rust
trait ToShoutySnekCase: ToOwned { ... }
```

Oh heck, `ToShoutySnekCase` is an alias for [`ToShoutySnakeCase`](../index.md). See
ToShoutySnakeCase for more documentation.

#### Required Methods

- `fn TO_SHOUTY_SNEK_CASE(&self) -> <Self as >::Owned`

  CONVERT THIS TYPE TO SNEK CASE.

#### Implementors

- `T`

