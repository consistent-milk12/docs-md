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

## Structs

### `AsKebabCase<T: AsRef<str>>`

```rust
struct AsKebabCase<T: AsRef<str>>(T);
```

This wrapper performs a kebab case conversion in `fmt::Display`.

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

### `AsLowerCamelCase<T: AsRef<str>>`

```rust
struct AsLowerCamelCase<T: AsRef<str>>(T);
```

This wrapper performs a lower camel case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsLowerCamelCase;

let sentence = "It is we who built these palaces and cities.";
assert_eq!(format!("{}", AsLowerCamelCase(sentence)), "itIsWeWhoBuiltThesePalacesAndCities");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsLowerCamelCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsLowerCamelCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsShoutyKebabCase<T: AsRef<str>>`

```rust
struct AsShoutyKebabCase<T: AsRef<str>>(T);
```

This wrapper performs a kebab case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsShoutyKebabCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsShoutyKebabCase(sentence)), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutyKebabCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutyKebabCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsShoutySnakeCase<T: AsRef<str>>`

```rust
struct AsShoutySnakeCase<T: AsRef<str>>(T);
```

This wrapper performs a shouty snake  case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(format!("{}", AsShoutySnakeCase(sentence)), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutySnakeCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutySnakeCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsShoutySnekCase<T: AsRef<str>>`

```rust
struct AsShoutySnekCase<T: AsRef<str>>(T);
```

This wrapper performs a shouty snake  case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsShoutySnakeCase;

let sentence = "That world is growing in this minute.";
assert_eq!(format!("{}", AsShoutySnakeCase(sentence)), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsShoutySnakeCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsShoutySnakeCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsSnakeCase<T: AsRef<str>>`

```rust
struct AsSnakeCase<T: AsRef<str>>(T);
```

This wrapper performs a snake case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(format!("{}", AsSnakeCase(sentence)), "we_carry_a_new_world_here_in_our_hearts");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsSnakeCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsSnakeCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsSnekCase<T: AsRef<str>>`

```rust
struct AsSnekCase<T: AsRef<str>>(T);
```

This wrapper performs a snake case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsSnakeCase;

let sentence = "We carry a new world here, in our hearts.";
assert_eq!(format!("{}", AsSnakeCase(sentence)), "we_carry_a_new_world_here_in_our_hearts");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsSnakeCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsSnakeCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsTitleCase<T: AsRef<str>>`

```rust
struct AsTitleCase<T: AsRef<str>>(T);
```

This wrapper performs a title case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsTitleCase;

let sentence = "We have always lived in slums and holes in the wall.";
assert_eq!(format!("{}", AsTitleCase(sentence)), "We Have Always Lived In Slums And Holes In The Wall");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsTitleCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsTitleCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsTrainCase<T: AsRef<str>>`

```rust
struct AsTrainCase<T: AsRef<str>>(T);
```

This wrapper performs a train case conversion in `fmt::Display`.

## Example:

```rust
use heck::AsTrainCase;

let sentence = "We are going to inherit the earth.";
assert_eq!(format!("{}", AsTrainCase(sentence)), "We-Are-Going-To-Inherit-The-Earth");
```

#### Trait Implementations

##### `impl<T: AsRef<str>> Display for AsTrainCase<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for AsTrainCase<T>`

- `fn to_string(self: &Self) -> String`

### `AsUpperCamelCase<T: AsRef<str>>`

```rust
struct AsUpperCamelCase<T: AsRef<str>>(T);
```

This wrapper performs a upper camel case conversion in `fmt::Display`.

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

### `AsPascalCase<T: AsRef<str>>`

```rust
struct AsPascalCase<T: AsRef<str>>(T);
```

This wrapper performs a upper camel case conversion in `fmt::Display`.

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

