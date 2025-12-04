# Crate `syn`

[![github](#github)
](https://github.com/dtolnay/syn)&ensp;[![crates-io]](https://crates.io/crates/syn)&ensp;[![docs-rs]](crate)

[github](#github)
: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

Syn is a parsing library for parsing a stream of Rust tokens into a syntax
tree of Rust source code.

Currently this library is geared toward use in Rust procedural macros, but
contains some APIs that may be useful more generally.

- **Data structures** — Syn provides a complete syntax tree that can
  represent any valid Rust source code. The syntax tree is rooted at
  `syn::File` which represents a full source file, but there are other
  entry points that may be useful to procedural macros including
  `syn::Item`, `syn::Expr` and `syn::Type`.

- **Derives** — Of particular interest to derive macros is
  `syn::DeriveInput` which is any of the three legal input items to a
  derive macro. An example below shows using this type in a library that can
  derive implementations of a user-defined trait.

- **Parsing** — Parsing in Syn is built around [parser functions] with the
  signature `fn(ParseStream) -> Result<T>`. Every syntax tree node defined
  by Syn is individually parsable and may be used as a building block for
  custom syntaxes, or you may dream up your own brand new syntax without
  involving any of our syntax tree types.

- **Location information** — Every token parsed by Syn is associated with a
  `Span` that tracks line and column information back to the source of that
  token. These spans allow a procedural macro to display detailed error
  messages pointing to all the right places in the user's code. There is an
  example of this below.

- **Feature flags** — Functionality is aggressively feature gated so your
  procedural macros enable only what they need, and do not pay in compile
  time for all the rest.





[parser functions]: mod@parse

<br>

# Example of a derive macro

The canonical derive macro using Syn looks like this. We write an ordinary
Rust function tagged with a `proc_macro_derive` attribute and the name of
the trait we are deriving. Any time that derive appears in the user's code,
the Rust compiler passes their data structure as tokens into our macro. We
get to execute arbitrary Rust code to figure out what to do with those
tokens, then hand some tokens back to the compiler to compile into the
user's crate.

```toml
[dependencies](#dependencies)

syn = "2.0"
quote = "1.0"

[lib](#lib)

proc-macro = true
```

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

# const IGNORE_TOKENS: &str = stringify! {
#[proc_macro_derive(MyMacro)]
# };
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
```

The [`heapsize`](#heapsize) example directory shows a complete working implementation
of a derive macro. The example derives a `HeapSize` trait which computes an
estimate of the amount of heap memory owned by a value.

```
pub trait HeapSize {
    /// Total number of bytes of heap memory owned by `self`.
    fn heap_size_of_children(&self) -> usize;
}
```

The derive macro allows users to write `#[derive(HeapSize)]` on data
structures in their program.

```
# const IGNORE_TOKENS: &str = stringify! {
#[derive(HeapSize)]
# };
struct Demo<'a, T: ?Sized> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: String,
}
```

<p><br></p>

# Spans and error reporting

The token-based procedural macro API provides great control over where the
compiler's error messages are displayed in user code. Consider the error the
user sees if one of their field types does not implement `HeapSize`.

```
# const IGNORE_TOKENS: &str = stringify! {
#[derive(HeapSize)]
# };
struct Broken {
    ok: String,
    bad: std::thread::Thread,
}
```

By tracking span information all the way through the expansion of a
procedural macro as shown in the `heapsize` example, token-based macros in
Syn are able to trigger errors that directly pinpoint the source of the
problem.

```text
error[E0277]: the trait bound `std::thread::Thread: HeapSize` is not satisfied
 --> src/main.rs:7:5
  |
7 |     bad: std::thread::Thread,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `HeapSize` is not implemented for `Thread`
```

<br>

# Parsing a custom syntax

The [`lazy-static`](#lazy-static) example directory shows the implementation of a
`functionlike!(...)` procedural macro in which the input tokens are parsed
using Syn's parsing API.

The example reimplements the popular `lazy_static` crate from crates.io as a
procedural macro.

```
# macro_rules! lazy_static {
#     ($($tt:tt)*) => {}
# }
#
lazy_static! {
    static ref USERNAME: Regex = Regex::new("^[a-z0-9_-]{3,16}$").unwrap();
}
```

The implementation shows how to trigger custom warnings and error messages
on the macro input.

```text
warning: come on, pick a more creative name
  --> src/main.rs:10:16
   |
10 |     static ref FOO: String = "lazy_static".to_owned();
   |                ^^^
```

<br>

# Testing

When testing macros, we often care not just that the macro can be used
successfully but also that when the macro is provided with invalid input it
produces maximally helpful error messages. Consider using the [`trybuild`](#trybuild)
crate to write tests for errors that are emitted by your macro or errors
detected by the Rust compiler in the expanded code following misuse of the
macro. Such tests help avoid regressions from later refactors that
mistakenly make an error no longer trigger or be less helpful than it used
to be.

<br>

# Debugging

When developing a procedural macro it can be helpful to look at what the
generated code looks like. Use `cargo rustc -- -Zunstable-options
--pretty=expanded` or the [`cargo expand`](#cargo-expand) subcommand.

To show the expanded code for some crate that uses your procedural macro,
run `cargo expand` from that crate. To show the expanded code for one of
your own test cases, run `cargo expand --test the_test_case` where the last
argument is the name of the test file without the `.rs` extension.

This write-up by Brandon W Maister discusses debugging in more detail:
[Debugging Rust's new Custom Derive system][debugging](#debugging)
.

[debugging](#debugging)
: https://quodlibetor.github.io/posts/debugging-rusts-new-custom-derive-system/

<br>

# Optional features

Syn puts a lot of functionality behind optional features in order to
optimize compile time for the most common use cases. The following features
are available.

- **`derive`** *(enabled by default)* — Data structures for representing the
  possible input to a derive macro, including structs and enums and types.
- **`full`** — Data structures for representing the syntax tree of all valid
  Rust source code, including items and expressions.
- **`parsing`** *(enabled by default)* — Ability to parse input tokens into
  a syntax tree node of a chosen type.
- **`printing`** *(enabled by default)* — Ability to print a syntax tree
  node as tokens of Rust source code.
- **`visit`** — Trait for traversing a syntax tree.
- **`visit-mut`** — Trait for traversing and mutating in place a syntax
  tree.
- **`fold`** — Trait for transforming an owned syntax tree.
- **`clone-impls`** *(enabled by default)* — Clone impls for all syntax tree
  types.
- **`extra-traits`** — Debug, Eq, PartialEq, Hash impls for all syntax tree
  types.
- **`proc-macro`** *(enabled by default)* — Runtime dependency on the
  dynamic library libproc_macro from rustc toolchain.

## Modules

- [`token`](token/index.md) - Tokens representing Rust punctuation, keywords, and delimiters.
- [`buffer`](buffer/index.md) - A stably addressed token buffer supporting efficient traversal based on a
- [`ext`](ext/index.md) - Extension traits to provide parsing methods on foreign types.
- [`meta`](meta/index.md) - Facility for interpreting structured content inside of an `Attribute`.
- [`parse`](parse/index.md) - Parsing interface for parsing a token stream into a syntax tree node.
- [`punctuated`](punctuated/index.md) - A punctuated sequence of syntax tree nodes separated by punctuation.
- [`spanned`](spanned/index.md) - A trait that can provide the `Span` of the complete contents of a syntax

## Structs

### `Attribute`

```rust
struct Attribute {
    pub pound_token: $crate::token::Pound,
    pub style: AttrStyle,
    pub bracket_token: token::Bracket,
    pub meta: Meta,
}
```

An attribute, like `#[repr(transparent)]`.

<br>

# Syntax

Rust has six types of attributes.

- Outer attributes like `#[repr(transparent)]`. These appear outside or
  in front of the item they describe.

- Inner attributes like `#![feature(proc_macro)]`. These appear inside
  of the item they describe, usually a module.

- Outer one-line doc comments like `/// Example`.

- Inner one-line doc comments like `//! Please file an issue`.

- Outer documentation blocks `/** Example */`.

- Inner documentation blocks `/*! Please file an issue */`.

The `style` field of type `AttrStyle` distinguishes whether an attribute
is outer or inner.

Every attribute has a `path` that indicates the intended interpretation
of the rest of the attribute's contents. The path and the optional
additional contents are represented together in the `meta` field of the
attribute in three possible varieties:

- Meta::Path &mdash; attributes whose information content conveys just a
  path, for example the `#[test](#test)
` attribute.

- Meta::List &mdash; attributes that carry arbitrary tokens after the
  path, surrounded by a delimiter (parenthesis, bracket, or brace). For
  example `#[derive(Copy)]` or `#[precondition(x < 5)]`.

- Meta::NameValue &mdash; attributes with an `=` sign after the path,
  followed by a Rust expression. For example `#[path =
  "sys/windows.rs"]`.

All doc comments are represented in the NameValue style with a path of
"doc", as this is how they are processed by the compiler and by
`macro_rules!` macros.

```text
#[derive(Copy, Clone)]
  ~~~~~~Path
  ^^^^^^^^^^^^^^^^^^^Meta::List

#[path = "sys/windows.rs"]
  ~~~~Path
  ^^^^^^^^^^^^^^^^^^^^^^^Meta::NameValue

#[test](#test)

  ^^^^Meta::Path
```

<br>

# Parsing from tokens to Attribute

This type does not implement the [`Parse`](parse/index.md) trait and thus cannot be
parsed directly by `ParseStream::parse`. Instead use
`ParseStream::call` with one of the two parser functions
`Attribute::parse_outer` or `Attribute::parse_inner` depending on
which you intend to parse.



```
use syn::{Attribute, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};

// Parses a unit struct with attributes.
//
//     #[path = "s.tmpl"]
//     struct S;
struct UnitStruct {
    attrs: Vec<Attribute>,
    struct_token: Token![struct](#struct)
,
    name: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(UnitStruct {
            attrs: input.call(Attribute::parse_outer)?,
            struct_token: input.parse()?,
            name: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}
```

<p><br></p>

# Parsing from Attribute to structured arguments

The grammar of attributes in Rust is very flexible, which makes the
syntax tree not that useful on its own. In particular, arguments of the
`Meta::List` variety of attribute are held in an arbitrary `tokens:
TokenStream`. Macros are expected to check the `path` of the attribute,
decide whether they recognize it, and then parse the remaining tokens
according to whatever grammar they wish to require for that kind of
attribute. Use [`parse_args()`](#parse-args) to parse those tokens into the expected
data structure.

<p><br></p>

# Doc comments

The compiler transforms doc comments, such as `/// comment` and `/*!
comment */`, into attributes before macros are expanded. Each comment is
expanded into an attribute of the form `#[doc = r"comment"]`.

As an example, the following `mod` items are expanded identically:

```
# use syn::{ItemMod, parse_quote};
let doc: ItemMod = parse_quote! {
    /// Single line doc comments
    /// We write so many!
    /**
     * Multi-line comments...
     * May span many lines
     */
    mod example {
        //! Of course, they can be inner too
        /*! And fit in a single line */
    }
};
let attr: ItemMod = parse_quote! {
    #[doc = r" Single line doc comments"]
    #[doc = r" We write so many!"]
    #[doc = r"
     * Multi-line comments...
     * May span many lines
     "]
    mod example {
        #![doc = r" Of course, they can be inner too"]
        #![doc = r" And fit in a single line "]
    }
};
assert_eq!(doc, attr);
```

#### Implementations

- `fn path(self: &Self) -> &Path`
  Returns the path that identifies the interpretation of this attribute.

- `fn parse_args<T: Parse>(self: &Self) -> Result<T>`
  Parse the arguments to the attribute as a syntax tree.

- `fn parse_args_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>`
  Parse the arguments to the attribute using the given parser.

- `fn parse_nested_meta(self: &Self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>`
  Parse the arguments to the attribute, expecting it to follow the

- `fn parse_outer(input: ParseStream<'_>) -> Result<Vec<Self>>`
  Parses zero or more outer attributes from the stream.

- `fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>>`
  Parses zero or more inner attributes from the stream.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `MetaList`

```rust
struct MetaList {
    pub path: crate::path::Path,
    pub delimiter: crate::mac::MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

A structured list within an attribute, like `derive(Copy, Clone)`.

#### Implementations

- `fn parse_args<T: Parse>(self: &Self) -> Result<T>`
  See [`Attribute::parse_args`].

- `fn parse_args_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>`
  See [`Attribute::parse_args_with`].

- `fn parse_nested_meta(self: &Self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>`
  See [`Attribute::parse_nested_meta`].

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `MetaNameValue`

```rust
struct MetaNameValue {
    pub path: crate::path::Path,
    pub eq_token: $crate::token::Eq,
    pub value: crate::expr::Expr,
}
```

A name-value pair within an attribute, like `feature = "nightly"`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Field`

```rust
struct Field {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub mutability: crate::restriction::FieldMutability,
    pub ident: Option<crate::ident::Ident>,
    pub colon_token: Option<$crate::token::Colon>,
    pub ty: crate::ty::Type,
}
```

A field of a struct or enum variant.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  Name of the field, if any.
  
  Fields of tuple structs have no names.

#### Implementations

- `fn parse_named(input: ParseStream<'_>) -> Result<Self>`
  Parses a named (braced struct) field.

- `fn parse_unnamed(input: ParseStream<'_>) -> Result<Self>`
  Parses an unnamed (tuple struct) field.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `FieldsNamed`

```rust
struct FieldsNamed {
    pub brace_token: token::Brace,
    pub named: crate::punctuated::Punctuated<Field, $crate::token::Comma>,
}
```

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `FieldsUnnamed`

```rust
struct FieldsUnnamed {
    pub paren_token: token::Paren,
    pub unnamed: crate::punctuated::Punctuated<Field, $crate::token::Comma>,
}
```

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Variant`

```rust
struct Variant {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub fields: Fields,
    pub discriminant: Option<($crate::token::Eq, crate::expr::Expr)>,
}
```

An enum variant.

#### Fields

- **`ident`**: `crate::ident::Ident`

  Name of the variant.

- **`fields`**: `Fields`

  Content stored in the variant.

- **`discriminant`**: `Option<($crate::token::Eq, crate::expr::Expr)>`

  Explicit discriminant: `Variant = 1`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `DataEnum`

```rust
struct DataEnum {
    pub enum_token: $crate::token::Enum,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>,
}
```

An enum input to a `proc_macro_derive` macro.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `DataStruct`

```rust
struct DataStruct {
    pub struct_token: $crate::token::Struct,
    pub fields: crate::data::Fields,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A struct input to a `proc_macro_derive` macro.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `DataUnion`

```rust
struct DataUnion {
    pub union_token: $crate::token::Union,
    pub fields: crate::data::FieldsNamed,
}
```

An untagged union input to a `proc_macro_derive` macro.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `DeriveInput`

```rust
struct DeriveInput {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub data: Data,
}
```

Data structure sent to a `proc_macro_derive` macro.

#### Trait Implementations

##### `impl From`

- `fn from(input: ItemEnum) -> DeriveInput`

##### `impl From`

- `fn from(input: ItemStruct) -> DeriveInput`

##### `impl From`

- `fn from(input: ItemUnion) -> DeriveInput`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Error`

```rust
struct Error {
    // [REDACTED: Private Fields]
}
```

Error returned when a Syn parser cannot parse the input tokens.

# Error reporting in proc macros

The correct way to report errors back to the compiler from a procedural
macro is by emitting an appropriately spanned invocation of
[`compile_error!`](#compile-error) in the generated code. This produces a better diagnostic
message than simply panicking the macro.

When parsing macro input, the [`parse_macro_input!`](#parse-macro-input) macro handles the
conversion to `compile_error!` automatically.

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, ItemFn};

# const IGNORE: &str = stringify! {
#[proc_macro_attribute](#proc-macro-attribute)

# };
pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MyAttrArgs);
    let input = parse_macro_input!(input as ItemFn);

    /* ... */
    # TokenStream::new()
}

struct MyAttrArgs {
    # _k: [(); { stringify! {
    ...
    # }; 0 }]
}

impl Parse for MyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        # stringify! {
        ...
        # };
        # unimplemented!()
    }
}
```

For errors that arise later than the initial parsing stage, the
[`.to_compile_error()`](#to-compile-error) or [`.into_compile_error()`](#into-compile-error) methods can be used to
perform an explicit conversion to `compile_error!`.


```
# extern crate proc_macro;
#
# use proc_macro::TokenStream;
# use syn::{parse_macro_input, DeriveInput};
#
# const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
# };
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // fn(DeriveInput) -> syn::Result<proc_macro2::TokenStream>
    expand::my_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
#
# mod expand {
#     use proc_macro2::TokenStream;
#     use syn::{DeriveInput, Result};
#
#     pub fn my_derive(input: DeriveInput) -> Result<TokenStream> {
#         unimplemented!()
#     }
# }
```

#### Implementations

- `fn new<T: Display>(span: Span, message: T) -> Self`
  Usually the [`ParseStream::error`] method will be used instead, which

- `fn new_spanned<T: ToTokens, U: Display>(tokens: T, message: U) -> Self`
  Creates an error with the specified message spanning the given syntax

- `fn span(self: &Self) -> Span`
  The source location of the error.

- `fn to_compile_error(self: &Self) -> TokenStream`
  Render the error as an invocation of [`compile_error!`].

- `fn into_compile_error(self: Self) -> TokenStream`
  Render the error as an invocation of [`compile_error!`].

- `fn combine(self: &mut Self, another: Error)`
  Add another error message to self such that when `to_compile_error()` is

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(err: LexError) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator`

- `type Item = Error`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl Extend`

- `fn extend<T: IntoIterator<Item = Error>>(self: &mut Self, iter: T)`

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

### `Arm`

```rust
struct Arm {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: crate::pat::Pat,
    pub guard: Option<($crate::token::If, Box<Expr>)>,
    pub fat_arrow_token: $crate::token::FatArrow,
    pub body: Box<Expr>,
    pub comma: Option<$crate::token::Comma>,
}
```

One arm of a `match` expression: `0..=10 => { return true; }`.

As in:

```
# fn f() -> bool {
#     let n = 0;
match n {
    0..=10 => {
        return true;
    }
    // ...
    # _ => {}
}
#   false
# }
```

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Arm>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Label`

```rust
struct Label {
    pub name: crate::lifetime::Lifetime,
    pub colon_token: $crate::token::Colon,
}
```

A lifetime labeling a `for`, `while`, or `loop`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBinary`

```rust
struct ExprBinary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub op: crate::op::BinOp,
    pub right: Box<Expr>,
}
```

A binary operation: `a + b`, `a += b`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprCall`

```rust
struct ExprCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub func: Box<Expr>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A function call expression: `invoke(a, b)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprCast`

```rust
struct ExprCast {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub as_token: $crate::token::As,
    pub ty: Box<crate::ty::Type>,
}
```

A cast expression: `foo as f64`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprField`

```rust
struct ExprField {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: $crate::token::Dot,
    pub member: Member,
}
```

Access of a named struct field (`obj.k`) or unnamed tuple struct
field (`obj.0`).

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprIndex`

```rust
struct ExprIndex {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub bracket_token: token::Bracket,
    pub index: Box<Expr>,
}
```

A square bracketed indexing expression: `vector[2]`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprLit`

```rust
struct ExprLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprMacro`

```rust
struct ExprMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprMethodCall`

```rust
struct ExprMethodCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub receiver: Box<Expr>,
    pub dot_token: $crate::token::Dot,
    pub method: crate::ident::Ident,
    pub turbofish: Option<crate::path::AngleBracketedGenericArguments>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A method call expression: `x.foo::<T>(a, b)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprParen`

```rust
struct ExprParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub expr: Box<Expr>,
}
```

A parenthesized expression: `(a + b)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprPath`

```rust
struct ExprPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprReference`

```rust
struct ExprReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: $crate::token::And,
    pub mutability: Option<$crate::token::Mut>,
    pub expr: Box<Expr>,
}
```

A referencing operation: `&a` or `&mut a`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprStruct`

```rust
struct ExprStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldValue, $crate::token::Comma>,
    pub dot2_token: Option<$crate::token::DotDot>,
    pub rest: Option<Box<Expr>>,
}
```

A struct literal expression: `Point { x: 1, y: 1 }`.

The `rest` provides the value of the remaining fields as in `S { a:
1, b: 1, ..rest }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprUnary`

```rust
struct ExprUnary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub op: crate::op::UnOp,
    pub expr: Box<Expr>,
}
```

A unary operation: `!x`, `*x`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `FieldValue`

```rust
struct FieldValue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: Member,
    pub colon_token: Option<$crate::token::Colon>,
    pub expr: Expr,
}
```

A field-value pair in a struct literal.

#### Fields

- **`colon_token`**: `Option<$crate::token::Colon>`

  The colon in `Struct { x: x }`. If written in shorthand like
  `Struct { x }`, there is no colon.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Index`

```rust
struct Index {
    pub index: u32,
    pub span: proc_macro2::Span,
}
```

The index of an unnamed tuple struct field.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(index: usize) -> Index`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl IdentFragment`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn span(self: &Self) -> Option<Span>`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprArray`

```rust
struct ExprArray {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A slice literal expression: `[a, b, c, d]`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAssign`

```rust
struct ExprAssign {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub eq_token: $crate::token::Eq,
    pub right: Box<Expr>,
}
```

An assignment expression: `a = compute()`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAsync`

```rust
struct ExprAsync {
    pub attrs: Vec<crate::attr::Attribute>,
    pub async_token: $crate::token::Async,
    pub capture: Option<$crate::token::Move>,
    pub block: crate::stmt::Block,
}
```

An async block: `async { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAwait`

```rust
struct ExprAwait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: $crate::token::Dot,
    pub await_token: $crate::token::Await,
}
```

An await expression: `fut.await`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBlock`

```rust
struct ExprBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub block: crate::stmt::Block,
}
```

A blocked scope: `{ ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBreak`

```rust
struct ExprBreak {
    pub attrs: Vec<crate::attr::Attribute>,
    pub break_token: $crate::token::Break,
    pub label: Option<crate::lifetime::Lifetime>,
    pub expr: Option<Box<Expr>>,
}
```

A `break`, with an optional label to break and an optional
expression.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprClosure`

```rust
struct ExprClosure {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub constness: Option<$crate::token::Const>,
    pub movability: Option<$crate::token::Static>,
    pub asyncness: Option<$crate::token::Async>,
    pub capture: Option<$crate::token::Move>,
    pub or1_token: $crate::token::Or,
    pub inputs: crate::punctuated::Punctuated<crate::pat::Pat, $crate::token::Comma>,
    pub or2_token: $crate::token::Or,
    pub output: crate::ty::ReturnType,
    pub body: Box<Expr>,
}
```

A closure expression: `|a, b| a + b`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprConst`

```rust
struct ExprConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprContinue`

```rust
struct ExprContinue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub continue_token: $crate::token::Continue,
    pub label: Option<crate::lifetime::Lifetime>,
}
```

A `continue`, with an optional label.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprForLoop`

```rust
struct ExprForLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub for_token: $crate::token::For,
    pub pat: Box<crate::pat::Pat>,
    pub in_token: $crate::token::In,
    pub expr: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A for loop: `for pat in expr { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprGroup`

```rust
struct ExprGroup {
    pub attrs: Vec<crate::attr::Attribute>,
    pub group_token: token::Group,
    pub expr: Box<Expr>,
}
```

An expression contained within invisible delimiters.

This variant is important for faithfully representing the precedence
of expressions and is related to `None`-delimited spans in a
`TokenStream`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprIf`

```rust
struct ExprIf {
    pub attrs: Vec<crate::attr::Attribute>,
    pub if_token: $crate::token::If,
    pub cond: Box<Expr>,
    pub then_branch: crate::stmt::Block,
    pub else_branch: Option<($crate::token::Else, Box<Expr>)>,
}
```

An `if` expression with an optional `else` block: `if expr { ... }
else { ... }`.

The `else` branch expression may only be an `If` or `Block`
expression, not any of the other types of expression.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprInfer`

```rust
struct ExprInfer {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: $crate::token::Underscore,
}
```

The inferred value of a const generic argument, denoted `_`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprLet`

```rust
struct ExprLet {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: $crate::token::Let,
    pub pat: Box<crate::pat::Pat>,
    pub eq_token: $crate::token::Eq,
    pub expr: Box<Expr>,
}
```

A `let` guard: `let Some(x) = opt`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprLoop`

```rust
struct ExprLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub loop_token: $crate::token::Loop,
    pub body: crate::stmt::Block,
}
```

Conditionless loop: `loop { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprMatch`

```rust
struct ExprMatch {
    pub attrs: Vec<crate::attr::Attribute>,
    pub match_token: $crate::token::Match,
    pub expr: Box<Expr>,
    pub brace_token: token::Brace,
    pub arms: Vec<Arm>,
}
```

A `match` expression: `match n { Some(n) => {}, None => {} }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprRange`

```rust
struct ExprRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprRawAddr`

```rust
struct ExprRawAddr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: $crate::token::And,
    pub raw: $crate::token::Raw,
    pub mutability: PointerMutability,
    pub expr: Box<Expr>,
}
```

Address-of operation: `&raw const place` or `&raw mut place`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprRepeat`

```rust
struct ExprRepeat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub expr: Box<Expr>,
    pub semi_token: $crate::token::Semi,
    pub len: Box<Expr>,
}
```

An array literal constructed from one repeated element: `[0u8; N]`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprReturn`

```rust
struct ExprReturn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub return_token: $crate::token::Return,
    pub expr: Option<Box<Expr>>,
}
```

A `return`, with an optional value to be returned.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTry`

```rust
struct ExprTry {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub question_token: $crate::token::Question,
}
```

A try-expression: `expr?`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTryBlock`

```rust
struct ExprTryBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub try_token: $crate::token::Try,
    pub block: crate::stmt::Block,
}
```

A try block: `try { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTuple`

```rust
struct ExprTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A tuple expression: `(a, b, c, d)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprUnsafe`

```rust
struct ExprUnsafe {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafe_token: $crate::token::Unsafe,
    pub block: crate::stmt::Block,
}
```

An unsafe block: `unsafe { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprWhile`

```rust
struct ExprWhile {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub while_token: $crate::token::While,
    pub cond: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A while loop: `while expr { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ExprYield`

```rust
struct ExprYield {
    pub attrs: Vec<crate::attr::Attribute>,
    pub yield_token: $crate::token::Yield,
    pub expr: Option<Box<Expr>>,
}
```

A yield expression: `yield expr`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `File`

```rust
struct File {
    pub shebang: Option<String>,
    pub attrs: Vec<crate::attr::Attribute>,
    pub items: Vec<crate::item::Item>,
}
```

A complete file of Rust source code.

Typically `File` objects are created with [`parse_file`](#parse-file).

# Example

Parse a Rust source file into a `syn::File` and print out a debug
representation of the syntax tree.

```
use std::env;
use std::fs;
use std::process;

fn main() {
# }
#
# fn fake_main() {
    let mut args = env::args();
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let src = fs::read_to_string(&filename).expect("unable to read file");
    let syntax = syn::parse_file(&src).expect("unable to parse file");

    // Debug impl is available if Syn is built with "extra-traits" feature.
    println!("{:#?}", syntax);
}
```

Running with its own source code as input, this program prints output
that begins with:

```text
File {
    shebang: None,
    attrs: [],
    items: [
        Use(
            ItemUse {
                attrs: [],
                vis: Inherited,
                use_token: Use,
                leading_colon: None,
                tree: Path(
                    UsePath {
                        ident: Ident(
                            std,
                        ),
                        colon2_token: Colon2,
                        tree: Name(
                            UseName {
                                ident: Ident(
                                    env,
                                ),
                            },
                        ),
                    },
                ),
                semi_token: Semi,
            },
        ),
...
```

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `BoundLifetimes`

```rust
struct BoundLifetimes {
    pub for_token: $crate::token::For,
    pub lt_token: $crate::token::Lt,
    pub lifetimes: crate::punctuated::Punctuated<GenericParam, $crate::token::Comma>,
    pub gt_token: $crate::token::Gt,
}
```

A set of bound lifetimes: `for<'a, 'b, 'c>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default`

- `fn default() -> Self`

### `ConstParam`

```rust
struct ConstParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub colon_token: $crate::token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: Option<$crate::token::Eq>,
    pub default: Option<crate::expr::Expr>,
}
```

A const generic parameter: `const LENGTH: usize`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Generics`

```rust
struct Generics {
    pub lt_token: Option<$crate::token::Lt>,
    pub params: crate::punctuated::Punctuated<GenericParam, $crate::token::Comma>,
    pub gt_token: Option<$crate::token::Gt>,
    pub where_clause: Option<WhereClause>,
}
```

Lifetimes and type parameters attached to a declaration of a function,
enum, trait, etc.

This struct represents two distinct optional syntactic elements,
[generic parameters] and [where clause]. In some locations of the
grammar, there may be other tokens in between these two things.

[generic parameters]: https://doc.rust-lang.org/stable/reference/items/generics.html#generic-parameters
[where clause]: https://doc.rust-lang.org/stable/reference/items/generics.html#where-clauses

#### Implementations

- `fn lifetimes(self: &Self) -> Lifetimes<'_>`
  Iterator over the lifetime parameters in `self.params`.

- `fn lifetimes_mut(self: &mut Self) -> LifetimesMut<'_>`
  Iterator over the lifetime parameters in `self.params`.

- `fn type_params(self: &Self) -> TypeParams<'_>`
  Iterator over the type parameters in `self.params`.

- `fn type_params_mut(self: &mut Self) -> TypeParamsMut<'_>`
  Iterator over the type parameters in `self.params`.

- `fn const_params(self: &Self) -> ConstParams<'_>`
  Iterator over the constant parameters in `self.params`.

- `fn const_params_mut(self: &mut Self) -> ConstParamsMut<'_>`
  Iterator over the constant parameters in `self.params`.

- `fn make_where_clause(self: &mut Self) -> &mut WhereClause`
  Initializes an empty `where`-clause if there is not one present already.

- `fn split_for_impl(self: &Self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>)`
  Split a type's generics into the pieces required for impl'ing a trait

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default`

- `fn default() -> Self`

### `LifetimeParam`

```rust
struct LifetimeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: Option<$crate::token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, $crate::token::Plus>,
}
```

A lifetime definition: `'a: 'b + 'c + 'd`.

#### Implementations

- `fn new(lifetime: Lifetime) -> Self`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PredicateLifetime`

```rust
struct PredicateLifetime {
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: $crate::token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, $crate::token::Plus>,
}
```

A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PredicateType`

```rust
struct PredicateType {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: crate::ty::Type,
    pub colon_token: $crate::token::Colon,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>,
}
```

A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  Any lifetimes from a `for` binding

- **`bounded_ty`**: `crate::ty::Type`

  The type being bounded

- **`bounds`**: `crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>`

  Trait and lifetime bounds (`Clone+Send+'static`)

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TraitBound`

```rust
struct TraitBound {
    pub paren_token: Option<token::Paren>,
    pub modifier: TraitBoundModifier,
    pub lifetimes: Option<BoundLifetimes>,
    pub path: crate::path::Path,
}
```

A trait used as a bound on a type parameter.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  The `for<'a>` in `for<'a> Foo<&'a T>`

- **`path`**: `crate::path::Path`

  The `Foo<&'a T>` in `for<'a> Foo<&'a T>`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParam`

```rust
struct TypeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub colon_token: Option<$crate::token::Colon>,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>,
    pub eq_token: Option<$crate::token::Eq>,
    pub default: Option<crate::ty::Type>,
}
```

A generic type parameter: `T: Into<String>`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(ident: Ident) -> Self`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `WhereClause`

```rust
struct WhereClause {
    pub where_token: $crate::token::Where,
    pub predicates: crate::punctuated::Punctuated<WherePredicate, $crate::token::Comma>,
}
```

A `where` clause in a definition: `where T: Deserialize<'de>, D:
'static`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PreciseCapture`

```rust
struct PreciseCapture {
    pub use_token: $crate::token::Use,
    pub lt_token: $crate::token::Lt,
    pub params: crate::punctuated::Punctuated<CapturedParam, $crate::token::Comma>,
    pub gt_token: $crate::token::Gt,
}
```

Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
use<'a, T>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ImplGenerics<'a>`

```rust
struct ImplGenerics<'a>();
```

Returned by `Generics::split_for_impl`.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Turbofish<'a>`

```rust
struct Turbofish<'a>();
```

Returned by `TypeGenerics::as_turbofish`.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeGenerics<'a>`

```rust
struct TypeGenerics<'a>();
```

Returned by `Generics::split_for_impl`.

#### Implementations

- `fn as_turbofish(self: &Self) -> Turbofish<'a>`
  Turn a type's generics like `<X, Y>` into a turbofish like `::<X, Y>`.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemFn`

```rust
struct ForeignItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub semi_token: $crate::token::Semi,
}
```

A foreign function in an `extern` block.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemMacro`

```rust
struct ForeignItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within an extern block.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemStatic`

```rust
struct ForeignItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: $crate::token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: $crate::token::Semi,
}
```

A foreign static item in an `extern` block: `static ext: u8`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemType`

```rust
struct ForeignItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub semi_token: $crate::token::Semi,
}
```

A foreign type in an `extern` block: `type void`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemConst`

```rust
struct ImplItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<$crate::token::Default>,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: $crate::token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: $crate::token::Eq,
    pub expr: crate::expr::Expr,
    pub semi_token: $crate::token::Semi,
}
```

An associated constant within an impl block.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemFn`

```rust
struct ImplItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<$crate::token::Default>,
    pub sig: Signature,
    pub block: crate::stmt::Block,
}
```

An associated function within an impl block.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemMacro`

```rust
struct ImplItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within an impl block.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemType`

```rust
struct ImplItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<$crate::token::Default>,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: $crate::token::Eq,
    pub ty: crate::ty::Type,
    pub semi_token: $crate::token::Semi,
}
```

An associated type within an impl block.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemConst`

```rust
struct ItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: $crate::token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: $crate::token::Semi,
}
```

A constant item: `const MAX: u16 = 65535`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemEnum`

```rust
struct ItemEnum {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub enum_token: $crate::token::Enum,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>,
}
```

An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemExternCrate`

```rust
struct ItemExternCrate {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub extern_token: $crate::token::Extern,
    pub crate_token: $crate::token::Crate,
    pub ident: crate::ident::Ident,
    pub rename: Option<($crate::token::As, crate::ident::Ident)>,
    pub semi_token: $crate::token::Semi,
}
```

An `extern crate` item: `extern crate serde`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemFn`

```rust
struct ItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub block: Box<crate::stmt::Block>,
}
```

A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemForeignMod`

```rust
struct ItemForeignMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub abi: crate::ty::Abi,
    pub brace_token: token::Brace,
    pub items: Vec<ForeignItem>,
}
```

A block of foreign items: `extern "C" { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemImpl`

```rust
struct ItemImpl {
    pub attrs: Vec<crate::attr::Attribute>,
    pub defaultness: Option<$crate::token::Default>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub impl_token: $crate::token::Impl,
    pub generics: crate::generics::Generics,
    pub trait_: Option<(Option<$crate::token::Not>, crate::path::Path, $crate::token::For)>,
    pub self_ty: Box<crate::ty::Type>,
    pub brace_token: token::Brace,
    pub items: Vec<ImplItem>,
}
```

An impl block providing trait or associated items: `impl<A> Trait
for Data<A> { ... }`.

#### Fields

- **`trait_`**: `Option<(Option<$crate::token::Not>, crate::path::Path, $crate::token::For)>`

  Trait this impl implements.

- **`self_ty`**: `Box<crate::ty::Type>`

  The Self type of the impl.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemMacro`

```rust
struct ItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: Option<crate::ident::Ident>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation, which includes `macro_rules!` definitions.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  The `example` in `macro_rules! example { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemMod`

```rust
struct ItemMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub mod_token: $crate::token::Mod,
    pub ident: crate::ident::Ident,
    pub content: Option<(token::Brace, Vec<Item>)>,
    pub semi: Option<$crate::token::Semi>,
}
```

A module or module declaration: `mod m` or `mod m { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemStatic`

```rust
struct ItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: $crate::token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: $crate::token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: $crate::token::Semi,
}
```

A static item: `static BIKE: Shed = Shed(42)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemStruct`

```rust
struct ItemStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub struct_token: $crate::token::Struct,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::Fields,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A struct definition: `struct Foo<A> { x: A }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemTrait`

```rust
struct ItemTrait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub auto_token: Option<$crate::token::Auto>,
    pub restriction: Option<ImplRestriction>,
    pub trait_token: $crate::token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<$crate::token::Colon>,
    pub supertraits: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
    pub brace_token: token::Brace,
    pub items: Vec<TraitItem>,
}
```

A trait definition: `pub trait Iterator { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemTraitAlias`

```rust
struct ItemTraitAlias {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub trait_token: $crate::token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: $crate::token::Eq,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
    pub semi_token: $crate::token::Semi,
}
```

A trait alias: `pub trait SharableIterator = Iterator + Sync`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemType`

```rust
struct ItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: $crate::token::Eq,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: $crate::token::Semi,
}
```

A type alias: `type Result<T> = std::result::Result<T, MyError>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemUnion`

```rust
struct ItemUnion {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub union_token: $crate::token::Union,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::FieldsNamed,
}
```

A union definition: `union Foo<A, B> { x: A, y: B }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ItemUse`

```rust
struct ItemUse {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub use_token: $crate::token::Use,
    pub leading_colon: Option<$crate::token::PathSep>,
    pub tree: UseTree,
    pub semi_token: $crate::token::Semi,
}
```

A use declaration: `use std::collections::HashMap`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Receiver`

```rust
struct Receiver {
    pub attrs: Vec<crate::attr::Attribute>,
    pub reference: Option<($crate::token::And, Option<crate::lifetime::Lifetime>)>,
    pub mutability: Option<$crate::token::Mut>,
    pub self_token: $crate::token::SelfValue,
    pub colon_token: Option<$crate::token::Colon>,
    pub ty: Box<crate::ty::Type>,
}
```

The `self` argument of an associated method.

If `colon_token` is present, the receiver is written with an explicit
type such as `self: Box<Self>`. If `colon_token` is absent, the receiver
is written in shorthand such as `self` or `&self` or `&mut self`. In the
shorthand case, the type in `ty` is reconstructed as one of `Self`,
`&Self`, or `&mut Self`.

#### Implementations

- `fn lifetime(self: &Self) -> Option<&Lifetime>`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Signature`

```rust
struct Signature {
    pub constness: Option<$crate::token::Const>,
    pub asyncness: Option<$crate::token::Async>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub abi: Option<crate::ty::Abi>,
    pub fn_token: $crate::token::Fn,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<FnArg, $crate::token::Comma>,
    pub variadic: Option<Variadic>,
    pub output: crate::ty::ReturnType,
}
```

A function signature in a trait or implementation: `unsafe fn
initialize(&self)`.

#### Implementations

- `fn receiver(self: &Self) -> Option<&Receiver>`
  A method's `self` receiver, such as `&self` or `self: Box<Self>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemConst`

```rust
struct TraitItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: $crate::token::Colon,
    pub ty: crate::ty::Type,
    pub default: Option<($crate::token::Eq, crate::expr::Expr)>,
    pub semi_token: $crate::token::Semi,
}
```

An associated constant within the definition of a trait.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemFn`

```rust
struct TraitItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub sig: Signature,
    pub default: Option<crate::stmt::Block>,
    pub semi_token: Option<$crate::token::Semi>,
}
```

An associated function within the definition of a trait.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemMacro`

```rust
struct TraitItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within the definition of a trait.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemType`

```rust
struct TraitItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<$crate::token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
    pub default: Option<($crate::token::Eq, crate::ty::Type)>,
    pub semi_token: $crate::token::Semi,
}
```

An associated type within the definition of a trait.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `UseGlob`

```rust
struct UseGlob {
    pub star_token: $crate::token::Star,
}
```

A glob import in a `use` item: `*`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `UseGroup`

```rust
struct UseGroup {
    pub brace_token: token::Brace,
    pub items: crate::punctuated::Punctuated<UseTree, $crate::token::Comma>,
}
```

A braced group of imports in a `use` item: `{A, B, C}`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `UseName`

```rust
struct UseName {
    pub ident: crate::ident::Ident,
}
```

An identifier imported by a `use` item: `HashMap`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `UsePath`

```rust
struct UsePath {
    pub ident: crate::ident::Ident,
    pub colon2_token: $crate::token::PathSep,
    pub tree: Box<UseTree>,
}
```

A path prefix of imports in a `use` item: `std::...`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `UseRename`

```rust
struct UseRename {
    pub ident: crate::ident::Ident,
    pub as_token: $crate::token::As,
    pub rename: crate::ident::Ident,
}
```

An renamed identifier imported by a `use` item: `HashMap as Map`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Variadic`

```rust
struct Variadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Option<(Box<crate::pat::Pat>, $crate::token::Colon)>,
    pub dots: $crate::token::DotDotDot,
    pub comma: Option<$crate::token::Comma>,
}
```

The variadic argument of a foreign function.

```rust
# struct c_char;
# struct c_int;
#
extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    //                               ^^^
}
```

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Lifetime`

```rust
struct Lifetime {
    pub apostrophe: proc_macro2::Span,
    pub ident: proc_macro2::Ident,
}
```

A Rust lifetime: `'a`.

Lifetime names must conform to the following rules:

- Must start with an apostrophe.
- Must not consist of just an apostrophe: `'`.
- Character after the apostrophe must be `_` or a Unicode code point with
  the XID_Start property.
- All following characters must be Unicode code points with the XID_Continue
  property.

#### Implementations

- `fn new(symbol: &str, span: Span) -> Self`
  # Panics

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, h: &mut H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Lifetime) -> Ordering`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Lifetime) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Lifetime) -> Option<Ordering>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitBool`

```rust
struct LitBool {
    pub value: bool,
    pub span: proc_macro2::Span,
}
```

A boolean literal: `true` or `false`.

#### Implementations

- `fn new(value: bool, span: Span) -> Self`

- `fn value(self: &Self) -> bool`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn token(self: &Self) -> Ident`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitByte`

```rust
struct LitByte {
    // [REDACTED: Private Fields]
}
```

A byte literal: `b'f'`.

#### Implementations

- `fn new(value: u8, span: Span) -> Self`

- `fn value(self: &Self) -> u8`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn suffix(self: &Self) -> &str`

- `fn token(self: &Self) -> Literal`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitByteStr`

```rust
struct LitByteStr {
    // [REDACTED: Private Fields]
}
```

A byte string literal: `b"foo"`.

#### Implementations

- `fn new(value: &[u8], span: Span) -> Self`

- `fn value(self: &Self) -> Vec<u8>`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn suffix(self: &Self) -> &str`

- `fn token(self: &Self) -> Literal`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitCStr`

```rust
struct LitCStr {
    // [REDACTED: Private Fields]
}
```

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- `fn new(value: &CStr, span: Span) -> Self`

- `fn value(self: &Self) -> CString`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn suffix(self: &Self) -> &str`

- `fn token(self: &Self) -> Literal`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitChar`

```rust
struct LitChar {
    // [REDACTED: Private Fields]
}
```

A character literal: `'a'`.

#### Implementations

- `fn new(value: char, span: Span) -> Self`

- `fn value(self: &Self) -> char`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn suffix(self: &Self) -> &str`

- `fn token(self: &Self) -> Literal`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitFloat`

```rust
struct LitFloat {
    // [REDACTED: Private Fields]
}
```

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- `fn new(repr: &str, span: Span) -> Self`

- `fn base10_digits(self: &Self) -> &str`

- `fn base10_parse<N>(self: &Self) -> Result<N>`

- `fn suffix(self: &Self) -> &str`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn token(self: &Self) -> Literal`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(token: Literal) -> Self`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitInt`

```rust
struct LitInt {
    // [REDACTED: Private Fields]
}
```

An integer literal: `1` or `1u16`.

#### Implementations

- `fn new(repr: &str, span: Span) -> Self`

- `fn base10_digits(self: &Self) -> &str`

- `fn base10_parse<N>(self: &Self) -> Result<N>`
  Parses the literal into a selected number type.

- `fn suffix(self: &Self) -> &str`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn token(self: &Self) -> Literal`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(token: Literal) -> Self`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LitStr`

```rust
struct LitStr {
    // [REDACTED: Private Fields]
}
```

A UTF-8 string literal: `"foo"`.

#### Implementations

- `fn new(value: &str, span: Span) -> Self`

- `fn value(self: &Self) -> String`

- `fn parse<T: Parse>(self: &Self) -> Result<T>`
  Parse a syntax tree node from the content of this string literal.

- `fn parse_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>`
  Invoke parser on the content of this string literal.

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn suffix(self: &Self) -> &str`

- `fn token(self: &Self) -> Literal`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Macro`

```rust
struct Macro {
    pub path: crate::path::Path,
    pub bang_token: $crate::token::Not,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

A macro invocation: `println!("{}", mac)`.

#### Implementations

- `fn parse_body<T: Parse>(self: &Self) -> Result<T>`
  Parse the tokens within the macro invocation's delimiters into a syntax

- `fn parse_body_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>`
  Parse the tokens within the macro invocation's delimiters using the

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `FieldPat`

```rust
struct FieldPat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: crate::expr::Member,
    pub colon_token: Option<$crate::token::Colon>,
    pub pat: Box<Pat>,
}
```

A single field in a struct pattern.

Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatConst`

