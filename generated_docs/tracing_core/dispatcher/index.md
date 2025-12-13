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

## Contents

- [Structs](#structs)
  - [`Dispatch`](#dispatch)
  - [`WeakDispatch`](#weakdispatch)
  - [`State`](#state)
  - [`Entered`](#entered)
  - [`DefaultGuard`](#defaultguard)
  - [`SetGlobalDefaultError`](#setglobaldefaulterror)
  - [`Registrar`](#registrar)
- [Enums](#enums)
  - [`Kind`](#kind)
- [Functions](#functions)
  - [`with_default`](#with-default)
  - [`set_default`](#set-default)
  - [`set_global_default`](#set-global-default)
  - [`get_default`](#get-default)
  - [`get_global`](#get-global)
- [Constants](#constants)
  - [`CURRENT_STATE`](#current-state)
  - [`UNINITIALIZED`](#uninitialized)
  - [`INITIALIZING`](#initializing)
  - [`INITIALIZED`](#initialized)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Dispatch`](#dispatch) | struct | `Dispatch` trace data to a [`Subscriber`]. |
| [`WeakDispatch`](#weakdispatch) | struct | `WeakDispatch` is a version of [`Dispatch`] that holds a non-owning reference to a [`Subscriber`]. |
| [`State`](#state) | struct | The dispatch state of a thread. |
| [`Entered`](#entered) | struct | While this guard is active, additional calls to subscriber functions on the default dispatcher will not be able to access the dispatch context. |
| [`DefaultGuard`](#defaultguard) | struct | A guard that resets the current default dispatcher to the prior default dispatcher when dropped. |
| [`SetGlobalDefaultError`](#setglobaldefaulterror) | struct | Returned if setting the global dispatcher fails. |
| [`Registrar`](#registrar) | struct |  |
| [`Kind`](#kind) | enum |  |
| [`with_default`](#with-default) | fn | Sets this dispatch as the default for the duration of a closure. |
| [`set_default`](#set-default) | fn | Sets the dispatch as the default dispatch for the duration of the lifetime of the returned DefaultGuard |
| [`set_global_default`](#set-global-default) | fn | Sets this dispatch as the global default for the duration of the entire program. |
| [`get_default`](#get-default) | fn | Executes a closure with a reference to this thread's current [dispatcher]. |
| [`get_global`](#get-global) | fn |  |
| [`CURRENT_STATE`](#current-state) | const |  |
| [`UNINITIALIZED`](#uninitialized) | const |  |
| [`INITIALIZING`](#initializing) | const |  |
| [`INITIALIZED`](#initialized) | const |  |

## Structs

### `Dispatch`

```rust
struct Dispatch {
    subscriber: Kind<alloc::sync::Arc<dyn Subscriber + Send + Sync>>,
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:149-151`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L149-L151)*

`Dispatch` trace data to a [`Subscriber`](../subscriber/index.md).

#### Implementations

- <span id="dispatch-none"></span>`fn none() -> Self`

  Returns a new `Dispatch` that discards events and spans.

- <span id="dispatch-new"></span>`fn new<S>(subscriber: S) -> Self`

  Returns a `Dispatch` that forwards to the given [`Subscriber`](../subscriber/index.md).

- <span id="dispatch-registrar"></span>`fn registrar(&self) -> Registrar` — [`Registrar`](#registrar)

- <span id="dispatch-downgrade"></span>`fn downgrade(&self) -> WeakDispatch` — [`WeakDispatch`](#weakdispatch)

  Creates a [`WeakDispatch`](#weakdispatch) from this `Dispatch`.

  

  A [`WeakDispatch`](#weakdispatch) is similar to a [`Dispatch`](#dispatch), but it does not prevent

  the underlying [`Subscriber`](../subscriber/index.md) from being dropped. Instead, it only permits

  access while other references to the `Subscriber` exist. This is equivalent

  to the standard library's `Arc::downgrade` method, but for `Dispatch`

  rather than `Arc`.

  

  The primary use for creating a [`WeakDispatch`](#weakdispatch) is to allow a `Subscriber`

  to hold a cyclical reference to itself without creating a memory leak.

  See [here] for details.

  

- <span id="dispatch-subscriber"></span>`fn subscriber(&self) -> &dyn Subscriber + Send + Sync` — [`Subscriber`](../subscriber/index.md#subscriber)

- <span id="dispatch-register-callsite"></span>`fn register_callsite(&self, metadata: &'static Metadata<'static>) -> subscriber::Interest` — [`Metadata`](../metadata/index.md#metadata), [`Interest`](../subscriber/index.md#interest)

  Registers a new callsite with this subscriber, returning whether or not

  the subscriber is interested in being notified about the callsite.

  

  This calls the `register_callsite` function on the [`Subscriber`](../subscriber/index.md)

  that this `Dispatch` forwards to.

  

- <span id="dispatch-max-level-hint"></span>`fn max_level_hint(&self) -> Option<LevelFilter>` — [`LevelFilter`](../metadata/index.md#levelfilter)

  Returns the highest [verbosity level][`level`](../../tracing_attributes/attr/kw/index.md) that this [`Subscriber`](../subscriber/index.md) will

  enable, or `None`, if the subscriber does not implement level-based

  filtering or chooses not to implement this method.

  

  This calls the `max_level_hint` function on the [`Subscriber`](../subscriber/index.md)

  that this `Dispatch` forwards to.

  

  

- <span id="dispatch-new-span"></span>`fn new_span(&self, span: &span::Attributes<'_>) -> span::Id` — [`Attributes`](../span/index.md#attributes), [`Id`](../span/index.md#id)

  Record the construction of a new span, returning a new [ID] for the

  span being constructed.

  

  This calls the `new_span` function on the [`Subscriber`](../subscriber/index.md) that this

  `Dispatch` forwards to.

  

  

- <span id="dispatch-record"></span>`fn record(&self, span: &span::Id, values: &span::Record<'_>)` — [`Id`](../span/index.md#id), [`Record`](../span/index.md#record)

  Record a set of values on a span.

  

  This calls the `record` function on the [`Subscriber`](../subscriber/index.md) that this

  `Dispatch` forwards to.

  

- <span id="dispatch-record-follows-from"></span>`fn record_follows_from(&self, span: &span::Id, follows: &span::Id)` — [`Id`](../span/index.md#id)

  Adds an indication that `span` follows from the span with the id

  `follows`.

  

  This calls the `record_follows_from` function on the [`Subscriber`](../subscriber/index.md)

  that this `Dispatch` forwards to.

  

- <span id="dispatch-enabled"></span>`fn enabled(&self, metadata: &Metadata<'_>) -> bool` — [`Metadata`](../metadata/index.md#metadata)

  Returns true if a span with the specified [`metadata`](../metadata/index.md) would be

  recorded.

  

  This calls the `enabled` function on the [`Subscriber`](../subscriber/index.md) that this

  `Dispatch` forwards to.

  

  

- <span id="dispatch-event"></span>`fn event(&self, event: &Event<'_>)` — [`Event`](../event/index.md#event)

  Records that an [`Event`](../event/index.md) has occurred.

  

  This calls the [`event`](../event/index.md) function on the [`Subscriber`](../subscriber/index.md) that this

  `Dispatch` forwards to.

  

  

- <span id="dispatch-enter"></span>`fn enter(&self, span: &span::Id)` — [`Id`](../span/index.md#id)

  Records that a span has been can_enter.

  

  This calls the `enter` function on the [`Subscriber`](../subscriber/index.md) that this

  `Dispatch` forwards to.

  

- <span id="dispatch-exit"></span>`fn exit(&self, span: &span::Id)` — [`Id`](../span/index.md#id)

  Records that a span has been exited.

  

  This calls the [`exit`](#exit) function on the [`Subscriber`](../subscriber/index.md) that this

  `Dispatch` forwards to.

  

- <span id="dispatch-clone-span"></span>`fn clone_span(&self, id: &span::Id) -> span::Id` — [`Id`](../span/index.md#id)

  Notifies the subscriber that a [span ID] has been cloned.

  

  This function must only be called with span IDs that were returned by

  this `Dispatch`'s `new_span` function. The `tracing` crate upholds

  this guarantee and any other libraries implementing instrumentation APIs

  must as well.

  

  This calls the `clone_span` function on the `Subscriber` that this

  `Dispatch` forwards to.

  

  

  

- <span id="dispatch-drop-span"></span>`fn drop_span(&self, id: span::Id)` — [`Id`](../span/index.md#id)

  Notifies the subscriber that a [span ID] has been dropped.

  

  This function must only be called with span IDs that were returned by

  this `Dispatch`'s `new_span` function. The `tracing` crate upholds

  this guarantee and any other libraries implementing instrumentation APIs

  must as well.

  

  This calls the `drop_span` function on the [`Subscriber`](../subscriber/index.md) that this

  `Dispatch` forwards to.

  

  <pre class="compile_fail" style="white-space:normal;font:inherit;">

      <strong>Deprecated</strong>: The <a href="#method.try_close"><code>

      try_close</code></a> method is functionally identical, but returns

      <code>true</code> if the span is now closed. It should be used

      instead of this method.

  </pre>

  

  

  

  

- <span id="dispatch-try-close"></span>`fn try_close(&self, id: span::Id) -> bool` — [`Id`](../span/index.md#id)

  Notifies the subscriber that a [span ID] has been dropped, and returns

  `true` if there are now 0 IDs referring to that span.

  

  This function must only be called with span IDs that were returned by

  this `Dispatch`'s `new_span` function. The `tracing` crate upholds

  this guarantee and any other libraries implementing instrumentation APIs

  must as well.

  

  This calls the `try_close` function on the [`Subscriber`](../subscriber/index.md) that this

   `Dispatch` forwards to.

  

  

  

- <span id="dispatch-current-span"></span>`fn current_span(&self) -> span::Current` — [`Current`](../span/index.md#current)

  Returns a type representing this subscriber's view of the current span.

  

  This calls the `current` function on the `Subscriber` that this

  `Dispatch` forwards to.

- <span id="dispatch-is"></span>`fn is<T: Any>(&self) -> bool`

  Returns `true` if this `Dispatch` forwards to a `Subscriber` of type

  `T`.

- <span id="dispatch-downcast-ref"></span>`fn downcast_ref<T: Any>(&self) -> Option<&T>`

  Returns some reference to the `Subscriber` this `Dispatch` forwards to

  if it is of type `T`, or `None` if it isn't.

#### Trait Implementations

##### `impl Any for Dispatch`

- <span id="dispatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dispatch`

- <span id="dispatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dispatch`

- <span id="dispatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Dispatch`

- <span id="dispatch-clone"></span>`fn clone(&self) -> Dispatch` — [`Dispatch`](#dispatch)

##### `impl CloneToUninit for Dispatch`

- <span id="dispatch-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Dispatch`

- <span id="dispatch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dispatch`

- <span id="dispatch-default"></span>`fn default() -> Self`

  Returns the current default dispatcher

##### `impl<T> From for Dispatch`

- <span id="dispatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Dispatch`

- <span id="dispatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Dispatch`

- <span id="dispatch-toowned-type-owned"></span>`type Owned = T`

- <span id="dispatch-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dispatch-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Dispatch`

- <span id="dispatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dispatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dispatch`

- <span id="dispatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dispatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WeakDispatch`

```rust
struct WeakDispatch {
    subscriber: Kind<alloc::sync::Weak<dyn Subscriber + Send + Sync>>,
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:172-174`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L172-L174)*

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

- <span id="weakdispatch-upgrade"></span>`fn upgrade(&self) -> Option<Dispatch>` — [`Dispatch`](#dispatch)

  Attempts to upgrade this `WeakDispatch` to a [`Dispatch`](#dispatch).

  

  Returns `None` if the referenced `Dispatch` has already been dropped.

  

  ## Examples

  

  ```rust

  use tracing_core::subscriber::NoSubscriber;

  use tracing_core::dispatcher::Dispatch;

  let strong = Dispatch::new(NoSubscriber::default());

  let weak = strong.downgrade();

  

  // The strong here keeps it alive, so we can still access the object.

  assert!(weak.upgrade().is_some());

  

  drop(strong); // But not any more.

  assert!(weak.upgrade().is_none());

  ```

#### Trait Implementations

##### `impl Any for WeakDispatch`

- <span id="weakdispatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WeakDispatch`

- <span id="weakdispatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WeakDispatch`

- <span id="weakdispatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WeakDispatch`

- <span id="weakdispatch-clone"></span>`fn clone(&self) -> WeakDispatch` — [`WeakDispatch`](#weakdispatch)

##### `impl CloneToUninit for WeakDispatch`

- <span id="weakdispatch-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WeakDispatch`

- <span id="weakdispatch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WeakDispatch`

- <span id="weakdispatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WeakDispatch`

- <span id="weakdispatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for WeakDispatch`

- <span id="weakdispatch-toowned-type-owned"></span>`type Owned = T`

- <span id="weakdispatch-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="weakdispatch-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WeakDispatch`

- <span id="weakdispatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="weakdispatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WeakDispatch`

- <span id="weakdispatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="weakdispatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `State`

```rust
struct State {
    default: std::cell::RefCell<Option<Dispatch>>,
    can_enter: std::cell::Cell<bool>,
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:212-223`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L212-L223)*

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

- <span id="state-set-default"></span>`fn set_default(new_dispatch: Dispatch) -> DefaultGuard` — [`Dispatch`](#dispatch), [`DefaultGuard`](#defaultguard)

  Replaces the current default dispatcher on this thread with the provided

  dispatcher.Any

  

  Dropping the returned `ResetGuard` will reset the default dispatcher to

  the previous value.

- <span id="state-enter"></span>`fn enter(&self) -> Option<Entered<'_>>` — [`Entered`](#entered)

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Entered<'a>`

```rust
struct Entered<'a>(&'a State);
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:229`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L229)*

While this guard is active, additional calls to subscriber functions on
the default dispatcher will not be able to access the dispatch context.
Dropping the guard will allow the dispatch context to be re-entered.

#### Implementations

- <span id="entered-current"></span>`fn current(&self) -> Ref<'a, Dispatch>` — [`Dispatch`](#dispatch)

#### Trait Implementations

##### `impl Any for Entered<'a>`

- <span id="entered-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entered<'a>`

- <span id="entered-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entered<'a>`

- <span id="entered-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for Entered<'_>`

- <span id="entered-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Entered<'a>`

- <span id="entered-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Entered<'a>`

- <span id="entered-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Entered<'a>`

- <span id="entered-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entered-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Entered<'a>`

- <span id="entered-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entered-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DefaultGuard`

```rust
struct DefaultGuard(Option<Dispatch>);
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:236`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L236)*

A guard that resets the current default dispatcher to the prior
default dispatcher when dropped.

#### Trait Implementations

##### `impl Any for DefaultGuard`

- <span id="defaultguard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DefaultGuard`

- <span id="defaultguard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DefaultGuard`

- <span id="defaultguard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DefaultGuard`

- <span id="defaultguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for DefaultGuard`

- <span id="defaultguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for DefaultGuard`

- <span id="defaultguard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DefaultGuard`

- <span id="defaultguard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DefaultGuard`

- <span id="defaultguard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="defaultguard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DefaultGuard`

- <span id="defaultguard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="defaultguard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SetGlobalDefaultError`

```rust
struct SetGlobalDefaultError {
    _no_construct: (),
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:345-347`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L345-L347)*

Returned if setting the global dispatcher fails.

#### Implementations

- <span id="setglobaldefaulterror-const-message"></span>`const MESSAGE: &'static str`

#### Trait Implementations

##### `impl Any for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for SetGlobalDefaultError`

##### `impl<T> From for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="setglobaldefaulterror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="setglobaldefaulterror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Registrar`

```rust
struct Registrar(Kind<alloc::sync::Weak<dyn Subscriber + Send + Sync>>);
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:458`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L458)*

#### Implementations

- <span id="registrar-upgrade"></span>`fn upgrade(&self) -> Option<Dispatch>` — [`Dispatch`](#dispatch)

#### Trait Implementations

##### `impl Any for Registrar`

- <span id="registrar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Registrar`

- <span id="registrar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Registrar`

- <span id="registrar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Registrar`

- <span id="registrar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Registrar`

- <span id="registrar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Registrar`

- <span id="registrar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="registrar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Registrar`

- <span id="registrar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="registrar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Kind<T>`

```rust
enum Kind<T> {
    Global(&'static dyn Subscriber + Send + Sync),
    Scoped(T),
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:177-180`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L177-L180)*

#### Implementations

- <span id="kind-downgrade"></span>`fn downgrade(&self) -> Kind<Weak<dyn Subscriber + Send + Sync>>` — [`Kind`](#kind), [`Subscriber`](../subscriber/index.md#subscriber)

#### Trait Implementations

##### `impl<T> Any for Kind<T>`

- <span id="kind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Kind<T>`

- <span id="kind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Kind<T>`

- <span id="kind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Kind<T>`

- <span id="kind-clone"></span>`fn clone(&self) -> Kind<T>` — [`Kind`](#kind)

##### `impl<T> CloneToUninit for Kind<T>`

- <span id="kind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Kind<T>`

- <span id="kind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Kind<T>`

- <span id="kind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToOwned for Kind<T>`

- <span id="kind-toowned-type-owned"></span>`type Owned = T`

- <span id="kind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="kind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Kind<T>`

- <span id="kind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Kind<T>`

- <span id="kind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `with_default`

```rust
fn with_default<T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:254-261`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L254-L261)*

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

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:276-281`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L276-L281)*

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

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:299-332`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L299-L332)*

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

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:379-398`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L379-L398)*

Executes a closure with a reference to this thread's current [dispatcher](#dispatcher).

Note that calls to `get_default` should not be nested; if this function is
called while inside of another `get_default`, that closure will be provided
with `Dispatch::none` rather than the previously set dispatcher.


### `get_global`

```rust
fn get_global() -> &'static Dispatch
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:446-455`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L446-L455)*

## Constants

### `CURRENT_STATE`
```rust
const CURRENT_STATE: thread::LocalKey<State>;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:183-190`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L183-L190)*

### `UNINITIALIZED`
```rust
const UNINITIALIZED: usize = 0usize;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:198`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L198)*

### `INITIALIZING`
```rust
const INITIALIZING: usize = 1usize;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:199`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L199)*

### `INITIALIZED`
```rust
const INITIALIZED: usize = 2usize;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:200`](../../../.source_1765633015/tracing-core-0.1.35/src/dispatcher.rs#L200)*

