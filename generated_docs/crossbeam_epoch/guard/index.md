*[crossbeam_epoch](../index.md) / [guard](index.md)*

---

# Module `guard`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Guard`](#guard) | struct | A guard that keeps the current thread pinned. |
| [`unprotected`](#unprotected) | fn | Returns a reference to a dummy guard that allows unprotected access to [`Atomic`]s. |

## Structs

### `Guard`

```rust
struct Guard {
    local: *const crate::internal::Local,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/guard.rs:69-71`](../../../.source_1765521767/crossbeam-epoch-0.9.18/src/guard.rs#L69-L71)*

A guard that keeps the current thread pinned.

# Pinning

The current thread is pinned by calling [`pin`](../default/index.md), which returns a new guard:

```rust
use crossbeam_epoch as epoch;

// It is often convenient to prefix a call to `pin` with a `&` in order to create a reference.
// This is not really necessary, but makes passing references to the guard a bit easier.
let guard = &epoch::pin();
```

When a guard gets dropped, the current thread is automatically unpinned.

# Pointers on the stack

Having a guard allows us to create pointers on the stack to heap-allocated objects.
For example:

```rust
use crossbeam_epoch::{self as epoch, Atomic};
use std::sync::atomic::Ordering::SeqCst;

// Create a heap-allocated number.
let a = Atomic::new(777);

// Pin the current thread.
let guard = &epoch::pin();

// Load the heap-allocated object and create pointer `p` on the stack.
let p = a.load(SeqCst, guard);

// Dereference the pointer and print the value:
if let Some(num) = unsafe { p.as_ref() } {
    println!("The number is {}.", num);
}
unsafe { drop(a.into_owned()); } // avoid leak
```

# Multiple guards

Pinning is reentrant and it is perfectly legal to create multiple guards. In that case, the
thread will actually be pinned only when the first guard is created and unpinned when the last
one is dropped:

```rust
use crossbeam_epoch as epoch;

let guard1 = epoch::pin();
let guard2 = epoch::pin();
assert!(epoch::is_pinned());
drop(guard1);
assert!(epoch::is_pinned());
drop(guard2);
assert!(!epoch::is_pinned());
```


#### Implementations

- <span id="guard-defer"></span>`fn defer<F, R>(&self, f: F)`

  Stores a function so that it can be executed at some point after all currently pinned

  threads get unpinned.

  

  This method first stores `f` into the thread-local (or handle-local) cache. If this cache

  becomes full, some functions are moved into the global cache. At the same time, some

  functions from both local and global caches may get executed in order to incrementally

  clean up the caches as they fill up.

  

  There is no guarantee when exactly `f` will be executed. The only guarantee is that it

  won't be executed until all currently pinned threads get unpinned. In theory, `f` might

  never run, but the epoch-based garbage collection will make an effort to execute it

  reasonably soon.

  

  If this method is called from an [`unprotected`](#unprotected) guard, the function will simply be

  executed immediately.

- <span id="guard-defer-unchecked"></span>`unsafe fn defer_unchecked<F, R>(&self, f: F)`

  Stores a function so that it can be executed at some point after all currently pinned

  threads get unpinned.

  

  This method first stores `f` into the thread-local (or handle-local) cache. If this cache

  becomes full, some functions are moved into the global cache. At the same time, some

  functions from both local and global caches may get executed in order to incrementally

  clean up the caches as they fill up.

  

  There is no guarantee when exactly `f` will be executed. The only guarantee is that it

  won't be executed until all currently pinned threads get unpinned. In theory, `f` might

  never run, but the epoch-based garbage collection will make an effort to execute it

  reasonably soon.

  

  If this method is called from an [`unprotected`](#unprotected) guard, the function will simply be

  executed immediately.

  

  # Safety

  

  The given function must not hold reference onto the stack. It is highly recommended that

  the passed function is **always** marked with `move` in order to prevent accidental

  borrows.

  

  ```rust

  use crossbeam_epoch as epoch;

  

  let guard = &epoch::pin();

  let message = "Hello!";

  unsafe {

      // ALWAYS use `move` when sending a closure into `defer_unchecked`.

      guard.defer_unchecked(move || {

          println!("{}", message);

      });

  }

  ```

  

  Apart from that, keep in mind that another thread may execute `f`, so anything accessed by

  the closure must be `Send`.

  

  We intentionally didn't require `F: Send`, because Rust's type systems usually cannot prove

  `F: Send` for typical use cases. For example, consider the following code snippet, which

  exemplifies the typical use case of deferring the deallocation of a shared reference:

  

  ```ignore

  let shared = Owned::new(7i32).into_shared(guard);

  guard.defer_unchecked(move || shared.into_owned()); // `Shared` is not `Send`!

  ```

  

  While `Shared` is not `Send`, it's safe for another thread to call the deferred function,

  because it's called only after the grace period and `shared` is no longer shared with other

  threads. But we don't expect type systems to prove this.

  

  # Examples

  

  When a heap-allocated object in a data structure becomes unreachable, it has to be

  deallocated. However, the current thread and other threads may be still holding references

  on the stack to that same object. Therefore it cannot be deallocated before those references

  get dropped. This method can defer deallocation until all those threads get unpinned and

  consequently drop all their references on the stack.

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic, Owned};

  use std::sync::atomic::Ordering::SeqCst;

  

  let a = Atomic::new("foo");

  

  // Now suppose that `a` is shared among multiple threads and concurrently

  // accessed and modified...

  

  // Pin the current thread.

  let guard = &epoch::pin();

  

  // Steal the object currently stored in `a` and swap it with another one.

  let p = a.swap(Owned::new("bar").into_shared(guard), SeqCst, guard);

  

  if !p.is_null() {

      // The object `p` is pointing to is now unreachable.

      // Defer its deallocation until all currently pinned threads get unpinned.

      unsafe {

          // ALWAYS use `move` when sending a closure into `defer_unchecked`.

          guard.defer_unchecked(move || {

              println!("{} is now being deallocated.", p.deref());

              // Now we have unique access to the object pointed to by `p` and can turn it

              // into an `Owned`. Dropping the `Owned` will deallocate the object.

              drop(p.into_owned());

          });

      }

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-defer-destroy"></span>`unsafe fn defer_destroy<T>(&self, ptr: Shared<'_, T>)` — [`Shared`](../atomic/index.md#shared)

  Stores a destructor for an object so that it can be deallocated and dropped at some point

  after all currently pinned threads get unpinned.

  

  This method first stores the destructor into the thread-local (or handle-local) cache. If

  this cache becomes full, some destructors are moved into the global cache. At the same

  time, some destructors from both local and global caches may get executed in order to

  incrementally clean up the caches as they fill up.

  

  There is no guarantee when exactly the destructor will be executed. The only guarantee is

  that it won't be executed until all currently pinned threads get unpinned. In theory, the

  destructor might never run, but the epoch-based garbage collection will make an effort to

  execute it reasonably soon.

  

  If this method is called from an [`unprotected`](#unprotected) guard, the destructor will simply be

  executed immediately.

  

  # Safety

  

  The object must not be reachable by other threads anymore, otherwise it might be still in

  use when the destructor runs.

  

  Apart from that, keep in mind that another thread may execute the destructor, so the object

  must be sendable to other threads.

  

  We intentionally didn't require `T: Send`, because Rust's type systems usually cannot prove

  `T: Send` for typical use cases. For example, consider the following code snippet, which

  exemplifies the typical use case of deferring the deallocation of a shared reference:

  

  ```ignore

  let shared = Owned::new(7i32).into_shared(guard);

  guard.defer_destroy(shared); // `Shared` is not `Send`!

  ```

  

  While `Shared` is not `Send`, it's safe for another thread to call the destructor, because

  it's called only after the grace period and `shared` is no longer shared with other

  threads. But we don't expect type systems to prove this.

  

  # Examples

  

  When a heap-allocated object in a data structure becomes unreachable, it has to be

  deallocated. However, the current thread and other threads may be still holding references

  on the stack to that same object. Therefore it cannot be deallocated before those references

  get dropped. This method can defer deallocation until all those threads get unpinned and

  consequently drop all their references on the stack.

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic, Owned};

  use std::sync::atomic::Ordering::SeqCst;

  

  let a = Atomic::new("foo");

  

  // Now suppose that `a` is shared among multiple threads and concurrently

  // accessed and modified...

  

  // Pin the current thread.

  let guard = &epoch::pin();

  

  // Steal the object currently stored in `a` and swap it with another one.

  let p = a.swap(Owned::new("bar").into_shared(guard), SeqCst, guard);

  

  if !p.is_null() {

      // The object `p` is pointing to is now unreachable.

      // Defer its deallocation until all currently pinned threads get unpinned.

      unsafe {

          guard.defer_destroy(p);

      }

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-flush"></span>`fn flush(&self)`

  Clears up the thread-local cache of deferred functions by executing them or moving into the

  global cache.

  

  Call this method after deferring execution of a function if you want to get it executed as

  soon as possible. Flushing will make sure it is residing in in the global cache, so that

  any thread has a chance of taking the function and executing it.

  

  If this method is called from an [`unprotected`](#unprotected) guard, it is a no-op (nothing happens).

  

  # Examples

  

  ```rust

  use crossbeam_epoch as epoch;

  

  let guard = &epoch::pin();

  guard.defer(move || {

      println!("This better be printed as soon as possible!");

  });

  guard.flush();

  ```

- <span id="guard-repin"></span>`fn repin(&mut self)`

  Unpins and then immediately re-pins the thread.

  

  This method is useful when you don't want delay the advancement of the global epoch by

  holding an old epoch. For safety, you should not maintain any guard-based reference across

  the call (the latter is enforced by `&mut self`). The thread will only be repinned if this

  is the only active guard for the current thread.

  

  If this method is called from an [`unprotected`](#unprotected) guard, then the call will be just no-op.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic};

  use std::sync::atomic::Ordering::SeqCst;

  

  let a = Atomic::new(777);

  let mut guard = epoch::pin();

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  guard.repin();

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-repin-after"></span>`fn repin_after<F, R>(&mut self, f: F) -> R`

  Temporarily unpins the thread, executes the given function and then re-pins the thread.

  

  This method is useful when you need to perform a long-running operation (e.g. sleeping)

  and don't need to maintain any guard-based reference across the call (the latter is enforced

  by `&mut self`). The thread will only be unpinned if this is the only active guard for the

  current thread.

  

  If this method is called from an [`unprotected`](#unprotected) guard, then the passed function is called

  directly without unpinning the thread.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic};

  use std::sync::atomic::Ordering::SeqCst;

  use std::thread;

  use std::time::Duration;

  

  let a = Atomic::new(777);

  let mut guard = epoch::pin();

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  guard.repin_after(|| thread::sleep(Duration::from_millis(50)));

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-collector"></span>`fn collector(&self) -> Option<&Collector>` — [`Collector`](../collector/index.md#collector)

  Returns the `Collector` associated with this guard.

  

  This method is useful when you need to ensure that all guards used with

  a data structure come from the same collector.

  

  If this method is called from an [`unprotected`](#unprotected) guard, then `None` is returned.

  

  # Examples

  

  ```rust

  use crossbeam_epoch as epoch;

  

  let guard1 = epoch::pin();

  let guard2 = epoch::pin();

  assert!(guard1.collector() == guard2.collector());

  ```

#### Trait Implementations

##### `impl Any for Guard`

- <span id="guard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Guard`

- <span id="guard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Guard`

- <span id="guard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Guard`

- <span id="guard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Guard`

- <span id="guard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Guard`

- <span id="guard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Guard`

- <span id="guard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Guard`

- <span id="guard-pointable-const-align"></span>`const ALIGN: usize`

- <span id="guard-pointable-type-init"></span>`type Init = T`

- <span id="guard-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="guard-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="guard-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="guard-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Guard`

- <span id="guard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="guard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Guard`

- <span id="guard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="guard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `unprotected`

```rust
unsafe fn unprotected() -> &'static Guard
```

*Defined in [`crossbeam-epoch-0.9.18/src/guard.rs:513-523`](../../../.source_1765521767/crossbeam-epoch-0.9.18/src/guard.rs#L513-L523)*

Returns a reference to a dummy guard that allows unprotected access to [`Atomic`](../atomic/index.md)s.

This guard should be used in special occasions only. Note that it doesn't actually keep any
thread pinned - it's just a fake guard that allows loading from [`Atomic`](../atomic/index.md)s unsafely.

Note that calling `defer` with a dummy guard will not defer the function - it will just
execute the function immediately.

If necessary, it's possible to create more dummy guards by cloning: `unprotected().clone()`.

# Safety

Loading and dereferencing data from an [`Atomic`](../atomic/index.md) using this guard is safe only if the
[`Atomic`](../atomic/index.md) is not being concurrently modified by other threads.

# Examples

```rust
use crossbeam_epoch::{self as epoch, Atomic};
use std::sync::atomic::Ordering::Relaxed;

let a = Atomic::new(7);

unsafe {
    // Load `a` without pinning the current thread.
    a.load(Relaxed, epoch::unprotected());

    // It's possible to create more dummy guards.
    let dummy = epoch::unprotected();

    dummy.defer(move || {
        println!("This gets executed immediately.");
    });

    // Dropping `dummy` doesn't affect the current thread - it's just a noop.
}
unsafe { drop(a.into_owned()); } // avoid leak
```

The most common use of this function is when constructing or destructing a data structure.

For example, we can use a dummy guard in the destructor of a Treiber stack because at that
point no other thread could concurrently modify the [`Atomic`](../atomic/index.md)s we are accessing.

If we were to actually pin the current thread during destruction, that would just unnecessarily
delay garbage collection and incur some performance cost, so in cases like these `unprotected`
is very helpful.

```rust
use crossbeam_epoch::{self as epoch, Atomic};
use std::mem::ManuallyDrop;
use std::sync::atomic::Ordering::Relaxed;

struct Stack<T> {
    head: Atomic<Node<T>>,
}

struct Node<T> {
    data: ManuallyDrop<T>,
    next: Atomic<Node<T>>,
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe {
            // Unprotected load.
            let mut node = self.head.load(Relaxed, epoch::unprotected());

            while let Some(n) = node.as_ref() {
                // Unprotected load.
                let next = n.next.load(Relaxed, epoch::unprotected());

                // Take ownership of the node, then drop its data and deallocate it.
                let mut o = node.into_owned();
                ManuallyDrop::drop(&mut o.data);
                drop(o);

                node = next;
            }
        }
    }
}
```



