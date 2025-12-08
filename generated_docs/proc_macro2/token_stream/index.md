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

- `fn _new(inner: imp::TokenStream) -> Self` — [`TokenStream`](../imp/index.md)

- `fn _new_fallback(inner: fallback::TokenStream) -> Self`

- `fn new() -> Self`

- `fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for TokenStream`

- `fn clone(self: &Self) -> TokenStream` — [`TokenStream`](../index.md)

##### `impl Debug for TokenStream`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TokenStream`

- `fn default() -> Self`

##### `impl Display for TokenStream`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- `fn extend<I: IntoIterator<Item = TokenTree>>(self: &mut Self, streams: I)`

##### `impl FromIterator for TokenStream`

- `fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self`

##### `impl FromStr for TokenStream`

- `type Err = LexError`

- `fn from_str(src: &str) -> Result<TokenStream, LexError>` — [`TokenStream`](../index.md), [`LexError`](../index.md)

##### `impl IntoIterator for TokenStream`

- `type Item = TokenTree`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl<T> ToString for TokenStream`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- `fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

- `fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

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

##### `impl Clone for IntoIter`

- `fn clone(self: &Self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl Debug for IntoIter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for IntoIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for IntoIter`

- `type Item = TokenTree`

- `fn next(self: &mut Self) -> Option<TokenTree>` — [`TokenTree`](../index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

