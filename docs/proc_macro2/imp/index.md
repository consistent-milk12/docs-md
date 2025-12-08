*[proc_macro2](../index.md) / [imp](index.md)*

---

# Module `imp`

## Structs

### `DeferredTokenStream`

```rust
struct DeferredTokenStream {
    stream: proc_macro::TokenStream,
    extra: Vec<proc_macro::TokenTree>,
}
```

#### Implementations

- `fn new(stream: proc_macro::TokenStream) -> Self`

- `fn is_empty(self: &Self) -> bool`

- `fn evaluate_now(self: &mut Self)`

- `fn into_token_stream(self: Self) -> proc_macro::TokenStream`

#### Trait Implementations

##### `impl Clone for DeferredTokenStream`

- `fn clone(self: &Self) -> DeferredTokenStream` — [`DeferredTokenStream`](#deferredtokenstream)

## Enums

### `TokenStream`

```rust
enum TokenStream {
    Compiler(DeferredTokenStream),
    Fallback(fallback::TokenStream),
}
```

#### Implementations

- `fn new() -> Self`

- `fn from_str_checked(src: &str) -> Result<Self, LexError>` — [`LexError`](#lexerror)

- `fn is_empty(self: &Self) -> bool`

- `fn unwrap_nightly(self: Self) -> proc_macro::TokenStream`

- `fn unwrap_stable(self: Self) -> fallback::TokenStream`

#### Trait Implementations

##### `impl Clone for TokenStream`

- `fn clone(self: &Self) -> TokenStream` — [`TokenStream`](#tokenstream)

##### `impl Debug for TokenStream`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TokenStream`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- `fn extend<I: IntoIterator<Item = TokenStream>>(self: &mut Self, streams: I)`

##### `impl FromIterator for TokenStream`

- `fn from_iter<I: IntoIterator<Item = TokenTree>>(trees: I) -> Self`

##### `impl IntoIterator for TokenStream`

- `type Item = TokenTree`

- `type IntoIter = TokenTreeIter`

- `fn into_iter(self: Self) -> TokenTreeIter` — [`TokenTreeIter`](#tokentreeiter)

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl<T> ToString for TokenStream`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- `fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

- `fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

- `fn borrow_mut(self: &mut Self) -> &mut T`

### `LexError`

```rust
enum LexError {
    Compiler(proc_macro::LexError),
    Fallback(fallback::LexError),
    CompilerPanic,
}
```

#### Implementations

- `fn span(self: &Self) -> Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Debug for LexError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LexError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for LexError`

- `fn to_string(self: &Self) -> String`

### `TokenTreeIter`

```rust
enum TokenTreeIter {
    Compiler(proc_macro::token_stream::IntoIter),
    Fallback(crate::rcvec::RcVecIntoIter<crate::TokenTree>),
}
```

#### Trait Implementations

##### `impl Clone for TokenTreeIter`

- `fn clone(self: &Self) -> TokenTreeIter` — [`TokenTreeIter`](#tokentreeiter)

##### `impl<I> IntoIterator for TokenTreeIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for TokenTreeIter`

- `type Item = TokenTree`

- `fn next(self: &mut Self) -> Option<TokenTree>` — [`TokenTree`](../index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Span`

```rust
enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}
```

#### Implementations

- `fn call_site() -> Self`

- `fn mixed_site() -> Self`

- `fn resolved_at(self: &Self, other: Span) -> Span` — [`Span`](#span)

- `fn located_at(self: &Self, other: Span) -> Span` — [`Span`](#span)

- `fn unwrap(self: Self) -> proc_macro::Span`

- `fn join(self: &Self, other: Span) -> Option<Span>` — [`Span`](#span)

- `fn source_text(self: &Self) -> Option<String>`

- `fn unwrap_nightly(self: Self) -> proc_macro::Span`

#### Trait Implementations

##### `impl Clone for Span`

- `fn clone(self: &Self) -> Span` — [`Span`](#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SpanError for proc_macro2::Span`

### `Group`

```rust
enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
}
```

#### Implementations

- `fn new(delimiter: Delimiter, stream: TokenStream) -> Self` — [`Delimiter`](../index.md), [`TokenStream`](#tokenstream)

- `fn delimiter(self: &Self) -> Delimiter` — [`Delimiter`](../index.md)

- `fn stream(self: &Self) -> TokenStream` — [`TokenStream`](#tokenstream)

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn span_open(self: &Self) -> Span` — [`Span`](#span)

- `fn span_close(self: &Self) -> Span` — [`Span`](#span)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

- `fn unwrap_nightly(self: Self) -> proc_macro::Group`

#### Trait Implementations

##### `impl Clone for Group`

- `fn clone(self: &Self) -> Group` — [`Group`](#group)

##### `impl Debug for Group`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Group`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for proc_macro2::Group`

##### `impl Sealed for proc_macro2::Group`

##### `impl<T> ToString for Group`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::Group`

- `fn leaf_token(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::TokenTree), Reject>` — [`Cursor`](../parse/index.md), [`TokenTree`](../index.md), [`Reject`](../parse/index.md)

##### `impl Token for proc_macro2::Group`

### `Ident`

```rust
enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}
```

#### Implementations

- `fn new_checked(string: &str, span: Span) -> Self` — [`Span`](#span)

- `fn new_raw_checked(string: &str, span: Span) -> Self` — [`Span`](#span)

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

- `fn unwrap_nightly(self: Self) -> proc_macro::Ident`

#### Trait Implementations

##### `impl Clone for Ident`

- `fn clone(self: &Self) -> Ident` — [`Ident`](#ident)

##### `impl Debug for Ident`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IdentExt for proc_macro2::Ident`

##### `impl IdentFragment for proc_macro2::Ident`

- `type Error = Infallible`

##### `impl Parse for proc_macro2::Ident`

##### `impl<T> PartialEq for Ident`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl Sealed for proc_macro2::Ident`

##### `impl<T> ToString for Ident`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::Ident`

##### `impl Token for proc_macro2::Ident`

### `Literal`

```rust
enum Literal {
    Compiler(proc_macro::Literal),
    Fallback(fallback::Literal),
}
```

#### Implementations

- `fn from_str_checked(repr: &str) -> Result<Self, LexError>` — [`LexError`](#lexerror)

- `unsafe fn from_str_unchecked(repr: &str) -> Self`

- `fn u8_suffixed(n: u8) -> Literal` — [`Literal`](#literal)

- `fn u16_suffixed(n: u16) -> Literal` — [`Literal`](#literal)

- `fn u32_suffixed(n: u32) -> Literal` — [`Literal`](#literal)

- `fn u64_suffixed(n: u64) -> Literal` — [`Literal`](#literal)

- `fn u128_suffixed(n: u128) -> Literal` — [`Literal`](#literal)

- `fn usize_suffixed(n: usize) -> Literal` — [`Literal`](#literal)

- `fn i8_suffixed(n: i8) -> Literal` — [`Literal`](#literal)

- `fn i16_suffixed(n: i16) -> Literal` — [`Literal`](#literal)

- `fn i32_suffixed(n: i32) -> Literal` — [`Literal`](#literal)

- `fn i64_suffixed(n: i64) -> Literal` — [`Literal`](#literal)

- `fn i128_suffixed(n: i128) -> Literal` — [`Literal`](#literal)

- `fn isize_suffixed(n: isize) -> Literal` — [`Literal`](#literal)

- `fn f32_suffixed(n: f32) -> Literal` — [`Literal`](#literal)

- `fn f64_suffixed(n: f64) -> Literal` — [`Literal`](#literal)

- `fn u8_unsuffixed(n: u8) -> Literal` — [`Literal`](#literal)

- `fn u16_unsuffixed(n: u16) -> Literal` — [`Literal`](#literal)

- `fn u32_unsuffixed(n: u32) -> Literal` — [`Literal`](#literal)

- `fn u64_unsuffixed(n: u64) -> Literal` — [`Literal`](#literal)

- `fn u128_unsuffixed(n: u128) -> Literal` — [`Literal`](#literal)

- `fn usize_unsuffixed(n: usize) -> Literal` — [`Literal`](#literal)

- `fn i8_unsuffixed(n: i8) -> Literal` — [`Literal`](#literal)

- `fn i16_unsuffixed(n: i16) -> Literal` — [`Literal`](#literal)

- `fn i32_unsuffixed(n: i32) -> Literal` — [`Literal`](#literal)

- `fn i64_unsuffixed(n: i64) -> Literal` — [`Literal`](#literal)

- `fn i128_unsuffixed(n: i128) -> Literal` — [`Literal`](#literal)

- `fn isize_unsuffixed(n: isize) -> Literal` — [`Literal`](#literal)

- `fn f32_unsuffixed(f: f32) -> Literal` — [`Literal`](#literal)

- `fn f64_unsuffixed(f: f64) -> Literal` — [`Literal`](#literal)

- `fn string(string: &str) -> Literal` — [`Literal`](#literal)

- `fn character(ch: char) -> Literal` — [`Literal`](#literal)

- `fn byte_character(byte: u8) -> Literal` — [`Literal`](#literal)

- `fn byte_string(bytes: &[u8]) -> Literal` — [`Literal`](#literal)

- `fn c_string(string: &CStr) -> Literal` — [`Literal`](#literal)

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

- `fn subspan<R: RangeBounds<usize>>(self: &Self, range: R) -> Option<Span>` — [`Span`](#span)

- `fn unwrap_nightly(self: Self) -> proc_macro::Literal`

#### Trait Implementations

##### `impl Clone for Literal`

- `fn clone(self: &Self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Literal`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for proc_macro2::Literal`

##### `impl Sealed for proc_macro2::Literal`

##### `impl<T> ToString for Literal`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::Literal`

- `fn literal_nocapture(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

##### `impl Token for proc_macro2::Literal`

## Functions

### `mismatch`

```rust
fn mismatch(line: u32) -> never
```

### `into_compiler_token`

```rust
fn into_compiler_token(token: crate::TokenTree) -> proc_macro::TokenTree
```

### `debug_span_field_if_nontrivial`

```rust
fn debug_span_field_if_nontrivial(debug: &mut fmt::DebugStruct<'_, '_>, span: Span)
```

## Macros

### `suffixed_numbers!`

### `unsuffixed_integers!`

