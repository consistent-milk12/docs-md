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
  - [`lower_camel`](#lower-camel)
  - [`shouty_kebab`](#shouty-kebab)
  - [`shouty_snake`](#shouty-snake)
  - [`snake`](#snake)
  - [`title`](#title)
  - [`train`](#train)
  - [`upper_camel`](#upper-camel)
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
| [`lower_camel`](#lower-camel) | mod |  |
| [`shouty_kebab`](#shouty-kebab) | mod |  |
| [`shouty_snake`](#shouty-snake) | mod |  |
| [`snake`](#snake) | mod |  |
| [`title`](#title) | mod |  |
| [`train`](#train) | mod |  |
| [`upper_camel`](#upper-camel) | mod |  |
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

*Defined in [`heck-0.5.0/src/kebab.rs:40`](../../.source_1765633015/heck-0.5.0/src/kebab.rs#L40)*

This wrapper performs a kebab case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsKebabCase(sentence)), "we-are-going-to-inherit-the-earth");
```

#### Trait Implementations

##### `impl<T> Any for AsKebabCase<T>`

- <span id="askebabcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsKebabCase<T>`

- <span id="askebabcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsKebabCase<T>`

- <span id="askebabcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsKebabCase<T>`

- <span id="askebabcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsKebabCase<T>`

- <span id="askebabcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsKebabCase<T>`

- <span id="askebabcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsKebabCase<T>`

- <span id="askebabcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsKebabCase<T>`

- <span id="askebabcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="askebabcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsKebabCase<T>`

- <span id="askebabcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="askebabcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsLowerCamelCase<T: AsRef<str>>`

```rust
struct AsLowerCamelCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/lower_camel.rs:44`](../../.source_1765633015/heck-0.5.0/src/lower_camel.rs#L44)*

This wrapper performs a lower camel case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsLowerCamelCase;

let sentence = "It is we who built these palaces and cities.";
assert_eq!(format!("{}", AsLowerCamelCase(sentence)), "itIsWeWhoBuiltThesePalacesAndCities");
```

#### Trait Implementations

##### `impl<T> Any for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aslowercamelcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsLowerCamelCase<T>`

- <span id="aslowercamelcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aslowercamelcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsShoutyKebabCase<T: AsRef<str>>`

```rust
struct AsShoutyKebabCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/shouty_kebab.rs:41`](../../.source_1765633015/heck-0.5.0/src/shouty_kebab.rs#L41)*

This wrapper performs a kebab case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsShoutyKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsShoutyKebabCase(sentence)), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
```

#### Trait Implementations

##### `impl<T> Any for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="asshoutykebabcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsShoutyKebabCase<T>`

- <span id="asshoutykebabcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="asshoutykebabcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsShoutySnakeCase<T: AsRef<str>>`

```rust
struct AsShoutySnakeCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/shouty_snake.rs:55`](../../.source_1765633015/heck-0.5.0/src/shouty_snake.rs#L55)*

This wrapper performs a shouty snake  case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(format!("{}", AsShoutySnakeCase(sentence)), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Trait Implementations

##### `impl<T> Any for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="asshoutysnakecase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="asshoutysnakecase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsShoutySnekCase<T: AsRef<str>>`

```rust
struct AsShoutySnekCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/shouty_snake.rs:55`](../../.source_1765633015/heck-0.5.0/src/shouty_snake.rs#L55)*

This wrapper performs a shouty snake  case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(format!("{}", AsShoutySnakeCase(sentence)), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Trait Implementations

##### `impl<T> Any for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="asshoutysnakecase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsShoutySnakeCase<T>`

- <span id="asshoutysnakecase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="asshoutysnakecase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsSnakeCase<T: AsRef<str>>`

```rust
struct AsSnakeCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/snake.rs:55`](../../.source_1765633015/heck-0.5.0/src/snake.rs#L55)*

This wrapper performs a snake case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(format!("{}", AsSnakeCase(sentence)), "we_carry_a_new_world_here_in_our_hearts");
```

#### Trait Implementations

##### `impl<T> Any for AsSnakeCase<T>`

- <span id="assnakecase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsSnakeCase<T>`

- <span id="assnakecase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsSnakeCase<T>`

- <span id="assnakecase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsSnakeCase<T>`

- <span id="assnakecase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsSnakeCase<T>`

- <span id="assnakecase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsSnakeCase<T>`

- <span id="assnakecase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsSnakeCase<T>`

- <span id="assnakecase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsSnakeCase<T>`

- <span id="assnakecase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assnakecase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsSnakeCase<T>`

- <span id="assnakecase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assnakecase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsSnekCase<T: AsRef<str>>`

```rust
struct AsSnekCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/snake.rs:55`](../../.source_1765633015/heck-0.5.0/src/snake.rs#L55)*

This wrapper performs a snake case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(format!("{}", AsSnakeCase(sentence)), "we_carry_a_new_world_here_in_our_hearts");
```

#### Trait Implementations

##### `impl<T> Any for AsSnakeCase<T>`

- <span id="assnakecase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsSnakeCase<T>`

- <span id="assnakecase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsSnakeCase<T>`

- <span id="assnakecase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsSnakeCase<T>`

- <span id="assnakecase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsSnakeCase<T>`

- <span id="assnakecase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsSnakeCase<T>`

- <span id="assnakecase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsSnakeCase<T>`

- <span id="assnakecase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsSnakeCase<T>`

- <span id="assnakecase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assnakecase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsSnakeCase<T>`

- <span id="assnakecase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assnakecase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsTitleCase<T: AsRef<str>>`

```rust
struct AsTitleCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/title.rs:44`](../../.source_1765633015/heck-0.5.0/src/title.rs#L44)*

This wrapper performs a title case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsTitleCase;

let sentence = "We have always lived in slums and holes in the wall.";
assert_eq!(format!("{}", AsTitleCase(sentence)), "We Have Always Lived In Slums And Holes In The Wall");
```

#### Trait Implementations

##### `impl<T> Any for AsTitleCase<T>`

- <span id="astitlecase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsTitleCase<T>`

- <span id="astitlecase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsTitleCase<T>`

- <span id="astitlecase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsTitleCase<T>`

- <span id="astitlecase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsTitleCase<T>`

- <span id="astitlecase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsTitleCase<T>`

- <span id="astitlecase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsTitleCase<T>`

- <span id="astitlecase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsTitleCase<T>`

- <span id="astitlecase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="astitlecase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsTitleCase<T>`

- <span id="astitlecase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="astitlecase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsTrainCase<T: AsRef<str>>`

```rust
struct AsTrainCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/train.rs:41`](../../.source_1765633015/heck-0.5.0/src/train.rs#L41)*

This wrapper performs a train case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsTrainCase(sentence)), "We-Are-Going-To-Inherit-The-Earth");
```

#### Trait Implementations

##### `impl<T> Any for AsTrainCase<T>`

- <span id="astraincase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsTrainCase<T>`

- <span id="astraincase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsTrainCase<T>`

- <span id="astraincase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsTrainCase<T>`

- <span id="astraincase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsTrainCase<T>`

- <span id="astraincase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsTrainCase<T>`

- <span id="astraincase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsTrainCase<T>`

- <span id="astraincase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsTrainCase<T>`

- <span id="astraincase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="astraincase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsTrainCase<T>`

- <span id="astraincase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="astraincase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsUpperCamelCase<T: AsRef<str>>`

```rust
struct AsUpperCamelCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:57`](../../.source_1765633015/heck-0.5.0/src/upper_camel.rs#L57)*

This wrapper performs a upper camel case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsUpperCamelCase;

let sentence = "We are not in the least afraid of ruins.";
assert_eq!(format!("{}", AsUpperCamelCase(sentence)), "WeAreNotInTheLeastAfraidOfRuins");
```

#### Trait Implementations

##### `impl<T> Any for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="asuppercamelcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="asuppercamelcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AsPascalCase<T: AsRef<str>>`

```rust
struct AsPascalCase<T: AsRef<str>>(T);
```

*Defined in [`heck-0.5.0/src/upper_camel.rs:57`](../../.source_1765633015/heck-0.5.0/src/upper_camel.rs#L57)*

This wrapper performs a upper camel case conversion in [`fmt::Display`](../miette_derive/index.md).

## Example:

```rust
use heck::AsUpperCamelCase;

let sentence = "We are not in the least afraid of ruins.";
assert_eq!(format!("{}", AsUpperCamelCase(sentence)), "WeAreNotInTheLeastAfraidOfRuins");
```

#### Trait Implementations

##### `impl<T> Any for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: AsRef<str>> Display for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToString for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="asuppercamelcase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AsUpperCamelCase<T>`

- <span id="asuppercamelcase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="asuppercamelcase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ToKebabCase`

```rust
trait ToKebabCase: ToOwned { ... }
```

*Defined in [`heck-0.5.0/src/kebab.rs:19-22`](../../.source_1765633015/heck-0.5.0/src/kebab.rs#L19-L22)*

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

*Defined in [`heck-0.5.0/src/lower_camel.rs:23-26`](../../.source_1765633015/heck-0.5.0/src/lower_camel.rs#L23-L26)*

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

*Defined in [`heck-0.5.0/src/shouty_kebab.rs:20-23`](../../.source_1765633015/heck-0.5.0/src/shouty_kebab.rs#L20-L23)*

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

*Defined in [`heck-0.5.0/src/shouty_snake.rs:20-23`](../../.source_1765633015/heck-0.5.0/src/shouty_snake.rs#L20-L23)*

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

*Defined in [`heck-0.5.0/src/shouty_snake.rs:27-31`](../../.source_1765633015/heck-0.5.0/src/shouty_snake.rs#L27-L31)*

Oh heck, `ToShoutySnekCase` is an alias for [`ToShoutySnakeCase`](shouty_snake/index.md). See
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

*Defined in [`heck-0.5.0/src/snake.rs:21-24`](../../.source_1765633015/heck-0.5.0/src/snake.rs#L21-L24)*

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

*Defined in [`heck-0.5.0/src/snake.rs:28-31`](../../.source_1765633015/heck-0.5.0/src/snake.rs#L28-L31)*

Oh heck, `SnekCase` is an alias for [`ToSnakeCase`](snake/index.md). See ToSnakeCase for
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

*Defined in [`heck-0.5.0/src/title.rs:23-26`](../../.source_1765633015/heck-0.5.0/src/title.rs#L23-L26)*

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

*Defined in [`heck-0.5.0/src/train.rs:20-23`](../../.source_1765633015/heck-0.5.0/src/train.rs#L20-L23)*

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

*Defined in [`heck-0.5.0/src/upper_camel.rs:36-39`](../../.source_1765633015/heck-0.5.0/src/upper_camel.rs#L36-L39)*

`ToPascalCase` is an alias for [`ToUpperCamelCase`](upper_camel/index.md). See ToUpperCamelCase for more
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

*Defined in [`heck-0.5.0/src/upper_camel.rs:23-26`](../../.source_1765633015/heck-0.5.0/src/upper_camel.rs#L23-L26)*

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

*Defined in [`heck-0.5.0/src/lib.rs:69-159`](../../.source_1765633015/heck-0.5.0/src/lib.rs#L69-L159)*

### `lowercase`

```rust
fn lowercase(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`heck-0.5.0/src/lib.rs:161-172`](../../.source_1765633015/heck-0.5.0/src/lib.rs#L161-L172)*

### `uppercase`

```rust
fn uppercase(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`heck-0.5.0/src/lib.rs:174-180`](../../.source_1765633015/heck-0.5.0/src/lib.rs#L174-L180)*

### `capitalize`

```rust
fn capitalize(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`heck-0.5.0/src/lib.rs:182-192`](../../.source_1765633015/heck-0.5.0/src/lib.rs#L182-L192)*

