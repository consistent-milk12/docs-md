*[heck](../index.md) / [snake](index.md)*

---

# Module `snake`

## Structs

### `AsSnakeCase<T: AsRef<str>>`

```rust
struct AsSnakeCase<T: AsRef<str>>(T);
```

This wrapper performs a snake case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(format!("{}", AsSnakeCase(sentence)), "we_carry_a_new_world_here_in_our_hearts");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsSnakeCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsSnakeCase<T>`

- `fn to_string(self: &Self) -> String`

## Traits

### `ToSnakeCase`

```rust
trait ToSnakeCase: ToOwned { ... }
```

This trait defines a snake case conversion.

In snake_case, word boundaries are indicated by underscores.

## Example:

```rust
use heck::ToSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(sentence.to_snake_case(), "we_carry_a_new_world_here_in_our_hearts");
```

#### Required Methods

- `fn to_snake_case(self: &Self) -> <Self as >::Owned`

  Convert this type to snake case.

### `ToSnekCase`

```rust
trait ToSnekCase: ToOwned { ... }
```

Oh heck, `SnekCase` is an alias for [`ToSnakeCase`](#tosnakecase). See ToSnakeCase for
more documentation.

#### Required Methods

- `fn to_snek_case(self: &Self) -> <Self as >::Owned`

  Convert this type to snek case.

