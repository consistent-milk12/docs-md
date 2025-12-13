*[syn](../index.md) / [lit](index.md)*

---

# Module `lit`

## Contents

- [Modules](#modules)
  - [`debug_impls`](#debug-impls)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
  - [`value`](#value)
- [Structs](#structs)
  - [`LitStr`](#litstr)
  - [`LitByteStr`](#litbytestr)
  - [`LitCStr`](#litcstr)
  - [`LitByte`](#litbyte)
  - [`LitChar`](#litchar)
  - [`LitRepr`](#litrepr)
  - [`LitInt`](#litint)
  - [`LitIntRepr`](#litintrepr)
  - [`LitFloat`](#litfloat)
  - [`LitFloatRepr`](#litfloatrepr)
  - [`LitBool`](#litbool)
- [Enums](#enums)
  - [`Lit`](#lit)
- [Macros](#macros)
  - [`lit_extra_traits!`](#lit-extra-traits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`debug_impls`](#debug-impls) | mod |  |
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`value`](#value) | mod |  |
| [`LitStr`](#litstr) | struct | A UTF-8 string literal: `"foo"`. |
| [`LitByteStr`](#litbytestr) | struct | A byte string literal: `b"foo"`. |
| [`LitCStr`](#litcstr) | struct | A nul-terminated C-string literal: `c"foo"`. |
| [`LitByte`](#litbyte) | struct | A byte literal: `b'f'`. |
| [`LitChar`](#litchar) | struct | A character literal: `'a'`. |
| [`LitRepr`](#litrepr) | struct |  |
| [`LitInt`](#litint) | struct | An integer literal: `1` or `1u16`. |
| [`LitIntRepr`](#litintrepr) | struct |  |
| [`LitFloat`](#litfloat) | struct | A floating point literal: `1f64` or `1.0e10f64`. |
| [`LitFloatRepr`](#litfloatrepr) | struct |  |
| [`LitBool`](#litbool) | struct | A boolean literal: `true` or `false`. |
| [`Lit`](#lit) | enum | A Rust literal such as a string or integer or boolean. |
| [`lit_extra_traits!`](#lit-extra-traits) | macro |  |

## Modules

- [`debug_impls`](debug_impls/index.md)
- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)
- [`value`](value/index.md)

## Structs

### `LitStr`

```rust
struct LitStr {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:58-63`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L58-L63)*

A UTF-8 string literal: `"foo"`.

#### Implementations

- <span id="cratelitlitstr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitStr`

- <span id="litstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitStr`

- <span id="litstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitStr`

- <span id="litstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitStr`

- <span id="litstr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitStr`

- <span id="litstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitStr`

- <span id="cratelitlitstr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitStr`

##### `impl<T> From for LitStr`

- <span id="litstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitStr`

- <span id="litstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitStr`

- <span id="litstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitStr`

- <span id="cratelitlitstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitStr`

- <span id="litstr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitStr`

##### `impl Spanned for LitStr`

- <span id="litstr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitStr`

- <span id="litstr-toowned-type-owned"></span>`type Owned = T`

- <span id="litstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitStr`

- <span id="cratelitlitstr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitStr`

##### `impl<U> TryFrom for LitStr`

- <span id="litstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitStr`

- <span id="litstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitByteStr`

```rust
struct LitByteStr {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:65-70`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L65-L70)*

A byte string literal: `b"foo"`.

#### Implementations

- <span id="cratelitlitbytestr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitByteStr`

- <span id="litbytestr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitByteStr`

- <span id="litbytestr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitByteStr`

- <span id="litbytestr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitByteStr`

- <span id="litbytestr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitByteStr`

- <span id="litbytestr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByteStr`

##### `impl<T> From for LitByteStr`

- <span id="litbytestr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitByteStr`

- <span id="litbytestr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitByteStr`

- <span id="litbytestr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitByteStr`

- <span id="litbytestr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitByteStr`

##### `impl Spanned for LitByteStr`

- <span id="litbytestr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitByteStr`

- <span id="litbytestr-toowned-type-owned"></span>`type Owned = T`

- <span id="litbytestr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litbytestr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitByteStr`

- <span id="cratelitlitbytestr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByteStr`

##### `impl<U> TryFrom for LitByteStr`

- <span id="litbytestr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litbytestr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitByteStr`

- <span id="litbytestr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litbytestr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitCStr`

```rust
struct LitCStr {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:72-77`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L72-L77)*

A nul-terminated C-string literal: `c"foo"`.

#### Implementations

- <span id="cratelitlitcstr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitCStr`

- <span id="litcstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitCStr`

- <span id="litcstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitCStr`

- <span id="litcstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitCStr`

- <span id="litcstr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitCStr`

- <span id="litcstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitCStr`

- <span id="cratelitlitcstr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitCStr`

##### `impl<T> From for LitCStr`

- <span id="litcstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitCStr`

- <span id="litcstr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitCStr`

- <span id="litcstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitCStr`

- <span id="cratelitlitcstr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitCStr`

- <span id="litcstr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitCStr`

##### `impl Spanned for LitCStr`

- <span id="litcstr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitCStr`

- <span id="litcstr-toowned-type-owned"></span>`type Owned = T`

- <span id="litcstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litcstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitCStr`

- <span id="cratelitlitcstr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitCStr`

##### `impl<U> TryFrom for LitCStr`

- <span id="litcstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litcstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitCStr`

- <span id="litcstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litcstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitByte`

```rust
struct LitByte {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:79-84`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L79-L84)*

A byte literal: `b'f'`.

#### Implementations

- <span id="cratelitlitbyte-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitByte`

- <span id="litbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitByte`

- <span id="litbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitByte`

- <span id="litbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitByte`

- <span id="litbyte-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitByte`

- <span id="litbyte-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitByte`

- <span id="cratelitlitbyte-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitByte`

##### `impl<T> From for LitByte`

- <span id="litbyte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitByte`

- <span id="litbyte-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitByte`

- <span id="litbyte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitByte`

- <span id="cratelitlitbyte-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitByte`

- <span id="litbyte-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitByte`

##### `impl Spanned for LitByte`

- <span id="litbyte-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitByte`

- <span id="litbyte-toowned-type-owned"></span>`type Owned = T`

- <span id="litbyte-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litbyte-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitByte`

- <span id="cratelitlitbyte-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitByte`

##### `impl<U> TryFrom for LitByte`

- <span id="litbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitByte`

- <span id="litbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitChar`

```rust
struct LitChar {
    repr: Box<LitRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:86-91`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L86-L91)*

A character literal: `'a'`.

#### Implementations

- <span id="cratelitlitchar-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitChar`

- <span id="litchar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitChar`

- <span id="litchar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitChar`

- <span id="litchar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitChar`

- <span id="litchar-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitChar`

- <span id="litchar-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitChar`

- <span id="cratelitlitchar-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitChar`

##### `impl<T> From for LitChar`

- <span id="litchar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitChar`

- <span id="litchar-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitChar`

- <span id="litchar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitChar`

- <span id="cratelitlitchar-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitChar`

- <span id="litchar-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitChar`

##### `impl Spanned for LitChar`

- <span id="litchar-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitChar`

- <span id="litchar-toowned-type-owned"></span>`type Owned = T`

- <span id="litchar-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litchar-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitChar`

- <span id="cratelitlitchar-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitChar`

##### `impl<U> TryFrom for LitChar`

- <span id="litchar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litchar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitChar`

- <span id="litchar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litchar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitRepr`

```rust
struct LitRepr {
    token: proc_macro2::Literal,
    suffix: Box<str>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:93-96`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L93-L96)*

#### Trait Implementations

##### `impl Any for LitRepr`

- <span id="litrepr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitRepr`

- <span id="litrepr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitRepr`

- <span id="litrepr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitRepr`

- <span id="litrepr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitRepr`

- <span id="litrepr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for LitRepr`

- <span id="litrepr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LitRepr`

- <span id="litrepr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LitRepr`

- <span id="litrepr-toowned-type-owned"></span>`type Owned = T`

- <span id="litrepr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litrepr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LitRepr`

- <span id="litrepr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litrepr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitRepr`

- <span id="litrepr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litrepr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitInt`

```rust
struct LitInt {
    repr: Box<LitIntRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:98-103`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L98-L103)*

An integer literal: `1` or `1u16`.

#### Implementations

- <span id="cratelitlitint-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitInt`

- <span id="litint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitInt`

- <span id="litint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitInt`

- <span id="litint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitInt`

- <span id="litint-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitInt`

- <span id="litint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitInt`

- <span id="cratelitlitint-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitInt`

- <span id="litint-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitInt`

##### `impl<T> From for LitInt`

- <span id="litint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitInt`

- <span id="litint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitInt`

- <span id="litint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitInt`

- <span id="cratelitlitint-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitInt`

- <span id="litint-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitInt`

##### `impl Spanned for LitInt`

- <span id="litint-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitInt`

- <span id="litint-toowned-type-owned"></span>`type Owned = T`

- <span id="litint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for LitInt`

- <span id="litint-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitInt`

- <span id="cratelitlitint-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitInt`

##### `impl<U> TryFrom for LitInt`

- <span id="litint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitInt`

- <span id="litint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitIntRepr`

```rust
struct LitIntRepr {
    token: proc_macro2::Literal,
    digits: Box<str>,
    suffix: Box<str>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:105-109`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L105-L109)*

#### Trait Implementations

##### `impl Any for LitIntRepr`

- <span id="litintrepr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitIntRepr`

- <span id="litintrepr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitIntRepr`

- <span id="litintrepr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitIntRepr`

- <span id="litintrepr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitIntRepr`

- <span id="litintrepr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for LitIntRepr`

- <span id="litintrepr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LitIntRepr`

- <span id="litintrepr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LitIntRepr`

- <span id="litintrepr-toowned-type-owned"></span>`type Owned = T`

- <span id="litintrepr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litintrepr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LitIntRepr`

- <span id="litintrepr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litintrepr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitIntRepr`

- <span id="litintrepr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litintrepr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitFloat`

```rust
struct LitFloat {
    repr: Box<LitFloatRepr>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:111-118`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L111-L118)*

A floating point literal: `1f64` or `1.0e10f64`.

Must be finite. May not be infinite or NaN.

#### Implementations

- <span id="cratelitlitfloat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitFloat`

- <span id="litfloat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitFloat`

- <span id="litfloat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitFloat`

- <span id="litfloat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitFloat`

- <span id="litfloat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitFloat`

- <span id="litfloat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitFloat`

- <span id="cratelitlitfloat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LitFloat`

- <span id="litfloat-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitFloat`

##### `impl<T> From for LitFloat`

- <span id="litfloat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LitFloat`

- <span id="litfloat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitFloat`

- <span id="litfloat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitFloat`

- <span id="cratelitlitfloat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LitFloat`

- <span id="litfloat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitFloat`

##### `impl Spanned for LitFloat`

- <span id="litfloat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitFloat`

- <span id="litfloat-toowned-type-owned"></span>`type Owned = T`

- <span id="litfloat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litfloat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for LitFloat`

- <span id="litfloat-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for crate::lit::LitFloat`

- <span id="cratelitlitfloat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitFloat`

##### `impl<U> TryFrom for LitFloat`

- <span id="litfloat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litfloat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitFloat`

- <span id="litfloat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litfloat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitFloatRepr`

```rust
struct LitFloatRepr {
    token: proc_macro2::Literal,
    digits: Box<str>,
    suffix: Box<str>,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:120-124`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L120-L124)*

#### Trait Implementations

##### `impl Any for LitFloatRepr`

- <span id="litfloatrepr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitFloatRepr`

- <span id="litfloatrepr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitFloatRepr`

- <span id="litfloatrepr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitFloatRepr`

- <span id="litfloatrepr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitFloatRepr`

- <span id="litfloatrepr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for LitFloatRepr`

- <span id="litfloatrepr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LitFloatRepr`

- <span id="litfloatrepr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LitFloatRepr`

- <span id="litfloatrepr-toowned-type-owned"></span>`type Owned = T`

- <span id="litfloatrepr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litfloatrepr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LitFloatRepr`

- <span id="litfloatrepr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litfloatrepr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitFloatRepr`

- <span id="litfloatrepr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litfloatrepr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LitBool`

```rust
struct LitBool {
    pub value: bool,
    pub span: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/lit.rs:126-132`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L126-L132)*

A boolean literal: `true` or `false`.

#### Implementations

- <span id="cratelitlitbool-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for LitBool`

- <span id="litbool-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitBool`

- <span id="litbool-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitBool`

- <span id="litbool-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::LitBool`

- <span id="cratelitbool-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LitBool`

- <span id="litbool-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::lit::LitBool`

- <span id="cratelitlitbool-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LitBool`

##### `impl<T> From for LitBool`

- <span id="litbool-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::LitBool`

- <span id="cratelitbool-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LitBool`

- <span id="litbool-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::LitBool`

- <span id="cratelitlitbool-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::LitBool`

- <span id="cratelitbool-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LitBool`

##### `impl Spanned for LitBool`

- <span id="litbool-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitBool`

- <span id="litbool-toowned-type-owned"></span>`type Owned = T`

- <span id="litbool-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litbool-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::lit::LitBool`

- <span id="cratelitlitbool-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for crate::lit::LitBool`

##### `impl<U> TryFrom for LitBool`

- <span id="litbool-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litbool-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitBool`

- <span id="litbool-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litbool-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

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

*Defined in [`syn-2.0.111/src/lit.rs:17-56`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L17-L56)*

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

- <span id="cratelitlit-new"></span>`fn new(token: Literal) -> Self`

  Interpret a Syn literal from a proc-macro2 literal.

- <span id="cratelitlit-from-str"></span>`fn from_str(token: Literal, repr: &str) -> Self`

- <span id="cratelitlit-suffix"></span>`fn suffix(&self) -> &str`

- <span id="cratelitlit-span"></span>`fn span(&self) -> Span`

- <span id="cratelitlit-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Any for Lit`

- <span id="lit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lit`

- <span id="lit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lit`

- <span id="lit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Lit`

- <span id="cratelit-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Lit`

- <span id="lit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Lit`

- <span id="cratelit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Lit`

##### `impl<T> From for Lit`

- <span id="lit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Lit`

- <span id="cratelit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Lit`

- <span id="lit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::lit::Lit`

- <span id="cratelitlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Lit`

- <span id="cratelit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Lit`

##### `impl Spanned for Lit`

- <span id="lit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Lit`

- <span id="lit-toowned-type-owned"></span>`type Owned = T`

- <span id="lit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Lit`

- <span id="lit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl Token for crate::lit::Lit`

##### `impl<U> TryFrom for Lit`

- <span id="lit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lit`

- <span id="lit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `lit_extra_traits!`

*Defined in [`syn-2.0.111/src/lit.rs:778-818`](../../../.source_1765633015/syn-2.0.111/src/lit.rs#L778-L818)*

