*[miette_derive](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Display`](#display) | struct |  |
| [`explicit_named_args`](#explicit-named-args) | fn |  |
| [`take_int`](#take-int) | fn |  |
| [`take_ident`](#take-ident) | fn |  |
| [`parse_token_expr`](#parse-token-expr) | fn |  |

## Structs

### `Display`

```rust
struct Display {
    pub fmt: syn::LitStr,
    pub args: proc_macro2::TokenStream,
    pub has_bonus_display: bool,
}
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:12-16`](../../../.source_1765633015/miette-derive-7.6.0/src/fmt.rs#L12-L16)*

#### Implementations

- <span id="display-expand-shorthand"></span>`fn expand_shorthand(&mut self, members: &Set<Member>)`

#### Trait Implementations

##### `impl Any for Display`

- <span id="display-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Display`

- <span id="display-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Display`

- <span id="display-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Display`

- <span id="display-clone"></span>`fn clone(&self) -> Display` — [`Display`](#display)

##### `impl CloneToUninit for Display`

- <span id="display-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Display`

- <span id="display-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Display`

- <span id="display-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Display`

- <span id="display-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Display`

- <span id="display-toowned-type-owned"></span>`type Owned = T`

- <span id="display-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="display-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Display`

- <span id="display-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Display`

- <span id="display-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="display-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Display`

- <span id="display-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="display-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `explicit_named_args`

```rust
fn explicit_named_args(input: syn::parse::ParseStream<'_>) -> syn::Result<std::collections::HashSet<syn::Ident>>
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:116-131`](../../../.source_1765633015/miette-derive-7.6.0/src/fmt.rs#L116-L131)*

### `take_int`

```rust
fn take_int(read: &mut &str) -> String
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:133-145`](../../../.source_1765633015/miette-derive-7.6.0/src/fmt.rs#L133-L145)*

### `take_ident`

```rust
fn take_ident(read: &mut &str) -> syn::Ident
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:147-164`](../../../.source_1765633015/miette-derive-7.6.0/src/fmt.rs#L147-L164)*

### `parse_token_expr`

```rust
fn parse_token_expr(input: syn::parse::ParseStream<'_>, begin_expr: bool) -> syn::Result<proc_macro2::TokenStream>
```

*Defined in [`miette-derive-7.6.0/src/fmt.rs:166-235`](../../../.source_1765633015/miette-derive-7.6.0/src/fmt.rs#L166-L235)*

