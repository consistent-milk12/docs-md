*[proc_macro2](../index.md) / [token_stream](index.md)*

---

# Module `token_stream`

Public implementation details for the `TokenStream` type, such as iterators.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenStream`](#tokenstream) | struct |  |
| [`IntoIter`](#intoiter) | struct | An iterator over `TokenStream`'s `TokenTree`s. |

## Structs

### `TokenStream`

```rust
struct TokenStream {
    inner: imp::TokenStream,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

*Defined in [`proc-macro2-1.0.103/src/lib.rs:205-208`](../../../.source_1765210505/proc-macro2-1.0.103/src/lib.rs#L205-L208)*

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

#### Implementations

- <span id="tokenstream-new"></span>`fn _new(inner: imp::TokenStream) -> Self` — [`TokenStream`](../imp/index.md)

- <span id="tokenstream-new-fallback"></span>`fn _new_fallback(inner: fallback::TokenStream) -> Self`

- <span id="tokenstream-new"></span>`fn new() -> Self`

- <span id="tokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for TokenStream`

- <span id="tokenstream-clone"></span>`fn clone(&self) -> TokenStream` — [`TokenStream`](../index.md)

##### `impl Debug for TokenStream`

- <span id="tokenstream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TokenStream`

- <span id="tokenstream-default"></span>`fn default() -> Self`

##### `impl Display for TokenStream`

- <span id="tokenstream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, streams: I)`

##### `impl FromIterator for TokenStream`

- <span id="tokenstream-from-iter"></span>`fn from_iter<I: IntoIterator<Item = TokenTree>>(streams: I) -> Self`

##### `impl FromStr for TokenStream`

- <span id="tokenstream-type-err"></span>`type Err = LexError`

- <span id="tokenstream-from-str"></span>`fn from_str(src: &str) -> Result<TokenStream, LexError>` — [`TokenStream`](../index.md), [`LexError`](../index.md)

##### `impl IntoIterator for TokenStream`

- <span id="tokenstream-type-item"></span>`type Item = TokenTree`

- <span id="tokenstream-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="tokenstream-into-iter"></span>`fn into_iter(self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl ToString for TokenStream`

- <span id="tokenstream-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-byte-string"></span>`fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

- <span id="proc-macro2tokenstream-cooked-byte-string"></span>`fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md), [`Reject`](../parse/index.md)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

### `IntoIter`

```rust
struct IntoIter {
    inner: imp::TokenTreeIter,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

*Defined in [`proc-macro2-1.0.103/src/lib.rs:1460-1463`](../../../.source_1765210505/proc-macro2-1.0.103/src/lib.rs#L1460-L1463)*

An iterator over `TokenStream`'s `TokenTree`s.

The iteration is "shallow", e.g. the iterator doesn't recurse into
delimited groups, and returns whole groups as token trees.

#### Trait Implementations

##### `impl Clone for IntoIter`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl Debug for IntoIter`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-type-item"></span>`type Item = TokenTree`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<TokenTree>` — [`TokenTree`](../index.md)

- <span id="intoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

