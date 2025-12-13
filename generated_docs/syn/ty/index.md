*[syn](../index.md) / [ty](index.md)*

---

# Module `ty`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`TypeArray`](#typearray)
  - [`TypeBareFn`](#typebarefn)
  - [`TypeGroup`](#typegroup)
  - [`TypeImplTrait`](#typeimpltrait)
  - [`TypeInfer`](#typeinfer)
  - [`TypeMacro`](#typemacro)
  - [`TypeNever`](#typenever)
  - [`TypeParen`](#typeparen)
  - [`TypePath`](#typepath)
  - [`TypePtr`](#typeptr)
  - [`TypeReference`](#typereference)
  - [`TypeSlice`](#typeslice)
  - [`TypeTraitObject`](#typetraitobject)
  - [`TypeTuple`](#typetuple)
  - [`Abi`](#abi)
  - [`BareFnArg`](#barefnarg)
  - [`BareVariadic`](#barevariadic)
- [Enums](#enums)
  - [`Type`](#type)
  - [`ReturnType`](#returntype)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`TypeArray`](#typearray) | struct | A fixed size array type: `[T; n]`. |
| [`TypeBareFn`](#typebarefn) | struct | A bare function type: `fn(usize) -> bool`. |
| [`TypeGroup`](#typegroup) | struct | A type contained within invisible delimiters. |
| [`TypeImplTrait`](#typeimpltrait) | struct | An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or a lifetime. |
| [`TypeInfer`](#typeinfer) | struct | Indication that a type should be inferred by the compiler: `_`. |
| [`TypeMacro`](#typemacro) | struct | A macro in the type position. |
| [`TypeNever`](#typenever) | struct | The never type: `!`. |
| [`TypeParen`](#typeparen) | struct | A parenthesized type equivalent to the inner type. |
| [`TypePath`](#typepath) | struct | A path like `std::slice::Iter`, optionally qualified with a self-type as in `<Vec<T> as SomeTrait>::Associated`. |
| [`TypePtr`](#typeptr) | struct | A raw pointer type: `*const T` or `*mut T`. |
| [`TypeReference`](#typereference) | struct | A reference type: `&'a T` or `&'a mut T`. |
| [`TypeSlice`](#typeslice) | struct | A dynamically sized slice type: `[T]`. |
| [`TypeTraitObject`](#typetraitobject) | struct | A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a trait or a lifetime. |
| [`TypeTuple`](#typetuple) | struct | A tuple type: `(A, B, C, String)`. |
| [`Abi`](#abi) | struct | The binary interface of a function: `extern "C"`. |
| [`BareFnArg`](#barefnarg) | struct | An argument in a function type: the `usize` in `fn(usize) -> bool`. |
| [`BareVariadic`](#barevariadic) | struct | The variadic argument of a function pointer like `fn(usize, ...)`. |
| [`Type`](#type) | enum | The possible types that a Rust value could have. |
| [`ReturnType`](#returntype) | enum | Return type of a function signature. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `TypeArray`

```rust
struct TypeArray {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
    pub semi_token: token::Semi,
    pub len: crate::expr::Expr,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:92-101`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L92-L101)*

A fixed size array type: `[T; n]`.

#### Implementations

- <span id="cratetypearray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeArray`

- <span id="typearray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeArray`

- <span id="typearray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeArray`

- <span id="typearray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeArray`

- <span id="cratetypearray-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeArray`

- <span id="typearray-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeArray`

- <span id="cratetypearray-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeArray`

##### `impl<T> From for TypeArray`

- <span id="typearray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeArray`

- <span id="cratetypearray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeArray`

- <span id="typearray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeArray`

- <span id="cratetytypearray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeArray`

- <span id="cratetypearray-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeArray`

##### `impl Spanned for TypeArray`

- <span id="typearray-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeArray`

- <span id="typearray-toowned-type-owned"></span>`type Owned = T`

- <span id="typearray-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typearray-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeArray`

- <span id="cratetytypearray-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeArray`

- <span id="typearray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typearray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeArray`

- <span id="typearray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typearray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeBareFn`

```rust
struct TypeBareFn {
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: Option<Abi>,
    pub fn_token: token::Fn,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<BareFnArg, token::Comma>,
    pub variadic: Option<BareVariadic>,
    pub output: ReturnType,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:103-116`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L103-L116)*

A bare function type: `fn(usize) -> bool`.

#### Implementations

- <span id="cratetypebarefn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeBareFn`

- <span id="typebarefn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeBareFn`

- <span id="typebarefn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeBareFn`

- <span id="typebarefn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeBareFn`

- <span id="cratetypebarefn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeBareFn`

- <span id="typebarefn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeBareFn`

- <span id="cratetypebarefn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeBareFn`

##### `impl<T> From for TypeBareFn`

- <span id="typebarefn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeBareFn`

- <span id="cratetypebarefn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeBareFn`

- <span id="typebarefn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeBareFn`

- <span id="cratetypebarefn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeBareFn`

##### `impl Spanned for TypeBareFn`

- <span id="typebarefn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeBareFn`

- <span id="typebarefn-toowned-type-owned"></span>`type Owned = T`

- <span id="typebarefn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typebarefn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeBareFn`

- <span id="typebarefn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typebarefn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeBareFn`

- <span id="typebarefn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typebarefn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeGroup`

```rust
struct TypeGroup {
    pub group_token: token::Group,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:118-125`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L118-L125)*

A type contained within invisible delimiters.

#### Implementations

- <span id="cratetypegroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeGroup`

- <span id="typegroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeGroup`

- <span id="typegroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeGroup`

- <span id="typegroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeGroup`

- <span id="cratetypegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeGroup`

- <span id="typegroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeGroup`

- <span id="cratetypegroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeGroup`

##### `impl<T> From for TypeGroup`

- <span id="typegroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeGroup`

- <span id="cratetypegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeGroup`

- <span id="typegroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeGroup`

- <span id="cratetytypegroup-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeGroup`

- <span id="cratetypegroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeGroup`

##### `impl Spanned for TypeGroup`

- <span id="typegroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeGroup`

- <span id="typegroup-toowned-type-owned"></span>`type Owned = T`

- <span id="typegroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typegroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeGroup`

- <span id="cratetytypegroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeGroup`

- <span id="typegroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typegroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeGroup`

- <span id="typegroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typegroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeImplTrait`

```rust
struct TypeImplTrait {
    pub impl_token: token::Impl,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:127-135`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L127-L135)*

An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
a lifetime.

#### Implementations

- <span id="cratetytypeimpltrait-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for TypeImplTrait`

- <span id="typeimpltrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeImplTrait`

- <span id="typeimpltrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeImplTrait`

- <span id="typeimpltrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeImplTrait`

- <span id="typeimpltrait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeImplTrait`

##### `impl<T> From for TypeImplTrait`

- <span id="typeimpltrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeImplTrait`

- <span id="typeimpltrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeImplTrait`

##### `impl Spanned for TypeImplTrait`

- <span id="typeimpltrait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeImplTrait`

- <span id="typeimpltrait-toowned-type-owned"></span>`type Owned = T`

- <span id="typeimpltrait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeimpltrait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeImplTrait`

- <span id="typeimpltrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeimpltrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeImplTrait`

- <span id="typeimpltrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeimpltrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeInfer`

```rust
struct TypeInfer {
    pub underscore_token: token::Underscore,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:137-143`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L137-L143)*

Indication that a type should be inferred by the compiler: `_`.

#### Implementations

- <span id="cratetypeinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeInfer`

- <span id="typeinfer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeInfer`

- <span id="typeinfer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeInfer`

- <span id="typeinfer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeInfer`

- <span id="cratetypeinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeInfer`

- <span id="typeinfer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeInfer`

- <span id="cratetypeinfer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeInfer`

##### `impl<T> From for TypeInfer`

- <span id="typeinfer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeInfer`

- <span id="cratetypeinfer-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for TypeInfer`

- <span id="typeinfer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeInfer`

- <span id="cratetypeinfer-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for TypeInfer`

##### `impl Spanned for TypeInfer`

- <span id="typeinfer-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeInfer`

- <span id="typeinfer-toowned-type-owned"></span>`type Owned = T`

- <span id="typeinfer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeinfer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeInfer`

- <span id="typeinfer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeinfer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeInfer`

- <span id="typeinfer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeinfer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeMacro`

```rust
struct TypeMacro {
    pub mac: crate::mac::Macro,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:145-151`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L145-L151)*

A macro in the type position.

#### Implementations

- <span id="cratetypemacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeMacro`

- <span id="typemacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeMacro`

- <span id="typemacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeMacro`

- <span id="typemacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeMacro`

- <span id="cratetypemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeMacro`

- <span id="typemacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeMacro`

- <span id="cratetypemacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeMacro`

##### `impl<T> From for TypeMacro`

- <span id="typemacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeMacro`

- <span id="cratetypemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeMacro`

- <span id="typemacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeMacro`

- <span id="cratetytypemacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeMacro`

- <span id="cratetypemacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeMacro`

##### `impl Spanned for TypeMacro`

- <span id="typemacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeMacro`

- <span id="typemacro-toowned-type-owned"></span>`type Owned = T`

- <span id="typemacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typemacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeMacro`

- <span id="cratetytypemacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeMacro`

- <span id="typemacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typemacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeMacro`

- <span id="typemacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typemacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeNever`

```rust
struct TypeNever {
    pub bang_token: token::Not,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:153-159`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L153-L159)*

The never type: `!`.

#### Implementations

- <span id="cratetypenever-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeNever`

- <span id="typenever-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeNever`

- <span id="typenever-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeNever`

- <span id="typenever-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeNever`

- <span id="cratetypenever-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeNever`

- <span id="typenever-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeNever`

- <span id="cratetypenever-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeNever`

##### `impl<T> From for TypeNever`

- <span id="typenever-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeNever`

- <span id="cratetypenever-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for TypeNever`

- <span id="typenever-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeNever`

- <span id="cratetytypenever-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeNever`

- <span id="cratetypenever-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for TypeNever`

##### `impl Spanned for TypeNever`

- <span id="typenever-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeNever`

- <span id="typenever-toowned-type-owned"></span>`type Owned = T`

- <span id="typenever-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typenever-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeNever`

- <span id="cratetytypenever-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeNever`

- <span id="typenever-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typenever-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeNever`

- <span id="typenever-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typenever-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParen`

```rust
struct TypeParen {
    pub paren_token: token::Paren,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:161-168`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L161-L168)*

A parenthesized type equivalent to the inner type.

#### Implementations

- <span id="cratetytypeparen-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for TypeParen`

- <span id="typeparen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParen`

- <span id="typeparen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParen`

- <span id="typeparen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeParen`

- <span id="cratetypeparen-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeParen`

- <span id="typeparen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeParen`

- <span id="cratetypeparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParen`

##### `impl<T> From for TypeParen`

- <span id="typeparen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeParen`

- <span id="cratetypeparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeParen`

- <span id="typeparen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeParen`

- <span id="cratetytypeparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeParen`

- <span id="cratetypeparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParen`

##### `impl Spanned for TypeParen`

- <span id="typeparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeParen`

- <span id="typeparen-toowned-type-owned"></span>`type Owned = T`

- <span id="typeparen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeparen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeParen`

- <span id="cratetytypeparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeParen`

- <span id="typeparen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParen`

- <span id="typeparen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypePath`

```rust
struct TypePath {
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:170-178`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L170-L178)*

A path like `std::slice::Iter`, optionally qualified with a
self-type as in `<Vec<T> as SomeTrait>::Associated`.

#### Implementations

- <span id="cratetypepath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypePath`

- <span id="typepath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypePath`

- <span id="typepath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypePath`

- <span id="typepath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypePath`

- <span id="cratetypepath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypePath`

- <span id="typepath-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypePath`

- <span id="cratetypepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePath`

##### `impl<T> From for TypePath`

- <span id="typepath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypePath`

- <span id="cratetypepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypePath`

- <span id="typepath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypePath`

- <span id="cratetytypepath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypePath`

- <span id="cratetypepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypePath`

##### `impl Spanned for TypePath`

- <span id="typepath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypePath`

- <span id="typepath-toowned-type-owned"></span>`type Owned = T`

- <span id="typepath-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typepath-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypePath`

- <span id="cratetytypepath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypePath`

- <span id="typepath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typepath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypePath`

- <span id="typepath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typepath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypePtr`

```rust
struct TypePtr {
    pub star_token: token::Star,
    pub const_token: Option<token::Const>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:180-189`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L180-L189)*

A raw pointer type: `*const T` or `*mut T`.

#### Implementations

- <span id="cratetypeptr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypePtr`

- <span id="typeptr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypePtr`

- <span id="typeptr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypePtr`

- <span id="typeptr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypePtr`

- <span id="cratetypeptr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypePtr`

- <span id="typeptr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypePtr`

- <span id="cratetypeptr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePtr`

##### `impl<T> From for TypePtr`

- <span id="typeptr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypePtr`

- <span id="cratetypeptr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypePtr`

- <span id="typeptr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypePtr`

- <span id="cratetytypeptr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypePtr`

- <span id="cratetypeptr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypePtr`

##### `impl Spanned for TypePtr`

- <span id="typeptr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypePtr`

- <span id="typeptr-toowned-type-owned"></span>`type Owned = T`

- <span id="typeptr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeptr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypePtr`

- <span id="cratetytypeptr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypePtr`

- <span id="typeptr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeptr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypePtr`

- <span id="typeptr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeptr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeReference`

```rust
struct TypeReference {
    pub and_token: token::And,
    pub lifetime: Option<crate::lifetime::Lifetime>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:191-200`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L191-L200)*

A reference type: `&'a T` or `&'a mut T`.

#### Implementations

- <span id="cratetypereference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeReference`

- <span id="typereference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeReference`

- <span id="typereference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeReference`

- <span id="typereference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeReference`

- <span id="cratetypereference-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeReference`

- <span id="typereference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeReference`

- <span id="cratetypereference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeReference`

##### `impl<T> From for TypeReference`

- <span id="typereference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeReference`

- <span id="cratetypereference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeReference`

- <span id="typereference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeReference`

- <span id="cratetytypereference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeReference`

- <span id="cratetypereference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeReference`

##### `impl Spanned for TypeReference`

- <span id="typereference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeReference`

- <span id="typereference-toowned-type-owned"></span>`type Owned = T`

- <span id="typereference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typereference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeReference`

- <span id="cratetytypereference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeReference`

- <span id="typereference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typereference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeReference`

- <span id="typereference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typereference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeSlice`

```rust
struct TypeSlice {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:202-209`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L202-L209)*

A dynamically sized slice type: `[T]`.

#### Implementations

- <span id="cratetypeslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeSlice`

- <span id="typeslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeSlice`

- <span id="typeslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeSlice`

- <span id="typeslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeSlice`

- <span id="cratetypeslice-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeSlice`

- <span id="typeslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeSlice`

- <span id="cratetypeslice-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeSlice`

##### `impl<T> From for TypeSlice`

- <span id="typeslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeSlice`

- <span id="cratetypeslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeSlice`

- <span id="typeslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeSlice`

- <span id="cratetytypeslice-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeSlice`

- <span id="cratetypeslice-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeSlice`

##### `impl Spanned for TypeSlice`

- <span id="typeslice-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeSlice`

- <span id="typeslice-toowned-type-owned"></span>`type Owned = T`

- <span id="typeslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeSlice`

- <span id="cratetytypeslice-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeSlice`

- <span id="typeslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeSlice`

- <span id="typeslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeTraitObject`

```rust
struct TypeTraitObject {
    pub dyn_token: Option<token::Dyn>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:211-219`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L211-L219)*

A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
trait or a lifetime.

#### Implementations

- <span id="cratetytypetraitobject-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratetytypetraitobject-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratetytypetraitobject-parse-bounds"></span>`fn parse_bounds(dyn_span: Span, input: ParseStream<'_>, allow_plus: bool) -> Result<Punctuated<TypeParamBound, token::Plus>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Punctuated`](../punctuated/index.md#punctuated), [`TypeParamBound`](../generics/index.md#typeparambound), [`Plus`](../token/index.md#plus)

#### Trait Implementations

##### `impl Any for TypeTraitObject`

- <span id="typetraitobject-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeTraitObject`

- <span id="typetraitobject-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeTraitObject`

- <span id="typetraitobject-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeTraitObject`

- <span id="cratetypetraitobject-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeTraitObject`

- <span id="typetraitobject-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeTraitObject`

- <span id="cratetypetraitobject-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTraitObject`

##### `impl<T> From for TypeTraitObject`

- <span id="typetraitobject-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeTraitObject`

- <span id="cratetypetraitobject-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeTraitObject`

- <span id="typetraitobject-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeTraitObject`

- <span id="cratetypetraitobject-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeTraitObject`

##### `impl Spanned for TypeTraitObject`

- <span id="typetraitobject-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeTraitObject`

- <span id="typetraitobject-toowned-type-owned"></span>`type Owned = T`

- <span id="typetraitobject-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typetraitobject-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeTraitObject`

- <span id="typetraitobject-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typetraitobject-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeTraitObject`

- <span id="typetraitobject-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typetraitobject-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeTuple`

```rust
struct TypeTuple {
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Type, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:221-228`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L221-L228)*

A tuple type: `(A, B, C, String)`.

#### Implementations

- <span id="cratetypetuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TypeTuple`

- <span id="typetuple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeTuple`

- <span id="typetuple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeTuple`

- <span id="typetuple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeTuple`

- <span id="cratetypetuple-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeTuple`

- <span id="typetuple-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeTuple`

- <span id="cratetypetuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTuple`

##### `impl<T> From for TypeTuple`

- <span id="typetuple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeTuple`

- <span id="cratetypetuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeTuple`

- <span id="typetuple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::TypeTuple`

- <span id="cratetytypetuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeTuple`

- <span id="cratetypetuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeTuple`

##### `impl Spanned for TypeTuple`

- <span id="typetuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeTuple`

- <span id="typetuple-toowned-type-owned"></span>`type Owned = T`

- <span id="typetuple-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typetuple-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::TypeTuple`

- <span id="cratetytypetuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeTuple`

- <span id="typetuple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typetuple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeTuple`

- <span id="typetuple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typetuple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Abi`

```rust
struct Abi {
    pub extern_token: token::Extern,
    pub name: Option<crate::lit::LitStr>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:230-237`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L230-L237)*

The binary interface of a function: `extern "C"`.

#### Trait Implementations

##### `impl Any for Abi`

- <span id="abi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Abi`

- <span id="abi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Abi`

- <span id="abi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Abi`

- <span id="crateabi-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Abi`

- <span id="abi-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Abi`

- <span id="crateabi-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Abi`

##### `impl<T> From for Abi`

- <span id="abi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Abi`

- <span id="crateabi-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Abi`

- <span id="abi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::Abi`

- <span id="cratetyabi-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Abi`

- <span id="crateabi-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Abi`

##### `impl Spanned for Abi`

- <span id="abi-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Abi`

- <span id="abi-toowned-type-owned"></span>`type Owned = T`

- <span id="abi-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="abi-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::Abi`

- <span id="cratetyabi-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Abi`

- <span id="abi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Abi`

- <span id="abi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BareFnArg`

```rust
struct BareFnArg {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub ty: Type,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:239-247`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L239-L247)*

An argument in a function type: the `usize` in `fn(usize) -> bool`.

#### Trait Implementations

##### `impl Any for BareFnArg`

- <span id="barefnarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BareFnArg`

- <span id="barefnarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BareFnArg`

- <span id="barefnarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BareFnArg`

- <span id="cratebarefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BareFnArg`

- <span id="barefnarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::BareFnArg`

- <span id="cratebarefnarg-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareFnArg`

##### `impl<T> From for BareFnArg`

- <span id="barefnarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BareFnArg`

- <span id="cratebarefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BareFnArg`

- <span id="barefnarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::BareFnArg`

- <span id="cratebarefnarg-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BareFnArg`

##### `impl Spanned for BareFnArg`

- <span id="barefnarg-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BareFnArg`

- <span id="barefnarg-toowned-type-owned"></span>`type Owned = T`

- <span id="barefnarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="barefnarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BareFnArg`

- <span id="barefnarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="barefnarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BareFnArg`

- <span id="barefnarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="barefnarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BareVariadic`

```rust
struct BareVariadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/ty.rs:249-258`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L249-L258)*

The variadic argument of a function pointer like `fn(usize, ...)`.

#### Trait Implementations

##### `impl Any for BareVariadic`

- <span id="barevariadic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BareVariadic`

- <span id="barevariadic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BareVariadic`

- <span id="barevariadic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BareVariadic`

- <span id="cratebarevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BareVariadic`

- <span id="barevariadic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::BareVariadic`

- <span id="cratebarevariadic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareVariadic`

##### `impl<T> From for BareVariadic`

- <span id="barevariadic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BareVariadic`

- <span id="cratebarevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BareVariadic`

- <span id="barevariadic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::BareVariadic`

- <span id="cratebarevariadic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BareVariadic`

##### `impl Spanned for BareVariadic`

- <span id="barevariadic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BareVariadic`

- <span id="barevariadic-toowned-type-owned"></span>`type Owned = T`

- <span id="barevariadic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="barevariadic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::BareVariadic`

- <span id="cratetybarevariadic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BareVariadic`

- <span id="barevariadic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="barevariadic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BareVariadic`

- <span id="barevariadic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="barevariadic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

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

*Defined in [`syn-2.0.111/src/ty.rs:13-90`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L13-L90)*

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

- <span id="cratetytype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  In some positions, types may not contain the `+` character, to

  disambiguate them. For example in the expression `1 as T`, T may not

  contain a `+` character.

  

  This parser does not allow a `+`, while the default parser does.

#### Trait Implementations

##### `impl Any for Type`

- <span id="type-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Type`

- <span id="type-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Type`

- <span id="type-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Type`

- <span id="cratetype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Type`

- <span id="type-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Type`

- <span id="cratetype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Type`

##### `impl<T> From for Type`

- <span id="type-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Type`

- <span id="cratetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Type`

- <span id="type-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::Type`

- <span id="cratetytype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Type`

- <span id="cratetype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Type`

##### `impl Spanned for Type`

- <span id="type-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Type`

- <span id="type-toowned-type-owned"></span>`type Owned = T`

- <span id="type-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="type-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Type`

- <span id="type-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Type`

- <span id="type-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="type-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Type`

- <span id="type-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="type-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReturnType`

```rust
enum ReturnType {
    Default,
    Type(token::RArrow, Box<Type>),
}
```

*Defined in [`syn-2.0.111/src/ty.rs:260-271`](../../../.source_1765521767/syn-2.0.111/src/ty.rs#L260-L271)*

Return type of a function signature.

#### Variants

- **`Default`**

  Return type is not specified.
  
  Functions default to `()` and closures default to type inference.

- **`Type`**

  A particular type is returned.

#### Implementations

- <span id="cratetyreturntype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for ReturnType`

- <span id="returntype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReturnType`

- <span id="returntype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReturnType`

- <span id="returntype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ReturnType`

- <span id="cratereturntype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ReturnType`

- <span id="returntype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ReturnType`

- <span id="cratereturntype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ReturnType`

##### `impl<T> From for ReturnType`

- <span id="returntype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ReturnType`

- <span id="cratereturntype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ReturnType`

- <span id="returntype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::ty::ReturnType`

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ReturnType`

- <span id="cratereturntype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ReturnType`

##### `impl Spanned for ReturnType`

- <span id="returntype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ReturnType`

- <span id="returntype-toowned-type-owned"></span>`type Owned = T`

- <span id="returntype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="returntype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::ty::ReturnType`

- <span id="cratetyreturntype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ReturnType`

- <span id="returntype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="returntype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReturnType`

- <span id="returntype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="returntype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

