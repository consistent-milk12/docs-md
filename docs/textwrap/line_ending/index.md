*[textwrap](../index.md) / [line_ending](index.md)*

---

# Module `line_ending`

Line ending detection and conversion.

## Structs

### `NonEmptyLines<'a>`

```rust
struct NonEmptyLines<'a>(&'a str);
```

An iterator over the lines of a string, as tuples of string slice
and [`LineEnding`](../index.md) value; it only emits non-empty lines (i.e. having
some content before the terminating `\r\n` or `\n`).

This struct is used internally by the library.

#### Trait Implementations

##### `impl<'a> Clone for NonEmptyLines<'a>`

- `fn clone(self: &Self) -> NonEmptyLines<'a>` — [`NonEmptyLines`](#nonemptylines)

##### `impl<'a> Copy for NonEmptyLines<'a>`

##### `impl<'a> Debug for NonEmptyLines<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for NonEmptyLines<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for NonEmptyLines<'a>`

- `type Item = (&'a str, Option<LineEnding>)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Enums

### `LineEnding`

```rust
enum LineEnding {
    CRLF,
    LF,
}
```

Supported line endings. Like in the Rust standard library, two line
endings are supported: `\r\n` and `\n`

#### Variants

- **`CRLF`**

  _Carriage return and line feed_ – a line ending sequence
  historically used in Windows. Corresponds to the sequence
  of ASCII control characters `0x0D 0x0A` or `\r\n`

- **`LF`**

  _Line feed_ – a line ending historically used in Unix.
   Corresponds to the ASCII control character `0x0A` or `\n`

#### Implementations

- `const fn as_str(self: &Self) -> &'static str`

#### Trait Implementations

##### `impl Clone for LineEnding`

- `fn clone(self: &Self) -> LineEnding` — [`LineEnding`](../index.md)

##### `impl Copy for LineEnding`

##### `impl Debug for LineEnding`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LineEnding`

##### `impl PartialEq for LineEnding`

- `fn eq(self: &Self, other: &LineEnding) -> bool` — [`LineEnding`](../index.md)

##### `impl StructuralPartialEq for LineEnding`

