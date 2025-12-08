*[syn](../index.md) / [attr](index.md)*

---

# Module `attr`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

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

This type does not implement the [`Parse`](../parse/index.md) trait and thus cannot be
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

- `fn path(self: &Self) -> &Path` — [`Path`](../path/index.md)

- `fn parse_args<T: Parse>(self: &Self) -> Result<T>` — [`Result`](../error/index.md)

- `fn parse_args_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md), [`Parser`](../parse/index.md)

- `fn parse_nested_meta(self: &Self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](../meta/index.md), [`Result`](../error/index.md)

- `fn parse_outer(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

- `fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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

- `fn parse_args<T: Parse>(self: &Self) -> Result<T>` — [`Result`](../error/index.md)

- `fn parse_args_with<F: Parser>(self: &Self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md), [`Parser`](../parse/index.md)

- `fn parse_nested_meta(self: &Self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](../meta/index.md), [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::MetaList`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::MetaList`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaList`

##### `impl Hash for crate::MetaList`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::attr::MetaList`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

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

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::MetaNameValue`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for MetaNameValue`

##### `impl<T> Spanned for MetaNameValue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::attr::MetaNameValue`

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

- `fn path(self: &Self) -> &Path` — [`Path`](../path/index.md)

- `fn require_path_only(self: &Self) -> Result<&Path>` — [`Result`](../error/index.md), [`Path`](../path/index.md)

- `fn require_list(self: &Self) -> Result<&MetaList>` — [`Result`](../error/index.md), [`MetaList`](#metalist)

- `fn require_name_value(self: &Self) -> Result<&MetaNameValue>` — [`Result`](../error/index.md), [`MetaNameValue`](#metanamevalue)

#### Trait Implementations

##### `impl Clone for crate::Meta`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Meta`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Meta`

##### `impl Hash for crate::Meta`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::attr::Meta`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Meta`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Meta`

##### `impl<T> Spanned for Meta`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::attr::Meta`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Traits

### `FilterAttrs<'a>`

```rust
trait FilterAttrs<'a> { ... }
```

#### Required Methods

- `type Ret: 1`

- `fn outer(self: Self) -> <Self as >::Ret`

- `fn inner(self: Self) -> <Self as >::Ret`

