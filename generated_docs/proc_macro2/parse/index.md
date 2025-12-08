*[proc_macro2](../index.md) / [parse](index.md)*

---

# Module `parse`

## Contents

- [Structs](#structs)
  - [`Cursor`](#cursor)
  - [`Reject`](#reject)
- [Functions](#functions)
  - [`skip_whitespace`](#skip_whitespace)
  - [`block_comment`](#block_comment)
  - [`is_whitespace`](#is_whitespace)
  - [`word_break`](#word_break)
  - [`token_stream`](#token_stream)
  - [`lex_error`](#lex_error)
  - [`leaf_token`](#leaf_token)
  - [`ident`](#ident)
  - [`ident_any`](#ident_any)
  - [`ident_not_raw`](#ident_not_raw)
  - [`literal`](#literal)
  - [`literal_nocapture`](#literal_nocapture)
  - [`literal_suffix`](#literal_suffix)
  - [`string`](#string)
  - [`cooked_string`](#cooked_string)
  - [`raw_string`](#raw_string)
  - [`byte_string`](#byte_string)
  - [`cooked_byte_string`](#cooked_byte_string)
  - [`delimiter_of_raw_string`](#delimiter_of_raw_string)
  - [`raw_byte_string`](#raw_byte_string)
  - [`c_string`](#c_string)
  - [`raw_c_string`](#raw_c_string)
  - [`cooked_c_string`](#cooked_c_string)
  - [`byte`](#byte)
  - [`character`](#character)
  - [`backslash_x_char`](#backslash_x_char)
  - [`backslash_x_byte`](#backslash_x_byte)
  - [`backslash_x_nonzero`](#backslash_x_nonzero)
  - [`backslash_u`](#backslash_u)
  - [`trailing_backslash`](#trailing_backslash)
  - [`float`](#float)
  - [`float_digits`](#float_digits)
  - [`int`](#int)
  - [`digits`](#digits)
  - [`punct`](#punct)
  - [`punct_char`](#punct_char)
  - [`doc_comment`](#doc_comment)
  - [`doc_comment_contents`](#doc_comment_contents)
  - [`take_until_newline_or_eof`](#take_until_newline_or_eof)
- [Type Aliases](#type-aliases)
  - [`PResult`](#presult)
- [Constants](#constants)
  - [`ERROR`](#error)
- [Macros](#macros)
  - [`next_ch!`](#next_ch)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Cursor`](#cursor) | struct |  |
| [`Reject`](#reject) | struct |  |
| [`skip_whitespace`](#skip_whitespace) | fn |  |
| [`block_comment`](#block_comment) | fn |  |
| [`is_whitespace`](#is_whitespace) | fn |  |
| [`word_break`](#word_break) | fn |  |
| [`token_stream`](#token_stream) | fn |  |
| [`lex_error`](#lex_error) | fn |  |
| [`leaf_token`](#leaf_token) | fn |  |
| [`ident`](#ident) | fn |  |
| [`ident_any`](#ident_any) | fn |  |
| [`ident_not_raw`](#ident_not_raw) | fn |  |
| [`literal`](#literal) | fn |  |
| [`literal_nocapture`](#literal_nocapture) | fn |  |
| [`literal_suffix`](#literal_suffix) | fn |  |
| [`string`](#string) | fn |  |
| [`cooked_string`](#cooked_string) | fn |  |
| [`raw_string`](#raw_string) | fn |  |
| [`byte_string`](#byte_string) | fn |  |
| [`cooked_byte_string`](#cooked_byte_string) | fn |  |
| [`delimiter_of_raw_string`](#delimiter_of_raw_string) | fn |  |
| [`raw_byte_string`](#raw_byte_string) | fn |  |
| [`c_string`](#c_string) | fn |  |
| [`raw_c_string`](#raw_c_string) | fn |  |
| [`cooked_c_string`](#cooked_c_string) | fn |  |
| [`byte`](#byte) | fn |  |
| [`character`](#character) | fn |  |
| [`backslash_x_char`](#backslash_x_char) | fn |  |
| [`backslash_x_byte`](#backslash_x_byte) | fn |  |
| [`backslash_x_nonzero`](#backslash_x_nonzero) | fn |  |
| [`backslash_u`](#backslash_u) | fn |  |
| [`trailing_backslash`](#trailing_backslash) | fn |  |
| [`float`](#float) | fn |  |
| [`float_digits`](#float_digits) | fn |  |
| [`int`](#int) | fn |  |
| [`digits`](#digits) | fn |  |
| [`punct`](#punct) | fn |  |
| [`punct_char`](#punct_char) | fn |  |
| [`doc_comment`](#doc_comment) | fn |  |
| [`doc_comment_contents`](#doc_comment_contents) | fn |  |
| [`take_until_newline_or_eof`](#take_until_newline_or_eof) | fn |  |
| [`PResult`](#presult) | type |  |
| [`ERROR`](#error) | const |  |
| [`next_ch!`](#next_ch) | macro |  |

## Structs

### `Cursor<'a>`

```rust
struct Cursor<'a> {
    rest: &'a str,
}
```

#### Implementations

- <span id="cursor-advance"></span>`fn advance(&self, bytes: usize) -> Cursor<'a>` — [`Cursor`](#cursor)

- <span id="cursor-starts-with"></span>`fn starts_with(&self, s: &str) -> bool`

- <span id="cursor-starts-with-char"></span>`fn starts_with_char(&self, ch: char) -> bool`

- <span id="cursor-starts-with-fn"></span>`fn starts_with_fn<Pattern>(&self, f: Pattern) -> bool`

- <span id="cursor-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="cursor-len"></span>`fn len(&self) -> usize`

- <span id="cursor-as-bytes"></span>`fn as_bytes(&self) -> &'a [u8]`

- <span id="cursor-bytes"></span>`fn bytes(&self) -> Bytes<'a>`

- <span id="cursor-chars"></span>`fn chars(&self) -> Chars<'a>`

- <span id="cursor-char-indices"></span>`fn char_indices(&self) -> CharIndices<'a>`

- <span id="cursor-parse"></span>`fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject>` — [`Cursor`](#cursor), [`Reject`](#reject)

#### Trait Implementations

##### `impl<'a> Clone for Cursor<'a>`

- <span id="cursor-clone"></span>`fn clone(&self) -> Cursor<'a>` — [`Cursor`](#cursor)

##### `impl<'a> Copy for Cursor<'a>`

##### `impl<'a> Eq for Cursor<'a>`

##### `impl<'a> PartialEq for Cursor<'a>`

- <span id="cursor-eq"></span>`fn eq(&self, other: &Cursor<'a>) -> bool` — [`Cursor`](#cursor)

##### `impl<'a> StructuralPartialEq for Cursor<'a>`

### `Reject`

```rust
struct Reject;
```

## Functions

### `skip_whitespace`

```rust
fn skip_whitespace(input: Cursor<'_>) -> Cursor<'_>
```

### `block_comment`

```rust
fn block_comment(input: Cursor<'_>) -> Result<(Cursor<'_>, &str), Reject>
```

### `is_whitespace`

```rust
fn is_whitespace(ch: char) -> bool
```

### `word_break`

```rust
fn word_break(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `token_stream`

```rust
fn token_stream(input: Cursor<'_>) -> Result<crate::fallback::TokenStream, crate::fallback::LexError>
```

### `lex_error`

```rust
fn lex_error(cursor: Cursor<'_>) -> crate::fallback::LexError
```

### `leaf_token`

```rust
fn leaf_token(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::TokenTree), Reject>
```

### `ident`

```rust
fn ident(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Ident), Reject>
```

### `ident_any`

```rust
fn ident_any(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Ident), Reject>
```

### `ident_not_raw`

```rust
fn ident_not_raw(input: Cursor<'_>) -> Result<(Cursor<'_>, &str), Reject>
```

### `literal`

```rust
fn literal(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::fallback::Literal), Reject>
```

### `literal_nocapture`

```rust
fn literal_nocapture(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `literal_suffix`

```rust
fn literal_suffix(input: Cursor<'_>) -> Cursor<'_>
```

### `string`

```rust
fn string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `cooked_string`

```rust
fn cooked_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `raw_string`

```rust
fn raw_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `byte_string`

```rust
fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `cooked_byte_string`

```rust
fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `delimiter_of_raw_string`

```rust
fn delimiter_of_raw_string(input: Cursor<'_>) -> Result<(Cursor<'_>, &str), Reject>
```

### `raw_byte_string`

```rust
fn raw_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `c_string`

```rust
fn c_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `raw_c_string`

```rust
fn raw_c_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `cooked_c_string`

```rust
fn cooked_c_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `byte`

```rust
fn byte(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `character`

```rust
fn character(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `backslash_x_char`

```rust
fn backslash_x_char<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, char)>
```

### `backslash_x_byte`

```rust
fn backslash_x_byte<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, u8)>
```

### `backslash_x_nonzero`

```rust
fn backslash_x_nonzero<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, char)>
```

### `backslash_u`

```rust
fn backslash_u<I>(chars: &mut I) -> Result<char, Reject>
where
    I: Iterator<Item = (usize, char)>
```

### `trailing_backslash`

```rust
fn trailing_backslash(input: &mut Cursor<'_>, last: u8) -> Result<(), Reject>
```

### `float`

```rust
fn float(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `float_digits`

```rust
fn float_digits(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `int`

```rust
fn int(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `digits`

```rust
fn digits(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

### `punct`

```rust
fn punct(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Punct), Reject>
```

### `punct_char`

```rust
fn punct_char(input: Cursor<'_>) -> Result<(Cursor<'_>, char), Reject>
```

### `doc_comment`

```rust
fn doc_comment<'a>(input: Cursor<'a>, trees: &mut crate::fallback::TokenStreamBuilder) -> Result<(Cursor<'a>, ()), Reject>
```

### `doc_comment_contents`

```rust
fn doc_comment_contents(input: Cursor<'_>) -> Result<(Cursor<'_>, (&str, bool)), Reject>
```

### `take_until_newline_or_eof`

```rust
fn take_until_newline_or_eof(input: Cursor<'_>) -> (Cursor<'_>, &str)
```

## Type Aliases

### `PResult<'a, O>`

```rust
type PResult<'a, O> = Result<(Cursor<'a>, O), Reject>;
```

## Constants

### `ERROR`

```rust
const ERROR: &str;
```

## Macros

### `next_ch!`