```rust
struct PatConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatIdent`

```rust
struct PatIdent {
    pub attrs: Vec<crate::attr::Attribute>,
    pub by_ref: Option<$crate::token::Ref>,
    pub mutability: Option<$crate::token::Mut>,
    pub ident: crate::ident::Ident,
    pub subpat: Option<($crate::token::At, Box<Pat>)>,
}
```

A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

It may also be a unit struct or struct variant (e.g. `None`), or a
constant; these cannot be distinguished syntactically.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatLit`

```rust
struct PatLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatMacro`

```rust
struct PatMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatOr`

```rust
struct PatOr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub leading_vert: Option<$crate::token::Or>,
    pub cases: crate::punctuated::Punctuated<Pat, $crate::token::Or>,
}
```

A pattern that matches any one of a set of cases.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatParen`

```rust
struct PatParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub pat: Box<Pat>,
}
```

A parenthesized pattern: `(A | B)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatPath`

```rust
struct PatPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatRange`

```rust
struct PatRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatReference`

```rust
struct PatReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: $crate::token::And,
    pub mutability: Option<$crate::token::Mut>,
    pub pat: Box<Pat>,
}
```

A reference pattern: `&mut var`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatRest`

```rust
struct PatRest {
    pub attrs: Vec<crate::attr::Attribute>,
    pub dot2_token: $crate::token::DotDot,
}
```

The dots in a tuple or slice pattern: `[0, 1, ..]`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatSlice`

