*[proc_macro2](../index.md) / [imp](index.md)*

---

# Module `imp`

## Contents

- [Structs](#structs)
  - [`DeferredTokenStream`](#deferredtokenstream)
- [Enums](#enums)
  - [`TokenStream`](#tokenstream)
  - [`LexError`](#lexerror)
  - [`TokenTreeIter`](#tokentreeiter)
  - [`Span`](#span)
  - [`Group`](#group)
  - [`Ident`](#ident)
  - [`Literal`](#literal)
- [Functions](#functions)
  - [`mismatch`](#mismatch)
  - [`into_compiler_token`](#into_compiler_token)
  - [`debug_span_field_if_nontrivial`](#debug_span_field_if_nontrivial)
- [Macros](#macros)
  - [`suffixed_numbers!`](#suffixed_numbers)
  - [`unsuffixed_integers!`](#unsuffixed_integers)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DeferredTokenStream`](#deferredtokenstream) | struct |  |
| [`TokenStream`](#tokenstream) | enum |  |
| [`LexError`](#lexerror) | enum |  |
| [`TokenTreeIter`](#tokentreeiter) | enum |  |
| [`Span`](#span) | enum |  |
| [`Group`](#group) | enum |  |
| [`Ident`](#ident) | enum |  |
| [`Literal`](#literal) | enum |  |
| [`mismatch`](#mismatch) | fn |  |
| [`into_compiler_token`](#into_compiler_token) | fn |  |
| [`debug_span_field_if_nontrivial`](#debug_span_field_if_nontrivial) | fn |  |
| [`suffixed_numbers!`](#suffixed_numbers) | macro |  |
| [`unsuffixed_integers!`](#unsuffixed_integers) | macro |  |

## Structs

### `DeferredTokenStream`

```rust
struct DeferredTokenStream {
    stream: proc_macro::TokenStream,
    extra: Vec<proc_macro::TokenTree>,
}
```

#### Implementations

- <span id="deferredtokenstream-new"></span>`fn new(stream: proc_macro::TokenStream) -> Self`

- <span id="deferredtokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="deferredtokenstream-evaluate-now"></span>`fn evaluate_now(&mut self)`

- <span id="deferredtokenstream-into-token-stream"></span>`fn into_token_stream(self) -> proc_macro::TokenStream`

#### Trait Implementations

##### `impl Clone for DeferredTokenStream`

- <span id="deferredtokenstream-clone"></span>`fn clone(&self) -> DeferredTokenStream` — [`DeferredTokenStream`](#deferredtokenstream)

## Enums

### `TokenStream`

```rust
enum TokenStream {
    Compiler(DeferredTokenStream),
    Fallback(fallback::TokenStream),
}
```

#### Implementations

- <span id="tokenstream-new"></span>`fn new() -> Self`

- <span id="tokenstream-from-str-checked"></span>`fn from_str_checked(src: &str) -> Result<Self, LexError>` — [`LexError`](#lexerror)

- <span id="tokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="tokenstream-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::TokenStream`

- <span id="tokenstream-unwrap-stable"></span>`fn unwrap_stable(self) -> fallback::TokenStream`

#### Trait Implementations

##### `impl Clone for TokenStream`

- <span id="tokenstream-clone"></span>`fn clone(&self) -> TokenStream` — [`TokenStream`](#tokenstream)

##### `impl Debug for TokenStream`

- <span id="tokenstream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TokenStream`

- <span id="tokenstream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, stream: I)`

##### `impl FromIterator for TokenStream`

- <span id="tokenstream-from-iter"></span>`fn from_iter<I: IntoIterator<Item = TokenTree>>(trees: I) -> Self`

##### `impl IntoIterator for TokenStream`

- <span id="tokenstream-item"></span>`type Item = TokenTree`

- <span id="tokenstream-intoiter"></span>`type IntoIter = TokenTreeIter`

- <span id="tokenstream-into-iter"></span>`fn into_iter(self) -> TokenTreeIter` — [`TokenTreeIter`](#tokentreeiter)

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl<T> ToString for TokenStream`

- <span id="tokenstream-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-byte-string"></span>`fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

- <span id="proc-macro2tokenstream-cooked-byte-string"></span>`fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

### `LexError`

```rust
enum LexError {
    Compiler(proc_macro::LexError),
    Fallback(fallback::LexError),
    CompilerPanic,
}
```

#### Implementations

- <span id="lexerror-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Debug for LexError`

- <span id="lexerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LexError`

- <span id="lexerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for LexError`

- <span id="lexerror-to-string"></span>`fn to_string(&self) -> String`

### `TokenTreeIter`

```rust
enum TokenTreeIter {
    Compiler(proc_macro::token_stream::IntoIter),
    Fallback(crate::rcvec::RcVecIntoIter<crate::TokenTree>),
}
```

#### Trait Implementations

##### `impl Clone for TokenTreeIter`

- <span id="tokentreeiter-clone"></span>`fn clone(&self) -> TokenTreeIter` — [`TokenTreeIter`](#tokentreeiter)

##### `impl<I> IntoIterator for TokenTreeIter`

- <span id="tokentreeiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tokentreeiter-intoiter"></span>`type IntoIter = I`

- <span id="tokentreeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TokenTreeIter`

- <span id="tokentreeiter-item"></span>`type Item = TokenTree`

- <span id="tokentreeiter-next"></span>`fn next(&mut self) -> Option<TokenTree>` — [`TokenTree`](../index.md)

- <span id="tokentreeiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Span`

```rust
enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}
```

#### Implementations

- <span id="span-call-site"></span>`fn call_site() -> Self`

- <span id="span-mixed-site"></span>`fn mixed_site() -> Self`

- <span id="span-resolved-at"></span>`fn resolved_at(&self, other: Span) -> Span` — [`Span`](#span)

- <span id="span-located-at"></span>`fn located_at(&self, other: Span) -> Span` — [`Span`](#span)

- <span id="span-unwrap"></span>`fn unwrap(self) -> proc_macro::Span`

- <span id="span-join"></span>`fn join(&self, other: Span) -> Option<Span>` — [`Span`](#span)

- <span id="span-source-text"></span>`fn source_text(&self) -> Option<String>`

- <span id="span-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::Span`

#### Trait Implementations

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SpanError for proc_macro2::Span`

### `Group`

```rust
enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
}
```

#### Implementations

- <span id="group-new"></span>`fn new(delimiter: Delimiter, stream: TokenStream) -> Self` — [`Delimiter`](../index.md), [`TokenStream`](#tokenstream)

- <span id="group-delimiter"></span>`fn delimiter(&self) -> Delimiter` — [`Delimiter`](../index.md)

- <span id="group-stream"></span>`fn stream(&self) -> TokenStream` — [`TokenStream`](#tokenstream)

- <span id="group-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

- <span id="group-span-open"></span>`fn span_open(&self) -> Span` — [`Span`](#span)

- <span id="group-span-close"></span>`fn span_close(&self) -> Span` — [`Span`](#span)

- <span id="group-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

- <span id="group-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::Group`

#### Trait Implementations

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Group` — [`Group`](#group)

##### `impl Debug for Group`

- <span id="group-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Group`

- <span id="group-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for proc_macro2::Group`

##### `impl Sealed for proc_macro2::Group`

##### `impl<T> ToString for Group`

- <span id="group-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Group`

- <span id="proc-macro2group-leaf-token"></span>`fn leaf_token(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::TokenTree), Reject>` — [`Cursor`](../parse/index.md), [`TokenTree`](../index.md), [`Reject`](../parse/index.md)

##### `impl Token for proc_macro2::Group`

### `Ident`

```rust
enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}
```

#### Implementations

- <span id="ident-new-checked"></span>`fn new_checked(string: &str, span: Span) -> Self` — [`Span`](#span)

- <span id="ident-new-raw-checked"></span>`fn new_raw_checked(string: &str, span: Span) -> Self` — [`Span`](#span)

- <span id="ident-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

- <span id="ident-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

- <span id="ident-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::Ident`

#### Trait Implementations

##### `impl Clone for Ident`

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](#ident)

##### `impl Debug for Ident`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IdentExt for proc_macro2::Ident`

##### `impl IdentFragment for proc_macro2::Ident`

- <span id="proc-macro2ident-error"></span>`type Error = Infallible`

##### `impl Parse for proc_macro2::Ident`

##### `impl PartialEq for Ident`

- <span id="ident-eq"></span>`fn eq(&self, other: &Ident) -> bool` — [`Ident`](#ident)

##### `impl Sealed for proc_macro2::Ident`

##### `impl<T> ToString for Ident`

- <span id="ident-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="literal-from-str-checked"></span>`fn from_str_checked(repr: &str) -> Result<Self, LexError>` — [`LexError`](#lexerror)

- <span id="literal-from-str-unchecked"></span>`unsafe fn from_str_unchecked(repr: &str) -> Self`

- <span id="literal-u8-suffixed"></span>`fn u8_suffixed(n: u8) -> Literal` — [`Literal`](#literal)

- <span id="literal-u16-suffixed"></span>`fn u16_suffixed(n: u16) -> Literal` — [`Literal`](#literal)

- <span id="literal-u32-suffixed"></span>`fn u32_suffixed(n: u32) -> Literal` — [`Literal`](#literal)

- <span id="literal-u64-suffixed"></span>`fn u64_suffixed(n: u64) -> Literal` — [`Literal`](#literal)

- <span id="literal-u128-suffixed"></span>`fn u128_suffixed(n: u128) -> Literal` — [`Literal`](#literal)

- <span id="literal-usize-suffixed"></span>`fn usize_suffixed(n: usize) -> Literal` — [`Literal`](#literal)

- <span id="literal-i8-suffixed"></span>`fn i8_suffixed(n: i8) -> Literal` — [`Literal`](#literal)

- <span id="literal-i16-suffixed"></span>`fn i16_suffixed(n: i16) -> Literal` — [`Literal`](#literal)

- <span id="literal-i32-suffixed"></span>`fn i32_suffixed(n: i32) -> Literal` — [`Literal`](#literal)

- <span id="literal-i64-suffixed"></span>`fn i64_suffixed(n: i64) -> Literal` — [`Literal`](#literal)

- <span id="literal-i128-suffixed"></span>`fn i128_suffixed(n: i128) -> Literal` — [`Literal`](#literal)

- <span id="literal-isize-suffixed"></span>`fn isize_suffixed(n: isize) -> Literal` — [`Literal`](#literal)

- <span id="literal-f32-suffixed"></span>`fn f32_suffixed(n: f32) -> Literal` — [`Literal`](#literal)

- <span id="literal-f64-suffixed"></span>`fn f64_suffixed(n: f64) -> Literal` — [`Literal`](#literal)

- <span id="literal-u8-unsuffixed"></span>`fn u8_unsuffixed(n: u8) -> Literal` — [`Literal`](#literal)

- <span id="literal-u16-unsuffixed"></span>`fn u16_unsuffixed(n: u16) -> Literal` — [`Literal`](#literal)

- <span id="literal-u32-unsuffixed"></span>`fn u32_unsuffixed(n: u32) -> Literal` — [`Literal`](#literal)

- <span id="literal-u64-unsuffixed"></span>`fn u64_unsuffixed(n: u64) -> Literal` — [`Literal`](#literal)

- <span id="literal-u128-unsuffixed"></span>`fn u128_unsuffixed(n: u128) -> Literal` — [`Literal`](#literal)

- <span id="literal-usize-unsuffixed"></span>`fn usize_unsuffixed(n: usize) -> Literal` — [`Literal`](#literal)

- <span id="literal-i8-unsuffixed"></span>`fn i8_unsuffixed(n: i8) -> Literal` — [`Literal`](#literal)

- <span id="literal-i16-unsuffixed"></span>`fn i16_unsuffixed(n: i16) -> Literal` — [`Literal`](#literal)

- <span id="literal-i32-unsuffixed"></span>`fn i32_unsuffixed(n: i32) -> Literal` — [`Literal`](#literal)

- <span id="literal-i64-unsuffixed"></span>`fn i64_unsuffixed(n: i64) -> Literal` — [`Literal`](#literal)

- <span id="literal-i128-unsuffixed"></span>`fn i128_unsuffixed(n: i128) -> Literal` — [`Literal`](#literal)

- <span id="literal-isize-unsuffixed"></span>`fn isize_unsuffixed(n: isize) -> Literal` — [`Literal`](#literal)

- <span id="literal-f32-unsuffixed"></span>`fn f32_unsuffixed(f: f32) -> Literal` — [`Literal`](#literal)

- <span id="literal-f64-unsuffixed"></span>`fn f64_unsuffixed(f: f64) -> Literal` — [`Literal`](#literal)

- <span id="literal-string"></span>`fn string(string: &str) -> Literal` — [`Literal`](#literal)

- <span id="literal-character"></span>`fn character(ch: char) -> Literal` — [`Literal`](#literal)

- <span id="literal-byte-character"></span>`fn byte_character(byte: u8) -> Literal` — [`Literal`](#literal)

- <span id="literal-byte-string"></span>`fn byte_string(bytes: &[u8]) -> Literal` — [`Literal`](#literal)

- <span id="literal-c-string"></span>`fn c_string(string: &CStr) -> Literal` — [`Literal`](#literal)

- <span id="literal-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

- <span id="literal-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

- <span id="literal-subspan"></span>`fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span>` — [`Span`](#span)

- <span id="literal-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::Literal`

#### Trait Implementations

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- <span id="literal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Literal`

- <span id="literal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for proc_macro2::Literal`

##### `impl Sealed for proc_macro2::Literal`

##### `impl<T> ToString for Literal`

- <span id="literal-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Literal`

- <span id="proc-macro2literal-literal-nocapture"></span>`fn literal_nocapture(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

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

