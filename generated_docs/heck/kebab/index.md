*[heck](../index.md) / [kebab](index.md)*

---

# Module `kebab`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsKebabCase`](#askebabcase) | struct | This wrapper performs a kebab case conversion in [`fmt::Display`]. |
| [`ToKebabCase`](#tokebabcase) | trait | This trait defines a kebab case conversion. |

## Structs

### `AsKebabCase<T: AsRef<str>>`

```rust
struct AsKebabCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/kebab.rs:40`](../../../.source_1765521767/heck-0.5.0/src/kebab.rs#L40)*

This wrapper performs a kebab case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsKebabCase(sentence)), "we-are-going-to-inherit-the-earth");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsKebabCase<T>`

- <span id="askebabcase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsKebabCase<T>`

- <span id="askebabcase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToKebabCase`

```rust
trait ToKebabCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/kebab.rs:19-22`](../../../.source_1765521767/heck-0.5.0/src/kebab.rs#L19-L22)*

This trait defines a kebab case conversion.

In kebab-case, word boundaries are indicated by hyphens.

## Example:

```rust
use heck::ToKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(sentence.to_kebab_case(), "we-are-going-to-inherit-the-earth");
```

#### Required Methods

- `fn to_kebab_case(&self) -> <Self as >::Owned`

  Convert this type to kebab case.

#### Implementors

- `str`

