# Crate `proc_macro2`

[![github]](https://github.com/dtolnay/proc-macro2)&ensp;[![crates-io]](https://crates.io/crates/proc-macro2)&ensp;[![docs-rs]](crate)



<br>

A wrapper around the procedural macro API of the compiler's `proc_macro`
crate. This library serves two purposes:

- **Bring proc-macro-like functionality to other contexts like build.rs and
  main.rs.** Types from `proc_macro` are entirely specific to procedural
  macros and cannot ever exist in code outside of a procedural macro.
  Meanwhile `proc_macro2` types may exist anywhere including non-macro code.
  By developing foundational libraries like [`syn`](../syn/index.md) and [`quote`](../quote/index.md) against
  `proc_macro2` rather than `proc_macro`, the procedural macro ecosystem
  becomes easily applicable to many other use cases and we avoid
  reimplementing non-macro equivalents of those libraries.

- **Make procedural macros unit testable.** As a consequence of being
  specific to procedural macros, nothing that uses `proc_macro` can be
  executed from a unit test. In order for helper libraries or components of
  a macro to be testable in isolation, they must be implemented using
  `proc_macro2`.


# Usage

The skeleton of a typical procedural macro typically looks like this:

```rust
extern crate proc_macro;

const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
};
#[cfg(wrap_proc_macro)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        /* transform input */
        input
    };

    proc_macro::TokenStream::from(output)
}
```

If parsing with [Syn], you'll use `parse_macro_input!` instead to
propagate parse errors correctly back to the compiler when parsing fails.

# Unstable features

The default feature set of proc-macro2 tracks the most recent stable
compiler API. Functionality in `proc_macro` that is not yet stable is not
exposed by proc-macro2 by default.

To opt into the additional APIs available in the most recent nightly
compiler, the `procmacro2_semver_exempt` config flag must be passed to
rustc. We will polyfill those nightly-only APIs back to Rust 1.60.0. As
these are unstable APIs that track the nightly compiler, minor versions of
proc-macro2 may make breaking changes to them at any time.

```sh
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build
```

Note that this must not only be done for your crate, but for any crate that
depends on your crate. This infectious nature is intentional, as it serves
as a reminder that you are outside of the normal semver guarantees.

Semver exempt methods are marked as such in the proc-macro2 documentation.

# Thread-Safety

Most types in this crate are `!Sync` because the underlying compiler
types make use of thread-local memory, meaning they cannot be accessed from
a different thread.

## Modules

- [`marker`](marker/index.md) - 
- [`parse`](parse/index.md) - 
- [`probe`](probe/index.md) - 
- [`rcvec`](rcvec/index.md) - 
- [`detection`](detection/index.md) - 
- [`extra`](extra/index.md) - Items which do not have a correspondence to any API in the proc_macro crate,
- [`imp`](imp/index.md) - 
- [`token_stream`](token_stream/index.md) - Public implementation details for the `TokenStream` type, such as iterators.

## Structs

### `TokenStream`

```rust
struct TokenStream {
    inner: imp::TokenStream,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro]`,
`#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.

#### Implementations

- `fn _new(inner: imp::TokenStream) -> Self` — [`TokenStream`](imp/index.md)

- `fn _new_fallback(inner: fallback::TokenStream) -> Self`

- `fn new() -> Self`

- `fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for TokenStream`

- `fn clone(self: &Self) -> TokenStream` — [`TokenStream`](#tokenstream)

##### `impl Debug for TokenStream`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TokenStream`

- `fn default() -> Self`

##### `impl Display for TokenStream`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend for TokenStream`

- `fn extend<I: IntoIterator<Item = TokenTree>>(self: &mut Self, streams: I)`

##### `impl FromIterator for TokenStream`

- `fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self`

##### `impl FromStr for TokenStream`

- `type Err = LexError`

- `fn from_str(src: &str) -> Result<TokenStream, LexError>` — [`TokenStream`](#tokenstream), [`LexError`](#lexerror)

##### `impl IntoIterator for TokenStream`

- `type Item = TokenTree`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> IntoIter` — [`IntoIter`](token_stream/index.md)

##### `impl Parse for proc_macro2::TokenStream`

##### `impl Sealed for proc_macro2::TokenStream`

##### `impl<T> ToString for TokenStream`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::TokenStream`

- `fn byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](parse/index.md), [`Reject`](parse/index.md)

- `fn cooked_byte_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](parse/index.md), [`Reject`](parse/index.md)

##### `impl TokenStreamExt for proc_macro2::TokenStream`

- `fn borrow_mut(self: &mut Self) -> &mut T`

### `LexError`

```rust
struct LexError {
    inner: imp::LexError,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

Error returned from `TokenStream::from_str`.

#### Implementations

- `fn span(self: &Self) -> Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Debug for LexError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LexError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for LexError`

##### `impl<T> ToString for LexError`

- `fn to_string(self: &Self) -> String`

### `Span`

```rust
struct Span {
    inner: imp::Span,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

A region of source code, along with macro expansion information.

#### Implementations

- `fn _new(inner: imp::Span) -> Self` — [`Span`](imp/index.md)

- `fn _new_fallback(inner: fallback::Span) -> Self`

- `fn call_site() -> Self`

- `fn mixed_site() -> Self`

- `fn resolved_at(self: &Self, other: Span) -> Span` — [`Span`](#span)

- `fn located_at(self: &Self, other: Span) -> Span` — [`Span`](#span)

- `fn unwrap(self: Self) -> proc_macro::Span`

- `fn join(self: &Self, other: Span) -> Option<Span>` — [`Span`](#span)

- `fn source_text(self: &Self) -> Option<String>`

#### Trait Implementations

##### `impl Clone for Span`

- `fn clone(self: &Self) -> Span` — [`Span`](#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SpanError for proc_macro2::Span`

### `Group`

```rust
struct Group {
    inner: imp::Group,
}
```

A delimited token stream.

A `Group` internally contains a `TokenStream` which is surrounded by
`Delimiter`s.

#### Implementations

- `fn _new(inner: imp::Group) -> Self` — [`Group`](imp/index.md)

- `fn _new_fallback(inner: fallback::Group) -> Self`

- `fn new(delimiter: Delimiter, stream: TokenStream) -> Self` — [`Delimiter`](#delimiter), [`TokenStream`](#tokenstream)

- `fn delimiter(self: &Self) -> Delimiter` — [`Delimiter`](#delimiter)

- `fn stream(self: &Self) -> TokenStream` — [`TokenStream`](#tokenstream)

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn span_open(self: &Self) -> Span` — [`Span`](#span)

- `fn span_close(self: &Self) -> Span` — [`Span`](#span)

- `fn delim_span(self: &Self) -> DelimSpan` — [`DelimSpan`](extra/index.md)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for Group`

- `fn clone(self: &Self) -> Group` — [`Group`](#group)

##### `impl Debug for Group`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Group`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for proc_macro2::Group`

##### `impl Sealed for proc_macro2::Group`

##### `impl<T> ToString for Group`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::Group`

- `fn leaf_token(input: Cursor<'_>) -> Result<(Cursor<'_>, crate::TokenTree), Reject>` — [`Cursor`](parse/index.md), [`TokenTree`](#tokentree), [`Reject`](parse/index.md)

##### `impl Token for proc_macro2::Group`

### `Punct`

```rust
struct Punct {
    ch: char,
    spacing: Spacing,
    span: Span,
}
```

A `Punct` is a single punctuation character like `+`, `-` or `#`.

Multicharacter operators like `+=` are represented as two instances of
`Punct` with different forms of `Spacing` returned.

#### Implementations

- `fn new(ch: char, spacing: Spacing) -> Self` — [`Spacing`](#spacing)

- `fn as_char(self: &Self) -> char`

- `fn spacing(self: &Self) -> Spacing` — [`Spacing`](#spacing)

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for Punct`

- `fn clone(self: &Self) -> Punct` — [`Punct`](#punct)

##### `impl Debug for Punct`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Punct`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for proc_macro2::Punct`

##### `impl PunctExt for proc_macro2::Punct`

##### `impl Sealed for proc_macro2::Punct`

##### `impl<T> ToString for Punct`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::Punct`

- `fn ident_not_raw(input: Cursor<'_>) -> Result<(Cursor<'_>, &str), Reject>` — [`Cursor`](parse/index.md), [`Reject`](parse/index.md)

##### `impl Token for proc_macro2::Punct`

### `Ident`

```rust
struct Ident {
    inner: imp::Ident,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

A word of Rust code, which may be a keyword or legal variable name.

An identifier consists of at least one Unicode code point, the first of
which has the XID_Start property and the rest of which have the XID_Continue
property.

- The empty string is not an identifier. Use `Option<Ident>`.
- A lifetime is not an identifier. Use `syn::Lifetime` instead.

An identifier constructed with `Ident::new` is permitted to be a Rust
keyword, though parsing one through its [`Parse`](#parse) implementation rejects
Rust keywords. Use `input.call(Ident::parse_any)` when parsing to match the
behaviour of `Ident::new`.

# Examples

A new ident can be created from a string using the `Ident::new` function.
A span must be provided explicitly which governs the name resolution
behavior of the resulting identifier.

```rust
use proc_macro2::{Ident, Span};

fn main() {
    let call_ident = Ident::new("calligraphy", Span::call_site());

    println!("{}", call_ident);
}
```

An ident can be interpolated into a token stream using the `quote!` macro.

```rust
use proc_macro2::{Ident, Span};
use quote::quote;

fn main() {
    let ident = Ident::new("demo", Span::call_site());

    // Create a variable binding whose name is this ident.
    let expanded = quote! { let #ident = 10; };

    // Create a variable binding with a slightly different name.
    let temp_ident = Ident::new(&format!("new_{}", ident), Span::call_site());
    let expanded = quote! { let #temp_ident = 10; };
}
```

A string representation of the ident is available through the `to_string()`
method.

```rust
use proc_macro2::{Ident, Span};

let ident = Ident::new("another_identifier", Span::call_site());

// Examine the ident as a string.
let ident_string = ident.to_string();
if ident_string.len() > 60 {
    println!("Very long identifier: {}", ident_string)
}
```

#### Implementations

- `fn _new(inner: imp::Ident) -> Self` — [`Ident`](imp/index.md)

- `fn _new_fallback(inner: fallback::Ident) -> Self`

- `fn new(string: &str, span: Span) -> Self` — [`Span`](#span)

- `fn new_raw(string: &str, span: Span) -> Self` — [`Span`](#span)

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for Ident`

- `fn clone(self: &Self) -> Ident` — [`Ident`](#ident)

##### `impl Debug for Ident`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ident`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ident`

##### `impl Hash for Ident`

- `fn hash<H: Hasher>(self: &Self, hasher: &mut H)`

##### `impl IdentExt for proc_macro2::Ident`

##### `impl IdentFragment for proc_macro2::Ident`

- `type Error = Infallible`

##### `impl Ord for Ident`

- `fn cmp(self: &Self, other: &Ident) -> Ordering` — [`Ident`](#ident)

##### `impl Parse for proc_macro2::Ident`

##### `impl PartialEq for Ident`

- `fn eq(self: &Self, other: &Ident) -> bool` — [`Ident`](#ident)

##### `impl PartialOrd for Ident`

- `fn partial_cmp(self: &Self, other: &Ident) -> Option<Ordering>` — [`Ident`](#ident)

##### `impl Sealed for proc_macro2::Ident`

##### `impl<T> ToString for Ident`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::Ident`

##### `impl Token for proc_macro2::Ident`

### `Literal`

```rust
struct Literal {
    inner: imp::Literal,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
byte character (`b'a'`), an integer or floating point number with or without
a suffix (`1`, `1u8`, `2.3`, `2.3f32`).

Boolean literals like `true` and `false` do not belong here, they are
`Ident`s.

#### Implementations

- `fn _new(inner: imp::Literal) -> Self` — [`Literal`](imp/index.md)

- `fn _new_fallback(inner: fallback::Literal) -> Self`

- `fn u8_suffixed(n: u8) -> Literal` — [`Literal`](#literal)

- `fn u16_suffixed(n: u16) -> Literal` — [`Literal`](#literal)

- `fn u32_suffixed(n: u32) -> Literal` — [`Literal`](#literal)

- `fn u64_suffixed(n: u64) -> Literal` — [`Literal`](#literal)

- `fn u128_suffixed(n: u128) -> Literal` — [`Literal`](#literal)

- `fn usize_suffixed(n: usize) -> Literal` — [`Literal`](#literal)

- `fn i8_suffixed(n: i8) -> Literal` — [`Literal`](#literal)

- `fn i16_suffixed(n: i16) -> Literal` — [`Literal`](#literal)

- `fn i32_suffixed(n: i32) -> Literal` — [`Literal`](#literal)

- `fn i64_suffixed(n: i64) -> Literal` — [`Literal`](#literal)

- `fn i128_suffixed(n: i128) -> Literal` — [`Literal`](#literal)

- `fn isize_suffixed(n: isize) -> Literal` — [`Literal`](#literal)

- `fn u8_unsuffixed(n: u8) -> Literal` — [`Literal`](#literal)

- `fn u16_unsuffixed(n: u16) -> Literal` — [`Literal`](#literal)

- `fn u32_unsuffixed(n: u32) -> Literal` — [`Literal`](#literal)

- `fn u64_unsuffixed(n: u64) -> Literal` — [`Literal`](#literal)

- `fn u128_unsuffixed(n: u128) -> Literal` — [`Literal`](#literal)

- `fn usize_unsuffixed(n: usize) -> Literal` — [`Literal`](#literal)

- `fn i8_unsuffixed(n: i8) -> Literal` — [`Literal`](#literal)

- `fn i16_unsuffixed(n: i16) -> Literal` — [`Literal`](#literal)

- `fn i32_unsuffixed(n: i32) -> Literal` — [`Literal`](#literal)

- `fn i64_unsuffixed(n: i64) -> Literal` — [`Literal`](#literal)

- `fn i128_unsuffixed(n: i128) -> Literal` — [`Literal`](#literal)

- `fn isize_unsuffixed(n: isize) -> Literal` — [`Literal`](#literal)

- `fn f64_unsuffixed(f: f64) -> Literal` — [`Literal`](#literal)

- `fn f64_suffixed(f: f64) -> Literal` — [`Literal`](#literal)

- `fn f32_unsuffixed(f: f32) -> Literal` — [`Literal`](#literal)

- `fn f32_suffixed(f: f32) -> Literal` — [`Literal`](#literal)

- `fn string(string: &str) -> Literal` — [`Literal`](#literal)

- `fn character(ch: char) -> Literal` — [`Literal`](#literal)

- `fn byte_character(byte: u8) -> Literal` — [`Literal`](#literal)

- `fn byte_string(bytes: &[u8]) -> Literal` — [`Literal`](#literal)

- `fn c_string(string: &CStr) -> Literal` — [`Literal`](#literal)

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

- `fn subspan<R: RangeBounds<usize>>(self: &Self, range: R) -> Option<Span>` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for Literal`

- `fn clone(self: &Self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Literal`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromStr for Literal`

- `type Err = LexError`

- `fn from_str(repr: &str) -> Result<Self, LexError>` — [`LexError`](#lexerror)

##### `impl Parse for proc_macro2::Literal`

##### `impl Sealed for proc_macro2::Literal`

##### `impl<T> ToString for Literal`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::Literal`

- `fn literal_nocapture(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](parse/index.md), [`Reject`](parse/index.md)

##### `impl Token for proc_macro2::Literal`

## Enums

### `TokenTree`

```rust
enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}
```

A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).

#### Variants

- **`Group`**

  A token stream surrounded by bracket delimiters.

- **`Ident`**

  An identifier.

- **`Punct`**

  A single punctuation character (`+`, `,`, `$`, etc.).

- **`Literal`**

  A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.

#### Implementations

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn set_span(self: &mut Self, span: Span)` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for TokenTree`

- `fn clone(self: &Self) -> TokenTree` — [`TokenTree`](#tokentree)

##### `impl Debug for TokenTree`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TokenTree`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse for proc_macro2::TokenTree`

##### `impl Sealed for proc_macro2::TokenTree`

##### `impl<T> ToString for TokenTree`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for proc_macro2::TokenTree`

- `fn cooked_string(input: Cursor<'_>) -> Result<Cursor<'_>, Reject>` — [`Cursor`](parse/index.md), [`Reject`](parse/index.md)

##### `impl Token for proc_macro2::TokenTree`

### `Delimiter`

```rust
enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}
```

Describes how a sequence of token trees is delimited.

#### Variants

- **`Parenthesis`**

  `( ... )`

- **`Brace`**

  `{ ... }`

- **`Bracket`**

  `[ ... ]`

- **`None`**

  `∅ ... ∅`
  
  An invisible delimiter, that may, for example, appear around tokens
  coming from a "macro variable" `$var`. It is important to preserve
  operator priorities in cases like `$var * 3` where `$var` is `1 + 2`.
  Invisible delimiters may not survive roundtrip of a token stream through
  a string.
  
  <div class="warning">
  
  Note: rustc currently can ignore the grouping of tokens delimited by `None` in the output
  of a proc_macro. Only `None`-delimited groups created by a macro_rules macro in the input
  of a proc_macro macro are preserved, and only in very specific circumstances.
  Any `None`-delimited groups (re)created by a proc_macro will therefore not preserve
  operator priorities as indicated above. The other `Delimiter` variants should be used
  instead in this context. This is a rustc bug. For details, see
  [rust-lang/rust#67062](https://github.com/rust-lang/rust/issues/67062).
  
  </div>

#### Trait Implementations

##### `impl Clone for Delimiter`

- `fn clone(self: &Self) -> Delimiter` — [`Delimiter`](#delimiter)

##### `impl Copy for Delimiter`

##### `impl Debug for Delimiter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Delimiter`

##### `impl PartialEq for Delimiter`

- `fn eq(self: &Self, other: &Delimiter) -> bool` — [`Delimiter`](#delimiter)

##### `impl StructuralPartialEq for Delimiter`

### `Spacing`

```rust
enum Spacing {
    Alone,
    Joint,
}
```

Whether a `Punct` is followed immediately by another `Punct` or followed by
another token or whitespace.

#### Variants

- **`Alone`**

  E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.

- **`Joint`**

  E.g. `+` is `Joint` in `+=` or `'` is `Joint` in `'#`.
  
  Additionally, single quote `'` can join with identifiers to form
  lifetimes `'ident`.

#### Trait Implementations

##### `impl Clone for Spacing`

- `fn clone(self: &Self) -> Spacing` — [`Spacing`](#spacing)

##### `impl Copy for Spacing`

##### `impl Debug for Spacing`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Spacing`

##### `impl PartialEq for Spacing`

- `fn eq(self: &Self, other: &Spacing) -> bool` — [`Spacing`](#spacing)

##### `impl StructuralPartialEq for Spacing`

## Macros

### `suffixed_int_literals!`

### `unsuffixed_int_literals!`

