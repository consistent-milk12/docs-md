*[regex](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | An error that occurred during parsing or compiling a regular expression. |

## Enums

### `Error`

```rust
enum Error {
    Syntax(alloc::string::String),
    CompiledTooBig(usize),
}
```

*Defined in [`regex-1.12.2/src/error.rs:8-32`](../../../.source_1765521767/regex-1.12.2/src/error.rs#L8-L32)*

An error that occurred during parsing or compiling a regular expression.

#### Variants

- **`Syntax`**

  A syntax error.

- **`CompiledTooBig`**

  The compiled program exceeded the set size
  limit. The argument is the size limit imposed by
  [`RegexBuilder::size_limit`](crate::RegexBuilder::size_limit). Even
  when not configured explicitly, it defaults to a reasonable limit.
  
  If you're getting this error, it occurred because your regex has been
  compiled to an intermediate state that is too big. It is important to
  note that exceeding this limit does _not_ mean the regex is too big to
  _work_, but rather, the regex is big enough that it may wind up being
  surprisingly slow when used in a search. In other words, this error is
  meant to be a practical heuristic for avoiding a performance footgun,
  and especially so for the case where the regex pattern is coming from
  an untrusted source.
  
  There are generally two ways to move forward if you hit this error.
  The first is to find some way to use a smaller regex. The second is to
  increase the size limit via `RegexBuilder::size_limit`. However, if
  your regex pattern is not from a trusted source, then neither of these
  approaches may be appropriate. Instead, you'll have to determine just
  how big of a regex you want to allow.

#### Implementations

- <span id="error-from-meta-build-error"></span>`fn from_meta_build_error(err: meta::BuildError) -> Error` — [`Error`](#error)

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for Error`

- <span id="error-error-description"></span>`fn description(&self) -> &str`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToOwned for Error`

- <span id="error-toowned-type-owned"></span>`type Owned = T`

- <span id="error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

