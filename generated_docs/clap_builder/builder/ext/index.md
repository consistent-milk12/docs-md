*[clap_builder](../../index.md) / [builder](../index.md) / [ext](index.md)*

---

# Module `ext`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Extensions`](#extensions) | struct |  |
| [`Extension`](#extension) | trait |  |

## Structs

### `Extensions`

```rust
struct Extensions {
    extensions: self::flat_map::FlatMap<self::any_value::AnyValueId, self::any_value::AnyValue>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/ext.rs:6-8`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/ext.rs#L6-L8)*

#### Implementations

- <span id="extensions-get"></span>`fn get<T: Extension>(&self) -> Option<&T>`

- <span id="extensions-set"></span>`fn set<T: Extension>(&mut self, tagged: T) -> bool`

- <span id="extensions-remove"></span>`fn remove<T: Extension>(&mut self) -> Option<T>`

- <span id="extensions-update"></span>`fn update(&mut self, other: &Self)`

#### Trait Implementations

##### `impl Any for Extensions`

- <span id="extensions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Extensions`

- <span id="extensions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Extensions`

- <span id="extensions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Extensions`

- <span id="extensions-clone"></span>`fn clone(&self) -> Extensions` — [`Extensions`](#extensions)

##### `impl CloneToUninit for Extensions`

- <span id="extensions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Extensions`

- <span id="extensions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Extensions`

- <span id="extensions-default"></span>`fn default() -> Extensions` — [`Extensions`](#extensions)

##### `impl<T> From for Extensions`

- <span id="extensions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Extensions`

- <span id="extensions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Extensions`

- <span id="extensions-toowned-type-owned"></span>`type Owned = T`

- <span id="extensions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="extensions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Extensions`

- <span id="extensions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extensions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Extensions`

- <span id="extensions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extensions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Extension`

```rust
trait Extension: std::fmt::Debug + Clone + std::any::Any + Send + Sync + 'static { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/ext.rs:44`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/ext.rs#L44)*

#### Implementors

- `T`

