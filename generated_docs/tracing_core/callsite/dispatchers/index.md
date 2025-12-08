*[tracing_core](../../index.md) / [callsite](../index.md) / [dispatchers](index.md)*

---

# Module `dispatchers`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Dispatchers`](#dispatchers) | struct |  |
| [`Rebuilder`](#rebuilder) | enum |  |

## Structs

### `Dispatchers`

```rust
struct Dispatchers {
    has_just_one: std::sync::atomic::AtomicBool,
}
```

#### Implementations

- <span id="dispatchers-new"></span>`const fn new() -> Self`

- <span id="dispatchers-rebuilder"></span>`fn rebuilder(&self) -> Rebuilder<'_>` — [`Rebuilder`](#rebuilder)

- <span id="dispatchers-register-dispatch"></span>`fn register_dispatch(&self, dispatch: &dispatcher::Dispatch) -> Rebuilder<'_>` — [`Dispatch`](../../index.md), [`Rebuilder`](#rebuilder)

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

- <span id="rebuilder-for-each"></span>`fn for_each(&self, f: impl FnMut(&dispatcher::Dispatch))` — [`Dispatch`](../../index.md)