```rust
struct PatSlice {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatStruct`

```rust
struct PatStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldPat, $crate::token::Comma>,
    pub rest: Option<PatRest>,
}
```

A struct or struct variant pattern: `Variant { x, y, .. }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatTuple`

```rust
struct PatTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A tuple pattern: `(a, b)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatTupleStruct`

```rust
struct PatTupleStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatType`

```rust
struct PatType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Box<Pat>,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
}
```

A type ascription pattern: `foo: f64`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PatWild`

```rust
struct PatWild {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: $crate::token::Underscore,
}
```

A pattern that matches any value: `_`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `AngleBracketedGenericArguments`

```rust
struct AngleBracketedGenericArguments {
    pub colon2_token: Option<$crate::token::PathSep>,
    pub lt_token: $crate::token::Lt,
    pub args: crate::punctuated::Punctuated<GenericArgument, $crate::token::Comma>,
    pub gt_token: $crate::token::Gt,
}
```

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

#### Implementations

- `fn parse_turbofish(input: ParseStream<'_>) -> Result<Self>`
  Parse `::<…>` with mandatory leading `::`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `AssocConst`

```rust
struct AssocConst {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: $crate::token::Eq,
    pub value: crate::expr::Expr,
}
```

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `AssocType`

```rust
struct AssocType {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: $crate::token::Eq,
    pub ty: crate::ty::Type,
}
```

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Constraint`

```rust
struct Constraint {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub colon_token: $crate::token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
}
```

An associated type bound: `Iterator<Item: Display>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ParenthesizedGenericArguments`

```rust
struct ParenthesizedGenericArguments {
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<crate::ty::Type, $crate::token::Comma>,
    pub output: crate::ty::ReturnType,
}
```

Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
C`.

