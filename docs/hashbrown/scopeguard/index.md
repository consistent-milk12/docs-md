*[hashbrown](../index.md) / [scopeguard](index.md)*

---

# Module `scopeguard`

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

#### Implementations

- `fn into_inner(guard: Self) -> T`

#### Trait Implementations

##### `impl<T, F> Deref for ScopeGuard<T, F>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl<T, F> DerefMut for ScopeGuard<T, F>`

- `fn deref_mut(self: &mut Self) -> &mut T`

##### `impl<T, F> Drop for ScopeGuard<T, F>`

- `fn drop(self: &mut Self)`

##### `impl<P, T> Receiver for ScopeGuard<T, F>`

- `type Target = T`

## Functions

### `guard`

```rust
fn guard<T, F>(value: T, dropfn: F) -> ScopeGuard<T, F>
where
    F: FnMut(&mut T)
```

