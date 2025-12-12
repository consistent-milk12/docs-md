*[heck](../index.md) / [title](index.md)*

---

# Module `title`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsTitleCase`](#astitlecase) | struct | This wrapper performs a title case conversion in [`fmt::Display`]. |
| [`ToTitleCase`](#totitlecase) | trait | This trait defines a title case conversion. |

## Structs

### `AsTitleCase<T: AsRef<str>>`

```rust
struct AsTitleCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/title.rs:44`](../../../.source_1765521767/heck-0.5.0/src/title.rs#L44)*

This wrapper performs a title case conversion in [`fmt::Display`](../../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsTitleCase;

let sentence = "We have always lived in slums and holes in the wall.";
assert_eq!(format!("{}", AsTitleCase(sentence)), "We Have Always Lived In Slums And Holes In The Wall");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsTitleCase<T>`

- <span id="astitlecase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsTitleCase<T>`

- <span id="astitlecase-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ToTitleCase`

```rust
trait ToTitleCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/title.rs:23-26`](../../../.source_1765521767/heck-0.5.0/src/title.rs#L23-L26)*

This trait defines a title case conversion.

In Title Case, word boundaries are indicated by spaces, and every word is
capitalized.

## Example:

```rust
use heck::ToTitleCase;

let sentence = "We have always lived in slums and holes in the wall.";
assert_eq!(sentence.to_title_case(), "We Have Always Lived In Slums And Holes In The Wall");
```

#### Required Methods

- `fn to_title_case(&self) -> <Self as >::Owned`

  Convert this type to title case.

#### Implementors

- `str`

