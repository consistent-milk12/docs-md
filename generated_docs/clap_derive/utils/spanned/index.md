*[clap_derive](../../index.md) / [utils](../index.md) / [spanned](index.md)*

---

# Module `spanned`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sp`](#sp) | struct | An entity with a span attached. |

## Structs

### `Sp<T>`

```rust
struct Sp<T> {
    val: T,
    span: proc_macro2::Span,
}
```

*Defined in [`clap_derive-4.5.49/src/utils/spanned.rs:9-12`](../../../../.source_1765521767/clap_derive-4.5.49/src/utils/spanned.rs#L9-L12)*

An entity with a span attached.

#### Implementations

- <span id="sp-new"></span>`fn new(val: T, span: Span) -> Self`

- <span id="sp-get"></span>`fn get(&self) -> &T`

- <span id="sp-span"></span>`fn span(&self) -> Span`

#### Trait Implementations

##### `impl<T> Any for Sp<T>`

- <span id="sp-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: AsRef<str>> AsRef for Sp<T>`

- <span id="sp-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for Sp<T>`

- <span id="sp-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Sp<T>`

- <span id="sp-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Sp<T>`

- <span id="sp-clone"></span>`fn clone(&self) -> Sp<T>` — [`Sp`](#sp)

##### `impl<T> CloneToUninit for Sp<T>`

- <span id="sp-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for Sp<T>`

##### `impl<T: fmt::Debug> Debug for Sp<T>`

- <span id="sp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Deref for Sp<T>`

- <span id="sp-deref-type-target"></span>`type Target = T`

- <span id="sp-deref"></span>`fn deref(&self) -> &T`

##### `impl<T> DerefMut for Sp<T>`

- <span id="sp-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T> From for Sp<T>`

- <span id="sp-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Sp<T>`

- <span id="sp-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U, T: PartialEq<U>> PartialEq for Sp<T>`

- <span id="sp-partialeq-eq"></span>`fn eq(&self, other: &U) -> bool`

##### `impl<T> Receiver for Sp<T>`

- <span id="sp-receiver-type-target"></span>`type Target = T`

##### `impl<T> Spanned for Sp<T>`

- <span id="sp-spanned-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToOwned for Sp<T>`

- <span id="sp-toowned-type-owned"></span>`type Owned = T`

- <span id="sp-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sp-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T: ToTokens> ToTokens for Sp<T>`

- <span id="sp-totokens-to-tokens"></span>`fn to_tokens(&self, stream: &mut TokenStream)`

##### `impl<T, U> TryFrom for Sp<T>`

- <span id="sp-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sp-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Sp<T>`

- <span id="sp-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sp-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

