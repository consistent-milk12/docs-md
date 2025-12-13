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

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:19-21`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L19-L21)*

Representation of a demangled symbol name.

#### Trait Implementations

##### `impl Any for Demangle<'a>`

- <span id="demangle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Demangle<'a>`

- <span id="demangle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Demangle<'a>`

- <span id="demangle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Demangle<'s>`

- <span id="demangle-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Demangle<'a>`

- <span id="demangle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Demangle<'a>`

- <span id="demangle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Demangle<'a>`

- <span id="demangle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="demangle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Demangle<'a>`

- <span id="demangle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="demangle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ident<'s>`

```rust
struct Ident<'s> {
    ascii: &'s str,
    punycode: &'s str,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:108-113`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L108-L113)*

#### Fields

- **`ascii`**: `&'s str`

  ASCII part of the identifier.

- **`punycode`**: `&'s str`

  Punycode insertion codes for Unicode codepoints, if any.

#### Implementations

- <span id="ident-try-small-punycode-decode"></span>`fn try_small_punycode_decode<F: FnOnce(&[char]) -> R, R>(&self, f: F) -> Option<R>`

  Attempt to decode punycode on the stack (allocation-free),

  and pass the char slice to the closure, if successful.

  This supports up to `SMALL_PUNYCODE_LEN` characters.

- <span id="ident-punycode-decode"></span>`fn punycode_decode<F: FnMut(usize, char) -> Result<(), ()>>(&self, insert: F) -> Result<(), ()>`

  Decode punycode as insertion positions and characters

  and pass them to the closure, which can return `Err(())`

  to stop the decoding process.

#### Trait Implementations

##### `impl Any for Ident<'s>`

- <span id="ident-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ident<'s>`

- <span id="ident-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ident<'s>`

- <span id="ident-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Ident<'s>`

- <span id="ident-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Ident<'s>`

- <span id="ident-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ident<'s>`

- <span id="ident-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Ident<'s>`

- <span id="ident-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ident-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ident<'s>`

- <span id="ident-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ident-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HexNibbles<'s>`

```rust
struct HexNibbles<'s> {
    nibbles: &'s str,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:269-271`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L269-L271)*

Sequence of lowercase hexadecimal nibbles (`0-9a-f`), used by leaf consts.

#### Implementations

- <span id="hexnibbles-try-parse-uint"></span>`fn try_parse_uint(&self) -> Option<u64>`

  Decode an integer value (with the "most significant nibble" first),

  returning `None` if it can't fit in an `u64`.

- <span id="hexnibbles-try-parse-str-chars"></span>`fn try_parse_str_chars(&self) -> Option<impl Iterator<Item = char> + 's>`

  Decode a UTF-8 byte sequence (with each byte using a pair of nibbles)

  into individual `char`s, returning `None` for invalid UTF-8.

#### Trait Implementations

##### `impl Any for HexNibbles<'s>`

- <span id="hexnibbles-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HexNibbles<'s>`

- <span id="hexnibbles-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HexNibbles<'s>`

- <span id="hexnibbles-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for HexNibbles<'s>`

- <span id="hexnibbles-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HexNibbles<'s>`

- <span id="hexnibbles-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HexNibbles<'s>`

- <span id="hexnibbles-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hexnibbles-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HexNibbles<'s>`

- <span id="hexnibbles-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hexnibbles-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Parser<'s>`

```rust
struct Parser<'s> {
    sym: &'s str,
    next: usize,
    depth: u32,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:398-402`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L398-L402)*

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

#### Trait Implementations

##### `impl Any for Parser<'s>`

- <span id="parser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Parser<'s>`

- <span id="parser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Parser<'s>`

- <span id="parser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Parser<'s>`

- <span id="parser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parser<'s>`

- <span id="parser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Parser<'s>`

- <span id="parser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parser<'s>`

- <span id="parser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Printer<'a, 'b: 'a, 's>`

```rust
struct Printer<'a, 'b: 'a, 's> {
    parser: Result<Parser<'s>, ParseError>,
    out: Option<&'a mut fmt::Formatter<'b>>,
    bound_lifetime_depth: u32,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:568-584`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L568-L584)*

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

  Eat the given character from the parser,

  returning `false` if the parser errored.

- <span id="printer-skipping-printing"></span>`fn skipping_printing<F>(&mut self, f: F)`

  Skip printing (i.e. `self.out` will be `None`) for the duration of the

  given closure. This should not change parsing behavior, only disable the

  output, but there may be optimizations (such as not traversing backrefs).

- <span id="printer-print-backref"></span>`fn print_backref<F>(&mut self, f: F) -> fmt::Result`

  Print the target of a backref, using the given closure.

  When printing is being skipped, the backref will only be parsed,

  ignoring the backref's target completely.

- <span id="printer-pop-depth"></span>`fn pop_depth(&mut self)`

- <span id="printer-print"></span>`fn print(&mut self, x: impl fmt::Display) -> fmt::Result`

  Output the given value to `self.out` (using `fmt::Display` formatting),

  if printing isn't being skipped.

- <span id="printer-print-quoted-escaped-chars"></span>`fn print_quoted_escaped_chars(&mut self, quote: char, chars: impl Iterator<Item = char>) -> fmt::Result`

  Output the given `char`s (escaped using `char::escape_debug`), with the

  whole sequence wrapped in quotes, for either a `char` or `&str` literal,

  if printing isn't being skipped.