#### Fields

- **`inputs`**: `crate::punctuated::Punctuated<crate::ty::Type, $crate::token::Comma>`

  `(A, B)`

- **`output`**: `crate::ty::ReturnType`

  `C`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Path`

```rust
struct Path {
    pub leading_colon: Option<$crate::token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, $crate::token::PathSep>,
}
```

A path at which a named item is exported (e.g. `std::collections::HashMap`).

#### Implementations

- `fn is_ident<I>(self: &Self, ident: &I) -> bool`
  Determines whether this is a path of length 1 equal to the given

- `fn get_ident(self: &Self) -> Option<&Ident>`
  If this path consists of a single ident, returns the ident.

- `fn require_ident(self: &Self) -> Result<&Ident>`
  An error if this path is not a single ident, as defined in `get_ident`.

- `fn parse_mod_style(input: ParseStream<'_>) -> Result<Self>`
  Parse a `Path` containing no path arguments on any of its segments.

#### Trait Implementations

##### `impl From<T>`

- `fn from(segment: T) -> Self`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

A segment of a path together with any path arguments on that segment.

#### Trait Implementations

##### `impl From<T>`

- `fn from(ident: T) -> Self`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `QSelf`

```rust
struct QSelf {
    pub lt_token: $crate::token::Lt,
    pub ty: Box<crate::ty::Type>,
    pub position: usize,
    pub as_token: Option<$crate::token::As>,
    pub gt_token: $crate::token::Gt,
}
```

The explicit Self type in a qualified path: the `T` in `<T as
Display>::fmt`.

The actual path, including the trait and the associated item, is stored
separately. The `position` field represents the index of the associated
item qualified with this Self type.

```text
<Vec<T> as a::b::Trait>::AssociatedItem
 ^~~~~~    ~~~~~~~~~~~~~~^
 ty        position = 3

<Vec<T>>::AssociatedItem
 ^~~~~~   ^
 ty       position = 0
```

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned`

- `fn span(self: &Self) -> Span`

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

### `VisRestricted`

```rust
struct VisRestricted {
    pub pub_token: $crate::token::Pub,
    pub paren_token: token::Paren,
    pub in_token: Option<$crate::token::In>,
    pub path: Box<crate::path::Path>,
}
```

A visibility level restricted to some path: `pub(self)` or
`pub(super)` or `pub(crate)` or `pub(in some::module)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Block`

```rust
struct Block {
    pub brace_token: token::Brace,
    pub stmts: Vec<Stmt>,
}
```

A braced block containing Rust statements.

#### Fields

- **`stmts`**: `Vec<Stmt>`

  Statements in a block

#### Implementations

- `fn parse_within(input: ParseStream<'_>) -> Result<Vec<Stmt>>`
  Parse the body of a block as zero or more statements, possibly

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Local`

```rust
struct Local {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: $crate::token::Let,
    pub pat: crate::pat::Pat,
    pub init: Option<LocalInit>,
    pub semi_token: $crate::token::Semi,
}
```

A local `let` binding: `let x: u64 = s.parse()?;`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LocalInit`

```rust
struct LocalInit {
    pub eq_token: $crate::token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub diverge: Option<($crate::token::Else, Box<crate::expr::Expr>)>,
}
```

The expression assigned in a local `let` binding, including optional
diverging `else` block.

`LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and
`= r else { return }` in `let Ok(x) = r else { return }`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `StmtMacro`

```rust
struct StmtMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation in statement position.

Syntactically it's ambiguous which other kind of statement this macro
would expand to. It can be any of local variable (`let`), item, or
expression.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Abi`

```rust
struct Abi {
    pub extern_token: $crate::token::Extern,
    pub name: Option<crate::lit::LitStr>,
}
```

The binary interface of a function: `extern "C"`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `BareFnArg`

```rust
struct BareFnArg {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, $crate::token::Colon)>,
    pub ty: Type,
}
```

An argument in a function type: the `usize` in `fn(usize) -> bool`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `BareVariadic`

```rust
struct BareVariadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, $crate::token::Colon)>,
    pub dots: $crate::token::DotDotDot,
    pub comma: Option<$crate::token::Comma>,
}
```

The variadic argument of a function pointer like `fn(usize, ...)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeArray`

```rust
struct TypeArray {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
    pub semi_token: $crate::token::Semi,
    pub len: crate::expr::Expr,
}
```

A fixed size array type: `[T; n]`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeBareFn`

