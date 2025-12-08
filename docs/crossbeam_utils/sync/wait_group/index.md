*[crossbeam_utils](../../index.md) / [sync](../index.md) / [wait_group](index.md)*

---

# Module `wait_group`

## Structs

### `WaitGroup`

```rust
struct WaitGroup {
    inner: std::sync::Arc<Inner>,
}
```

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

- `fn new() -> Self`

- `fn wait(self: Self)`

#### Trait Implementations

##### `impl Clone for WaitGroup`

- `fn clone(self: &Self) -> WaitGroup` — [`WaitGroup`](#waitgroup)

##### `impl Debug for WaitGroup`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WaitGroup`

- `fn default() -> Self`

##### `impl Drop for WaitGroup`

- `fn drop(self: &mut Self)`

### `Inner`

```rust
struct Inner {
    cvar: std::sync::Condvar,
    count: std::sync::Mutex<usize>,
}
```

Inner state of a `WaitGroup`.

