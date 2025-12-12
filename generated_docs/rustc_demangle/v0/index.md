*[rustc_demangle](../index.md) / [v0](index.md)*

---

# Module `v0`

## Contents

- [Structs](#structs)
  - [`Demangle`](#demangle)
  - [`Ident`](#ident)
  - [`HexNibbles`](#hexnibbles)
  - [`Parser`](#parser)
  - [`Printer`](#printer)
- [Enums](#enums)
  - [`ParseError`](#parseerror)
- [Functions](#functions)
  - [`demangle`](#demangle)
  - [`basic_type`](#basic-type)
- [Constants](#constants)
  - [`MAX_DEPTH`](#max-depth)
  - [`SMALL_PUNYCODE_LEN`](#small-punycode-len)
- [Macros](#macros)
  - [`write!`](#write)
  - [`invalid!`](#invalid)
  - [`parse!`](#parse)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Demangle`](#demangle) | struct | Representation of a demangled symbol name. |
| [`Ident`](#ident) | struct |  |
| [`HexNibbles`](#hexnibbles) | struct | Sequence of lowercase hexadecimal nibbles (`0-9a-f`), used by leaf consts. |
| [`Parser`](#parser) | struct |  |
| [`Printer`](#printer) | struct |  |
| [`ParseError`](#parseerror) | enum |  |
| [`demangle`](#demangle) | fn | De-mangles a Rust symbol into a more readable version |
| [`basic_type`](#basic-type) | fn |  |
| [`MAX_DEPTH`](#max-depth) | const |  |
| [`SMALL_PUNYCODE_LEN`](#small-punycode-len) | const |  |
| [`write!`](#write) | macro |  |
| [`invalid!`](#invalid) | macro | Mark the parser as errored (with `ParseError::Invalid`), print the appropriate message (see `ParseError::message`) and return early. |
| [`parse!`](#parse) | macro | Call a parser method (if the parser hasn't errored yet), and mark the parser as errored if it returns `Err`. |

## Structs

### `Demangle<'a>`

```rust
struct Demangle<'a> {
    inner: &'a str,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:19-21`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L19-L21)*

Representation of a demangled symbol name.

#### Trait Implementations

##### `impl Display for Demangle<'s>`

- <span id="demangle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Ident<'s>`

```rust
struct Ident<'s> {
    ascii: &'s str,
    punycode: &'s str,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:108-113`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L108-L113)*

#### Fields

- **`ascii`**: `&'s str`

  ASCII part of the identifier.

- **`punycode`**: `&'s str`

  Punycode insertion codes for Unicode codepoints, if any.

#### Implementations

- <span id="ident-try-small-punycode-decode"></span>`fn try_small_punycode_decode<F: FnOnce(&[char]) -> R, R>(&self, f: F) -> Option<R>`

- <span id="ident-punycode-decode"></span>`fn punycode_decode<F: FnMut(usize, char) -> Result<(), ()>>(&self, insert: F) -> Result<(), ()>`

#### Trait Implementations

##### `impl Display for Ident<'s>`

- <span id="ident-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HexNibbles<'s>`

```rust
struct HexNibbles<'s> {
    nibbles: &'s str,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:269-271`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L269-L271)*

Sequence of lowercase hexadecimal nibbles (`0-9a-f`), used by leaf consts.

#### Implementations

- <span id="hexnibbles-try-parse-uint"></span>`fn try_parse_uint(&self) -> Option<u64>`

- <span id="hexnibbles-try-parse-str-chars"></span>`fn try_parse_str_chars(&self) -> Option<impl Iterator<Item = char> + 's>`

### `Parser<'s>`

```rust
struct Parser<'s> {
    sym: &'s str,
    next: usize,
    depth: u32,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:398-402`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L398-L402)*

#### Implementations

- <span id="parser-push-depth"></span>`fn push_depth(&mut self) -> Result<(), ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-pop-depth"></span>`fn pop_depth(&mut self)`

- <span id="parser-peek"></span>`fn peek(&self) -> Option<u8>`

- <span id="parser-eat"></span>`fn eat(&mut self, b: u8) -> bool`

- <span id="parser-next"></span>`fn next(&mut self) -> Result<u8, ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-hex-nibbles"></span>`fn hex_nibbles(&mut self) -> Result<HexNibbles<'s>, ParseError>` — [`HexNibbles`](#hexnibbles), [`ParseError`](#parseerror)

- <span id="parser-digit-10"></span>`fn digit_10(&mut self) -> Result<u8, ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-digit-62"></span>`fn digit_62(&mut self) -> Result<u8, ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-integer-62"></span>`fn integer_62(&mut self) -> Result<u64, ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-opt-integer-62"></span>`fn opt_integer_62(&mut self, tag: u8) -> Result<u64, ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-disambiguator"></span>`fn disambiguator(&mut self) -> Result<u64, ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-namespace"></span>`fn namespace(&mut self) -> Result<Option<char>, ParseError>` — [`ParseError`](#parseerror)

- <span id="parser-backref"></span>`fn backref(&mut self) -> Result<Parser<'s>, ParseError>` — [`Parser`](#parser), [`ParseError`](#parseerror)

- <span id="parser-ident"></span>`fn ident(&mut self) -> Result<Ident<'s>, ParseError>` — [`Ident`](#ident), [`ParseError`](#parseerror)

### `Printer<'a, 'b: 'a, 's>`

```rust
struct Printer<'a, 'b: 'a, 's> {
    parser: Result<Parser<'s>, ParseError>,
    out: Option<&'a mut fmt::Formatter<'b>>,
    bound_lifetime_depth: u32,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:568-584`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L568-L584)*

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

- <span id="printer-eat"></span>`fn eat(&mut self, b: u8) -> bool`

- <span id="printer-skipping-printing"></span>`fn skipping_printing<F>(&mut self, f: F)`

- <span id="printer-print-backref"></span>`fn print_backref<F>(&mut self, f: F) -> fmt::Result`

- <span id="printer-pop-depth"></span>`fn pop_depth(&mut self)`

- <span id="printer-print"></span>`fn print(&mut self, x: impl fmt::Display) -> fmt::Result`

- <span id="printer-print-quoted-escaped-chars"></span>`fn print_quoted_escaped_chars(&mut self, quote: char, chars: impl Iterator<Item = char>) -> fmt::Result`

- <span id="printer-print-lifetime-from-index"></span>`fn print_lifetime_from_index(&mut self, lt: u64) -> fmt::Result`

- <span id="printer-in-binder"></span>`fn in_binder<F>(&mut self, f: F) -> fmt::Result`

- <span id="printer-print-sep-list"></span>`fn print_sep_list<F>(&mut self, f: F, sep: &str) -> Result<usize, fmt::Error>`

- <span id="printer-print-path"></span>`fn print_path(&mut self, in_value: bool) -> fmt::Result`

- <span id="printer-print-generic-arg"></span>`fn print_generic_arg(&mut self) -> fmt::Result`

- <span id="printer-print-type"></span>`fn print_type(&mut self) -> fmt::Result`

- <span id="printer-print-path-maybe-open-generics"></span>`fn print_path_maybe_open_generics(&mut self) -> Result<bool, fmt::Error>`

- <span id="printer-print-dyn-trait"></span>`fn print_dyn_trait(&mut self) -> fmt::Result`

- <span id="printer-print-pat"></span>`fn print_pat(&mut self) -> fmt::Result`

- <span id="printer-print-const"></span>`fn print_const(&mut self, in_value: bool) -> fmt::Result`

- <span id="printer-print-const-uint"></span>`fn print_const_uint(&mut self, ty_tag: u8) -> fmt::Result`

- <span id="printer-print-const-str-literal"></span>`fn print_const_str_literal(&mut self) -> fmt::Result`

## Enums

### `ParseError`

```rust
enum ParseError {
    Invalid,
    RecursedTooDeep,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:24-30`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L24-L30)*

#### Variants

- **`Invalid`**

  Symbol doesn't match the expected `v0` grammar.

- **`RecursedTooDeep`**

  Parsing the symbol crossed the recursion limit (see `MAX_DEPTH`).

#### Implementations

- <span id="parseerror-message"></span>`fn message(&self) -> &str`

#### Trait Implementations

##### `impl Debug for ParseError`

- <span id="parseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseError`

##### `impl PartialEq for ParseError`

- <span id="parseerror-eq"></span>`fn eq(&self, other: &ParseError) -> bool` — [`ParseError`](#parseerror)

##### `impl StructuralPartialEq for ParseError`

## Functions

### `demangle`

```rust
fn demangle(s: &str) -> Result<(Demangle<'_>, &str), ParseError>
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:37-91`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L37-L91)*

De-mangles a Rust symbol into a more readable version

This function will take a **mangled** symbol and return a value. When printed,
the de-mangled version will be written. If the symbol does not look like
a mangled symbol, the original value will be written instead.

### `basic_type`

```rust
fn basic_type(tag: u8) -> Option<&'static str>
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:370-396`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L370-L396)*

## Constants

### `MAX_DEPTH`
```rust
const MAX_DEPTH: u32 = 500u32;
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:16`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L16)*

### `SMALL_PUNYCODE_LEN`
```rust
const SMALL_PUNYCODE_LEN: usize = 128usize;
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:115`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L115)*

## Macros

### `write!`

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:5-12`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L5-L12)*

### `invalid!`

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:598-605`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L598-L605)*

Mark the parser as errored (with `ParseError::Invalid`), print the
appropriate message (see `ParseError::message`) and return early.

### `parse!`

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:616-630`](../../../.source_1765210505/rustc-demangle-0.1.26/src/v0.rs#L616-L630)*

Call a parser method (if the parser hasn't errored yet),
and mark the parser as errored if it returns `Err`.

If the parser errored, before or now, this returns early,
from the current function, after printing either:
* for a new error, the appropriate message (see `ParseError::message`)
* for an earlier error, only `?` -  this allows callers to keep printing
  the approximate syntax of the path/type/const, despite having errors,
  e.g. `Vec<[(A, ?); ?]>` instead of `Vec<[(A, ?`