```rust
struct TypeBareFn {
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub abi: Option<Abi>,
    pub fn_token: $crate::token::Fn,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<BareFnArg, $crate::token::Comma>,
    pub variadic: Option<BareVariadic>,
    pub output: ReturnType,
}
```

A bare function type: `fn(usize) -> bool`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeGroup`

```rust
struct TypeGroup {
    pub group_token: token::Group,
    pub elem: Box<Type>,
}
```

A type contained within invisible delimiters.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeImplTrait`

```rust
struct TypeImplTrait {
    pub impl_token: $crate::token::Impl,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
}
```

An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
a lifetime.

#### Implementations

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeInfer`

```rust
struct TypeInfer {
    pub underscore_token: $crate::token::Underscore,
}
```

Indication that a type should be inferred by the compiler: `_`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeMacro`

```rust
struct TypeMacro {
    pub mac: crate::mac::Macro,
}
```

A macro in the type position.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeNever`

```rust
struct TypeNever {
    pub bang_token: $crate::token::Not,
}
```

The never type: `!`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParen`

```rust
struct TypeParen {
    pub paren_token: token::Paren,
    pub elem: Box<Type>,
}
```

A parenthesized type equivalent to the inner type.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypePath`

```rust
struct TypePath {
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::slice::Iter`, optionally qualified with a
self-type as in `<Vec<T> as SomeTrait>::Associated`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypePtr`

```rust
struct TypePtr {
    pub star_token: $crate::token::Star,
    pub const_token: Option<$crate::token::Const>,
    pub mutability: Option<$crate::token::Mut>,
    pub elem: Box<Type>,
}
```

A raw pointer type: `*const T` or `*mut T`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeReference`

```rust
struct TypeReference {
    pub and_token: $crate::token::And,
    pub lifetime: Option<crate::lifetime::Lifetime>,
    pub mutability: Option<$crate::token::Mut>,
    pub elem: Box<Type>,
}
```

A reference type: `&'a T` or `&'a mut T`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeSlice`

```rust
struct TypeSlice {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
}
```

A dynamically sized slice type: `[T]`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeTraitObject`

```rust
struct TypeTraitObject {
    pub dyn_token: Option<$crate::token::Dyn>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
}
```

A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
trait or a lifetime.

#### Implementations

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeTuple`

```rust
struct TypeTuple {
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Type, $crate::token::Comma>,
}
```

A tuple type: `(A, B, C, String)`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `AttrStyle`

```rust
enum AttrStyle {
    Outer,
    Inner($crate::token::Not),
}
```

Distinguishes between attributes that decorate an item and attributes
that are contained within an item.

# Outer attributes

- `#[repr(transparent)]`
- `/// # Example`
- `/** Please file an issue */`

# Inner attributes

- `#![feature(proc_macro)]`
- `//! # Example`
- `/*! Please file an issue */`

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

- `fn clone(self: &Self) -> Self`

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

### `Meta`

```rust
enum Meta {
    Path(crate::path::Path),
    List(MetaList),
    NameValue(MetaNameValue),
}
```

Content of a compile-time structured attribute.

## Path

A meta path is like the `test` in `#[test](#test)
`.

## List

A meta list is like the `derive(Copy)` in `#[derive(Copy)]`.

## NameValue

A name-value meta is like the `path = "..."` in `#[path =
"sys/windows.rs"]`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`List`**

  A structured list within an attribute, like `derive(Copy, Clone)`.

- **`NameValue`**

  A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- `fn path(self: &Self) -> &Path`
  Returns the path that begins this structured meta item.

- `fn require_path_only(self: &Self) -> Result<&Path>`
  Error if this is a `Meta::List` or `Meta::NameValue`.

- `fn require_list(self: &Self) -> Result<&MetaList>`
  Error if this is a `Meta::Path` or `Meta::NameValue`.

- `fn require_name_value(self: &Self) -> Result<&MetaNameValue>`
  Error if this is a `Meta::Path` or `Meta::List`.

#### Trait Implementations

##### `impl From`

- `fn from(meta: MetaNameValue) -> Meta`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(meta: Path) -> Meta`

##### `impl From`

- `fn from(meta: MetaList) -> Meta`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Fields`

```rust
enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
```

Data stored within an enum variant or struct.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Named`**

  Named fields of a struct or struct variant such as `Point { x: f64,
  y: f64 }`.

- **`Unnamed`**

  Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

- **`Unit`**

  Unit struct or unit variant such as `None`.

#### Implementations

- `fn iter(self: &Self) -> punctuated::Iter<'_, Field>`
  Get an iterator over the borrowed [`Field`] items in this object. This

- `fn iter_mut(self: &mut Self) -> punctuated::IterMut<'_, Field>`
  Get an iterator over the mutably borrowed [`Field`] items in this

- `fn len(self: &Self) -> usize`
  Returns the number of fields.

- `fn is_empty(self: &Self) -> bool`
  Returns `true` if there are zero fields.

- `fn members(self: &Self) -> Members<'_>`
  Get an iterator over the fields of a struct or variant as [`Member`]s.

#### Trait Implementations

##### `impl From`

- `fn from(e: FieldsNamed) -> Fields`

##### `impl From`

- `fn from(e: FieldsUnnamed) -> Fields`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator`

- `type Item = Field`

- `type IntoIter = IntoIter<Field>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Data`

```rust
enum Data {
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}
```

The storage of a struct, enum or union data structure.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `PointerMutability`

```rust
enum PointerMutability {
    Const($crate::token::Const),
    Mut($crate::token::Mut),
}
```

Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
isn't the implicit default.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `RangeLimits`

```rust
enum RangeLimits {
    HalfOpen($crate::token::DotDot),
    Closed($crate::token::DotDotEq),
}
```

Limit types of a range, inclusive or exclusive.

#### Variants

- **`HalfOpen`**

  Inclusive at the beginning, exclusive at the end.

- **`Closed`**

  Inclusive at the beginning and end.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Expr`

```rust
enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    RawAddr(ExprRawAddr),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(proc_macro2::TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
}
```

A Rust expression.

*This type is available only if Syn is built with the `"derive"` or `"full"`
feature, but most of the variants are not available unless "full" is enabled.*

# Syntax tree enums

This type is a syntax tree enum. In Syn this and other syntax tree enums
are designed to be traversed using the following rebinding idiom.

```
# use syn::Expr;
#
# fn example(expr: Expr) {
# const IGNORE: &str = stringify! {
let expr: Expr = /* ... */;
# };
match expr {
    Expr::MethodCall(expr) => {
        /* ... */
    }
    Expr::Cast(expr) => {
        /* ... */
    }
    Expr::If(expr) => {
        /* ... */
    }

    /* ... */
    # _ => {}
# }
# }
```

We begin with a variable `expr` of type `Expr` that has no fields
(because it is an enum), and by matching on it and rebinding a variable
with the same name `expr` we effectively imbue our variable with all of
the data fields provided by the variant that it turned out to be. So for
example above if we ended up in the `MethodCall` case then we get to use
`expr.receiver`, `expr.args` etc; if we ended up in the `If` case we get
to use `expr.cond`, `expr.then_branch`, `expr.else_branch`.

This approach avoids repeating the variant names twice on every line.

```
# use syn::{Expr, ExprMethodCall};
#
# fn example(expr: Expr) {
// Repetitive; recommend not doing this.
match expr {
    Expr::MethodCall(ExprMethodCall { method, args, .. }) => {
# }
# _ => {}
# }
# }
```

In general, the name to which a syntax tree enum variant is bound should
be a suitable name for the complete syntax tree enum type.

```
# use syn::{Expr, ExprField};
#
# fn example(discriminant: ExprField) {
// Binding is called `base` which is the name I would use if I were
// assigning `*discriminant.base` without an `if let`.
if let Expr::Tuple(base) = *discriminant.base {
# }
# }
```

A sign that you may not be choosing the right variable names is if you
see names getting repeated in your code, like accessing
`receiver.receiver` or `pat.pat` or `cond.cond`.

#### Variants

- **`Array`**

  A slice literal expression: `[a, b, c, d]`.

- **`Assign`**

  An assignment expression: `a = compute()`.

- **`Async`**

  An async block: `async { ... }`.

- **`Await`**

  An await expression: `fut.await`.

- **`Binary`**

  A binary operation: `a + b`, `a += b`.

- **`Block`**

  A blocked scope: `{ ... }`.

- **`Break`**

  A `break`, with an optional label to break and an optional
  expression.

- **`Call`**

  A function call expression: `invoke(a, b)`.

- **`Cast`**

  A cast expression: `foo as f64`.

- **`Closure`**

  A closure expression: `|a, b| a + b`.

- **`Const`**

  A const block: `const { ... }`.

- **`Continue`**

  A `continue`, with an optional label.

- **`Field`**

  Access of a named struct field (`obj.k`) or unnamed tuple struct
  field (`obj.0`).

- **`ForLoop`**

  A for loop: `for pat in expr { ... }`.

- **`Group`**

  An expression contained within invisible delimiters.
  
  This variant is important for faithfully representing the precedence
  of expressions and is related to `None`-delimited spans in a
  `TokenStream`.

- **`If`**

  An `if` expression with an optional `else` block: `if expr { ... }
  else { ... }`.
  
  The `else` branch expression may only be an `If` or `Block`
  expression, not any of the other types of expression.

- **`Index`**

  A square bracketed indexing expression: `vector[2]`.

- **`Infer`**

  The inferred value of a const generic argument, denoted `_`.

- **`Let`**

  A `let` guard: `let Some(x) = opt`.

- **`Lit`**

  A literal in place of an expression: `1`, `"foo"`.

- **`Loop`**

  Conditionless loop: `loop { ... }`.

- **`Macro`**

  A macro invocation expression: `format!("{}", q)`.

- **`Match`**

  A `match` expression: `match n { Some(n) => {}, None => {} }`.

- **`MethodCall`**

  A method call expression: `x.foo::<T>(a, b)`.

- **`Paren`**

  A parenthesized expression: `(a + b)`.

- **`Path`**

  A path like `std::mem::replace` possibly containing generic
  parameters and a qualified self-type.
  
  A plain identifier like `x` is a path of length 1.

- **`Range`**

  A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

- **`RawAddr`**

  Address-of operation: `&raw const place` or `&raw mut place`.

- **`Reference`**

  A referencing operation: `&a` or `&mut a`.

- **`Repeat`**

  An array literal constructed from one repeated element: `[0u8; N]`.

- **`Return`**

  A `return`, with an optional value to be returned.

- **`Struct`**

  A struct literal expression: `Point { x: 1, y: 1 }`.
  
  The `rest` provides the value of the remaining fields as in `S { a:
  1, b: 1, ..rest }`.

- **`Try`**

  A try-expression: `expr?`.

- **`TryBlock`**

  A try block: `try { ... }`.

- **`Tuple`**

  A tuple expression: `(a, b, c, d)`.

- **`Unary`**

  A unary operation: `!x`, `*x`.

- **`Unsafe`**

  An unsafe block: `unsafe { ... }`.

- **`Verbatim`**

  Tokens in expression position not interpreted by Syn.

- **`While`**

  A while loop: `while expr { ... }`.

- **`Yield`**

