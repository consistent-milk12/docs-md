*[proc_macro2](../index.md) / [parse](index.md)*

---

# Module `parse`

## Structs

### `Cursor<'a>`

```rust
struct Cursor<'a> {
    rest: &'a str,
}
```

#### Implementations

- `fn advance(self: &Self, bytes: usize) -> Cursor<'a>` — [`Cursor`](#cursor)

- `fn starts_with(self: &Self, s: &str) -> bool`

- `fn starts_with_char(self: &Self, ch: char) -> bool`

- `fn starts_with_fn<Pattern>(self: &Self, f: Pattern) -> bool`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn as_bytes(self: &Self) -> &'a [u8]`

- `fn bytes(self: &Self) -> Bytes<'a>`

- `fn chars(self: &Self) -> Chars<'a>`

- `fn char_indices(self: &Self) -> CharIndices<'a>`

- `fn parse(self: &Self, tag: &str) -> Result<Cursor<'a>, Reject>` — [`Cursor`](#cursor), [`Reject`](#reject)

#### Trait Implementations

##### `impl<'a> Clone for Cursor<'a>`

- `fn clone(self: &Self) -> Cursor<'a>` — [`Cursor`](#cursor)

##### `impl<'a> Copy for Cursor<'a>`

##### `impl<'a> Eq for Cursor<'a>`

##### `impl<'a> PartialEq for Cursor<'a>`

- `fn eq(self: &Self, other: &Cursor<'a>) -> bool` — [`Cursor`](#cursor)

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

