*[thiserror_impl](../index.md) / [attr](index.md)*

---

# Module `attr`

## Contents

- [Structs](#structs)
  - [`Attrs`](#attrs)
  - [`Display`](#display)
  - [`Source`](#source)
  - [`From`](#from)
  - [`Transparent`](#transparent)
  - [`Fmt`](#fmt)
- [Enums](#enums)
  - [`Trait`](#trait)
- [Functions](#functions)
  - [`get`](#get)
  - [`parse_error_attribute`](#parse-error-attribute)
  - [`parse_token_expr`](#parse-token-expr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Attrs`](#attrs) | struct |  |
| [`Display`](#display) | struct |  |
| [`Source`](#source) | struct |  |
| [`From`](#from) | struct |  |
| [`Transparent`](#transparent) | struct |  |
| [`Fmt`](#fmt) | struct |  |
| [`Trait`](#trait) | enum |  |
| [`get`](#get) | fn |  |
| [`parse_error_attribute`](#parse-error-attribute) | fn |  |
| [`parse_token_expr`](#parse-token-expr) | fn |  |

## Structs

### `Attrs<'a>`

```rust
struct Attrs<'a> {
    pub display: Option<Display<'a>>,
    pub source: Option<Source<'a>>,
    pub backtrace: Option<&'a syn::Attribute>,
    pub from: Option<From<'a>>,
    pub transparent: Option<Transparent<'a>>,
    pub fmt: Option<Fmt<'a>>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:11-18`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L11-L18)*

#### Trait Implementations

##### `impl Any for Attrs<'a>`

- <span id="attrs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Attrs<'a>`

- <span id="attrs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Attrs<'a>`

- <span id="attrs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Attrs<'a>`

- <span id="attrs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Attrs<'a>`

- <span id="attrs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Attrs<'a>`

- <span id="attrs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attrs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Attrs<'a>`

- <span id="attrs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attrs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Display<'a>`

```rust
struct Display<'a> {
    pub original: &'a syn::Attribute,
    pub fmt: syn::LitStr,
    pub args: proc_macro2::TokenStream,
    pub requires_fmt_machinery: bool,
    pub has_bonus_display: bool,
    pub infinite_recursive: bool,
    pub implied_bounds: std::collections::BTreeSet<(usize, Trait)>,
    pub bindings: Vec<(syn::Ident, proc_macro2::TokenStream)>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:21-30`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L21-L30)*

#### Implementations

- <span id="crateattrdisplay-expand-shorthand"></span>`fn expand_shorthand(&mut self, fields: &[Field<'_>], container: ContainerKind) -> Result<()>` — [`Field`](../ast/index.md#field), [`ContainerKind`](../ast/index.md#containerkind)

#### Trait Implementations

##### `impl Any for Display<'a>`

- <span id="display-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Display<'a>`

- <span id="display-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Display<'a>`

- <span id="display-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Display<'a>`

- <span id="display-clone"></span>`fn clone(&self) -> Display<'a>` — [`Display`](#display)

##### `impl CloneToUninit for Display<'a>`

- <span id="display-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Display<'a>`

- <span id="display-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Display<'a>`

- <span id="display-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Display<'a>`

- <span id="display-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Display<'a>`

- <span id="display-toowned-type-owned"></span>`type Owned = T`

- <span id="display-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="display-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Display<'_>`

- <span id="display-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Display<'a>`

- <span id="display-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="display-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Display<'a>`

- <span id="display-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="display-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Source<'a>`

```rust
struct Source<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:33-36`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L33-L36)*

#### Trait Implementations

##### `impl Any for Source<'a>`

- <span id="source-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Source<'a>`

- <span id="source-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Source<'a>`

- <span id="source-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Source<'a>`

- <span id="source-clone"></span>`fn clone(&self) -> Source<'a>` — [`Source`](#source)

##### `impl CloneToUninit for Source<'a>`

- <span id="source-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Source<'a>`

##### `impl<T> From for Source<'a>`

- <span id="source-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Source<'a>`

- <span id="source-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Source<'a>`

- <span id="source-toowned-type-owned"></span>`type Owned = T`

- <span id="source-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="source-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Source<'a>`

- <span id="source-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="source-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Source<'a>`

- <span id="source-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="source-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `From<'a>`

```rust
struct From<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:39-42`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L39-L42)*

#### Trait Implementations

##### `impl Any for From<'a>`

- <span id="from-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for From<'a>`

- <span id="from-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for From<'a>`

- <span id="from-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for From<'a>`

- <span id="from-clone"></span>`fn clone(&self) -> From<'a>` — [`From`](#from)

##### `impl CloneToUninit for From<'a>`

- <span id="from-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for From<'a>`

##### `impl<T> From for From<'a>`

- <span id="from-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for From<'a>`

- <span id="from-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for From<'a>`

- <span id="from-toowned-type-owned"></span>`type Owned = T`

- <span id="from-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="from-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for From<'a>`

- <span id="from-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="from-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for From<'a>`

- <span id="from-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="from-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Transparent<'a>`

```rust
struct Transparent<'a> {
    pub original: &'a syn::Attribute,
    pub span: proc_macro2::Span,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:45-48`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L45-L48)*

#### Trait Implementations

##### `impl Any for Transparent<'a>`

- <span id="transparent-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Transparent<'a>`

- <span id="transparent-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Transparent<'a>`

- <span id="transparent-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Transparent<'a>`

- <span id="transparent-clone"></span>`fn clone(&self) -> Transparent<'a>` — [`Transparent`](#transparent)

##### `impl CloneToUninit for Transparent<'a>`

- <span id="transparent-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Transparent<'a>`

##### `impl<T> From for Transparent<'a>`

- <span id="transparent-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Transparent<'a>`

- <span id="transparent-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Transparent<'a>`

- <span id="transparent-toowned-type-owned"></span>`type Owned = T`

- <span id="transparent-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="transparent-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Transparent<'a>`

- <span id="transparent-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="transparent-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Transparent<'a>`

- <span id="transparent-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="transparent-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fmt<'a>`

```rust
struct Fmt<'a> {
    pub original: &'a syn::Attribute,
    pub path: syn::ExprPath,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:51-54`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L51-L54)*

#### Trait Implementations

##### `impl Any for Fmt<'a>`

- <span id="fmt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fmt<'a>`

- <span id="fmt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fmt<'a>`

- <span id="fmt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Fmt<'a>`

- <span id="fmt-clone"></span>`fn clone(&self) -> Fmt<'a>` — [`Fmt`](#fmt)

##### `impl CloneToUninit for Fmt<'a>`

- <span id="fmt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Fmt<'a>`

- <span id="fmt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fmt<'a>`

- <span id="fmt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Fmt<'a>`

- <span id="fmt-toowned-type-owned"></span>`type Owned = T`

- <span id="fmt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fmt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Fmt<'a>`

- <span id="fmt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fmt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fmt<'a>`

- <span id="fmt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fmt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Trait`

```rust
enum Trait {
    Debug,
    Display,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
}
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:57-67`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L57-L67)*

#### Trait Implementations

##### `impl Any for Trait`

- <span id="trait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Trait`

- <span id="trait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Trait`

- <span id="trait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Trait`

- <span id="trait-clone"></span>`fn clone(&self) -> Trait` — [`Trait`](#trait)

##### `impl CloneToUninit for Trait`

- <span id="trait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Trait`

##### `impl Debug for Trait`

- <span id="trait-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Trait`

##### `impl<T> From for Trait`

- <span id="trait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Trait`

- <span id="trait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Trait`

- <span id="trait-ord-cmp"></span>`fn cmp(&self, other: &Trait) -> cmp::Ordering` — [`Trait`](#trait)

##### `impl PartialEq for Trait`

- <span id="trait-partialeq-eq"></span>`fn eq(&self, other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl PartialOrd for Trait`

- <span id="trait-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Trait) -> option::Option<cmp::Ordering>` — [`Trait`](#trait)

##### `impl Spanned for Trait`

- <span id="trait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl StructuralPartialEq for Trait`

##### `impl ToOwned for Trait`

- <span id="trait-toowned-type-owned"></span>`type Owned = T`

- <span id="trait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="trait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Trait`

- <span id="trait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Trait`

- <span id="trait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Trait`

- <span id="trait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `get`

```rust
fn get(input: &[syn::Attribute]) -> syn::Result<Attrs<'_>>
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:69-122`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L69-L122)*

### `parse_error_attribute`

```rust
fn parse_error_attribute<'a>(attrs: &mut Attrs<'a>, attr: &'a syn::Attribute) -> syn::Result<()>
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:124-194`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L124-L194)*

### `parse_token_expr`

```rust
fn parse_token_expr(input: syn::parse::ParseStream<'_>, begin_expr: bool) -> syn::Result<proc_macro2::TokenStream>
```

*Defined in [`thiserror-impl-2.0.17/src/attr.rs:196-300`](../../../.source_1765633015/thiserror-impl-2.0.17/src/attr.rs#L196-L300)*

