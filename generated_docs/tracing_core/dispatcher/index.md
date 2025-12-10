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
  - [`with_default`](#with_default)
  - [`set_default`](#set_default)
  - [`set_global_default`](#set_global_default)
  - [`get_default`](#get_default)
  - [`get_global`](#get_global)
- [Constants](#constants)
  - [`CURRENT_STATE`](#current_state)
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
| [`with_default`](#with_default) | fn | Sets this dispatch as the default for the duration of a closure. |
| [`set_default`](#set_default) | fn | Sets the dispatch as the default dispatch for the duration of the lifetime of the returned DefaultGuard |
| [`set_global_default`](#set_global_default) | fn | Sets this dispatch as the global default for the duration of the entire program. |
| [`get_default`](#get_default) | fn | Executes a closure with a reference to this thread's current [dispatcher]. |
| [`get_global`](#get_global) | fn |  |
| [`CURRENT_STATE`](#current_state) | const |  |
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

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:149-151`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L149-L151)*

`Dispatch` trace data to a [`Subscriber`](../subscriber/index.md).

#### Implementations

- <span id="dispatch-none"></span>`fn none() -> Self`

- <span id="dispatch-new"></span>`fn new<S>(subscriber: S) -> Self`

- <span id="dispatch-registrar"></span>`fn registrar(&self) -> Registrar` — [`Registrar`](#registrar)

- <span id="dispatch-downgrade"></span>`fn downgrade(&self) -> WeakDispatch` — [`WeakDispatch`](#weakdispatch)

- <span id="dispatch-subscriber"></span>`fn subscriber(&self) -> &dyn Subscriber + Send + Sync` — [`Subscriber`](../subscriber/index.md#subscriber)

- <span id="dispatch-register-callsite"></span>`fn register_callsite(&self, metadata: &'static Metadata<'static>) -> subscriber::Interest` — [`Metadata`](../metadata/index.md#metadata), [`Interest`](../subscriber/index.md#interest)

- <span id="dispatch-max-level-hint"></span>`fn max_level_hint(&self) -> Option<LevelFilter>` — [`LevelFilter`](../metadata/index.md#levelfilter)

- <span id="dispatch-new-span"></span>`fn new_span(&self, span: &span::Attributes<'_>) -> span::Id` — [`Attributes`](../span/index.md#attributes), [`Id`](../span/index.md#id)

- <span id="dispatch-record"></span>`fn record(&self, span: &span::Id, values: &span::Record<'_>)` — [`Id`](../span/index.md#id), [`Record`](../span/index.md#record)

- <span id="dispatch-record-follows-from"></span>`fn record_follows_from(&self, span: &span::Id, follows: &span::Id)` — [`Id`](../span/index.md#id)

- <span id="dispatch-enabled"></span>`fn enabled(&self, metadata: &Metadata<'_>) -> bool` — [`Metadata`](../metadata/index.md#metadata)

- <span id="dispatch-event"></span>`fn event(&self, event: &Event<'_>)` — [`Event`](../event/index.md#event)

- <span id="dispatch-enter"></span>`fn enter(&self, span: &span::Id)` — [`Id`](../span/index.md#id)

- <span id="dispatch-exit"></span>`fn exit(&self, span: &span::Id)` — [`Id`](../span/index.md#id)

- <span id="dispatch-clone-span"></span>`fn clone_span(&self, id: &span::Id) -> span::Id` — [`Id`](../span/index.md#id)

- <span id="dispatch-drop-span"></span>`fn drop_span(&self, id: span::Id)` — [`Id`](../span/index.md#id)

- <span id="dispatch-try-close"></span>`fn try_close(&self, id: span::Id) -> bool` — [`Id`](../span/index.md#id)

- <span id="dispatch-current-span"></span>`fn current_span(&self) -> span::Current` — [`Current`](../span/index.md#current)

- <span id="dispatch-is"></span>`fn is<T: Any>(&self) -> bool`

- <span id="dispatch-downcast-ref"></span>`fn downcast_ref<T: Any>(&self) -> Option<&T>`

#### Trait Implementations

##### `impl Clone for Dispatch`

- <span id="dispatch-clone"></span>`fn clone(&self) -> Dispatch` — [`Dispatch`](#dispatch)

##### `impl Debug for Dispatch`

- <span id="dispatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dispatch`

- <span id="dispatch-default"></span>`fn default() -> Self`

### `WeakDispatch`

```rust
struct WeakDispatch {
    subscriber: Kind<alloc::sync::Weak<dyn Subscriber + Send + Sync>>,
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:172-174`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L172-L174)*

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

#### Trait Implementations

##### `impl Clone for WeakDispatch`

- <span id="weakdispatch-clone"></span>`fn clone(&self) -> WeakDispatch` — [`WeakDispatch`](#weakdispatch)

##### `impl Debug for WeakDispatch`

- <span id="weakdispatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `State`

```rust
struct State {
    default: std::cell::RefCell<Option<Dispatch>>,
    can_enter: std::cell::Cell<bool>,
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:212-223`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L212-L223)*

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

- <span id="state-enter"></span>`fn enter(&self) -> Option<Entered<'_>>` — [`Entered`](#entered)

### `Entered<'a>`

```rust
struct Entered<'a>(&'a State);
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:229`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L229)*

While this guard is active, additional calls to subscriber functions on
the default dispatcher will not be able to access the dispatch context.
Dropping the guard will allow the dispatch context to be re-entered.

#### Implementations

- <span id="entered-current"></span>`fn current(&self) -> Ref<'a, Dispatch>` — [`Dispatch`](#dispatch)

#### Trait Implementations

##### `impl Drop for Entered<'_>`

- <span id="entered-drop"></span>`fn drop(&mut self)`

### `DefaultGuard`

```rust
struct DefaultGuard(Option<Dispatch>);
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:236`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L236)*

A guard that resets the current default dispatcher to the prior
default dispatcher when dropped.

#### Trait Implementations

##### `impl Debug for DefaultGuard`

- <span id="defaultguard-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for DefaultGuard`

- <span id="defaultguard-drop"></span>`fn drop(&mut self)`

### `SetGlobalDefaultError`

```rust
struct SetGlobalDefaultError {
    _no_construct: (),
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:345-347`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L345-L347)*

Returned if setting the global dispatcher fails.

#### Implementations

- <span id="setglobaldefaulterror-const-message"></span>`const MESSAGE: &'static str`

#### Trait Implementations

##### `impl Debug for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for SetGlobalDefaultError`

##### `impl ToString for SetGlobalDefaultError`

- <span id="setglobaldefaulterror-to-string"></span>`fn to_string(&self) -> String`

### `Registrar`

```rust
struct Registrar(Kind<alloc::sync::Weak<dyn Subscriber + Send + Sync>>);
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:458`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L458)*

#### Implementations

- <span id="registrar-upgrade"></span>`fn upgrade(&self) -> Option<Dispatch>` — [`Dispatch`](#dispatch)

## Enums

### `Kind<T>`

```rust
enum Kind<T> {
    Global(&'static dyn Subscriber + Send + Sync),
    Scoped(T),
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:177-180`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L177-L180)*

#### Implementations

- <span id="kind-downgrade"></span>`fn downgrade(&self) -> Kind<Weak<dyn Subscriber + Send + Sync>>` — [`Kind`](#kind), [`Subscriber`](../subscriber/index.md#subscriber)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Kind<T>`

- <span id="kind-clone"></span>`fn clone(&self) -> Kind<T>` — [`Kind`](#kind)

## Functions

### `with_default`

```rust
fn with_default<T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:254-261`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L254-L261)*

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

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:276-281`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L276-L281)*

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

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:299-332`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L299-L332)*

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

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:379-398`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L379-L398)*

Executes a closure with a reference to this thread's current [dispatcher](#dispatcher).

Note that calls to `get_default` should not be nested; if this function is
called while inside of another `get_default`, that closure will be provided
with `Dispatch::none` rather than the previously set dispatcher.


### `get_global`

```rust
fn get_global() -> &'static Dispatch
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:446-455`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L446-L455)*

## Constants

### `CURRENT_STATE`
```rust
const CURRENT_STATE: thread::LocalKey<State>;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:183-190`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L183-L190)*

### `UNINITIALIZED`
```rust
const UNINITIALIZED: usize = 0usize;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:198`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L198)*

### `INITIALIZING`
```rust
const INITIALIZING: usize = 1usize;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:199`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L199)*

### `INITIALIZED`
```rust
const INITIALIZED: usize = 2usize;
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:200`](../../../.source_1765210505/tracing-core-0.1.35/src/dispatcher.rs#L200)*

