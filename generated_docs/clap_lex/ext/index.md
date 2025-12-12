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

*Defined in [`clap_lex-0.7.6/src/ext.rs:247-250`](../../../.source_1765210505/clap_lex-0.7.6/src/ext.rs#L247-L250)*

#### Trait Implementations

##### `impl IntoIterator for Split<'s, 'n>`

- <span id="split-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="split-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Split<'s, '_>`

- <span id="split-iterator-type-item"></span>`type Item = &'s OsStr`

- <span id="split-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Traits

### `OsStrExt`

```rust
trait OsStrExt: private::Sealed { ... }
```

*Defined in [`clap_lex-0.7.6/src/ext.rs:4-183`](../../../.source_1765210505/clap_lex-0.7.6/src/ext.rs#L4-L183)*

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

*Defined in [`clap_lex-0.7.6/src/ext.rs:275-284`](../../../.source_1765210505/clap_lex-0.7.6/src/ext.rs#L275-L284)*

Split an `OsStr`

# Safety

`index` must be at a valid UTF-8 boundary

