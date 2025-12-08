*[heck](../index.md) / [shouty_kebab](index.md)*

---

# Module `shouty_kebab`

## Structs

### `AsShoutyKebabCase<T: AsRef<str>>`

```rust
struct AsShoutyKebabCase<T: AsRef<str>>(T);
```

This wrapper performs a kebab case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsShoutyKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsShoutyKebabCase(sentence)), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutyKebabCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutyKebabCase<T>`

- `fn to_string(self: &Self) -> String`

## Traits

### `ToShoutyKebabCase`

```rust
trait ToShoutyKebabCase: ToOwned { ... }
```

This trait defines a shouty kebab case conversion.

In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all
words are in uppercase.

## Example:

```rust
use heck::ToShoutyKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(sentence.to_shouty_kebab_case(), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
```

#### Required Methods

- `fn to_shouty_kebab_case(self: &Self) -> <Self as >::Owned`

  Convert this type to shouty kebab case.