  A yield expression: `yield expr`.

#### Implementations

- `const PLACEHOLDER: Self`

- `fn parse_without_eager_brace(input: ParseStream<'_>) -> Result<Expr>`
  An alternative to the primary `Expr::parse` parser (from the [`Parse`]

- `fn parse_with_earlier_boundary_rule(input: ParseStream<'_>) -> Result<Expr>`
  An alternative to the primary `Expr::parse` parser (from the [`Parse`]

- `fn peek(input: ParseStream<'_>) -> bool`
  Returns whether the next token in the parse stream is one that might

#### Trait Implementations

##### `impl From`

- `fn from(e: ExprLoop) -> Expr`

##### `impl From`

- `fn from(e: ExprTry) -> Expr`

##### `impl From`

- `fn from(e: ExprBinary) -> Expr`

##### `impl From`

- `fn from(e: ExprRange) -> Expr`

##### `impl From`

- `fn from(e: ExprYield) -> Expr`

##### `impl From`

- `fn from(e: ExprStruct) -> Expr`

##### `impl From`

- `fn from(e: ExprRepeat) -> Expr`

##### `impl From`

- `fn from(e: ExprAwait) -> Expr`

##### `impl From`

- `fn from(e: ExprPath) -> Expr`

##### `impl From`

- `fn from(e: ExprMatch) -> Expr`

##### `impl From`

- `fn from(e: ExprRawAddr) -> Expr`

##### `impl From`

- `fn from(e: ExprIndex) -> Expr`

##### `impl From`

- `fn from(e: ExprField) -> Expr`

##### `impl From`

- `fn from(e: ExprAssign) -> Expr`

##### `impl From`

- `fn from(e: ExprCast) -> Expr`

##### `impl From`

- `fn from(e: ExprContinue) -> Expr`

##### `impl From`

- `fn from(e: ExprReturn) -> Expr`

##### `impl From`

- `fn from(e: ExprBreak) -> Expr`

##### `impl From`

- `fn from(e: ExprTuple) -> Expr`

##### `impl From`

- `fn from(e: ExprClosure) -> Expr`

##### `impl From`

- `fn from(e: ExprConst) -> Expr`

##### `impl From`

- `fn from(e: ExprUnary) -> Expr`

##### `impl From`

- `fn from(e: ExprWhile) -> Expr`

##### `impl From`

- `fn from(e: ExprReference) -> Expr`

##### `impl From`

- `fn from(e: ExprForLoop) -> Expr`

##### `impl From`

- `fn from(e: ExprUnsafe) -> Expr`

##### `impl From`

- `fn from(e: ExprArray) -> Expr`

##### `impl From`

- `fn from(e: ExprMacro) -> Expr`

##### `impl From`

- `fn from(e: ExprAsync) -> Expr`

##### `impl From`

- `fn from(e: ExprMethodCall) -> Expr`

##### `impl From`

- `fn from(e: ExprInfer) -> Expr`

##### `impl From`

- `fn from(e: ExprBlock) -> Expr`

##### `impl From`

- `fn from(e: ExprLet) -> Expr`

##### `impl From`

- `fn from(e: ExprGroup) -> Expr`

##### `impl From`

- `fn from(e: ExprTryBlock) -> Expr`

##### `impl From`

- `fn from(e: ExprCall) -> Expr`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: ExprParen) -> Expr`

##### `impl From`

- `fn from(e: ExprLit) -> Expr`

##### `impl From`

- `fn from(e: ExprIf) -> Expr`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Member`

```rust
enum Member {
    Named(crate::ident::Ident),
    Unnamed(Index),
}
```

A struct or tuple struct field accessed in a struct literal or field
expression.

#### Variants

- **`Named`**

  A named field like `self.x`.

- **`Unnamed`**

  An unnamed field like `self.0`.

#### Trait Implementations

##### `impl From`

- `fn from(index: Index) -> Member`

##### `impl From`

- `fn from(ident: Ident) -> Member`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(index: usize) -> Member`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl IdentFragment`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn span(self: &Self) -> Option<Span>`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `GenericParam`

```rust
enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}
```

A generic type parameter, lifetime, or const generic: `T: Into<String>`,
`'a: 'b`, `const LEN: usize`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Lifetime`**

  A lifetime parameter: `'a: 'b + 'c + 'd`.

- **`Type`**

  A generic type parameter: `T: Into<String>`.

- **`Const`**

  A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl From`

- `fn from(e: TypeParam) -> GenericParam`

##### `impl From`

- `fn from(e: LifetimeParam) -> GenericParam`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: ConstParam) -> GenericParam`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe($crate::token::Question),
}
```

A modifier on a trait bound, currently only used for the `?` in
`?Sized`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParamBound`

```rust
enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(crate::lifetime::Lifetime),
    PreciseCapture(PreciseCapture),
    Verbatim(proc_macro2::TokenStream),
}
```

A trait or lifetime used as a bound on a type parameter.

#### Trait Implementations

##### `impl From`

- `fn from(e: Lifetime) -> TypeParamBound`

##### `impl From`

- `fn from(e: TraitBound) -> TypeParamBound`

##### `impl From`

- `fn from(e: PreciseCapture) -> TypeParamBound`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `WherePredicate`

```rust
enum WherePredicate {
    Lifetime(PredicateLifetime),
    Type(PredicateType),
}
```

A single predicate in a `where` clause: `T: Deserialize<'de>`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Lifetime`**

  A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

- **`Type`**

  A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Trait Implementations

##### `impl From`

- `fn from(e: PredicateLifetime) -> WherePredicate`

##### `impl From`

- `fn from(e: PredicateType) -> WherePredicate`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `CapturedParam`

```rust
enum CapturedParam {
    Lifetime(crate::lifetime::Lifetime),
    Ident(crate::ident::Ident),
}
```

Single parameter in a precise capturing bound.

#### Variants

- **`Lifetime`**

  A lifetime parameter in precise capturing bound: `fn f<'a>() -> impl
  Trait + use<'a>`.

- **`Ident`**

  A type parameter or const generic parameter in precise capturing
  bound: `fn f<T>() -> impl Trait + use<T>` or `fn f<const K: T>() ->
  impl Trait + use<K>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `FnArg`

```rust
enum FnArg {
    Receiver(Receiver),
    Typed(crate::pat::PatType),
}
```

An argument in a function signature: the `n: usize` in `fn f(n: usize)`.

#### Variants

- **`Receiver`**

  The `self` argument of an associated method.

- **`Typed`**

  A function argument accepted by pattern and type.

#### Trait Implementations

##### `impl From`

- `fn from(e: PatType) -> FnArg`

##### `impl From`

- `fn from(e: Receiver) -> FnArg`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItem`

```rust
enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an `extern` block.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Fn`**

  A foreign function in an `extern` block.

- **`Static`**

  A foreign static item in an `extern` block: `static ext: u8`.

- **`Type`**

  A foreign type in an `extern` block: `type void`.

- **`Macro`**

  A macro invocation within an extern block.

- **`Verbatim`**

  Tokens in an `extern` block not interpreted by Syn.

#### Trait Implementations

##### `impl From`

- `fn from(e: ForeignItemStatic) -> ForeignItem`

##### `impl From`

- `fn from(e: ForeignItemFn) -> ForeignItem`

##### `impl From`

- `fn from(e: ForeignItemMacro) -> ForeignItem`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: ForeignItemType) -> ForeignItem`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItem`

```rust
enum ImplItem {
    Const(ImplItemConst),
    Fn(ImplItemFn),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an impl block.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Const`**

  An associated constant within an impl block.

- **`Fn`**

  An associated function within an impl block.

- **`Type`**

  An associated type within an impl block.

- **`Macro`**

  A macro invocation within an impl block.

- **`Verbatim`**

  Tokens within an impl block not interpreted by Syn.

#### Trait Implementations

##### `impl From`

- `fn from(e: ImplItemConst) -> ImplItem`

##### `impl From`

- `fn from(e: ImplItemType) -> ImplItem`

##### `impl From`

- `fn from(e: ImplItemMacro) -> ImplItem`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: ImplItemFn) -> ImplItem`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ImplRestriction`

```rust
enum ImplRestriction {
}
```

Unused, but reserved for RFC 3323 restrictions.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `Item`

```rust
enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(proc_macro2::TokenStream),
}
```

Things that can appear directly inside of a module or scope.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Const`**

  A constant item: `const MAX: u16 = 65535`.

- **`Enum`**

  An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

- **`ExternCrate`**

  An `extern crate` item: `extern crate serde`.

- **`Fn`**

  A free-standing function: `fn process(n: usize) -> Result<()> { ...
  }`.

- **`ForeignMod`**

  A block of foreign items: `extern "C" { ... }`.

- **`Impl`**

  An impl block providing trait or associated items: `impl<A> Trait
  for Data<A> { ... }`.

- **`Macro`**

  A macro invocation, which includes `macro_rules!` definitions.

- **`Mod`**

  A module or module declaration: `mod m` or `mod m { ... }`.

- **`Static`**

  A static item: `static BIKE: Shed = Shed(42)`.

- **`Struct`**

  A struct definition: `struct Foo<A> { x: A }`.

- **`Trait`**

  A trait definition: `pub trait Iterator { ... }`.

- **`TraitAlias`**

  A trait alias: `pub trait SharableIterator = Iterator + Sync`.

- **`Type`**

  A type alias: `type Result<T> = std::result::Result<T, MyError>`.

- **`Union`**

  A union definition: `union Foo<A, B> { x: A, y: B }`.

- **`Use`**

  A use declaration: `use std::collections::HashMap`.

- **`Verbatim`**

  Tokens forming an item not interpreted by Syn.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: ItemUnion) -> Item`

##### `impl From`

- `fn from(e: ItemStatic) -> Item`

##### `impl From`

- `fn from(input: DeriveInput) -> Item`

##### `impl From`

- `fn from(e: ItemEnum) -> Item`

##### `impl From`

- `fn from(e: ItemExternCrate) -> Item`

##### `impl From`

- `fn from(e: ItemTrait) -> Item`

##### `impl From`

- `fn from(e: ItemImpl) -> Item`

##### `impl From`

- `fn from(e: ItemForeignMod) -> Item`

##### `impl From`

- `fn from(e: ItemMacro) -> Item`

##### `impl From`

- `fn from(e: ItemMod) -> Item`

##### `impl From`

- `fn from(e: ItemTraitAlias) -> Item`

##### `impl From`

- `fn from(e: ItemConst) -> Item`

##### `impl From`

- `fn from(e: ItemStruct) -> Item`

##### `impl From`

- `fn from(e: ItemFn) -> Item`

##### `impl From`

- `fn from(e: ItemUse) -> Item`

##### `impl From`

- `fn from(e: ItemType) -> Item`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `StaticMutability`

```rust
enum StaticMutability {
    Mut($crate::token::Mut),
    None,
}
```

The mutability of an `Item::Static` or `ForeignItem::Static`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItem`

```rust
enum TraitItem {
    Const(TraitItemConst),
    Fn(TraitItemFn),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item declaration within the definition of a trait.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Const`**

  An associated constant within the definition of a trait.

- **`Fn`**

  An associated function within the definition of a trait.

- **`Type`**

  An associated type within the definition of a trait.

- **`Macro`**

  A macro invocation within the definition of a trait.

- **`Verbatim`**

  Tokens within the definition of a trait not interpreted by Syn.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: TraitItemType) -> TraitItem`

##### `impl From`

- `fn from(e: TraitItemMacro) -> TraitItem`

##### `impl From`

- `fn from(e: TraitItemConst) -> TraitItem`

##### `impl From`

- `fn from(e: TraitItemFn) -> TraitItem`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `UseTree`

```rust
enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}
```

A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Path`**

  A path prefix of imports in a `use` item: `std::...`.

- **`Name`**

  An identifier imported by a `use` item: `HashMap`.

- **`Rename`**

  An renamed identifier imported by a `use` item: `HashMap as Map`.

- **`Glob`**

  A glob import in a `use` item: `*`.

- **`Group`**

  A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl From`

- `fn from(e: UseGroup) -> UseTree`

##### `impl From`

- `fn from(e: UseRename) -> UseTree`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: UsePath) -> UseTree`

##### `impl From`

- `fn from(e: UseGlob) -> UseTree`

##### `impl From`

- `fn from(e: UseName) -> UseTree`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<UseTree>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Lit`

```rust
enum Lit {
    Str(LitStr),
    ByteStr(LitByteStr),
    CStr(LitCStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Verbatim(proc_macro2::Literal),
}
```

A Rust literal such as a string or integer or boolean.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Str`**

  A UTF-8 string literal: `"foo"`.

- **`ByteStr`**

  A byte string literal: `b"foo"`.

- **`CStr`**

  A nul-terminated C-string literal: `c"foo"`.

- **`Byte`**

  A byte literal: `b'f'`.

- **`Char`**

  A character literal: `'a'`.

- **`Int`**

  An integer literal: `1` or `1u16`.

- **`Float`**

  A floating point literal: `1f64` or `1.0e10f64`.
  
  Must be finite. May not be infinite or NaN.

- **`Bool`**

  A boolean literal: `true` or `false`.

- **`Verbatim`**

  A raw token literal not interpreted by Syn.

#### Implementations

- `fn new(token: Literal) -> Self`
  Interpret a Syn literal from a proc-macro2 literal.

- `fn suffix(self: &Self) -> &str`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

#### Trait Implementations

##### `impl From`

- `fn from(e: LitFloat) -> Lit`

##### `impl From`

- `fn from(e: LitStr) -> Lit`

##### `impl From`

- `fn from(e: LitBool) -> Lit`

##### `impl From`

- `fn from(e: LitByte) -> Lit`

##### `impl From`

- `fn from(e: LitCStr) -> Lit`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: LitByteStr) -> Lit`

##### `impl From`

- `fn from(e: LitChar) -> Lit`

##### `impl From`

- `fn from(e: LitInt) -> Lit`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `MacroDelimiter`

```rust
enum MacroDelimiter {
    Paren(crate::token::Paren),
    Brace(crate::token::Brace),
    Bracket(crate::token::Bracket),
}
```

A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.

#### Implementations

- `fn span(self: &Self) -> &DelimSpan`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `BinOp`

