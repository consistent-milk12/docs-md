# Crate `syn`

[![github]](https://github.com/dtolnay/syn)&ensp;[![crates-io]](https://crates.io/crates/syn)&ensp;[![docs-rs]](crate)



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
[dependencies]
syn = "2.0"
quote = "1.0"

[lib]
proc-macro = true
```

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

const IGNORE_TOKENS: &str = stringify! {
#[proc_macro_derive(MyMacro)]
};
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

The `heapsize` example directory shows a complete working implementation
of a derive macro. The example derives a `HeapSize` trait which computes an
estimate of the amount of heap memory owned by a value.

```rust
pub trait HeapSize {
    /// Total number of bytes of heap memory owned by `self`.
    fn heap_size_of_children(&self) -> usize;
}
```

The derive macro allows users to write `#[derive(HeapSize)]` on data
structures in their program.

```rust
const IGNORE_TOKENS: &str = stringify! {
#[derive(HeapSize)]
};
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

```rust
const IGNORE_TOKENS: &str = stringify! {
#[derive(HeapSize)]
};
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

The `lazy-static` example directory shows the implementation of a
`functionlike!(...)` procedural macro in which the input tokens are parsed
using Syn's parsing API.

The example reimplements the popular `lazy_static` crate from crates.io as a
procedural macro.

```rust
macro_rules! lazy_static {
    ($($tt:tt)*) => {}
}

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
produces maximally helpful error messages. Consider using the `trybuild`
crate to write tests for errors that are emitted by your macro or errors
detected by the Rust compiler in the expanded code following misuse of the
macro. Such tests help avoid regressions from later refactors that
mistakenly make an error no longer trigger or be less helpful than it used
to be.

<br>

# Debugging

When developing a procedural macro it can be helpful to look at what the
generated code looks like. Use `cargo rustc -- -Zunstable-options
--pretty=expanded` or the `cargo expand` subcommand.

To show the expanded code for some crate that uses your procedural macro,
run `cargo expand` from that crate. To show the expanded code for one of
your own test cases, run `cargo expand --test the_test_case` where the last
argument is the name of the test file without the `.rs` extension.

This write-up by Brandon W Maister discusses debugging in more detail:
[Debugging Rust's new Custom Derive system][debugging].

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

- [`macros`](macros/index.md) - 
- [`group`](group/index.md) - 
- [`token`](token/index.md) - Tokens representing Rust punctuation, keywords, and delimiters.
- [`attr`](attr/index.md) - 
- [`bigint`](bigint/index.md) - 
- [`buffer`](buffer/index.md) - A stably addressed token buffer supporting efficient traversal based on a
- [`classify`](classify/index.md) - 
- [`custom_keyword`](custom_keyword/index.md) - 
- [`custom_punctuation`](custom_punctuation/index.md) - 
- [`data`](data/index.md) - 
- [`derive`](derive/index.md) - 
- [`drops`](drops/index.md) - 
- [`error`](error/index.md) - 
- [`expr`](expr/index.md) - 
- [`ext`](ext/index.md) - Extension traits to provide parsing methods on foreign types.
- [`file`](file/index.md) - 
- [`fixup`](fixup/index.md) - 
- [`generics`](generics/index.md) - 
- [`ident`](ident/index.md) - 
- [`item`](item/index.md) - 
- [`lifetime`](lifetime/index.md) - 
- [`lit`](lit/index.md) - 
- [`lookahead`](lookahead/index.md) - 
- [`mac`](mac/index.md) - 
- [`meta`](meta/index.md) - Facility for interpreting structured content inside of an `Attribute`.
- [`op`](op/index.md) - 
- [`parse`](parse/index.md) - Parsing interface for parsing a token stream into a syntax tree node.
- [`parse_macro_input`](parse_macro_input/index.md) - 
- [`parse_quote`](parse_quote/index.md) - 
- [`pat`](pat/index.md) - 
- [`path`](path/index.md) - 
- [`precedence`](precedence/index.md) - 
- [`print`](print/index.md) - 
- [`punctuated`](punctuated/index.md) - A punctuated sequence of syntax tree nodes separated by punctuation.
- [`restriction`](restriction/index.md) - 
- [`sealed`](sealed/index.md) - 
- [`span`](span/index.md) - 
- [`spanned`](spanned/index.md) - A trait that can provide the `Span` of the complete contents of a syntax
- [`stmt`](stmt/index.md) - 
- [`thread`](thread/index.md) - 
- [`tt`](tt/index.md) - 
- [`ty`](ty/index.md) - 
- [`verbatim`](verbatim/index.md) - 
- [`whitespace`](whitespace/index.md) - 
- [`gen`](gen/index.md) - 
- [`visit_mut`](visit_mut/index.md) - 

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
  path, for example the `#[test]` attribute.

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

#[test]
  ^^^^Meta::Path
```

<br>

# Parsing from tokens to Attribute

This type does not implement the [`Parse`](parse/index.md) trait and thus cannot be
parsed directly by `ParseStream::parse`. Instead use
`ParseStream::call` with one of the two parser functions
`Attribute::parse_outer` or `Attribute::parse_inner` depending on
which you intend to parse.



```rust
use syn::{Attribute, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};

// Parses a unit struct with attributes.
//
//     #[path = "s.tmpl"]
//     struct S;
struct UnitStruct {
    attrs: Vec<Attribute>,
    struct_token: Token![struct],
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
attribute. Use `parse_args()` to parse those tokens into the expected
data structure.

<p><br></p>

# Doc comments

The compiler transforms doc comments, such as `/// comment` and `/*!
comment */`, into attributes before macros are expanded. Each comment is
expanded into an attribute of the form `#[doc = r"comment"]`.

As an example, the following `mod` items are expanded identically:

```rust
use syn::{ItemMod, parse_quote};
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

- `fn path(self: &Self) -> &Path` — [`Path`](#path)

- `fn parse_args<T: Parse>(self: &Self) -> Result<T>` — [`Result`](#result)

- `fn parse_args_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>` — [`Result`](#result), [`Parser`](parse/index.md)

- `fn parse_nested_meta(self: &Self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](meta/index.md), [`Result`](#result)

- `fn parse_outer(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Attribute`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Attribute`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Attribute`

##### `impl Hash for crate::Attribute`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Attribute`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Attribute`

##### `impl<T> Spanned for Attribute`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::attr::Attribute`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::MetaList`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::MetaList`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaList`

##### `impl Hash for crate::MetaList`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::attr::MetaList`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::MetaList`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for MetaList`

##### `impl<T> Spanned for MetaList`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::attr::MetaList`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `MetaNameValue`

```rust
struct MetaNameValue {
    pub path: crate::path::Path,
    pub eq_token: $crate::token::Eq,
    pub value: crate::expr::Expr,
}
```

A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::MetaNameValue`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::MetaNameValue`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaNameValue`

##### `impl Hash for crate::MetaNameValue`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::attr::MetaNameValue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::MetaNameValue`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for MetaNameValue`

##### `impl<T> Spanned for MetaNameValue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::attr::MetaNameValue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn parse_named(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse_unnamed(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Field`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Field`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Field`

##### `impl Hash for crate::Field`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Field`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Field`

##### `impl<T> Spanned for Field`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::Field`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldsNamed`

```rust
struct FieldsNamed {
    pub brace_token: token::Brace,
    pub named: crate::punctuated::Punctuated<Field, $crate::token::Comma>,
}
```

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsNamed`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldsNamed`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsNamed`

##### `impl Hash for crate::FieldsNamed`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::data::FieldsNamed`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FieldsNamed`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsNamed`

##### `impl<T> Spanned for FieldsNamed`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::FieldsNamed`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldsUnnamed`

```rust
struct FieldsUnnamed {
    pub paren_token: token::Paren,
    pub unnamed: crate::punctuated::Punctuated<Field, $crate::token::Comma>,
}
```

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsUnnamed`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldsUnnamed`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsUnnamed`

##### `impl Hash for crate::FieldsUnnamed`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::data::FieldsUnnamed`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FieldsUnnamed`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsUnnamed`

##### `impl<T> Spanned for FieldsUnnamed`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::FieldsUnnamed`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::Variant`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Variant`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variant`

##### `impl Hash for crate::Variant`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::data::Variant`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Variant`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Variant`

##### `impl<T> Spanned for Variant`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::Variant`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `DataEnum`

```rust
struct DataEnum {
    pub enum_token: $crate::token::Enum,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>,
}
```

An enum input to a `proc_macro_derive` macro.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataEnum`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DataEnum`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataEnum`

##### `impl Hash for crate::DataEnum`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::DataEnum`

- `fn eq(self: &Self, other: &Self) -> bool`

### `DataStruct`

```rust
struct DataStruct {
    pub struct_token: $crate::token::Struct,
    pub fields: crate::data::Fields,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A struct input to a `proc_macro_derive` macro.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DataStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataStruct`

##### `impl Hash for crate::DataStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::DataStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

### `DataUnion`

```rust
struct DataUnion {
    pub union_token: $crate::token::Union,
    pub fields: crate::data::FieldsNamed,
}
```

An untagged union input to a `proc_macro_derive` macro.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataUnion`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DataUnion`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataUnion`

##### `impl Hash for crate::DataUnion`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::DataUnion`

- `fn eq(self: &Self, other: &Self) -> bool`

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

##### `impl Clone for crate::DeriveInput`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DeriveInput`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DeriveInput`

##### `impl Hash for crate::DeriveInput`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::derive::DeriveInput`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::DeriveInput`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for DeriveInput`

##### `impl<T> Spanned for DeriveInput`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::derive::DeriveInput`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Error`

```rust
struct Error {
    messages: Vec<ErrorMessage>,
}
```

Error returned when a Syn parser cannot parse the input tokens.

# Error reporting in proc macros

The correct way to report errors back to the compiler from a procedural
macro is by emitting an appropriately spanned invocation of
`compile_error!` in the generated code. This produces a better diagnostic
message than simply panicking the macro.

When parsing macro input, the `parse_macro_input!` macro handles the
conversion to `compile_error!` automatically.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, ItemFn};

const IGNORE: &str = stringify! {
#[proc_macro_attribute]
};
pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MyAttrArgs);
    let input = parse_macro_input!(input as ItemFn);

    /* ... */
    TokenStream::new()
}

struct MyAttrArgs {
    _k: [(); { stringify! {
    ...
    }; 0 }]
}

impl Parse for MyAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        stringify! {
        ...
        };
        unimplemented!()
    }
}
```

For errors that arise later than the initial parsing stage, the
`.to_compile_error()` or `.into_compile_error()` methods can be used to
perform an explicit conversion to `compile_error!`.


```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

