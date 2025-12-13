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

- [`unix`](unix/index.md) — Unix-specific extensions.

## Structs

### `Scope<'env>`

```rust
struct Scope<'env> {
    handles: std::sync::Arc<std::sync::Mutex<std::vec::Vec<std::sync::Arc<std::sync::Mutex<Option<thread::JoinHandle<()>>>>>>>,
    wait_group: crate::sync::WaitGroup,
    _marker: std::marker::PhantomData<&'env mut &'env ()>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/thread.rs:213-222`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/thread.rs#L213-L222)*

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

  Spawns a scoped thread.

  

  This method is similar to the [`spawn`](#spawn) function in Rust's standard library. The difference

  is that this thread is scoped, meaning it's guaranteed to terminate before the scope exits,

  allowing it to reference variables outside the scope.

  

  The scoped thread is passed a reference to this scope as an argument, which can be used for

  spawning nested threads.

  

  The returned [handle](ScopedJoinHandle) can be used to manually

  [join](ScopedJoinHandle::join) the thread before the scope exits.

  

  This will create a thread using default parameters of [`ScopedThreadBuilder`](#scopedthreadbuilder), if you want to specify the

  stack size or the name of the thread, use this API instead.

  

  # Panics

  

  Panics if the OS fails to create a thread; use `ScopedThreadBuilder::spawn`

  to recover from such errors.

  

  # Examples

  

  ```rust

  use crossbeam_utils::thread;

  

  thread::scope(|s| {

      let handle = s.spawn(|_| {

          println!("A child thread is running");

          42

      });

  

      // Join the thread and retrieve its result.

      let res = handle.join().unwrap();

      assert_eq!(res, 42);

  }).unwrap();

  ```

- <span id="scope-builder"></span>`fn builder<'scope>(self: &'scope Self) -> ScopedThreadBuilder<'scope, 'env>` — [`ScopedThreadBuilder`](#scopedthreadbuilder)

  Creates a builder that can configure a thread before spawning.

  

  # Examples

  

  ```rust

  use crossbeam_utils::thread;

  

  thread::scope(|s| {

      s.builder()

          .spawn(|_| println!("A child thread is running"))

          .unwrap();

  }).unwrap();

  ```

#### Trait Implementations

##### `impl Any for Scope<'env>`

- <span id="scope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Scope<'env>`

- <span id="scope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Scope<'env>`

- <span id="scope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Scope<'_>`

- <span id="scope-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Scope<'env>`

- <span id="scope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Scope<'env>`

- <span id="scope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for Scope<'_>`

##### `impl<U> TryFrom for Scope<'env>`

- <span id="scope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Scope<'env>`

- <span id="scope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScopedThreadBuilder<'scope, 'env>`

```rust
struct ScopedThreadBuilder<'scope, 'env> {
    scope: &'scope Scope<'env>,
    builder: thread::Builder,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/thread.rs:336-339`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/thread.rs#L336-L339)*

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

  Sets the name for the new thread.

  

  The name must not contain null bytes (`\0`).

  

  For more information about named threads, see [here][naming-threads].

  

  # Examples

  

  ```rust

  use crossbeam_utils::thread;

  use std::thread::current;

  

  thread::scope(|s| {

      s.builder()

          .name("my thread".to_string())

          .spawn(|_| assert_eq!(current().name(), Some("my thread")))

          .unwrap();

  }).unwrap();

  ```

- <span id="scopedthreadbuilder-stack-size"></span>`fn stack_size(self, size: usize) -> ScopedThreadBuilder<'scope, 'env>` — [`ScopedThreadBuilder`](#scopedthreadbuilder)

  Sets the size of the stack for the new thread.

  

  The stack size is measured in bytes.

  

  For more information about the stack size for threads, see [here][stack-size].

  

  # Examples

  

  ```rust

  use crossbeam_utils::thread;

  

  thread::scope(|s| {

      s.builder()

          .stack_size(32 * 1024)

          .spawn(|_| println!("Running a child thread"))

          .unwrap();

  }).unwrap();

  ```

- <span id="scopedthreadbuilder-spawn"></span>`fn spawn<F, T>(self, f: F) -> io::Result<ScopedJoinHandle<'scope, T>>` — [`ScopedJoinHandle`](#scopedjoinhandle)

  Spawns a scoped thread with this configuration.

  

  The scoped thread is passed a reference to this scope as an argument, which can be used for

  spawning nested threads.

  

  The returned handle can be used to manually join the thread before the scope exits.

  

  # Errors

  

  Unlike the `Scope::spawn` method, this method yields an

  `io::Result` to capture any failure to create the thread at

  the OS level.

  

  # Panics

  

  Panics if a thread name was set and it contained null bytes.

  

  # Examples

  

  ```rust

  use crossbeam_utils::thread;

  

  thread::scope(|s| {

      let handle = s.builder()

          .spawn(|_| {

              println!("A child thread is running");

              42

          })

          .unwrap();

  

      // Join the thread and retrieve its result.

      let res = handle.join().unwrap();

      assert_eq!(res, 42);

  }).unwrap();

  ```

#### Trait Implementations

##### `impl Any for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scopedthreadbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScopedThreadBuilder<'scope, 'env>`

- <span id="scopedthreadbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scopedthreadbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScopedJoinHandle<'scope, T>`

```rust
struct ScopedJoinHandle<'scope, T> {
    handle: std::sync::Arc<std::sync::Mutex<Option<thread::JoinHandle<()>>>>,
    result: std::sync::Arc<std::sync::Mutex<Option<T>>>,
    thread: thread::Thread,
    _marker: std::marker::PhantomData<&'scope ()>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/thread.rs:496-508`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/thread.rs#L496-L508)*

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

  Waits for the thread to finish and returns its result.

  

  If the child thread panics, an error is returned. Note that if panics are implemented by

  aborting the process, no error is returned; see the notes of [std::panic::catch_unwind].

  

  # Panics

  

  This function may panic on some platforms if a thread attempts to join itself or otherwise

  may create a deadlock with joining threads.

  

  # Examples

  

  ```rust

  use crossbeam_utils::thread;

  

  thread::scope(|s| {

      let handle1 = s.spawn(|_| println!("I'm a happy thread :)"));

      let handle2 = s.spawn(|_| panic!("I'm a sad thread :("));

  

      // Join the first thread and verify that it succeeded.

      let res = handle1.join();

      assert!(res.is_ok());

  

      // Join the second thread and verify that it panicked.

      let res = handle2.join();

      assert!(res.is_err());

  }).unwrap();

  ```

- <span id="scopedjoinhandle-thread"></span>`fn thread(&self) -> &thread::Thread`

  Returns a handle to the underlying thread.

  

  # Examples

  

  ```rust

  use crossbeam_utils::thread;

  

  thread::scope(|s| {

      let handle = s.spawn(|_| println!("A child thread is running"));

      println!("The child thread ID: {:?}", handle.thread().id());

  }).unwrap();

  ```

#### Trait Implementations

##### `impl<T> Any for ScopedJoinHandle<'scope, T>`

- <span id="scopedjoinhandle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScopedJoinHandle<'scope, T>`

- <span id="scopedjoinhandle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScopedJoinHandle<'scope, T>`

- <span id="scopedjoinhandle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for ScopedJoinHandle<'_, T>`

- <span id="scopedjoinhandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ScopedJoinHandle<'scope, T>`

- <span id="scopedjoinhandle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ScopedJoinHandle<'scope, T>`

- <span id="scopedjoinhandle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> JoinHandleExt for super::ScopedJoinHandle<'_, T>`

- <span id="superscopedjoinhandle-joinhandleext-as-pthread-t"></span>`fn as_pthread_t(&self) -> RawPthread`

- <span id="superscopedjoinhandle-joinhandleext-into-pthread-t"></span>`fn into_pthread_t(self) -> RawPthread`

##### `impl<T> Send for ScopedJoinHandle<'_, T>`

##### `impl<T> Sync for ScopedJoinHandle<'_, T>`

##### `impl<T, U> TryFrom for ScopedJoinHandle<'scope, T>`

- <span id="scopedjoinhandle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scopedjoinhandle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ScopedJoinHandle<'scope, T>`

- <span id="scopedjoinhandle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scopedjoinhandle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `scope`

```rust
fn scope<'env, F, R>(f: F) -> thread::Result<R>
where
    F: FnOnce(&Scope<'env>) -> R
```

*Defined in [`crossbeam-utils-0.8.21/src/thread.rs:153-210`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/thread.rs#L153-L210)*

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

*Defined in [`crossbeam-utils-0.8.21/src/thread.rs:127`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/thread.rs#L127)*

### `SharedOption<T>`

```rust
type SharedOption<T> = std::sync::Arc<std::sync::Mutex<Option<T>>>;
```

*Defined in [`crossbeam-utils-0.8.21/src/thread.rs:128`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/thread.rs#L128)*

