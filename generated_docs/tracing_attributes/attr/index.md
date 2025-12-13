*[tracing_attributes](../index.md) / [attr](index.md)*

---

# Module `attr`

## Contents

- [Modules](#modules)
  - [`kw`](#kw)
- [Structs](#structs)
  - [`EventArgs`](#eventargs)
  - [`InstrumentArgs`](#instrumentargs)
  - [`StrArg`](#strarg)
  - [`ExprArg`](#exprarg)
  - [`Skips`](#skips)
  - [`Fields`](#fields)
  - [`Field`](#field)
- [Enums](#enums)
  - [`LitStrOrIdent`](#litstrorident)
  - [`FormatMode`](#formatmode)
  - [`FieldKind`](#fieldkind)
  - [`FieldName`](#fieldname)
  - [`Level`](#level)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`kw`](#kw) | mod |  |
| [`EventArgs`](#eventargs) | struct | Arguments to `#[instrument(err(...))]` and `#[instrument(ret(...))]` which describe how the return value event should be emitted. |
| [`InstrumentArgs`](#instrumentargs) | struct |  |
| [`StrArg`](#strarg) | struct |  |
| [`ExprArg`](#exprarg) | struct |  |
| [`Skips`](#skips) | struct |  |
| [`Fields`](#fields) | struct |  |
| [`Field`](#field) | struct |  |
| [`LitStrOrIdent`](#litstrorident) | enum |  |
| [`FormatMode`](#formatmode) | enum |  |
| [`FieldKind`](#fieldkind) | enum |  |
| [`FieldName`](#fieldname) | enum |  |
| [`Level`](#level) | enum |  |

## Modules

- [`kw`](kw/index.md)

## Structs

### `EventArgs`

```rust
struct EventArgs {
    level: Option<Level>,
    mode: FormatMode,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:13-16`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L13-L16)*

Arguments to `#[instrument(err(...))]` and `#[instrument(ret(...))]` which describe how the
return value event should be emitted.

#### Implementations

- <span id="eventargs-level"></span>`fn level(&self, default: Level) -> Level` — [`Level`](#level)

#### Trait Implementations

##### `impl Any for EventArgs`

- <span id="eventargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EventArgs`

- <span id="eventargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EventArgs`

- <span id="eventargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EventArgs`

- <span id="eventargs-clone"></span>`fn clone(&self) -> EventArgs` — [`EventArgs`](#eventargs)

##### `impl CloneToUninit for EventArgs`

- <span id="eventargs-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for EventArgs`

- <span id="eventargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EventArgs`

- <span id="eventargs-default"></span>`fn default() -> EventArgs` — [`EventArgs`](#eventargs)

##### `impl<T> From for EventArgs`

- <span id="eventargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EventArgs`

- <span id="eventargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for EventArgs`

- <span id="eventargs-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl ToOwned for EventArgs`

- <span id="eventargs-toowned-type-owned"></span>`type Owned = T`

- <span id="eventargs-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="eventargs-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EventArgs`

- <span id="eventargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="eventargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EventArgs`

- <span id="eventargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="eventargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InstrumentArgs`

```rust
struct InstrumentArgs {
    level: Option<Level>,
    name: Option<LitStrOrIdent>,
    target: Option<LitStrOrIdent>,
    parent: Option<syn::Expr>,
    follows_from: Option<syn::Expr>,
    skips: std::collections::HashSet<syn::Ident>,
    skip_all: bool,
    fields: Option<Fields>,
    err_args: Option<EventArgs>,
    ret_args: Option<EventArgs>,
    parse_warnings: Vec<syn::Error>,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:19-32`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L19-L32)*

#### Fields

- **`parse_warnings`**: `Vec<syn::Error>`

  Errors describing any unrecognized parse inputs that we skipped.

#### Implementations

- <span id="instrumentargs-level"></span>`fn level(&self) -> Level` — [`Level`](#level)

- <span id="instrumentargs-target"></span>`fn target(&self) -> impl ToTokens`

- <span id="instrumentargs-warnings"></span>`fn warnings(&self) -> impl ToTokens`

  Generate "deprecation" warnings for any unrecognized attribute inputs

  that we skipped.

  

  For backwards compatibility, we need to emit compiler warnings rather

  than errors for unrecognized inputs. Generating a fake deprecation is

  the only way to do this on stable Rust right now.

#### Trait Implementations

##### `impl Any for InstrumentArgs`

- <span id="instrumentargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InstrumentArgs`

- <span id="instrumentargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InstrumentArgs`

- <span id="instrumentargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for InstrumentArgs`

- <span id="instrumentargs-clone"></span>`fn clone(&self) -> InstrumentArgs` — [`InstrumentArgs`](#instrumentargs)

##### `impl CloneToUninit for InstrumentArgs`

- <span id="instrumentargs-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for InstrumentArgs`

- <span id="instrumentargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for InstrumentArgs`

- <span id="instrumentargs-default"></span>`fn default() -> InstrumentArgs` — [`InstrumentArgs`](#instrumentargs)

##### `impl<T> From for InstrumentArgs`

- <span id="instrumentargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InstrumentArgs`

- <span id="instrumentargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for InstrumentArgs`

- <span id="instrumentargs-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl ToOwned for InstrumentArgs`

- <span id="instrumentargs-toowned-type-owned"></span>`type Owned = T`

- <span id="instrumentargs-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="instrumentargs-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for InstrumentArgs`

- <span id="instrumentargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="instrumentargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InstrumentArgs`

- <span id="instrumentargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="instrumentargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StrArg<T>`

```rust
struct StrArg<T> {
    value: LitStrOrIdent,
    _p: std::marker::PhantomData<T>,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:241-244`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L241-L244)*

#### Trait Implementations

##### `impl<T> Any for StrArg<T>`

- <span id="strarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StrArg<T>`

- <span id="strarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrArg<T>`

- <span id="strarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StrArg<T>`

- <span id="strarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for StrArg<T>`

- <span id="strarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Parse> Parse for StrArg<T>`

- <span id="strarg-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T, U> TryFrom for StrArg<T>`

- <span id="strarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for StrArg<T>`

- <span id="strarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprArg<T>`

```rust
struct ExprArg<T> {
    value: syn::Expr,
    _p: std::marker::PhantomData<T>,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:258-261`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L258-L261)*

#### Trait Implementations

##### `impl<T> Any for ExprArg<T>`

- <span id="exprarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprArg<T>`

- <span id="exprarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprArg<T>`

- <span id="exprarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ExprArg<T>`

- <span id="exprarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ExprArg<T>`

- <span id="exprarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Parse> Parse for ExprArg<T>`

- <span id="exprarg-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T, U> TryFrom for ExprArg<T>`

- <span id="exprarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ExprArg<T>`

- <span id="exprarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Skips`

```rust
struct Skips(std::collections::HashSet<syn::Ident>);
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:275`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L275)*

#### Trait Implementations

##### `impl Any for Skips`

- <span id="skips-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Skips`

- <span id="skips-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Skips`

- <span id="skips-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Skips`

- <span id="skips-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Skips`

- <span id="skips-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Skips`

- <span id="skips-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for Skips`

- <span id="skips-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skips-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Skips`

- <span id="skips-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skips-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fields`

```rust
struct Fields(syn::punctuated::Punctuated<Field, token::Comma>);
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:307`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L307)*

#### Trait Implementations

##### `impl Any for Fields`

- <span id="fields-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fields`

- <span id="fields-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fields`

- <span id="fields-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Fields`

- <span id="fields-clone"></span>`fn clone(&self) -> Fields` — [`Fields`](#fields)

##### `impl CloneToUninit for Fields`

- <span id="fields-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Fields`

- <span id="fields-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Fields`

- <span id="fields-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fields`

- <span id="fields-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Fields`

- <span id="fields-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl Spanned for Fields`

- <span id="fields-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Fields`

- <span id="fields-toowned-type-owned"></span>`type Owned = T`

- <span id="fields-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fields-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Fields`

- <span id="fields-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Fields`

- <span id="fields-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fields-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fields`

- <span id="fields-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fields-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Field`

```rust
struct Field {
    name: FieldName,
    value: Option<syn::Expr>,
    kind: FieldKind,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:310-314`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L310-L314)*

#### Trait Implementations

##### `impl Any for Field`

- <span id="field-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Field`

- <span id="field-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Field`

- <span id="field-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Field`

- <span id="field-clone"></span>`fn clone(&self) -> Field` — [`Field`](#field)

##### `impl CloneToUninit for Field`

- <span id="field-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Field`

- <span id="field-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Field`

- <span id="field-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Field`

- <span id="field-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Field`

- <span id="field-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl Spanned for Field`

- <span id="field-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Field`

- <span id="field-toowned-type-owned"></span>`type Owned = T`

- <span id="field-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="field-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Field`

- <span id="field-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Field`

- <span id="field-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="field-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Field`

- <span id="field-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="field-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `LitStrOrIdent`

```rust
enum LitStrOrIdent {
    LitStr(syn::LitStr),
    Ident(syn::Ident),
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:218-221`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L218-L221)*

#### Trait Implementations

##### `impl Any for LitStrOrIdent`

- <span id="litstrorident-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LitStrOrIdent`

- <span id="litstrorident-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LitStrOrIdent`

- <span id="litstrorident-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LitStrOrIdent`

- <span id="litstrorident-clone"></span>`fn clone(&self) -> LitStrOrIdent` — [`LitStrOrIdent`](#litstrorident)

##### `impl CloneToUninit for LitStrOrIdent`

- <span id="litstrorident-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for LitStrOrIdent`

- <span id="litstrorident-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LitStrOrIdent`

- <span id="litstrorident-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LitStrOrIdent`

- <span id="litstrorident-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for LitStrOrIdent`

- <span id="litstrorident-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl Spanned for LitStrOrIdent`

- <span id="litstrorident-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LitStrOrIdent`

- <span id="litstrorident-toowned-type-owned"></span>`type Owned = T`

- <span id="litstrorident-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="litstrorident-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for LitStrOrIdent`

- <span id="litstrorident-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for LitStrOrIdent`

- <span id="litstrorident-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="litstrorident-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LitStrOrIdent`

- <span id="litstrorident-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="litstrorident-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FormatMode`

```rust
enum FormatMode {
    Default,
    Display,
    Debug,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:299-304`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L299-L304)*

#### Trait Implementations

##### `impl Any for FormatMode`

- <span id="formatmode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FormatMode`

- <span id="formatmode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FormatMode`

- <span id="formatmode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FormatMode`

- <span id="formatmode-clone"></span>`fn clone(&self) -> FormatMode` — [`FormatMode`](#formatmode)

##### `impl CloneToUninit for FormatMode`

- <span id="formatmode-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FormatMode`

- <span id="formatmode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FormatMode`

- <span id="formatmode-default"></span>`fn default() -> FormatMode` — [`FormatMode`](#formatmode)

##### `impl Eq for FormatMode`

##### `impl<T> From for FormatMode`

- <span id="formatmode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FormatMode`

- <span id="formatmode-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FormatMode`

- <span id="formatmode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FormatMode`

- <span id="formatmode-partialeq-eq"></span>`fn eq(&self, other: &FormatMode) -> bool` — [`FormatMode`](#formatmode)

##### `impl StructuralPartialEq for FormatMode`

##### `impl ToOwned for FormatMode`

- <span id="formatmode-toowned-type-owned"></span>`type Owned = T`

- <span id="formatmode-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="formatmode-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FormatMode`

- <span id="formatmode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="formatmode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FormatMode`

- <span id="formatmode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="formatmode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldKind`

```rust
enum FieldKind {
    Debug,
    Display,
    Value,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:317-321`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L317-L321)*

#### Trait Implementations

##### `impl Any for FieldKind`

- <span id="fieldkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldKind`

- <span id="fieldkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldKind`

- <span id="fieldkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FieldKind`

- <span id="fieldkind-clone"></span>`fn clone(&self) -> FieldKind` — [`FieldKind`](#fieldkind)

##### `impl CloneToUninit for FieldKind`

- <span id="fieldkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FieldKind`

- <span id="fieldkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FieldKind`

##### `impl<T> From for FieldKind`

- <span id="fieldkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FieldKind`

- <span id="fieldkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FieldKind`

- <span id="fieldkind-partialeq-eq"></span>`fn eq(&self, other: &FieldKind) -> bool` — [`FieldKind`](#fieldkind)

##### `impl Spanned for FieldKind`

- <span id="fieldkind-spanned-span"></span>`fn span(&self) -> Span`

##### `impl StructuralPartialEq for FieldKind`

##### `impl ToOwned for FieldKind`

- <span id="fieldkind-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for FieldKind`

- <span id="fieldkind-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldKind`

- <span id="fieldkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldKind`

- <span id="fieldkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldName`

```rust
enum FieldName {
    Expr(syn::Expr),
    Punctuated(syn::punctuated::Punctuated<syn::Ident, token::Dot>),
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:324-327`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L324-L327)*

#### Trait Implementations

##### `impl Any for FieldName`

- <span id="fieldname-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldName`

- <span id="fieldname-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldName`

- <span id="fieldname-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FieldName`

- <span id="fieldname-clone"></span>`fn clone(&self) -> FieldName` — [`FieldName`](#fieldname)

##### `impl CloneToUninit for FieldName`

- <span id="fieldname-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FieldName`

- <span id="fieldname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FieldName`

- <span id="fieldname-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FieldName`

- <span id="fieldname-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for FieldName`

- <span id="fieldname-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldName`

- <span id="fieldname-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldname-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldname-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for FieldName`

- <span id="fieldname-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldName`

- <span id="fieldname-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldname-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldName`

- <span id="fieldname-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldname-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Level`

```rust
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Path(syn::Path),
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:429-436`](../../../.source_1765521767/tracing-attributes-0.1.31/src/attr.rs#L429-L436)*

#### Trait Implementations

##### `impl Any for Level`

- <span id="level-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Level`

- <span id="level-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Level`

- <span id="level-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Level`

- <span id="level-clone"></span>`fn clone(&self) -> Level` — [`Level`](#level)

##### `impl CloneToUninit for Level`

- <span id="level-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Level`

- <span id="level-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Level`

- <span id="level-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Level`

- <span id="level-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Level`

- <span id="level-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl Spanned for Level`

- <span id="level-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Level`

- <span id="level-toowned-type-owned"></span>`type Owned = T`

- <span id="level-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="level-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Level`

- <span id="level-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Level`

- <span id="level-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="level-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Level`

- <span id="level-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="level-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

