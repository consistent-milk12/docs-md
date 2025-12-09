*[regex_automata](../../../index.md) / [util](../../index.md) / [lazy](../index.md) / [lazy](index.md)*

---

# Module `lazy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Lazy`](#lazy) | struct | A non-std lazy initialized value. |

## Structs

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    data: core::sync::atomic::AtomicPtr<T>,
    create: F,
    owned: core::marker::PhantomData<alloc::boxed::Box<T>>,
}
```

A non-std lazy initialized value.

This might run the initialization function more than once, but will
never block.

I wish I could get these semantics into the non-alloc non-std Lazy
type below, but I'm not sure how to do it. If you can do an alloc,
then the implementation becomes very simple if you don't care about
redundant work precisely because a pointer can be atomically swapped.

Perhaps making this approach work in the non-alloc non-std case
requires asking the caller for a pointer? It would make the API less
convenient I think.

#### Implementations

- <span id="lazy-new"></span>`const fn new(create: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

#### Trait Implementations

##### `impl<T: fmt::Debug, F: Fn() -> T> Debug for Lazy<T, F>`

- <span id="lazy-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F> Drop for Lazy<T, F>`

- <span id="lazy-drop"></span>`fn drop(&mut self)`

##### `impl<T: Send + Sync, F: Send + Sync> Sync for Lazy<T, F>`