```rust
enum BinOp {
    Add($crate::token::Plus),
    Sub($crate::token::Minus),
    Mul($crate::token::Star),
    Div($crate::token::Slash),
    Rem($crate::token::Percent),
    And($crate::token::AndAnd),
    Or($crate::token::OrOr),
    BitXor($crate::token::Caret),
    BitAnd($crate::token::And),
    BitOr($crate::token::Or),
    Shl($crate::token::Shl),
    Shr($crate::token::Shr),
    Eq($crate::token::EqEq),
    Lt($crate::token::Lt),
    Le($crate::token::Le),
    Ne($crate::token::Ne),
    Ge($crate::token::Ge),
    Gt($crate::token::Gt),
    AddAssign($crate::token::PlusEq),
    SubAssign($crate::token::MinusEq),
    MulAssign($crate::token::StarEq),
    DivAssign($crate::token::SlashEq),
    RemAssign($crate::token::PercentEq),
    BitXorAssign($crate::token::CaretEq),
    BitAndAssign($crate::token::AndEq),
    BitOrAssign($crate::token::OrEq),
    ShlAssign($crate::token::ShlEq),
    ShrAssign($crate::token::ShrEq),
}
```

A binary operator: `+`, `+=`, `&`.

#### Variants

- **`Add`**

  The `+` operator (addition)

- **`Sub`**

  The `-` operator (subtraction)

- **`Mul`**

  The `*` operator (multiplication)

- **`Div`**

  The `/` operator (division)

- **`Rem`**

  The `%` operator (modulus)

- **`And`**

  The `&&` operator (logical and)

- **`Or`**

  The `||` operator (logical or)

- **`BitXor`**

  The `^` operator (bitwise xor)

- **`BitAnd`**

  The `&` operator (bitwise and)

- **`BitOr`**

  The `|` operator (bitwise or)

- **`Shl`**

  The `<<` operator (shift left)

- **`Shr`**

  The `>>` operator (shift right)

- **`Eq`**

  The `==` operator (equality)

- **`Lt`**

  The `<` operator (less than)

- **`Le`**

  The `<=` operator (less than or equal to)

- **`Ne`**

  The `!=` operator (not equal to)

- **`Ge`**

  The `>=` operator (greater than or equal to)

- **`Gt`**

  The `>` operator (greater than)

- **`AddAssign`**

  The `+=` operator

- **`SubAssign`**

  The `-=` operator

- **`MulAssign`**

  The `*=` operator

- **`DivAssign`**

  The `/=` operator

- **`RemAssign`**

  The `%=` operator

- **`BitXorAssign`**

  The `^=` operator

- **`BitAndAssign`**

  The `&=` operator

- **`BitOrAssign`**

  The `|=` operator

- **`ShlAssign`**

  The `<<=` operator

- **`ShrAssign`**

  The `>>=` operator

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `UnOp`

```rust
enum UnOp {
    Deref($crate::token::Star),
    Not($crate::token::Not),
    Neg($crate::token::Minus),
}
```

A unary operator: `*`, `!`, `-`.

#### Variants

- **`Deref`**

  The `*` operator for dereferencing

- **`Not`**

  The `!` operator for logical inversion

- **`Neg`**

  The `-` operator for negation

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Pat`

```rust
enum Pat {
    Const(PatConst),
    Ident(PatIdent),
    Lit(PatLit),
    Macro(PatMacro),
    Or(PatOr),
    Paren(PatParen),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Rest(PatRest),
    Slice(PatSlice),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Type(PatType),
    Verbatim(proc_macro2::TokenStream),
    Wild(PatWild),
}
```

A pattern in a local binding, function signature, match expression, or
various other places.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Const`**

  A const block: `const { ... }`.

- **`Ident`**

  A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

- **`Lit`**

  A literal pattern: `0`.

- **`Macro`**

  A macro in pattern position.

- **`Or`**

  A pattern that matches any one of a set of cases.

- **`Paren`**

  A parenthesized pattern: `(A | B)`.

- **`Path`**

  A path pattern like `Color::Red`, optionally qualified with a
  self-type.
  
  Unqualified path patterns can legally refer to variants, structs,
  constants or associated constants. Qualified path patterns like
  `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
  associated constants.

- **`Range`**

  A range pattern: `1..=2`.

- **`Reference`**

  A reference pattern: `&mut var`.

- **`Rest`**

  The dots in a tuple or slice pattern: `[0, 1, ..]`.

- **`Slice`**

  A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

- **`Struct`**

  A struct or struct variant pattern: `Variant { x, y, .. }`.

- **`Tuple`**

  A tuple pattern: `(a, b)`.

- **`TupleStruct`**

  A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

- **`Type`**

  A type ascription pattern: `foo: f64`.

- **`Verbatim`**

  Tokens in pattern position not interpreted by Syn.

- **`Wild`**

  A pattern that matches any value: `_`.

#### Implementations

- `fn parse_single(input: ParseStream<'_>) -> Result<Self>`
  Parse a pattern that does _not_ involve `|` at the top level.

- `fn parse_multi(input: ParseStream<'_>) -> Result<Self>`
  Parse a pattern, possibly involving `|`, but not a leading `|`.

- `fn parse_multi_with_leading_vert(input: ParseStream<'_>) -> Result<Self>`
  Parse a pattern, possibly involving `|`, possibly including a

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: PatTupleStruct) -> Pat`

##### `impl From`

- `fn from(e: PatSlice) -> Pat`

##### `impl From`

- `fn from(e: PatIdent) -> Pat`

##### `impl From`

- `fn from(e: PatOr) -> Pat`

##### `impl From`

- `fn from(e: PatRange) -> Pat`

##### `impl From`

- `fn from(e: PatRest) -> Pat`

##### `impl From`

- `fn from(e: PatLit) -> Pat`

##### `impl From`

- `fn from(e: PatParen) -> Pat`

##### `impl From`

- `fn from(e: PatPath) -> Pat`

##### `impl From`

- `fn from(e: PatReference) -> Pat`

##### `impl From`

- `fn from(e: PatStruct) -> Pat`

##### `impl From`

- `fn from(e: PatTuple) -> Pat`

##### `impl From`

- `fn from(e: PatConst) -> Pat`

##### `impl From`

- `fn from(e: PatType) -> Pat`

##### `impl From`

- `fn from(e: PatWild) -> Pat`

##### `impl From`

- `fn from(e: PatMacro) -> Pat`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `GenericArgument`

```rust
enum GenericArgument {
    Lifetime(crate::lifetime::Lifetime),
    Type(crate::ty::Type),
    Const(crate::expr::Expr),
    AssocType(AssocType),
    AssocConst(AssocConst),
    Constraint(Constraint),
}
```

An individual generic argument, like `'a`, `T`, or `Item = T`.

#### Variants

- **`Lifetime`**

  A lifetime argument.

- **`Type`**

  A type argument.

- **`Const`**

  A const expression. Must be inside of a block.
  
  NOTE: Identity expressions are represented as Type arguments, as
  they are indistinguishable syntactically.

- **`AssocType`**

  A binding (equality constraint) on an associated type: the `Item =
  u8` in `Iterator<Item = u8>`.

- **`AssocConst`**

  An equality constraint on an associated constant: the `PANIC =
  false` in `Trait<PANIC = false>`.

- **`Constraint`**

  An associated type bound: `Iterator<Item: Display>`.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PathArguments`

```rust
enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
```

Angle bracketed or parenthesized arguments of a path segment.

## Angle bracketed

The `<'a, T>` in `std::slice::iter<'a, T>`.

## Parenthesized

The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Variants

- **`AngleBracketed`**

  The `<'a, T>` in `std::slice::iter<'a, T>`.

- **`Parenthesized`**

  The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Implementations

- `fn is_empty(self: &Self) -> bool`

- `fn is_none(self: &Self) -> bool`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default`

- `fn default() -> Self`

### `FieldMutability`

```rust
enum FieldMutability {
    None,
}
```

Unused, but reserved for RFC 3323 restrictions.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `Visibility`

```rust
enum Visibility {
    Public($crate::token::Pub),
    Restricted(VisRestricted),
    Inherited,
}
```

The visibility level of an item: inherited or `pub` or
`pub(restricted)`.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Public`**

  A public visibility level: `pub`.

- **`Restricted`**

  A visibility level restricted to some path: `pub(self)` or
  `pub(super)` or `pub(crate)` or `pub(in some::module)`.

- **`Inherited`**

  An inherited visibility, which usually means private.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Stmt`

```rust
enum Stmt {
    Local(Local),
    Item(crate::item::Item),
    Expr(crate::expr::Expr, Option<$crate::token::Semi>),
    Macro(StmtMacro),
}
```

A statement, usually ending in a semicolon.

#### Variants

- **`Local`**

  A local (let) binding.

- **`Item`**

  An item definition.

- **`Expr`**

  Expression, with or without trailing semicolon.

- **`Macro`**

  A macro invocation in statement position.
  
  Syntactically it's ambiguous which other kind of statement this
  macro would expand to. It can be any of local variable (`let`),
  item, or expression.

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ReturnType`

```rust
enum ReturnType {
    Default,
    Type($crate::token::RArrow, Box<Type>),
}
```

Return type of a function signature.

#### Variants

- **`Default`**

  Return type is not specified.
  
  Functions default to `()` and closures default to type inference.

- **`Type`**

  A particular type is returned.

#### Implementations

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Type`

```rust
enum Type {
    Array(TypeArray),
    BareFn(TypeBareFn),
    Group(TypeGroup),
    ImplTrait(TypeImplTrait),
    Infer(TypeInfer),
    Macro(TypeMacro),
    Never(TypeNever),
    Paren(TypeParen),
    Path(TypePath),
    Ptr(TypePtr),
    Reference(TypeReference),
    Slice(TypeSlice),
    TraitObject(TypeTraitObject),
    Tuple(TypeTuple),
    Verbatim(proc_macro2::TokenStream),
}
```

The possible types that a Rust value could have.

# Syntax tree enum

This type is a [syntax tree enum].

[syntax tree enum]: crate::expr::Expr#syntax-tree-enums

#### Variants

- **`Array`**

  A fixed size array type: `[T; n]`.

- **`BareFn`**

  A bare function type: `fn(usize) -> bool`.

- **`Group`**

  A type contained within invisible delimiters.

- **`ImplTrait`**

  An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
  a lifetime.

- **`Infer`**

  Indication that a type should be inferred by the compiler: `_`.

- **`Macro`**

  A macro in the type position.

- **`Never`**

  The never type: `!`.

- **`Paren`**

  A parenthesized type equivalent to the inner type.

- **`Path`**

  A path like `std::slice::Iter`, optionally qualified with a
  self-type as in `<Vec<T> as SomeTrait>::Associated`.

- **`Ptr`**

  A raw pointer type: `*const T` or `*mut T`.

- **`Reference`**

  A reference type: `&'a T` or `&'a mut T`.

- **`Slice`**

  A dynamically sized slice type: `[T]`.

- **`TraitObject`**

  A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
  trait or a lifetime.

- **`Tuple`**

  A tuple type: `(A, B, C, String)`.

- **`Verbatim`**

  Tokens in type position not interpreted by Syn.

#### Implementations

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>`
  In some positions, types may not contain the `+` character, to

#### Trait Implementations

##### `impl From`

- `fn from(e: TypeTuple) -> Type`

##### `impl From`

- `fn from(e: TypeReference) -> Type`

##### `impl From`

- `fn from(e: TypeTraitObject) -> Type`

##### `impl From`

- `fn from(e: TypeImplTrait) -> Type`

##### `impl From`

- `fn from(e: TypeMacro) -> Type`

##### `impl From`

- `fn from(e: TypeGroup) -> Type`

##### `impl From`

- `fn from(e: TypeArray) -> Type`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: TypeInfer) -> Type`

##### `impl From`

- `fn from(e: TypePath) -> Type`

##### `impl From`

- `fn from(e: TypeNever) -> Type`

##### `impl From`

- `fn from(e: TypeBareFn) -> Type`

##### `impl From`

- `fn from(e: TypePtr) -> Type`

##### `impl From`

- `fn from(e: TypeSlice) -> Type`

##### `impl From`

- `fn from(e: TypeParen) -> Type`

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

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Parse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse<T: parse::Parse>(tokens: proc_macro::TokenStream) -> Result<T>
```

Parse tokens of source code into the chosen syntax tree node.

This is preferred over parsing a string because tokens are able to preserve
information about where in the user's code they were originally written (the
"span" of the token), possibly allowing the compiler to produce better error
messages.

This function parses a `proc_macro::TokenStream` which is the type used for
interop with the compiler in a procedural macro. To parse a
`proc_macro2::TokenStream`, use `syn::parse2` instead.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

### `parse2`

```rust
fn parse2<T: parse::Parse>(tokens: proc_macro2::TokenStream) -> Result<T>
```

Parse a proc-macro2 token stream into the chosen syntax tree node.

This function parses a `proc_macro2::TokenStream` which is commonly useful
when the input comes from a node of the Syn syntax tree, for example the
body tokens of a [`Macro`](#macro) node. When in a procedural macro parsing the
`proc_macro::TokenStream` provided by the compiler, use `syn::parse`
instead.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

### `parse_str`

```rust
fn parse_str<T: parse::Parse>(s: &str) -> Result<T>
```

Parse a string of Rust code into the chosen syntax tree node.

This function enforces that the input is fully parsed. If there are any
unparsed tokens at the end of the stream, an error is returned.

# Hygiene

Every span in the resulting syntax tree will be set to resolve at the macro
call site.

# Examples

```
use syn::{Expr, Result};

