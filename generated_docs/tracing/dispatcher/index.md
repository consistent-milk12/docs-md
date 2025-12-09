*[tracing](../index.md) / [dispatcher](index.md)*

---

# Module `dispatcher`

Dispatches trace events to [`Subscriber`](../../tracing_core/subscriber/index.md)s.

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
<strong>Note</strong>: The thread-local scoped dispatcher (<code>with_default</code>)
requires the Rust standard library. <code>no_std</code> users should
use <a href="fn.set_global_default.html"><code>set_global_default</code></a>
instead.
</pre>

## Accessing the Default Subscriber

A thread's current default subscriber can be accessed using the
[`get_default`](#get-default) function, which executes a closure with a reference to the
currently default `Dispatch`. This is used primarily by `tracing`
instrumentation.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`with_default`](#with_default) | mod |  |
| [`DefaultGuard`](#defaultguard) | trait |  |
| [`set_default`](#set_default) | type |  |

## Modules

- [`with_default`](with_default/index.md)

## Traits

### `DefaultGuard`

```rust
trait DefaultGuard: Automaton + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { ... }
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2643-2646`](../../../.source_1765210505/aho-corasick-1.1.4/src/ahocorasick.rs#L2643-L2646)*

A trait that effectively gives us practical dynamic dispatch over anything
that impls `Automaton`, but without needing to add a bunch of bounds to
the core `Automaton` trait. Basically, we provide all of the marker traits
that our automatons have, in addition to `Debug` impls and requiring that
there is no borrowed data. Without these, the main `AhoCorasick` type would
not be able to meaningfully impl `Debug` or the marker traits without also
requiring that all impls of `Automaton` do so, which would be not great.

## Type Aliases

*Defined in [`tracing-0.1.43/src/dispatcher.rs:128`](../../../.source_1765210505/tracing-0.1.43/src/dispatcher.rs#L128)*

