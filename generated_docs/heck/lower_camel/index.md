*[heck](../index.md) / [lower_camel](index.md)*

---

# Module `lower_camel`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsLowerCamelCase`](#aslowercamelcase) | struct | This wrapper performs a lower camel case conversion in [`fmt::Display`]. |
| [`ToLowerCamelCase`](#tolowercamelcase) | trait | This trait defines a lower camel case conversion. |

## Structs

### `AsLowerCamelCase<T: AsRef<str>>`

```rust
struct AsLowerCamelCase<T: AsRef<str>>(T);
```

This wrapper performs a lower camel case conversion in [`fmt::Display`](../../miette_derive/index.md).

## Example:

```rust
use heck::AsLowerCamelCase;

let sentence = "It is we who built these palaces and cities.";
assert_eq!(format!("{}", AsLowerCamelCase(sentence)), "itIsWeWhoBuiltThesePalacesAndCities");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToLowerCamelCase`

```rust
trait ToLowerCamelCase: ToOwned { ... }
```

This trait defines a lower camel case conversion.

In lowerCamelCase, word boundaries are indicated by capital letters,
excepting the first word.

## Example:

```rust
use heck::ToLowerCamelCase;

let sentence = "It is we who built these palaces and cities.";
assert_eq!(sentence.to_lower_camel_case(), "itIsWeWhoBuiltThesePalacesAndCities");
```

#### Required Methods

- `fn to_lower_camel_case(&self) -> <Self as >::Owned`

  Convert this type to lower camel case.

#### Implementors

- `str`

