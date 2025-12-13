*[tracing_attributes](../../index.md) / [attr](../index.md) / [kw](index.md)*

---

# Module `kw`

## Contents

- [Structs](#structs)
  - [`fields`](#fields)
  - [`skip`](#skip)
  - [`skip_all`](#skip-all)
  - [`level`](#level)
  - [`target`](#target)
  - [`parent`](#parent)
  - [`follows_from`](#follows-from)
  - [`name`](#name)
  - [`err`](#err)
  - [`ret`](#ret)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fields`](#fields) | struct |  |
| [`skip`](#skip) | struct |  |
| [`skip_all`](#skip-all) | struct |  |
| [`level`](#level) | struct |  |
| [`target`](#target) | struct |  |
| [`parent`](#parent) | struct |  |
| [`follows_from`](#follows-from) | struct |  |
| [`name`](#name) | struct |  |
| [`err`](#err) | struct |  |
| [`ret`](#ret) | struct |  |

## Structs

### `fields`

```rust
struct fields {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:497`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L497)*

#### Trait Implementations

##### `impl Any for fields`

- <span id="fields-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for fields`

- <span id="fields-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for fields`

- <span id="fields-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for fields`

- <span id="fields-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for fields`

- <span id="fields-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for fields`

##### `impl Debug for fields`

- <span id="fields-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for fields`

- <span id="fields-default"></span>`fn default() -> Self`

##### `impl Eq for fields`

##### `impl<T> From for fields`

- <span id="fields-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for fields`

- <span id="fields-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for fields`

- <span id="fields-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for fields`

- <span id="fields-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<fields>` — [`fields`](#fields)

##### `impl PartialEq for fields`

- <span id="fields-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for fields`

- <span id="fields-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for fields`

- <span id="fields-toowned-type-owned"></span>`type Owned = T`

- <span id="fields-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fields-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for fields`

- <span id="fields-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for fields`

- <span id="fields-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="fields-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for fields`

- <span id="fields-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fields-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for fields`

- <span id="fields-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fields-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `skip`

```rust
struct skip {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:498`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L498)*

#### Trait Implementations

##### `impl Any for skip`

- <span id="skip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for skip`

- <span id="skip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for skip`

- <span id="skip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for skip`

- <span id="skip-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for skip`

- <span id="skip-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for skip`

##### `impl Debug for skip`

- <span id="skip-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for skip`

- <span id="skip-default"></span>`fn default() -> Self`

##### `impl Eq for skip`

##### `impl<T> From for skip`

- <span id="skip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for skip`

- <span id="skip-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for skip`

- <span id="skip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for skip`

- <span id="skip-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<skip>` — [`skip`](#skip)

##### `impl PartialEq for skip`

- <span id="skip-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for skip`

- <span id="skip-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for skip`

- <span id="skip-toowned-type-owned"></span>`type Owned = T`

- <span id="skip-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skip-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for skip`

- <span id="skip-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for skip`

- <span id="skip-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="skip-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for skip`

- <span id="skip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for skip`

- <span id="skip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `skip_all`

```rust
struct skip_all {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:499`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L499)*

#### Trait Implementations

##### `impl Any for skip_all`

- <span id="skip-all-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for skip_all`

- <span id="skip-all-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for skip_all`

- <span id="skip-all-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for skip_all`

- <span id="skip-all-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for skip_all`

- <span id="skip-all-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for skip_all`

##### `impl Debug for skip_all`

- <span id="skip-all-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for skip_all`

- <span id="skip-all-default"></span>`fn default() -> Self`

##### `impl Eq for skip_all`

##### `impl<T> From for skip_all`

- <span id="skip-all-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for skip_all`

- <span id="skip-all-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for skip_all`

- <span id="skip-all-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for skip_all`

- <span id="skip-all-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<skip_all>` — [`skip_all`](#skip-all)

##### `impl PartialEq for skip_all`

- <span id="skip-all-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for skip_all`

- <span id="skip-all-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for skip_all`

- <span id="skip-all-toowned-type-owned"></span>`type Owned = T`

- <span id="skip-all-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skip-all-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for skip_all`

- <span id="skip-all-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for skip_all`

- <span id="skip-all-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="skip-all-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for skip_all`

- <span id="skip-all-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skip-all-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for skip_all`

- <span id="skip-all-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skip-all-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `level`

```rust
struct level {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:500`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L500)*

#### Trait Implementations

##### `impl Any for level`

- <span id="level-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for level`

- <span id="level-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for level`

- <span id="level-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for level`

- <span id="level-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for level`

- <span id="level-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for level`

##### `impl Debug for level`

- <span id="level-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for level`

- <span id="level-default"></span>`fn default() -> Self`

##### `impl Eq for level`

##### `impl<T> From for level`

- <span id="level-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for level`

- <span id="level-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for level`

- <span id="level-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for level`

- <span id="level-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<level>` — [`level`](#level)

##### `impl PartialEq for level`

- <span id="level-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for level`

- <span id="level-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for level`

- <span id="level-toowned-type-owned"></span>`type Owned = T`

- <span id="level-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="level-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for level`

- <span id="level-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for level`

- <span id="level-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="level-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for level`

- <span id="level-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="level-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for level`

- <span id="level-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="level-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `target`

```rust
struct target {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:501`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L501)*

#### Trait Implementations

##### `impl Any for target`

- <span id="target-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for target`

- <span id="target-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for target`

- <span id="target-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for target`

- <span id="target-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for target`

- <span id="target-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for target`

##### `impl Debug for target`

- <span id="target-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for target`

- <span id="target-default"></span>`fn default() -> Self`

##### `impl Eq for target`

##### `impl<T> From for target`

- <span id="target-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for target`

- <span id="target-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for target`

- <span id="target-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for target`

- <span id="target-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<target>` — [`target`](#target)

##### `impl PartialEq for target`

- <span id="target-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for target`

- <span id="target-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for target`

- <span id="target-toowned-type-owned"></span>`type Owned = T`

- <span id="target-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="target-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for target`

- <span id="target-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for target`

- <span id="target-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="target-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for target`

- <span id="target-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="target-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for target`

- <span id="target-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="target-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `parent`

```rust
struct parent {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:502`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L502)*

#### Trait Implementations

##### `impl Any for parent`

- <span id="parent-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for parent`

- <span id="parent-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for parent`

- <span id="parent-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for parent`

- <span id="parent-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for parent`

- <span id="parent-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for parent`

##### `impl Debug for parent`

- <span id="parent-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for parent`

- <span id="parent-default"></span>`fn default() -> Self`

##### `impl Eq for parent`

##### `impl<T> From for parent`

- <span id="parent-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for parent`

- <span id="parent-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for parent`

- <span id="parent-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for parent`

- <span id="parent-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<parent>` — [`parent`](#parent)

##### `impl PartialEq for parent`

- <span id="parent-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for parent`

- <span id="parent-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for parent`

- <span id="parent-toowned-type-owned"></span>`type Owned = T`

- <span id="parent-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parent-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for parent`

- <span id="parent-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for parent`

- <span id="parent-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="parent-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for parent`

- <span id="parent-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parent-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for parent`

- <span id="parent-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parent-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `follows_from`

```rust
struct follows_from {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:503`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L503)*

#### Trait Implementations

##### `impl Any for follows_from`

- <span id="follows-from-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for follows_from`

- <span id="follows-from-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for follows_from`

- <span id="follows-from-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for follows_from`

- <span id="follows-from-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for follows_from`

- <span id="follows-from-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for follows_from`

##### `impl Debug for follows_from`

- <span id="follows-from-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for follows_from`

- <span id="follows-from-default"></span>`fn default() -> Self`

##### `impl Eq for follows_from`

##### `impl<T> From for follows_from`

- <span id="follows-from-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for follows_from`

- <span id="follows-from-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for follows_from`

- <span id="follows-from-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for follows_from`

- <span id="follows-from-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<follows_from>` — [`follows_from`](#follows-from)

##### `impl PartialEq for follows_from`

- <span id="follows-from-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for follows_from`

- <span id="follows-from-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for follows_from`

- <span id="follows-from-toowned-type-owned"></span>`type Owned = T`

- <span id="follows-from-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="follows-from-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for follows_from`

- <span id="follows-from-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for follows_from`

- <span id="follows-from-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="follows-from-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for follows_from`

- <span id="follows-from-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="follows-from-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for follows_from`

- <span id="follows-from-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="follows-from-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `name`

```rust
struct name {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:504`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L504)*

#### Trait Implementations

##### `impl Any for name`

- <span id="name-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for name`

- <span id="name-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for name`

- <span id="name-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for name`

- <span id="name-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for name`

- <span id="name-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for name`

##### `impl Debug for name`

- <span id="name-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for name`

- <span id="name-default"></span>`fn default() -> Self`

##### `impl Eq for name`

##### `impl<T> From for name`

- <span id="name-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for name`

- <span id="name-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for name`

- <span id="name-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for name`

- <span id="name-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<name>` — [`name`](#name)

##### `impl PartialEq for name`

- <span id="name-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for name`

- <span id="name-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for name`

- <span id="name-toowned-type-owned"></span>`type Owned = T`

- <span id="name-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="name-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for name`

- <span id="name-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for name`

- <span id="name-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="name-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for name`

- <span id="name-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="name-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for name`

- <span id="name-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="name-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `err`

```rust
struct err {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:505`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L505)*

#### Trait Implementations

##### `impl Any for err`

- <span id="err-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for err`

- <span id="err-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for err`

- <span id="err-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for err`

- <span id="err-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for err`

- <span id="err-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for err`

##### `impl Debug for err`

- <span id="err-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for err`

- <span id="err-default"></span>`fn default() -> Self`

##### `impl Eq for err`

##### `impl<T> From for err`

- <span id="err-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for err`

- <span id="err-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for err`

- <span id="err-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for err`

- <span id="err-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<err>` — [`err`](#err)

##### `impl PartialEq for err`

- <span id="err-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for err`

- <span id="err-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for err`

- <span id="err-toowned-type-owned"></span>`type Owned = T`

- <span id="err-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="err-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for err`

- <span id="err-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for err`

- <span id="err-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="err-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for err`

- <span id="err-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="err-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for err`

- <span id="err-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="err-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ret`

```rust
struct ret {
    pub span: __private::Span,
}
```

*Defined in [`tracing-attributes-0.1.31/src/attr.rs:506`](../../../../.source_1765633015/tracing-attributes-0.1.31/src/attr.rs#L506)*

#### Trait Implementations

##### `impl Any for ret`

- <span id="ret-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ret`

- <span id="ret-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ret`

- <span id="ret-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ret`

- <span id="ret-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ret`

- <span id="ret-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ret`

##### `impl Debug for ret`

- <span id="ret-debug-fmt"></span>`fn fmt(&self, f: &mut __private::Formatter<'_>) -> __private::FmtResult`

##### `impl Default for ret`

- <span id="ret-default"></span>`fn default() -> Self`

##### `impl Eq for ret`

##### `impl<T> From for ret`

- <span id="ret-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ret`

- <span id="ret-hash"></span>`fn hash<__H: __private::Hasher>(&self, _state: &mut __H)`

##### `impl<U> Into for ret`

- <span id="ret-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for ret`

- <span id="ret-parse"></span>`fn parse(input: parse::ParseStream<'_>) -> parse::Result<ret>` — [`ret`](#ret)

##### `impl PartialEq for ret`

- <span id="ret-partialeq-eq"></span>`fn eq(&self, _other: &Self) -> __private::bool`

##### `impl Spanned for ret`

- <span id="ret-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ret`

- <span id="ret-toowned-type-owned"></span>`type Owned = T`

- <span id="ret-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ret-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for ret`

- <span id="ret-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut __private::TokenStream2)`

##### `impl Token for ret`

- <span id="ret-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool`

- <span id="ret-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for ret`

- <span id="ret-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ret-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ret`

- <span id="ret-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ret-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

