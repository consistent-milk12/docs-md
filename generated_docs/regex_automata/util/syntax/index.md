*[regex_automata](../../index.md) / [util](../index.md) / [syntax](index.md)*

---

# Module `syntax`

Utilities for dealing with the syntax of a regular expression.

This module currently only exposes a [`Config`](#config) type that
itself represents a wrapper around the configuration for a
[`regex-syntax::ParserBuilder`](regex_syntax::ParserBuilder). The purpose of
this wrapper is to make configuring syntax options very similar to how other
configuration is done throughout this crate. Namely, instead of duplicating
syntax options across every builder (of which there are many), we instead
create small config objects like this one that can be passed around and
composed.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | A common set of configuration options that apply to the syntax of a regex. |
| [`parse`](#parse) | fn | A convenience routine for parsing a pattern into an HIR value with the default configuration. |
| [`parse_many`](#parse-many) | fn | A convenience routine for parsing many patterns into HIR value with the default configuration. |
| [`parse_with`](#parse-with) | fn | A convenience routine for parsing a pattern into an HIR value using a `Config`. |
| [`parse_many_with`](#parse-many-with) | fn | A convenience routine for parsing many patterns into HIR values using a `Config`. |

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

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:145-157`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/syntax.rs#L145-L157)*

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

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

- <span id="config-case-insensitive"></span>`fn case_insensitive(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-multi-line"></span>`fn multi_line(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-dot-matches-new-line"></span>`fn dot_matches_new_line(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-crlf"></span>`fn crlf(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-line-terminator"></span>`fn line_terminator(self, byte: u8) -> Config` — [`Config`](#config)

- <span id="config-swap-greed"></span>`fn swap_greed(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-ignore-whitespace"></span>`fn ignore_whitespace(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-unicode"></span>`fn unicode(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-utf8"></span>`fn utf8(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-nest-limit"></span>`fn nest_limit(self, limit: u32) -> Config` — [`Config`](#config)

- <span id="config-octal"></span>`fn octal(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-get-unicode"></span>`fn get_unicode(&self) -> bool`

- <span id="config-get-case-insensitive"></span>`fn get_case_insensitive(&self) -> bool`

- <span id="config-get-multi-line"></span>`fn get_multi_line(&self) -> bool`

- <span id="config-get-dot-matches-new-line"></span>`fn get_dot_matches_new_line(&self) -> bool`

- <span id="config-get-crlf"></span>`fn get_crlf(&self) -> bool`

- <span id="config-get-line-terminator"></span>`fn get_line_terminator(&self) -> u8`

- <span id="config-get-swap-greed"></span>`fn get_swap_greed(&self) -> bool`

- <span id="config-get-ignore-whitespace"></span>`fn get_ignore_whitespace(&self) -> bool`

- <span id="config-get-utf8"></span>`fn get_utf8(&self) -> bool`

- <span id="config-get-nest-limit"></span>`fn get_nest_limit(&self) -> u32`

- <span id="config-get-octal"></span>`fn get_octal(&self) -> bool`

- <span id="config-apply"></span>`fn apply(&self, builder: &mut ParserBuilder)`

- <span id="config-apply-ast"></span>`fn apply_ast(&self, builder: &mut ast::parse::ParserBuilder)`

- <span id="config-apply-hir"></span>`fn apply_hir(&self, builder: &mut hir::translate::TranslatorBuilder)`

#### Trait Implementations

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl Copy for Config`

##### `impl Debug for Config`

- <span id="config-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

## Functions

### `parse`

```rust
fn parse(pattern: &str) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:37-39`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/syntax.rs#L37-L39)*

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

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:63-65`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/syntax.rs#L63-L65)*

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

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:86-90`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/syntax.rs#L86-L90)*

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

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:118-129`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/syntax.rs#L118-L129)*

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

