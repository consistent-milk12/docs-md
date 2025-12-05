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
    // [REDACTED: Private Fields]
}
```

`Dispatch` trace data to a [`Subscriber`](../subscriber/index.md).

#### Implementations

- `fn none() -> Self`
  Returns a new `Dispatch` that discards events and spans.

- `fn new<S>(subscriber: S) -> Self`
  Returns a `Dispatch` that forwards to the given [`Subscriber`].

- `fn downgrade(self: &Self) -> WeakDispatch`
  Creates a [`WeakDispatch`] from this `Dispatch`.

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> subscriber::Interest`
  Registers a new callsite with this subscriber, returning whether or not

- `fn new_span(self: &Self, span: &span::Attributes<'_>) -> span::Id`
  Record the construction of a new span, returning a new [ID] for the

- `fn record(self: &Self, span: &span::Id, values: &span::Record<'_>)`
  Record a set of values on a span.

- `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)`
  Adds an indication that `span` follows from the span with the id

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool`
  Returns true if a span with the specified [metadata] would be

- `fn event(self: &Self, event: &Event<'_>)`
  Records that an [`Event`] has occurred.

- `fn enter(self: &Self, span: &span::Id)`
  Records that a span has been can_enter.

- `fn exit(self: &Self, span: &span::Id)`
  Records that a span has been exited.

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id`
  Notifies the subscriber that a [span ID] has been cloned.

- `fn drop_span(self: &Self, id: span::Id)`
  Notifies the subscriber that a [span ID] has been dropped.

- `fn try_close(self: &Self, id: span::Id) -> bool`
  Notifies the subscriber that a [span ID] has been dropped, and returns

- `fn current_span(self: &Self) -> span::Current`
  Returns a type representing this subscriber's view of the current span.

- `fn is<T: Any>(self: &Self) -> bool`
  Returns `true` if this `Dispatch` forwards to a `Subscriber` of type

- `fn downcast_ref<T: Any>(self: &Self) -> Option<&T>`
  Returns some reference to the `Subscriber` this `Dispatch` forwards to

#### Trait Implementations

##### `impl From<S>`

- `fn from(subscriber: S) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Dispatch`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Self`
  Returns the current default dispatcher

### `WeakDispatch`

```rust
struct WeakDispatch {
    // [REDACTED: Private Fields]
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
[`Dispatch`](#dispatch) rather than an [`Arc`](#arc).

[here]: Subscriber#avoiding-memory-leaks

#### Implementations

- `fn upgrade(self: &Self) -> Option<Dispatch>`
  Attempts to upgrade this `WeakDispatch` to a [`Dispatch`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> WeakDispatch`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DefaultGuard`

```rust
struct DefaultGuard();
```

A guard that resets the current default dispatcher to the prior
default dispatcher when dropped.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SetGlobalDefaultError`

```rust
struct SetGlobalDefaultError {
    // [REDACTED: Private Fields]
}
```

Returned if setting the global dispatcher fails.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `with_default`

```rust
fn with_default<T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T
```

Sets this dispatch as the default for the duration of a closure.

The default dispatcher is used when creating a new [span](#span) or
[`Event`](../event/index.md).

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This function required the Rust standard library.
    <code>no_std</code> users should use <a href="fn.set_global_default.html">
    <code>set_global_default</code></a> instead.
</pre>

[span](#span): super::span



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

[span](#span): super::span



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

[dispatcher](#dispatcher): super::dispatcher::Dispatch

