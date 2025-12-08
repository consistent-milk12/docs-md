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

#### Implementations

- <span id="anyvalue-new"></span>`fn new<V: std::any::Any + Clone + Send + Sync + 'static>(inner: V) -> Self`

- <span id="anyvalue-downcast-ref"></span>`fn downcast_ref<T: std::any::Any + Clone + Send + Sync + 'static>(&self) -> Option<&T>`

- <span id="anyvalue-downcast-into"></span>`fn downcast_into<T: std::any::Any + Clone + Send + Sync>(self) -> Result<T, Self>`

- <span id="anyvalue-type-id"></span>`fn type_id(&self) -> AnyValueId` — [`AnyValueId`](#anyvalueid)

#### Trait Implementations

##### `impl Clone for AnyValue`

- <span id="anyvalue-clone"></span>`fn clone(&self) -> AnyValue` — [`AnyValue`](#anyvalue)

##### `impl Debug for AnyValue`

- <span id="anyvalue-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

### `AnyValueId`

```rust
struct AnyValueId {
    type_id: std::any::TypeId,
    type_name: &'static str,
}
```

#### Implementations

- <span id="anyvalueid-of"></span>`fn of<A: ?Sized + 'static>() -> Self`

#### Trait Implementations

##### `impl Clone for AnyValueId`

- <span id="anyvalueid-clone"></span>`fn clone(&self) -> AnyValueId` — [`AnyValueId`](#anyvalueid)

##### `impl Copy for AnyValueId`

##### `impl Debug for AnyValueId`

- <span id="anyvalueid-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl Eq for AnyValueId`

##### `impl Hash for AnyValueId`

- <span id="anyvalueid-hash"></span>`fn hash<H: std::hash::Hasher>(&self, state: &mut H)`

##### `impl Ord for AnyValueId`

- <span id="anyvalueid-cmp"></span>`fn cmp(&self, other: &Self) -> std::cmp::Ordering`

##### `impl PartialEq for AnyValueId`

- <span id="anyvalueid-eq"></span>`fn eq(&self, other: &std::any::TypeId) -> bool`

##### `impl PartialOrd for AnyValueId`

- <span id="anyvalueid-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>`

