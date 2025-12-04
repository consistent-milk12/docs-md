*[regex_automata](../../index.md) / [util](../index.md) / [syntax](index.md)*

---

# Module `syntax`

Utilities for dealing with the syntax of a regular expression.

This module currently only exposes a [`Config`](dfa/onepass/index.md) type that
itself represents a wrapper around the configuration for a
[`regex-syntax::ParserBuilder`](regex_syntax::ParserBuilder). The purpose of
this wrapper is to make configuring syntax options very similar to how other
configuration is done throughout this crate. Namely, instead of duplicating
syntax options across every builder (of which there are many), we instead
create small config objects like this one that can be passed around and
composed.

## Structs

### `Config`

```rust
struct Config {
    // [REDACTED: Private Fields]
}
```

A common set of configuration options that apply to the syntax of a regex.

This represents a group of configuration options that specifically apply
to how the concrete syntax of a regular expression is interpreted. In
particular, they are generally forwarded to the
[`ParserBuilder`](https://docs.rs/regex-syntax/*/regex_syntax/struct.ParserBuilder.html)
in the
[`regex-syntax`](https://docs.rs/regex-syntax)
crate when building a regex from its concrete syntax directly.

These options are defined as a group since they apply to every regex engine
in this crate. Instead of re-defining them on every engine's builder, they
are instead provided here as one cohesive unit.

#### Implementations

- `fn new() -> Config`
  Return a new default syntax configuration.

- `fn case_insensitive(self: Self, yes: bool) -> Config`
  Enable or disable the case insensitive flag by default.

- `fn multi_line(self: Self, yes: bool) -> Config`
  Enable or disable the multi-line matching flag by default.

- `fn dot_matches_new_line(self: Self, yes: bool) -> Config`
  Enable or disable the "dot matches any character" flag by default.

- `fn crlf(self: Self, yes: bool) -> Config`
  Enable or disable the "CRLF mode" flag by default.

- `fn line_terminator(self: Self, byte: u8) -> Config`
  Sets the line terminator for use with `(?u-s:.)` and `(?-us:.)`.

- `fn swap_greed(self: Self, yes: bool) -> Config`
  Enable or disable the "swap greed" flag by default.

- `fn ignore_whitespace(self: Self, yes: bool) -> Config`
  Enable verbose mode in the regular expression.

- `fn unicode(self: Self, yes: bool) -> Config`
  Enable or disable the Unicode flag (`u`) by default.

- `fn utf8(self: Self, yes: bool) -> Config`
  When disabled, the builder will permit the construction of a regular

- `fn nest_limit(self: Self, limit: u32) -> Config`
  Set the nesting limit used for the regular expression parser.

- `fn octal(self: Self, yes: bool) -> Config`
  Whether to support octal syntax or not.

- `fn get_unicode(self: &Self) -> bool`
  Returns whether "unicode" mode is enabled.

- `fn get_case_insensitive(self: &Self) -> bool`
  Returns whether "case insensitive" mode is enabled.

- `fn get_multi_line(self: &Self) -> bool`
  Returns whether "multi line" mode is enabled.

- `fn get_dot_matches_new_line(self: &Self) -> bool`
  Returns whether "dot matches new line" mode is enabled.

- `fn get_crlf(self: &Self) -> bool`
  Returns whether "CRLF" mode is enabled.

- `fn get_line_terminator(self: &Self) -> u8`
  Returns the line terminator in this syntax configuration.

- `fn get_swap_greed(self: &Self) -> bool`
  Returns whether "swap greed" mode is enabled.

- `fn get_ignore_whitespace(self: &Self) -> bool`
  Returns whether "ignore whitespace" mode is enabled.

- `fn get_utf8(self: &Self) -> bool`
  Returns whether UTF-8 mode is enabled.

- `fn get_nest_limit(self: &Self) -> u32`
  Returns the "nest limit" setting.

- `fn get_octal(self: &Self) -> bool`
  Returns whether "octal" mode is enabled.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Config`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Config`

## Functions

### `parse`

```rust
fn parse(pattern: &str) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

A convenience routine for parsing a pattern into an HIR value with the
default configuration.

# Example

This shows how to parse a pattern into an HIR value:

```
use regex_automata::util::syntax;

let hir = syntax::parse(r"([a-z]+)|([0-9]+)")?;
assert_eq!(Some(1), hir.properties().static_explicit_captures_len());

# Ok::<(), Box<dyn std::error::Error>>(())
```

### `parse_many`

```rust
fn parse_many<P: AsRef<str>>(patterns: &[P]) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error>
```

A convenience routine for parsing many patterns into HIR value with the
default configuration.

# Example

This shows how to parse many patterns into an corresponding HIR values:

```
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let hirs = syntax::parse_many(&[
    r"([a-z]+)|([0-9]+)",
    r"foo(A-Z]+)bar",
])?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert_eq!(Some(1), props.static_explicit_captures_len());

# Ok::<(), Box<dyn std::error::Error>>(())
```

### `parse_with`

```rust
fn parse_with(pattern: &str, config: &Config) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

A convenience routine for parsing a pattern into an HIR value using a
`Config`.

# Example

This shows how to parse a pattern into an HIR value with a non-default
configuration:

```
use regex_automata::util::syntax;

let hir = syntax::parse_with(
    r"^[a-z]+$",
    &syntax::Config::new().multi_line(true).crlf(true),
)?;
assert!(hir.properties().look_set().contains_anchor_crlf());

# Ok::<(), Box<dyn std::error::Error>>(())
```

### `parse_many_with`

```rust
fn parse_many_with<P: AsRef<str>>(patterns: &[P], config: &Config) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error>
```

A convenience routine for parsing many patterns into HIR values using a
`Config`.

# Example

This shows how to parse many patterns into an corresponding HIR values
with a non-default configuration:

```
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let patterns = &[
    r"([a-z]+)|([0-9]+)",
    r"\W",
    r"foo(A-Z]+)bar",
];
let config = syntax::Config::new().unicode(false).utf8(false);
let hirs = syntax::parse_many_with(patterns, &config)?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert!(!props.is_utf8());

# Ok::<(), Box<dyn std::error::Error>>(())
```

