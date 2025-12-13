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
  - [`into_compiler_token`](#into-compiler-token)
  - [`debug_span_field_if_nontrivial`](#debug-span-field-if-nontrivial)
  - [`invalidate_current_thread_spans`](#invalidate-current-thread-spans)
- [Macros](#macros)
  - [`suffixed_numbers!`](#suffixed-numbers)
  - [`unsuffixed_integers!`](#unsuffixed-integers)

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
| [`into_compiler_token`](#into-compiler-token) | fn |  |
| [`debug_span_field_if_nontrivial`](#debug-span-field-if-nontrivial) | fn |  |
| [`invalidate_current_thread_spans`](#invalidate-current-thread-spans) | fn |  |
| [`suffixed_numbers!`](#suffixed-numbers) | macro |  |
| [`unsuffixed_integers!`](#unsuffixed-integers) | macro |  |

## Structs

### `DeferredTokenStream`

```rust
struct DeferredTokenStream {
    stream: proc_macro::TokenStream,
    extra: Vec<proc_macro::TokenTree>,
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:31-34`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L31-L34)*

#### Implementations

- <span id="deferredtokenstream-new"></span>`fn new(stream: proc_macro::TokenStream) -> Self`

- <span id="deferredtokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="deferredtokenstream-evaluate-now"></span>`fn evaluate_now(&mut self)`

- <span id="deferredtokenstream-into-token-stream"></span>`fn into_token_stream(self) -> proc_macro::TokenStream`

#### Trait Implementations

##### `impl Any for DeferredTokenStream`

- <span id="deferredtokenstream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DeferredTokenStream`

- <span id="deferredtokenstream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DeferredTokenStream`

- <span id="deferredtokenstream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DeferredTokenStream`

- <span id="deferredtokenstream-clone"></span>`fn clone(&self) -> DeferredTokenStream` — [`DeferredTokenStream`](#deferredtokenstream)

##### `impl CloneToUninit for DeferredTokenStream`

- <span id="deferredtokenstream-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for DeferredTokenStream`

- <span id="deferredtokenstream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DeferredTokenStream`

- <span id="deferredtokenstream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DeferredTokenStream`

- <span id="deferredtokenstream-toowned-type-owned"></span>`type Owned = T`

- <span id="deferredtokenstream-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="deferredtokenstream-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DeferredTokenStream`

- <span id="deferredtokenstream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deferredtokenstream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DeferredTokenStream`

- <span id="deferredtokenstream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deferredtokenstream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `TokenStream`

```rust
enum TokenStream {
    Compiler(DeferredTokenStream),
    Fallback(fallback::TokenStream),
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:21-24`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L21-L24)*

#### Implementations

- <span id="tokenstream-new"></span>`fn new() -> Self`

- <span id="tokenstream-from-str-checked"></span>`fn from_str_checked(src: &str) -> Result<Self, LexError>` — [`LexError`](#lexerror)

- <span id="tokenstream-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="tokenstream-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::TokenStream`

- <span id="tokenstream-unwrap-stable"></span>`fn unwrap_stable(self) -> fallback::TokenStream`

#### Trait Implementations

##### `impl Any for TokenStream`

- <span id="tokenstream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TokenStream`

- <span id="tokenstream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TokenStream`

- <span id="tokenstream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TokenStream`

- <span id="tokenstream-clone"></span>`fn clone(&self) -> TokenStream` — [`TokenStream`](#tokenstream)

##### `impl CloneToUninit for TokenStream`

- <span id="tokenstream-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TokenStream`

- <span id="tokenstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TokenStream`

- <span id="tokenstream-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- <span id="tokenstream-extend"></span>`fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, stream: I)`

##### `impl<T> From for TokenStream`

- <span id="tokenstream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for TokenStream`

- <span id="tokenstream-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = TokenTree>>(trees: I) -> Self`

##### `impl<U> Into for TokenStream`

- <span id="tokenstream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TokenStream`

- <span id="tokenstream-intoiterator-type-item"></span>`type Item = TokenTree`

- <span id="tokenstream-intoiterator-type-intoiter"></span>`type IntoIter = TokenTreeIter`

- <span id="tokenstream-intoiterator-into-iter"></span>`fn into_iter(self) -> TokenTreeIter` — [`TokenTreeIter`](#tokentreeiter)

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

### `LexError`

```rust
enum LexError {
    Compiler(proc_macro::LexError),
    Fallback(fallback::LexError),
    CompilerPanic,
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:36-43`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L36-L43)*

#### Implementations

- <span id="lexerror-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Any for LexError`

- <span id="lexerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LexError`

- <span id="lexerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LexError`

- <span id="lexerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LexError`

- <span id="lexerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LexError`

- <span id="lexerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LexError`

- <span id="lexerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LexError`

- <span id="lexerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for LexError`

- <span id="lexerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for LexError`

- <span id="lexerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lexerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LexError`

- <span id="lexerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lexerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TokenTreeIter`

```rust
enum TokenTreeIter {
    Compiler(proc_macro::token_stream::IntoIter),
    Fallback(crate::rcvec::RcVecIntoIter<crate::TokenTree>),
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:312-315`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L312-L315)*

#### Trait Implementations

##### `impl Any for TokenTreeIter`

- <span id="tokentreeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TokenTreeIter`

- <span id="tokentreeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TokenTreeIter`

- <span id="tokentreeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TokenTreeIter`

- <span id="tokentreeiter-clone"></span>`fn clone(&self) -> TokenTreeIter` — [`TokenTreeIter`](#tokentreeiter)

##### `impl CloneToUninit for TokenTreeIter`

- <span id="tokentreeiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for TokenTreeIter`

- <span id="tokentreeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TokenTreeIter`

- <span id="tokentreeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TokenTreeIter`

- <span id="tokentreeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tokentreeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tokentreeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TokenTreeIter`

- <span id="tokentreeiter-iterator-type-item"></span>`type Item = TokenTree`

- <span id="tokentreeiter-iterator-next"></span>`fn next(&mut self) -> Option<TokenTree>` — [`TokenTree`](../index.md#tokentree)

- <span id="tokentreeiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for TokenTreeIter`

- <span id="tokentreeiter-toowned-type-owned"></span>`type Owned = T`

- <span id="tokentreeiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tokentreeiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TokenTreeIter`

- <span id="tokentreeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tokentreeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TokenTreeIter`

- <span id="tokentreeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tokentreeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Span`

```rust
enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:370-373`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L370-L373)*

#### Implementations

- <span id="span-call-site"></span>`fn call_site() -> Self`

- <span id="span-mixed-site"></span>`fn mixed_site() -> Self`

- <span id="span-resolved-at"></span>`fn resolved_at(&self, other: Span) -> Span` — [`Span`](#span)

- <span id="span-located-at"></span>`fn located_at(&self, other: Span) -> Span` — [`Span`](#span)

- <span id="span-unwrap"></span>`fn unwrap(self) -> proc_macro::Span`

- <span id="span-byte-range"></span>`fn byte_range(&self) -> Range<usize>`

- <span id="span-start"></span>`fn start(&self) -> LineColumn` — [`LineColumn`](../location/index.md#linecolumn)

- <span id="span-end"></span>`fn end(&self) -> LineColumn` — [`LineColumn`](../location/index.md#linecolumn)

- <span id="span-file"></span>`fn file(&self) -> String`

- <span id="span-local-file"></span>`fn local_file(&self) -> Option<PathBuf>`

- <span id="span-join"></span>`fn join(&self, other: Span) -> Option<Span>` — [`Span`](#span)

- <span id="span-source-text"></span>`fn source_text(&self) -> Option<String>`

- <span id="span-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::Span`

#### Trait Implementations

##### `impl Any for Span`

- <span id="span-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Span`

- <span id="span-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Span`

- <span id="span-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl CloneToUninit for Span`

- <span id="span-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Span`

- <span id="span-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Span`

- <span id="span-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl SpanError for proc_macro2::Span`

##### `impl ToOwned for Span`

- <span id="span-toowned-type-owned"></span>`type Owned = T`

- <span id="span-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="span-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Span`

- <span id="span-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="span-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Span`

- <span id="span-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="span-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Group`

```rust
enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:558-561`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L558-L561)*

#### Implementations

- <span id="group-new"></span>`fn new(delimiter: Delimiter, stream: TokenStream) -> Self` — [`Delimiter`](../index.md#delimiter), [`TokenStream`](#tokenstream)

- <span id="group-delimiter"></span>`fn delimiter(&self) -> Delimiter` — [`Delimiter`](../index.md#delimiter)

- <span id="group-stream"></span>`fn stream(&self) -> TokenStream` — [`TokenStream`](#tokenstream)

- <span id="group-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

- <span id="group-span-open"></span>`fn span_open(&self) -> Span` — [`Span`](#span)

- <span id="group-span-close"></span>`fn span_close(&self) -> Span` — [`Span`](#span)

- <span id="group-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

- <span id="group-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::Group`

#### Trait Implementations

##### `impl Any for Group`

- <span id="group-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Group`

- <span id="group-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Group`

- <span id="group-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Group` — [`Group`](#group)

##### `impl CloneToUninit for Group`

- <span id="group-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Group`

- <span id="group-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Group`

- <span id="group-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Group`

- <span id="group-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Group`

- <span id="group-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for proc_macro2::Group`

##### `impl Sealed for proc_macro2::Group`

##### `impl ToOwned for Group`

- <span id="group-toowned-type-owned"></span>`type Owned = T`

- <span id="group-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="group-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Group`

- <span id="group-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Group`

- <span id="proc-macro2group-totokens-lex-error"></span>`fn lex_error(cursor: Cursor<'_>) -> crate::fallback::LexError` — [`Cursor`](../parse/index.md#cursor)

##### `impl Token for proc_macro2::Group`

##### `impl<U> TryFrom for Group`

- <span id="group-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="group-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Group`

- <span id="group-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="group-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ident`

```rust
enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:663-666`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L663-L666)*

#### Implementations

- <span id="ident-new-checked"></span>`fn new_checked(string: &str, span: Span) -> Self` — [`Span`](#span)

- <span id="ident-new-raw-checked"></span>`fn new_raw_checked(string: &str, span: Span) -> Self` — [`Span`](#span)

- <span id="ident-span"></span>`fn span(&self) -> Span` — [`Span`](#span)

- <span id="ident-set-span"></span>`fn set_span(&mut self, span: Span)` — [`Span`](#span)

- <span id="ident-unwrap-nightly"></span>`fn unwrap_nightly(self) -> proc_macro::Ident`

#### Trait Implementations

##### `impl Any for Ident`

- <span id="ident-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ident`

- <span id="ident-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ident`

- <span id="ident-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ident`

- <span id="ident-clone"></span>`fn clone(&self) -> Ident` — [`Ident`](#ident)

##### `impl CloneToUninit for Ident`

- <span id="ident-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Ident`

- <span id="ident-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- <span id="ident-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Ident`

- <span id="ident-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl IdentExt for proc_macro2::Ident`

##### `impl IdentFragment for proc_macro2::Ident`

- <span id="proc-macro2ident-identfragment-type-error"></span>`type Error = Infallible`

##### `impl<U> Into for Ident`

- <span id="ident-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for proc_macro2::Ident`

##### `impl PartialEq for Ident`

- <span id="ident-partialeq-eq"></span>`fn eq(&self, other: &Ident) -> bool` — [`Ident`](#ident)

##### `impl Sealed for proc_macro2::Ident`

##### `impl ToOwned for Ident`

- <span id="ident-toowned-type-owned"></span>`type Owned = T`

- <span id="ident-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ident-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Ident`

- <span id="ident-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Ident`

- <span id="proc-macro2ident-totokens-ident"></span>`fn ident(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::Ident), Reject>` — [`Cursor`](../parse/index.md#cursor), [`Ident`](../index.md#ident), [`Reject`](../parse/index.md#reject)

##### `impl Token for proc_macro2::Ident`

##### `impl<U> TryFrom for Ident`

- <span id="ident-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ident-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ident`

- <span id="ident-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ident-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Literal`

```rust
enum Literal {
    Compiler(proc_macro::Literal),
    Fallback(fallback::Literal),
}
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:758-761`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L758-L761)*

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

##### `impl Any for Literal`

- <span id="literal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Literal`

- <span id="literal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Literal`

- <span id="literal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl CloneToUninit for Literal`

- <span id="literal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Literal`

- <span id="literal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Literal`

- <span id="literal-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Literal`

- <span id="literal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Literal`

- <span id="literal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for proc_macro2::Literal`

##### `impl Sealed for proc_macro2::Literal`

##### `impl ToOwned for Literal`

- <span id="literal-toowned-type-owned"></span>`type Owned = T`

- <span id="literal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="literal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Literal`

- <span id="literal-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for proc_macro2::Literal`

##### `impl Token for proc_macro2::Literal`

##### `impl<U> TryFrom for Literal`

- <span id="literal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="literal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Literal`

- <span id="literal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="literal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `mismatch`

```rust
fn mismatch(line: u32) -> never
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:46-56`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L46-L56)*

### `into_compiler_token`

```rust
fn into_compiler_token(token: crate::TokenTree) -> proc_macro::TokenTree
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:161-176`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L161-L176)*

### `debug_span_field_if_nontrivial`

```rust
fn debug_span_field_if_nontrivial(debug: &mut fmt::DebugStruct<'_, '_>, span: Span)
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:548-555`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L548-L555)*

### `invalidate_current_thread_spans`

```rust
fn invalidate_current_thread_spans()
```

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:976-984`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L976-L984)*

## Macros

### `suffixed_numbers!`

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:763-773`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L763-L773)*

### `unsuffixed_integers!`

*Defined in [`proc-macro2-1.0.103/src/wrapper.rs:775-785`](../../../.source_1765633015/proc-macro2-1.0.103/src/wrapper.rs#L775-L785)*