const IGNORE: &str = stringify! {
#[proc_macro_derive(MyDerive)]
};
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // fn(DeriveInput) -> syn::Result<proc_macro2::TokenStream>
    expand::my_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

mod expand {
    use proc_macro2::TokenStream;
    use syn::{DeriveInput, Result};

    pub fn my_derive(input: DeriveInput) -> Result<TokenStream> {
        unimplemented!()
    }
}
```

#### Implementations

- `fn new<T: Display>(span: Span, message: T) -> Self`

- `fn new_spanned<T: ToTokens, U: Display>(tokens: T, message: U) -> Self`

- `fn span(self: &Self) -> Span`

- `fn to_compile_error(self: &Self) -> TokenStream`

- `fn into_compile_error(self: Self) -> TokenStream`

- `fn combine(self: &mut Self, another: Error)` — [`Error`](#error)

#### Trait Implementations

##### `impl Clone for Error`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for Error`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

##### `impl Extend for Error`

- `fn extend<T: IntoIterator<Item = Error>>(self: &mut Self, iter: T)`

##### `impl IntoIterator for Error`

- `type Item = Error`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

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

```rust
fn f() -> bool {
    let n = 0;
match n {
    0..=10 => {
        return true;
    }
    // ...
    _ => {}
}
  false
}
```

#### Implementations

- `fn parse_multiple(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Arm`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Arm`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Arm`

##### `impl Hash for crate::Arm`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::Arm`

