*[syn](../index.md) / [attr](index.md)*

---

# Module `attr`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Attribute`](#attribute)
  - [`MetaList`](#metalist)
  - [`MetaNameValue`](#metanamevalue)
- [Enums](#enums)
  - [`AttrStyle`](#attrstyle)
  - [`Meta`](#meta)
- [Traits](#traits)
  - [`FilterAttrs`](#filterattrs)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Attribute`](#attribute) | struct | An attribute, like `#[repr(transparent)]`. |
| [`MetaList`](#metalist) | struct | A structured list within an attribute, like `derive(Copy, Clone)`. |
| [`MetaNameValue`](#metanamevalue) | struct | A name-value pair within an attribute, like `feature = "nightly"`. |
| [`AttrStyle`](#attrstyle) | enum | Distinguishes between attributes that decorate an item and attributes |
| [`Meta`](#meta) | enum | Content of a compile-time structured attribute. |
| [`FilterAttrs`](#filterattrs) | trait |  |

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Attribute`

```rust
struct Attribute {
    pub pound_token: token::Pound,
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

- <span id="attribute-path"></span>`fn path(&self) -> &Path` — [`Path`](../index.md)

- <span id="attribute-parse-args"></span>`fn parse_args<T: Parse>(&self) -> Result<T>` — [`Result`](../index.md)

- <span id="attribute-parse-args-with"></span>`fn parse_args_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](../index.md), [`Parser`](../parse/index.md)

- <span id="attribute-parse-nested-meta"></span>`fn parse_nested_meta(&self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](../meta/index.md), [`Result`](../index.md)

- <span id="attribute-parse-outer"></span>`fn parse_outer(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="attribute-parse-inner"></span>`fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::Attribute`

- <span id="crateattribute-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Attribute`

- <span id="crateattribute-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Attribute`

##### `impl Hash for crate::Attribute`

- <span id="crateattribute-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Attribute`

- <span id="crateattribute-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Attribute`

##### `impl<T> Spanned for Attribute`

- <span id="attribute-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::Attribute`

- <span id="crateattrattribute-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="cratemetalist-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::MetaList`

- <span id="cratemetalist-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::MetaList`

- <span id="cratemetalist-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaList`

##### `impl Hash for crate::MetaList`

- <span id="cratemetalist-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::attr::MetaList`

- <span id="crateattrmetalist-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::MetaList`

- <span id="cratemetalist-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for MetaList`

##### `impl<T> Spanned for MetaList`

- <span id="metalist-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::MetaList`

- <span id="crateattrmetalist-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `MetaNameValue`

```rust
struct MetaNameValue {
    pub path: crate::path::Path,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- <span id="cratemetanamevalue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::MetaNameValue`

- <span id="cratemetanamevalue-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::MetaNameValue`

- <span id="cratemetanamevalue-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaNameValue`

##### `impl Hash for crate::MetaNameValue`

- <span id="cratemetanamevalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::MetaNameValue`

- <span id="cratemetanamevalue-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for MetaNameValue`

##### `impl<T> Spanned for MetaNameValue`

- <span id="metanamevalue-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `AttrStyle`

```rust
enum AttrStyle {
    Outer,
    Inner(token::Not),
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

- <span id="crateattrstyle-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::AttrStyle`

##### `impl Debug for crate::AttrStyle`

- <span id="crateattrstyle-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AttrStyle`

##### `impl Hash for crate::AttrStyle`

- <span id="crateattrstyle-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AttrStyle`

- <span id="crateattrstyle-eq"></span>`fn eq(&self, other: &Self) -> bool`

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

- <span id="meta-path"></span>`fn path(&self) -> &Path` — [`Path`](../index.md)

- <span id="meta-require-path-only"></span>`fn require_path_only(&self) -> Result<&Path>` — [`Result`](../index.md), [`Path`](../index.md)

- <span id="meta-require-list"></span>`fn require_list(&self) -> Result<&MetaList>` — [`Result`](../index.md), [`MetaList`](../index.md)

- <span id="meta-require-name-value"></span>`fn require_name_value(&self) -> Result<&MetaNameValue>` — [`Result`](../index.md), [`MetaNameValue`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::Meta`

- <span id="cratemeta-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Meta`

- <span id="cratemeta-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Meta`

##### `impl Hash for crate::Meta`

- <span id="cratemeta-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::attr::Meta`

- <span id="crateattrmeta-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Meta`

- <span id="cratemeta-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Meta`

##### `impl<T> Spanned for Meta`

- <span id="meta-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::attr::Meta`

- <span id="crateattrmeta-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Traits

### `FilterAttrs<'a>`

```rust
trait FilterAttrs<'a> { ... }
```

#### Required Methods

- `type Ret: 1`

- `fn outer(self) -> <Self as >::Ret`

- `fn inner(self) -> <Self as >::Ret`

