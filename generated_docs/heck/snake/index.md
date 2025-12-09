*[heck](../index.md) / [snake](index.md)*

---

# Module `snake`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsSnakeCase`](#assnakecase) | struct | This wrapper performs a snake case conversion in [`fmt::Display`]. |
| [`ToSnakeCase`](#tosnakecase) | trait | This trait defines a snake case conversion. |
| [`ToSnekCase`](#tosnekcase) | trait | Oh heck, `SnekCase` is an alias for [`ToSnakeCase`]. |

## Structs

### `AsSnakeCase<T: AsRef<str>>`

```rust
struct AsSnakeCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/snake.rs:55`](../../../.source_1765210505/heck-0.5.0/src/snake.rs#L55)*

This wrapper performs a snake case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(format!("{}", AsSnakeCase(sentence)), "we_carry_a_new_world_here_in_our_hearts");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsSnakeCase<T>`

- <span id="assnakecase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsSnakeCase<T>`

- <span id="assnakecase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToSnakeCase`

```rust
trait ToSnakeCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/snake.rs:21-24`](../../../.source_1765210505/heck-0.5.0/src/snake.rs#L21-L24)*

This trait defines a snake case conversion.

In snake_case, word boundaries are indicated by underscores.

## Example:

```rust
use heck::ToSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(sentence.to_snake_case(), "we_carry_a_new_world_here_in_our_hearts");
```

#### Required Methods

- `fn to_snake_case(&self) -> <Self as >::Owned`

  Convert this type to snake case.

#### Implementors

- `str`

### `ToSnekCase`

```rust
trait ToSnekCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/snake.rs:28-31`](../../../.source_1765210505/heck-0.5.0/src/snake.rs#L28-L31)*

Oh heck, `SnekCase` is an alias for [`ToSnakeCase`](#tosnakecase). See ToSnakeCase for
more documentation.

#### Required Methods

- `fn to_snek_case(&self) -> <Self as >::Owned`

  Convert this type to snek case.

#### Implementors

- `T`

