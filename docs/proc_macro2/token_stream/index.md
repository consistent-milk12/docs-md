*[proc_macro2](../index.md) / [token_stream](index.md)*

---

# Module `token_stream`

Public implementation details for the `TokenStream` type, such as iterators.

## Structs

### `TokenStream`

```rust
struct TokenStream {
    inner: imp::TokenStream,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

#### Implementations

- `fn _new(inner: imp::TokenStream) -> Self` — [`TokenStream`](../../imp/index.md)

- `fn _new_fallback(inner: fallback::TokenStream) -> Self`

- `fn new() -> Self`

- `fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> TokenStream` — [`TokenStream`](../../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Self`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend`

- `fn extend<I: IntoIterator<Item = TokenStream>>(self: &mut Self, streams: I)`

##### `impl FromIterator`

- `fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self`

##### `impl FromStr`

- `type Err = LexError`

- `fn from_str(src: &str) -> Result<TokenStream, LexError>` — [`TokenStream`](../../index.md), [`LexError`](../../index.md)

##### `impl IntoIterator`

- `type Item = TokenTree`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> IntoIter` — [`IntoIter`](../../token_stream/index.md)

##### `impl Parse`

##### `impl Sealed`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens`

- `fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../../parse/index.md), [`Reject`](../../parse/index.md)

- `fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../../parse/index.md), [`Reject`](../../parse/index.md)

##### `impl TokenStreamExt`

- `fn borrow_mut(self: &mut Self) -> &mut T`

### `IntoIter`

```rust
struct IntoIter {
    inner: imp::TokenTreeIter,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

An iterator over `TokenStream`'s `TokenTree`s.

The iteration is "shallow", e.g. the iterator doesn't recurse into
delimited groups, and returns whole groups as token trees.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> IntoIter` — [`IntoIter`](../../token_stream/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator`

- `type Item = TokenTree`

- `fn next(self: &mut Self) -> Option<TokenTree>` — [`TokenTree`](../../index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

