*[syn](../index.md) / [thread](index.md)*

---

# Module `thread`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ThreadBound`](#threadbound) | struct | ThreadBound is a Sync-maker and Send-maker that allows accessing a value |

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

- <span id="threadbound-new"></span>`fn new(value: T) -> Self`

- <span id="threadbound-get"></span>`fn get(&self) -> Option<&T>`

#### Trait Implementations

##### `impl<T: Copy> Clone for ThreadBound<T>`

- <span id="threadbound-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Copy> Copy for ThreadBound<T>`

##### `impl<T: Debug> Debug for ThreadBound<T>`

- <span id="threadbound-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Copy> Send for ThreadBound<T>`

##### `impl<T> Sync for ThreadBound<T>`

