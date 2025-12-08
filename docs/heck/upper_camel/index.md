*[heck](../index.md) / [upper_camel](index.md)*

---

# Module `upper_camel`

## Structs

### `AsUpperCamelCase<T: AsRef<str>>`

```rust
struct AsUpperCamelCase<T: AsRef<str>>(T);
```

This wrapper performs a upper camel case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsUpperCamelCase;

let sentence = "We are not in the least afraid of ruins.";
assert_eq!(format!("{}", AsUpperCamelCase(sentence)), "WeAreNotInTheLeastAfraidOfRuins");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsUpperCamelCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsUpperCamelCase<T>`

- `fn to_string(self: &Self) -> String`

## Traits

### `ToUpperCamelCase`

```rust
trait ToUpperCamelCase: ToOwned { ... }
```

This trait defines an upper camel case conversion.

In UpperCamelCase, word boundaries are indicated by capital letters,
including the first word.

## Example:

```rust
use heck::ToUpperCamelCase;

let sentence = "We are not in the least afraid of ruins.";
assert_eq!(sentence.to_upper_camel_case(), "WeAreNotInTheLeastAfraidOfRuins");
```

#### Required Methods

- `fn to_upper_camel_case(self: &Self) -> <Self as >::Owned`

  Convert this type to upper camel case.

### `ToPascalCase`

```rust
trait ToPascalCase: ToOwned { ... }
```

`ToPascalCase` is an alias for [`ToUpperCamelCase`](#touppercamelcase). See ToUpperCamelCase for more
documentation.

#### Required Methods

- `fn to_pascal_case(self: &Self) -> <Self as >::Owned`

  Convert this type to upper camel case.

