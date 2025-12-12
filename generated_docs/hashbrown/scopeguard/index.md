*[hashbrown](../index.md) / [scopeguard](index.md)*

---

# Module `scopeguard`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ScopeGuard`](#scopeguard) | struct |  |
| [`guard`](#guard) | fn |  |

## Structs

### `ScopeGuard<T, F>`

```rust
struct ScopeGuard<T, F>
where
    F: FnMut(&mut T) {
    dropfn: F,
    value: T,
}
```

*Defined in [`hashbrown-0.16.1/src/scopeguard.rs:8-14`](../../../.source_1765521767/hashbrown-0.16.1/src/scopeguard.rs#L8-L14)*

#### Implementations

- <span id="scopeguard-into-inner"></span>`fn into_inner(guard: Self) -> T`

#### Trait Implementations

##### `impl<T, F> Deref for ScopeGuard<T, F>`

- <span id="scopeguard-deref-type-target"></span>`type Target = T`

- <span id="scopeguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<T, F> DerefMut for ScopeGuard<T, F>`

- <span id="scopeguard-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T, F> Drop for ScopeGuard<T, F>`

- <span id="scopeguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for ScopeGuard<T, F>`

- <span id="scopeguard-receiver-type-target"></span>`type Target = T`

## Functions

### `guard`

```rust
fn guard<T, F>(value: T, dropfn: F) -> ScopeGuard<T, F>
where
    F: FnMut(&mut T)
```

*Defined in [`hashbrown-0.16.1/src/scopeguard.rs:17-22`](../../../.source_1765521767/hashbrown-0.16.1/src/scopeguard.rs#L17-L22)*

