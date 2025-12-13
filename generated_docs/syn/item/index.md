*[syn](../index.md) / [item](index.md)*

---

# Module `item`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`ItemConst`](#itemconst)
  - [`ItemEnum`](#itemenum)
  - [`ItemExternCrate`](#itemexterncrate)
  - [`ItemFn`](#itemfn)
  - [`ItemForeignMod`](#itemforeignmod)
  - [`ItemImpl`](#itemimpl)
  - [`ItemMacro`](#itemmacro)
  - [`ItemMod`](#itemmod)
  - [`ItemStatic`](#itemstatic)
  - [`ItemStruct`](#itemstruct)
  - [`ItemTrait`](#itemtrait)
  - [`ItemTraitAlias`](#itemtraitalias)
  - [`ItemType`](#itemtype)
  - [`ItemUnion`](#itemunion)
  - [`ItemUse`](#itemuse)
  - [`UsePath`](#usepath)
  - [`UseName`](#usename)
  - [`UseRename`](#userename)
  - [`UseGlob`](#useglob)
  - [`UseGroup`](#usegroup)
  - [`ForeignItemFn`](#foreignitemfn)
  - [`ForeignItemStatic`](#foreignitemstatic)
  - [`ForeignItemType`](#foreignitemtype)
  - [`ForeignItemMacro`](#foreignitemmacro)
  - [`TraitItemConst`](#traititemconst)
  - [`TraitItemFn`](#traititemfn)
  - [`TraitItemType`](#traititemtype)
  - [`TraitItemMacro`](#traititemmacro)
  - [`ImplItemConst`](#implitemconst)
  - [`ImplItemFn`](#implitemfn)
  - [`ImplItemType`](#implitemtype)
  - [`ImplItemMacro`](#implitemmacro)
  - [`Signature`](#signature)
  - [`Receiver`](#receiver)
  - [`Variadic`](#variadic)
- [Enums](#enums)
  - [`Item`](#item)
  - [`UseTree`](#usetree)
  - [`ForeignItem`](#foreignitem)
  - [`TraitItem`](#traititem)
  - [`ImplItem`](#implitem)
  - [`FnArg`](#fnarg)
  - [`StaticMutability`](#staticmutability)
  - [`ImplRestriction`](#implrestriction)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`ItemConst`](#itemconst) | struct | A constant item: `const MAX: u16 = 65535`. |
| [`ItemEnum`](#itemenum) | struct | An enum definition: `enum Foo<A, B> { A(A), B(B) }`. |
| [`ItemExternCrate`](#itemexterncrate) | struct | An `extern crate` item: `extern crate serde`. |
| [`ItemFn`](#itemfn) | struct | A free-standing function: `fn process(n: usize) -> Result<()> { ... |
| [`ItemForeignMod`](#itemforeignmod) | struct | A block of foreign items: `extern "C" { ... |
| [`ItemImpl`](#itemimpl) | struct | An impl block providing trait or associated items: `impl<A> Trait for Data<A> { ... |
| [`ItemMacro`](#itemmacro) | struct | A macro invocation, which includes `macro_rules!` definitions. |
| [`ItemMod`](#itemmod) | struct | A module or module declaration: `mod m` or `mod m { ... |
| [`ItemStatic`](#itemstatic) | struct | A static item: `static BIKE: Shed = Shed(42)`. |
| [`ItemStruct`](#itemstruct) | struct | A struct definition: `struct Foo<A> { x: A }`. |
| [`ItemTrait`](#itemtrait) | struct | A trait definition: `pub trait Iterator { ... |
| [`ItemTraitAlias`](#itemtraitalias) | struct | A trait alias: `pub trait SharableIterator = Iterator + Sync`. |
| [`ItemType`](#itemtype) | struct | A type alias: `type Result<T> = std::result::Result<T, MyError>`. |
| [`ItemUnion`](#itemunion) | struct | A union definition: `union Foo<A, B> { x: A, y: B }`. |
| [`ItemUse`](#itemuse) | struct | A use declaration: `use std::collections::HashMap`. |
| [`UsePath`](#usepath) | struct | A path prefix of imports in a `use` item: `std::...`. |
| [`UseName`](#usename) | struct | An identifier imported by a `use` item: `HashMap`. |
| [`UseRename`](#userename) | struct | An renamed identifier imported by a `use` item: `HashMap as Map`. |
| [`UseGlob`](#useglob) | struct | A glob import in a `use` item: `*`. |
| [`UseGroup`](#usegroup) | struct | A braced group of imports in a `use` item: `{A, B, C}`. |
| [`ForeignItemFn`](#foreignitemfn) | struct | A foreign function in an `extern` block. |
| [`ForeignItemStatic`](#foreignitemstatic) | struct | A foreign static item in an `extern` block: `static ext: u8`. |
| [`ForeignItemType`](#foreignitemtype) | struct | A foreign type in an `extern` block: `type void`. |
| [`ForeignItemMacro`](#foreignitemmacro) | struct | A macro invocation within an extern block. |
| [`TraitItemConst`](#traititemconst) | struct | An associated constant within the definition of a trait. |
| [`TraitItemFn`](#traititemfn) | struct | An associated function within the definition of a trait. |
| [`TraitItemType`](#traititemtype) | struct | An associated type within the definition of a trait. |
| [`TraitItemMacro`](#traititemmacro) | struct | A macro invocation within the definition of a trait. |
| [`ImplItemConst`](#implitemconst) | struct | An associated constant within an impl block. |
| [`ImplItemFn`](#implitemfn) | struct | An associated function within an impl block. |
| [`ImplItemType`](#implitemtype) | struct | An associated type within an impl block. |
| [`ImplItemMacro`](#implitemmacro) | struct | A macro invocation within an impl block. |
| [`Signature`](#signature) | struct | A function signature in a trait or implementation: `unsafe fn initialize(&self)`. |
| [`Receiver`](#receiver) | struct | The `self` argument of an associated method. |
| [`Variadic`](#variadic) | struct | The variadic argument of a foreign function. |
| [`Item`](#item) | enum | Things that can appear directly inside of a module or scope. |
| [`UseTree`](#usetree) | enum | A suffix of an import tree in a `use` item: `Type as Renamed` or `*`. |
| [`ForeignItem`](#foreignitem) | enum | An item within an `extern` block. |
| [`TraitItem`](#traititem) | enum | An item declaration within the definition of a trait. |
| [`ImplItem`](#implitem) | enum | An item within an impl block. |
| [`FnArg`](#fnarg) | enum | An argument in a function signature: the `n: usize` in `fn f(n: usize)`. |
| [`StaticMutability`](#staticmutability) | enum | The mutability of an `Item::Static` or `ForeignItem::Static`. |
| [`ImplRestriction`](#implrestriction) | enum | Unused, but reserved for RFC 3323 restrictions. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `ItemConst`

```rust
struct ItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:101-116`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L101-L116)*

A constant item: `const MAX: u16 = 65535`.

#### Implementations

- <span id="crateitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemConst`

- <span id="itemconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemConst`

- <span id="itemconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemConst`

- <span id="itemconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemConst`

- <span id="crateitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemConst`

- <span id="itemconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemConst`

- <span id="crateitemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemConst`

##### `impl<T> From for ItemConst`

- <span id="itemconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemConst`

- <span id="crateitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemConst`

- <span id="itemconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemConst`

- <span id="crateitemitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemConst`

- <span id="crateitemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemConst`

##### `impl Spanned for ItemConst`

- <span id="itemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemConst`

- <span id="itemconst-toowned-type-owned"></span>`type Owned = T`

- <span id="itemconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemConst`

- <span id="crateitemitemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemConst`

- <span id="itemconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemConst`

- <span id="itemconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemEnum`

```rust
struct ItemEnum {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub enum_token: token::Enum,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:118-130`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L118-L130)*

An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

#### Implementations

- <span id="crateitemenum-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemEnum`

- <span id="itemenum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemEnum`

- <span id="itemenum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemEnum`

- <span id="itemenum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemEnum`

- <span id="crateitemenum-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemEnum`

- <span id="itemenum-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemEnum`

- <span id="crateitemenum-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemEnum`

##### `impl<T> From for ItemEnum`

- <span id="itemenum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemEnum`

- <span id="crateitemenum-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemEnum`

- <span id="itemenum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemEnum`

- <span id="crateitemitemenum-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemEnum`

- <span id="crateitemenum-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemEnum`

##### `impl Spanned for ItemEnum`

- <span id="itemenum-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemEnum`

- <span id="itemenum-toowned-type-owned"></span>`type Owned = T`

- <span id="itemenum-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemenum-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemEnum`

- <span id="crateitemitemenum-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemEnum`

- <span id="itemenum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemenum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemEnum`

- <span id="itemenum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemenum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemExternCrate`

```rust
struct ItemExternCrate {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub extern_token: token::Extern,
    pub crate_token: token::Crate,
    pub ident: crate::ident::Ident,
    pub rename: Option<(token::As, crate::ident::Ident)>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:132-144`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L132-L144)*

An `extern crate` item: `extern crate serde`.

#### Implementations

- <span id="crateitemexterncrate-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemExternCrate`

- <span id="itemexterncrate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemExternCrate`

- <span id="itemexterncrate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemExternCrate`

- <span id="itemexterncrate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemExternCrate`

- <span id="crateitemexterncrate-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemExternCrate`

- <span id="itemexterncrate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemExternCrate`

- <span id="crateitemexterncrate-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemExternCrate`

##### `impl<T> From for ItemExternCrate`

- <span id="itemexterncrate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemExternCrate`

- <span id="crateitemexterncrate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemExternCrate`

- <span id="itemexterncrate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemExternCrate`

- <span id="crateitemexterncrate-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemExternCrate`

##### `impl Spanned for ItemExternCrate`

- <span id="itemexterncrate-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemExternCrate`

- <span id="itemexterncrate-toowned-type-owned"></span>`type Owned = T`

- <span id="itemexterncrate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemexterncrate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemExternCrate`

- <span id="crateitemitemexterncrate-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemExternCrate`

- <span id="itemexterncrate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemexterncrate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemExternCrate`

- <span id="itemexterncrate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemexterncrate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemFn`

```rust
struct ItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub block: Box<crate::stmt::Block>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:146-155`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L146-L155)*

A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.

#### Implementations

- <span id="crateitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemFn`

- <span id="itemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemFn`

- <span id="itemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemFn`

- <span id="itemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemFn`

- <span id="crateitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemFn`

- <span id="itemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemFn`

- <span id="crateitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemFn`

##### `impl<T> From for ItemFn`

- <span id="itemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemFn`

- <span id="crateitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemFn`

- <span id="itemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemFn`

- <span id="crateitemitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemFn`

- <span id="crateitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemFn`

##### `impl Spanned for ItemFn`

- <span id="itemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemFn`

- <span id="itemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="itemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemFn`

- <span id="crateitemitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemFn`

- <span id="itemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemFn`

- <span id="itemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemForeignMod`

```rust
struct ItemForeignMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: crate::ty::Abi,
    pub brace_token: token::Brace,
    pub items: Vec<ForeignItem>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:157-167`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L157-L167)*

A block of foreign items: `extern "C" { ... }`.

#### Implementations

- <span id="crateitemforeignmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemForeignMod`

- <span id="itemforeignmod-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemForeignMod`

- <span id="itemforeignmod-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemForeignMod`

- <span id="itemforeignmod-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemForeignMod`

- <span id="crateitemforeignmod-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemForeignMod`

- <span id="itemforeignmod-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemForeignMod`

- <span id="crateitemforeignmod-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemForeignMod`

##### `impl<T> From for ItemForeignMod`

- <span id="itemforeignmod-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemForeignMod`

- <span id="crateitemforeignmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemForeignMod`

- <span id="itemforeignmod-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemForeignMod`

- <span id="crateitemforeignmod-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemForeignMod`

##### `impl Spanned for ItemForeignMod`

- <span id="itemforeignmod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemForeignMod`

- <span id="itemforeignmod-toowned-type-owned"></span>`type Owned = T`

- <span id="itemforeignmod-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemforeignmod-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemForeignMod`

- <span id="crateitemitemforeignmod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemForeignMod`

- <span id="itemforeignmod-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemforeignmod-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemForeignMod`

- <span id="itemforeignmod-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemforeignmod-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemImpl`

```rust
struct ItemImpl {
    pub attrs: Vec<crate::attr::Attribute>,
    pub defaultness: Option<token::Default>,
    pub unsafety: Option<token::Unsafe>,
    pub impl_token: token::Impl,
    pub generics: crate::generics::Generics,
    pub trait_: Option<(Option<token::Not>, crate::path::Path, token::For)>,
    pub self_ty: Box<crate::ty::Type>,
    pub brace_token: token::Brace,
    pub items: Vec<ImplItem>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:169-186`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L169-L186)*

An impl block providing trait or associated items: `impl<A> Trait
for Data<A> { ... }`.

#### Fields

- **`trait_`**: `Option<(Option<token::Not>, crate::path::Path, token::For)>`

  Trait this impl implements.

- **`self_ty`**: `Box<crate::ty::Type>`

  The Self type of the impl.

#### Implementations

- <span id="crateitemimpl-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemImpl`

- <span id="itemimpl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemImpl`

- <span id="itemimpl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemImpl`

- <span id="itemimpl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemImpl`

- <span id="crateitemimpl-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemImpl`

- <span id="itemimpl-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemImpl`

- <span id="crateitemimpl-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemImpl`

##### `impl<T> From for ItemImpl`

- <span id="itemimpl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemImpl`

- <span id="crateitemimpl-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemImpl`

- <span id="itemimpl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemImpl`

- <span id="crateitemitemimpl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemImpl`

- <span id="crateitemimpl-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemImpl`

##### `impl Spanned for ItemImpl`

- <span id="itemimpl-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemImpl`

- <span id="itemimpl-toowned-type-owned"></span>`type Owned = T`

- <span id="itemimpl-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemimpl-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemImpl`

- <span id="crateitemitemimpl-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemImpl`

- <span id="itemimpl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemimpl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemImpl`

- <span id="itemimpl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemimpl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemMacro`

```rust
struct ItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: Option<crate::ident::Ident>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:188-198`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L188-L198)*

A macro invocation, which includes `macro_rules!` definitions.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  The `example` in `macro_rules! example { ... }`.

#### Implementations

- <span id="crateitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemMacro`

- <span id="itemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemMacro`

- <span id="itemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemMacro`

- <span id="itemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemMacro`

- <span id="crateitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemMacro`

- <span id="itemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemMacro`

- <span id="crateitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMacro`

##### `impl<T> From for ItemMacro`

- <span id="itemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemMacro`

- <span id="crateitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemMacro`

- <span id="itemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemMacro`

- <span id="crateitemitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemMacro`

- <span id="crateitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemMacro`

##### `impl Spanned for ItemMacro`

- <span id="itemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemMacro`

- <span id="itemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="itemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemMacro`

- <span id="crateitemitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemMacro`

- <span id="itemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemMacro`

- <span id="itemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemMod`

```rust
struct ItemMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<token::Unsafe>,
    pub mod_token: token::Mod,
    pub ident: crate::ident::Ident,
    pub content: Option<(token::Brace, Vec<Item>)>,
    pub semi: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:200-212`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L200-L212)*

A module or module declaration: `mod m` or `mod m { ... }`.

#### Implementations

- <span id="crateitemmod-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemMod`

- <span id="itemmod-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemMod`

- <span id="itemmod-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemMod`

- <span id="itemmod-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemMod`

- <span id="crateitemmod-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemMod`

- <span id="itemmod-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemMod`

- <span id="crateitemmod-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMod`

##### `impl<T> From for ItemMod`

- <span id="itemmod-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemMod`

- <span id="crateitemmod-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemMod`

- <span id="itemmod-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemMod`

- <span id="crateitemitemmod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemMod`

- <span id="crateitemmod-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemMod`

##### `impl Spanned for ItemMod`

- <span id="itemmod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemMod`

- <span id="itemmod-toowned-type-owned"></span>`type Owned = T`

- <span id="itemmod-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemmod-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemMod`

- <span id="crateitemitemmod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemMod`

- <span id="itemmod-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemmod-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemMod`

- <span id="itemmod-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemmod-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemStatic`

```rust
struct ItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:214-229`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L214-L229)*

A static item: `static BIKE: Shed = Shed(42)`.

#### Implementations

- <span id="crateitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemStatic`

- <span id="itemstatic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemStatic`

- <span id="itemstatic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemStatic`

- <span id="itemstatic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemStatic`

- <span id="crateitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemStatic`

- <span id="itemstatic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemStatic`

- <span id="crateitemstatic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStatic`

##### `impl<T> From for ItemStatic`

- <span id="itemstatic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemStatic`

- <span id="crateitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemStatic`

- <span id="itemstatic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemStatic`

- <span id="crateitemitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemStatic`

- <span id="crateitemstatic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemStatic`

##### `impl Spanned for ItemStatic`

- <span id="itemstatic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemStatic`

- <span id="itemstatic-toowned-type-owned"></span>`type Owned = T`

- <span id="itemstatic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemstatic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemStatic`

- <span id="crateitemitemstatic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemStatic`

- <span id="itemstatic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemstatic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemStatic`

- <span id="itemstatic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemstatic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemStruct`

```rust
struct ItemStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub struct_token: token::Struct,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::Fields,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:231-243`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L231-L243)*

A struct definition: `struct Foo<A> { x: A }`.

#### Implementations

- <span id="crateitemstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemStruct`

- <span id="itemstruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemStruct`

- <span id="itemstruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemStruct`

- <span id="itemstruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemStruct`

- <span id="crateitemstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemStruct`

- <span id="itemstruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemStruct`

- <span id="crateitemstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStruct`

##### `impl<T> From for ItemStruct`

- <span id="itemstruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemStruct`

- <span id="crateitemstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemStruct`

- <span id="itemstruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemStruct`

- <span id="crateitemitemstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemStruct`

- <span id="crateitemstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemStruct`

##### `impl Spanned for ItemStruct`

- <span id="itemstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemStruct`

- <span id="itemstruct-toowned-type-owned"></span>`type Owned = T`

- <span id="itemstruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemstruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemStruct`

- <span id="crateitemitemstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemStruct`

- <span id="itemstruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemstruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemStruct`

- <span id="itemstruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemstruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemTrait`

```rust
struct ItemTrait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<token::Unsafe>,
    pub auto_token: Option<token::Auto>,
    pub restriction: Option<ImplRestriction>,
    pub trait_token: token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<token::Colon>,
    pub supertraits: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub brace_token: token::Brace,
    pub items: Vec<TraitItem>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:245-262`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L245-L262)*

A trait definition: `pub trait Iterator { ... }`.

#### Implementations

- <span id="crateitemtrait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemTrait`

- <span id="itemtrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemTrait`

- <span id="itemtrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemTrait`

- <span id="itemtrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemTrait`

- <span id="crateitemtrait-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemTrait`

- <span id="itemtrait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemTrait`

- <span id="crateitemtrait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTrait`

##### `impl<T> From for ItemTrait`

- <span id="itemtrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemTrait`

- <span id="crateitemtrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemTrait`

- <span id="itemtrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemTrait`

- <span id="crateitemitemtrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemTrait`

- <span id="crateitemtrait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemTrait`

##### `impl Spanned for ItemTrait`

- <span id="itemtrait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemTrait`

- <span id="itemtrait-toowned-type-owned"></span>`type Owned = T`

- <span id="itemtrait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemtrait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemTrait`

- <span id="crateitemitemtrait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemTrait`

- <span id="itemtrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemtrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemTrait`

- <span id="itemtrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemtrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemTraitAlias`

```rust
struct ItemTraitAlias {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub trait_token: token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:264-277`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L264-L277)*

A trait alias: `pub trait SharableIterator = Iterator + Sync`.

#### Implementations

- <span id="crateitemtraitalias-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemTraitAlias`

- <span id="itemtraitalias-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemTraitAlias`

- <span id="itemtraitalias-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemTraitAlias`

- <span id="itemtraitalias-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemTraitAlias`

- <span id="itemtraitalias-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTraitAlias`

##### `impl<T> From for ItemTraitAlias`

- <span id="itemtraitalias-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemTraitAlias`

- <span id="itemtraitalias-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemTraitAlias`

- <span id="crateitemtraitalias-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemTraitAlias`

##### `impl Spanned for ItemTraitAlias`

- <span id="itemtraitalias-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemTraitAlias`

- <span id="itemtraitalias-toowned-type-owned"></span>`type Owned = T`

- <span id="itemtraitalias-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemtraitalias-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemTraitAlias`

- <span id="crateitemitemtraitalias-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemTraitAlias`

- <span id="itemtraitalias-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemtraitalias-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemTraitAlias`

- <span id="itemtraitalias-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemtraitalias-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemType`

```rust
struct ItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:279-292`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L279-L292)*

A type alias: `type Result<T> = std::result::Result<T, MyError>`.

#### Implementations

- <span id="crateitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemType`

- <span id="itemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemType`

- <span id="itemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemType`

- <span id="itemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemType`

- <span id="crateitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemType`

- <span id="itemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemType`

- <span id="crateitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemType`

##### `impl<T> From for ItemType`

- <span id="itemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemType`

- <span id="crateitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemType`

- <span id="itemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemType`

- <span id="crateitemitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemType`

- <span id="crateitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemType`

##### `impl Spanned for ItemType`

- <span id="itemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemType`

- <span id="itemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="itemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemType`

- <span id="crateitemitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemType`

- <span id="itemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemType`

- <span id="itemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemUnion`

```rust
struct ItemUnion {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub union_token: token::Union,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::FieldsNamed,
}
```

*Defined in [`syn-2.0.111/src/item.rs:294-305`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L294-L305)*

A union definition: `union Foo<A, B> { x: A, y: B }`.

#### Implementations

- <span id="crateitemunion-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemUnion`

- <span id="itemunion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemUnion`

- <span id="itemunion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemUnion`

- <span id="itemunion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemUnion`

- <span id="crateitemunion-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemUnion`

- <span id="itemunion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemUnion`

- <span id="crateitemunion-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUnion`

##### `impl<T> From for ItemUnion`

- <span id="itemunion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemUnion`

- <span id="crateitemunion-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemUnion`

- <span id="itemunion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemUnion`

- <span id="crateitemitemunion-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemUnion`

- <span id="crateitemunion-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemUnion`

##### `impl Spanned for ItemUnion`

- <span id="itemunion-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemUnion`

- <span id="itemunion-toowned-type-owned"></span>`type Owned = T`

- <span id="itemunion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemunion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemUnion`

- <span id="crateitemitemunion-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemUnion`

- <span id="itemunion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemunion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemUnion`

- <span id="itemunion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemunion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ItemUse`

```rust
struct ItemUse {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub use_token: token::Use,
    pub leading_colon: Option<token::PathSep>,
    pub tree: UseTree,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:307-318`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L307-L318)*

A use declaration: `use std::collections::HashMap`.

#### Implementations

- <span id="crateitemuse-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ItemUse`

- <span id="itemuse-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ItemUse`

- <span id="itemuse-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItemUse`

- <span id="itemuse-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ItemUse`

- <span id="crateitemuse-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ItemUse`

- <span id="itemuse-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ItemUse`

- <span id="crateitemuse-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUse`

##### `impl<T> From for ItemUse`

- <span id="itemuse-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ItemUse`

- <span id="crateitemuse-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ItemUse`

- <span id="itemuse-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ItemUse`

- <span id="crateitemitemuse-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ItemUse`

- <span id="crateitemuse-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ItemUse`

##### `impl Spanned for ItemUse`

- <span id="itemuse-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ItemUse`

- <span id="itemuse-toowned-type-owned"></span>`type Owned = T`

- <span id="itemuse-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="itemuse-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ItemUse`

- <span id="crateitemitemuse-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ItemUse`

- <span id="itemuse-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itemuse-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ItemUse`

- <span id="itemuse-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itemuse-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UsePath`

```rust
struct UsePath {
    pub ident: crate::ident::Ident,
    pub colon2_token: token::PathSep,
    pub tree: Box<UseTree>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:451-459`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L451-L459)*

A path prefix of imports in a `use` item: `std::...`.

#### Trait Implementations

##### `impl Any for UsePath`

- <span id="usepath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UsePath`

- <span id="usepath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UsePath`

- <span id="usepath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UsePath`

- <span id="crateusepath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UsePath`

- <span id="usepath-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UsePath`

- <span id="crateusepath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UsePath`

##### `impl<T> From for UsePath`

- <span id="usepath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UsePath`

- <span id="crateusepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UsePath`

- <span id="usepath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UsePath`

- <span id="crateusepath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UsePath`

##### `impl Spanned for UsePath`

- <span id="usepath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UsePath`

- <span id="usepath-toowned-type-owned"></span>`type Owned = T`

- <span id="usepath-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usepath-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UsePath`

- <span id="crateitemusepath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UsePath`

- <span id="usepath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usepath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UsePath`

- <span id="usepath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usepath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseName`

```rust
struct UseName {
    pub ident: crate::ident::Ident,
}
```

*Defined in [`syn-2.0.111/src/item.rs:461-467`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L461-L467)*

An identifier imported by a `use` item: `HashMap`.

#### Trait Implementations

##### `impl Any for UseName`

- <span id="usename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseName`

- <span id="usename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseName`

- <span id="usename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseName`

- <span id="crateusename-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseName`

- <span id="usename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseName`

- <span id="crateusename-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseName`

##### `impl<T> From for UseName`

- <span id="usename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseName`

- <span id="crateusename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseName`

- <span id="usename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseName`

- <span id="crateusename-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseName`

##### `impl Spanned for UseName`

- <span id="usename-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseName`

- <span id="usename-toowned-type-owned"></span>`type Owned = T`

- <span id="usename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseName`

- <span id="crateitemusename-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseName`

- <span id="usename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseName`

- <span id="usename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseRename`

```rust
struct UseRename {
    pub ident: crate::ident::Ident,
    pub as_token: token::As,
    pub rename: crate::ident::Ident,
}
```

*Defined in [`syn-2.0.111/src/item.rs:469-477`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L469-L477)*

An renamed identifier imported by a `use` item: `HashMap as Map`.

#### Trait Implementations

##### `impl Any for UseRename`

- <span id="userename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseRename`

- <span id="userename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseRename`

- <span id="userename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseRename`

- <span id="crateuserename-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseRename`

- <span id="userename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseRename`

- <span id="crateuserename-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseRename`

##### `impl<T> From for UseRename`

- <span id="userename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseRename`

- <span id="crateuserename-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseRename`

- <span id="userename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseRename`

- <span id="crateuserename-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseRename`

##### `impl Spanned for UseRename`

- <span id="userename-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseRename`

- <span id="userename-toowned-type-owned"></span>`type Owned = T`

- <span id="userename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="userename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseRename`

- <span id="crateitemuserename-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseRename`

- <span id="userename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseRename`

- <span id="userename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseGlob`

```rust
struct UseGlob {
    pub star_token: token::Star,
}
```

*Defined in [`syn-2.0.111/src/item.rs:479-485`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L479-L485)*

A glob import in a `use` item: `*`.

#### Trait Implementations

##### `impl Any for UseGlob`

- <span id="useglob-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseGlob`

- <span id="useglob-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseGlob`

- <span id="useglob-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseGlob`

- <span id="crateuseglob-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseGlob`

- <span id="useglob-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseGlob`

- <span id="crateuseglob-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGlob`

##### `impl<T> From for UseGlob`

- <span id="useglob-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseGlob`

- <span id="crateuseglob-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for UseGlob`

- <span id="useglob-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseGlob`

- <span id="crateuseglob-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl Sealed for UseGlob`

##### `impl Spanned for UseGlob`

- <span id="useglob-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseGlob`

- <span id="useglob-toowned-type-owned"></span>`type Owned = T`

- <span id="useglob-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="useglob-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseGlob`

- <span id="crateitemuseglob-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseGlob`

- <span id="useglob-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="useglob-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseGlob`

- <span id="useglob-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="useglob-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UseGroup`

```rust
struct UseGroup {
    pub brace_token: token::Brace,
    pub items: crate::punctuated::Punctuated<UseTree, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:487-494`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L487-L494)*

A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Any for UseGroup`

- <span id="usegroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseGroup`

- <span id="usegroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseGroup`

- <span id="usegroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseGroup`

- <span id="crateusegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseGroup`

- <span id="usegroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseGroup`

- <span id="crateusegroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGroup`

##### `impl<T> From for UseGroup`

- <span id="usegroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseGroup`

- <span id="crateusegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseGroup`

- <span id="usegroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::UseGroup`

- <span id="crateusegroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseGroup`

##### `impl Spanned for UseGroup`

- <span id="usegroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseGroup`

- <span id="usegroup-toowned-type-owned"></span>`type Owned = T`

- <span id="usegroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usegroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::UseGroup`

- <span id="crateitemusegroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UseGroup`

- <span id="usegroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usegroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseGroup`

- <span id="usegroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usegroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemFn`

```rust
struct ForeignItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:542-551`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L542-L551)*

A foreign function in an `extern` block.

#### Implementations

- <span id="crateforeignitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemFn`

- <span id="foreignitemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemFn`

- <span id="foreignitemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemFn`

- <span id="foreignitemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemFn`

- <span id="crateforeignitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemFn`

- <span id="foreignitemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemFn`

- <span id="crateforeignitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemFn`

##### `impl<T> From for ForeignItemFn`

- <span id="foreignitemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemFn`

- <span id="crateforeignitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemFn`

- <span id="foreignitemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemFn`

- <span id="crateforeignitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemFn`

##### `impl Spanned for ForeignItemFn`

- <span id="foreignitemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemFn`

- <span id="foreignitemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemFn`

- <span id="crateitemforeignitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemFn`

- <span id="foreignitemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemFn`

- <span id="foreignitemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemStatic`

```rust
struct ForeignItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:553-566`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L553-L566)*

A foreign static item in an `extern` block: `static ext: u8`.

#### Implementations

- <span id="crateforeignitemstatic-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemStatic`

- <span id="foreignitemstatic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemStatic`

- <span id="foreignitemstatic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemStatic`

- <span id="foreignitemstatic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemStatic`

- <span id="foreignitemstatic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemStatic`

##### `impl<T> From for ForeignItemStatic`

- <span id="foreignitemstatic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemStatic`

- <span id="foreignitemstatic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemStatic`

- <span id="crateforeignitemstatic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemStatic`

##### `impl Spanned for ForeignItemStatic`

- <span id="foreignitemstatic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemStatic`

- <span id="foreignitemstatic-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemstatic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemstatic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemStatic`

- <span id="crateitemforeignitemstatic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemStatic`

- <span id="foreignitemstatic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemstatic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemStatic`

- <span id="foreignitemstatic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemstatic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemType`

```rust
struct ForeignItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:568-579`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L568-L579)*

A foreign type in an `extern` block: `type void`.

#### Implementations

- <span id="crateforeignitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemType`

- <span id="foreignitemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemType`

- <span id="foreignitemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemType`

- <span id="foreignitemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemType`

- <span id="crateforeignitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemType`

- <span id="foreignitemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemType`

- <span id="crateforeignitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemType`

##### `impl<T> From for ForeignItemType`

- <span id="foreignitemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemType`

- <span id="crateforeignitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemType`

- <span id="foreignitemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemType`

- <span id="crateforeignitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemType`

##### `impl Spanned for ForeignItemType`

- <span id="foreignitemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemType`

- <span id="foreignitemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemType`

- <span id="crateitemforeignitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemType`

- <span id="foreignitemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemType`

- <span id="foreignitemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ForeignItemMacro`

```rust
struct ForeignItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:581-589`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L581-L589)*

A macro invocation within an extern block.

#### Implementations

- <span id="crateforeignitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ForeignItemMacro`

- <span id="foreignitemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItemMacro`

- <span id="foreignitemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItemMacro`

- <span id="foreignitemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItemMacro`

- <span id="foreignitemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemMacro`

##### `impl<T> From for ForeignItemMacro`

- <span id="foreignitemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItemMacro`

- <span id="foreignitemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItemMacro`

- <span id="crateforeignitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItemMacro`

##### `impl Spanned for ForeignItemMacro`

- <span id="foreignitemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItemMacro`

- <span id="foreignitemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ForeignItemMacro`

- <span id="crateitemforeignitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ForeignItemMacro`

- <span id="foreignitemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItemMacro`

- <span id="foreignitemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemConst`

```rust
struct TraitItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub default: Option<(token::Eq, crate::expr::Expr)>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:637-650`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L637-L650)*

An associated constant within the definition of a trait.

#### Implementations

- <span id="cratetraititemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemConst`

- <span id="traititemconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemConst`

- <span id="traititemconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemConst`

- <span id="traititemconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemConst`

- <span id="cratetraititemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemConst`

- <span id="traititemconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemConst`

- <span id="cratetraititemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemConst`

##### `impl<T> From for TraitItemConst`

- <span id="traititemconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemConst`

- <span id="cratetraititemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemConst`

- <span id="traititemconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemConst`

- <span id="cratetraititemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemConst`

##### `impl Spanned for TraitItemConst`

- <span id="traititemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemConst`

- <span id="traititemconst-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemConst`

- <span id="crateitemtraititemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemConst`

- <span id="traititemconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemConst`

- <span id="traititemconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemFn`

```rust
struct TraitItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub sig: Signature,
    pub default: Option<crate::stmt::Block>,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:652-661`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L652-L661)*

An associated function within the definition of a trait.

#### Implementations

- <span id="cratetraititemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemFn`

- <span id="traititemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemFn`

- <span id="traititemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemFn`

- <span id="traititemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemFn`

- <span id="cratetraititemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemFn`

- <span id="traititemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemFn`

- <span id="cratetraititemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemFn`

##### `impl<T> From for TraitItemFn`

- <span id="traititemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemFn`

- <span id="cratetraititemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemFn`

- <span id="traititemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemFn`

- <span id="cratetraititemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemFn`

##### `impl Spanned for TraitItemFn`

- <span id="traititemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemFn`

- <span id="traititemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemFn`

- <span id="crateitemtraititemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemFn`

- <span id="traititemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemFn`

- <span id="traititemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemType`

```rust
struct TraitItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    pub default: Option<(token::Eq, crate::ty::Type)>,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:663-676`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L663-L676)*

An associated type within the definition of a trait.

#### Implementations

- <span id="cratetraititemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemType`

- <span id="traititemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemType`

- <span id="traititemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemType`

- <span id="traititemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemType`

- <span id="cratetraititemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemType`

- <span id="traititemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemType`

- <span id="cratetraititemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemType`

##### `impl<T> From for TraitItemType`

- <span id="traititemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemType`

- <span id="cratetraititemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemType`

- <span id="traititemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemType`

- <span id="cratetraititemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemType`

##### `impl Spanned for TraitItemType`

- <span id="traititemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemType`

- <span id="traititemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemType`

- <span id="crateitemtraititemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemType`

- <span id="traititemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemType`

- <span id="traititemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitItemMacro`

```rust
struct TraitItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:678-686`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L678-L686)*

A macro invocation within the definition of a trait.

#### Implementations

- <span id="cratetraititemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for TraitItemMacro`

- <span id="traititemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItemMacro`

- <span id="traititemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItemMacro`

- <span id="traititemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItemMacro`

- <span id="cratetraititemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItemMacro`

- <span id="traititemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItemMacro`

- <span id="cratetraititemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemMacro`

##### `impl<T> From for TraitItemMacro`

- <span id="traititemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItemMacro`

- <span id="cratetraititemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItemMacro`

- <span id="traititemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItemMacro`

- <span id="cratetraititemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItemMacro`

##### `impl Spanned for TraitItemMacro`

- <span id="traititemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItemMacro`

- <span id="traititemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="traititemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::TraitItemMacro`

- <span id="crateitemtraititemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitItemMacro`

- <span id="traititemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItemMacro`

- <span id="traititemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemConst`

```rust
struct ImplItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: token::Eq,
    pub expr: crate::expr::Expr,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:734-750`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L734-L750)*

An associated constant within an impl block.

#### Implementations

- <span id="crateimplitemconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemConst`

- <span id="implitemconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemConst`

- <span id="implitemconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemConst`

- <span id="implitemconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemConst`

- <span id="crateimplitemconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemConst`

- <span id="implitemconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemConst`

- <span id="crateimplitemconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemConst`

##### `impl<T> From for ImplItemConst`

- <span id="implitemconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemConst`

- <span id="crateimplitemconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemConst`

- <span id="implitemconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemConst`

- <span id="crateimplitemconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemConst`

##### `impl Spanned for ImplItemConst`

- <span id="implitemconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemConst`

- <span id="implitemconst-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemConst`

- <span id="crateitemimplitemconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemConst`

- <span id="implitemconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemConst`

- <span id="implitemconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemFn`

```rust
struct ImplItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub sig: Signature,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/item.rs:752-762`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L752-L762)*

An associated function within an impl block.

#### Implementations

- <span id="crateimplitemfn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemFn`

- <span id="implitemfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemFn`

- <span id="implitemfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemFn`

- <span id="implitemfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemFn`

- <span id="crateimplitemfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemFn`

- <span id="implitemfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemFn`

- <span id="crateimplitemfn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemFn`

##### `impl<T> From for ImplItemFn`

- <span id="implitemfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemFn`

- <span id="crateimplitemfn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemFn`

- <span id="implitemfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemFn`

- <span id="crateimplitemfn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemFn`

##### `impl Spanned for ImplItemFn`

- <span id="implitemfn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemFn`

- <span id="implitemfn-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemFn`

- <span id="crateitemimplitemfn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemFn`

- <span id="implitemfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemFn`

- <span id="implitemfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemType`

```rust
struct ImplItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<token::Default>,
    pub type_token: token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
    pub semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:764-778`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L764-L778)*

An associated type within an impl block.

#### Implementations

- <span id="crateimplitemtype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemType`

- <span id="implitemtype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemType`

- <span id="implitemtype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemType`

- <span id="implitemtype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemType`

- <span id="crateimplitemtype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemType`

- <span id="implitemtype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemType`

- <span id="crateimplitemtype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemType`

##### `impl<T> From for ImplItemType`

- <span id="implitemtype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemType`

- <span id="crateimplitemtype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemType`

- <span id="implitemtype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemType`

- <span id="crateimplitemtype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemType`

##### `impl Spanned for ImplItemType`

- <span id="implitemtype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemType`

- <span id="implitemtype-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemtype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemtype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemType`

- <span id="crateitemimplitemtype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemType`

- <span id="implitemtype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemtype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemType`

- <span id="implitemtype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemtype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplItemMacro`

```rust
struct ImplItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:780-788`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L780-L788)*

A macro invocation within an impl block.

#### Implementations

- <span id="crateimplitemmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ImplItemMacro`

- <span id="implitemmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItemMacro`

- <span id="implitemmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItemMacro`

- <span id="implitemmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItemMacro`

- <span id="crateimplitemmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItemMacro`

- <span id="implitemmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItemMacro`

- <span id="crateimplitemmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemMacro`

##### `impl<T> From for ImplItemMacro`

- <span id="implitemmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItemMacro`

- <span id="crateimplitemmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItemMacro`

- <span id="implitemmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItemMacro`

- <span id="crateimplitemmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItemMacro`

##### `impl Spanned for ImplItemMacro`

- <span id="implitemmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItemMacro`

- <span id="implitemmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="implitemmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitemmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::ImplItemMacro`

- <span id="crateitemimplitemmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplItemMacro`

- <span id="implitemmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitemmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItemMacro`

- <span id="implitemmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitemmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Signature`

```rust
struct Signature {
    pub constness: Option<token::Const>,
    pub asyncness: Option<token::Async>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: Option<crate::ty::Abi>,
    pub fn_token: token::Fn,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<FnArg, token::Comma>,
    pub variadic: Option<Variadic>,
    pub output: crate::ty::ReturnType,
}
```

*Defined in [`syn-2.0.111/src/item.rs:790-807`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L790-L807)*

A function signature in a trait or implementation: `unsafe fn
initialize(&self)`.

#### Implementations

- <span id="signature-receiver"></span>`fn receiver(&self) -> Option<&Receiver>` — [`Receiver`](#receiver)

  A method's `self` receiver, such as `&self` or `self: Box<Self>`.

#### Trait Implementations

##### `impl Any for Signature`

- <span id="signature-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Signature`

- <span id="signature-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Signature`

- <span id="signature-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Signature`

- <span id="cratesignature-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Signature`

- <span id="signature-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Signature`

- <span id="cratesignature-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Signature`

##### `impl<T> From for Signature`

- <span id="signature-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Signature`

- <span id="cratesignature-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Signature`

- <span id="signature-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::Signature`

- <span id="crateitemsignature-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Signature`

- <span id="cratesignature-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Signature`

##### `impl Spanned for Signature`

- <span id="signature-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Signature`

- <span id="signature-toowned-type-owned"></span>`type Owned = T`

- <span id="signature-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="signature-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::Signature`

- <span id="crateitemsignature-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Signature`

- <span id="signature-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="signature-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Signature`

- <span id="signature-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="signature-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Receiver`

```rust
struct Receiver {
    pub attrs: Vec<crate::attr::Attribute>,
    pub reference: Option<(token::And, Option<crate::lifetime::Lifetime>)>,
    pub mutability: Option<token::Mut>,
    pub self_token: token::SelfValue,
    pub colon_token: Option<token::Colon>,
    pub ty: Box<crate::ty::Type>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:832-849`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L832-L849)*

The `self` argument of an associated method.

If `colon_token` is present, the receiver is written with an explicit
type such as `self: Box<Self>`. If `colon_token` is absent, the receiver
is written in shorthand such as `self` or `&self` or `&mut self`. In the
shorthand case, the type in `ty` is reconstructed as one of `Self`,
`&Self`, or `&mut Self`.

#### Implementations

- <span id="receiver-lifetime"></span>`fn lifetime(&self) -> Option<&Lifetime>` — [`Lifetime`](../lifetime/index.md#lifetime)

#### Trait Implementations

##### `impl Any for Receiver`

- <span id="receiver-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Receiver`

- <span id="receiver-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Receiver`

- <span id="receiver-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Receiver`

- <span id="cratereceiver-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Receiver`

- <span id="receiver-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Receiver`

- <span id="cratereceiver-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Receiver`

##### `impl<T> From for Receiver`

- <span id="receiver-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Receiver`

- <span id="cratereceiver-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Receiver`

- <span id="receiver-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::Receiver`

- <span id="crateitemreceiver-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Receiver`

- <span id="cratereceiver-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Receiver`

##### `impl Spanned for Receiver`

- <span id="receiver-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Receiver`

- <span id="receiver-toowned-type-owned"></span>`type Owned = T`

- <span id="receiver-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="receiver-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::Receiver`

- <span id="crateitemreceiver-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Receiver`

- <span id="receiver-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="receiver-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Receiver`

- <span id="receiver-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="receiver-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Variadic`

```rust
struct Variadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Option<(Box<crate::pat::Pat>, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/item.rs:857-876`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L857-L876)*

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

##### `impl Any for Variadic`

- <span id="variadic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Variadic`

- <span id="variadic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Variadic`

- <span id="variadic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Variadic`

- <span id="cratevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Variadic`

- <span id="variadic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Variadic`

- <span id="cratevariadic-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variadic`

##### `impl<T> From for Variadic`

- <span id="variadic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Variadic`

- <span id="cratevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Variadic`

- <span id="variadic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Variadic`

- <span id="cratevariadic-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Variadic`

##### `impl Spanned for Variadic`

- <span id="variadic-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Variadic`

- <span id="variadic-toowned-type-owned"></span>`type Owned = T`

- <span id="variadic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variadic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::Variadic`

- <span id="crateitemvariadic-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Variadic`

- <span id="variadic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variadic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Variadic`

- <span id="variadic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variadic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

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

*Defined in [`syn-2.0.111/src/item.rs:20-99`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L20-L99)*

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

- <span id="item-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](../attr/index.md#attribute)

#### Trait Implementations

##### `impl Any for Item`

- <span id="item-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Item`

- <span id="item-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Item`

- <span id="item-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Item`

- <span id="crateitem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Item`

- <span id="item-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Item`

- <span id="crateitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Item`

##### `impl<T> From for Item`

- <span id="item-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Item`

- <span id="crateitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Item`

- <span id="item-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::Item`

- <span id="crateitemitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Item`

- <span id="crateitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Item`

##### `impl Spanned for Item`

- <span id="item-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Item`

- <span id="item-toowned-type-owned"></span>`type Owned = T`

- <span id="item-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="item-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Item`

- <span id="item-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Item`

- <span id="item-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="item-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Item`

- <span id="item-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="item-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:424-449`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L424-L449)*

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

##### `impl Any for UseTree`

- <span id="usetree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UseTree`

- <span id="usetree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UseTree`

- <span id="usetree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UseTree`

- <span id="crateusetree-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UseTree`

- <span id="usetree-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::UseTree`

- <span id="crateusetree-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseTree`

##### `impl<T> From for UseTree`

- <span id="usetree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UseTree`

- <span id="crateusetree-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UseTree`

- <span id="usetree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::UseTree`

- <span id="crateitemusetree-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<UseTree>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`UseTree`](#usetree)

##### `impl PartialEq for crate::UseTree`

- <span id="crateusetree-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UseTree`

##### `impl Spanned for UseTree`

- <span id="usetree-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UseTree`

- <span id="usetree-toowned-type-owned"></span>`type Owned = T`

- <span id="usetree-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="usetree-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for UseTree`

- <span id="usetree-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for UseTree`

- <span id="usetree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usetree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UseTree`

- <span id="usetree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usetree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:496-540`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L496-L540)*

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

##### `impl Any for ForeignItem`

- <span id="foreignitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForeignItem`

- <span id="foreignitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForeignItem`

- <span id="foreignitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ForeignItem`

- <span id="crateforeignitem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ForeignItem`

- <span id="foreignitem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ForeignItem`

- <span id="crateforeignitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItem`

##### `impl<T> From for ForeignItem`

- <span id="foreignitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ForeignItem`

- <span id="crateforeignitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ForeignItem`

- <span id="foreignitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ForeignItem`

- <span id="crateitemforeignitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ForeignItem`

- <span id="crateforeignitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ForeignItem`

##### `impl Spanned for ForeignItem`

- <span id="foreignitem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ForeignItem`

- <span id="foreignitem-toowned-type-owned"></span>`type Owned = T`

- <span id="foreignitem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foreignitem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for ForeignItem`

- <span id="foreignitem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for ForeignItem`

- <span id="foreignitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreignitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForeignItem`

- <span id="foreignitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreignitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:591-635`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L591-L635)*

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

##### `impl Any for TraitItem`

- <span id="traititem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitItem`

- <span id="traititem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitItem`

- <span id="traititem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitItem`

- <span id="cratetraititem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitItem`

- <span id="traititem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitItem`

- <span id="cratetraititem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItem`

##### `impl<T> From for TraitItem`

- <span id="traititem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitItem`

- <span id="cratetraititem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitItem`

- <span id="traititem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::TraitItem`

- <span id="crateitemtraititem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitItem`

- <span id="cratetraititem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitItem`

##### `impl Spanned for TraitItem`

- <span id="traititem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitItem`

- <span id="traititem-toowned-type-owned"></span>`type Owned = T`

- <span id="traititem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traititem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for TraitItem`

- <span id="traititem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for TraitItem`

- <span id="traititem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traititem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitItem`

- <span id="traititem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traititem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/item.rs:688-732`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L688-L732)*

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

##### `impl Any for ImplItem`

- <span id="implitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplItem`

- <span id="implitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplItem`

- <span id="implitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplItem`

- <span id="crateimplitem-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplItem`

- <span id="implitem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplItem`

- <span id="crateimplitem-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItem`

##### `impl<T> From for ImplItem`

- <span id="implitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplItem`

- <span id="crateimplitem-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ImplItem`

- <span id="implitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::ImplItem`

- <span id="crateitemimplitem-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ImplItem`

- <span id="crateimplitem-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplItem`

##### `impl Spanned for ImplItem`

- <span id="implitem-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplItem`

- <span id="implitem-toowned-type-owned"></span>`type Owned = T`

- <span id="implitem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implitem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for ImplItem`

- <span id="implitem-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for ImplItem`

- <span id="implitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplItem`

- <span id="implitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FnArg`

```rust
enum FnArg {
    Receiver(Receiver),
    Typed(crate::pat::PatType),
}
```

*Defined in [`syn-2.0.111/src/item.rs:820-830`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L820-L830)*

An argument in a function signature: the `n: usize` in `fn f(n: usize)`.

#### Variants

- **`Receiver`**

  The `self` argument of an associated method.

- **`Typed`**

  A function argument accepted by pattern and type.

#### Trait Implementations

##### `impl Any for FnArg`

- <span id="fnarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FnArg`

- <span id="fnarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FnArg`

- <span id="fnarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FnArg`

- <span id="cratefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FnArg`

- <span id="fnarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FnArg`

- <span id="cratefnarg-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FnArg`

##### `impl<T> From for FnArg`

- <span id="fnarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FnArg`

- <span id="cratefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FnArg`

- <span id="fnarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::FnArg`

- <span id="crateitemfnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::FnArg`

- <span id="cratefnarg-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FnArg`

##### `impl Spanned for FnArg`

- <span id="fnarg-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FnArg`

- <span id="fnarg-toowned-type-owned"></span>`type Owned = T`

- <span id="fnarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fnarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for FnArg`

- <span id="fnarg-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for FnArg`

- <span id="fnarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fnarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FnArg`

- <span id="fnarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fnarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StaticMutability`

```rust
enum StaticMutability {
    Mut(token::Mut),
    None,
}
```

*Defined in [`syn-2.0.111/src/item.rs:878-886`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L878-L886)*

The mutability of an `Item::Static` or `ForeignItem::Static`.

#### Trait Implementations

##### `impl Any for StaticMutability`

- <span id="staticmutability-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StaticMutability`

- <span id="staticmutability-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StaticMutability`

- <span id="staticmutability-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::StaticMutability`

- <span id="cratestaticmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for StaticMutability`

- <span id="staticmutability-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::StaticMutability`

- <span id="cratestaticmutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StaticMutability`

##### `impl<T> From for StaticMutability`

- <span id="staticmutability-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::StaticMutability`

- <span id="cratestaticmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for StaticMutability`

- <span id="staticmutability-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::StaticMutability`

- <span id="cratestaticmutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for StaticMutability`

##### `impl Spanned for StaticMutability`

- <span id="staticmutability-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for StaticMutability`

- <span id="staticmutability-toowned-type-owned"></span>`type Owned = T`

- <span id="staticmutability-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="staticmutability-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::item::StaticMutability`

- <span id="crateitemstaticmutability-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for StaticMutability`

- <span id="staticmutability-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="staticmutability-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StaticMutability`

- <span id="staticmutability-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="staticmutability-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplRestriction`

```rust
enum ImplRestriction {
}
```

*Defined in [`syn-2.0.111/src/item.rs:888-903`](../../../.source_1765633015/syn-2.0.111/src/item.rs#L888-L903)*

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Any for ImplRestriction`

- <span id="implrestriction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplRestriction`

- <span id="implrestriction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplRestriction`

- <span id="implrestriction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ImplRestriction`

- <span id="crateimplrestriction-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplRestriction`

- <span id="implrestriction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ImplRestriction`

- <span id="crateimplrestriction-debug-fmt"></span>`fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplRestriction`

##### `impl<T> From for ImplRestriction`

- <span id="implrestriction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ImplRestriction`

- <span id="crateimplrestriction-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl<U> Into for ImplRestriction`

- <span id="implrestriction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::ImplRestriction`

- <span id="crateimplrestriction-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl ToOwned for ImplRestriction`

- <span id="implrestriction-toowned-type-owned"></span>`type Owned = T`

- <span id="implrestriction-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implrestriction-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImplRestriction`

- <span id="implrestriction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implrestriction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplRestriction`

- <span id="implrestriction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implrestriction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

