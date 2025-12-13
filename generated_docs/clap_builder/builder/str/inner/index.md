*[clap_builder](../../../index.md) / [builder](../../index.md) / [str](../index.md) / [inner](index.md)*

---

# Module `inner`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Inner`](#inner) | struct |  |

## Structs

### `Inner`

```rust
struct Inner(&'static str);
```

*Defined in [`clap_builder-4.5.53/src/builder/str.rs:277`](../../../../../.source_1765633015/clap_builder-4.5.53/src/builder/str.rs#L277)*

#### Implementations

- <span id="inner-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="inner-as-str"></span>`fn as_str(&self) -> &str`

- <span id="inner-into-string"></span>`fn into_string(self) -> String`

#### Trait Implementations

##### `impl Any for Inner`

- <span id="inner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inner`

- <span id="inner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inner`

- <span id="inner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Inner`

- <span id="inner-clone"></span>`fn clone(&self) -> Inner` — [`Inner`](#inner)

##### `impl CloneToUninit for Inner`

- <span id="inner-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for inner::Inner`

- <span id="innerinner-default"></span>`fn default() -> Self`

##### `impl Eq for inner::Inner`

##### `impl<T> From for Inner`

- <span id="inner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for inner::Inner`

- <span id="innerinner-hash"></span>`fn hash<H: std::hash::Hasher>(&self, state: &mut H)`

##### `impl<U> Into for Inner`

- <span id="inner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for inner::Inner`

- <span id="innerinner-ord-cmp"></span>`fn cmp(&self, other: &Inner) -> std::cmp::Ordering` — [`Inner`](#inner)

##### `impl PartialEq for inner::Inner`

- <span id="innerinner-partialeq-eq"></span>`fn eq(&self, other: &Inner) -> bool` — [`Inner`](#inner)

##### `impl PartialOrd for inner::Inner`

- <span id="innerinner-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>`

##### `impl ToOwned for Inner`

- <span id="inner-toowned-type-owned"></span>`type Owned = T`

- <span id="inner-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="inner-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Inner`

- <span id="inner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Inner`

- <span id="inner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

