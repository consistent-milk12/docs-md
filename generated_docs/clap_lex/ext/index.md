*[clap_lex](../index.md) / [ext](index.md)*

---

# Module `ext`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Split`](#split) | struct |  |
| [`OsStrExt`](#osstrext) | trait | String-like methods for [`OsStr`] |
| [`split_at`](#split-at) | fn | Split an `OsStr` |

## Modules

- [`private`](private/index.md)

## Structs

### `Split<'s, 'n>`

```rust
struct Split<'s, 'n> {
    haystack: Option<&'s std::ffi::OsStr>,
    needle: &'n str,
}
```

*Defined in [`clap_lex-0.7.6/src/ext.rs:247-250`](../../../.source_1765633015/clap_lex-0.7.6/src/ext.rs#L247-L250)*

#### Trait Implementations

##### `impl Any for Split<'s, 'n>`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split<'s, 'n>`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split<'s, 'n>`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Split<'s, 'n>`

- <span id="split-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Split<'s, 'n>`

- <span id="split-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Split<'s, 'n>`

- <span id="split-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="split-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Split<'s, '_>`

- <span id="split-iterator-type-item"></span>`type Item = &'s OsStr`

- <span id="split-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for Split<'s, 'n>`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Split<'s, 'n>`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `OsStrExt`

```rust
trait OsStrExt: private::Sealed { ... }
```

*Defined in [`clap_lex-0.7.6/src/ext.rs:4-183`](../../../.source_1765633015/clap_lex-0.7.6/src/ext.rs#L4-L183)*

String-like methods for [`OsStr`](../../clap_builder/builder/os_str/index.md)

#### Required Methods

- `fn try_str(&self) -> Result<&str, std::str::Utf8Error>`

  Converts to a string slice.

- `fn contains(&self, needle: &str) -> bool`

  Returns `true` if the given pattern matches a sub-slice of

- `fn find(&self, needle: &str) -> Option<usize>`

  Returns the byte index of the first character of this string slice that

- `fn strip_prefix(&self, prefix: &str) -> Option<&OsStr>`

  Returns a string slice with the prefix removed.

- `fn starts_with(&self, prefix: &str) -> bool`

  Returns `true` if the given pattern matches a prefix of this

- `fn split<'s, 'n>(self: &'s Self, needle: &'n str) -> Split<'s, 'n>`

  An iterator over substrings of this string slice, separated by

- `fn split_once(&self, needle: &str) -> Option<(&OsStr, &OsStr)>`

  Splits the string on the first occurrence of the specified delimiter and

#### Implementors

- `std::ffi::OsStr`

## Functions

### `split_at`

```rust
unsafe fn split_at(os: &std::ffi::OsStr, index: usize) -> (&std::ffi::OsStr, &std::ffi::OsStr)
```

*Defined in [`clap_lex-0.7.6/src/ext.rs:275-284`](../../../.source_1765633015/clap_lex-0.7.6/src/ext.rs#L275-L284)*

Split an `OsStr`

# Safety

`index` must be at a valid UTF-8 boundary

