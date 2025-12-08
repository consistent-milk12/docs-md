*[heck](../index.md) / [kebab](index.md)*

---

# Module `kebab`

## Structs

### `AsKebabCase<T: AsRef<str>>`

```rust
struct AsKebabCase<T: AsRef<str>>(T);
```

This wrapper performs a kebab case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsKebabCase(sentence)), "we-are-going-to-inherit-the-earth");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsKebabCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsKebabCase<T>`

- `fn to_string(self: &Self) -> String`

## Traits

### `ToKebabCase`

```rust
trait ToKebabCase: ToOwned { ... }
```

This trait defines a kebab case conversion.

In kebab-case, word boundaries are indicated by hyphens.

## Example:

```rust
use heck::ToKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(sentence.to_kebab_case(), "we-are-going-to-inherit-the-earth");
```

#### Required Methods

- `fn to_kebab_case(self: &Self) -> <Self as >::Owned`

  Convert this type to kebab case.

