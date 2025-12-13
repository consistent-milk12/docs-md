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

*Defined in [`proc-macro2-1.0.103/src/lib.rs:205-208`](../../../.source_1765633015/proc-macro2-1.0.103/src/lib.rs#L205-L208)*

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

#### Implementations

- <span id="tokenstream-new"></span>`fn _new(inner: imp::TokenStream) -> Self` — [`TokenStream`](../imp/index.md#tokenstream)

- <span id="tokenstream-new-fallback"></span>`fn _new_fallback(inner: fallback::TokenStream) -> Self`

- <span id="tokenstream-new"></span>`fn new() -> Self`

  Returns an empty `TokenStream` containing no token trees.

- <span id="tokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

  Checks if this `TokenStream` is empty.

#### Trait Implementations

##### `impl Any for TokenStream`

- <span id="tokenstream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TokenStream`

- <span id="tokenstream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TokenStream`

- <span id="tokenstream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TokenStream`

- <span id="tokenstream-clone"></span>`fn clone(&self) -> TokenStream` — [`TokenStream`](../index.md#tokenstream)

##### `impl CloneToUninit for TokenStream`

- <span id="tokenstream-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TokenStream`

- <span id="tokenstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TokenStream`

- <span id="tokenstream-default"></span>`fn default() -> Self`

##### `impl Display for TokenStream`

- <span id="tokenstream-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, streams: I)`

##### `impl<T> From for TokenStream`

- <span id="tokenstream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for TokenStream`

- <span id="tokenstream-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = TokenTree>>(streams: I) -> Self`

##### `impl FromStr for TokenStream`

- <span id="tokenstream-fromstr-type-err"></span>`type Err = LexError`

- <span id="tokenstream-fromstr-from-str"></span>`fn from_str(src: &str) -> Result<TokenStream, LexError>` — [`TokenStream`](../index.md#tokenstream), [`LexError`](../index.md#lexerror)

##### `impl<U> Into for TokenStream`

- <span id="tokenstream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TokenStream`

- <span id="tokenstream-intoiterator-type-item"></span>`type Item = TokenTree`

- <span id="tokenstream-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="tokenstream-intoiterator-into-iter"></span>`fn into_iter(self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl ToOwned for TokenStream`

- <span id="tokenstream-toowned-type-owned"></span>`type Owned = T`

- <span id="tokenstream-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tokenstream-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for TokenStream`

- <span id="tokenstream-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-totokens-raw-string"></span>`fn raw_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md#cursor), [`Reject`](../parse/index.md#reject)

- <span id="proc-macro2tokenstream-totokens-byte-string"></span>`fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](../parse/index.md#cursor), [`Reject`](../parse/index.md#reject)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

- <span id="proc-macro2tokenstream-tokenstreamext-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<U> TryFrom for TokenStream`

- <span id="tokenstream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tokenstream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TokenStream`

- <span id="tokenstream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tokenstream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter`

```rust
struct IntoIter {
    inner: imp::TokenTreeIter,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

*Defined in [`proc-macro2-1.0.103/src/lib.rs:1460-1463`](../../../.source_1765633015/proc-macro2-1.0.103/src/lib.rs#L1460-L1463)*

An iterator over `TokenStream`'s `TokenTree`s.

The iteration is "shallow", e.g. the iterator doesn't recurse into
delimited groups, and returns whole groups as token trees.

#### Trait Implementations

##### `impl Any for IntoIter`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for IntoIter`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl CloneToUninit for IntoIter`

- <span id="intoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for IntoIter`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntoIter`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntoIter`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-iterator-type-item"></span>`type Item = TokenTree`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<TokenTree>` — [`TokenTree`](../index.md#tokentree)

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for IntoIter`

- <span id="intoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="intoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IntoIter`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoIter`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

