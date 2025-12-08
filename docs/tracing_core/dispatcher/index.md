*[tracing_core](../index.md) / [dispatcher](index.md)*

---

# Module `dispatcher`

Dispatches trace events to [`Subscriber`](../subscriber/index.md)s.

The _dispatcher_ is the component of the tracing system which is responsible
for forwarding trace data from the instrumentation points that generate it
to the subscriber that collects it.

# Using the Trace Dispatcher

Every thread in a program using `tracing` has a _default subscriber_. When
events occur, or spans are created, they are dispatched to the thread's
current subscriber.

## Setting the Default Subscriber

By default, the current subscriber is an empty implementation that does
nothing. To use a subscriber implementation, it must be set as the default.
There are two methods for doing so: [`with_default`](#with-default) and
[`set_global_default`](#set-global-default). `with_default` sets the default subscriber for the
duration of a scope, while `set_global_default` sets a default subscriber
for the entire process.

To use either of these functions, we must first wrap our subscriber in a
[`Dispatch`](#dispatch), a cloneable, type-erased reference to a subscriber. For
example:
```rust
pub struct FooSubscriber;
use tracing_core::{
  dispatcher, Event, Metadata,
  span::{Attributes, Id, Record}
};
impl tracing_core::Subscriber for FooSubscriber {
  fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
  fn record(&self, _: &Id, _: &Record) {}
  fn event(&self, _: &Event) {}
  fn record_follows_from(&self, _: &Id, _: &Id) {}
  fn enabled(&self, _: &Metadata) -> bool { false }
  fn enter(&self, _: &Id) {}
  fn exit(&self, _: &Id) {}
}
impl FooSubscriber { fn new() -> Self { FooSubscriber } }
use dispatcher::Dispatch;

let my_subscriber = FooSubscriber::new();
let my_dispatch = Dispatch::new(my_subscriber);
```
Then, we can use [`with_default`](#with-default) to set our `Dispatch` as the default for
the duration of a block:
```rust
pub struct FooSubscriber;
use tracing_core::{
  dispatcher, Event, Metadata,
  span::{Attributes, Id, Record}
};
impl tracing_core::Subscriber for FooSubscriber {
  fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
  fn record(&self, _: &Id, _: &Record) {}
  fn event(&self, _: &Event) {}
  fn record_follows_from(&self, _: &Id, _: &Id) {}
  fn enabled(&self, _: &Metadata) -> bool { false }
  fn enter(&self, _: &Id) {}
  fn exit(&self, _: &Id) {}
}
impl FooSubscriber { fn new() -> Self { FooSubscriber } }
let my_subscriber = FooSubscriber::new();
let my_dispatch = dispatcher::Dispatch::new(my_subscriber);
// no default subscriber

#[cfg(feature = "std")]
dispatcher::with_default(&my_dispatch, || {
    // my_subscriber is the default
});

// no default subscriber again
```
It's important to note that `with_default` will not propagate the current
thread's default subscriber to any threads spawned within the `with_default`
block. To propagate the default subscriber to new threads, either use
`with_default` from the new thread, or use `set_global_default`.

As an alternative to `with_default`, we can use [`set_global_default`](#set-global-default) to
set a `Dispatch` as the default for all threads, for the lifetime of the
program. For example:
```rust
pub struct FooSubscriber;
use tracing_core::{
  dispatcher, Event, Metadata,
  span::{Attributes, Id, Record}
};
impl tracing_core::Subscriber for FooSubscriber {
  fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
  fn record(&self, _: &Id, _: &Record) {}
  fn event(&self, _: &Event) {}
  fn record_follows_from(&self, _: &Id, _: &Id) {}
  fn enabled(&self, _: &Metadata) -> bool { false }
  fn enter(&self, _: &Id) {}
  fn exit(&self, _: &Id) {}
}
impl FooSubscriber { fn new() -> Self { FooSubscriber } }
let my_subscriber = FooSubscriber::new();
let my_dispatch = dispatcher::Dispatch::new(my_subscriber);
// no default subscriber

dispatcher::set_global_default(my_dispatch)
    // `set_global_default` will return an error if the global default
    // subscriber has already been set.
    .expect("global default was already set!");

// `my_subscriber` is now the default
```

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>:the thread-local scoped dispatcher
    (<a href="#fn.with_default"><code>with_default</code></a>) requires the
    Rust standard library. <code>no_std</code> users should use
    <a href="#fn.set_global_default"><code>set_global_default</code></a>
    instead.
</pre>

## Accessing the Default Subscriber

A thread's current default subscriber can be accessed using the
[`get_default`](#get-default) function, which executes a closure with a reference to the
currently default `Dispatch`. This is used primarily by `tracing`
instrumentation.

## Structs

### `Dispatch`

```rust
struct Dispatch {
    subscriber: Kind<alloc::sync::Arc<dyn Subscriber + Send + Sync>>,
}
```

`Dispatch` trace data to a [`Subscriber`](../subscriber/index.md).

#### Implementations

- `fn none() -> Self`

- `fn new<S>(subscriber: S) -> Self`

- `fn registrar(self: &Self) -> Registrar` — [`Registrar`](#registrar)

- `fn downgrade(self: &Self) -> WeakDispatch` — [`WeakDispatch`](#weakdispatch)

- `fn subscriber(self: &Self) -> &dyn Subscriber + Send + Sync` — [`Subscriber`](../subscriber/index.md)

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> subscriber::Interest` — [`Metadata`](../metadata/index.md), [`Interest`](../subscriber/index.md)

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>` — [`LevelFilter`](../metadata/index.md)

- `fn new_span(self: &Self, span: &span::Attributes<'_>) -> span::Id` — [`Attributes`](../span/index.md), [`Id`](../span/index.md)

- `fn record(self: &Self, span: &span::Id, values: &span::Record<'_>)` — [`Id`](../span/index.md), [`Record`](../span/index.md)

- `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)` — [`Id`](../span/index.md)

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool` — [`Metadata`](../metadata/index.md)

- `fn event(self: &Self, event: &Event<'_>)` — [`Event`](../event/index.md)

- `fn enter(self: &Self, span: &span::Id)` — [`Id`](../span/index.md)

- `fn exit(self: &Self, span: &span::Id)` — [`Id`](../span/index.md)

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id` — [`Id`](../span/index.md)

- `fn drop_span(self: &Self, id: span::Id)` — [`Id`](../span/index.md)

- `fn try_close(self: &Self, id: span::Id) -> bool` — [`Id`](../span/index.md)

- `fn current_span(self: &Self) -> span::Current` — [`Current`](../span/index.md)

- `fn is<T: Any>(self: &Self) -> bool`

- `fn downcast_ref<T: Any>(self: &Self) -> Option<&T>`

#### Trait Implementations

##### `impl Clone for Dispatch`

- `fn clone(self: &Self) -> Dispatch` — [`Dispatch`](#dispatch)

##### `impl Debug for Dispatch`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dispatch`

- `fn default() -> Self`

### `WeakDispatch`

```rust
struct WeakDispatch {
    subscriber: Kind<alloc::sync::Weak<dyn Subscriber + Send + Sync>>,
}
```

`WeakDispatch` is a version of [`Dispatch`](#dispatch) that holds a non-owning reference
to a [`Subscriber`](../subscriber/index.md).

The `Subscriber` may be accessed by calling `WeakDispatch::upgrade`,
which returns an `Option<Dispatch>`. If all [`Dispatch`](#dispatch) clones that point
at the `Subscriber` have been dropped, `WeakDispatch::upgrade` will return
`None`. Otherwise, it will return `Some(Dispatch)`.

A `WeakDispatch` may be created from a [`Dispatch`](#dispatch) by calling the
`Dispatch::downgrade` method. The primary use for creating a
[`WeakDispatch`](#weakdispatch) is to allow a Subscriber` to hold a cyclical reference to
itself without creating a memory leak. See [here] for details.

This type is analogous to the `std::sync::Weak` type, but for a
[`Dispatch`](#dispatch) rather than an `Arc`.



#### Implementations

- `fn upgrade(self: &Self) -> Option<Dispatch>` — [`Dispatch`](#dispatch)

#### Trait Implementations

##### `impl Clone for WeakDispatch`

- `fn clone(self: &Self) -> WeakDispatch` — [`WeakDispatch`](#weakdispatch)

##### `impl Debug for WeakDispatch`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `State`

```rust
struct State {
    default: std::cell::RefCell<Option<Dispatch>>,
    can_enter: std::cell::Cell<bool>,
}
```

The dispatch state of a thread.

#### Fields

- **`default`**: `std::cell::RefCell<Option<Dispatch>>`

  This thread's current default dispatcher.

- **`can_enter`**: `std::cell::Cell<bool>`

  Whether or not we can currently begin dispatching a trace event.
  
  This is set to `false` when functions such as `enter`, `exit`, `event`,
  and `new_span` are called on this thread's default dispatcher, to
  prevent further trace events triggered inside those functions from
  creating an infinite recursion. When we finish handling a dispatch, this
  is set back to `true`.

#### Implementations

- `fn set_default(new_dispatch: Dispatch) -> DefaultGuard` — [`Dispatch`](#dispatch), [`DefaultGuard`](#defaultguard)

- `fn enter(self: &Self) -> Option<Entered<'_>>` — [`Entered`](#entered)

### `Entered<'a>`

```rust
struct Entered<'a>(&'a State);
```

While this guard is active, additional calls to subscriber functions on
the default dispatcher will not be able to access the dispatch context.
Dropping the guard will allow the dispatch context to be re-entered.

#### Implementations

- `fn current(self: &Self) -> Ref<'a, Dispatch>` — [`Dispatch`](#dispatch)

#### Trait Implementations

##### `impl Drop for Entered<'_>`

- `fn drop(self: &mut Self)`

### `DefaultGuard`

```rust
struct DefaultGuard(Option<Dispatch>);
```

A guard that resets the current default dispatcher to the prior
default dispatcher when dropped.

#### Trait Implementations

##### `impl Debug for DefaultGuard`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Drop for DefaultGuard`

- `fn drop(self: &mut Self)`

### `SetGlobalDefaultError`

```rust
struct SetGlobalDefaultError {
    _no_construct: (),
}
```

Returned if setting the global dispatcher fails.

#### Implementations

- `const MESSAGE: &'static str`

#### Trait Implementations

##### `impl Debug for SetGlobalDefaultError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SetGlobalDefaultError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for SetGlobalDefaultError`

##### `impl<T> ToString for SetGlobalDefaultError`

- `fn to_string(self: &Self) -> String`

### `Registrar`

```rust
struct Registrar(Kind<alloc::sync::Weak<dyn Subscriber + Send + Sync>>);
```

#### Implementations

- `fn upgrade(self: &Self) -> Option<Dispatch>` — [`Dispatch`](#dispatch)

## Enums

### `Kind<T>`

```rust
enum Kind<T> {
    Global(&'static dyn Subscriber + Send + Sync),
    Scoped(T),
}
```

#### Implementations

- `fn downgrade(self: &Self) -> Kind<Weak<dyn Subscriber + Send + Sync>>` — [`Kind`](#kind), [`Subscriber`](../subscriber/index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Kind<T>`

- `fn clone(self: &Self) -> Kind<T>` — [`Kind`](#kind)

## Functions

### `with_default`

```rust
fn with_default<T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T
```

Sets this dispatch as the default for the duration of a closure.

The default dispatcher is used when creating a new [`span`](../span/index.md) or
[`Event`](../event/index.md).

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This function required the Rust standard library.
    <code>no_std</code> users should use <a href="fn.set_global_default.html">
    <code>set_global_default</code></a> instead.
</pre>




### `set_default`

```rust
fn set_default(dispatcher: &Dispatch) -> DefaultGuard
```

Sets the dispatch as the default dispatch for the duration of the lifetime
of the returned DefaultGuard

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This function required the Rust standard library.
    <code>no_std</code> users should use <a href="fn.set_global_default.html">
    <code>set_global_default</code></a> instead.
</pre>


### `set_global_default`

```rust
fn set_global_default(dispatcher: Dispatch) -> Result<(), SetGlobalDefaultError>
```

Sets this dispatch as the global default for the duration of the entire program.
Will be used as a fallback if no thread-local dispatch has been set in a thread
(using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail.
Returns `Err` if the global default has already been set.

<div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
    <strong>Warning</strong>: In general, libraries should <em>not</em> call
    <code>set_global_default()</code>! Doing so will cause conflicts when
    executables that depend on the library try to set the default later.
</pre></div>




### `get_default`

```rust
fn get_default<T, F>(f: F) -> T
where
    F: FnMut(&Dispatch) -> T
```

Executes a closure with a reference to this thread's current [dispatcher](#dispatcher).

Note that calls to `get_default` should not be nested; if this function is
called while inside of another `get_default`, that closure will be provided
with `Dispatch::none` rather than the previously set dispatcher.


### `get_global`

```rust
fn get_global() -> &'static Dispatch
```

## Constants

### `CURRENT_STATE`

```rust
const CURRENT_STATE: $crate::thread::LocalKey<State>;
```

### `UNINITIALIZED`

```rust
const UNINITIALIZED: usize = 0usize;
```

### `INITIALIZING`

```rust
const INITIALIZING: usize = 1usize;
```

### `INITIALIZED`

```rust
const INITIALIZED: usize = 2usize;
```

