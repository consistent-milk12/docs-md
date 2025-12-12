*[proc_macro2](../index.md) / [parse](index.md)*

---

# Module `parse`

## Contents

- [Structs](#structs)
  - [`Cursor`](#cursor)
  - [`Reject`](#reject)
- [Functions](#functions)
  - [`skip_whitespace`](#skip-whitespace)
  - [`block_comment`](#block-comment)
  - [`is_whitespace`](#is-whitespace)
  - [`word_break`](#word-break)
  - [`token_stream`](#token-stream)
  - [`lex_error`](#lex-error)
  - [`leaf_token`](#leaf-token)
  - [`ident`](#ident)
  - [`ident_any`](#ident-any)
  - [`ident_not_raw`](#ident-not-raw)
  - [`literal`](#literal)
  - [`literal_nocapture`](#literal-nocapture)
  - [`literal_suffix`](#literal-suffix)
  - [`string`](#string)
  - [`cooked_string`](#cooked-string)
  - [`raw_string`](#raw-string)
  - [`byte_string`](#byte-string)
  - [`cooked_byte_string`](#cooked-byte-string)
  - [`delimiter_of_raw_string`](#delimiter-of-raw-string)
  - [`raw_byte_string`](#raw-byte-string)
  - [`c_string`](#c-string)
  - [`raw_c_string`](#raw-c-string)
  - [`cooked_c_string`](#cooked-c-string)
  - [`byte`](#byte)
  - [`character`](#character)
  - [`backslash_x_char`](#backslash-x-char)
  - [`backslash_x_byte`](#backslash-x-byte)
  - [`backslash_x_nonzero`](#backslash-x-nonzero)
  - [`backslash_u`](#backslash-u)
  - [`trailing_backslash`](#trailing-backslash)
  - [`float`](#float)
  - [`float_digits`](#float-digits)
  - [`int`](#int)
  - [`digits`](#digits)
  - [`punct`](#punct)
  - [`punct_char`](#punct-char)
  - [`doc_comment`](#doc-comment)
  - [`doc_comment_contents`](#doc-comment-contents)
  - [`take_until_newline_or_eof`](#take-until-newline-or-eof)
- [Type Aliases](#type-aliases)
  - [`PResult`](#presult)
- [Constants](#constants)
  - [`ERROR`](#error)
- [Macros](#macros)
  - [`next_ch!`](#next-ch)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Cursor`](#cursor) | struct |  |
| [`Reject`](#reject) | struct |  |
| [`skip_whitespace`](#skip-whitespace) | fn |  |
| [`block_comment`](#block-comment) | fn |  |
| [`is_whitespace`](#is-whitespace) | fn |  |
| [`word_break`](#word-break) | fn |  |
| [`token_stream`](#token-stream) | fn |  |
| [`lex_error`](#lex-error) | fn |  |
| [`leaf_token`](#leaf-token) | fn |  |
| [`ident`](#ident) | fn |  |
| [`ident_any`](#ident-any) | fn |  |
| [`ident_not_raw`](#ident-not-raw) | fn |  |
| [`literal`](#literal) | fn |  |
| [`literal_nocapture`](#literal-nocapture) | fn |  |
| [`literal_suffix`](#literal-suffix) | fn |  |
| [`string`](#string) | fn |  |
| [`cooked_string`](#cooked-string) | fn |  |
| [`raw_string`](#raw-string) | fn |  |
| [`byte_string`](#byte-string) | fn |  |
| [`cooked_byte_string`](#cooked-byte-string) | fn |  |
| [`delimiter_of_raw_string`](#delimiter-of-raw-string) | fn |  |
| [`raw_byte_string`](#raw-byte-string) | fn |  |
| [`c_string`](#c-string) | fn |  |
| [`raw_c_string`](#raw-c-string) | fn |  |
| [`cooked_c_string`](#cooked-c-string) | fn |  |
| [`byte`](#byte) | fn |  |
| [`character`](#character) | fn |  |
| [`backslash_x_char`](#backslash-x-char) | fn |  |
| [`backslash_x_byte`](#backslash-x-byte) | fn |  |
| [`backslash_x_nonzero`](#backslash-x-nonzero) | fn |  |
| [`backslash_u`](#backslash-u) | fn |  |
| [`trailing_backslash`](#trailing-backslash) | fn |  |
| [`float`](#float) | fn |  |
| [`float_digits`](#float-digits) | fn |  |
| [`int`](#int) | fn |  |
| [`digits`](#digits) | fn |  |
| [`punct`](#punct) | fn |  |
| [`punct_char`](#punct-char) | fn |  |
| [`doc_comment`](#doc-comment) | fn |  |
| [`doc_comment_contents`](#doc-comment-contents) | fn |  |
| [`take_until_newline_or_eof`](#take-until-newline-or-eof) | fn |  |
| [`PResult`](#presult) | type |  |
| [`ERROR`](#error) | const |  |
| [`next_ch!`](#next-ch) | macro |  |

## Structs

### `Cursor<'a>`

```rust
struct Cursor<'a> {
    rest: &'a str,
    off: u32,
}
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:10-14`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L10-L14)*

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

##### `impl Clone for Cursor<'a>`

- <span id="cursor-clone"></span>`fn clone(&self) -> Cursor<'a>` — [`Cursor`](#cursor)

##### `impl Copy for Cursor<'a>`

##### `impl Eq for Cursor<'a>`

##### `impl PartialEq for Cursor<'a>`

- <span id="cursor-eq"></span>`fn eq(&self, other: &Cursor<'a>) -> bool` — [`Cursor`](#cursor)

##### `impl StructuralPartialEq for Cursor<'a>`

### `Reject`

```rust
struct Reject;
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:74`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L74)*

## Functions

### `skip_whitespace`

```rust
fn skip_whitespace(input: Cursor<'_>) -> Cursor<'_>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:77-123`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L77-L123)*

### `block_comment`

```rust
fn block_comment(input: Cursor<'_>) -> Result<(Cursor<'_>, &str), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:125-150`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L125-L150)*

### `is_whitespace`

```rust
fn is_whitespace(ch: char) -> bool
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:152-155`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L152-L155)*

### `word_break`

```rust
fn word_break(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:157-162`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L157-L162)*

### `token_stream`

```rust
fn token_stream(input: Cursor<'_>) -> Result<crate::fallback::TokenStream, crate::fallback::LexError>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:168-251`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L168-L251)*

### `lex_error`

```rust
fn lex_error(cursor: Cursor<'_>) -> crate::fallback::LexError
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:253-264`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L253-L264)*

### `leaf_token`

```rust
fn leaf_token(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::TokenTree), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:266-281`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L266-L281)*

### `ident`

```rust
fn ident(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Ident), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:283-294`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L283-L294)*

### `ident_any`

```rust
fn ident_any(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Ident), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:296-316`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L296-L316)*

### `ident_not_raw`

```rust
fn ident_not_raw(input: Cursor<'_>) -> Result<(Cursor<'_>, &str), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:318-335`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L318-L335)*

### `literal`

```rust
fn literal(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::fallback::Literal), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:337-341`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L337-L341)*

### `literal_nocapture`

```rust
fn literal_nocapture(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:343-361`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L343-L361)*

### `literal_suffix`

```rust
fn literal_suffix(input: Cursor<'_>) -> Cursor<'_>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:363-368`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L363-L368)*

### `string`

```rust
fn string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:370-378`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L370-L378)*

### `cooked_string`

```rust
fn cooked_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:380-412`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L380-L412)*

### `raw_string`

```rust
fn raw_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:414-431`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L414-L431)*

### `byte_string`

```rust
fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:433-441`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L433-L441)*

### `cooked_byte_string`

```rust
fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:443-472`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L443-L472)*

### `delimiter_of_raw_string`

```rust
fn delimiter_of_raw_string(input: Cursor<'_>) -> Result<(Cursor<'_>, &str), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:474-489`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L474-L489)*

### `raw_byte_string`

```rust
fn raw_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:491-512`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L491-L512)*

### `c_string`

```rust
fn c_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:514-522`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L514-L522)*

### `raw_c_string`

```rust
fn raw_c_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:524-542`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L524-L542)*

### `cooked_c_string`

```rust
fn cooked_c_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:544-579`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L544-L579)*

### `byte`

```rust
fn byte(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:581-601`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L581-L601)*

### `character`

```rust
fn character(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:603-621`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L603-L621)*

### `backslash_x_char`

```rust
fn backslash_x_char<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, char)>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:635-642`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L635-L642)*

### `backslash_x_byte`

```rust
fn backslash_x_byte<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, u8)>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:644-651`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L644-L651)*

### `backslash_x_nonzero`

```rust
fn backslash_x_nonzero<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, char)>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:653-664`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L653-L664)*

### `backslash_u`

```rust
fn backslash_u<I>(chars: &mut I) -> Result<char, Reject>
where
    I: Iterator<Item = (usize, char)>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:666-690`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L666-L690)*

### `trailing_backslash`

```rust
fn trailing_backslash(input: &mut Cursor<'_>, last: u8) -> Result<(), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:692-709`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L692-L709)*

### `float`

```rust
fn float(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:711-719`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L711-L719)*

### `float_digits`

```rust
fn float_digits(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:721-804`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L721-L804)*

### `int`

```rust
fn int(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:806-814`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L806-L814)*

### `digits`

```rust
fn digits(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:816-869`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L816-L869)*

### `punct`

```rust
fn punct(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Punct), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:871-889`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L871-L889)*

### `punct_char`

```rust
fn punct_char(input: Cursor<'_>) -> Result<(Cursor<'_>, char), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:891-910`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L891-L910)*

### `doc_comment`

```rust
fn doc_comment<'a>(input: Cursor<'a>, trees: &mut crate::fallback::TokenStreamBuilder) -> Result<(Cursor<'a>, ()), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:912-958`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L912-L958)*

### `doc_comment_contents`

```rust
fn doc_comment_contents(input: Cursor<'_>) -> Result<(Cursor<'_>, (&str, bool)), Reject>
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:960-981`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L960-L981)*

### `take_until_newline_or_eof`

```rust
fn take_until_newline_or_eof(input: Cursor<'_>) -> (Cursor<'_>, &str)
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:983-995`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L983-L995)*

## Type Aliases

### `PResult<'a, O>`

```rust
type PResult<'a, O> = Result<(Cursor<'a>, O), Reject>;
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:75`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L75)*

## Constants

### `ERROR`
```rust
const ERROR: &str;
```

*Defined in [`proc-macro2-1.0.103/src/parse.rs:166`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L166)*

## Macros

### `next_ch!`

*Defined in [`proc-macro2-1.0.103/src/parse.rs:623-633`](../../../.source_1765521767/proc-macro2-1.0.103/src/parse.rs#L623-L633)*

