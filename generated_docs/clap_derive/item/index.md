*[clap_derive](../index.md) / [item](index.md)*

---

# Module `item`

## Contents

- [Structs](#structs)
  - [`Item`](#item)
  - [`Method`](#method)
  - [`Deprecation`](#deprecation)
- [Enums](#enums)
  - [`ValueParser`](#valueparser)
  - [`Action`](#action)
  - [`Kind`](#kind)
  - [`CasingStyle`](#casingstyle)
  - [`Name`](#name)
- [Functions](#functions)
  - [`default_value_parser`](#default-value-parser)
  - [`default_action`](#default-action)
  - [`assert_attr_kind`](#assert-attr-kind)
  - [`process_author_str`](#process-author-str)
- [Constants](#constants)
  - [`DEFAULT_CASING`](#default-casing)
  - [`DEFAULT_ENV_CASING`](#default-env-casing)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Item`](#item) | struct |  |
| [`Method`](#method) | struct |  |
| [`Deprecation`](#deprecation) | struct |  |
| [`ValueParser`](#valueparser) | enum |  |
| [`Action`](#action) | enum |  |
| [`Kind`](#kind) | enum |  |
| [`CasingStyle`](#casingstyle) | enum | Defines the casing for the attributes long representation. |
| [`Name`](#name) | enum |  |
| [`default_value_parser`](#default-value-parser) | fn |  |
| [`default_action`](#default-action) | fn |  |
| [`assert_attr_kind`](#assert-attr-kind) | fn |  |
| [`process_author_str`](#process-author-str) | fn | replace all `:` with `, ` when not inside the `<>` |
| [`DEFAULT_CASING`](#default-casing) | const | Default casing style for generated arguments. |
| [`DEFAULT_ENV_CASING`](#default-env-casing) | const | Default casing style for environment variables |

## Structs

### `Item`

```rust
struct Item {
    name: Name,
    casing: self::spanned::Sp<CasingStyle>,
    env_casing: self::spanned::Sp<CasingStyle>,
    ty: Option<syn::Type>,
    doc_comment: Vec<Method>,
    methods: Vec<Method>,
    deprecations: Vec<Deprecation>,
    value_parser: Option<ValueParser>,
    action: Option<Action>,
    verbatim_doc_comment: bool,
    force_long_help: bool,
    next_display_order: Option<Method>,
    next_help_heading: Option<Method>,
    is_enum: bool,
    is_positional: bool,
    skip_group: bool,
    group_id: Name,
    group_methods: Vec<Method>,
    kind: self::spanned::Sp<Kind>,
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:33-53`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L33-L53)*

#### Implementations

- <span id="item-from-args-struct"></span>`fn from_args_struct(input: &DeriveInput, name: Name) -> Result<Self, syn::Error>` — [`Name`](#name)

- <span id="item-from-subcommand-enum"></span>`fn from_subcommand_enum(input: &DeriveInput, name: Name) -> Result<Self, syn::Error>` — [`Name`](#name)

- <span id="item-from-value-enum"></span>`fn from_value_enum(input: &DeriveInput, name: Name) -> Result<Self, syn::Error>` — [`Name`](#name)

- <span id="item-from-subcommand-variant"></span>`fn from_subcommand_variant(variant: &Variant, struct_casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>) -> Result<Self, syn::Error>` — [`Sp`](../utils/spanned/index.md#sp), [`CasingStyle`](#casingstyle)

- <span id="item-from-value-enum-variant"></span>`fn from_value_enum_variant(variant: &Variant, argument_casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>) -> Result<Self, syn::Error>` — [`Sp`](../utils/spanned/index.md#sp), [`CasingStyle`](#casingstyle)

- <span id="item-from-args-field"></span>`fn from_args_field(field: &Field, struct_casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>) -> Result<Self, syn::Error>` — [`Sp`](../utils/spanned/index.md#sp), [`CasingStyle`](#casingstyle)

- <span id="item-new"></span>`fn new(name: Name, ident: Ident, ty: Option<Type>, casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>, kind: Sp<Kind>) -> Self` — [`Name`](#name), [`Sp`](../utils/spanned/index.md#sp), [`CasingStyle`](#casingstyle), [`Kind`](#kind)

- <span id="item-push-method"></span>`fn push_method(&mut self, kind: AttrKind, name: Ident, arg: impl ToTokens)` — [`AttrKind`](../attr/index.md#attrkind)

- <span id="item-push-method"></span>`fn push_method_(&mut self, kind: AttrKind, name: Ident, arg: TokenStream)` — [`AttrKind`](../attr/index.md#attrkind)

- <span id="item-infer-kind"></span>`fn infer_kind(&mut self, attrs: &[ClapAttr]) -> Result<(), syn::Error>` — [`ClapAttr`](../attr/index.md#clapattr)

- <span id="item-push-attrs"></span>`fn push_attrs(&mut self, attrs: &[ClapAttr]) -> Result<(), syn::Error>` — [`ClapAttr`](../attr/index.md#clapattr)

- <span id="item-push-doc-comment"></span>`fn push_doc_comment(&mut self, attrs: &[Attribute], short_name: &str, long_name: Option<&str>)`

- <span id="item-set-kind"></span>`fn set_kind(&mut self, kind: Sp<Kind>) -> Result<(), syn::Error>` — [`Sp`](../utils/spanned/index.md#sp), [`Kind`](#kind)

- <span id="item-find-default-method"></span>`fn find_default_method(&self) -> Option<&Method>` — [`Method`](#method)

- <span id="item-initial-top-level-methods"></span>`fn initial_top_level_methods(&self) -> TokenStream`

  generate methods from attributes on top of struct or enum

- <span id="item-final-top-level-methods"></span>`fn final_top_level_methods(&self) -> TokenStream`

- <span id="item-field-methods"></span>`fn field_methods(&self) -> TokenStream`

  generate methods on top of a field

- <span id="item-group-id"></span>`fn group_id(&self) -> &Name` — [`Name`](#name)

- <span id="item-group-methods"></span>`fn group_methods(&self) -> TokenStream`

- <span id="item-deprecations"></span>`fn deprecations(&self) -> TokenStream`

- <span id="item-next-display-order"></span>`fn next_display_order(&self) -> TokenStream`

- <span id="item-next-help-heading"></span>`fn next_help_heading(&self) -> TokenStream`

- <span id="item-id"></span>`fn id(&self) -> &Name` — [`Name`](#name)

- <span id="item-cased-name"></span>`fn cased_name(&self) -> TokenStream`

- <span id="item-value-name"></span>`fn value_name(&self) -> TokenStream`

- <span id="item-value-parser"></span>`fn value_parser(&self, field_type: &Type) -> Method` — [`Method`](#method)

- <span id="item-action"></span>`fn action(&self, field_type: &Type) -> Method` — [`Method`](#method)

- <span id="item-kind"></span>`fn kind(&self) -> Sp<Kind>` — [`Sp`](../utils/spanned/index.md#sp), [`Kind`](#kind)

- <span id="item-is-positional"></span>`fn is_positional(&self) -> bool`

- <span id="item-casing"></span>`fn casing(&self) -> Sp<CasingStyle>` — [`Sp`](../utils/spanned/index.md#sp), [`CasingStyle`](#casingstyle)

- <span id="item-env-casing"></span>`fn env_casing(&self) -> Sp<CasingStyle>` — [`Sp`](../utils/spanned/index.md#sp), [`CasingStyle`](#casingstyle)

- <span id="item-has-explicit-methods"></span>`fn has_explicit_methods(&self) -> bool`

- <span id="item-skip-group"></span>`fn skip_group(&self) -> bool`

#### Trait Implementations

##### `impl Any for Item`

- <span id="item-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Item`

- <span id="item-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Item`

- <span id="item-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Item`

- <span id="item-clone"></span>`fn clone(&self) -> Item` — [`Item`](#item)

##### `impl CloneToUninit for Item`

- <span id="item-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Item`

- <span id="item-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Item`

- <span id="item-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Item`

- <span id="item-toowned-type-owned"></span>`type Owned = T`

- <span id="item-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="item-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Item`

- <span id="item-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="item-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Item`

- <span id="item-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="item-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Method`

```rust
struct Method {
    name: syn::Ident,
    args: proc_macro2::TokenStream,
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1232-1235`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1232-L1235)*

#### Implementations

- <span id="method-new"></span>`fn new(name: Ident, args: TokenStream) -> Self`

- <span id="method-from-env"></span>`fn from_env(ident: Ident, env_var: &str) -> Result<Option<Self>, syn::Error>`

- <span id="method-args"></span>`fn args(&self) -> &TokenStream`

#### Trait Implementations

##### `impl Any for Method`

- <span id="method-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Method`

- <span id="method-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Method`

- <span id="method-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Method`

- <span id="method-clone"></span>`fn clone(&self) -> Method` — [`Method`](#method)

##### `impl CloneToUninit for Method`

- <span id="method-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Method`

- <span id="method-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Method`

- <span id="method-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Method`

- <span id="method-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Method`

- <span id="method-toowned-type-owned"></span>`type Owned = T`

- <span id="method-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="method-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Method`

- <span id="method-totokens-to-tokens"></span>`fn to_tokens(&self, ts: &mut TokenStream)`

##### `impl<U> TryFrom for Method`

- <span id="method-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="method-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Method`

- <span id="method-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="method-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Deprecation`

```rust
struct Deprecation {
    span: proc_macro2::Span,
    id: &'static str,
    version: &'static str,
    description: String,
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1285-1290`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1285-L1290)*

#### Implementations

- <span id="deprecation-attribute"></span>`fn attribute(version: &'static str, old: AttrKind, new: AttrKind, span: Span) -> Self` — [`AttrKind`](../attr/index.md#attrkind)

#### Trait Implementations

##### `impl Any for Deprecation`

- <span id="deprecation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Deprecation`

- <span id="deprecation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Deprecation`

- <span id="deprecation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Deprecation`

- <span id="deprecation-clone"></span>`fn clone(&self) -> Deprecation` — [`Deprecation`](#deprecation)

##### `impl CloneToUninit for Deprecation`

- <span id="deprecation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Deprecation`

- <span id="deprecation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Deprecation`

- <span id="deprecation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Deprecation`

- <span id="deprecation-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Deprecation`

- <span id="deprecation-toowned-type-owned"></span>`type Owned = T`

- <span id="deprecation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="deprecation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Deprecation`

- <span id="deprecation-totokens-to-tokens"></span>`fn to_tokens(&self, ts: &mut TokenStream)`

##### `impl<U> TryFrom for Deprecation`

- <span id="deprecation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deprecation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Deprecation`

- <span id="deprecation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deprecation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ValueParser`

```rust
enum ValueParser {
    Explicit(Method),
    Implicit(syn::Ident),
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1096-1099`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1096-L1099)*

#### Implementations

- <span id="valueparser-resolve"></span>`fn resolve(self, _inner_type: &Type) -> Method` — [`Method`](#method)

- <span id="valueparser-span"></span>`fn span(&self) -> Span`

#### Trait Implementations

##### `impl Any for ValueParser`

- <span id="valueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueParser`

- <span id="valueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueParser`

- <span id="valueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueParser`

- <span id="valueparser-clone"></span>`fn clone(&self) -> ValueParser` — [`ValueParser`](#valueparser)

##### `impl CloneToUninit for ValueParser`

- <span id="valueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for ValueParser`

- <span id="valueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ValueParser`

- <span id="valueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ValueParser`

- <span id="valueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="valueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueParser`

- <span id="valueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueParser`

- <span id="valueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Action`

```rust
enum Action {
    Explicit(Method),
    Implicit(syn::Ident),
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1128-1131`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1128-L1131)*

#### Implementations

- <span id="action-resolve"></span>`fn resolve(self, _field_type: &Type) -> Method` — [`Method`](#method)

- <span id="action-span"></span>`fn span(&self) -> Span`

#### Trait Implementations

##### `impl Any for Action`

- <span id="action-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Action`

- <span id="action-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Action`

- <span id="action-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Action`

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](#action)

##### `impl CloneToUninit for Action`

- <span id="action-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Action`

- <span id="action-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Action`

- <span id="action-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Action`

- <span id="action-toowned-type-owned"></span>`type Owned = T`

- <span id="action-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="action-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Action`

- <span id="action-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="action-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Action`

- <span id="action-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="action-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Kind`

```rust
enum Kind {
    Arg(self::spanned::Sp<self::ty::Ty>),
    Command(self::spanned::Sp<self::ty::Ty>),
    Value,
    FromGlobal(self::spanned::Sp<self::ty::Ty>),
    Subcommand(self::spanned::Sp<self::ty::Ty>),
    Flatten(self::spanned::Sp<self::ty::Ty>),
    Skip(Option<crate::attr::AttrValue>, crate::attr::AttrKind),
    ExternalSubcommand,
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1181-1190`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1181-L1190)*

#### Implementations

- <span id="kind-name"></span>`fn name(&self) -> &'static str`

- <span id="kind-attr-kind"></span>`fn attr_kind(&self) -> AttrKind` — [`AttrKind`](../attr/index.md#attrkind)

- <span id="kind-ty"></span>`fn ty(&self) -> Option<&Sp<Ty>>` — [`Sp`](../utils/spanned/index.md#sp), [`Ty`](../utils/ty/index.md#ty)

#### Trait Implementations

##### `impl Any for Kind`

- <span id="kind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Kind`

- <span id="kind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Kind`

- <span id="kind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Kind`

- <span id="kind-clone"></span>`fn clone(&self) -> Kind` — [`Kind`](#kind)

##### `impl CloneToUninit for Kind`

- <span id="kind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Kind`

- <span id="kind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Kind`

- <span id="kind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Kind`

- <span id="kind-toowned-type-owned"></span>`type Owned = T`

- <span id="kind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="kind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Kind`

- <span id="kind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Kind`

- <span id="kind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CasingStyle`

```rust
enum CasingStyle {
    Camel,
    Kebab,
    Pascal,
    ScreamingSnake,
    Snake,
    Lower,
    Upper,
    Verbatim,
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1378-1395`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1378-L1395)*

Defines the casing for the attributes long representation.

#### Variants

- **`Camel`**

  Indicate word boundaries with uppercase letter, excluding the first word.

- **`Kebab`**

  Keep all letters lowercase and indicate word boundaries with hyphens.

- **`Pascal`**

  Indicate word boundaries with uppercase letter, including the first word.

- **`ScreamingSnake`**

  Keep all letters uppercase and indicate word boundaries with underscores.

- **`Snake`**

  Keep all letters lowercase and indicate word boundaries with underscores.

- **`Lower`**

  Keep all letters lowercase and remove word boundaries.

- **`Upper`**

  Keep all letters uppercase and remove word boundaries.

- **`Verbatim`**

  Use the original attribute name defined in the code.

#### Implementations

- <span id="casingstyle-from-lit"></span>`fn from_lit(name: &LitStr) -> Result<Sp<Self>, syn::Error>` — [`Sp`](../utils/spanned/index.md#sp)

#### Trait Implementations

##### `impl Any for CasingStyle`

- <span id="casingstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CasingStyle`

- <span id="casingstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CasingStyle`

- <span id="casingstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CasingStyle`

- <span id="casingstyle-clone"></span>`fn clone(&self) -> CasingStyle` — [`CasingStyle`](#casingstyle)

##### `impl CloneToUninit for CasingStyle`

- <span id="casingstyle-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CasingStyle`

##### `impl Debug for CasingStyle`

- <span id="casingstyle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CasingStyle`

##### `impl<T> From for CasingStyle`

- <span id="casingstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CasingStyle`

- <span id="casingstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CasingStyle`

- <span id="casingstyle-partialeq-eq"></span>`fn eq(&self, other: &CasingStyle) -> bool` — [`CasingStyle`](#casingstyle)

##### `impl StructuralPartialEq for CasingStyle`

##### `impl ToOwned for CasingStyle`

- <span id="casingstyle-toowned-type-owned"></span>`type Owned = T`

- <span id="casingstyle-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="casingstyle-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CasingStyle`

- <span id="casingstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="casingstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CasingStyle`

- <span id="casingstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="casingstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Name`

```rust
enum Name {
    Derived(syn::Ident),
    Assigned(proc_macro2::TokenStream),
}
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1422-1425`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1422-L1425)*

#### Implementations

- <span id="name-translate"></span>`fn translate(self, style: CasingStyle) -> TokenStream` — [`CasingStyle`](#casingstyle)

- <span id="name-translate-char"></span>`fn translate_char(self, style: CasingStyle) -> TokenStream` — [`CasingStyle`](#casingstyle)

#### Trait Implementations

##### `impl Any for Name`

- <span id="name-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Name`

- <span id="name-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Name`

- <span id="name-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Name`

- <span id="name-clone"></span>`fn clone(&self) -> Name` — [`Name`](#name)

##### `impl CloneToUninit for Name`

- <span id="name-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Name`

- <span id="name-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Name`

- <span id="name-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Name`

- <span id="name-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Name`

- <span id="name-toowned-type-owned"></span>`type Owned = T`

- <span id="name-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="name-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Name`

- <span id="name-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Name`

- <span id="name-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="name-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Name`

- <span id="name-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="name-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `default_value_parser`

```rust
fn default_value_parser(inner_type: &syn::Type, span: proc_macro2::Span) -> Method
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1117-1125`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1117-L1125)*

### `default_action`

```rust
fn default_action(field_type: &syn::Type, span: proc_macro2::Span) -> Method
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1149-1177`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1149-L1177)*

### `assert_attr_kind`

```rust
fn assert_attr_kind(attr: &crate::attr::ClapAttr, possible_kind: &[crate::attr::AttrKind]) -> Result<(), syn::Error>
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1332-1349`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1332-L1349)*

### `process_author_str`

```rust
fn process_author_str(author: &str) -> String
```

*Defined in [`clap_derive-4.5.49/src/item.rs:1355-1374`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L1355-L1374)*

replace all `:` with `, ` when not inside the `<>`

`"author1:author2:author3" => "author1, author2, author3"`
`"author1 <http://website1.com>:author2" => "author1 <http://website1.com>, author2"`

## Constants

### `DEFAULT_CASING`
```rust
const DEFAULT_CASING: CasingStyle;
```

*Defined in [`clap_derive-4.5.49/src/item.rs:27`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L27)*

Default casing style for generated arguments.

### `DEFAULT_ENV_CASING`
```rust
const DEFAULT_ENV_CASING: CasingStyle;
```

*Defined in [`clap_derive-4.5.49/src/item.rs:30`](../../../.source_1765633015/clap_derive-4.5.49/src/item.rs#L30)*

Default casing style for environment variables

