*[rustc_demangle](../index.md) / [v0](index.md)*

---

# Module `v0`

## Structs

### `Demangle<'a>`

```rust
struct Demangle<'a> {
    inner: &'a str,
}
```

Representation of a demangled symbol name.

#### Trait Implementations

##### `impl<'s> Display for Demangle<'s>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Ident<'s>`

```rust
struct Ident<'s> {
    ascii: &'s str,
    punycode: &'s str,
}
```

#### Fields

- **`ascii`**: `&'s str`

  ASCII part of the identifier.

- **`punycode`**: `&'s str`

  Punycode insertion codes for Unicode codepoints, if any.

#### Implementations

- `fn try_small_punycode_decode<F: FnOnce(&[char]) -> R, R>(self: &Self, f: F) -> Option<R>`

- `fn punycode_decode<F: FnMut(usize, char) -> Result<(), ()>>(self: &Self, insert: F) -> Result<(), ()>`

#### Trait Implementations

##### `impl<'s> Display for Ident<'s>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HexNibbles<'s>`

```rust
struct HexNibbles<'s> {
    nibbles: &'s str,
}
```

Sequence of lowercase hexadecimal nibbles (`0-9a-f`), used by leaf consts.

#### Implementations

- `fn try_parse_uint(self: &Self) -> Option<u64>`

- `fn try_parse_str_chars(self: &Self) -> Option<impl Iterator<Item = char> + 's>`

### `Parser<'s>`

```rust
struct Parser<'s> {
    sym: &'s str,
    next: usize,
    depth: u32,
}
```

#### Implementations

