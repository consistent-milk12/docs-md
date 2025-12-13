*[clap_derive](../index.md) / [attr](index.md)*

---

# Module `attr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ClapAttr`](#clapattr) | struct |  |
| [`MagicAttrName`](#magicattrname) | enum |  |
| [`AttrValue`](#attrvalue) | enum |  |
| [`AttrKind`](#attrkind) | enum |  |

## Structs

### `ClapAttr`

```rust
struct ClapAttr {
    kind: self::spanned::Sp<AttrKind>,
    name: syn::Ident,
    magic: Option<MagicAttrName>,
    value: Option<AttrValue>,
}
```

*Defined in [`clap_derive-4.5.49/src/attr.rs:17-22`](../../../.source_1765633015/clap_derive-4.5.49/src/attr.rs#L17-L22)*

#### Implementations

- <span id="clapattr-parse-all"></span>`fn parse_all(all_attrs: &[Attribute]) -> Result<Vec<Self>, syn::Error>`

- <span id="clapattr-value-or-abort"></span>`fn value_or_abort(&self) -> Result<&AttrValue, syn::Error>` — [`AttrValue`](#attrvalue)

- <span id="clapattr-lit-str-or-abort"></span>`fn lit_str_or_abort(&self) -> Result<&LitStr, syn::Error>`

#### Trait Implementations

##### `impl Any for ClapAttr`

- <span id="clapattr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClapAttr`

- <span id="clapattr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClapAttr`

- <span id="clapattr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClapAttr`

- <span id="clapattr-clone"></span>`fn clone(&self) -> ClapAttr` — [`ClapAttr`](#clapattr)

##### `impl CloneToUninit for ClapAttr`

- <span id="clapattr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for ClapAttr`

- <span id="clapattr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClapAttr`

- <span id="clapattr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for ClapAttr`

- <span id="clapattr-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl ToOwned for ClapAttr`

- <span id="clapattr-toowned-type-owned"></span>`type Owned = T`

- <span id="clapattr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="clapattr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClapAttr`

- <span id="clapattr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="clapattr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClapAttr`

- <span id="clapattr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="clapattr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MagicAttrName`

```rust
enum MagicAttrName {
    Short,
    Long,
    ValueParser,
    Action,
    Env,
    Flatten,
    ValueEnum,
    FromGlobal,
    Subcommand,
    VerbatimDocComment,
    ExternalSubcommand,
    About,
    LongAbout,
    LongHelp,
    Author,
    Version,
    RenameAllEnv,
    RenameAll,
    Skip,
    DefaultValueT,
    DefaultValuesT,
    DefaultValueOsT,
    DefaultValuesOsT,
    NextDisplayOrder,
    NextHelpHeading,
}
```

*Defined in [`clap_derive-4.5.49/src/attr.rs:145-171`](../../../.source_1765633015/clap_derive-4.5.49/src/attr.rs#L145-L171)*

#### Trait Implementations

##### `impl Any for MagicAttrName`

- <span id="magicattrname-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MagicAttrName`

- <span id="magicattrname-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MagicAttrName`

- <span id="magicattrname-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MagicAttrName`

- <span id="magicattrname-clone"></span>`fn clone(&self) -> MagicAttrName` — [`MagicAttrName`](#magicattrname)

##### `impl CloneToUninit for MagicAttrName`

- <span id="magicattrname-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MagicAttrName`

##### `impl Eq for MagicAttrName`

##### `impl<T> From for MagicAttrName`

- <span id="magicattrname-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MagicAttrName`

- <span id="magicattrname-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MagicAttrName`

- <span id="magicattrname-partialeq-eq"></span>`fn eq(&self, other: &MagicAttrName) -> bool` — [`MagicAttrName`](#magicattrname)

##### `impl StructuralPartialEq for MagicAttrName`

##### `impl ToOwned for MagicAttrName`

- <span id="magicattrname-toowned-type-owned"></span>`type Owned = T`

- <span id="magicattrname-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="magicattrname-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MagicAttrName`

- <span id="magicattrname-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="magicattrname-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MagicAttrName`

- <span id="magicattrname-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="magicattrname-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttrValue`

```rust
enum AttrValue {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
    Call(Vec<syn::Expr>),
}
```

*Defined in [`clap_derive-4.5.49/src/attr.rs:175-179`](../../../.source_1765633015/clap_derive-4.5.49/src/attr.rs#L175-L179)*

#### Trait Implementations

##### `impl Any for AttrValue`

- <span id="attrvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttrValue`

- <span id="attrvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttrValue`

- <span id="attrvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttrValue`

- <span id="attrvalue-clone"></span>`fn clone(&self) -> AttrValue` — [`AttrValue`](#attrvalue)

##### `impl CloneToUninit for AttrValue`

- <span id="attrvalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for AttrValue`

- <span id="attrvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttrValue`

- <span id="attrvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for AttrValue`

- <span id="attrvalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AttrValue`

- <span id="attrvalue-toowned-type-owned"></span>`type Owned = T`

- <span id="attrvalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attrvalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for AttrValue`

- <span id="attrvalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for AttrValue`

- <span id="attrvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attrvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttrValue`

- <span id="attrvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attrvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttrKind`

```rust
enum AttrKind {
    Clap,
    StructOpt,
    Command,
    Group,
    Arg,
    Value,
}
```

*Defined in [`clap_derive-4.5.49/src/attr.rs:195-202`](../../../.source_1765633015/clap_derive-4.5.49/src/attr.rs#L195-L202)*

#### Implementations

- <span id="attrkind-as-str"></span>`fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Any for AttrKind`

- <span id="attrkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttrKind`

- <span id="attrkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttrKind`

- <span id="attrkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttrKind`

- <span id="attrkind-clone"></span>`fn clone(&self) -> AttrKind` — [`AttrKind`](#attrkind)

##### `impl CloneToUninit for AttrKind`

- <span id="attrkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AttrKind`

##### `impl Eq for AttrKind`

##### `impl<T> From for AttrKind`

- <span id="attrkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttrKind`

- <span id="attrkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AttrKind`

- <span id="attrkind-partialeq-eq"></span>`fn eq(&self, other: &AttrKind) -> bool` — [`AttrKind`](#attrkind)

##### `impl StructuralPartialEq for AttrKind`

##### `impl ToOwned for AttrKind`

- <span id="attrkind-toowned-type-owned"></span>`type Owned = T`

- <span id="attrkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attrkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttrKind`

- <span id="attrkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attrkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttrKind`

- <span id="attrkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attrkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

