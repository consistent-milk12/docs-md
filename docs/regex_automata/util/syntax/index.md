*[regex_automata](../../index.md) / [util](../index.md) / [syntax](index.md)*

---

# Module `syntax`

Utilities for dealing with the syntax of a regular expression.

This module currently only exposes a [`Config`](../../dfa/onepass/index.md) type that
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
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
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

- `fn new() -> Config` — [`Config`](#config)

- `fn case_insensitive(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn multi_line(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn dot_matches_new_line(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn crlf(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn line_terminator(self: Self, byte: u8) -> Config` — [`Config`](#config)

- `fn swap_greed(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn ignore_whitespace(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn unicode(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn utf8(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn nest_limit(self: Self, limit: u32) -> Config` — [`Config`](#config)

- `fn octal(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn get_unicode(self: &Self) -> bool`

- `fn get_case_insensitive(self: &Self) -> bool`

- `fn get_multi_line(self: &Self) -> bool`

- `fn get_dot_matches_new_line(self: &Self) -> bool`

- `fn get_crlf(self: &Self) -> bool`

- `fn get_line_terminator(self: &Self) -> u8`

- `fn get_swap_greed(self: &Self) -> bool`

- `fn get_ignore_whitespace(self: &Self) -> bool`

- `fn get_utf8(self: &Self) -> bool`

- `fn get_nest_limit(self: &Self) -> u32`

- `fn get_octal(self: &Self) -> bool`

- `fn apply(self: &Self, builder: &mut ParserBuilder)`

- `fn apply_ast(self: &Self, builder: &mut ast::parse::ParserBuilder)`

- `fn apply_hir(self: &Self, builder: &mut hir::translate::TranslatorBuilder)`

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](#config)

##### `impl Copy for Config`

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Config`

- `fn default() -> Config` — [`Config`](#config)

## Functions

### `parse`

```rust
fn parse(pattern: &str) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

A convenience routine for parsing a pattern into an HIR value with the
default configuration.

# Example

This shows how to parse a pattern into an HIR value:

```rust
use regex_automata::util::syntax;

let hir = syntax::parse(r"([a-z]+)|([0-9]+)")?;
assert_eq!(Some(1), hir.properties().static_explicit_captures_len());

Ok::<(), Box<dyn std::error::Error>>(())
```

### `parse_many`

```rust
fn parse_many<P: AsRef<str>>(patterns: &[P]) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error>
```

A convenience routine for parsing many patterns into HIR value with the
default configuration.

# Example

This shows how to parse many patterns into an corresponding HIR values:

```rust
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

Ok::<(), Box<dyn std::error::Error>>(())
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

```rust
use regex_automata::util::syntax;

let hir = syntax::parse_with(
    r"^[a-z]+$",
    &syntax::Config::new().multi_line(true).crlf(true),
)?;
assert!(hir.properties().look_set().contains_anchor_crlf());

Ok::<(), Box<dyn std::error::Error>>(())
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

```rust
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

Ok::<(), Box<dyn std::error::Error>>(())
```

