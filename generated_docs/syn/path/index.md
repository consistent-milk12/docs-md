*[syn](../index.md) / [path](index.md)*

---

# Module `path`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Path`](#path)
  - [`PathSegment`](#pathsegment)
  - [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments)
  - [`AssocType`](#assoctype)
  - [`AssocConst`](#assocconst)
  - [`Constraint`](#constraint)
  - [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments)
  - [`QSelf`](#qself)
- [Enums](#enums)
  - [`PathArguments`](#patharguments)
  - [`GenericArgument`](#genericargument)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Path`](#path) | struct | A path at which a named item is exported (e.g. `std::collections::HashMap`). |
| [`PathSegment`](#pathsegment) | struct | A segment of a path together with any path arguments on that segment. |
| [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments) | struct | Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K, V>`. |
| [`AssocType`](#assoctype) | struct | A binding (equality constraint) on an associated type: the `Item = u8` in `Iterator<Item = u8>`. |
| [`AssocConst`](#assocconst) | struct | An equality constraint on an associated constant: the `PANIC = false` in `Trait<PANIC = false>`. |
| [`Constraint`](#constraint) | struct | An associated type bound: `Iterator<Item: Display>`. |
| [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments) | struct | Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) -> C`. |
| [`QSelf`](#qself) | struct | The explicit Self type in a qualified path: the `T` in `<T as Display>::fmt`. |
| [`PathArguments`](#patharguments) | enum | Angle bracketed or parenthesized arguments of a path segment. |
| [`GenericArgument`](#genericargument) | enum | An individual generic argument, like `'a`, `T`, or `Item = T`. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Path`

```rust
struct Path {
    pub leading_colon: Option<token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, token::PathSep>,
}
```

*Defined in [`syn-2.0.111/src/path.rs:11-18`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L11-L18)*

A path at which a named item is exported (e.g. `std::collections::HashMap`).

#### Implementations

- <span id="cratepathpath-parse-mod-style"></span>`fn parse_mod_style(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a `Path` containing no path arguments on any of its segments.

  

  # Example

  

  ```rust

  use syn::{Path, Result, Token};

  use syn::parse::{Parse, ParseStream};

  

  // A simplified single `use` statement like:

  //

  //     use std::collections::HashMap;

  //

  // Note that generic parameters are not allowed in a `use` statement

  // so the following must not be accepted.

  //

  //     use a::<b>::c;

  struct SingleUse {

      use_token: Token![use],

      path: Path,

  }

  

  impl Parse for SingleUse {

      fn parse(input: ParseStream) -> Result<Self> {

          Ok(SingleUse {

              use_token: input.parse()?,

              path: input.call(Path::parse_mod_style)?,

          })

      }

  }

  ```

- <span id="cratepathpath-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratepathpath-parse-rest"></span>`fn parse_rest(input: ParseStream<'_>, path: &mut Self, expr_style: bool) -> Result<()>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratepathpath-is-mod-style"></span>`fn is_mod_style(&self) -> bool`

#### Trait Implementations

##### `impl Any for Path`

- <span id="path-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Path`

- <span id="path-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Path`

- <span id="path-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Path`

- <span id="cratepath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Path`

- <span id="path-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Path`

- <span id="cratepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Path`

##### `impl<T> From for Path`

- <span id="path-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Path`

- <span id="cratepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Path`

- <span id="path-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::Path`

- <span id="cratepathpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Path`

- <span id="cratepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialEq for syn::Path`

##### `impl Sealed for Path`

##### `impl Spanned for Path`

- <span id="path-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Path`

- <span id="path-toowned-type-owned"></span>`type Owned = T`

- <span id="path-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="path-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::Path`

- <span id="cratepathpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Path`

- <span id="path-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="path-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Path`

- <span id="path-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="path-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

*Defined in [`syn-2.0.111/src/path.rs:107-114`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L107-L114)*

A segment of a path together with any path arguments on that segment.

#### Implementations

- <span id="cratepathpathsegment-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for PathSegment`

- <span id="pathsegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathSegment`

- <span id="pathsegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathSegment`

- <span id="pathsegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PathSegment`

- <span id="cratepathsegment-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PathSegment`

- <span id="pathsegment-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PathSegment`

- <span id="cratepathsegment-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PathSegment`

##### `impl<T> From for PathSegment`

- <span id="pathsegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PathSegment`

- <span id="cratepathsegment-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PathSegment`

- <span id="pathsegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::PathSegment`

- <span id="cratepathpathsegment-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PathSegment`

- <span id="cratepathsegment-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PathSegment`

##### `impl Spanned for PathSegment`

- <span id="pathsegment-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PathSegment`

- <span id="pathsegment-toowned-type-owned"></span>`type Owned = T`

- <span id="pathsegment-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pathsegment-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::PathSegment`

- <span id="cratepathpathsegment-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PathSegment`

- <span id="pathsegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathsegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathSegment`

- <span id="pathsegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathsegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AngleBracketedGenericArguments`

```rust
struct AngleBracketedGenericArguments {
    pub colon2_token: Option<token::PathSep>,
    pub lt_token: token::Lt,
    pub args: crate::punctuated::Punctuated<GenericArgument, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/path.rs:196-206`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L196-L206)*

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

#### Implementations

- <span id="cratepathanglebracketedgenericarguments-parse-turbofish"></span>`fn parse_turbofish(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse `::<…>` with mandatory leading `::`.

  

  The ordinary [`Parse`](../parse/index.md) impl for `AngleBracketedGenericArguments`

  parses optional leading `::`.

- <span id="cratepathanglebracketedgenericarguments-do-parse"></span>`fn do_parse(colon2_token: Option<token::PathSep>, input: ParseStream<'_>) -> Result<Self>` — [`PathSep`](../token/index.md#pathsep), [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AngleBracketedGenericArguments`

##### `impl<T> From for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AngleBracketedGenericArguments`

##### `impl Spanned for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-toowned-type-owned"></span>`type Owned = T`

- <span id="anglebracketedgenericarguments-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="anglebracketedgenericarguments-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anglebracketedgenericarguments-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anglebracketedgenericarguments-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AssocType`

```rust
struct AssocType {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
}
```

*Defined in [`syn-2.0.111/src/path.rs:208-218`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L208-L218)*

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

#### Trait Implementations

##### `impl Any for AssocType`

- <span id="assoctype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssocType`

- <span id="assoctype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssocType`

- <span id="assoctype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AssocType`

- <span id="crateassoctype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AssocType`

- <span id="assoctype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::AssocType`

- <span id="crateassoctype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocType`

##### `impl<T> From for AssocType`

- <span id="assoctype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AssocType`

- <span id="crateassoctype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AssocType`

- <span id="assoctype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::AssocType`

- <span id="crateassoctype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AssocType`

##### `impl Spanned for AssocType`

- <span id="assoctype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AssocType`

- <span id="assoctype-toowned-type-owned"></span>`type Owned = T`

- <span id="assoctype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="assoctype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::AssocType`

- <span id="cratepathassoctype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for AssocType`

- <span id="assoctype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assoctype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssocType`

- <span id="assoctype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assoctype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AssocConst`

```rust
struct AssocConst {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

*Defined in [`syn-2.0.111/src/path.rs:220-230`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L220-L230)*

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

#### Trait Implementations

##### `impl Any for AssocConst`

- <span id="assocconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssocConst`

- <span id="assocconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssocConst`

- <span id="assocconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::AssocConst`

- <span id="crateassocconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AssocConst`

- <span id="assocconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::AssocConst`

- <span id="crateassocconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocConst`

##### `impl<T> From for AssocConst`

- <span id="assocconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::AssocConst`

- <span id="crateassocconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for AssocConst`

- <span id="assocconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::AssocConst`

- <span id="crateassocconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for AssocConst`

##### `impl Spanned for AssocConst`

- <span id="assocconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AssocConst`

- <span id="assocconst-toowned-type-owned"></span>`type Owned = T`

- <span id="assocconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="assocconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::AssocConst`

- <span id="cratepathassocconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for AssocConst`

- <span id="assocconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assocconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssocConst`

- <span id="assocconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assocconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Constraint`

```rust
struct Constraint {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/path.rs:232-241`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L232-L241)*

An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Any for Constraint`

- <span id="constraint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Constraint`

- <span id="constraint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Constraint`

- <span id="constraint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Constraint`

- <span id="crateconstraint-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Constraint`

- <span id="constraint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Constraint`

- <span id="crateconstraint-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Constraint`

##### `impl<T> From for Constraint`

- <span id="constraint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Constraint`

- <span id="crateconstraint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Constraint`

- <span id="constraint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Constraint`

- <span id="crateconstraint-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Constraint`

##### `impl Spanned for Constraint`

- <span id="constraint-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Constraint`

- <span id="constraint-toowned-type-owned"></span>`type Owned = T`

- <span id="constraint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constraint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::Constraint`

- <span id="cratepathconstraint-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Constraint`

- <span id="constraint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constraint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Constraint`

- <span id="constraint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constraint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParenthesizedGenericArguments`

```rust
struct ParenthesizedGenericArguments {
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<crate::ty::Type, token::Comma>,
    pub output: crate::ty::ReturnType,
}
```

*Defined in [`syn-2.0.111/src/path.rs:243-254`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L243-L254)*

Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
C`.

#### Fields

- **`inputs`**: `crate::punctuated::Punctuated<crate::ty::Type, token::Comma>`

  `(A, B)`

- **`output`**: `crate::ty::ReturnType`

  `C`

#### Implementations

- <span id="crateparenthesizedgenericarguments-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ParenthesizedGenericArguments`

##### `impl<T> From for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ParenthesizedGenericArguments`

##### `impl Spanned for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-toowned-type-owned"></span>`type Owned = T`

- <span id="parenthesizedgenericarguments-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parenthesizedgenericarguments-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parenthesizedgenericarguments-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parenthesizedgenericarguments-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `QSelf`

```rust
struct QSelf {
    pub lt_token: token::Lt,
    pub ty: Box<crate::ty::Type>,
    pub position: usize,
    pub as_token: Option<token::As>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/path.rs:256-281`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L256-L281)*

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

##### `impl Any for QSelf`

- <span id="qself-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for QSelf`

- <span id="qself-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for QSelf`

- <span id="qself-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::QSelf`

- <span id="crateqself-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for QSelf`

- <span id="qself-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::QSelf`

- <span id="crateqself-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::QSelf`

##### `impl<T> From for QSelf`

- <span id="qself-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::QSelf`

- <span id="crateqself-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for QSelf`

- <span id="qself-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::QSelf`

- <span id="crateqself-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::QSelf`

##### `impl Spanned for crate::path::QSelf`

- <span id="cratepathqself-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for QSelf`

- <span id="qself-toowned-type-owned"></span>`type Owned = T`

- <span id="qself-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="qself-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for QSelf`

- <span id="qself-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="qself-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for QSelf`

- <span id="qself-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="qself-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `PathArguments`

```rust
enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
```

*Defined in [`syn-2.0.111/src/path.rs:128-146`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L128-L146)*

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

- <span id="patharguments-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="patharguments-is-none"></span>`fn is_none(&self) -> bool`

#### Trait Implementations

##### `impl Any for PathArguments`

- <span id="patharguments-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathArguments`

- <span id="patharguments-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathArguments`

- <span id="patharguments-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PathArguments`

- <span id="cratepatharguments-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PathArguments`

- <span id="patharguments-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PathArguments`

- <span id="cratepatharguments-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathArguments`

- <span id="patharguments-default"></span>`fn default() -> Self`

##### `impl Eq for crate::PathArguments`

##### `impl<T> From for PathArguments`

- <span id="patharguments-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PathArguments`

- <span id="cratepatharguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PathArguments`

- <span id="patharguments-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PathArguments`

- <span id="cratepatharguments-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PathArguments`

##### `impl Spanned for PathArguments`

- <span id="patharguments-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PathArguments`

- <span id="patharguments-toowned-type-owned"></span>`type Owned = T`

- <span id="patharguments-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patharguments-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::PathArguments`

- <span id="cratepathpatharguments-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PathArguments`

- <span id="patharguments-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patharguments-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathArguments`

- <span id="patharguments-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patharguments-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/path.rs:171-194`](../../../.source_1765521767/syn-2.0.111/src/path.rs#L171-L194)*

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

##### `impl Any for GenericArgument`

- <span id="genericargument-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericArgument`

- <span id="genericargument-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericArgument`

- <span id="genericargument-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::GenericArgument`

- <span id="crategenericargument-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for GenericArgument`

- <span id="genericargument-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::GenericArgument`

- <span id="crategenericargument-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericArgument`

##### `impl<T> From for GenericArgument`

- <span id="genericargument-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::GenericArgument`

- <span id="crategenericargument-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for GenericArgument`

- <span id="genericargument-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::path::GenericArgument`

- <span id="cratepathgenericargument-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::GenericArgument`

- <span id="crategenericargument-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for GenericArgument`

##### `impl Spanned for GenericArgument`

- <span id="genericargument-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for GenericArgument`

- <span id="genericargument-toowned-type-owned"></span>`type Owned = T`

- <span id="genericargument-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericargument-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::path::GenericArgument`

- <span id="cratepathgenericargument-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for GenericArgument`

- <span id="genericargument-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericargument-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericArgument`

- <span id="genericargument-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericargument-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

