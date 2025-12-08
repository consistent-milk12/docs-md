*[crossbeam_utils](../index.md) / [thread](index.md)*

---

# Module `thread`

Threads that can borrow variables from the stack.

Create a scope when spawned threads need to access variables on the stack:

```rust
use crossbeam_utils::thread;

let people = vec![
    "Alice".to_string(),
    "Bob".to_string(),
    "Carol".to_string(),
];

thread::scope(|s| {
    for person in &people {
        s.spawn(move |_| {
            println!("Hello, {}!", person);
        });
    }
}).unwrap();
```

# Why scoped threads?

Suppose we wanted to re-write the previous example using plain threads:

```compile_fail,E0597
use std::thread;

let people = vec![
    "Alice".to_string(),
    "Bob".to_string(),
    "Carol".to_string(),
];

let mut threads = Vec::new();

for person in &people {
    threads.push(thread::spawn(move || {
        println!("Hello, {}!", person);
    }));
}

for thread in threads {
    thread.join().unwrap();
}
```

This doesn't work because the borrow checker complains about `people` not living long enough:

```text
error[E0597]: `people` does not live long enough
  --> src/main.rs:12:20
   |
12 |     for person in &people {
   |                    ^^^^^^ borrowed value does not live long enough
...
21 | }
   | - borrowed value only lives until here
   |
   = note: borrowed value must be valid for the static lifetime...
```

The problem here is that spawned threads are not allowed to borrow variables on stack because
the compiler cannot prove they will be joined before `people` is destroyed.

Scoped threads are a mechanism to guarantee to the compiler that spawned threads will be joined
before the scope ends.

# How scoped threads work

If a variable is borrowed by a thread, the thread must complete before the variable is
destroyed. Threads spawned using [`std::thread::spawn`](../../rayon_core/spawn/index.md) can only borrow variables with the
`'static` lifetime because the borrow checker cannot be sure when the thread will complete.

A scope creates a clear boundary between variables outside the scope and threads inside the
scope. Whenever a scope spawns a thread, it promises to join the thread before the scope ends.
This way we guarantee to the borrow checker that scoped threads only live within the scope and
can safely access variables outside it.

# Nesting scoped threads

Sometimes scoped threads need to spawn more threads within the same scope. This is a little
tricky because argument `s` lives *inside* the invocation of `thread::scope()` and as such
cannot be borrowed by scoped threads:

```compile_fail,E0521
use crossbeam_utils::thread;

thread::scope(|s| {
    s.spawn(|_| {
        // Not going to compile because we're trying to borrow `s`,
        // which lives *inside* the scope! :(
        s.spawn(|_| println!("nested thread"));
    });
});
```

Fortunately, there is a solution. Every scoped thread is passed a reference to its scope as an
argument, which can be used for spawning nested threads:

```rust
use crossbeam_utils::thread;

thread::scope(|s| {
    // Note the `|s|` here.
    s.spawn(|s| {
        // Yay, this works because we're using a fresh argument `s`! :)
        s.spawn(|_| println!("nested thread"));
    });
}).unwrap();
```

## Contents

