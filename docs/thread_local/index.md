# Crate `thread_local`

Per-object thread-local storage

This library provides the `ThreadLocal` type which allows a separate copy of
an object to be used for each thread. This allows for per-object
thread-local storage, unlike the standard library's `thread_local!` macro
which only allows static thread-local storage.

Per-thread objects are not destroyed when a thread exits. Instead, objects
are only destroyed when the `ThreadLocal` containing them is destroyed.

You can also iterate over the thread-local values of all thread in a
`ThreadLocal` object using the `iter_mut` and `into_iter` methods. This can
only be done if you have mutable access to the `ThreadLocal` object, which
guarantees that you are the only thread currently accessing it.

Note that since thread IDs are recycled when a thread exits, it is possible
for one thread to retrieve the object of another thread. Since this can only
occur after a thread has exited this does not lead to any race conditions.

# Examples

Basic usage of `ThreadLocal`:

```rust
use thread_local::ThreadLocal;
let tls: ThreadLocal<u32> = ThreadLocal::new();
assert_eq!(tls.get(), None);
assert_eq!(tls.get_or(|| 5), &5);
assert_eq!(tls.get(), Some(&5));
```

Combining thread-local values into a single result:

```rust
use thread_local::ThreadLocal;
use std::sync::Arc;
use std::cell::Cell;
use std::thread;

let tls = Arc::new(ThreadLocal::new());

// Create a bunch of threads to do stuff
for _ in 0..5 {
    let tls2 = tls.clone();
    thread::spawn(move || {
        // Increment a counter to count some event...
        let cell = tls2.get_or(|| Cell::new(0));
        cell.set(cell.get() + 1);
    }).join().unwrap();
}

// Once all threads are done, collect the counter values and return the
// sum of all thread-local counter values.
let tls = Arc::try_unwrap(tls).unwrap();
let total = tls.into_iter().fold(0, |x, y| x + y.get());
assert_eq!(total, 5);
```

## Structs

### `CachedIntoIter<T: Send>`

```rust
struct CachedIntoIter<T: Send> {
    inner: super::IntoIter<T>,
}
```

An iterator that moves out of a `CachedThreadLocal`.

#### Trait Implementations

##### `impl<T: Send> ExactSizeIterator for CachedIntoIter<T>`

##### `impl<I> IntoIterator for CachedIntoIter<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T: Send> Iterator for CachedIntoIter<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `CachedIterMut<'a, T: Send + 'a>`

```rust
struct CachedIterMut<'a, T: Send + 'a> {
    inner: super::IterMut<'a, T>,
}
```

Mutable iterator over the contents of a `CachedThreadLocal`.

#### Trait Implementations

##### `impl<'a, T: Send + 'a> ExactSizeIterator for CachedIterMut<'a, T>`

##### `impl<I> IntoIterator for CachedIterMut<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T: Send + 'a> Iterator for CachedIterMut<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<&'a mut T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `CachedThreadLocal<T: Send>`

```rust
struct CachedThreadLocal<T: Send> {
    inner: super::ThreadLocal<T>,
}
```

Wrapper around [`ThreadLocal`](#threadlocal).

This used to add a fast path for a single thread, however that has been
obsoleted by performance improvements to [`ThreadLocal`](#threadlocal) itself.

#### Implementations

- `fn new() -> CachedThreadLocal<T>` — [`CachedThreadLocal`](cached/index.md)

- `fn get(self: &Self) -> Option<&T>`

- `fn get_or<F>(self: &Self, create: F) -> &T`

- `fn get_or_try<F, E>(self: &Self, create: F) -> Result<&T, E>`

- `fn iter_mut(self: &mut Self) -> CachedIterMut<'_, T>` — [`CachedIterMut`](cached/index.md)

- `fn clear(self: &mut Self)`

#### Trait Implementations

##### `impl<T: Send + fmt::Debug> Debug for CachedThreadLocal<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> Default for CachedThreadLocal<T>`

- `fn default() -> CachedThreadLocal<T>` — [`CachedThreadLocal`](cached/index.md)

##### `impl<T: Send> IntoIterator for CachedThreadLocal<T>`

- `type Item = T`

- `type IntoIter = CachedIntoIter<T>`

- `fn into_iter(self: Self) -> CachedIntoIter<T>` — [`CachedIntoIter`](cached/index.md)

##### `impl<T: Send + UnwindSafe> UnwindSafe for CachedThreadLocal<T>`

### `ThreadLocal<T: Send>`

```rust
struct ThreadLocal<T: Send> {
    buckets: [std::sync::atomic::AtomicPtr<Entry<T>>; 63],
    values: std::sync::atomic::AtomicUsize,
}
```

Thread-local variable wrapper

See the [module-level documentation](index.html) for more.

#### Fields

- **`buckets`**: `[std::sync::atomic::AtomicPtr<Entry<T>>; 63]`

  The buckets in the thread local. The nth bucket contains `2^n`
  elements. Each bucket is lazily allocated.

- **`values`**: `std::sync::atomic::AtomicUsize`

  The number of values in the thread local. This can be less than the real number of values,
  but is never more.

#### Implementations

- `fn get_or_default(self: &Self) -> &T`

#### Trait Implementations

##### `impl<T: Send + fmt::Debug> Debug for ThreadLocal<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> Default for ThreadLocal<T>`

- `fn default() -> ThreadLocal<T>` — [`ThreadLocal`](#threadlocal)

##### `impl<T: Send> Drop for ThreadLocal<T>`

- `fn drop(self: &mut Self)`

##### `impl<T: Send> IntoIterator for ThreadLocal<T>`

- `type Item = T`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T: Send> Sync for ThreadLocal<T>`

##### `impl<T: Send + UnwindSafe> UnwindSafe for ThreadLocal<T>`

### `Iter<'a, T: Send + Sync>`

```rust
struct Iter<'a, T: Send + Sync> {
    thread_local: &'a ThreadLocal<T>,
    raw: RawIter,
}
```

Iterator over the contents of a `ThreadLocal`.

#### Trait Implementations

##### `impl<'a, T: $crate::fmt::Debug + Send + Sync> Debug for Iter<'a, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send + Sync> FusedIterator for Iter<'_, T>`

##### `impl<I> IntoIterator for Iter<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T: Send + Sync> Iterator for Iter<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IterMut<'a, T: Send>`

```rust
struct IterMut<'a, T: Send> {
    thread_local: &'a mut ThreadLocal<T>,
    raw: RawIter,
}
```

Mutable iterator over the contents of a `ThreadLocal`.

#### Trait Implementations

##### `impl<'a, T: Send + fmt::Debug> Debug for IterMut<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> ExactSizeIterator for IterMut<'_, T>`

##### `impl<T: Send> FusedIterator for IterMut<'_, T>`

##### `impl<I> IntoIterator for IterMut<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T: Send> Iterator for IterMut<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<&'a mut T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IntoIter<T: Send>`

```rust
struct IntoIter<T: Send> {
    thread_local: ThreadLocal<T>,
    raw: RawIter,
}
```

An iterator that moves out of a `ThreadLocal`.

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug + Send> Debug for IntoIter<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> ExactSizeIterator for IntoIter<T>`

##### `impl<T: Send> FusedIterator for IntoIter<T>`

##### `impl<I> IntoIterator for IntoIter<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T: Send> Iterator for IntoIter<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

