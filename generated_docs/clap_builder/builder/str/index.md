*[clap_builder](../../index.md) / [builder](../index.md) / [str](index.md)*

---

# Module `str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inner`](#inner) | mod |  |
| [`Str`](#str) | struct | A UTF-8-encoded fixed string |

## Modules

- [`inner`](inner/index.md)

## Structs

### `Str`

```rust
struct Str {
    name: inner::Inner,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/str.rs:13-15`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/str.rs#L13-L15)*

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `String`), enable the `string`
feature

</div>

#### Implementations

- <span id="str-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="str-into-inner"></span>`fn into_inner(self) -> Inner` — [`Inner`](inner/index.md#inner)

- <span id="str-as-str"></span>`fn as_str(&self) -> &str`

  Get the raw string of the `Str`

#### Trait Implementations

##### `impl Any for Str`

- <span id="str-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Str`

- <span id="str-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for Str`

- <span id="str-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Str`

- <span id="str-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Str`

- <span id="str-clone"></span>`fn clone(&self) -> Str` — [`Str`](#str)

##### `impl CloneToUninit for Str`

- <span id="str-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Str`

- <span id="str-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Str`

- <span id="str-default"></span>`fn default() -> Str` — [`Str`](#str)

##### `impl Deref for Str`

- <span id="str-deref-type-target"></span>`type Target = str`

- <span id="str-deref"></span>`fn deref(&self) -> &str`

##### `impl Display for Str`

- <span id="str-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Str`

##### `impl<T> From for Str`

- <span id="str-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Str`

- <span id="str-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Str`

- <span id="str-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Str`

- <span id="str-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<String>` — [`Resettable`](../resettable/index.md#resettable)

##### `impl Ord for Str`

- <span id="str-ord-cmp"></span>`fn cmp(&self, other: &Str) -> cmp::Ordering` — [`Str`](#str)

##### `impl PartialEq for Str`

- <span id="str-partialeq-eq"></span>`fn eq(&self, other: &Str) -> bool` — [`Str`](#str)

##### `impl PartialOrd for Str`

- <span id="str-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Str) -> option::Option<cmp::Ordering>` — [`Str`](#str)

##### `impl Receiver for Str`

- <span id="str-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Str`

##### `impl ToOwned for Str`

- <span id="str-toowned-type-owned"></span>`type Owned = T`

- <span id="str-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="str-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Str`

- <span id="str-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Str`

- <span id="str-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="str-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Str`

- <span id="str-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="str-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