- `fn push_depth(self: &mut Self) -> Result<(), ParseError>` — [`ParseError`](#parseerror)

- `fn pop_depth(self: &mut Self)`

- `fn peek(self: &Self) -> Option<u8>`

- `fn eat(self: &mut Self, b: u8) -> bool`

- `fn next(self: &mut Self) -> Result<u8, ParseError>` — [`ParseError`](#parseerror)

- `fn hex_nibbles(self: &mut Self) -> Result<HexNibbles<'s>, ParseError>` — [`HexNibbles`](#hexnibbles), [`ParseError`](#parseerror)

- `fn digit_10(self: &mut Self) -> Result<u8, ParseError>` — [`ParseError`](#parseerror)

- `fn digit_62(self: &mut Self) -> Result<u8, ParseError>` — [`ParseError`](#parseerror)

- `fn integer_62(self: &mut Self) -> Result<u64, ParseError>` — [`ParseError`](#parseerror)

- `fn opt_integer_62(self: &mut Self, tag: u8) -> Result<u64, ParseError>` — [`ParseError`](#parseerror)

- `fn disambiguator(self: &mut Self) -> Result<u64, ParseError>` — [`ParseError`](#parseerror)

- `fn namespace(self: &mut Self) -> Result<Option<char>, ParseError>` — [`ParseError`](#parseerror)

- `fn backref(self: &mut Self) -> Result<Parser<'s>, ParseError>` — [`Parser`](#parser), [`ParseError`](#parseerror)

- `fn ident(self: &mut Self) -> Result<Ident<'s>, ParseError>` — [`Ident`](#ident), [`ParseError`](#parseerror)

### `Printer<'a, 'b: 'a, 's>`

```rust
struct Printer<'a, 'b: 'a, 's> {
    parser: Result<Parser<'s>, ParseError>,
    out: Option<&'a mut fmt::Formatter<'b>>,
    bound_lifetime_depth: u32,
}
```

#### Fields

- **`parser`**: `Result<Parser<'s>, ParseError>`

  The input parser to demangle from, or `Err` if any (parse) error was
  encountered (in order to disallow further likely-incorrect demangling).
  
  See also the documentation on the `invalid!` and `parse!` macros below.

- **`out`**: `Option<&'a mut fmt::Formatter<'b>>`

  The output formatter to demangle to, or `None` while skipping printing.

- **`bound_lifetime_depth`**: `u32`

  Cumulative number of lifetimes bound by `for<...>` binders ('G'),
  anywhere "around" the current entity (e.g. type) being demangled.
  This value is not tracked while skipping printing, as it'd be unused.
  
  See also the documentation on the `Printer::in_binder` method.

#### Implementations

- `fn eat(self: &mut Self, b: u8) -> bool`

- `fn skipping_printing<F>(self: &mut Self, f: F)`

- `fn print_backref<F>(self: &mut Self, f: F) -> fmt::Result`

- `fn pop_depth(self: &mut Self)`

- `fn print(self: &mut Self, x: impl fmt::Display) -> fmt::Result`

- `fn print_quoted_escaped_chars(self: &mut Self, quote: char, chars: impl Iterator<Item = char>) -> fmt::Result`

- `fn print_lifetime_from_index(self: &mut Self, lt: u64) -> fmt::Result`

- `fn in_binder<F>(self: &mut Self, f: F) -> fmt::Result`

- `fn print_sep_list<F>(self: &mut Self, f: F, sep: &str) -> Result<usize, fmt::Error>`

- `fn print_path(self: &mut Self, in_value: bool) -> fmt::Result`

- `fn print_generic_arg(self: &mut Self) -> fmt::Result`

- `fn print_type(self: &mut Self) -> fmt::Result`

- `fn print_path_maybe_open_generics(self: &mut Self) -> Result<bool, fmt::Error>`

- `fn print_dyn_trait(self: &mut Self) -> fmt::Result`

- `fn print_pat(self: &mut Self) -> fmt::Result`

- `fn print_const(self: &mut Self, in_value: bool) -> fmt::Result`

- `fn print_const_uint(self: &mut Self, ty_tag: u8) -> fmt::Result`

- `fn print_const_str_literal(self: &mut Self) -> fmt::Result`

## Enums

### `ParseError`

```rust
enum ParseError {
    Invalid,
    RecursedTooDeep,
}
```

#### Variants

- **`Invalid`**

  Symbol doesn't match the expected `v0` grammar.

- **`RecursedTooDeep`**

  Parsing the symbol crossed the recursion limit (see `MAX_DEPTH`).

#### Implementations

- `fn message(self: &Self) -> &str`

#### Trait Implementations

##### `impl Debug for ParseError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ParseError`

##### `impl PartialEq for ParseError`

- `fn eq(self: &Self, other: &ParseError) -> bool` — [`ParseError`](#parseerror)

##### `impl StructuralPartialEq for ParseError`

## Functions

### `demangle`

```rust
fn demangle(s: &str) -> Result<(Demangle<'_>, &str), ParseError>
```

De-mangles a Rust symbol into a more readable version

This function will take a **mangled** symbol and return a value. When printed,
the de-mangled version will be written. If the symbol does not look like
a mangled symbol, the original value will be written instead.

### `basic_type`

```rust
fn basic_type(tag: u8) -> Option<&'static str>
```

## Constants

### `MAX_DEPTH`

```rust
const MAX_DEPTH: u32 = 500u32;
```

### `SMALL_PUNYCODE_LEN`

```rust
const SMALL_PUNYCODE_LEN: usize = 128usize;
```

## Macros

### `write!`

### `invalid!`

Mark the parser as errored (with `ParseError::Invalid`), print the
appropriate message (see `ParseError::message`) and return early.

### `parse!`

Call a parser method (if the parser hasn't errored yet),
and mark the parser as errored if it returns `Err`.

If the parser errored, before or now, this returns early,
from the current function, after printing either:
* for a new error, the appropriate message (see `ParseError::message`)
* for an earlier error, only `?` -  this allows callers to keep printing
  the approximate syntax of the path/type/const, despite having errors,
  e.g. `Vec<[(A, ?); ?]>` instead of `Vec<[(A, ?`

