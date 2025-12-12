*[heck](../index.md) / [shouty_kebab](index.md)*

---

# Module `shouty_kebab`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsShoutyKebabCase`](#asshoutykebabcase) | struct | This wrapper performs a kebab case conversion in [`fmt::Display`]. |
| [`ToShoutyKebabCase`](#toshoutykebabcase) | trait | This trait defines a shouty kebab case conversion. |

## Structs

### `AsShoutyKebabCase<T: AsRef<str>>`

```rust
struct AsShoutyKebabCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/shouty_kebab.rs:41`](../../../.source_1765521767/heck-0.5.0/src/shouty_kebab.rs#L41)*

This wrapper performs a kebab case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsShoutyKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsShoutyKebabCase(sentence)), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToShoutyKebabCase`

```rust
trait ToShoutyKebabCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/shouty_kebab.rs:20-23`](../../../.source_1765521767/heck-0.5.0/src/shouty_kebab.rs#L20-L23)*

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

- `fn to_shouty_kebab_case(&self) -> <Self as >::Owned`

  Convert this type to shouty kebab case.

#### Implementors

- `str`

