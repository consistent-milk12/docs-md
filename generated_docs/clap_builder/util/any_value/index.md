*[clap_builder](../../index.md) / [util](../index.md) / [any_value](index.md)*

---

# Module `any_value`

## Structs

### `AnyValue`

```rust
struct AnyValue {
    inner: std::sync::Arc<dyn std::any::Any + Send + Sync>,
    id: AnyValueId,
}
```

#### Implementations

- `fn new<V: std::any::Any + Clone + Send + Sync + 'static>(inner: V) -> Self`

- `fn downcast_ref<T: std::any::Any + Clone + Send + Sync + 'static>(self: &Self) -> Option<&T>`

- `fn downcast_into<T: std::any::Any + Clone + Send + Sync>(self: Self) -> Result<T, Self>`

- `fn type_id(self: &Self) -> AnyValueId` — [`AnyValueId`](#anyvalueid)

#### Trait Implementations

##### `impl Clone for AnyValue`

- `fn clone(self: &Self) -> AnyValue` — [`AnyValue`](#anyvalue)

##### `impl Debug for AnyValue`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

### `AnyValueId`

```rust
struct AnyValueId {
    type_id: std::any::TypeId,
    type_name: &'static str,
}
```

#### Implementations

- `fn of<A: ?Sized + 'static>() -> Self`

#### Trait Implementations

##### `impl Clone for AnyValueId`

- `fn clone(self: &Self) -> AnyValueId` — [`AnyValueId`](#anyvalueid)

##### `impl Copy for AnyValueId`

##### `impl Debug for AnyValueId`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl Eq for AnyValueId`

##### `impl Hash for AnyValueId`

- `fn hash<H: std::hash::Hasher>(self: &Self, state: &mut H)`

##### `impl Ord for AnyValueId`

- `fn cmp(self: &Self, other: &Self) -> std::cmp::Ordering`

##### `impl PartialEq for AnyValueId`

- `fn eq(self: &Self, other: &std::any::TypeId) -> bool`

##### `impl PartialOrd for AnyValueId`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering>`

