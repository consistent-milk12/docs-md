*[clap_builder](../../index.md) / [util](../index.md) / [any_value](index.md)*

---

# Module `any_value`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AnyValue`](#anyvalue) | struct |  |
| [`AnyValueId`](#anyvalueid) | struct |  |

## Structs

### `AnyValue`

```rust
struct AnyValue {
    inner: std::sync::Arc<dyn std::any::Any + Send + Sync>,
    id: AnyValueId,
}
```

*Defined in [`clap_builder-4.5.53/src/util/any_value.rs:2-7`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/any_value.rs#L2-L7)*

#### Implementations

- <span id="anyvalue-new"></span>`fn new<V: std::any::Any + Clone + Send + Sync + 'static>(inner: V) -> Self`

- <span id="anyvalue-downcast-ref"></span>`fn downcast_ref<T: std::any::Any + Clone + Send + Sync + 'static>(&self) -> Option<&T>`

- <span id="anyvalue-downcast-into"></span>`fn downcast_into<T: std::any::Any + Clone + Send + Sync>(self) -> Result<T, Self>`

- <span id="anyvalue-type-id"></span>`fn type_id(&self) -> AnyValueId` — [`AnyValueId`](#anyvalueid)

#### Trait Implementations

##### `impl Any for AnyValue`

- <span id="anyvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnyValue`

- <span id="anyvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnyValue`

- <span id="anyvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AnyValue`

- <span id="anyvalue-clone"></span>`fn clone(&self) -> AnyValue` — [`AnyValue`](#anyvalue)

##### `impl CloneToUninit for AnyValue`

- <span id="anyvalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AnyValue`

- <span id="anyvalue-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl<T> From for AnyValue`

- <span id="anyvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AnyValue`

- <span id="anyvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AnyValue`

- <span id="anyvalue-toowned-type-owned"></span>`type Owned = T`

- <span id="anyvalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="anyvalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AnyValue`

- <span id="anyvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anyvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnyValue`

- <span id="anyvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anyvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AnyValueId`

```rust
struct AnyValueId {
    type_id: std::any::TypeId,
    type_name: &'static str,
}
```

*Defined in [`clap_builder-4.5.53/src/util/any_value.rs:42-46`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/any_value.rs#L42-L46)*

#### Implementations

- <span id="anyvalueid-of"></span>`fn of<A: ?Sized + 'static>() -> Self`

#### Trait Implementations

##### `impl Any for AnyValueId`

- <span id="anyvalueid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnyValueId`

- <span id="anyvalueid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnyValueId`

- <span id="anyvalueid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AnyValueId`

- <span id="anyvalueid-clone"></span>`fn clone(&self) -> AnyValueId` — [`AnyValueId`](#anyvalueid)

##### `impl CloneToUninit for AnyValueId`

- <span id="anyvalueid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AnyValueId`

##### `impl Debug for AnyValueId`

- <span id="anyvalueid-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl Eq for AnyValueId`

##### `impl<T> From for AnyValueId`

- <span id="anyvalueid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AnyValueId`

- <span id="anyvalueid-hash"></span>`fn hash<H: std::hash::Hasher>(&self, state: &mut H)`

##### `impl<U> Into for AnyValueId`

- <span id="anyvalueid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for AnyValueId`

- <span id="anyvalueid-ord-cmp"></span>`fn cmp(&self, other: &Self) -> std::cmp::Ordering`

##### `impl PartialEq for AnyValueId`

- <span id="anyvalueid-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for AnyValueId`

- <span id="anyvalueid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>`

##### `impl ToOwned for AnyValueId`

- <span id="anyvalueid-toowned-type-owned"></span>`type Owned = T`

- <span id="anyvalueid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="anyvalueid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AnyValueId`

- <span id="anyvalueid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anyvalueid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnyValueId`

- <span id="anyvalueid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anyvalueid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

