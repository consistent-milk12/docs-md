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

*Defined in [`clap_lex-0.7.6/src/ext.rs:247-250`](../../../.source_1765900590/clap_lex-0.7.6/src/ext.rs#L247-L250)*

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

*Defined in [`clap_lex-0.7.6/src/ext.rs:4-183`](../../../.source_1765900590/clap_lex-0.7.6/src/ext.rs#L4-L183)*

String-like methods for [`OsStr`](#osstr)

#### Required Methods

- `fn OsStrExt::try_str(&self) -> Result<&str, std::str::Utf8Error>`

  Converts to a string slice.
  
  The `Utf8Error` is guaranteed to have a valid UTF8 boundary
  in its `valid_up_to()`

- `fn OsStrExt::contains(&self, needle: &str) -> bool`

  Returns `true` if the given pattern matches a sub-slice of
  this string slice.
  
  Returns `false` if it does not.
  
  ##### Examples
  
  ```rust
  use clap_lex::OsStrExt as _;
  let bananas = std::ffi::OsStr::new("bananas");
  
  assert!(bananas.contains("nana"));
  assert!(!bananas.contains("apples"));
  ```

- `fn OsStrExt::find(&self, needle: &str) -> Option<usize>`

  Returns the byte index of the first character of this string slice that
  matches the pattern.
  
  Returns [`None`](#none) if the pattern doesn't match.
  
  ##### Examples
  
  ```rust
  use clap_lex::OsStrExt as _;
  let s = std::ffi::OsStr::new("Löwe 老虎 Léopard Gepardi");
  
  assert_eq!(s.find("L"), Some(0));
  assert_eq!(s.find("é"), Some(14));
  assert_eq!(s.find("par"), Some(17));
  ```
  
  Not finding the pattern:
  
  ```rust
  use clap_lex::OsStrExt as _;
  let s = std::ffi::OsStr::new("Löwe 老虎 Léopard");
  
  assert_eq!(s.find("1"), None);
  ```

- `fn OsStrExt::strip_prefix(&self, prefix: &str) -> Option<&OsStr>`

  Returns a string slice with the prefix removed.
  
  If the string starts with the pattern `prefix`, returns substring after the prefix, wrapped
  in `Some`.
  
  If the string does not start with `prefix`, returns `None`.
  
  ##### Examples
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  assert_eq!(OsStr::new("foo:bar").strip_prefix("foo:"), Some(OsStr::new("bar")));
  assert_eq!(OsStr::new("foo:bar").strip_prefix("bar"), None);
  assert_eq!(OsStr::new("foofoo").strip_prefix("foo"), Some(OsStr::new("foo")));
  ```

- `fn OsStrExt::starts_with(&self, prefix: &str) -> bool`

  Returns `true` if the given pattern matches a prefix of this
  string slice.
  
  Returns `false` if it does not.
  
  ##### Examples
  
  ```rust
  use clap_lex::OsStrExt as _;
  let bananas = std::ffi::OsStr::new("bananas");
  
  assert!(bananas.starts_with("bana"));
  assert!(!bananas.starts_with("nana"));
  ```

- `fn OsStrExt::split<'s, 'n>(self: &'s Self, needle: &'n str) -> Split<'s, 'n>`

  An iterator over substrings of this string slice, separated by
  characters matched by a pattern.
  
  ##### Examples
  
  Simple patterns:
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let v: Vec<_> = OsStr::new("Mary had a little lamb").split(" ").collect();
  assert_eq!(v, [OsStr::new("Mary"), OsStr::new("had"), OsStr::new("a"), OsStr::new("little"), OsStr::new("lamb")]);
  
  let v: Vec<_> = OsStr::new("").split("X").collect();
  assert_eq!(v, [OsStr::new("")]);
  
  let v: Vec<_> = OsStr::new("lionXXtigerXleopard").split("X").collect();
  assert_eq!(v, [OsStr::new("lion"), OsStr::new(""), OsStr::new("tiger"), OsStr::new("leopard")]);
  
  let v: Vec<_> = OsStr::new("lion::tiger::leopard").split("::").collect();
  assert_eq!(v, [OsStr::new("lion"), OsStr::new("tiger"), OsStr::new("leopard")]);
  ```
  
  If a string contains multiple contiguous separators, you will end up
  with empty strings in the output:
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let x = OsStr::new("||||a||b|c");
  let d: Vec<_> = x.split("|").collect();
  
  assert_eq!(d, &[OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new("a"), OsStr::new(""), OsStr::new("b"), OsStr::new("c")]);
  ```
  
  Contiguous separators are separated by the empty string.
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let x = OsStr::new("(///)");
  let d: Vec<_> = x.split("/").collect();
  
  assert_eq!(d, &[OsStr::new("("), OsStr::new(""), OsStr::new(""), OsStr::new(")")]);
  ```
  
  Separators at the start or end of a string are neighbored
  by empty strings.
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let d: Vec<_> = OsStr::new("010").split("0").collect();
  assert_eq!(d, &[OsStr::new(""), OsStr::new("1"), OsStr::new("")]);
  ```
  
  When the empty string is used as a separator, it panics
  
  ```should_panic
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let f: Vec<_> = OsStr::new("rust").split("").collect();
  assert_eq!(f, &[OsStr::new(""), OsStr::new("r"), OsStr::new("u"), OsStr::new("s"), OsStr::new("t"), OsStr::new("")]);
  ```
  
  Contiguous separators can lead to possibly surprising behavior
  when whitespace is used as the separator. This code is correct:
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let x = OsStr::new("    a  b c");
  let d: Vec<_> = x.split(" ").collect();
  
  assert_eq!(d, &[OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new("a"), OsStr::new(""), OsStr::new("b"), OsStr::new("c")]);
  ```
  
  It does _not_ give you:
  
  ```,ignore
  assert_eq!(d, &[OsStr::new("a"), OsStr::new("b"), OsStr::new("c")]);
  ```
  
  Use `split_whitespace` for this behavior.

- `fn OsStrExt::split_once(&self, needle: &str) -> Option<(&OsStr, &OsStr)>`

  Splits the string on the first occurrence of the specified delimiter and
  returns prefix before delimiter and suffix after delimiter.
  
  ##### Examples
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  assert_eq!(OsStr::new("cfg").split_once("="), None);
  assert_eq!(OsStr::new("cfg=").split_once("="), Some((OsStr::new("cfg"), OsStr::new(""))));
  assert_eq!(OsStr::new("cfg=foo").split_once("="), Some((OsStr::new("cfg"), OsStr::new("foo"))));
  assert_eq!(OsStr::new("cfg=foo=bar").split_once("="), Some((OsStr::new("cfg"), OsStr::new("foo=bar"))));
  ```

#### Implementors

- `std::ffi::OsStr`

## Functions

### `split_at`

```rust
unsafe fn split_at(os: &std::ffi::OsStr, index: usize) -> (&std::ffi::OsStr, &std::ffi::OsStr)
```

*Defined in [`clap_lex-0.7.6/src/ext.rs:275-284`](../../../.source_1765900590/clap_lex-0.7.6/src/ext.rs#L275-L284)*

Split an `OsStr`

# Safety

`index` must be at a valid UTF-8 boundary

