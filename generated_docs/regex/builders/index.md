*[regex](../index.md) / [builders](index.md)*

---

# Module `builders`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`string`](#string) | mod |  |
| [`bytes`](#bytes) | mod |  |
| [`Builder`](#builder) | struct | A builder for constructing a `Regex`, `bytes::Regex`, `RegexSet` or a `bytes::RegexSet`. |

## Modules

- [`string`](string/index.md)
- [`bytes`](bytes/index.md)

## Structs

### `Builder`

```rust
struct Builder {
    pats: alloc::vec::Vec<alloc::string::String>,
    metac: meta::Config,
    syntaxc: syntax::Config,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:44-48`](../../../.source_1765633015/regex-1.12.2/src/builders.rs#L44-L48)*

A builder for constructing a `Regex`, `bytes::Regex`, `RegexSet` or a
`bytes::RegexSet`.

This is essentially the implementation of the four different builder types
in the public API: `RegexBuilder`, `bytes::RegexBuilder`, `RegexSetBuilder`
and `bytes::RegexSetBuilder`.

#### Implementations

- <span id="builder-new"></span>`fn new<I, S>(patterns: I) -> Builder` — [`Builder`](#builder)

- <span id="builder-build-one-string"></span>`fn build_one_string(&self) -> Result<crate::Regex, Error>` — [`Regex`](../index.md#regex), [`Error`](../error/index.md#error)

- <span id="builder-build-one-bytes"></span>`fn build_one_bytes(&self) -> Result<crate::bytes::Regex, Error>` — [`Regex`](../regex/bytes/index.md#regex), [`Error`](../error/index.md#error)

- <span id="builder-build-many-string"></span>`fn build_many_string(&self) -> Result<crate::RegexSet, Error>` — [`RegexSet`](../index.md#regexset), [`Error`](../error/index.md#error)

- <span id="builder-build-many-bytes"></span>`fn build_many_bytes(&self) -> Result<crate::bytes::RegexSet, Error>` — [`RegexSet`](../regexset/bytes/index.md#regexset), [`Error`](../error/index.md#error)

- <span id="builder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-size-limit"></span>`fn size_limit(&mut self, limit: usize) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, limit: usize) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Any for Builder`

- <span id="builder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Builder`

- <span id="builder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Builder`

- <span id="builder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl CloneToUninit for Builder`

- <span id="builder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Builder`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

##### `impl<T> From for Builder`

- <span id="builder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Builder`

- <span id="builder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Builder`

- <span id="builder-toowned-type-owned"></span>`type Owned = T`

- <span id="builder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Builder`

- <span id="builder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Builder`

- <span id="builder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