- [Modules](#modules)
  - [`unix`](#unix)
- [Structs](#structs)
  - [`Scope`](#scope)
  - [`ScopedThreadBuilder`](#scopedthreadbuilder)
  - [`ScopedJoinHandle`](#scopedjoinhandle)
- [Functions](#functions)
  - [`scope`](#scope)
- [Type Aliases](#type-aliases)
  - [`SharedVec`](#sharedvec)
  - [`SharedOption`](#sharedoption)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod | Unix-specific extensions. |
| [`Scope`](#scope) | struct | A scope for spawning threads. |
| [`ScopedThreadBuilder`](#scopedthreadbuilder) | struct | Configures the properties of a new thread. |
| [`ScopedJoinHandle`](#scopedjoinhandle) | struct | A handle that can be used to join its scoped thread. |
| [`scope`](#scope) | fn | Creates a new scope for spawning threads. |
| [`SharedVec`](#sharedvec) | type |  |
| [`SharedOption`](#sharedoption) | type |  |

## Modules

- [`unix`](unix/index.md) - Unix-specific extensions.

## Structs

### `Scope<'env>`

```rust
struct Scope<'env> {
    handles: std::sync::Arc<std::sync::Mutex<std::vec::Vec<std::sync::Arc<std::sync::Mutex<Option<thread::JoinHandle<()>>>>>>>,
    wait_group: crate::sync::WaitGroup,
    _marker: std::marker::PhantomData<&'env mut &'env ()>,
}
```

A scope for spawning threads.

#### Fields

- **`handles`**: `std::sync::Arc<std::sync::Mutex<std::vec::Vec<std::sync::Arc<std::sync::Mutex<Option<thread::JoinHandle<()>>>>>>>`

  The list of the thread join handles.

- **`wait_group`**: `crate::sync::WaitGroup`

  Used to wait until all subscopes all dropped.

- **`_marker`**: `std::marker::PhantomData<&'env mut &'env ()>`

  Borrows data with invariant lifetime `'env`.

#### Implementations

- <span id="scope-spawn"></span>`fn spawn<'scope, F, T>(self: &'scope Self, f: F) -> ScopedJoinHandle<'scope, T>` — [`ScopedJoinHandle`](#scopedjoinhandle)

- <span id="scope-builder"></span>`fn builder<'scope>(self: &'scope Self) -> ScopedThreadBuilder<'scope, 'env>` — [`ScopedThreadBuilder`](#scopedthreadbuilder)

#### Trait Implementations

##### `impl Debug for Scope<'_>`

- <span id="scope-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Sync for Scope<'_>`

### `ScopedThreadBuilder<'scope, 'env>`

```rust
struct ScopedThreadBuilder<'scope, 'env> {
    scope: &'scope Scope<'env>,
    builder: thread::Builder,
}
```

Configures the properties of a new thread.

The two configurable properties are:

- `name`: Specifies an [associated name for the thread][naming-threads].
- `stack_size`: Specifies the [desired stack size for the thread][stack-size].

The [`spawn`](#spawn) method will take ownership of the builder and return an `io::Result` of the
thread handle with the given configuration.

The `Scope::spawn` method uses a builder with default configuration and unwraps its return
value. You may want to use this builder when you want to recover from a failure to launch a
thread.

# Examples

```rust
use crossbeam_utils::thread;

thread::scope(|s| {
    s.builder()
        .spawn(|_| println!("Running a child thread"))
        .unwrap();
}).unwrap();
```







#### Implementations

- <span id="scopedthreadbuilder-name"></span>`fn name(self, name: String) -> ScopedThreadBuilder<'scope, 'env>` — [`ScopedThreadBuilder`](#scopedthreadbuilder)

- <span id="scopedthreadbuilder-stack-size"></span>`fn stack_size(self, size: usize) -> ScopedThreadBuilder<'scope, 'env>` — [`ScopedThreadBuilder`](#scopedthreadbuilder)

- <span id="scopedthreadbuilder-spawn"></span>`fn spawn<F, T>(self, f: F) -> io::Result<ScopedJoinHandle<'scope, T>>` — [`ScopedJoinHandle`](#scopedjoinhandle)

#### Trait Implementations

##### `impl<'scope, 'env> Debug for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ScopedJoinHandle<'scope, T>`

```rust
struct ScopedJoinHandle<'scope, T> {
    handle: std::sync::Arc<std::sync::Mutex<Option<thread::JoinHandle<()>>>>,
    result: std::sync::Arc<std::sync::Mutex<Option<T>>>,
    thread: thread::Thread,
    _marker: std::marker::PhantomData<&'scope ()>,
}
```

A handle that can be used to join its scoped thread.

This struct is created by the `Scope::spawn` method and the
`ScopedThreadBuilder::spawn` method.

#### Fields

- **`handle`**: `std::sync::Arc<std::sync::Mutex<Option<thread::JoinHandle<()>>>>`

  A join handle to the spawned thread.

- **`result`**: `std::sync::Arc<std::sync::Mutex<Option<T>>>`

  Holds the result of the inner closure.

- **`thread`**: `thread::Thread`

  A handle to the spawned thread.

- **`_marker`**: `std::marker::PhantomData<&'scope ()>`

  Borrows the parent scope with lifetime `'scope`.

#### Implementations

- <span id="scopedjoinhandle-join"></span>`fn join(self) -> thread::Result<T>`

- <span id="scopedjoinhandle-thread"></span>`fn thread(&self) -> &thread::Thread`

#### Trait Implementations

##### `impl<T> Debug for ScopedJoinHandle<'_, T>`

- <span id="scopedjoinhandle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> JoinHandleExt for super::ScopedJoinHandle<'_, T>`

- <span id="superscopedjoinhandle-as-pthread-t"></span>`fn as_pthread_t(&self) -> RawPthread`

- <span id="superscopedjoinhandle-into-pthread-t"></span>`fn into_pthread_t(self) -> RawPthread`

##### `impl<T> Send for ScopedJoinHandle<'_, T>`

##### `impl<T> Sync for ScopedJoinHandle<'_, T>`

## Functions

### `scope`

```rust
fn scope<'env, F, R>(f: F) -> thread::Result<R>
where
    F: FnOnce(&Scope<'env>) -> R
```

Creates a new scope for spawning threads.

All child threads that haven't been manually joined will be automatically joined just before
this function invocation ends. If all joined threads have successfully completed, `Ok` is
returned with the return value of `f`. If any of the joined threads has panicked, an `Err` is
returned containing errors from panicked threads. Note that if panics are implemented by
aborting the process, no error is returned; see the notes of [std::panic::catch_unwind].

**Note:** Since Rust 1.63, this function is soft-deprecated in favor of the more efficient [`std::thread::scope`](../../rayon_core/scope/index.md).

# Examples

```rust
use crossbeam_utils::thread;

let var = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|_| {
        println!("A child thread borrowing `var`: {:?}", var);
    });
}).unwrap();
```

## Type Aliases

### `SharedVec<T>`

```rust
type SharedVec<T> = std::sync::Arc<std::sync::Mutex<std::vec::Vec<T>>>;
```

### `SharedOption<T>`

```rust
type SharedOption<T> = std::sync::Arc<std::sync::Mutex<Option<T>>>;
```

