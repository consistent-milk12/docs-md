*[regex](../../index.md) / [builders](../index.md) / [bytes](index.md)*

---

# Module `bytes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RegexBuilder`](#regexbuilder) | struct | A configurable builder for a [`Regex`]. |
| [`RegexSetBuilder`](#regexsetbuilder) | struct | A configurable builder for a [`RegexSet`]. |

## Structs

### `RegexBuilder`

```rust
struct RegexBuilder {
    builder: super::Builder,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:1372-1374`](../../../../.source_1765521767/regex-1.12.2/src/builders.rs#L1372-L1374)*

A configurable builder for a [`Regex`](../../regex/bytes/index.md).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- <span id="regexbuilder-new"></span>`fn new(pattern: &str) -> RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-build"></span>`fn build(&self) -> Result<Regex, Error>` — [`Regex`](../../regex/bytes/index.md#regex), [`Error`](../../error/index.md#error)

- <span id="regexbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

- <span id="regexbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

#### Trait Implementations

##### `impl Clone for RegexBuilder`

- <span id="regexbuilder-clone"></span>`fn clone(&self) -> RegexBuilder` — [`RegexBuilder`](../../bytes/index.md#regexbuilder)

##### `impl Debug for RegexBuilder`

- <span id="regexbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RegexSetBuilder`

```rust
struct RegexSetBuilder {
    builder: super::Builder,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:1966-1968`](../../../../.source_1765521767/regex-1.12.2/src/builders.rs#L1966-L1968)*

A configurable builder for a [`RegexSet`](../../regexset/bytes/index.md).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- <span id="regexsetbuilder-new"></span>`fn new<I, S>(patterns: I) -> RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-build"></span>`fn build(&self) -> Result<RegexSet, Error>` — [`RegexSet`](../../regexset/bytes/index.md#regexset), [`Error`](../../error/index.md#error)

- <span id="regexsetbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

- <span id="regexsetbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

#### Trait Implementations

##### `impl Clone for RegexSetBuilder`

- <span id="regexsetbuilder-clone"></span>`fn clone(&self) -> RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md#regexsetbuilder)

##### `impl Debug for RegexSetBuilder`

- <span id="regexsetbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

