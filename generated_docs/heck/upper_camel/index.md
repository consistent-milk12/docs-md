*[heck](../index.md) / [upper_camel](index.md)*

---

# Module `upper_camel`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsUpperCamelCase`](#asuppercamelcase) | struct | This wrapper performs a upper camel case conversion in [`fmt::Display`]. |
| [`ToUpperCamelCase`](#touppercamelcase) | trait | This trait defines an upper camel case conversion. |
| [`ToPascalCase`](#topascalcase) | trait | `ToPascalCase` is an alias for [`ToUpperCamelCase`]. |

## Structs

### `AsUpperCamelCase<T: AsRef<str>>`

```rust
struct AsUpperCamelCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:57`](../../../.source_1765210505/heck-0.5.0/src/upper_camel.rs#L57)*

This wrapper performs a upper camel case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsUpperCamelCase;

let sentence = "We are not in the least afraid of ruins.";
assert_eq!(format!("{}", AsUpperCamelCase(sentence)), "WeAreNotInTheLeastAfraidOfRuins");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToUpperCamelCase`

```rust
trait ToUpperCamelCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:23-26`](../../../.source_1765210505/heck-0.5.0/src/upper_camel.rs#L23-L26)*

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

- `fn to_upper_camel_case(&self) -> <Self as >::Owned`

  Convert this type to upper camel case.

#### Implementors

- `str`

### `ToPascalCase`

```rust
trait ToPascalCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:36-39`](../../../.source_1765210505/heck-0.5.0/src/upper_camel.rs#L36-L39)*

`ToPascalCase` is an alias for [`ToUpperCamelCase`](#touppercamelcase). See ToUpperCamelCase for more
documentation.

#### Required Methods

- `fn to_pascal_case(&self) -> <Self as >::Owned`

  Convert this type to upper camel case.

#### Implementors

- `T`