- <span id="printer-print-lifetime-from-index"></span>`fn print_lifetime_from_index(&mut self, lt: u64) -> fmt::Result`

  Print the lifetime according to the previously decoded index.

  An index of `0` always refers to `'_`, but starting with `1`,

  indices refer to late-bound lifetimes introduced by a binder.

- <span id="printer-in-binder"></span>`fn in_binder<F>(&mut self, f: F) -> fmt::Result`

  Optionally enter a binder ('G') for late-bound lifetimes,

  printing e.g. `for<'a, 'b> ` before calling the closure,

  and make those lifetimes visible to it (via depth level).

- <span id="printer-print-sep-list"></span>`fn print_sep_list<F>(&mut self, f: F, sep: &str) -> Result<usize, fmt::Error>`

  Print list elements using the given closure and separator,

  until the end of the list ('E') is found, or the parser errors.

  Returns the number of elements printed.

- <span id="printer-print-path"></span>`fn print_path(&mut self, in_value: bool) -> fmt::Result`

- <span id="printer-print-generic-arg"></span>`fn print_generic_arg(&mut self) -> fmt::Result`

- <span id="printer-print-type"></span>`fn print_type(&mut self) -> fmt::Result`

- <span id="printer-print-path-maybe-open-generics"></span>`fn print_path_maybe_open_generics(&mut self) -> Result<bool, fmt::Error>`

  A trait in a trait object may have some "existential projections"

  (i.e. associated type bindings) after it, which should be printed

  in the `<...>` of the trait, e.g. `dyn Trait<T, U, Assoc=X>`.

  To this end, this method will keep the `<...>` of an 'I' path

  open, by omitting the `>`, and return `Ok(true)` in that case.

- <span id="printer-print-dyn-trait"></span>`fn print_dyn_trait(&mut self) -> fmt::Result`

- <span id="printer-print-pat"></span>`fn print_pat(&mut self) -> fmt::Result`

- <span id="printer-print-const"></span>`fn print_const(&mut self, in_value: bool) -> fmt::Result`

- <span id="printer-print-const-uint"></span>`fn print_const_uint(&mut self, ty_tag: u8) -> fmt::Result`

- <span id="printer-print-const-str-literal"></span>`fn print_const_str_literal(&mut self) -> fmt::Result`

#### Trait Implementations

##### `impl Any for Printer<'a, 'b, 's>`

- <span id="printer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Printer<'a, 'b, 's>`

- <span id="printer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Printer<'a, 'b, 's>`

- <span id="printer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Printer<'a, 'b, 's>`

- <span id="printer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Printer<'a, 'b, 's>`

- <span id="printer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Printer<'a, 'b, 's>`

- <span id="printer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="printer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Printer<'a, 'b, 's>`

- <span id="printer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="printer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ParseError`

```rust
enum ParseError {
    Invalid,
    RecursedTooDeep,
}
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:24-30`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L24-L30)*

#### Variants

- **`Invalid`**

  Symbol doesn't match the expected `v0` grammar.

- **`RecursedTooDeep`**

  Parsing the symbol crossed the recursion limit (see `MAX_DEPTH`).

#### Implementations

- <span id="parseerror-message"></span>`fn message(&self) -> &str`

  Snippet to print when the error is initially encountered.

#### Trait Implementations

##### `impl Any for ParseError`

- <span id="parseerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseError`

- <span id="parseerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseError`

- <span id="parseerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseError`

- <span id="parseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseError`

##### `impl<T> From for ParseError`

- <span id="parseerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseError`

- <span id="parseerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ParseError`

- <span id="parseerror-partialeq-eq"></span>`fn eq(&self, other: &ParseError) -> bool` — [`ParseError`](#parseerror)

##### `impl StructuralPartialEq for ParseError`

##### `impl<U> TryFrom for ParseError`

- <span id="parseerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseError`

- <span id="parseerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `demangle`

```rust
fn demangle(s: &str) -> Result<(Demangle<'_>, &str), ParseError>
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:37-91`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L37-L91)*

De-mangles a Rust symbol into a more readable version

This function will take a **mangled** symbol and return a value. When printed,
the de-mangled version will be written. If the symbol does not look like
a mangled symbol, the original value will be written instead.

### `basic_type`

```rust
fn basic_type(tag: u8) -> Option<&'static str>
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:370-396`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L370-L396)*

## Constants

### `MAX_DEPTH`
```rust
const MAX_DEPTH: u32 = 500u32;
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:16`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L16)*

### `SMALL_PUNYCODE_LEN`
```rust
const SMALL_PUNYCODE_LEN: usize = 128usize;
```

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:115`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L115)*

## Macros

### `write!`

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:5-12`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L5-L12)*

### `invalid!`

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:598-605`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L598-L605)*

Mark the parser as errored (with `ParseError::Invalid`), print the
appropriate message (see `ParseError::message`) and return early.

### `parse!`

*Defined in [`rustc-demangle-0.1.26/src/v0.rs:616-630`](../../../.source_1765521767/rustc-demangle-0.1.26/src/v0.rs#L616-L630)*

Call a parser method (if the parser hasn't errored yet),
and mark the parser as errored if it returns `Err`.

If the parser errored, before or now, this returns early,
from the current function, after printing either:
* for a new error, the appropriate message (see `ParseError::message`)
* for an earlier error, only `?` -  this allows callers to keep printing
  the approximate syntax of the path/type/const, despite having errors,
  e.g. `Vec<[(A, ?); ?]>` instead of `Vec<[(A, ?`

