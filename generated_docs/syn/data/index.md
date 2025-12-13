*[syn](../index.md) / [data](index.md)*

---

# Module `data`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Variant`](#variant)
  - [`FieldsNamed`](#fieldsnamed)
  - [`FieldsUnnamed`](#fieldsunnamed)
  - [`Field`](#field)
  - [`Members`](#members)
- [Enums](#enums)
  - [`Fields`](#fields)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Variant`](#variant) | struct | An enum variant. |
| [`FieldsNamed`](#fieldsnamed) | struct | Named fields of a struct or struct variant such as `Point { x: f64, y: f64 }`. |
| [`FieldsUnnamed`](#fieldsunnamed) | struct | Unnamed fields of a tuple struct or tuple variant such as `Some(T)`. |
| [`Field`](#field) | struct | A field of a struct or enum variant. |
| [`Members`](#members) | struct |  |
| [`Fields`](#fields) | enum | Data stored within an enum variant or struct. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Variant`

```rust
struct Variant {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub fields: Fields,
    pub discriminant: Option<(token::Eq, crate::expr::Expr)>,
}
```

*Defined in [`syn-2.0.111/src/data.rs:9-24`](../../../.source_1765633015/syn-2.0.111/src/data.rs#L9-L24)*

An enum variant.

#### Fields

- **`ident`**: `crate::ident::Ident`

  Name of the variant.

- **`fields`**: `Fields`

  Content stored in the variant.

- **`discriminant`**: `Option<(token::Eq, crate::expr::Expr)>`

  Explicit discriminant: `Variant = 1`

#### Trait Implementations

##### `impl Any for Variant`

- <span id="variant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Variant`

- <span id="variant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Variant`

- <span id="variant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Variant`

- <span id="cratevariant-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Variant`

- <span id="variant-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Variant`

- <span id="cratevariant-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variant`

##### `impl<T> From for Variant`

- <span id="variant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Variant`

- <span id="cratevariant-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Variant`

- <span id="variant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::data::Variant`

- <span id="cratedatavariant-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Variant`

- <span id="cratevariant-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Variant`

##### `impl Spanned for Variant`

- <span id="variant-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Variant`

- <span id="variant-toowned-type-owned"></span>`type Owned = T`

- <span id="variant-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variant-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::Variant`

- <span id="cratedatavariant-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Variant`

- <span id="variant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Variant`

- <span id="variant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldsNamed`

```rust
struct FieldsNamed {
    pub brace_token: token::Brace,
    pub named: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/data.rs:48-56`](../../../.source_1765633015/syn-2.0.111/src/data.rs#L48-L56)*

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

#### Implementations

- <span id="cratefieldsnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for FieldsNamed`

- <span id="fieldsnamed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldsNamed`

- <span id="fieldsnamed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldsNamed`

- <span id="fieldsnamed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldsNamed`

- <span id="cratefieldsnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldsNamed`

- <span id="fieldsnamed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldsNamed`

- <span id="cratefieldsnamed-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsNamed`

##### `impl<T> From for FieldsNamed`

- <span id="fieldsnamed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldsNamed`

- <span id="cratefieldsnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldsNamed`

- <span id="fieldsnamed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::FieldsNamed`

- <span id="cratefieldsnamed-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldsNamed`

##### `impl Spanned for FieldsNamed`

- <span id="fieldsnamed-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldsNamed`

- <span id="fieldsnamed-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldsnamed-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldsnamed-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldsNamed`

- <span id="fieldsnamed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldsnamed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldsNamed`

- <span id="fieldsnamed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldsnamed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldsUnnamed`

```rust
struct FieldsUnnamed {
    pub paren_token: token::Paren,
    pub unnamed: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/data.rs:58-65`](../../../.source_1765633015/syn-2.0.111/src/data.rs#L58-L65)*

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

#### Implementations

- <span id="cratefieldsunnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for FieldsUnnamed`

- <span id="fieldsunnamed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldsUnnamed`

- <span id="fieldsunnamed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldsUnnamed`

- <span id="fieldsunnamed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldsUnnamed`

- <span id="fieldsunnamed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsUnnamed`

##### `impl<T> From for FieldsUnnamed`

- <span id="fieldsunnamed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldsUnnamed`

- <span id="fieldsunnamed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldsUnnamed`

##### `impl Spanned for FieldsUnnamed`

- <span id="fieldsunnamed-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldsUnnamed`

- <span id="fieldsunnamed-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldsunnamed-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldsunnamed-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldsUnnamed`

- <span id="fieldsunnamed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldsunnamed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldsUnnamed`

- <span id="fieldsunnamed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldsunnamed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Field`

```rust
struct Field {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub mutability: crate::restriction::FieldMutability,
    pub ident: Option<crate::ident::Ident>,
    pub colon_token: Option<token::Colon>,
    pub ty: crate::ty::Type,
}
```

*Defined in [`syn-2.0.111/src/data.rs:181-200`](../../../.source_1765633015/syn-2.0.111/src/data.rs#L181-L200)*

A field of a struct or enum variant.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  Name of the field, if any.
  
  Fields of tuple structs have no names.

#### Implementations

- <span id="cratedatafield-parse-named"></span>`fn parse_named(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses a named (braced struct) field.

- <span id="cratedatafield-parse-unnamed"></span>`fn parse_unnamed(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parses an unnamed (tuple struct) field.

#### Trait Implementations

##### `impl Any for Field`

- <span id="field-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Field`

- <span id="field-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Field`

- <span id="field-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Field`

- <span id="cratefield-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Field`

- <span id="field-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Field`

- <span id="cratefield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Field`

##### `impl<T> From for Field`

- <span id="field-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Field`

- <span id="cratefield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Field`

- <span id="field-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Field`

- <span id="cratefield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Field`

##### `impl Spanned for Field`

- <span id="field-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Field`

- <span id="field-toowned-type-owned"></span>`type Owned = T`

- <span id="field-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="field-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::data::Field`

- <span id="cratedatafield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Field`

- <span id="field-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="field-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Field`

- <span id="field-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="field-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Members<'a>`

```rust
struct Members<'a> {
    fields: punctuated::Iter<'a, Field>,
    index: u32,
}
```

*Defined in [`syn-2.0.111/src/data.rs:202-205`](../../../.source_1765633015/syn-2.0.111/src/data.rs#L202-L205)*

#### Trait Implementations

##### `impl Any for Members<'a>`

- <span id="members-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Members<'a>`

- <span id="members-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Members<'a>`

- <span id="members-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Members<'a>`

- <span id="members-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Members<'a>`

- <span id="members-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Members<'a>`

- <span id="members-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Members<'a>`

- <span id="members-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Members<'a>`

- <span id="members-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="members-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="members-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Members<'a>`

- <span id="members-iterator-type-item"></span>`type Item = Member`

- <span id="members-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for Members<'a>`

- <span id="members-toowned-type-owned"></span>`type Owned = T`

- <span id="members-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="members-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Members<'a>`

- <span id="members-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="members-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Members<'a>`

- <span id="members-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="members-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Fields`

```rust
enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
```

*Defined in [`syn-2.0.111/src/data.rs:26-46`](../../../.source_1765633015/syn-2.0.111/src/data.rs#L26-L46)*

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

- <span id="fields-iter"></span>`fn iter(&self) -> punctuated::Iter<'_, Field>` — [`Iter`](../punctuated/index.md#iter), [`Field`](#field)

  Get an iterator over the borrowed [`Field`](#field) items in this object. This

  iterator can be used to iterate over a named or unnamed struct or

  variant's fields uniformly.

- <span id="fields-iter-mut"></span>`fn iter_mut(&mut self) -> punctuated::IterMut<'_, Field>` — [`IterMut`](../punctuated/index.md#itermut), [`Field`](#field)

  Get an iterator over the mutably borrowed [`Field`](#field) items in this

  object. This iterator can be used to iterate over a named or unnamed

  struct or variant's fields uniformly.

- <span id="fields-len"></span>`fn len(&self) -> usize`

  Returns the number of fields.

- <span id="fields-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if there are zero fields.

- <span id="fields-members"></span>`fn members(&self) -> Members<'_>` — [`Members`](#members)

  Get an iterator over the fields of a struct or variant as [`Member`](../expr/index.md)s.

  This iterator can be used to iterate over a named or unnamed struct or

  variant's fields uniformly.

  

  # Example

  

  The following is a simplistic [`Clone`](../../fs_err/index.md) derive for structs. (A more

  complete implementation would additionally want to infer trait bounds on

  the generic type parameters.)

  

  ```rust

  use quote::quote;

  

  fn derive_clone(input: &syn::ItemStruct) -> proc_macro2::TokenStream {

      let ident = &input.ident;

      let members = input.fields.members();

      let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

      quote! {

          impl #impl_generics Clone for #ident #ty_generics #where_clause {

              fn clone(&self) -> Self {

                  Self {

                      #(#members: self.#members.clone()),*

                  }

              }

          }

      }

  }

  ```

  

  For structs with named fields, it produces an expression like `Self { a:

  self.a.clone() }`. For structs with unnamed fields, `Self { 0:

  self.0.clone() }`. And for unit structs, `Self {}`.

#### Trait Implementations

##### `impl Any for Fields`

- <span id="fields-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fields`

- <span id="fields-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fields`

- <span id="fields-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Fields`

- <span id="cratefields-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Fields`

- <span id="fields-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Fields`

- <span id="cratefields-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Fields`

##### `impl<T> From for Fields`

- <span id="fields-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Fields`

- <span id="cratefields-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Fields`

- <span id="fields-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Fields`

- <span id="fields-intoiterator-type-item"></span>`type Item = Field`

- <span id="fields-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<Field>`

- <span id="fields-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for crate::Fields`

- <span id="cratefields-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Fields`

##### `impl Spanned for Fields`

- <span id="fields-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Fields`

- <span id="fields-toowned-type-owned"></span>`type Owned = T`

- <span id="fields-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fields-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Fields`

- <span id="fields-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Fields`

- <span id="fields-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fields-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fields`

- <span id="fields-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fields-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

