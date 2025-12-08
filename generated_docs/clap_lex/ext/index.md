*[clap_lex](../index.md) / [ext](index.md)*

---

# Module `ext`

## Modules

- [`private`](private/index.md) - 

## Structs

### `Split<'s, 'n>`

```rust
struct Split<'s, 'n> {
    haystack: Option<&'s std::ffi::OsStr>,
    needle: &'n str,
}
```

#### Trait Implementations

##### `impl<I> IntoIterator for Split<'s, 'n>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'s> Iterator for Split<'s, '_>`

- `type Item = &'s OsStr`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

### `OsStrExt`

```rust
trait OsStrExt: private::Sealed { ... }
```

String-like methods for [`OsStr`](../../clap_builder/builder/index.md)

#### Required Methods

- `fn try_str(self: &Self) -> Result<&str, std::str::Utf8Error>`

  Converts to a string slice.

- `fn contains(self: &Self, needle: &str) -> bool`

  Returns `true` if the given pattern matches a sub-slice of

- `fn find(self: &Self, needle: &str) -> Option<usize>`

  Returns the byte index of the first character of this string slice that

- `fn strip_prefix(self: &Self, prefix: &str) -> Option<&OsStr>`

  Returns a string slice with the prefix removed.

- `fn starts_with(self: &Self, prefix: &str) -> bool`

  Returns `true` if the given pattern matches a prefix of this

- `fn split<'s, 'n>(self: &'s Self, needle: &'n str) -> Split<'s, 'n>`

  An iterator over substrings of this string slice, separated by

- `fn split_once(self: &Self, needle: &str) -> Option<(&OsStr, &OsStr)>`

  Splits the string on the first occurrence of the specified delimiter and

## Functions

### `split_at`

```rust
unsafe fn split_at(os: &std::ffi::OsStr, index: usize) -> (&std::ffi::OsStr, &std::ffi::OsStr)
```

Split an `OsStr`

# Safety

`index` must be at a valid UTF-8 boundary

