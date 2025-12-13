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
| [`AttrStyle`](#attrstyle) | enum | Distinguishes between attributes that decorate an item and attributes that are contained within an item. |
| [`Meta`](#meta) | enum | Content of a compile-time structured attribute. |
| [`FilterAttrs`](#filterattrs) | trait |  |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

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

*Defined in [`syn-2.0.111/src/attr.rs:19-179`](../../../.source_1765633015/syn-2.0.111/src/attr.rs#L19-L179)*

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

- <span id="attribute-path"></span>`fn path(&self) -> &Path` — [`Path`](../path/index.md#path)

  Returns the path that identifies the interpretation of this attribute.

  

  For example this would return the `test` in `#[test]`, the `derive` in

  `#[derive(Copy)]`, and the `path` in `#[path = "sys/windows.rs"]`.

- <span id="attribute-parse-args"></span>`fn parse_args<T: Parse>(&self) -> Result<T>` — [`Result`](../error/index.md#result)

  Parse the arguments to the attribute as a syntax tree.

  

  This is similar to pulling out the `TokenStream` from `Meta::List` and

  doing `syn::parse2::<T>(meta_list.tokens)`, except that using

  `parse_args` the error message has a more useful span when `tokens` is

  empty.

  

  The surrounding delimiters are *not* included in the input to the

  parser.

  

  ```text

  #[my_attr(value < 5)]

            ^^^^^^^^^ what gets parsed

  ```

  

  # Example

  

  ```rust

  use syn::{parse_quote, Attribute, Expr};

  

  let attr: Attribute = parse_quote! {

      #[precondition(value < 5)]

  };

  

  if attr.path().is_ident("precondition") {

      let precondition: Expr = attr.parse_args()?;

      // ...

  }

  anyhow::Ok(())

  ```

- <span id="attribute-parse-args-with"></span>`fn parse_args_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md#result), [`Parser`](../parse/index.md#parser)

  Parse the arguments to the attribute using the given parser.

  

  # Example

  

  ```rust

  use syn::{parse_quote, Attribute};

  

  let attr: Attribute = parse_quote! {

      #[inception { #[brrrrrrraaaaawwwwrwrrrmrmrmmrmrmmmmm] }]

  };

  

  let bwom = attr.parse_args_with(Attribute::parse_outer)?;

  

  // Attribute does not have a Parse impl, so we couldn't directly do:

  // let bwom: Attribute = attr.parse_args()?;

  anyhow::Ok(())

  ```

- <span id="attribute-parse-nested-meta"></span>`fn parse_nested_meta(&self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](../meta/index.md#parsenestedmeta), [`Result`](../error/index.md#result)

  Parse the arguments to the attribute, expecting it to follow the

  conventional structure used by most of Rust's built-in attributes.

  

  The [*Meta Item Attribute Syntax*][`syntax`](../../regex_automata/util/syntax/index.md) section in the Rust reference

  explains the convention in more detail. Not all attributes follow this

  convention, so `parse_args()` is available if you

  need to parse arbitrarily goofy attribute syntax.

  

  # Example

  

  We'll parse a struct, and then parse some of Rust's `#[repr]` attribute

  syntax.

  

  ```rust

  use syn::{parenthesized, parse_quote, token, ItemStruct, LitInt};

  

  let input: ItemStruct = parse_quote! {

      #[repr(C, align(4))]

      pub struct MyStruct(u16, u32);

  };

  

  let mut repr_c = false;

  let mut repr_transparent = false;

  let mut repr_align = None::<usize>;

  let mut repr_packed = None::<usize>;

  for attr in &input.attrs {

      if attr.path().is_ident("repr") {

          attr.parse_nested_meta(|meta| {

              // #[repr(C)]

              if meta.path.is_ident("C") {

                  repr_c = true;

                  return Ok(());

              }

  

              // #[repr(transparent)]

              if meta.path.is_ident("transparent") {

                  repr_transparent = true;

                  return Ok(());

              }

  

              // #[repr(align(N))]

              if meta.path.is_ident("align") {

                  let content;

                  parenthesized!(content in meta.input);

                  let lit: LitInt = content.parse()?;

                  let n: usize = lit.base10_parse()?;

                  repr_align = Some(n);

                  return Ok(());

              }

  

              // #[repr(packed)] or #[repr(packed(N))], omitted N means 1

              if meta.path.is_ident("packed") {

                  if meta.input.peek(token::Paren) {

                      let content;

                      parenthesized!(content in meta.input);

                      let lit: LitInt = content.parse()?;

                      let n: usize = lit.base10_parse()?;

                      repr_packed = Some(n);

                  } else {

                      repr_packed = Some(1);

                  }

                  return Ok(());

              }

  

              Err(meta.error("unrecognized repr"))

          })?;

      }

  }

  anyhow::Ok(())

  ```

  

  # Alternatives

  

  In some cases, for attributes which have nested layers of structured

  content, the following less flexible approach might be more convenient:

  

  ```rust

  use syn::{parse_quote, ItemStruct};

  

  let input: ItemStruct = parse_quote! {

      #[repr(C, align(4))]

      pub struct MyStruct(u16, u32);

  };

  

  use syn::punctuated::Punctuated;

  use syn::{parenthesized, token, Error, LitInt, Meta, Token};

  

  let mut repr_c = false;

  let mut repr_transparent = false;

  let mut repr_align = None::<usize>;

  let mut repr_packed = None::<usize>;

  for attr in &input.attrs {

      if attr.path().is_ident("repr") {

          let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

          for meta in nested {

              match meta {

                  // #[repr(C)]

                  Meta::Path(path) if path.is_ident("C") => {

                      repr_c = true;

                  }

  

                  // #[repr(align(N))]

                  Meta::List(meta) if meta.path.is_ident("align") => {

                      let lit: LitInt = meta.parse_args()?;

                      let n: usize = lit.base10_parse()?;

                      repr_align = Some(n);

                  }

  

                  /* ... */

  

                  _ => {

                      return Err(Error::new_spanned(meta, "unrecognized repr"));

                  }

              }

          }

      }

  }

  Ok(())

  ```

- <span id="attribute-parse-outer"></span>`fn parse_outer(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses zero or more outer attributes from the stream.

  

  # Example

  

  See

  [*Parsing from tokens to Attribute*](#parsing-from-tokens-to-attribute).

- <span id="attribute-parse-inner"></span>`fn parse_inner(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses zero or more inner attributes from the stream.

  

  # Example

  

  See

  [*Parsing from tokens to Attribute*](#parsing-from-tokens-to-attribute).

#### Trait Implementations

##### `impl Any for Attribute`

- <span id="attribute-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Attribute`

- <span id="attribute-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Attribute`

- <span id="attribute-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Attribute`

- <span id="crateattribute-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Attribute`

- <span id="attribute-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Attribute`

- <span id="crateattribute-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Attribute`

##### `impl<T> From for Attribute`

- <span id="attribute-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Attribute`

- <span id="crateattribute-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Attribute`

- <span id="attribute-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Attribute`

- <span id="crateattribute-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Attribute`

##### `impl Spanned for Attribute`

- <span id="attribute-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Attribute`

- <span id="attribute-toowned-type-owned"></span>`type Owned = T`

- <span id="attribute-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attribute-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::Attribute`

- <span id="crateattrattribute-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Attribute`

- <span id="attribute-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attribute-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Attribute`

- <span id="attribute-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attribute-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MetaList`

```rust
struct MetaList {
    pub path: crate::path::Path,
    pub delimiter: crate::mac::MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}
```

*Defined in [`syn-2.0.111/src/attr.rs:484-492`](../../../.source_1765633015/syn-2.0.111/src/attr.rs#L484-L492)*

A structured list within an attribute, like `derive(Copy, Clone)`.

#### Implementations

- <span id="metalist-parse-args"></span>`fn parse_args<T: Parse>(&self) -> Result<T>` — [`Result`](../error/index.md#result)

  See `Attribute::parse_args`.

- <span id="metalist-parse-args-with"></span>`fn parse_args_with<F: Parser>(&self, parser: F) -> Result<<F as >::Output>` — [`Result`](../error/index.md#result), [`Parser`](../parse/index.md#parser)

  See `Attribute::parse_args_with`.

- <span id="metalist-parse-nested-meta"></span>`fn parse_nested_meta(&self, logic: impl FnMut(ParseNestedMeta<'_>) -> Result<()>) -> Result<()>` — [`ParseNestedMeta`](../meta/index.md#parsenestedmeta), [`Result`](../error/index.md#result)

  See `Attribute::parse_nested_meta`.

#### Trait Implementations

##### `impl Any for MetaList`

- <span id="metalist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MetaList`

- <span id="metalist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MetaList`

- <span id="metalist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::MetaList`

- <span id="cratemetalist-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MetaList`

- <span id="metalist-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::MetaList`

- <span id="cratemetalist-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaList`

##### `impl<T> From for MetaList`

- <span id="metalist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::MetaList`

- <span id="cratemetalist-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for MetaList`

- <span id="metalist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::attr::MetaList`

- <span id="crateattrmetalist-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::MetaList`

- <span id="cratemetalist-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for MetaList`

##### `impl Spanned for MetaList`

- <span id="metalist-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for MetaList`

- <span id="metalist-toowned-type-owned"></span>`type Owned = T`

- <span id="metalist-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="metalist-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::MetaList`

- <span id="crateattrmetalist-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for MetaList`

- <span id="metalist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metalist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MetaList`

- <span id="metalist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metalist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MetaNameValue`

```rust
struct MetaNameValue {
    pub path: crate::path::Path,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

*Defined in [`syn-2.0.111/src/attr.rs:494-502`](../../../.source_1765633015/syn-2.0.111/src/attr.rs#L494-L502)*

A name-value pair within an attribute, like `feature = "nightly"`.

#### Implementations

- <span id="cratemetanamevalue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for MetaNameValue`

- <span id="metanamevalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MetaNameValue`

- <span id="metanamevalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MetaNameValue`

- <span id="metanamevalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::MetaNameValue`

- <span id="cratemetanamevalue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MetaNameValue`

- <span id="metanamevalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::MetaNameValue`

- <span id="cratemetanamevalue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::MetaNameValue`

##### `impl<T> From for MetaNameValue`

- <span id="metanamevalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::MetaNameValue`

- <span id="cratemetanamevalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for MetaNameValue`

- <span id="metanamevalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::MetaNameValue`

- <span id="cratemetanamevalue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for MetaNameValue`

##### `impl Spanned for MetaNameValue`

- <span id="metanamevalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for MetaNameValue`

- <span id="metanamevalue-toowned-type-owned"></span>`type Owned = T`

- <span id="metanamevalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="metanamevalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::MetaNameValue`

- <span id="crateattrmetanamevalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for MetaNameValue`

- <span id="metanamevalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metanamevalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MetaNameValue`

- <span id="metanamevalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metanamevalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `AttrStyle`

```rust
enum AttrStyle {
    Outer,
    Inner(token::Not),
}
```

*Defined in [`syn-2.0.111/src/attr.rs:429-449`](../../../.source_1765633015/syn-2.0.111/src/attr.rs#L429-L449)*

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

##### `impl Any for AttrStyle`

- <span id="attrstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttrStyle`

- <span id="attrstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttrStyle`

- <span id="attrstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AttrStyle`

- <span id="crateattrstyle-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AttrStyle`

- <span id="attrstyle-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::AttrStyle`

##### `impl Debug for crate::AttrStyle`

- <span id="crateattrstyle-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AttrStyle`

##### `impl<T> From for AttrStyle`

- <span id="attrstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AttrStyle`

- <span id="crateattrstyle-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AttrStyle`

- <span id="attrstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::AttrStyle`

- <span id="crateattrstyle-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for AttrStyle`

- <span id="attrstyle-toowned-type-owned"></span>`type Owned = T`

- <span id="attrstyle-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attrstyle-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttrStyle`

- <span id="attrstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attrstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttrStyle`

- <span id="attrstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attrstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Meta`

```rust
enum Meta {
    Path(crate::path::Path),
    List(MetaList),
    NameValue(MetaNameValue),
}
```

*Defined in [`syn-2.0.111/src/attr.rs:451-482`](../../../.source_1765633015/syn-2.0.111/src/attr.rs#L451-L482)*

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

- <span id="meta-path"></span>`fn path(&self) -> &Path` — [`Path`](../path/index.md#path)

  Returns the path that begins this structured meta item.

  

  For example this would return the `test` in `#[test]`, the `derive` in

  `#[derive(Copy)]`, and the `path` in `#[path = "sys/windows.rs"]`.

- <span id="meta-require-path-only"></span>`fn require_path_only(&self) -> Result<&Path>` — [`Result`](../error/index.md#result), [`Path`](../path/index.md#path)

  Error if this is a `Meta::List` or `Meta::NameValue`.

- <span id="meta-require-list"></span>`fn require_list(&self) -> Result<&MetaList>` — [`Result`](../error/index.md#result), [`MetaList`](#metalist)

  Error if this is a `Meta::Path` or `Meta::NameValue`.

- <span id="meta-require-name-value"></span>`fn require_name_value(&self) -> Result<&MetaNameValue>` — [`Result`](../error/index.md#result), [`MetaNameValue`](#metanamevalue)

  Error if this is a `Meta::Path` or `Meta::List`.

#### Trait Implementations

##### `impl Any for Meta`

- <span id="meta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Meta`

- <span id="meta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Meta`

- <span id="meta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Meta`

- <span id="cratemeta-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Meta`

- <span id="meta-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Meta`

- <span id="cratemeta-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Meta`

##### `impl<T> From for Meta`

- <span id="meta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Meta`

- <span id="cratemeta-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Meta`

- <span id="meta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::attr::Meta`

- <span id="crateattrmeta-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Meta`

- <span id="cratemeta-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Meta`

##### `impl Spanned for Meta`

- <span id="meta-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Meta`

- <span id="meta-toowned-type-owned"></span>`type Owned = T`

- <span id="meta-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="meta-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::attr::Meta`

- <span id="crateattrmeta-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Meta`

- <span id="meta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="meta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Meta`

- <span id="meta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="meta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `FilterAttrs<'a>`

```rust
trait FilterAttrs<'a> { ... }
```

*Defined in [`syn-2.0.111/src/attr.rs:594-600`](../../../.source_1765633015/syn-2.0.111/src/attr.rs#L594-L600)*

#### Associated Types

- `type Ret: 1`

#### Required Methods

- `fn outer(self) -> <Self as >::Ret`

- `fn inner(self) -> <Self as >::Ret`

#### Implementors

- `&'a [Attribute]`

