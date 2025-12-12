*[textwrap](../index.md) / [line_ending](index.md)*

---

# Module `line_ending`

Line ending detection and conversion.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NonEmptyLines`](#nonemptylines) | struct | An iterator over the lines of a string, as tuples of string slice and [`LineEnding`] value; it only emits non-empty lines (i.e. having some content before the terminating `\r\n` or `\n`). |
| [`LineEnding`](#lineending) | enum | Supported line endings. |

## Structs

### `NonEmptyLines<'a>`

```rust
struct NonEmptyLines<'a>(&'a str);
```

*Defined in [`textwrap-0.16.2/src/line_ending.rs:35`](../../../.source_1765521767/textwrap-0.16.2/src/line_ending.rs#L35)*

An iterator over the lines of a string, as tuples of string slice
and [`LineEnding`](#lineending) value; it only emits non-empty lines (i.e. having
some content before the terminating `\r\n` or `\n`).

This struct is used internally by the library.

#### Trait Implementations

##### `impl Clone for NonEmptyLines<'a>`

- <span id="nonemptylines-clone"></span>`fn clone(&self) -> NonEmptyLines<'a>` — [`NonEmptyLines`](#nonemptylines)

##### `impl Copy for NonEmptyLines<'a>`

##### `impl Debug for NonEmptyLines<'a>`

- <span id="nonemptylines-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NonEmptyLines<'a>`

- <span id="nonemptylines-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nonemptylines-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nonemptylines-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NonEmptyLines<'a>`

- <span id="nonemptylines-iterator-type-item"></span>`type Item = (&'a str, Option<LineEnding>)`

- <span id="nonemptylines-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `LineEnding`

```rust
enum LineEnding {
    CRLF,
    LF,
}
```

*Defined in [`textwrap-0.16.2/src/line_ending.rs:8-16`](../../../.source_1765521767/textwrap-0.16.2/src/line_ending.rs#L8-L16)*

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

- <span id="lineending-as-str"></span>`const fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Clone for LineEnding`

- <span id="lineending-clone"></span>`fn clone(&self) -> LineEnding` — [`LineEnding`](#lineending)

##### `impl Copy for LineEnding`

##### `impl Debug for LineEnding`

- <span id="lineending-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineEnding`

##### `impl PartialEq for LineEnding`

- <span id="lineending-eq"></span>`fn eq(&self, other: &LineEnding) -> bool` — [`LineEnding`](#lineending)

##### `impl StructuralPartialEq for LineEnding`

