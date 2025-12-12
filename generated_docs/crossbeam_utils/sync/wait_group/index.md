*[crossbeam_utils](../../index.md) / [sync](../index.md) / [wait_group](index.md)*

---

# Module `wait_group`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WaitGroup`](#waitgroup) | struct | Enables threads to synchronize the beginning or end of some computation. |
| [`Inner`](#inner) | struct | Inner state of a `WaitGroup`. |

## Structs

### `WaitGroup`

```rust
struct WaitGroup {
    inner: std::sync::Arc<Inner>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/wait_group.rs:46-48`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/wait_group.rs#L46-L48)*

Enables threads to synchronize the beginning or end of some computation.

# Wait groups vs barriers

`WaitGroup` is very similar to `Barrier`, but there are a few differences:

* `Barrier` needs to know the number of threads at construction, while `WaitGroup` is cloned to
  register more threads.

* A `Barrier` can be reused even after all threads have synchronized, while a `WaitGroup`
  synchronizes threads only once.

* All threads wait for others to reach the `Barrier`. With `WaitGroup`, each thread can choose
  to either wait for other threads or to continue without blocking.

# Examples

```rust
use crossbeam_utils::sync::WaitGroup;
use std::thread;

// Create a new wait group.
let wg = WaitGroup::new();

for _ in 0..4 {
    // Create another reference to the wait group.
    let wg = wg.clone();

    thread::spawn(move || {
        // Do some work.

        // Drop the reference to the wait group.
        drop(wg);
    });
}

// Block until all threads have finished their work.
wg.wait();
std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371
```


#### Implementations

- <span id="waitgroup-new"></span>`fn new() -> Self`

- <span id="waitgroup-wait"></span>`fn wait(self)`

#### Trait Implementations

##### `impl Clone for WaitGroup`

- <span id="waitgroup-clone"></span>`fn clone(&self) -> WaitGroup` — [`WaitGroup`](#waitgroup)

##### `impl Debug for WaitGroup`

- <span id="waitgroup-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WaitGroup`

- <span id="waitgroup-default"></span>`fn default() -> Self`

##### `impl Drop for WaitGroup`

- <span id="waitgroup-drop"></span>`fn drop(&mut self)`

### `Inner`

```rust
struct Inner {
    cvar: std::sync::Condvar,
    count: std::sync::Mutex<usize>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/wait_group.rs:51-54`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/wait_group.rs#L51-L54)*

Inner state of a `WaitGroup`.