fn run() -> Result<()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(code)?;
    println!("{:#?}", expr);
    Ok(())
}
#
# run().unwrap();
```

### `parse_file`

```rust
fn parse_file(content: &str) -> Result<File>
```

Parse the content of a file of Rust code.

This is different from `syn::parse_str::<File>(content)` in two ways:

- It discards a leading byte order mark `\u{FEFF}` if the file has one.
- It preserves the shebang line of the file, such as `#!/usr/bin/env rustx`.

If present, either of these would be an error using `from_str`.

# Examples

```no_run
use std::error::Error;
use std::fs;
use std::io::Read;

fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("path/to/code.rs")?;
    let ast = syn::parse_file(&content)?;
    if let Some(shebang) = ast.shebang {
        println!("{}", shebang);
    }
    println!("{} items", ast.items.len());

    Ok(())
}
#
# run().unwrap();
```

## Type Aliases

## Macros

### `parenthesized!`

Parse a set of parentheses and expose their content to subsequent parsers.

# Example

```
# use quote::quote;
#
use syn::{parenthesized, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

// Parse a simplified tuple struct syntax like:
//
//     struct S(A, B);
struct TupleStruct {
    struct_token: Token![struct](#struct)
,
    ident: Ident,
    paren_token: token::Paren,
    fields: Punctuated<Type, Token![,]>,
    semi_token: Token![;],
}

impl Parse for TupleStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(TupleStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            paren_token: parenthesized!(content in input),
            fields: content.parse_terminated(Type::parse, Token![,])?,
            semi_token: input.parse()?,
        })
    }
}
#
# fn main() {
#     let input = quote! {
#         struct S(A, B);
#     };
#     syn::parse2::<TupleStruct>(input).unwrap();
# }
```

### `braced!`

Parse a set of curly braces and expose their content to subsequent parsers.

# Example

```
# use quote::quote;
#
use syn::{braced, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

// Parse a simplified struct syntax like:
//
//     struct S {
//         a: A,
//         b: B,
//     }
struct Struct {
    struct_token: Token![struct](#struct)
,
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

struct Field {
    name: Ident,
    colon_token: Token![:],
    ty: Type,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Struct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}
#
# fn main() {
#     let input = quote! {
#         struct S {
#             a: A,
#             b: B,
#         }
#     };
#     syn::parse2::<Struct>(input).unwrap();
# }
```

### `bracketed!`

Parse a set of square brackets and expose their content to subsequent
parsers.

# Example

```
# use quote::quote;
#
use proc_macro2::TokenStream;
use syn::{bracketed, token, Result, Token};
use syn::parse::{Parse, ParseStream};

// Parse an outer attribute like:
//
//     #[repr(C, packed)]
struct OuterAttribute {
    pound_token: Token![#],
    bracket_token: token::Bracket,
    content: TokenStream,
}

impl Parse for OuterAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(OuterAttribute {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            content: content.parse()?,
        })
    }
}
#
# fn main() {
#     let input = quote! {
#         #[repr(C, packed)]
#     };
#     syn::parse2::<OuterAttribute>(input).unwrap();
# }
```

### `Token!`

A type-macro that expands to the name of the Rust type representation of a
given token.

As a type, `Token!` is commonly used in the type of struct fields, the type
of a `let` statement, or in turbofish for a `parse` function.

```
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};

// `struct Foo;`
pub struct UnitStruct {
    struct_token: Token![struct](#struct)
,
    ident: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_token: Token![struct](#struct)
 = input.parse()?;
        let ident: Ident = input.parse()?;
        let semi_token = input.parse::<Token![;]>()?;
        Ok(UnitStruct { struct_token, ident, semi_token })
    }
}
```

As an expression, `Token!` is used for peeking tokens or instantiating
tokens from a span.

```
# use syn::{Ident, Token};
# use syn::parse::{Parse, ParseStream, Result};
#
# struct UnitStruct {
#     struct_token: Token![struct](#struct)
,
#     ident: Ident,
#     semi_token: Token![;],
# }
#
# impl Parse for UnitStruct {
#     fn parse(input: ParseStream) -> Result<Self> {
#         unimplemented!()
#     }
# }
#
fn make_unit_struct(name: Ident) -> UnitStruct {
    let span = name.span();
    UnitStruct {
        struct_token: Token![struct](span),
        ident: name,
        semi_token: Token![;](span),
    }
}

# fn parse(input: ParseStream) -> Result<()> {
if input.peek(Token![struct](#struct)
) {
    let unit_struct: UnitStruct = input.parse()?;
    /* ... */
}
# Ok(())
# }
```

See the [token module] documentation for details and examples.

[token module]: crate::token

### `custom_keyword!`

Define a type that supports parsing and printing a given identifier as if it
were a keyword.

# Usage

As a convention, it is recommended that this macro be invoked within a
module called `kw` or `keyword` and that the resulting parser be invoked
with a `kw::` or `keyword::` prefix.

```
mod kw {
    syn::custom_keyword!(whatever);
}
```

The generated syntax tree node supports the following operations just like
any built-in keyword token.

- [Peeking] — `input.peek(kw::whatever)`

- [Parsing] — `input.parse::<kw::whatever>()?`

- [Printing] — `quote!( ... #whatever_token ... )`

- Construction from a [`Span`](#span) — `let whatever_token = kw::whatever(sp)`

- Field access to its span — `let sp = whatever_token.span`

[Peeking]: crate::parse::ParseBuffer::peek
[Parsing]: crate::parse::ParseBuffer::parse
[Printing]: quote::ToTokens

# Example

This example parses input that looks like `bool = true` or `str = "value"`.
The key must be either the identifier `bool` or the identifier `str`. If
`bool`, the value may be either `true` or `false`. If `str`, the value may
be any string literal.

The symbols `bool` and `str` are not reserved keywords in Rust so these are
not considered keywords in the `syn::token` module. Like any other
identifier that is not a keyword, these can be declared as custom keywords
by crates that need to use them as such.

```
use syn::{LitBool, LitStr, Result, Token};
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(bool);
    syn::custom_keyword!(str);
}

enum Argument {
    Bool {
        bool_token: kw::bool,
        eq_token: Token![=],
        value: LitBool,
    },
    Str {
        str_token: kw::str,
        eq_token: Token![=],
        value: LitStr,
    },
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::bool) {
            Ok(Argument::Bool {
                bool_token: input.parse::<kw::bool>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::str) {
            Ok(Argument::Str {
                str_token: input.parse::<kw::str>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
```

### `custom_punctuation!`

Define a type that supports parsing and printing a multi-character symbol
as if it were a punctuation token.

# Usage

```
syn::custom_punctuation!(LeftRightArrow, <=>);
```

The generated syntax tree node supports the following operations just like
any built-in punctuation token.

- [Peeking] — `input.peek(LeftRightArrow)`

- [Parsing] — `input.parse::<LeftRightArrow>()?`

- [Printing] — `quote!( ... #lrarrow ... )`

- Construction from a [`Span`](#span) — `let lrarrow = LeftRightArrow(sp)`

- Construction from multiple [`Span`](#span) — `let lrarrow = LeftRightArrow([sp, sp, sp])`

- Field access to its spans — `let spans = lrarrow.spans`

[Peeking]: crate::parse::ParseBuffer::peek
[Parsing]: crate::parse::ParseBuffer::parse
[Printing]: quote::ToTokens

# Example

```
use proc_macro2::{TokenStream, TokenTree};
use std::iter;
use syn::parse::{Parse, ParseStream, Peek, Result};
use syn::punctuated::Punctuated;
use syn::Expr;

syn::custom_punctuation!(PathSeparator, </>);

// expr </> expr </> expr ...
struct PathSegments {
    segments: Punctuated<Expr, PathSeparator>,
}

impl Parse for PathSegments {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut segments = Punctuated::new();

        let first = parse_until(input, PathSeparator)?;
        segments.push_value(syn::parse2(first)?);

        while input.peek(PathSeparator) {
            segments.push_punct(input.parse()?);

            let next = parse_until(input, PathSeparator)?;
            segments.push_value(syn::parse2(next)?);
        }

        Ok(PathSegments { segments })
    }
}

fn parse_until<E: Peek>(input: ParseStream, end: E) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();
    while !input.is_empty() && !input.peek(end) {
        let next: TokenTree = input.parse()?;
        tokens.extend(iter::once(next));
    }
    Ok(tokens)
}

fn main() {
    let input = r#" a::b </> c::d::e "#;
    let _: PathSegments = syn::parse_str(input).unwrap();
}
```

### `parse_macro_input!`

Parse the input TokenStream of a macro, triggering a compile error if the
tokens fail to parse.

Refer to the [`parse` module] documentation for more details about parsing
in Syn.

[`parse` module]: mod@crate::parse

<br>

# Intended usage

This macro must be called from a function that returns
`proc_macro::TokenStream`. Usually this will be your proc macro entry point,
the function that has the #\[proc_macro\] / #\[proc_macro_derive\] /
#\[proc_macro_attribute\] attribute.

```
# extern crate proc_macro;
#
use proc_macro::TokenStream;
use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};

struct MyMacroInput {
    /* ... */
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        /* ... */
#       Ok(MyMacroInput {})
    }
}

# const IGNORE: &str = stringify! {
#[proc_macro](#proc-macro)

# };
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MyMacroInput);

    /* ... */
#   TokenStream::new()
}
```

<br>

# Usage with Parser

This macro can also be used with the [`Parser` trait] for types that have
multiple ways that they can be parsed.

[`Parser` trait]: crate::parse::Parser

```
# extern crate proc_macro;
#
# use proc_macro::TokenStream;
# use syn::{parse_macro_input, Result};
# use syn::parse::ParseStream;
#
# struct MyMacroInput {}
#
impl MyMacroInput {
    fn parse_alternate(input: ParseStream) -> Result<Self> {
        /* ... */
#       Ok(MyMacroInput {})
    }
}

# const IGNORE: &str = stringify! {
#[proc_macro](#proc-macro)

# };
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens with MyMacroInput::parse_alternate);

    /* ... */
#   TokenStream::new()
}
```

<br>

# Expansion

`parse_macro_input!($variable as $Type)` expands to something like:

```no_run
# extern crate proc_macro;
#
# macro_rules! doc_test {
#     ($variable:ident as $Type:ty) => {
match syn::parse::<$Type>($variable) {
    Ok(syntax_tree) => syntax_tree,
    Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
}
#     };
# }
#
# fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
#     let _ = doc_test!(input as syn::Ident);
#     proc_macro::TokenStream::new()
# }
```

### `parse_quote!`

Quasi-quotation macro that accepts input like the [`quote!`](#quote) macro but uses
type inference to figure out a return type for those tokens.

The return type can be any syntax tree node that implements the [`Parse`](parse/index.md)
trait.

```
use quote::quote;
use syn::{parse_quote, Stmt};

fn main() {
    let name = quote!(v);
    let ty = quote!(u8);

    let stmt: Stmt = parse_quote! {
        let #name: #ty = Default::default();
    };

    println!("{:#?}", stmt);
}
```

*This macro is available only if Syn is built with both the `"parsing"` and
`"printing"` features.*

# Example

The following helper function adds a bound `T: HeapSize` to every type
parameter `T` in the input generics.

```
use syn::{parse_quote, Generics, GenericParam};

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(HeapSize));
        }
    }
    generics
}
```

# Special cases

This macro can parse the following additional types as a special case even
though they do not implement the `Parse` trait.

- [`Attribute`](#attribute) — parses one attribute, allowing either outer like `#[...]`
  or inner like `#![...]`
- [`Vec<Attribute>`](#vec) — parses multiple attributes, including mixed kinds in
  any order
- [`Punctuated<T, P>`](#punctuated) — parses zero or more `T` separated by punctuation
  `P` with optional trailing punctuation
- [`Vec<Arm>`](#vec) — parses arms separated by optional commas according to the
  same grammar as the inside of a `match` expression
- [`Vec<Stmt>`](#vec) — parses the same as `Block::parse_within`
- [`Pat`](#pat), [`Box<Pat>`](#box) — parses the same as
  `Pat::parse_multi_with_leading_vert`
- [`Field`](#field) — parses a named or unnamed struct field





# Panics

Panics if the tokens fail to parse as the expected syntax tree type. The
caller is responsible for ensuring that the input tokens are syntactically
valid.

### `parse_quote_spanned!`

This macro is [`parse_quote!`](#parse-quote) + `quote_spanned!`.

Please refer to each of their documentation.

# Example

```
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote_spanned, ReturnType, Signature};

// Changes `fn()` to `fn() -> Pin<Box<dyn Future<Output = ()>>>`,
// and `fn() -> T` to `fn() -> Pin<Box<dyn Future<Output = T>>>`,
// without introducing any call_site() spans.
fn make_ret_pinned_future(sig: &mut Signature) {
    let ret = match &sig.output {
        ReturnType::Default => quote_spanned!(sig.paren_token.span=> ()),
        ReturnType::Type(_, ret) => quote!(#ret),
    };
    sig.output = parse_quote_spanned! {ret.span()=>
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = #ret>>>
    };
}
```

