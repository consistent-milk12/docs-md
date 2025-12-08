*[regex_automata](../../../index.md) / [util](../../index.md) / [lazy](../index.md) / [lazy](index.md)*

---

# Module `lazy`

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

- `fn get(self: &Self) -> &T`

- `fn poll(self: &Self) -> Option<&T>`

#### Trait Implementations

##### `impl<T: fmt::Debug, F: Fn() -> T> Debug for Lazy<T, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F> Drop for Lazy<T, F>`

- `fn drop(self: &mut Self)`

##### `impl<T: Send + Sync, F: Send + Sync> Sync for Lazy<T, F>`

