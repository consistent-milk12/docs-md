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
  - [`AsKebabCase`](#askebabcase)
  - [`AsLowerCamelCase`](#aslowercamelcase)
  - [`AsShoutyKebabCase`](#asshoutykebabcase)
  - [`AsShoutySnakeCase`](#asshoutysnakecase)
  - [`AsShoutySnekCase`](#asshoutysnekcase)
  - [`AsSnakeCase`](#assnakecase)
  - [`AsSnekCase`](#assnekcase)
  - [`AsTitleCase`](#astitlecase)
  - [`AsTrainCase`](#astraincase)
  - [`AsUpperCamelCase`](#asuppercamelcase)
  - [`AsPascalCase`](#aspascalcase)
- [Traits](#traits)
  - [`ToKebabCase`](#tokebabcase)
  - [`ToLowerCamelCase`](#tolowercamelcase)
  - [`ToShoutyKebabCase`](#toshoutykebabcase)
  - [`ToShoutySnakeCase`](#toshoutysnakecase)
  - [`ToShoutySnekCase`](#toshoutysnekcase)
  - [`ToSnakeCase`](#tosnakecase)
  - [`ToSnekCase`](#tosnekcase)
  - [`ToTitleCase`](#totitlecase)
  - [`ToTrainCase`](#totraincase)
  - [`ToPascalCase`](#topascalcase)
  - [`ToUpperCamelCase`](#touppercamelcase)
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
| [`AsKebabCase`](#askebabcase) | struct |  |
| [`AsLowerCamelCase`](#aslowercamelcase) | struct |  |
| [`AsShoutyKebabCase`](#asshoutykebabcase) | struct |  |
| [`AsShoutySnakeCase`](#asshoutysnakecase) | struct |  |
| [`AsShoutySnekCase`](#asshoutysnekcase) | struct |  |
| [`AsSnakeCase`](#assnakecase) | struct |  |
| [`AsSnekCase`](#assnekcase) | struct |  |
| [`AsTitleCase`](#astitlecase) | struct |  |
| [`AsTrainCase`](#astraincase) | struct |  |
| [`AsUpperCamelCase`](#asuppercamelcase) | struct |  |
| [`AsPascalCase`](#aspascalcase) | struct |  |
| [`ToKebabCase`](#tokebabcase) | trait |  |
| [`ToLowerCamelCase`](#tolowercamelcase) | trait |  |
| [`ToShoutyKebabCase`](#toshoutykebabcase) | trait |  |
| [`ToShoutySnakeCase`](#toshoutysnakecase) | trait |  |
| [`ToShoutySnekCase`](#toshoutysnekcase) | trait |  |
| [`ToSnakeCase`](#tosnakecase) | trait |  |
| [`ToSnekCase`](#tosnekcase) | trait |  |
| [`ToTitleCase`](#totitlecase) | trait |  |
| [`ToTrainCase`](#totraincase) | trait |  |
| [`ToPascalCase`](#topascalcase) | trait |  |
| [`ToUpperCamelCase`](#touppercamelcase) | trait |  |
| [`transform`](#transform) | fn |  |
| [`lowercase`](#lowercase) | fn |  |
| [`uppercase`](#uppercase) | fn |  |
| [`capitalize`](#capitalize) | fn |  |

## Modules

- [`kebab`](kebab/index.md)
- [`lower_camel`](lower_camel/index.md)
- [`shouty_kebab`](shouty_kebab/index.md)
- [`shouty_snake`](shouty_snake/index.md)
- [`snake`](snake/index.md)
- [`title`](title/index.md)
- [`train`](train/index.md)
- [`upper_camel`](upper_camel/index.md)

## Structs

### `AsKebabCase<T: AsRef<str>>`

```rust
struct AsKebabCase<T: AsRef<str>>(T);
```

This wrapper performs a kebab case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a lower camel case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a kebab case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a shouty snake  case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a shouty snake  case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a snake case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a snake case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a title case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a train case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a upper camel case conversion in [`fmt::Display`](../miette_derive/index.md).

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

This wrapper performs a upper camel case conversion in [`fmt::Display`](../miette_derive/index.md).

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

- `fn to_kebab_case(&self) -> <Self as >::Owned`

  Convert this type to kebab case.

#### Implementors

- `str`

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

- `fn to_shouty_kebab_case(&self) -> <Self as >::Owned`

  Convert this type to shouty kebab case.

#### Implementors

- `str`

### `ToShoutySnakeCase`

```rust
trait ToShoutySnakeCase: ToOwned { ... }
```

This trait defines a shouty snake case conversion.

In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all
words are in uppercase.

## Example:

```rust
use heck::ToShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(sentence.to_shouty_snake_case(), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Required Methods

- `fn to_shouty_snake_case(&self) -> <Self as >::Owned`

  Convert this type to shouty snake case.

#### Implementors

- `str`

### `ToShoutySnekCase`

```rust
trait ToShoutySnekCase: ToOwned { ... }
```

Oh heck, `ToShoutySnekCase` is an alias for [`ToShoutySnakeCase`](#toshoutysnakecase). See
ToShoutySnakeCase for more documentation.

#### Required Methods

- `fn TO_SHOUTY_SNEK_CASE(&self) -> <Self as >::Owned`

  CONVERT THIS TYPE TO SNEK CASE.

#### Implementors

- `T`

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

- `fn to_snake_case(&self) -> <Self as >::Owned`

  Convert this type to snake case.

#### Implementors

- `str`

### `ToSnekCase`

```rust
trait ToSnekCase: ToOwned { ... }
```

Oh heck, `SnekCase` is an alias for [`ToSnakeCase`](#tosnakecase). See ToSnakeCase for
more documentation.

#### Required Methods

- `fn to_snek_case(&self) -> <Self as >::Owned`

  Convert this type to snek case.

#### Implementors

- `T`

### `ToTitleCase`

```rust
trait ToTitleCase: ToOwned { ... }
```

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

### `ToTrainCase`

```rust
trait ToTrainCase: ToOwned { ... }
```

This trait defines a train case conversion.

In Train-Case, word boundaries are indicated by hyphens and words start
with Capital Letters.

## Example:

```rust
use heck::ToTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(sentence.to_train_case(), "We-Are-Going-To-Inherit-The-Earth");
```

#### Required Methods

- `fn to_train_case(&self) -> <Self as >::Owned`

  Convert this type to Train-Case.

#### Implementors

- `str`

### `ToPascalCase`

```rust
trait ToPascalCase: ToOwned { ... }
```

`ToPascalCase` is an alias for [`ToUpperCamelCase`](#touppercamelcase). See ToUpperCamelCase for more
documentation.

#### Required Methods

- `fn to_pascal_case(&self) -> <Self as >::Owned`

  Convert this type to upper camel case.

#### Implementors

- `T`

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

- `fn to_upper_camel_case(&self) -> <Self as >::Owned`

  Convert this type to upper camel case.

#### Implementors

- `str`

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

