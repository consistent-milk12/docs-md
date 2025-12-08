*[crossbeam_epoch](../index.md) / [guard](index.md)*

---

# Module `guard`

## Structs

### `Guard`

```rust
struct Guard {
    local: *const crate::internal::Local,
}
```

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

- `fn defer<F, R>(self: &Self, f: F)`

- `unsafe fn defer_unchecked<F, R>(self: &Self, f: F)`

- `unsafe fn defer_destroy<T>(self: &Self, ptr: Shared<'_, T>)` — [`Shared`](../atomic/index.md)

- `fn flush(self: &Self)`

- `fn repin(self: &mut Self)`

- `fn repin_after<F, R>(self: &mut Self, f: F) -> R`

- `fn collector(self: &Self) -> Option<&Collector>` — [`Collector`](../collector/index.md)

#### Trait Implementations

##### `impl Debug for Guard`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Guard`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Guard`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `unprotected`

```rust
unsafe fn unprotected() -> &'static Guard
```

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



