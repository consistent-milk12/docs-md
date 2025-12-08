# Crate `heck`

**heck** is a case conversion library.

This library exists to provide case conversion between common cases like
CamelCase and snake_case. It is intended to be unicode aware, internally
consistent, and reasonably well performing.

## Definition of a word boundary

Word boundaries are defined by non-alphanumeric characters, as well as
within those words in this manner:

1. If an uppercase character is followed by lowercase letters, a word
boundary is considered to be just prior to that uppercase character.
2. If multiple uppercase characters are consecutive, they are considered to
be within a single word, except that the last will be part of the next word
if it is followed by lowercase characters (see rule 1).

That is, "HelloWorld" is segmented `Hello|World` whereas "XMLHttpRequest" is
segmented `XML|Http|Request`.

Characters not within words (such as spaces, punctuations, and underscores)
are not included in the output string except as they are a part of the case
being converted to. Multiple adjacent word boundaries (such as a series of
underscores) are folded into one. ("hello__world" in snake case is therefore
"hello_world", not the exact same string). Leading or trailing word boundary
indicators are dropped, except insofar as CamelCase capitalizes the first
word.

### Cases contained in this library:

1. UpperCamelCase
2. lowerCamelCase
3. snake_case
4. kebab-case
5. SHOUTY_SNAKE_CASE
6. Title Case
7. SHOUTY-KEBAB-CASE
8. Train-Case

## Contents

- [Modules](#modules)
  - [`kebab`](#kebab)
  - [`lower_camel`](#lower_camel)
  - [`shouty_kebab`](#shouty_kebab)
  - [`shouty_snake`](#shouty_snake)
  - [`snake`](#snake)
  - [`title`](#title)
  - [`train`](#train)
  - [`upper_camel`](#upper_camel)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`transform`](#transform)
  - [`lowercase`](#lowercase)
  - [`uppercase`](#uppercase)
  - [`capitalize`](#capitalize)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`kebab`](#kebab) | mod |  |
| [`lower_camel`](#lower_camel) | mod |  |
| [`shouty_kebab`](#shouty_kebab) | mod |  |
| [`shouty_snake`](#shouty_snake) | mod |  |
| [`snake`](#snake) | mod |  |
| [`title`](#title) | mod |  |
| [`train`](#train) | mod |  |
| [`upper_camel`](#upper_camel) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`transform`](#transform) | fn |  |
| [`lowercase`](#lowercase) | fn |  |
| [`uppercase`](#uppercase) | fn |  |
| [`capitalize`](#capitalize) | fn |  |

## Modules

- [`kebab`](kebab/index.md) - 
- [`lower_camel`](lower_camel/index.md) - 
- [`shouty_kebab`](shouty_kebab/index.md) - 
- [`shouty_snake`](shouty_snake/index.md) - 
- [`snake`](snake/index.md) - 
- [`title`](title/index.md) - 
- [`train`](train/index.md) - 
- [`upper_camel`](upper_camel/index.md) - 

## Structs

### `AsKebabCase<T: AsRef<str>>`

```rust
struct AsKebabCase<T: AsRef<str>>(T);
```

This wrapper performs a kebab case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

### `AsLowerCamelCase<T: AsRef<str>>`

```rust
struct AsLowerCamelCase<T: AsRef<str>>(T);
```

This wrapper performs a lower camel case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

### `AsShoutyKebabCase<T: AsRef<str>>`

```rust
struct AsShoutyKebabCase<T: AsRef<str>>(T);
```

This wrapper performs a kebab case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

### `AsShoutySnakeCase<T: AsRef<str>>`

```rust
struct AsShoutySnakeCase<T: AsRef<str>>(T);
```

This wrapper performs a shouty snake  case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(format!("{}", AsShoutySnakeCase(sentence)), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-to-string"></span>`fn to_string(&self) -> String`

### `AsShoutySnekCase<T: AsRef<str>>`

```rust
struct AsShoutySnekCase<T: AsRef<str>>(T);
```

This wrapper performs a shouty snake  case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(format!("{}", AsShoutySnakeCase(sentence)), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-to-string"></span>`fn to_string(&self) -> String`

### `AsSnakeCase<T: AsRef<str>>`

```rust
struct AsSnakeCase<T: AsRef<str>>(T);
```

This wrapper performs a snake case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

### `AsSnekCase<T: AsRef<str>>`

```rust
struct AsSnekCase<T: AsRef<str>>(T);
```

This wrapper performs a snake case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

### `AsTitleCase<T: AsRef<str>>`

```rust
struct AsTitleCase<T: AsRef<str>>(T);
```

This wrapper performs a title case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

### `AsTrainCase<T: AsRef<str>>`

```rust
struct AsTrainCase<T: AsRef<str>>(T);
```

This wrapper performs a train case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

## Example:

```rust
use heck::AsTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsTrainCase(sentence)), "We-Are-Going-To-Inherit-The-Earth");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsTrainCase<T>`

- <span id="astraincase-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsTrainCase<T>`

- <span id="astraincase-to-string"></span>`fn to_string(&self) -> String`

### `AsUpperCamelCase<T: AsRef<str>>`

```rust
struct AsUpperCamelCase<T: AsRef<str>>(T);
```

This wrapper performs a upper camel case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

### `AsPascalCase<T: AsRef<str>>`

```rust
struct AsPascalCase<T: AsRef<str>>(T);
```

This wrapper performs a upper camel case conversion in [`fmt::Display`](../miette_derive/fmt/index.md).

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

## Functions

### `transform`

```rust
fn transform<F, G>(s: &str, with_word: F, boundary: G, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    F: FnMut(&str, &mut fmt::Formatter<'_>) -> fmt::Result,
    G: FnMut(&mut fmt::Formatter<'_>) -> fmt::Result
```

### `lowercase`

```rust
fn lowercase(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

### `uppercase`

```rust
fn uppercase(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

### `capitalize`

```rust
fn capitalize(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

