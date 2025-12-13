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

*Defined in [`crossbeam-epoch-0.9.18/src/sync/queue.rs:22-25`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/sync/queue.rs#L22-L25)*

#### Implementations

- <span id="queue-new"></span>`fn new() -> Queue<T>` — [`Queue`](#queue)

  Create a new, empty queue.

- <span id="queue-push-internal"></span>`fn push_internal(&self, onto: Shared<'_, Node<T>>, new: Shared<'_, Node<T>>, guard: &Guard) -> bool` — [`Shared`](../../atomic/index.md#shared), [`Node`](#node), [`Guard`](../../guard/index.md#guard)

  Attempts to atomically place `n` into the `next` pointer of `onto`, and returns `true` on

  success. The queue's `tail` pointer may be updated.

- <span id="queue-push"></span>`fn push(&self, t: T, guard: &Guard)` — [`Guard`](../../guard/index.md#guard)

  Adds `t` to the back of the queue, possibly waking up threads blocked on `pop`.

- <span id="queue-pop-internal"></span>`fn pop_internal(&self, guard: &Guard) -> Result<Option<T>, ()>` — [`Guard`](../../guard/index.md#guard)

  Attempts to pop a data node. `Ok(None)` if queue is empty; `Err(())` if lost race to pop.

- <span id="queue-pop-if-internal"></span>`fn pop_if_internal<F>(&self, condition: F, guard: &Guard) -> Result<Option<T>, ()>` — [`Guard`](../../guard/index.md#guard)

  Attempts to pop a data node, if the data satisfies the given condition. `Ok(None)` if queue

  is empty or the data does not satisfy the condition; `Err(())` if lost race to pop.

- <span id="queue-try-pop"></span>`fn try_pop(&self, guard: &Guard) -> Option<T>` — [`Guard`](../../guard/index.md#guard)

  Attempts to dequeue from the front.

  

  Returns `None` if the queue is observed to be empty.

- <span id="queue-try-pop-if"></span>`fn try_pop_if<F>(&self, condition: F, guard: &Guard) -> Option<T>` — [`Guard`](../../guard/index.md#guard)

  Attempts to dequeue from the front, if the item satisfies the given condition.

  

  Returns `None` if the queue is observed to be empty, or the head does not satisfy the given

  condition.

#### Trait Implementations

##### `impl<T> Any for Queue<T>`

- <span id="queue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Queue<T>`

- <span id="queue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Queue<T>`

- <span id="queue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for Queue<T>`

- <span id="queue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Queue<T>`

- <span id="queue-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Queue<T>`

- <span id="queue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Queue<T>`

- <span id="queue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Queue<T>`

- <span id="queue-pointable-const-align"></span>`const ALIGN: usize`

- <span id="queue-pointable-type-init"></span>`type Init = T`

- <span id="queue-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="queue-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="queue-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="queue-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Queue<T>`

##### `impl<T: Send> Sync for Queue<T>`

##### `impl<T, U> TryFrom for Queue<T>`

- <span id="queue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="queue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Queue<T>`

- <span id="queue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="queue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Node<T>`

```rust
struct Node<T> {
    data: core::mem::MaybeUninit<T>,
    next: crate::Atomic<Node<T>>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/queue.rs:27-37`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/sync/queue.rs#L27-L37)*

#### Fields

- **`data`**: `core::mem::MaybeUninit<T>`

  The slot in which a value of type `T` can be stored.
  
  The type of `data` is `MaybeUninit<T>` because a `Node<T>` doesn't always contain a `T`.
  For example, the sentinel node in a queue never contains a value: its slot is always empty.
  Other nodes start their life with a push operation and contain a value until it gets popped
  out. After that such empty nodes get added to the collector for destruction.

#### Trait Implementations

##### `impl<T> Any for Node<T>`

- <span id="node-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Node<T>`

- <span id="node-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Node<T>`

- <span id="node-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Node<T>`

- <span id="node-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Node<T>`

- <span id="node-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Node<T>`

- <span id="node-pointable-const-align"></span>`const ALIGN: usize`

- <span id="node-pointable-type-init"></span>`type Init = T`

- <span id="node-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="node-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="node-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="node-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Node<T>`

- <span id="node-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="node-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Node<T>`

- <span id="node-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="node-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

