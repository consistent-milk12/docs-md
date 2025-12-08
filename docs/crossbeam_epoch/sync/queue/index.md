*[crossbeam_epoch](../../index.md) / [sync](../index.md) / [queue](index.md)*

---

# Module `queue`

Michael-Scott lock-free queue.

Usable with any number of producers and consumers.

Michael and Scott.  Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue
Algorithms.  PODC 1996.  <http://dl.acm.org/citation.cfm?id=248106>

Simon Doherty, Lindsay Groves, Victor Luchangco, and Mark Moir. 2004b. Formal Verification of a
Practical Lock-Free Queue Algorithm. <https://doi.org/10.1007/978-3-540-30232-2_7>

## Structs

### `Queue<T>`

```rust
struct Queue<T> {
    head: crossbeam_utils::CachePadded<crate::Atomic<Node<T>>>,
    tail: crossbeam_utils::CachePadded<crate::Atomic<Node<T>>>,
}
```

#### Implementations

- `fn new() -> Queue<T>` — [`Queue`](#queue)

- `fn push_internal(self: &Self, onto: Shared<'_, Node<T>>, new: Shared<'_, Node<T>>, guard: &Guard) -> bool` — [`Shared`](../../atomic/index.md), [`Node`](#node), [`Guard`](../../guard/index.md)

- `fn push(self: &Self, t: T, guard: &Guard)` — [`Guard`](../../guard/index.md)

- `fn pop_internal(self: &Self, guard: &Guard) -> Result<Option<T>, ()>` — [`Guard`](../../guard/index.md)

- `fn pop_if_internal<F>(self: &Self, condition: F, guard: &Guard) -> Result<Option<T>, ()>` — [`Guard`](../../guard/index.md)

- `fn try_pop(self: &Self, guard: &Guard) -> Option<T>` — [`Guard`](../../guard/index.md)

- `fn try_pop_if<F>(self: &Self, condition: F, guard: &Guard) -> Option<T>` — [`Guard`](../../guard/index.md)

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for Queue<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Drop for Queue<T>`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Queue<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Queue<T>`

##### `impl<T: Send> Sync for Queue<T>`

### `Node<T>`

```rust
struct Node<T> {
    data: core::mem::MaybeUninit<T>,
    next: crate::Atomic<Node<T>>,
}
```

#### Fields

- **`data`**: `core::mem::MaybeUninit<T>`

  The slot in which a value of type `T` can be stored.
  
  The type of `data` is `MaybeUninit<T>` because a `Node<T>` doesn't always contain a `T`.
  For example, the sentinel node in a queue never contains a value: its slot is always empty.
  Other nodes start their life with a push operation and contain a value until it gets popped
  out. After that such empty nodes get added to the collector for destruction.

#### Trait Implementations

##### `impl<T> Pointable for Node<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

