*[thiserror_impl](../index.md) / [unraw](index.md)*

---

# Module `unraw`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IdentUnraw`](#identunraw) | struct |  |
| [`MemberUnraw`](#memberunraw) | enum |  |

## Structs

### `IdentUnraw`

```rust
struct IdentUnraw(proc_macro2::Ident);
```

*Defined in [`thiserror-impl-2.0.17/src/unraw.rs:12`](../../../.source_1765633015/thiserror-impl-2.0.17/src/unraw.rs#L12)*

#### Implementations

- <span id="identunraw-new"></span>`fn new(ident: Ident) -> Self`

- <span id="identunraw-to-local"></span>`fn to_local(&self) -> Ident`

- <span id="identunraw-set-span"></span>`fn set_span(&mut self, span: Span)`

#### Trait Implementations

##### `impl Any for IdentUnraw`

- <span id="identunraw-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IdentUnraw`

- <span id="identunraw-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IdentUnraw`

- <span id="identunraw-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for IdentUnraw`

- <span id="identunraw-clone"></span>`fn clone(&self) -> IdentUnraw` — [`IdentUnraw`](#identunraw)

##### `impl CloneToUninit for IdentUnraw`

- <span id="identunraw-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Display for IdentUnraw`

- <span id="identunraw-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IdentUnraw`

##### `impl<T> From for IdentUnraw`

- <span id="identunraw-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IdentUnraw`

- <span id="identunraw-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for IdentUnraw`

- <span id="identunraw-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl Parse for IdentUnraw`

- <span id="identunraw-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>`

##### `impl PartialEq for IdentUnraw`

- <span id="identunraw-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for IdentUnraw`

- <span id="identunraw-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Spanned for IdentUnraw`

- <span id="identunraw-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for IdentUnraw`

- <span id="identunraw-toowned-type-owned"></span>`type Owned = T`

- <span id="identunraw-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="identunraw-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for IdentUnraw`

- <span id="identunraw-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for IdentUnraw`

- <span id="identunraw-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for IdentUnraw`

- <span id="identunraw-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="identunraw-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IdentUnraw`

- <span id="identunraw-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="identunraw-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MemberUnraw`

```rust
enum MemberUnraw {
    Named(IdentUnraw),
    Unnamed(syn::Index),
}
```

*Defined in [`thiserror-impl-2.0.17/src/unraw.rs:82-85`](../../../.source_1765633015/thiserror-impl-2.0.17/src/unraw.rs#L82-L85)*

#### Implementations

- <span id="memberunraw-span"></span>`fn span(&self) -> Span`

#### Trait Implementations

##### `impl Any for MemberUnraw`

- <span id="memberunraw-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MemberUnraw`

- <span id="memberunraw-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MemberUnraw`

- <span id="memberunraw-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MemberUnraw`

- <span id="memberunraw-clone"></span>`fn clone(&self) -> MemberUnraw` — [`MemberUnraw`](#memberunraw)

##### `impl CloneToUninit for MemberUnraw`

- <span id="memberunraw-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Display for MemberUnraw`

- <span id="memberunraw-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemberUnraw`

##### `impl<T> From for MemberUnraw`

- <span id="memberunraw-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for MemberUnraw`

- <span id="memberunraw-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl<U> Into for MemberUnraw`

- <span id="memberunraw-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MemberUnraw`

- <span id="memberunraw-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Spanned for MemberUnraw`

- <span id="memberunraw-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for MemberUnraw`

- <span id="memberunraw-toowned-type-owned"></span>`type Owned = T`

- <span id="memberunraw-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memberunraw-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for MemberUnraw`

- <span id="memberunraw-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl ToTokens for MemberUnraw`

- <span id="memberunraw-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for MemberUnraw`

- <span id="memberunraw-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memberunraw-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MemberUnraw`

- <span id="memberunraw-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memberunraw-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

