*[crossbeam_epoch](../../index.md) / [sync](../index.md) / [queue](index.md)*

---

# Module `queue`

Michael-Scott lock-free queue.

Usable with any number of producers and consumers.

Michael and Scott.  Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue
Algorithms.  PODC 1996.  <http://dl.acm.org/citation.cfm?id=248106>

Simon Doherty, Lindsay Groves, Victor Luchangco, and Mark Moir. 2004b. Formal Verification of a
Practical Lock-Free Queue Algorithm. <https://doi.org/10.1007/978-3-540-30232-2_7>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Queue`](#queue) | struct |  |
| [`Node`](#node) | struct |  |

## Structs

### `Queue<T>`

```rust
struct Queue<T> {
    head: crossbeam_utils::CachePadded<crate::Atomic<Node<T>>>,
    tail: crossbeam_utils::CachePadded<crate::Atomic<Node<T>>>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/queue.rs:22-25`](../../../../.source_1765210505/crossbeam-epoch-0.9.18/src/sync/queue.rs#L22-L25)*

#### Implementations

- <span id="queue-new"></span>`fn new() -> Queue<T>` — [`Queue`](#queue)

- <span id="queue-push-internal"></span>`fn push_internal(&self, onto: Shared<'_, Node<T>>, new: Shared<'_, Node<T>>, guard: &Guard) -> bool` — [`Shared`](../../atomic/index.md), [`Node`](#node), [`Guard`](../../guard/index.md)

- <span id="queue-push"></span>`fn push(&self, t: T, guard: &Guard)` — [`Guard`](../../guard/index.md)

- <span id="queue-pop-internal"></span>`fn pop_internal(&self, guard: &Guard) -> Result<Option<T>, ()>` — [`Guard`](../../guard/index.md)

- <span id="queue-pop-if-internal"></span>`fn pop_if_internal<F>(&self, condition: F, guard: &Guard) -> Result<Option<T>, ()>` — [`Guard`](../../guard/index.md)

- <span id="queue-try-pop"></span>`fn try_pop(&self, guard: &Guard) -> Option<T>` — [`Guard`](../../guard/index.md)

- <span id="queue-try-pop-if"></span>`fn try_pop_if<F>(&self, condition: F, guard: &Guard) -> Option<T>` — [`Guard`](../../guard/index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Queue<T>`

- <span id="queue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Queue<T>`

- <span id="queue-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Queue<T>`

- <span id="queue-const-align"></span>`const ALIGN: usize`

- <span id="queue-type-init"></span>`type Init = T`

- <span id="queue-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md)

- <span id="queue-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="queue-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="queue-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Queue<T>`

##### `impl<T: Send> Sync for Queue<T>`

### `Node<T>`

```rust
struct Node<T> {
    data: core::mem::MaybeUninit<T>,
    next: crate::Atomic<Node<T>>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/queue.rs:27-37`](../../../../.source_1765210505/crossbeam-epoch-0.9.18/src/sync/queue.rs#L27-L37)*

#### Fields

- **`data`**: `core::mem::MaybeUninit<T>`

  The slot in which a value of type `T` can be stored.
  
  The type of `data` is `MaybeUninit<T>` because a `Node<T>` doesn't always contain a `T`.
  For example, the sentinel node in a queue never contains a value: its slot is always empty.
  Other nodes start their life with a push operation and contain a value until it gets popped
  out. After that such empty nodes get added to the collector for destruction.

#### Trait Implementations

##### `impl<T> Pointable for Node<T>`

- <span id="node-const-align"></span>`const ALIGN: usize`

- <span id="node-type-init"></span>`type Init = T`

- <span id="node-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md)

- <span id="node-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="node-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="node-drop"></span>`unsafe fn drop(ptr: usize)`

