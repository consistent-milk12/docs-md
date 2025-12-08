*[syn](../index.md) / [thread](index.md)*

---

# Module `thread`

## Structs

### `ThreadBound<T>`

```rust
struct ThreadBound<T> {
    value: T,
    thread_id: std::thread::ThreadId,
}
```

ThreadBound is a Sync-maker and Send-maker that allows accessing a value
of type T only from the original thread on which the ThreadBound was
constructed.

#### Implementations

- `fn new(value: T) -> Self`

- `fn get(self: &Self) -> Option<&T>`

#### Trait Implementations

##### `impl<T: Copy> Clone for ThreadBound<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: Copy> Copy for ThreadBound<T>`

##### `impl<T: Debug> Debug for ThreadBound<T>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Copy> Send for ThreadBound<T>`

##### `impl<T> Sync for ThreadBound<T>`

