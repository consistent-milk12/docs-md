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

- <span id="error-from-meta-build-error"></span>`fn from_meta_build_error(err: meta::BuildError) -> Error` — [`Error`](../index.md)

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](../index.md)

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for Error`

- <span id="error-description"></span>`fn description(&self) -> &str`

##### `impl PartialEq for Error`

- <span id="error-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](../index.md)

##### `impl StructuralPartialEq for Error`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