- `fn parse(input: ParseStream<'_>) -> Result<Arm>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Arm`](#arm)

##### `impl PartialEq for crate::Arm`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Arm`

##### `impl<T> Spanned for Arm`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Arm`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Label`

```rust
struct Label {
    pub name: crate::lifetime::Lifetime,
    pub colon_token: $crate::token::Colon,
}
```

A lifetime labeling a `for`, `while`, or `loop`.

#### Trait Implementations

##### `impl Clone for crate::Label`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Label`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Label`

##### `impl Hash for crate::Label`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::Label`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Label`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Label`

##### `impl<T> Spanned for Label`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Label`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBinary`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprBinary`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBinary`

##### `impl Hash for crate::ExprBinary`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBinary`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprBinary`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBinary`

##### `impl<T> Spanned for ExprBinary`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprBinary`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCall`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprCall`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCall`

##### `impl Hash for crate::ExprCall`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCall`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprCall`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprCall`

##### `impl<T> Spanned for ExprCall`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprCall`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCast`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprCast`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCast`

##### `impl Hash for crate::ExprCast`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCast`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprCast`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprCast`

##### `impl<T> Spanned for ExprCast`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprCast`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprField`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprField`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprField`

##### `impl Hash for crate::ExprField`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprField`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprField`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprField`

##### `impl<T> Spanned for ExprField`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprField`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIndex`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprIndex`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIndex`

##### `impl Hash for crate::ExprIndex`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIndex`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprIndex`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprIndex`

##### `impl<T> Spanned for ExprIndex`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprIndex`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprLit`

```rust
struct ExprLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLit`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprLit`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl Hash for crate::ExprLit`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLit`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLit`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLit`

##### `impl<T> Spanned for ExprLit`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprLit`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprMacro`

```rust
struct ExprMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl Hash for crate::ExprMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMacro`

##### `impl<T> Spanned for ExprMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMethodCall`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprMethodCall`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMethodCall`

##### `impl Hash for crate::ExprMethodCall`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMethodCall`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMethodCall`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMethodCall`

##### `impl<T> Spanned for ExprMethodCall`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprMethodCall`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprParen`

```rust
struct ExprParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub expr: Box<Expr>,
}
```

A parenthesized expression: `(a + b)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprParen`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprParen`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprParen`

##### `impl Hash for crate::ExprParen`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprParen`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprParen`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprParen`

##### `impl<T> Spanned for ExprParen`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprParen`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprPath`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprPath`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl Hash for crate::ExprPath`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprPath`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprPath`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprPath`

##### `impl<T> Spanned for ExprPath`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprPath`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReference`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprReference`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReference`

##### `impl Hash for crate::ExprReference`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReference`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprReference`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprReference`

##### `impl<T> Spanned for ExprReference`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprReference`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprStruct`

##### `impl Hash for crate::ExprStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprStruct`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprStruct`

##### `impl<T> Spanned for ExprStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprUnary`

```rust
struct ExprUnary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub op: crate::op::UnOp,
    pub expr: Box<Expr>,
}
```

A unary operation: `!x`, `*x`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnary`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprUnary`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnary`

##### `impl Hash for crate::ExprUnary`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnary`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprUnary`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprUnary`

##### `impl<T> Spanned for ExprUnary`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnary`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::FieldValue`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldValue`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldValue`

##### `impl Hash for crate::FieldValue`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::FieldValue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FieldValue`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldValue`

##### `impl<T> Spanned for FieldValue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::FieldValue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Index`

```rust
struct Index {
    pub index: u32,
    pub span: proc_macro2::Span,
}
```

The index of an unnamed tuple struct field.

#### Trait Implementations

##### `impl Clone for crate::Index`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Index`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Index`

##### `impl Hash for Index`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl IdentFragment for Index`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn span(self: &Self) -> Option<Span>`

##### `impl Parse for crate::expr::Index`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for Index`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Index`

##### `impl<T> Spanned for Index`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Index`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprArray`

```rust
struct ExprArray {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A slice literal expression: `[a, b, c, d]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprArray`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprArray`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprArray`

##### `impl Hash for crate::ExprArray`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprArray`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprArray`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprArray`

##### `impl<T> Spanned for ExprArray`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprArray`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAssign`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprAssign`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAssign`

##### `impl Hash for crate::ExprAssign`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAssign`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprAssign`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAssign`

##### `impl<T> Spanned for ExprAssign`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprAssign`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAsync`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprAsync`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAsync`

##### `impl Hash for crate::ExprAsync`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAsync`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprAsync`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAsync`

##### `impl<T> Spanned for ExprAsync`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprAsync`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAwait`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprAwait`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAwait`

##### `impl Hash for crate::ExprAwait`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAwait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprAwait`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAwait`

##### `impl<T> Spanned for ExprAwait`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprAwait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprBlock`

```rust
struct ExprBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub block: crate::stmt::Block,
}
```

A blocked scope: `{ ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBlock`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprBlock`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBlock`

##### `impl Hash for crate::ExprBlock`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBlock`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprBlock`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBlock`

##### `impl<T> Spanned for ExprBlock`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprBlock`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBreak`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprBreak`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBreak`

##### `impl Hash for crate::ExprBreak`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBreak`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprBreak`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBreak`

##### `impl<T> Spanned for ExprBreak`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprBreak`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprClosure`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprClosure`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprClosure`

##### `impl Hash for crate::ExprClosure`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprClosure`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprClosure`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprClosure`

##### `impl<T> Spanned for ExprClosure`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprClosure`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprConst`

```rust
struct ExprConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl Hash for crate::ExprConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprConst`

##### `impl<T> Spanned for ExprConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprContinue`

```rust
struct ExprContinue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub continue_token: $crate::token::Continue,
    pub label: Option<crate::lifetime::Lifetime>,
}
```

A `continue`, with an optional label.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprContinue`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprContinue`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprContinue`

##### `impl Hash for crate::ExprContinue`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprContinue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprContinue`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprContinue`

##### `impl<T> Spanned for ExprContinue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprContinue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprForLoop`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprForLoop`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprForLoop`

##### `impl Hash for crate::ExprForLoop`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprForLoop`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprForLoop`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprForLoop`

##### `impl<T> Spanned for ExprForLoop`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprForLoop`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprGroup`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprGroup`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprGroup`

##### `impl Hash for crate::ExprGroup`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::ExprGroup`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprGroup`

##### `impl<T> Spanned for ExprGroup`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprGroup`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIf`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprIf`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIf`

##### `impl Hash for crate::ExprIf`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIf`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprIf`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprIf`

##### `impl<T> Spanned for ExprIf`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprIf`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprInfer`

```rust
struct ExprInfer {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: $crate::token::Underscore,
}
```

The inferred value of a const generic argument, denoted `_`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprInfer`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprInfer`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprInfer`

##### `impl Hash for crate::ExprInfer`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprInfer`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprInfer`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprInfer`

##### `impl<T> Spanned for ExprInfer`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprInfer`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLet`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprLet`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLet`

##### `impl Hash for crate::ExprLet`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLet`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLet`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLet`

##### `impl<T> Spanned for ExprLet`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprLet`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLoop`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprLoop`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLoop`

##### `impl Hash for crate::ExprLoop`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLoop`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLoop`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLoop`

##### `impl<T> Spanned for ExprLoop`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprLoop`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMatch`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprMatch`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMatch`

##### `impl Hash for crate::ExprMatch`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMatch`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMatch`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMatch`

##### `impl<T> Spanned for ExprMatch`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprMatch`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRange`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprRange`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl Hash for crate::ExprRange`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRange`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRange`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRange`

##### `impl<T> Spanned for ExprRange`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprRange`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRawAddr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprRawAddr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRawAddr`

##### `impl Hash for crate::ExprRawAddr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRawAddr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRawAddr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRawAddr`

##### `impl<T> Spanned for ExprRawAddr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprRawAddr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRepeat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprRepeat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRepeat`

##### `impl Hash for crate::ExprRepeat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRepeat`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRepeat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRepeat`

##### `impl<T> Spanned for ExprRepeat`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprRepeat`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprReturn`

```rust
struct ExprReturn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub return_token: $crate::token::Return,
    pub expr: Option<Box<Expr>>,
}
```

A `return`, with an optional value to be returned.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReturn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprReturn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReturn`

##### `impl Hash for crate::ExprReturn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReturn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprReturn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprReturn`

##### `impl<T> Spanned for ExprReturn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprReturn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprTry`

```rust
struct ExprTry {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub question_token: $crate::token::Question,
}
```

A try-expression: `expr?`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTry`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprTry`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTry`

##### `impl Hash for crate::ExprTry`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTry`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprTry`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTry`

##### `impl<T> Spanned for ExprTry`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprTry`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprTryBlock`

```rust
struct ExprTryBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub try_token: $crate::token::Try,
    pub block: crate::stmt::Block,
}
```

A try block: `try { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTryBlock`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprTryBlock`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTryBlock`

##### `impl Hash for crate::ExprTryBlock`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTryBlock`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprTryBlock`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTryBlock`

##### `impl<T> Spanned for ExprTryBlock`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprTryBlock`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprTuple`

```rust
struct ExprTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A tuple expression: `(a, b, c, d)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTuple`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprTuple`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTuple`

##### `impl Hash for crate::ExprTuple`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTuple`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprTuple`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTuple`

##### `impl<T> Spanned for ExprTuple`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprTuple`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprUnsafe`

```rust
struct ExprUnsafe {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafe_token: $crate::token::Unsafe,
    pub block: crate::stmt::Block,
}
```

An unsafe block: `unsafe { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnsafe`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprUnsafe`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnsafe`

##### `impl Hash for crate::ExprUnsafe`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnsafe`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprUnsafe`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprUnsafe`

##### `impl<T> Spanned for ExprUnsafe`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnsafe`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprWhile`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprWhile`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprWhile`

##### `impl Hash for crate::ExprWhile`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprWhile`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprWhile`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprWhile`

##### `impl<T> Spanned for ExprWhile`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprWhile`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprYield`

```rust
struct ExprYield {
    pub attrs: Vec<crate::attr::Attribute>,
    pub yield_token: $crate::token::Yield,
    pub expr: Option<Box<Expr>>,
}
```

A yield expression: `yield expr`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprYield`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprYield`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprYield`

##### `impl Hash for crate::ExprYield`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprYield`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprYield`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprYield`

##### `impl<T> Spanned for ExprYield`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprYield`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

```rust
use std::env;
use std::fs;
use std::process;

fn main() {
}

fn fake_main() {
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

##### `impl Clone for crate::File`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::File`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::File`

##### `impl Hash for crate::File`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::file::File`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::File`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for File`

##### `impl<T> Spanned for File`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::file::File`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::BoundLifetimes`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::BoundLifetimes`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoundLifetimes`

- `fn default() -> Self`

##### `impl Eq for crate::BoundLifetimes`

##### `impl Hash for crate::BoundLifetimes`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::BoundLifetimes`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::BoundLifetimes`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BoundLifetimes`

##### `impl<T> Spanned for BoundLifetimes`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::BoundLifetimes`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::ConstParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ConstParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ConstParam`

##### `impl Hash for crate::ConstParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::ConstParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ConstParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ConstParam`

##### `impl<T> Spanned for ConstParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::ConstParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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



#### Implementations

- `fn lifetimes(self: &Self) -> Lifetimes<'_>` — [`Lifetimes`](generics/index.md)

- `fn lifetimes_mut(self: &mut Self) -> LifetimesMut<'_>` — [`LifetimesMut`](generics/index.md)

- `fn type_params(self: &Self) -> TypeParams<'_>` — [`TypeParams`](generics/index.md)

- `fn type_params_mut(self: &mut Self) -> TypeParamsMut<'_>` — [`TypeParamsMut`](generics/index.md)

- `fn const_params(self: &Self) -> ConstParams<'_>` — [`ConstParams`](generics/index.md)

- `fn const_params_mut(self: &mut Self) -> ConstParamsMut<'_>` — [`ConstParamsMut`](generics/index.md)

- `fn make_where_clause(self: &mut Self) -> &mut WhereClause` — [`WhereClause`](#whereclause)

- `fn split_for_impl(self: &Self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>)` — [`ImplGenerics`](#implgenerics), [`TypeGenerics`](#typegenerics), [`WhereClause`](#whereclause)

#### Trait Implementations

##### `impl Clone for crate::Generics`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Generics`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Generics`

- `fn default() -> Self`

##### `impl Eq for crate::Generics`

##### `impl Hash for crate::Generics`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::Generics`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Generics`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Generics`

##### `impl<T> Spanned for Generics`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::Generics`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn new(lifetime: Lifetime) -> Self` — [`Lifetime`](#lifetime)

#### Trait Implementations

##### `impl Clone for crate::LifetimeParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::LifetimeParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LifetimeParam`

##### `impl Hash for crate::LifetimeParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::LifetimeParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::LifetimeParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LifetimeParam`

##### `impl<T> Spanned for LifetimeParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::LifetimeParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::PredicateLifetime`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PredicateLifetime`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateLifetime`

##### `impl Hash for crate::PredicateLifetime`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PredicateLifetime`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PredicateLifetime`

##### `impl<T> Spanned for PredicateLifetime`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::PredicateLifetime`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::PredicateType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PredicateType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateType`

##### `impl Hash for crate::PredicateType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PredicateType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PredicateType`

##### `impl<T> Spanned for PredicateType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::PredicateType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn do_parse(input: ParseStream<'_>, allow_const: bool) -> Result<Option<Self>>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::TraitBound`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitBound`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBound`

##### `impl Hash for crate::TraitBound`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBound`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitBound`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitBound`

##### `impl<T> Spanned for TraitBound`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::TraitBound`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::TypeParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParam`

##### `impl Hash for crate::TypeParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParam`

##### `impl<T> Spanned for TypeParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::TypeParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::WhereClause`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::WhereClause`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WhereClause`

##### `impl Hash for crate::WhereClause`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::WhereClause`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::WhereClause`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for WhereClause`

##### `impl<T> Spanned for WhereClause`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::WhereClause`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::PreciseCapture`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PreciseCapture`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PreciseCapture`

##### `impl Hash for crate::PreciseCapture`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::PreciseCapture`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PreciseCapture`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PreciseCapture`

##### `impl<T> Spanned for PreciseCapture`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::PreciseCapture`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ImplGenerics<'a>`

```rust
struct ImplGenerics<'a>(&'a Generics);
```

Returned by `Generics::split_for_impl`.

#### Trait Implementations

##### `impl<'a> Clone for ImplGenerics<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a> Debug for ImplGenerics<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for ImplGenerics<'a>`

##### `impl<'a> Hash for ImplGenerics<'a>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'a> PartialEq for ImplGenerics<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplGenerics<'a>`

##### `impl<T> Spanned for ImplGenerics<'a>`

- `fn span(self: &Self) -> Span`

##### `impl<'a> ToTokens for crate::generics::ImplGenerics<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Turbofish<'a>`

```rust
struct Turbofish<'a>(&'a Generics);
```

Returned by `TypeGenerics::as_turbofish`.

#### Trait Implementations

##### `impl<'a> Clone for Turbofish<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a> Debug for Turbofish<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for Turbofish<'a>`

##### `impl<'a> Hash for Turbofish<'a>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'a> PartialEq for Turbofish<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Turbofish<'a>`

##### `impl<T> Spanned for Turbofish<'a>`

- `fn span(self: &Self) -> Span`

##### `impl<'a> ToTokens for crate::generics::Turbofish<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeGenerics<'a>`

```rust
struct TypeGenerics<'a>(&'a Generics);
```

Returned by `Generics::split_for_impl`.

#### Implementations

- `fn as_turbofish(self: &Self) -> Turbofish<'a>` — [`Turbofish`](#turbofish)

#### Trait Implementations

##### `impl<'a> Clone for TypeGenerics<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a> Debug for TypeGenerics<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for TypeGenerics<'a>`

##### `impl<'a> Hash for TypeGenerics<'a>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'a> PartialEq for TypeGenerics<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeGenerics<'a>`

##### `impl<T> Spanned for TypeGenerics<'a>`

- `fn span(self: &Self) -> Span`

##### `impl<'a> ToTokens for crate::generics::TypeGenerics<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemFn`

##### `impl Hash for crate::ForeignItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemFn`

##### `impl<T> Spanned for ForeignItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ForeignItemMacro`

```rust
struct ForeignItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within an extern block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemMacro`

##### `impl Hash for crate::ForeignItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemMacro`

##### `impl<T> Spanned for ForeignItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemStatic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemStatic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemStatic`

##### `impl Hash for crate::ForeignItemStatic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemStatic`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemStatic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemStatic`

##### `impl<T> Spanned for ForeignItemStatic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemStatic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemType`

##### `impl Hash for crate::ForeignItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemType`

##### `impl<T> Spanned for ForeignItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemConst`

##### `impl Hash for crate::ImplItemConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemConst`

##### `impl<T> Spanned for ImplItemConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemFn`

##### `impl Hash for crate::ImplItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemFn`

##### `impl<T> Spanned for ImplItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ImplItemMacro`

```rust
struct ImplItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within an impl block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemMacro`

##### `impl Hash for crate::ImplItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemMacro`

##### `impl<T> Spanned for ImplItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemType`

##### `impl Hash for crate::ImplItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemType`

##### `impl<T> Spanned for ImplItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemConst`

##### `impl Hash for crate::ItemConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemConst`

##### `impl<T> Spanned for ItemConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemEnum`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemEnum`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemEnum`

##### `impl Hash for crate::ItemEnum`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemEnum`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemEnum`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemEnum`

##### `impl<T> Spanned for ItemEnum`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemEnum`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemExternCrate`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemExternCrate`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemExternCrate`

##### `impl Hash for crate::ItemExternCrate`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemExternCrate`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemExternCrate`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemExternCrate`

##### `impl<T> Spanned for ItemExternCrate`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemExternCrate`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemFn`

##### `impl Hash for crate::ItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemFn`

##### `impl<T> Spanned for ItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemForeignMod`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemForeignMod`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemForeignMod`

##### `impl Hash for crate::ItemForeignMod`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemForeignMod`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemForeignMod`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemForeignMod`

##### `impl<T> Spanned for ItemForeignMod`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemForeignMod`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemImpl`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemImpl`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemImpl`

##### `impl Hash for crate::ItemImpl`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemImpl`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemImpl`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemImpl`

##### `impl<T> Spanned for ItemImpl`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemImpl`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMacro`

##### `impl Hash for crate::ItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemMacro`

##### `impl<T> Spanned for ItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMod`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemMod`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMod`

##### `impl Hash for crate::ItemMod`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemMod`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemMod`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemMod`

##### `impl<T> Spanned for ItemMod`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemMod`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStatic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemStatic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStatic`

##### `impl Hash for crate::ItemStatic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemStatic`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemStatic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemStatic`

##### `impl<T> Spanned for ItemStatic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemStatic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStruct`

##### `impl Hash for crate::ItemStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemStruct`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemStruct`

##### `impl<T> Spanned for ItemStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTrait`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemTrait`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTrait`

##### `impl Hash for crate::ItemTrait`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemTrait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemTrait`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemTrait`

##### `impl<T> Spanned for ItemTrait`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemTrait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTraitAlias`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemTraitAlias`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTraitAlias`

##### `impl Hash for crate::ItemTraitAlias`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemTraitAlias`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemTraitAlias`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemTraitAlias`

##### `impl<T> Spanned for ItemTraitAlias`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemTraitAlias`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemType`

##### `impl Hash for crate::ItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemType`

##### `impl<T> Spanned for ItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUnion`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemUnion`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUnion`

##### `impl Hash for crate::ItemUnion`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemUnion`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemUnion`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemUnion`

##### `impl<T> Spanned for ItemUnion`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemUnion`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUse`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemUse`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUse`

##### `impl Hash for crate::ItemUse`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemUse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ItemUse`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemUse`

##### `impl<T> Spanned for ItemUse`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemUse`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn lifetime(self: &Self) -> Option<&Lifetime>` — [`Lifetime`](#lifetime)

#### Trait Implementations

##### `impl Clone for crate::Receiver`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Receiver`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Receiver`

##### `impl Hash for crate::Receiver`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::Receiver`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Receiver`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Receiver`

##### `impl<T> Spanned for Receiver`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::Receiver`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn receiver(self: &Self) -> Option<&Receiver>` — [`Receiver`](#receiver)

#### Trait Implementations

##### `impl Clone for crate::Signature`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Signature`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Signature`

##### `impl Hash for crate::Signature`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::Signature`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Signature`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Signature`

##### `impl<T> Spanned for Signature`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::Signature`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemConst`

##### `impl Hash for crate::TraitItemConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemConst`

##### `impl<T> Spanned for TraitItemConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemFn`

##### `impl Hash for crate::TraitItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemFn`

##### `impl<T> Spanned for TraitItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TraitItemMacro`

```rust
struct TraitItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within the definition of a trait.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemMacro`

##### `impl Hash for crate::TraitItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemMacro`

##### `impl<T> Spanned for TraitItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemType`

##### `impl Hash for crate::TraitItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemType`

##### `impl<T> Spanned for TraitItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UseGlob`

```rust
struct UseGlob {
    pub star_token: $crate::token::Star,
}
```

A glob import in a `use` item: `*`.

#### Trait Implementations

##### `impl Clone for crate::UseGlob`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseGlob`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGlob`

##### `impl Hash for crate::UseGlob`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl PartialEq for crate::UseGlob`

- `fn eq(self: &Self, _other: &Self) -> bool`

##### `impl<T> Sealed for UseGlob`

##### `impl<T> Spanned for UseGlob`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseGlob`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UseGroup`

```rust
struct UseGroup {
    pub brace_token: token::Brace,
    pub items: crate::punctuated::Punctuated<UseTree, $crate::token::Comma>,
}
```

A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Clone for crate::UseGroup`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseGroup`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGroup`

##### `impl Hash for crate::UseGroup`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UseGroup`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseGroup`

##### `impl<T> Spanned for UseGroup`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseGroup`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UseName`

```rust
struct UseName {
    pub ident: crate::ident::Ident,
}
```

An identifier imported by a `use` item: `HashMap`.

#### Trait Implementations

##### `impl Clone for crate::UseName`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseName`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseName`

##### `impl Hash for crate::UseName`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UseName`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseName`

##### `impl<T> Spanned for UseName`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseName`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::UsePath`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UsePath`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UsePath`

##### `impl Hash for crate::UsePath`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UsePath`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UsePath`

##### `impl<T> Spanned for UsePath`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UsePath`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::UseRename`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseRename`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseRename`

##### `impl Hash for crate::UseRename`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UseRename`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseRename`

##### `impl<T> Spanned for UseRename`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseRename`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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
struct c_char;
struct c_int;

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    //                               ^^^
}
```

#### Trait Implementations

##### `impl Clone for crate::Variadic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Variadic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variadic`

##### `impl Hash for crate::Variadic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Variadic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Variadic`

##### `impl<T> Spanned for Variadic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::Variadic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

#### Trait Implementations

##### `impl Clone for Lifetime`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Lifetime`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Lifetime`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Lifetime`

##### `impl Hash for Lifetime`

- `fn hash<H: Hasher>(self: &Self, h: &mut H)`

##### `impl Ord for Lifetime`

- `fn cmp(self: &Self, other: &Lifetime) -> Ordering` — [`Lifetime`](#lifetime)

##### `impl Parse for crate::lifetime::Lifetime`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for Lifetime`

- `fn eq(self: &Self, other: &Lifetime) -> bool` — [`Lifetime`](#lifetime)

##### `impl PartialOrd for Lifetime`

- `fn partial_cmp(self: &Self, other: &Lifetime) -> Option<Ordering>` — [`Lifetime`](#lifetime)

##### `impl<T> Sealed for Lifetime`

##### `impl<T> Spanned for Lifetime`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for Lifetime`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for crate::lifetime::Lifetime`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lifetime::Lifetime`

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

- `fn token(self: &Self) -> Ident` — [`Ident`](#ident)

#### Trait Implementations

##### `impl Clone for crate::LitBool`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitBool`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitBool`

##### `impl Hash for crate::LitBool`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitBool`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::LitBool`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitBool`

##### `impl<T> Spanned for LitBool`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitBool`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitBool`

### `LitByte`

```rust
struct LitByte {
    repr: Box<LitRepr>,
}
```

A byte literal: `b'f'`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitByte`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitByte`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByte`

##### `impl Hash for LitByte`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitByte`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitByte`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitByte`

##### `impl<T> Spanned for LitByte`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitByte`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByte`

### `LitByteStr`

```rust
struct LitByteStr {
    repr: Box<LitRepr>,
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

##### `impl Clone for LitByteStr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitByteStr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByteStr`

##### `impl Hash for LitByteStr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitByteStr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitByteStr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitByteStr`

##### `impl<T> Spanned for LitByteStr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitByteStr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByteStr`

### `LitCStr`

```rust
struct LitCStr {
    repr: Box<LitRepr>,
}
```

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitCStr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitCStr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitCStr`

##### `impl Hash for LitCStr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitCStr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitCStr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::lit::LitCStr`

##### `impl<T> Spanned for LitCStr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitCStr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitCStr`

### `LitChar`

```rust
struct LitChar {
    repr: Box<LitRepr>,
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

##### `impl Clone for LitChar`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitChar`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitChar`

##### `impl Hash for LitChar`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitChar`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitChar`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitChar`

##### `impl<T> Spanned for LitChar`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitChar`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitChar`

### `LitFloat`

```rust
struct LitFloat {
    repr: Box<LitFloatRepr>,
}
```

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitFloat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitFloat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitFloat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitFloat`

##### `impl Hash for LitFloat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitFloat`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitFloat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitFloat`

##### `impl<T> Spanned for LitFloat`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for LitFloat`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for crate::lit::LitFloat`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitFloat`

### `LitInt`

```rust
struct LitInt {
    repr: Box<LitIntRepr>,
}
```

An integer literal: `1` or `1u16`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for LitInt`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitInt`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitInt`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitInt`

##### `impl Hash for LitInt`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitInt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitInt`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitInt`

##### `impl<T> Spanned for LitInt`

- `fn span(self: &Self) -> Span`

##### `impl<T> ToString for LitInt`

- `fn to_string(self: &Self) -> String`

##### `impl ToTokens for crate::lit::LitInt`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitInt`

### `LitStr`

```rust
struct LitStr {
    repr: Box<LitRepr>,
}
```

A UTF-8 string literal: `"foo"`.

#### Implementations

- `fn new(value: &str, span: Span) -> Self`

- `fn value(self: &Self) -> String`

- `fn parse<T: Parse>(self: &Self) -> Result<T>` — [`Result`](#result)

- `fn parse_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>` — [`Result`](#result), [`Parser`](parse/index.md)

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

- `fn suffix(self: &Self) -> &str`

- `fn token(self: &Self) -> Literal`

#### Trait Implementations

##### `impl Clone for LitStr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::lit::LitStr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitStr`

##### `impl Hash for LitStr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::LitStr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for LitStr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LitStr`

##### `impl<T> Spanned for LitStr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::lit::LitStr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitStr`

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

- `fn parse_body<T: Parse>(self: &Self) -> Result<T>` — [`Result`](#result)

- `fn parse_body_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>` — [`Result`](#result), [`Parser`](parse/index.md)

#### Trait Implementations

##### `impl Clone for crate::Macro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Macro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Macro`

##### `impl Hash for crate::Macro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::mac::Macro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Macro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Macro`

##### `impl<T> Spanned for Macro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::mac::Macro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::FieldPat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldPat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldPat`

##### `impl Hash for crate::FieldPat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::FieldPat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldPat`

##### `impl<T> Spanned for FieldPat`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::FieldPat`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatConst`

```rust
struct PatConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl Hash for crate::ExprConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprConst`

##### `impl<T> Spanned for ExprConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatIdent`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatIdent`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatIdent`

##### `impl Hash for crate::PatIdent`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatIdent`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatIdent`

##### `impl<T> Spanned for PatIdent`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatIdent`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatLit`

```rust
struct PatLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLit`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprLit`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl Hash for crate::ExprLit`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLit`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprLit`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLit`

##### `impl<T> Spanned for ExprLit`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprLit`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatMacro`

```rust
struct PatMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl Hash for crate::ExprMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMacro`

##### `impl<T> Spanned for ExprMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatOr`

```rust
struct PatOr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub leading_vert: Option<$crate::token::Or>,
    pub cases: crate::punctuated::Punctuated<Pat, $crate::token::Or>,
}
```

A pattern that matches any one of a set of cases.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatOr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatOr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatOr`

##### `impl Hash for crate::PatOr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatOr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatOr`

##### `impl<T> Spanned for PatOr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatOr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatParen`

```rust
struct PatParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub pat: Box<Pat>,
}
```

A parenthesized pattern: `(A | B)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatParen`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatParen`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatParen`

##### `impl Hash for crate::PatParen`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatParen`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatParen`

##### `impl<T> Spanned for PatParen`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatParen`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprPath`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprPath`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl Hash for crate::ExprPath`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprPath`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprPath`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprPath`

##### `impl<T> Spanned for ExprPath`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprPath`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRange`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprRange`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl Hash for crate::ExprRange`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRange`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ExprRange`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRange`

##### `impl<T> Spanned for ExprRange`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprRange`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatReference`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatReference`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatReference`

##### `impl Hash for crate::PatReference`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatReference`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatReference`

##### `impl<T> Spanned for PatReference`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatReference`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatRest`

```rust
struct PatRest {
    pub attrs: Vec<crate::attr::Attribute>,
    pub dot2_token: $crate::token::DotDot,
}
```

The dots in a tuple or slice pattern: `[0, 1, ..]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatRest`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatRest`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatRest`

##### `impl Hash for crate::PatRest`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatRest`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatRest`

##### `impl<T> Spanned for PatRest`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatRest`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatSlice`

```rust
struct PatSlice {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatSlice`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatSlice`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatSlice`

##### `impl Hash for crate::PatSlice`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatSlice`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatSlice`

##### `impl<T> Spanned for PatSlice`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatSlice`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatStruct`

##### `impl Hash for crate::PatStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatStruct`

##### `impl<T> Spanned for PatStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatTuple`

```rust
struct PatTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A tuple pattern: `(a, b)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTuple`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatTuple`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTuple`

##### `impl Hash for crate::PatTuple`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatTuple`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatTuple`

##### `impl<T> Spanned for PatTuple`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatTuple`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTupleStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatTupleStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTupleStruct`

##### `impl Hash for crate::PatTupleStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatTupleStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatTupleStruct`

##### `impl<T> Spanned for PatTupleStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatTupleStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatType`

##### `impl Hash for crate::PatType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::pat::PatType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PatType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatType`

##### `impl<T> Spanned for PatType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatWild`

```rust
struct PatWild {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: $crate::token::Underscore,
}
```

A pattern that matches any value: `_`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatWild`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatWild`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatWild`

##### `impl Hash for crate::PatWild`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatWild`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatWild`

##### `impl<T> Spanned for PatWild`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatWild`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn parse_turbofish(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn do_parse(colon2_token: Option<$crate::token::PathSep>, input: ParseStream<'_>) -> Result<Self>` — [`PathSep`](token/index.md), [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::AngleBracketedGenericArguments`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::AngleBracketedGenericArguments`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AngleBracketedGenericArguments`

##### `impl Hash for crate::AngleBracketedGenericArguments`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::AngleBracketedGenericArguments`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::AngleBracketedGenericArguments`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for AngleBracketedGenericArguments`

##### `impl<T> Spanned for AngleBracketedGenericArguments`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::AngleBracketedGenericArguments`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::AssocConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::AssocConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocConst`

##### `impl Hash for crate::AssocConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::AssocConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocConst`

##### `impl<T> Spanned for AssocConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::AssocConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::AssocType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::AssocType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocType`

##### `impl Hash for crate::AssocType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::AssocType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocType`

##### `impl<T> Spanned for AssocType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::AssocType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::Constraint`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Constraint`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Constraint`

##### `impl Hash for crate::Constraint`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Constraint`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Constraint`

##### `impl<T> Spanned for Constraint`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::Constraint`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ParenthesizedGenericArguments`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ParenthesizedGenericArguments`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ParenthesizedGenericArguments`

##### `impl Hash for crate::ParenthesizedGenericArguments`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::ParenthesizedGenericArguments`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ParenthesizedGenericArguments`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ParenthesizedGenericArguments`

##### `impl<T> Spanned for ParenthesizedGenericArguments`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::ParenthesizedGenericArguments`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Path`

```rust
struct Path {
    pub leading_colon: Option<$crate::token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, $crate::token::PathSep>,
}
```

A path at which a named item is exported (e.g. `std::collections::HashMap`).

#### Implementations

- `fn parse_mod_style(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse_rest(input: ParseStream<'_>, path: &mut Self, expr_style: bool) -> Result<()>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn is_mod_style(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Path`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Path`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Path`

##### `impl Hash for crate::Path`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::Path`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Path`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl PartialEq for syn::Path`

##### `impl<T> Sealed for Path`

##### `impl<T> Spanned for Path`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::Path`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

A segment of a path together with any path arguments on that segment.

#### Implementations

- `fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::PathSegment`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PathSegment`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PathSegment`

##### `impl Hash for crate::PathSegment`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::PathSegment`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PathSegment`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PathSegment`

##### `impl<T> Spanned for PathSegment`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::PathSegment`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::QSelf`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::QSelf`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::QSelf`

##### `impl Hash for crate::QSelf`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::QSelf`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::QSelf`

##### `impl Spanned for crate::path::QSelf`

- `fn span(self: &Self) -> Span`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::VisRestricted`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::VisRestricted`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::VisRestricted`

##### `impl Hash for crate::VisRestricted`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::VisRestricted`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for VisRestricted`

##### `impl<T> Spanned for VisRestricted`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::restriction::VisRestricted`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn parse_within(input: ParseStream<'_>) -> Result<Vec<Stmt>>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Stmt`](#stmt)

#### Trait Implementations

##### `impl Clone for crate::Block`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Block`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Block`

##### `impl Hash for crate::Block`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::stmt::Block`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Block`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Block`

##### `impl<T> Spanned for Block`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::Block`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::Local`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Local`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Local`

##### `impl Hash for crate::Local`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Local`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Local`

##### `impl<T> Spanned for Local`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::Local`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::LocalInit`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::LocalInit`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LocalInit`

##### `impl Hash for crate::LocalInit`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::LocalInit`

- `fn eq(self: &Self, other: &Self) -> bool`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::StmtMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::StmtMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StmtMacro`

##### `impl Hash for crate::StmtMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::StmtMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for StmtMacro`

##### `impl<T> Spanned for StmtMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::StmtMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Abi`

```rust
struct Abi {
    pub extern_token: $crate::token::Extern,
    pub name: Option<crate::lit::LitStr>,
}
```

The binary interface of a function: `extern "C"`.

#### Trait Implementations

##### `impl Clone for crate::Abi`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Abi`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Abi`

##### `impl Hash for crate::Abi`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::Abi`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Abi`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Abi`

##### `impl<T> Spanned for Abi`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::Abi`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::BareFnArg`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::BareFnArg`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareFnArg`

##### `impl Hash for crate::BareFnArg`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::BareFnArg`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::BareFnArg`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BareFnArg`

##### `impl<T> Spanned for BareFnArg`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::BareFnArg`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::BareVariadic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::BareVariadic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareVariadic`

##### `impl Hash for crate::BareVariadic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::BareVariadic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BareVariadic`

##### `impl<T> Spanned for BareVariadic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::BareVariadic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeArray`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeArray`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeArray`

##### `impl Hash for crate::TypeArray`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeArray`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeArray`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeArray`

##### `impl<T> Spanned for TypeArray`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeArray`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeBareFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeBareFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeBareFn`

##### `impl Hash for crate::TypeBareFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeBareFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeBareFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeBareFn`

##### `impl<T> Spanned for TypeBareFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeBareFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeGroup`

```rust
struct TypeGroup {
    pub group_token: token::Group,
    pub elem: Box<Type>,
}
```

A type contained within invisible delimiters.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeGroup`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeGroup`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeGroup`

##### `impl Hash for crate::TypeGroup`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeGroup`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeGroup`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeGroup`

##### `impl<T> Spanned for TypeGroup`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeGroup`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::TypeImplTrait`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeImplTrait`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeImplTrait`

##### `impl Hash for crate::TypeImplTrait`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeImplTrait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeImplTrait`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeImplTrait`

##### `impl<T> Spanned for TypeImplTrait`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeImplTrait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeInfer`

```rust
struct TypeInfer {
    pub underscore_token: $crate::token::Underscore,
}
```

Indication that a type should be inferred by the compiler: `_`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeInfer`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeInfer`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeInfer`

##### `impl Hash for crate::TypeInfer`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeInfer`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeInfer`

- `fn eq(self: &Self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeInfer`

##### `impl<T> Spanned for TypeInfer`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeInfer`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeMacro`

```rust
struct TypeMacro {
    pub mac: crate::mac::Macro,
}
```

A macro in the type position.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeMacro`

##### `impl Hash for crate::TypeMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeMacro`

##### `impl<T> Spanned for TypeMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeNever`

```rust
struct TypeNever {
    pub bang_token: $crate::token::Not,
}
```

The never type: `!`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeNever`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeNever`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeNever`

##### `impl Hash for crate::TypeNever`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeNever`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeNever`

- `fn eq(self: &Self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeNever`

##### `impl<T> Spanned for TypeNever`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeNever`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeParen`

```rust
struct TypeParen {
    pub paren_token: token::Paren,
    pub elem: Box<Type>,
}
```

A parenthesized type equivalent to the inner type.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeParen`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeParen`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParen`

##### `impl Hash for crate::TypeParen`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeParen`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeParen`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParen`

##### `impl<T> Spanned for TypeParen`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeParen`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypePath`

```rust
struct TypePath {
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::slice::Iter`, optionally qualified with a
self-type as in `<Vec<T> as SomeTrait>::Associated`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePath`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypePath`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePath`

##### `impl Hash for crate::TypePath`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypePath`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypePath`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePath`

##### `impl<T> Spanned for TypePath`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypePath`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePtr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypePtr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePtr`

##### `impl Hash for crate::TypePtr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypePtr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypePtr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePtr`

##### `impl<T> Spanned for TypePtr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypePtr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeReference`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeReference`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeReference`

##### `impl Hash for crate::TypeReference`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeReference`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeReference`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeReference`

##### `impl<T> Spanned for TypeReference`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeReference`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeSlice`

```rust
struct TypeSlice {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
}
```

A dynamically sized slice type: `[T]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeSlice`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeSlice`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeSlice`

##### `impl Hash for crate::TypeSlice`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeSlice`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeSlice`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeSlice`

##### `impl<T> Spanned for TypeSlice`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeSlice`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeTraitObject`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeTraitObject`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTraitObject`

##### `impl Hash for crate::TypeTraitObject`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTraitObject`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeTraitObject`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTraitObject`

##### `impl<T> Spanned for TypeTraitObject`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeTraitObject`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeTuple`

```rust
struct TypeTuple {
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Type, $crate::token::Comma>,
}
```

A tuple type: `(A, B, C, String)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeTuple`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeTuple`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTuple`

##### `impl Hash for crate::TypeTuple`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTuple`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeTuple`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTuple`

##### `impl<T> Spanned for TypeTuple`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeTuple`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::AttrStyle`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::AttrStyle`

##### `impl Debug for crate::AttrStyle`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AttrStyle`

##### `impl Hash for crate::AttrStyle`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::AttrStyle`

- `fn eq(self: &Self, other: &Self) -> bool`

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

A meta path is like the `test` in `#[test]`.

## List

A meta list is like the `derive(Copy)` in `#[derive(Copy)]`.

## NameValue

A name-value meta is like the `path = "..."` in `#[path =
"sys/windows.rs"]`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`List`**

  A structured list within an attribute, like `derive(Copy, Clone)`.

- **`NameValue`**

  A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- `fn path(self: &Self) -> &Path` — [`Path`](#path)

- `fn require_path_only(self: &Self) -> Result<&Path>` — [`Result`](#result), [`Path`](#path)

- `fn require_list(self: &Self) -> Result<&MetaList>` — [`Result`](#result), [`MetaList`](#metalist)

- `fn require_name_value(self: &Self) -> Result<&MetaNameValue>` — [`Result`](#result), [`MetaNameValue`](#metanamevalue)

#### Trait Implementations

##### `impl Clone for crate::Meta`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Meta`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Meta`

##### `impl Hash for crate::Meta`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::attr::Meta`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Meta`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Meta`

##### `impl<T> Spanned for Meta`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::attr::Meta`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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


#### Variants

- **`Named`**

  Named fields of a struct or struct variant such as `Point { x: f64,
  y: f64 }`.

- **`Unnamed`**

  Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

- **`Unit`**

  Unit struct or unit variant such as `None`.

#### Implementations

- `fn iter(self: &Self) -> punctuated::Iter<'_, Field>` — [`Iter`](punctuated/index.md), [`Field`](#field)

- `fn iter_mut(self: &mut Self) -> punctuated::IterMut<'_, Field>` — [`IterMut`](punctuated/index.md), [`Field`](#field)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn members(self: &Self) -> Members<'_>` — [`Members`](data/index.md)

#### Trait Implementations

##### `impl Clone for crate::Fields`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Fields`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Fields`

##### `impl Hash for crate::Fields`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl IntoIterator for Fields`

- `type Item = Field`

- `type IntoIter = IntoIter<Field>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl PartialEq for crate::Fields`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Fields`

##### `impl<T> Spanned for Fields`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Fields`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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


#### Trait Implementations

##### `impl Clone for crate::Data`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Data`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Data`

##### `impl Hash for crate::Data`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Data`

- `fn eq(self: &Self, other: &Self) -> bool`

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

##### `impl Clone for crate::PointerMutability`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PointerMutability`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PointerMutability`

##### `impl Hash for crate::PointerMutability`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::PointerMutability`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::PointerMutability`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PointerMutability`

##### `impl<T> Spanned for PointerMutability`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::PointerMutability`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn parse_obsolete(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::RangeLimits`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::RangeLimits`

##### `impl Debug for crate::RangeLimits`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::RangeLimits`

##### `impl Hash for crate::RangeLimits`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::RangeLimits`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::RangeLimits`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for RangeLimits`

##### `impl<T> Spanned for RangeLimits`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::RangeLimits`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

```rust
use syn::Expr;

fn example(expr: Expr) {
const IGNORE: &str = stringify! {
let expr: Expr = /* ... */;
};
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
    _ => {}
}
}
```

We begin with a variable `expr` of type `Expr` that has no fields
(because it is an enum), and by matching on it and rebinding a variable
with the same name `expr` we effectively imbue our variable with all of
the data fields provided by the variant that it turned out to be. So for
example above if we ended up in the `MethodCall` case then we get to use
`expr.receiver`, `expr.args` etc; if we ended up in the `If` case we get
to use `expr.cond`, `expr.then_branch`, `expr.else_branch`.

This approach avoids repeating the variant names twice on every line.

```rust
use syn::{Expr, ExprMethodCall};

fn example(expr: Expr) {
// Repetitive; recommend not doing this.
match expr {
    Expr::MethodCall(ExprMethodCall { method, args, .. }) => {
}
_ => {}
}
}
```

In general, the name to which a syntax tree enum variant is bound should
be a suitable name for the complete syntax tree enum type.

```rust
use syn::{Expr, ExprField};

fn example(discriminant: ExprField) {
// Binding is called `base` which is the name I would use if I were
// assigning `*discriminant.base` without an `if let`.
if let Expr::Tuple(base) = *discriminant.base {
}
}
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

- `fn parse_without_eager_brace(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Expr`](#expr)

- `fn parse_with_earlier_boundary_rule(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Expr`](#expr)

- `fn peek(input: ParseStream<'_>) -> bool` — [`ParseStream`](parse/index.md)

- `fn replace_attrs(self: &mut Self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](#attribute)

#### Trait Implementations

##### `impl Clone for crate::Expr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Expr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Expr`

##### `impl Hash for crate::Expr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::Expr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Expr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Expr`

##### `impl<T> Spanned for Expr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Expr`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

#### Implementations

- `fn is_named(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Member`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Member`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Member`

##### `impl Hash for Member`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl IdentFragment for Member`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn span(self: &Self) -> Option<Span>`

##### `impl Parse for crate::expr::Member`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for Member`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Member`

##### `impl<T> Spanned for Member`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Member`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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


#### Variants

- **`Lifetime`**

  A lifetime parameter: `'a: 'b + 'c + 'd`.

- **`Type`**

  A generic type parameter: `T: Into<String>`.

- **`Const`**

  A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Clone for crate::GenericParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::GenericParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericParam`

##### `impl Hash for crate::GenericParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::GenericParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::GenericParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for GenericParam`

##### `impl<T> Spanned for GenericParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for GenericParam`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

##### `impl Clone for crate::TraitBoundModifier`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::TraitBoundModifier`

##### `impl Debug for crate::TraitBoundModifier`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBoundModifier`

##### `impl Hash for crate::TraitBoundModifier`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBoundModifier`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitBoundModifier`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitBoundModifier`

##### `impl<T> Spanned for TraitBoundModifier`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::TraitBoundModifier`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

#### Implementations

- `fn parse_single(input: ParseStream<'_>, allow_precise_capture: bool, allow_const: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse_multiple(input: ParseStream<'_>, allow_plus: bool, allow_precise_capture: bool, allow_const: bool) -> Result<Punctuated<Self, $crate::token::Plus>>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`Punctuated`](punctuated/index.md), [`Plus`](token/index.md)

#### Trait Implementations

##### `impl Clone for crate::TypeParamBound`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeParamBound`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParamBound`

##### `impl Hash for crate::TypeParamBound`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParamBound`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TypeParamBound`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParamBound`

##### `impl<T> Spanned for TypeParamBound`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for TypeParamBound`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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


#### Variants

- **`Lifetime`**

  A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

- **`Type`**

  A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Trait Implementations

##### `impl Clone for crate::WherePredicate`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::WherePredicate`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WherePredicate`

##### `impl Hash for crate::WherePredicate`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::WherePredicate`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::WherePredicate`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for WherePredicate`

##### `impl<T> Spanned for WherePredicate`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for WherePredicate`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

##### `impl Clone for crate::CapturedParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::CapturedParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::CapturedParam`

##### `impl Hash for crate::CapturedParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::CapturedParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::CapturedParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for CapturedParam`

##### `impl<T> Spanned for CapturedParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::CapturedParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::FnArg`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FnArg`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FnArg`

##### `impl Hash for crate::FnArg`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::FnArg`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::FnArg`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FnArg`

##### `impl<T> Spanned for FnArg`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for FnArg`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

##### `impl Clone for crate::ForeignItem`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItem`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItem`

##### `impl Hash for crate::ForeignItem`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItem`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ForeignItem`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItem`

##### `impl<T> Spanned for ForeignItem`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for ForeignItem`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

##### `impl Clone for crate::ImplItem`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItem`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItem`

##### `impl Hash for crate::ImplItem`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItem`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ImplItem`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItem`

##### `impl<T> Spanned for ImplItem`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for ImplItem`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `ImplRestriction`

```rust
enum ImplRestriction {
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::ImplRestriction`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplRestriction`

- `fn fmt(self: &Self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplRestriction`

##### `impl Hash for crate::ImplRestriction`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl PartialEq for crate::ImplRestriction`

- `fn eq(self: &Self, _other: &Self) -> bool`

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

#### Implementations

- `fn replace_attrs(self: &mut Self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](#attribute)

#### Trait Implementations

##### `impl Clone for crate::Item`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Item`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Item`

##### `impl Hash for crate::Item`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::Item`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Item`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Item`

##### `impl<T> Spanned for Item`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Item`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `StaticMutability`

```rust
enum StaticMutability {
    Mut($crate::token::Mut),
    None,
}
```

The mutability of an `Item::Static` or `ForeignItem::Static`.

#### Trait Implementations

##### `impl Clone for crate::StaticMutability`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::StaticMutability`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StaticMutability`

##### `impl Hash for crate::StaticMutability`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::StaticMutability`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::StaticMutability`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for StaticMutability`

##### `impl<T> Spanned for StaticMutability`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::StaticMutability`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::TraitItem`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItem`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItem`

##### `impl Hash for crate::TraitItem`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItem`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::TraitItem`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItem`

##### `impl<T> Spanned for TraitItem`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for TraitItem`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

##### `impl Clone for crate::UseTree`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseTree`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseTree`

##### `impl Hash for crate::UseTree`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::UseTree`

- `fn parse(input: ParseStream<'_>) -> Result<UseTree>` — [`ParseStream`](parse/index.md), [`Result`](#result), [`UseTree`](#usetree)

##### `impl PartialEq for crate::UseTree`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseTree`

##### `impl<T> Spanned for UseTree`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for UseTree`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

- `fn from_str(token: Literal, repr: &str) -> Self`

- `fn suffix(self: &Self) -> &str`

- `fn span(self: &Self) -> Span`

- `fn set_span(self: &mut Self, span: Span)`

#### Trait Implementations

##### `impl Clone for crate::Lit`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Lit`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Lit`

##### `impl Hash for crate::Lit`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::lit::Lit`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Lit`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Lit`

##### `impl<T> Spanned for Lit`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Lit`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token for crate::lit::Lit`

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

- `fn surround(self: &Self, tokens: &mut TokenStream, inner: TokenStream)`

#### Trait Implementations

##### `impl Clone for crate::MacroDelimiter`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::MacroDelimiter`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MacroDelimiter`

##### `impl Hash for crate::MacroDelimiter`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::MacroDelimiter`

- `fn eq(self: &Self, other: &Self) -> bool`

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

##### `impl Clone for crate::BinOp`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::BinOp`

##### `impl Debug for crate::BinOp`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BinOp`

##### `impl Hash for crate::BinOp`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::op::BinOp`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::BinOp`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BinOp`

##### `impl<T> Spanned for BinOp`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::op::BinOp`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::UnOp`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::UnOp`

##### `impl Debug for crate::UnOp`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UnOp`

##### `impl Hash for crate::UnOp`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::op::UnOp`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::UnOp`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UnOp`

##### `impl<T> Spanned for UnOp`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::op::UnOp`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn parse_single(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse_multi(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse_multi_with_leading_vert(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Pat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Pat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Pat`

##### `impl Hash for crate::Pat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Pat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Pat`

##### `impl<T> Spanned for Pat`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Pat`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

##### `impl Clone for crate::GenericArgument`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::GenericArgument`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericArgument`

##### `impl Hash for crate::GenericArgument`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::GenericArgument`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::GenericArgument`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for GenericArgument`

##### `impl<T> Spanned for GenericArgument`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::GenericArgument`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::PathArguments`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PathArguments`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathArguments`

- `fn default() -> Self`

##### `impl Eq for crate::PathArguments`

##### `impl Hash for crate::PathArguments`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PathArguments`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PathArguments`

##### `impl<T> Spanned for PathArguments`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::PathArguments`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldMutability`

```rust
enum FieldMutability {
    None,
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::FieldMutability`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldMutability`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldMutability`

##### `impl Hash for crate::FieldMutability`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::FieldMutability`

- `fn eq(self: &Self, other: &Self) -> bool`

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


#### Variants

- **`Public`**

  A public visibility level: `pub`.

- **`Restricted`**

  A visibility level restricted to some path: `pub(self)` or
  `pub(super)` or `pub(crate)` or `pub(in some::module)`.

- **`Inherited`**

  An inherited visibility, which usually means private.

#### Implementations

- `fn is_inherited(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Visibility`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Visibility`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Visibility`

##### `impl Hash for crate::Visibility`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::restriction::Visibility`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Visibility`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Visibility`

##### `impl<T> Spanned for Visibility`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::restriction::Visibility`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

##### `impl Clone for crate::Stmt`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Stmt`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Stmt`

##### `impl Hash for crate::Stmt`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::stmt::Stmt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Stmt`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Stmt`

##### `impl<T> Spanned for Stmt`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::Stmt`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

- `fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::ReturnType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ReturnType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ReturnType`

##### `impl Hash for crate::ReturnType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::ReturnType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::ReturnType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ReturnType`

##### `impl<T> Spanned for ReturnType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::ReturnType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

#### Trait Implementations

##### `impl Clone for crate::Type`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Type`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Type`

##### `impl Hash for crate::Type`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::Type`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](parse/index.md), [`Result`](#result)

##### `impl PartialEq for crate::Type`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Type`

##### `impl<T> Spanned for Type`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Type`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

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

```rust
use syn::{Expr, Result};

fn run() -> Result<()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(code)?;
    println!("{:#?}", expr);
    Ok(())
}

run().unwrap();
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

run().unwrap();
```

## Type Aliases

## Macros

### `parenthesized!`

Parse a set of parentheses and expose their content to subsequent parsers.

# Example

```rust
use quote::quote;

use syn::{parenthesized, token, Ident, Result, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

// Parse a simplified tuple struct syntax like:
//
//     struct S(A, B);
struct TupleStruct {
    struct_token: Token![struct],
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

fn main() {
    let input = quote! {
        struct S(A, B);
    };
    syn::parse2::<TupleStruct>(input).unwrap();
}
```

### `braced!`

Parse a set of curly braces and expose their content to subsequent parsers.

# Example

```rust
use quote::quote;

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
    struct_token: Token![struct],
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

fn main() {
    let input = quote! {
        struct S {
            a: A,
            b: B,
        }
    };
    syn::parse2::<Struct>(input).unwrap();
}
```

### `bracketed!`

Parse a set of square brackets and expose their content to subsequent
parsers.

# Example

```rust
use quote::quote;

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

fn main() {
    let input = quote! {
        #[repr(C, packed)]
    };
    syn::parse2::<OuterAttribute>(input).unwrap();
}
```

### `Token!`

A type-macro that expands to the name of the Rust type representation of a
given token.

As a type, `Token!` is commonly used in the type of struct fields, the type
of a `let` statement, or in turbofish for a `parse` function.

```rust
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};

// `struct Foo;`
pub struct UnitStruct {
    struct_token: Token![struct],
    ident: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_token: Token![struct] = input.parse()?;
        let ident: Ident = input.parse()?;
        let semi_token = input.parse::<Token![;]>()?;
        Ok(UnitStruct { struct_token, ident, semi_token })
    }
}
```

As an expression, `Token!` is used for peeking tokens or instantiating
tokens from a span.

```rust
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};

struct UnitStruct {
    struct_token: Token![struct],
    ident: Ident,
    semi_token: Token![;],
}

impl Parse for UnitStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        unimplemented!()
    }
}

fn make_unit_struct(name: Ident) -> UnitStruct {
    let span = name.span();
    UnitStruct {
        struct_token: Token![struct](span),
        ident: name,
        semi_token: Token![;](span),
    }
}

fn parse(input: ParseStream) -> Result<()> {
if input.peek(Token![struct]) {
    let unit_struct: UnitStruct = input.parse()?;
    /* ... */
}
Ok(())
}
```

See the [token module] documentation for details and examples.


### `custom_keyword!`

Define a type that supports parsing and printing a given identifier as if it
were a keyword.

# Usage

As a convention, it is recommended that this macro be invoked within a
module called `kw` or `keyword` and that the resulting parser be invoked
with a `kw::` or `keyword::` prefix.

```rust
mod kw {
    syn::custom_keyword!(whatever);
}
```

The generated syntax tree node supports the following operations just like
any built-in keyword token.

- [Peeking] — `input.peek(kw::whatever)`

- [Parsing] — `input.parse::<kw::whatever>()?`

- [Printing] — `quote!( ... #whatever_token ... )`

- Construction from a `Span` — `let whatever_token = kw::whatever(sp)`

- Field access to its span — `let sp = whatever_token.span`




# Example

This example parses input that looks like `bool = true` or `str = "value"`.
The key must be either the identifier `bool` or the identifier `str`. If
`bool`, the value may be either `true` or `false`. If `str`, the value may
be any string literal.

The symbols `bool` and `str` are not reserved keywords in Rust so these are
not considered keywords in the `syn::token` module. Like any other
identifier that is not a keyword, these can be declared as custom keywords
by crates that need to use them as such.

```rust
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

```rust
syn::custom_punctuation!(LeftRightArrow, <=>);
```

The generated syntax tree node supports the following operations just like
any built-in punctuation token.

- [Peeking] — `input.peek(LeftRightArrow)`

- [Parsing] — `input.parse::<LeftRightArrow>()?`

- [Printing] — `quote!( ... #lrarrow ... )`

- Construction from a `Span` — `let lrarrow = LeftRightArrow(sp)`

- Construction from multiple `Span` — `let lrarrow = LeftRightArrow([sp, sp, sp])`

- Field access to its spans — `let spans = lrarrow.spans`




# Example

```rust
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

<br>

# Intended usage

This macro must be called from a function that returns
`proc_macro::TokenStream`. Usually this will be your proc macro entry point,
the function that has the #\[proc_macro\] / #\[proc_macro_derive\] /
#\[proc_macro_attribute\] attribute.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Result};
use syn::parse::{Parse, ParseStream};

struct MyMacroInput {
    /* ... */
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        /* ... */
      Ok(MyMacroInput {})
    }
}

const IGNORE: &str = stringify! {
#[proc_macro]
};
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MyMacroInput);

    /* ... */
  TokenStream::new()
}
```

<br>

# Usage with Parser

This macro can also be used with the [`Parser` trait] for types that have
multiple ways that they can be parsed.

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Result};
use syn::parse::ParseStream;

struct MyMacroInput {}

impl MyMacroInput {
    fn parse_alternate(input: ParseStream) -> Result<Self> {
        /* ... */
      Ok(MyMacroInput {})
    }
}

const IGNORE: &str = stringify! {
#[proc_macro]
};
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens with MyMacroInput::parse_alternate);

    /* ... */
  TokenStream::new()
}
```

<br>

# Expansion

`parse_macro_input!($variable as $Type)` expands to something like:

```no_run
extern crate proc_macro;

macro_rules! doc_test {
    ($variable:ident as $Type:ty) => {
match syn::parse::<$Type>($variable) {
    Ok(syntax_tree) => syntax_tree,
    Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
}
    };
}

fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ = doc_test!(input as syn::Ident);
    proc_macro::TokenStream::new()
}
```

### `parse_quote!`

Quasi-quotation macro that accepts input like the `quote!` macro but uses
type inference to figure out a return type for those tokens.

The return type can be any syntax tree node that implements the [`Parse`](parse/index.md)
trait.

```rust
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

```rust
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
- `Vec<Attribute>` — parses multiple attributes, including mixed kinds in
  any order
- [`Punctuated<T, P>`](punctuated/index.md) — parses zero or more `T` separated by punctuation
  `P` with optional trailing punctuation
- `Vec<Arm>` — parses arms separated by optional commas according to the
  same grammar as the inside of a `match` expression
- `Vec<Stmt>` — parses the same as `Block::parse_within`
- [`Pat`](#pat), `Box<Pat>` — parses the same as
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

```rust
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

