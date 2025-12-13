*[syn](../index.md) / [derive](index.md)*

---

# Module `derive`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`DeriveInput`](#deriveinput)
  - [`DataStruct`](#datastruct)
  - [`DataEnum`](#dataenum)
  - [`DataUnion`](#dataunion)
- [Enums](#enums)
  - [`Data`](#data)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`DeriveInput`](#deriveinput) | struct | Data structure sent to a `proc_macro_derive` macro. |
| [`DataStruct`](#datastruct) | struct | A struct input to a `proc_macro_derive` macro. |
| [`DataEnum`](#dataenum) | struct | An enum input to a `proc_macro_derive` macro. |
| [`DataUnion`](#dataunion) | struct | An untagged union input to a `proc_macro_derive` macro. |
| [`Data`](#data) | enum | The storage of a struct, enum or union data structure. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

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

*Defined in [`syn-2.0.111/src/derive.rs:9-19`](../../../.source_1765521767/syn-2.0.111/src/derive.rs#L9-L19)*

Data structure sent to a `proc_macro_derive` macro.

#### Trait Implementations

##### `impl Any for DeriveInput`

- <span id="deriveinput-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DeriveInput`

- <span id="deriveinput-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DeriveInput`

- <span id="deriveinput-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DeriveInput`

- <span id="cratederiveinput-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DeriveInput`

- <span id="deriveinput-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DeriveInput`

- <span id="cratederiveinput-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DeriveInput`

##### `impl<T> From for DeriveInput`

- <span id="deriveinput-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DeriveInput`

- <span id="cratederiveinput-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DeriveInput`

- <span id="deriveinput-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::derive::DeriveInput`

- <span id="cratederivederiveinput-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::DeriveInput`

- <span id="cratederiveinput-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for DeriveInput`

##### `impl Spanned for DeriveInput`

- <span id="deriveinput-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for DeriveInput`

- <span id="deriveinput-toowned-type-owned"></span>`type Owned = T`

- <span id="deriveinput-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="deriveinput-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::derive::DeriveInput`

- <span id="cratederivederiveinput-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for DeriveInput`

- <span id="deriveinput-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deriveinput-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DeriveInput`

- <span id="deriveinput-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deriveinput-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataStruct`

```rust
struct DataStruct {
    pub struct_token: token::Struct,
    pub fields: crate::data::Fields,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/derive.rs:37-45`](../../../.source_1765521767/syn-2.0.111/src/derive.rs#L37-L45)*

A struct input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedatastruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for DataStruct`

- <span id="datastruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataStruct`

- <span id="datastruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataStruct`

- <span id="datastruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DataStruct`

- <span id="cratedatastruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DataStruct`

- <span id="datastruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DataStruct`

- <span id="cratedatastruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataStruct`

##### `impl<T> From for DataStruct`

- <span id="datastruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DataStruct`

- <span id="cratedatastruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DataStruct`

- <span id="datastruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::DataStruct`

- <span id="cratedatastruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for DataStruct`

- <span id="datastruct-toowned-type-owned"></span>`type Owned = T`

- <span id="datastruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="datastruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataStruct`

- <span id="datastruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="datastruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataStruct`

- <span id="datastruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="datastruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataEnum`

```rust
struct DataEnum {
    pub enum_token: token::Enum,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/derive.rs:47-55`](../../../.source_1765521767/syn-2.0.111/src/derive.rs#L47-L55)*

An enum input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedataenum-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for DataEnum`

- <span id="dataenum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataEnum`

- <span id="dataenum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataEnum`

- <span id="dataenum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DataEnum`

- <span id="cratedataenum-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DataEnum`

- <span id="dataenum-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DataEnum`

- <span id="cratedataenum-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataEnum`

##### `impl<T> From for DataEnum`

- <span id="dataenum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DataEnum`

- <span id="cratedataenum-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DataEnum`

- <span id="dataenum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::DataEnum`

- <span id="cratedataenum-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for DataEnum`

- <span id="dataenum-toowned-type-owned"></span>`type Owned = T`

- <span id="dataenum-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dataenum-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataEnum`

- <span id="dataenum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dataenum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataEnum`

- <span id="dataenum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dataenum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataUnion`

```rust
struct DataUnion {
    pub union_token: token::Union,
    pub fields: crate::data::FieldsNamed,
}
```

*Defined in [`syn-2.0.111/src/derive.rs:57-64`](../../../.source_1765521767/syn-2.0.111/src/derive.rs#L57-L64)*

An untagged union input to a `proc_macro_derive` macro.

#### Implementations

- <span id="cratedataunion-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for DataUnion`

- <span id="dataunion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataUnion`

- <span id="dataunion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataUnion`

- <span id="dataunion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::DataUnion`

- <span id="cratedataunion-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DataUnion`

- <span id="dataunion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::DataUnion`

- <span id="cratedataunion-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataUnion`

##### `impl<T> From for DataUnion`

- <span id="dataunion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::DataUnion`

- <span id="cratedataunion-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for DataUnion`

- <span id="dataunion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::DataUnion`

- <span id="cratedataunion-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for DataUnion`

- <span id="dataunion-toowned-type-owned"></span>`type Owned = T`

- <span id="dataunion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dataunion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataUnion`

- <span id="dataunion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dataunion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataUnion`

- <span id="dataunion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dataunion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Data`

```rust
enum Data {
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}
```

*Defined in [`syn-2.0.111/src/derive.rs:21-35`](../../../.source_1765521767/syn-2.0.111/src/derive.rs#L21-L35)*

The storage of a struct, enum or union data structure.

# Syntax tree enum

This type is a [syntax tree enum].


#### Trait Implementations

##### `impl Any for Data`

- <span id="data-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Data`

- <span id="data-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Data`

- <span id="data-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Data`

- <span id="cratedata-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Data`

- <span id="data-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Data`

- <span id="cratedata-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Data`

##### `impl<T> From for Data`

- <span id="data-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Data`

- <span id="cratedata-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Data`

- <span id="data-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Data`

- <span id="cratedata-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for Data`

- <span id="data-toowned-type-owned"></span>`type Owned = T`

- <span id="data-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="data-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Data`

- <span id="data-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="data-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Data`

- <span id="data-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="data-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

