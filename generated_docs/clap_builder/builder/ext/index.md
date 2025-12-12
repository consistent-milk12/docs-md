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

##### `impl Clone for Extensions`

- <span id="extensions-clone"></span>`fn clone(&self) -> Extensions` — [`Extensions`](#extensions)

##### `impl Debug for Extensions`

- <span id="extensions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Extensions`

- <span id="extensions-default"></span>`fn default() -> Extensions` — [`Extensions`](#extensions)

## Traits

### `Extension`

```rust
trait Extension: std::fmt::Debug + Clone + std::any::Any + Send + Sync + 'static { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/ext.rs:44`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/ext.rs#L44)*

#### Implementors

- `T`

