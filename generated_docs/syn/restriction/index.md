*[syn](../index.md) / [restriction](index.md)*

---

# Module `restriction`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`VisRestricted`](#visrestricted) | struct | A visibility level restricted to some path: `pub(self)` or `pub(super)` or `pub(crate)` or `pub(in some::module)`. |
| [`Visibility`](#visibility) | enum | The visibility level of an item: inherited or `pub` or `pub(restricted)`. |
| [`FieldMutability`](#fieldmutability) | enum | Unused, but reserved for RFC 3323 restrictions. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `VisRestricted`

```rust
struct VisRestricted {
    pub pub_token: token::Pub,
    pub paren_token: token::Paren,
    pub in_token: Option<token::In>,
    pub path: Box<crate::path::Path>,
}
```

*Defined in [`syn-2.0.111/src/restriction.rs:27-37`](../../../.source_1765521767/syn-2.0.111/src/restriction.rs#L27-L37)*

A visibility level restricted to some path: `pub(self)` or
`pub(super)` or `pub(crate)` or `pub(in some::module)`.

#### Implementations

- <span id="cratevisrestricted-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for VisRestricted`

- <span id="visrestricted-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VisRestricted`

- <span id="visrestricted-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VisRestricted`

- <span id="visrestricted-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::VisRestricted`

- <span id="cratevisrestricted-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for VisRestricted`

- <span id="visrestricted-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::VisRestricted`

- <span id="cratevisrestricted-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::VisRestricted`

##### `impl<T> From for VisRestricted`

- <span id="visrestricted-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::VisRestricted`

- <span id="cratevisrestricted-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for VisRestricted`

- <span id="visrestricted-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::VisRestricted`

- <span id="cratevisrestricted-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for VisRestricted`

##### `impl Spanned for VisRestricted`

- <span id="visrestricted-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for VisRestricted`

- <span id="visrestricted-toowned-type-owned"></span>`type Owned = T`

- <span id="visrestricted-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="visrestricted-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::restriction::VisRestricted`

- <span id="craterestrictionvisrestricted-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for VisRestricted`

- <span id="visrestricted-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="visrestricted-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VisRestricted`

- <span id="visrestricted-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="visrestricted-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Visibility`

```rust
enum Visibility {
    Public(token::Pub),
    Restricted(VisRestricted),
    Inherited,
}
```

*Defined in [`syn-2.0.111/src/restriction.rs:4-25`](../../../.source_1765521767/syn-2.0.111/src/restriction.rs#L4-L25)*

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

- <span id="craterestrictionvisibility-is-inherited"></span>`fn is_inherited(&self) -> bool`

#### Trait Implementations

##### `impl Any for Visibility`

- <span id="visibility-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Visibility`

- <span id="visibility-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Visibility`

- <span id="visibility-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Visibility`

- <span id="cratevisibility-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Visibility`

- <span id="visibility-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Visibility`

- <span id="cratevisibility-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Visibility`

##### `impl<T> From for Visibility`

- <span id="visibility-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Visibility`

- <span id="cratevisibility-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Visibility`

- <span id="visibility-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Visibility`

- <span id="cratevisibility-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Visibility`

##### `impl Spanned for Visibility`

- <span id="visibility-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Visibility`

- <span id="visibility-toowned-type-owned"></span>`type Owned = T`

- <span id="visibility-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="visibility-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Visibility`

- <span id="visibility-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="visibility-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Visibility`

- <span id="visibility-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="visibility-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldMutability`

```rust
enum FieldMutability {
    None,
}
```

*Defined in [`syn-2.0.111/src/restriction.rs:39-57`](../../../.source_1765521767/syn-2.0.111/src/restriction.rs#L39-L57)*

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Any for FieldMutability`

- <span id="fieldmutability-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldMutability`

- <span id="fieldmutability-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldMutability`

- <span id="fieldmutability-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldMutability`

- <span id="cratefieldmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldMutability`

- <span id="fieldmutability-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldMutability`

- <span id="cratefieldmutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldMutability`

##### `impl<T> From for FieldMutability`

- <span id="fieldmutability-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldMutability`

- <span id="cratefieldmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldMutability`

- <span id="fieldmutability-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::FieldMutability`

- <span id="cratefieldmutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for FieldMutability`

- <span id="fieldmutability-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldmutability-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldmutability-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FieldMutability`

- <span id="fieldmutability-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldmutability-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldMutability`

- <span id="fieldmutability-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldmutability-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

