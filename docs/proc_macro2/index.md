# Crate `proc_macro2`

[![github](#github)
](https://github.com/dtolnay/proc-macro2)&ensp;[![crates-io]](https://crates.io/crates/proc-macro2)&ensp;[![docs-rs]](crate)

[github](#github)
: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A wrapper around the procedural macro API of the compiler's [`proc_macro`](#proc-macro)
crate. This library serves two purposes:

- **Bring proc-macro-like functionality to other contexts like build.rs and
  main.rs.** Types from `proc_macro` are entirely specific to procedural
  macros and cannot ever exist in code outside of a procedural macro.
  Meanwhile `proc_macro2` types may exist anywhere including non-macro code.
  By developing foundational libraries like [syn](#syn)
 and [quote](#quote)
 against
  `proc_macro2` rather than `proc_macro`, the procedural macro ecosystem
  becomes easily applicable to many other use cases and we avoid
  reimplementing non-macro equivalents of those libraries.

- **Make procedural macros unit testable.** As a consequence of being
  specific to procedural macros, nothing that uses `proc_macro` can be
  executed from a unit test. In order for helper libraries or components of
  a macro to be testable in isolation, they must be implemented using
  `proc_macro2`.

[syn](#syn)
: https://github.com/dtolnay/syn
[quote](#quote)
: https://github.com/dtolnay/quote

# Usage

The skeleton of a typical procedural macro typically looks like this:

```
extern crate proc_macro;

# const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
# };
# #[cfg(wrap_proc_macro)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        /* transform input */
        # input
    };

    proc_macro::TokenStream::from(output)
}
```

If parsing with [Syn], you'll use [`parse_macro_input!`](#parse-macro-input) instead to
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

- [`extra`](extra/index.md) - Items which do not have a correspondence to any API in the proc_macro crate,
- [`token_stream`](token_stream/index.md) - Public implementation details for the `TokenStream` type, such as iterators.

## Structs

### `TokenStream`

```rust
struct TokenStream {
    // [REDACTED: Private Fields]
}
```

An abstract stream of tokens, or more concretely a sequence of token trees.

This type provides interfaces for iterating over token trees and for
collecting token trees into one stream.

Token stream is both the input and output of `#[proc_macro](#proc-macro)
`,
`#[proc_macro_attribute](#proc-macro-attribute)
` and `#[proc_macro_derive](#proc-macro-derive)
` definitions.

#### Implementations

- `fn new() -> Self`
  Returns an empty `TokenStream` containing no token trees.

- `fn is_empty(self: &Self) -> bool`
  Checks if this `TokenStream` is empty.

#### Trait Implementations

##### `impl From`

- `fn from(token: TokenTree) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(inner: proc_macro::TokenStream) -> Self`

##### `impl FromIterator`

- `fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self`

##### `impl FromIterator`

- `fn from_iter<I: IntoIterator<Item = TokenTree>>(streams: I) -> Self`

##### `impl FromStr`

- `type Err = LexError`

- `fn from_str(src: &str) -> Result<TokenStream, LexError>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator`

- `type Item = TokenTree`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> TokenStream`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Extend`

- `fn extend<I: IntoIterator<Item = TokenStream>>(self: &mut Self, streams: I)`

##### `impl Extend`

- `fn extend<I: IntoIterator<Item = TokenTree>>(self: &mut Self, streams: I)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Self`

##### `impl TokenStreamExt`

##### `impl ToTokens`

##### `impl Parse`

### `LexError`

```rust
struct LexError {
    // [REDACTED: Private Fields]
}
```

Error returned from `TokenStream::from_str`.

#### Implementations

- `fn span(self: &Self) -> Span`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Span`

```rust
struct Span {
    // [REDACTED: Private Fields]
}
```

A region of source code, along with macro expansion information.

#### Implementations

- `fn call_site() -> Self`
  The span of the invocation of the current procedural macro.

- `fn mixed_site() -> Self`
  The span located at the invocation of the procedural macro, but with

- `fn resolved_at(self: &Self, other: Span) -> Span`
  Creates a new span with the same line/column information as `self` but

- `fn located_at(self: &Self, other: Span) -> Span`
  Creates a new span with the same name resolution behavior as `self` but

- `fn unwrap(self: Self) -> proc_macro::Span`
  Convert `proc_macro2::Span` to `proc_macro::Span`.

- `fn join(self: &Self, other: Span) -> Option<Span>`
  Create a new span encompassing `self` and `other`.

- `fn source_text(self: &Self) -> Option<String>`
  Returns the source text behind a span. This preserves the original

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(proc_span: proc_macro::Span) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Span`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Group`

```rust
struct Group {
    // [REDACTED: Private Fields]
}
```

A delimited token stream.

A `Group` internally contains a `TokenStream` which is surrounded by
`Delimiter`s.

#### Implementations

- `fn new(delimiter: Delimiter, stream: TokenStream) -> Self`
  Creates a new `Group` with the given delimiter and token stream.

- `fn delimiter(self: &Self) -> Delimiter`
  Returns the punctuation used as the delimiter for this group: a set of

- `fn stream(self: &Self) -> TokenStream`
  Returns the `TokenStream` of tokens that are delimited in this `Group`.

- `fn span(self: &Self) -> Span`
  Returns the span for the delimiters of this token stream, spanning the

- `fn span_open(self: &Self) -> Span`
  Returns the span pointing to the opening delimiter of this group.

- `fn span_close(self: &Self) -> Span`
  Returns the span pointing to the closing delimiter of this group.

- `fn delim_span(self: &Self) -> DelimSpan`
  Returns an object that holds this group's `span_open()` and

- `fn set_span(self: &mut Self, span: Span)`
  Configures the span for this `Group`'s delimiters, but not its internal

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Group`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToTokens`

- `fn clone(self: &Self) -> DelimSpan`

##### `impl Parse`

##### `impl Token`

### `Punct`

```rust
struct Punct {
    // [REDACTED: Private Fields]
}
```

A `Punct` is a single punctuation character like `+`, `-` or `#`.

Multicharacter operators like `+=` are represented as two instances of
`Punct` with different forms of `Spacing` returned.

#### Implementations

- `fn new(ch: char, spacing: Spacing) -> Self`
  Creates a new `Punct` from the given character and spacing.

- `fn as_char(self: &Self) -> char`
  Returns the value of this punctuation character as `char`.

- `fn spacing(self: &Self) -> Spacing`
  Returns the spacing of this punctuation character, indicating whether

- `fn span(self: &Self) -> Span`
  Returns the span for this punctuation character.

- `fn set_span(self: &mut Self, span: Span)`
  Configure the span for this punctuation character.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Punct`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToTokens`

##### `impl Parse`

##### `impl Token`

### `Ident`

```rust
struct Ident {
    // [REDACTED: Private Fields]
}
```

A word of Rust code, which may be a keyword or legal variable name.

An identifier consists of at least one Unicode code point, the first of
which has the XID_Start property and the rest of which have the XID_Continue
property.

- The empty string is not an identifier. Use `Option<Ident>`.
- A lifetime is not an identifier. Use `syn::Lifetime` instead.

An identifier constructed with `Ident::new` is permitted to be a Rust
keyword, though parsing one through its [`Parse`](../regex_syntax/index.md) implementation rejects
Rust keywords. Use `input.call(Ident::parse_any)` when parsing to match the
behaviour of `Ident::new`.

# Examples

A new ident can be created from a string using the `Ident::new` function.
A span must be provided explicitly which governs the name resolution
behavior of the resulting identifier.

```
use proc_macro2::{Ident, Span};

fn main() {
    let call_ident = Ident::new("calligraphy", Span::call_site());

    println!("{}", call_ident);
}
```

An ident can be interpolated into a token stream using the `quote!` macro.

```
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

```
# use proc_macro2::{Ident, Span};
#
# let ident = Ident::new("another_identifier", Span::call_site());
#
// Examine the ident as a string.
let ident_string = ident.to_string();
if ident_string.len() > 60 {
    println!("Very long identifier: {}", ident_string)
}
```

#### Implementations

- `fn new(string: &str, span: Span) -> Self`
  Creates a new `Ident` with the given `string` as well as the specified

- `fn new_raw(string: &str, span: Span) -> Self`
  Same as `Ident::new`, but creates a raw identifier (`r#ident`). The

- `fn span(self: &Self) -> Span`
  Returns the span of this `Ident`.

- `fn set_span(self: &mut Self, span: Span)`
  Configures the span of this `Ident`, possibly changing its hygiene

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Ident`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, hasher: &mut H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Ident) -> Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Ident) -> bool`

##### `impl PartialEq<T>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Ident) -> Option<Ordering>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IdentFragment`

##### `impl ToTokens`

##### `impl Parse`

##### `impl Token`

##### `impl IdentExt`

### `Literal`

```rust
struct Literal {
    // [REDACTED: Private Fields]
}
```

A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
byte character (`b'a'`), an integer or floating point number with or without
a suffix (`1`, `1u8`, `2.3`, `2.3f32`).

Boolean literals like `true` and `false` do not belong here, they are
`Ident`s.

#### Implementations

- `fn u8_suffixed(n: u8) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn u16_suffixed(n: u16) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn u32_suffixed(n: u32) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn u64_suffixed(n: u64) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn u128_suffixed(n: u128) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn usize_suffixed(n: usize) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn i8_suffixed(n: i8) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn i16_suffixed(n: i16) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn i32_suffixed(n: i32) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn i64_suffixed(n: i64) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn i128_suffixed(n: i128) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn isize_suffixed(n: isize) -> Literal`
  Creates a new suffixed integer literal with the specified value.

- `fn u8_unsuffixed(n: u8) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn u16_unsuffixed(n: u16) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn u32_unsuffixed(n: u32) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn u64_unsuffixed(n: u64) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn u128_unsuffixed(n: u128) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn usize_unsuffixed(n: usize) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn i8_unsuffixed(n: i8) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn i16_unsuffixed(n: i16) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn i32_unsuffixed(n: i32) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn i64_unsuffixed(n: i64) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn i128_unsuffixed(n: i128) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn isize_unsuffixed(n: isize) -> Literal`
  Creates a new unsuffixed integer literal with the specified value.

- `fn f64_unsuffixed(f: f64) -> Literal`
  Creates a new unsuffixed floating-point literal.

- `fn f64_suffixed(f: f64) -> Literal`
  Creates a new suffixed floating-point literal.

- `fn f32_unsuffixed(f: f32) -> Literal`
  Creates a new unsuffixed floating-point literal.

- `fn f32_suffixed(f: f32) -> Literal`
  Creates a new suffixed floating-point literal.

- `fn string(string: &str) -> Literal`
  String literal.

- `fn character(ch: char) -> Literal`
  Character literal.

- `fn byte_character(byte: u8) -> Literal`
  Byte character literal.

- `fn byte_string(bytes: &[u8]) -> Literal`
  Byte string literal.

- `fn c_string(string: &CStr) -> Literal`
  C string literal.

- `fn span(self: &Self) -> Span`
  Returns the span encompassing this literal.

- `fn set_span(self: &mut Self, span: Span)`
  Configures the span associated for this literal.

- `fn subspan<R: RangeBounds<usize>>(self: &Self, range: R) -> Option<Span>`
  Returns a `Span` that is a subset of `self.span()` containing only

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = LexError`

- `fn from_str(repr: &str) -> Result<Self, LexError>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Literal`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToTokens`

##### `impl Parse`

##### `impl Token`

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

- `fn span(self: &Self) -> Span`
  Returns the span of this tree, delegating to the `span` method of

- `fn set_span(self: &mut Self, span: Span)`
  Configures the span for *only this token*.

#### Trait Implementations

##### `impl From`

- `fn from(g: Group) -> Self`

##### `impl From`

- `fn from(g: Ident) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(g: Punct) -> Self`

##### `impl From`

- `fn from(g: Literal) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> TokenTree`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToTokens`

##### `impl Token`

##### `impl Parse`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Delimiter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Delimiter) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Spacing`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Spacing) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

