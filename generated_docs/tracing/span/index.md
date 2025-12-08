*[tracing](../index.md) / [span](index.md)*

---

# Module `span`

 Spans represent periods of time in which a program was executing in a
 particular context.

 A span consists of [`fields`](../../tracing_attributes/attr/kw/index.md), user-defined key-value pairs of arbitrary data
 that describe the context the span represents, and a set of fixed attributes
 that describe all `tracing` spans and events. Attributes describing spans
 include:

 - An [`Id`](#id) assigned by the subscriber that uniquely identifies it in relation
   to other spans.
 - The span's [`parent`](../../tracing_core/parent/index.md) in the trace tree.
 - [Metadata] that describes static characteristics of all spans
   originating from that callsite, such as its name, source code location,
   [verbosity level], and the names of its fields.

 # Creating Spans

 Spans are created using the `span!` macro. This macro is invoked with the
 following arguments, in order:

 - The `target` and/or [`parent`][`parent`](../../tracing_core/parent/index.md) attributes, if the user wishes to
   override their default values.
 - The span's [verbosity level]
 - A string literal providing the span's name.
 - Finally, zero or more arbitrary key/value fields.

 For example:
 ```rust
 use tracing::{span, Level};

 /// Construct a new span at the `INFO` level named "my_span", with a single
 /// field named answer , with the value `42`.
 let my_span = span!(Level::INFO, "my_span", answer = 42);
 ```

 The documentation for the `span!` macro provides additional examples of
 the various options that exist when creating spans.

 The `trace_span!`, `debug_span!`, `info_span!`, `warn_span!`, and
 `error_span!` exist as shorthand for constructing spans at various
 verbosity levels.

 ## Recording Span Creation

 The [`Attributes`](#attributes) type contains data associated with a span, and is
 provided to the [`Subscriber`](../../tracing_core/index.md) when a new span is created. It contains
 the span's metadata, the ID of [the span's parent][`parent`](../../tracing_core/parent/index.md) if one was
 explicitly set, and any fields whose values were recorded when the span was
 constructed. The subscriber, which is responsible for recording `tracing`
 data, can then store or record these values.

 # The Span Lifecycle

 ## Entering a Span

 A thread of execution is said to _enter_ a span when it begins executing,
 and _exit_ the span when it switches to another context. Spans may be
 entered through the `enter`, `entered`, and `in_scope` methods.

 The `enter` method enters a span, returning a [`guard`](../../crossbeam_epoch/guard/index.md) that exits the span
 when dropped
 ```rust
 use tracing::{span, Level};
 let my_var: u64 = 5;
 let my_span = span!(Level::TRACE, "my_span", my_var);

 // `my_span` exists but has not been entered.

 // Enter `my_span`...
 let _enter = my_span.enter();

 // Perform some work inside of the context of `my_span`...
 // Dropping the `_enter` guard will exit the span.
```

 <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
     <strong>Warning</strong>: In asynchronous code that uses async/await syntax,
     <code>Span::enter</code> may produce incorrect traces if the returned drop
     guard is held across an await point. See
     <a href="struct.Span.html#in-asynchronous-code">the method documentation</a>
     for details.
 </pre></div>

 The `entered` method is analogous to `enter`, but moves the span into
 the returned guard, rather than borrowing it. This allows creating and
 entering a span in a single expression:

 ```rust
 use tracing::{span, Level};
 // Create a span and enter it, returning a guard:
 let span = span!(Level::INFO, "my_span").entered();

 // We are now inside the span! Like `enter()`, the guard returned by
 // `entered()` will exit the span when it is dropped...

 // ...but, it can also be exited explicitly, returning the `Span`
 // struct:
 let span = span.exit();
 ```

 Finally, `in_scope` takes a closure or function pointer and executes it
 inside the span:

 ```rust
 use tracing::{span, Level};
 let my_var: u64 = 5;
 let my_span = span!(Level::TRACE, "my_span", my_var = &my_var);

 my_span.in_scope(|| {
     // perform some work in the context of `my_span`...
 });

 // Perform some work outside of the context of `my_span`...

 my_span.in_scope(|| {
     // Perform some more work in the context of `my_span`.
 });
 ```

 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: Since entering a span takes <code>&self</code>, and
     <code>Span</code>s are <code>Clone</code>, <code>Send</code>, and
     <code>Sync</code>, it is entirely valid for multiple threads to enter the
     same span concurrently.
 </pre>

 ## Span Relationships

 Spans form a tree structure — unless it is a root span, all spans have a
 _parent_, and may have one or more _children_. When a new span is created,
 the current span becomes the new span's parent. The total execution time of
 a span consists of the time spent in that span and in the entire subtree
 represented by its children. Thus, a parent span always lasts for at least
 as long as the longest-executing span in its subtree.

 ```rust
 use tracing::{Level, span};
 // this span is considered the "root" of a new trace tree:
 span!(Level::INFO, "root").in_scope(|| {
     // since we are now inside "root", this span is considered a child
     // of "root":
     span!(Level::DEBUG, "outer_child").in_scope(|| {
         // this span is a child of "outer_child", which is in turn a
         // child of "root":
         span!(Level::TRACE, "inner_child").in_scope(|| {
             // and so on...
         });
     });
     // another span created here would also be a child of "root".
 });
```

 In addition, the parent of a span may be explicitly specified in
 the `span!` macro. For example:

 ```rust
 use tracing::{Level, span};
 // Create, but do not enter, a span called "foo".
 let foo = span!(Level::INFO, "foo");

 // Create and enter a span called "bar".
 let bar = span!(Level::INFO, "bar");
 let _enter = bar.enter();

 // Although we have currently entered "bar", "baz"'s parent span
 // will be "foo".
 let baz = span!(parent: &foo, Level::INFO, "baz");
 ```

 A child span should typically be considered _part_ of its parent. For
 example, if a subscriber is recording the length of time spent in various
 spans, it should generally include the time spent in a span's children as
 part of that span's duration.

 In addition to having zero or one parent, a span may also _follow from_ any
 number of other spans. This indicates a causal relationship between the span
 and the spans that it follows from, but a follower is *not* typically
 considered part of the duration of the span it follows. Unlike the parent, a
 span may record that it follows from another span after it is created, using
 the `follows_from` method.

 As an example, consider a listener task in a server. As the listener accepts
 incoming connections, it spawns new tasks that handle those connections. We
 might want to have a span representing the listener, and instrument each
 spawned handler task with its own span. We would want our instrumentation to
 record that the handler tasks were spawned as a result of the listener task.
 However, we might not consider the handler tasks to be _part_ of the time
 spent in the listener task, so we would not consider those spans children of
 the listener span. Instead, we would record that the handler tasks follow
 from the listener, recording the causal relationship but treating the spans
 as separate durations.

 ## Closing Spans

 Execution may enter and exit a span multiple times before that span is
 _closed_. Consider, for example, a future which has an associated
 span and enters that span every time it is polled:
 ```rust
 use std::future::Future;
 use std::task::{Context, Poll};
 use std::pin::Pin;
 struct MyFuture {
    // data
    span: tracing::Span,
 }

 impl Future for MyFuture {
     type Output = ();

     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
         let _enter = self.span.enter();
         // Do actual future work...
 Poll::Ready(())
     }
 }
 ```

 If this future was spawned on an executor, it might yield one or more times
 before `poll` returns `Poll::Ready`. If the future were to yield, then
 the executor would move on to poll the next future, which may _also_ enter
 an associated span or series of spans. Therefore, it is valid for a span to
 be entered repeatedly before it completes. Only the time when that span or
 one of its children was the current span is considered to be time spent in
 that span. A span which is not executing and has not yet been closed is said
 to be _idle_.

 Because spans may be entered and exited multiple times before they close,
 [`Subscriber`](../../tracing_core/index.md)s have separate trait methods which are called to notify them
 of span exits and when span handles are dropped. When execution exits a
 span, [`exit`](#exit) will always be called with that span's ID to notify the
 subscriber that the span has been exited. When span handles are dropped, the
 `drop_span` method is called with that span's ID. The subscriber may use
 this to determine whether or not the span will be entered again.

 If there is only a single handle with the capacity to exit a span, dropping
 that handle "closes" the span, since the capacity to enter it no longer
 exists. For example:
 ```rust
 use tracing::{Level, span};
 {
     span!(Level::TRACE, "my_span").in_scope(|| {
         // perform some work in the context of `my_span`...
     }); // --> Subscriber::exit(my_span)

     // The handle to `my_span` only lives inside of this block; when it is
     // dropped, the subscriber will be informed via `drop_span`.

 } // --> Subscriber::drop_span(my_span)
 ```

 However, if multiple handles exist, the span can still be re-entered even if
 one or more is dropped. For determining when _all_ handles to a span have
 been dropped, `Subscriber`s have a `clone_span` method, which is called
 every time a span handle is cloned. Combined with `drop_span`, this may be
 used to track the number of handles to a given span — if `drop_span` has
 been called one more time than the number of calls to `clone_span` for a
 given ID, then no more handles to the span with that ID exist. The
 subscriber may then treat it as closed.

 # When to use spans

 As a rule of thumb, spans should be used to represent discrete units of work
 (e.g., a given request's lifetime in a server) or periods of time spent in a
 given context (e.g., time spent interacting with an instance of an external
 system, such as a database).

 Which scopes in a program correspond to new spans depend somewhat on user
 intent. For example, consider the case of a loop in a program. Should we
 construct one span and perform the entire loop inside of that span, like:

 ```rust
 use tracing::{Level, span};
 let n = 1;
 let span = span!(Level::TRACE, "my_loop");
 let _enter = span.enter();
 for i in 0..n {
     let _ = i;
     // ...
 }
 ```
 Or, should we create a new span for each iteration of the loop, as in:
 ```rust
 use tracing::{Level, span};
 let n = 1u64;
 for i in 0..n {
     let span = span!(Level::TRACE, "my_loop", iteration = i);
     let _enter = span.enter();
     // ...
 }
 ```

 Depending on the circumstances, we might want to do either, or both. For
 example, if we want to know how long was spent in the loop overall, we would
 create a single span around the entire loop; whereas if we wanted to know how
 much time was spent in each individual iteration, we would enter a new span
 on every iteration.





















## Structs

### `Id<R: gimli::Reader>`

```rust
struct Id<R: gimli::Reader> {
    sections: alloc::sync::Arc<gimli::Dwarf<R>>,
    units: crate::unit::ResUnits<R>,
    sup_units: crate::unit::SupUnits<R>,
}
```

*Re-exported from `addr2line`*

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

#### Implementations

- `fn find_unit(self: &Self, offset: gimli::DebugInfoOffset<<R as >::Offset>, file: DebugFile) -> Result<(&gimli::Unit<R>, gimli::UnitOffset<<R as >::Offset>), gimli::Error>` — [`Id`](#id), [`Record`](#record)

- `fn find_dwarf_and_unit(self: &Self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Option<gimli::UnitRef<'_, R>>, Buf = R>>` — [`DefaultGuard`](../subscriber/index.md)

- `fn find_location(self: &Self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>` — [`Record`](#record), [`DefaultGuard`](../subscriber/index.md), [`set_global_default`](../dispatcher/index.md)

- `fn find_location_range(self: &Self, probe_low: u64, probe_high: u64) -> Result<LocationRangeIter<'_, R>, gimli::Error>` — [`Record`](#record)

- `fn find_frames(self: &Self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Result<FrameIter<'_, R>, gimli::Error>, Buf = R>>` — [`Record`](#record)

- `fn preload_units(self: &Self, probe: u64) -> impl Iterator<Item = (SplitDwarfLoad<R>, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(), gimli::Error> + '_)>` — [`DefaultGuard`](../subscriber/index.md), [`DefaultGuard`](../subscriber/index.md), [`Record`](#record)

- `fn from_sections(debug_abbrev: gimli::DebugAbbrev<R>, debug_addr: gimli::DebugAddr<R>, debug_aranges: gimli::DebugAranges<R>, debug_info: gimli::DebugInfo<R>, debug_line: gimli::DebugLine<R>, debug_line_str: gimli::DebugLineStr<R>, debug_ranges: gimli::DebugRanges<R>, debug_rnglists: gimli::DebugRngLists<R>, debug_str: gimli::DebugStr<R>, debug_str_offsets: gimli::DebugStrOffsets<R>, default_section: R) -> Result<Self, gimli::Error>` — [`Record`](#record)

- `fn from_dwarf(sections: gimli::Dwarf<R>) -> Result<Context<R>, gimli::Error>` — [`Record`](#record), [`Id`](#id)

- `fn from_arc_dwarf(sections: Arc<gimli::Dwarf<R>>) -> Result<Context<R>, gimli::Error>` — [`Record`](#record), [`Id`](#id)

### `Span`

```rust
struct Span {
    inner: Option<Inner>,
    meta: Option<&'static crate::Metadata<'static>>,
}
```

A handle representing a span, with the capability to enter the span if it
exists.

If the span was rejected by the current `Subscriber`'s filter, entering the
span will silently do nothing. Thus, the handle can be used in the same
manner regardless of whether or not the trace is currently being collected.

#### Fields

- **`inner`**: `Option<Inner>`

  A handle used to enter the span when it is not executing.
  
  If this is `None`, then the span has either closed or was never enabled.

- **`meta`**: `Option<&'static crate::Metadata<'static>>`

  Metadata describing the span.
  
  This might be `Some` even if `inner` is `None`, in the case that the
  span is disabled but the metadata is needed for `log` support.

#### Implementations

- `fn new(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Metadata`](../index.md), [`Span`](../index.md)

- `fn new_root(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Metadata`](../index.md), [`Span`](../index.md)

- `fn child_of(parent: impl Into<Option<Id>>, meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Id`](#id), [`Metadata`](../index.md), [`Span`](../index.md)

- `fn new_disabled(meta: &'static Metadata<'static>) -> Span` — [`Metadata`](../index.md), [`Span`](../index.md)

- `const fn none() -> Span` — [`Span`](../index.md)

- `fn current() -> Span` — [`Span`](../index.md)

- `fn make_with(meta: &'static Metadata<'static>, new_span: Attributes<'_>, dispatch: &Dispatch) -> Span` — [`Metadata`](../index.md), [`Attributes`](#attributes), [`Dispatch`](../index.md), [`Span`](../index.md)

- `fn enter(self: &Self) -> Entered<'_>` — [`Entered`](#entered)

- `fn entered(self: Self) -> EnteredSpan` — [`EnteredSpan`](#enteredspan)

- `fn or_current(self: Self) -> Self`

- `fn do_enter(self: &Self)`

- `fn do_exit(self: &Self)`

- `fn in_scope<F: FnOnce() -> T, T>(self: &Self, f: F) -> T`

- `fn field<Q: field::AsField + ?Sized>(self: &Self, field: &Q) -> Option<field::Field>`

- `fn has_field<Q: field::AsField + ?Sized>(self: &Self, field: &Q) -> bool`

- `fn record<Q: field::AsField + ?Sized, V: field::Value>(self: &Self, field: &Q, value: V) -> &Self`

- `fn is_disabled(self: &Self) -> bool`

- `fn is_none(self: &Self) -> bool`

- `fn follows_from(self: &Self, from: impl Into<Option<Id>>) -> &Self` — [`Id`](#id)

- `fn id(self: &Self) -> Option<Id>` — [`Id`](#id)

- `fn metadata(self: &Self) -> Option<&'static Metadata<'static>>` — [`Metadata`](../index.md)

- `fn with_subscriber<T>(self: &Self, f: impl FnOnce((&Id, &Dispatch)) -> T) -> Option<T>` — [`Id`](#id), [`Dispatch`](../index.md)

#### Trait Implementations

##### `impl Clone for Span`

- `fn clone(self: &Self) -> Span` — [`Span`](../index.md)

##### `impl Debug for Span`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Span`

- `fn drop(self: &mut Self)`

##### `impl Hash for Span`

- `fn hash<H: Hasher>(self: &Self, hasher: &mut H)`

##### `impl<T> Instrument for Span`

##### `impl PartialEq for Span`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> WithSubscriber for Span`

### `Inner`

```rust
struct Inner {
    id: Id,
    subscriber: crate::dispatcher::Dispatch,
}
```

A handle representing the capacity to enter a span which is known to exist.

Unlike `Span`, this type is only constructed for spans which _have_ been
enabled by the current filter. This type is primarily used for implementing
span handles; users should typically not need to interact with it directly.

#### Fields

- **`id`**: `Id`

  The span's ID, as provided by `subscriber`.

- **`subscriber`**: `crate::dispatcher::Dispatch`

  The subscriber that will receive events relating to this span.
  
  This should be the same subscriber that provided this span with its
  `id`.

#### Implementations

- `fn follows_from(self: &Self, from: &Id)` — [`Id`](#id)

- `fn id(self: &Self) -> Id` — [`Id`](#id)

- `fn record(self: &Self, values: &Record<'_>)` — [`Record`](#record)

- `fn new(id: Id, subscriber: &Dispatch) -> Self` — [`Id`](#id), [`Dispatch`](../index.md)

#### Trait Implementations

##### `impl Clone for Inner`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for Inner`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Hash for Inner`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<T> Instrument for Inner`

##### `impl PartialEq for Inner`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> WithSubscriber for Inner`

### `Entered<'a>`

```rust
struct Entered<'a> {
    span: &'a Span,
}
```

A guard representing a span which has been entered and is currently
executing.

When the guard is dropped, the span will be exited.

This is returned by the `Span::enter` function.


#### Trait Implementations

##### `impl<'a> Debug for Entered<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Drop for Entered<'_>`

- `fn drop(self: &mut Self)`

##### `impl<T> Instrument for Entered<'a>`

##### `impl<T> WithSubscriber for Entered<'a>`

### `EnteredSpan`

```rust
struct EnteredSpan {
    span: Span,
    _not_send: PhantomNotSend,
}
```

An owned version of [`Entered`](#entered), a guard representing a span which has been
entered and is currently executing.

When the guard is dropped, the span will be exited.

This is returned by the `Span::entered` function.


#### Fields

- **`_not_send`**: `PhantomNotSend`

  ```compile_fail
  use tracing::span::*;
  trait AssertSend: Send {}
  
  impl AssertSend for EnteredSpan {}
  ```

#### Implementations

- `fn id(self: &Self) -> Option<Id>` — [`Id`](#id)

- `fn exit(self: Self) -> Span` — [`Span`](../index.md)

#### Trait Implementations

##### `impl Debug for EnteredSpan`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deref for EnteredSpan`

- `type Target = Span`

- `fn deref(self: &Self) -> &Span` — [`Span`](../index.md)

##### `impl Drop for EnteredSpan`

- `fn drop(self: &mut Self)`

##### `impl<T> Instrument for EnteredSpan`

##### `impl<P, T> Receiver for EnteredSpan`

- `type Target = T`

##### `impl<T> WithSubscriber for EnteredSpan`

### `PhantomNotSend`

```rust
struct PhantomNotSend {
    ghost: core::marker::PhantomData<*mut ()>,
}
```

Technically, `EnteredSpan` _can_ implement both `Send` *and*
`Sync` safely. It doesn't, because it has a `PhantomNotSend` field,
specifically added in order to make it `!Send`.

Sending an `EnteredSpan` guard between threads cannot cause memory unsafety.
However, it *would* result in incorrect behavior, so we add a
`PhantomNotSend` to prevent it from being sent between threads. This is
because it must be *dropped* on the same thread that it was created;
otherwise, the span will never be exited on the thread where it was entered,
and it will attempt to exit the span on a thread that may never have entered
it. However, we still want them to be `Sync` so that a struct holding an
`Entered` guard can be `Sync`.

Thus, this is totally safe.

#### Trait Implementations

##### `impl Debug for PhantomNotSend`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for PhantomNotSend`

##### `impl Sync for PhantomNotSend`

##### `impl<T> WithSubscriber for PhantomNotSend`

## Traits

### `AsId`

```rust
trait AsId: crate::sealed::Sealed { ... }
```

Trait implemented by types which have a span `Id`.

#### Required Methods

- `fn as_id(self: &Self) -> Option<&Id>`

  Returns the `Id` of the span that `self` corresponds to, or `None` if

## Functions

## Constants

### `PhantomNotSend`

```rust
const PhantomNotSend: PhantomNotSend;
```

