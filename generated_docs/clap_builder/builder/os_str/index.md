*[clap_builder](../../index.md) / [builder](../index.md) / [os_str](index.md)*

---

# Module `os_str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inner`](#inner) | mod |  |
| [`OsStr`](#osstr) | struct | A UTF-8-encoded fixed string |

## Modules

- [`inner`](inner/index.md)

## Structs

### `OsStr`

```rust
struct OsStr {
    name: inner::Inner,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/os_str.rs:14-16`](../../../../.source_1765633015/clap_builder-4.5.53/src/builder/os_str.rs#L14-L16)*

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`
feature

</div>

#### Implementations

- <span id="osstr-from-static-ref"></span>`fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- <span id="osstr-as-os-str"></span>`fn as_os_str(&self) -> &std::ffi::OsStr`

  Get the raw string as an `std::ffi::OsStr`

- <span id="osstr-to-os-string"></span>`fn to_os_string(&self) -> std::ffi::OsString`

  Get the raw string as an `OsString`

#### Trait Implementations

##### `impl Any for OsStr`

- <span id="osstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for OsStr`

- <span id="osstr-asref-as-ref"></span>`fn as_ref(&self) -> &std::ffi::OsStr`

##### `impl<T> Borrow for OsStr`

- <span id="osstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OsStr`

- <span id="osstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OsStr`

- <span id="osstr-clone"></span>`fn clone(&self) -> OsStr` — [`OsStr`](#osstr)

##### `impl CloneToUninit for OsStr`

- <span id="osstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for OsStr`

- <span id="osstr-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for OsStr`

- <span id="osstr-default"></span>`fn default() -> OsStr` — [`OsStr`](#osstr)

##### `impl Deref for OsStr`

- <span id="osstr-deref-type-target"></span>`type Target = OsStr`

- <span id="osstr-deref"></span>`fn deref(&self) -> &std::ffi::OsStr`

##### `impl Eq for OsStr`

##### `impl<T> From for OsStr`

- <span id="osstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for OsStr`

- <span id="osstr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for OsStr`

- <span id="osstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for OsStr`

- <span id="osstr-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<OsStr>` — [`Resettable`](../resettable/index.md#resettable), [`OsStr`](#osstr)

##### `impl Ord for OsStr`

- <span id="osstr-ord-cmp"></span>`fn cmp(&self, other: &OsStr) -> cmp::Ordering` — [`OsStr`](#osstr)

##### `impl PartialEq for OsStr`

- <span id="osstr-partialeq-eq"></span>`fn eq(&self, other: &OsStr) -> bool` — [`OsStr`](#osstr)

##### `impl PartialOrd for OsStr`

- <span id="osstr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &OsStr) -> option::Option<cmp::Ordering>` — [`OsStr`](#osstr)

##### `impl Receiver for OsStr`

- <span id="osstr-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for OsStr`

##### `impl ToOwned for OsStr`

- <span id="osstr-toowned-type-owned"></span>`type Owned = T`

- <span id="osstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="osstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OsStr`

- <span id="osstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="osstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OsStr`

- <span id="osstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="osstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

