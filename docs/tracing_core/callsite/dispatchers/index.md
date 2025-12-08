*[tracing_core](../../index.md) / [callsite](../index.md) / [dispatchers](index.md)*

---

# Module `dispatchers`

## Structs

### `Dispatchers`

```rust
struct Dispatchers {
    has_just_one: std::sync::atomic::AtomicBool,
}
```

#### Implementations

- `const fn new() -> Self`

- `fn rebuilder(self: &Self) -> Rebuilder<'_>` — [`Rebuilder`](#rebuilder)

- `fn register_dispatch(self: &Self, dispatch: &dispatcher::Dispatch) -> Rebuilder<'_>` — [`Dispatch`](../../index.md), [`Rebuilder`](#rebuilder)

## Enums

### `Rebuilder<'a>`

```rust
enum Rebuilder<'a> {
    JustOne,
    Read(std::sync::RwLockReadGuard<'a, alloc::vec::Vec<dispatcher::Registrar>>),
    Write(std::sync::RwLockWriteGuard<'a, alloc::vec::Vec<dispatcher::Registrar>>),
}
```

#### Implementations

- `fn for_each(self: &Self, f: impl FnMut(&dispatcher::Dispatch))` — [`Dispatch`](../../index.md)

