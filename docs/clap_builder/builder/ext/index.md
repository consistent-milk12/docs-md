*[clap_builder](../../index.md) / [builder](../index.md) / [ext](index.md)*

---

# Module `ext`

## Structs

### `Extensions`

```rust
struct Extensions {
    extensions: self::flat_map::FlatMap<self::any_value::AnyValueId, self::any_value::AnyValue>,
}
```

#### Implementations

- `fn get<T: Extension>(self: &Self) -> Option<&T>`

- `fn set<T: Extension>(self: &mut Self, tagged: T) -> bool`

- `fn remove<T: Extension>(self: &mut Self) -> Option<T>`

- `fn update(self: &mut Self, other: &Self)`

#### Trait Implementations

##### `impl Clone for Extensions`

- `fn clone(self: &Self) -> Extensions` — [`Extensions`](#extensions)

##### `impl Debug for Extensions`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Extensions`

- `fn default() -> Extensions` — [`Extensions`](#extensions)

## Traits

### `Extension`

```rust
trait Extension: std::fmt::Debug + Clone + std::any::Any + Send + Sync + 'static { ... }
```

