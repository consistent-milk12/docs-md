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
 provided to the [`Subscriber`](../../tracing_core/subscriber/index.md) when a new span is created. It contains
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
 [`Subscriber`](../../tracing_core/subscriber/index.md)s have separate trait methods which are called to notify them
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





















## Contents

- [Structs](#structs)
  - [`Id`](#id)
  - [`Span`](#span)
  - [`Inner`](#inner)
  - [`Entered`](#entered)
  - [`EnteredSpan`](#enteredspan)
  - [`PhantomNotSend`](#phantomnotsend)
- [Traits](#traits)
  - [`AsId`](#asid)
- [Functions](#functions)
  - [`Attributes`](#attributes)
- [Constants](#constants)
  - [`PhantomNotSend`](#phantomnotsend)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Id`](#id) | struct |  |
| [`Span`](#span) | struct | A handle representing a span, with the capability to enter the span if it exists. |
| [`Inner`](#inner) | struct | A handle representing the capacity to enter a span which is known to exist. |
| [`Entered`](#entered) | struct | A guard representing a span which has been entered and is currently executing. |
| [`EnteredSpan`](#enteredspan) | struct | An owned version of [`Entered`], a guard representing a span which has been entered and is currently executing. |
| [`PhantomNotSend`](#phantomnotsend) | struct | Technically, `EnteredSpan` _can_ implement both `Send` *and* `Sync` safely. |
| [`AsId`](#asid) | trait | Trait implemented by types which have a span `Id`. |
| [`Attributes`](#attributes) | fn |  |
| [`PhantomNotSend`](#phantomnotsend) | const |  |

## Structs

### `Id<R: gimli::Reader>`

```rust
struct Id<R: gimli::Reader> {
    sections: alloc::sync::Arc<gimli::Dwarf<R>>,
    units: crate::unit::ResUnits<R>,
    sup_units: crate::unit::SupUnits<R>,
}
```

*Defined in [`addr2line-0.25.1/src/lib.rs:95-99`](../../../.source_1765210505/addr2line-0.25.1/src/lib.rs#L95-L99)*

*Re-exported from `addr2line`*

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

#### Implementations

- <span id="context-from-sections"></span>`fn from_sections(debug_abbrev: gimli::DebugAbbrev<R>, debug_addr: gimli::DebugAddr<R>, debug_aranges: gimli::DebugAranges<R>, debug_info: gimli::DebugInfo<R>, debug_line: gimli::DebugLine<R>, debug_line_str: gimli::DebugLineStr<R>, debug_ranges: gimli::DebugRanges<R>, debug_rnglists: gimli::DebugRngLists<R>, debug_str: gimli::DebugStr<R>, debug_str_offsets: gimli::DebugStrOffsets<R>, default_section: R) -> Result<Self, gimli::Error>` — [`Record`](#record)

- <span id="context-from-dwarf"></span>`fn from_dwarf(sections: gimli::Dwarf<R>) -> Result<Context<R>, gimli::Error>` — [`Record`](#record), [`Id`](#id)

- <span id="context-from-arc-dwarf"></span>`fn from_arc_dwarf(sections: Arc<gimli::Dwarf<R>>) -> Result<Context<R>, gimli::Error>` — [`Record`](#record), [`Id`](#id)

- <span id="context-find-unit"></span>`fn find_unit(&self, offset: gimli::DebugInfoOffset<<R as >::Offset>, file: DebugFile) -> Result<(&gimli::Unit<R>, gimli::UnitOffset<<R as >::Offset>), gimli::Error>` — [`Id`](#id), [`Record`](#record)

- <span id="context-find-dwarf-and-unit"></span>`fn find_dwarf_and_unit(&self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Option<gimli::UnitRef<'_, R>>, Buf = R>>` — [`DefaultGuard`](../dispatcher/index.md#defaultguard)

- <span id="context-find-location"></span>`fn find_location(&self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>` — [`Record`](#record), [`DefaultGuard`](../dispatcher/index.md#defaultguard), [`set_global_default`](../dispatcher/index.md#set-global-default)

- <span id="context-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64) -> Result<LocationRangeIter<'_, R>, gimli::Error>` — [`Record`](#record)

- <span id="context-find-frames"></span>`fn find_frames(&self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Result<FrameIter<'_, R>, gimli::Error>, Buf = R>>` — [`Record`](#record)

- <span id="context-preload-units"></span>`fn preload_units(&self, probe: u64) -> impl Iterator<Item = (SplitDwarfLoad<R>, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(), gimli::Error> + '_)>` — [`DefaultGuard`](../subscriber/index.md#defaultguard), [`DefaultGuard`](../dispatcher/index.md#defaultguard), [`Record`](#record)

### `Span`

```rust
struct Span {
    inner: Option<Inner>,
    meta: Option<&'static crate::Metadata<'static>>,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:349-359`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L349-L359)*

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

- <span id="span-new"></span>`fn new(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Metadata`](../index.md#metadata), [`Span`](#span)

- <span id="span-new-root"></span>`fn new_root(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Metadata`](../index.md#metadata), [`Span`](#span)

- <span id="span-child-of"></span>`fn child_of(parent: impl Into<Option<Id>>, meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Id`](#id), [`Metadata`](../index.md#metadata), [`Span`](#span)

- <span id="span-new-disabled"></span>`fn new_disabled(meta: &'static Metadata<'static>) -> Span` — [`Metadata`](../index.md#metadata), [`Span`](#span)

- <span id="span-none"></span>`const fn none() -> Span` — [`Span`](#span)

- <span id="span-current"></span>`fn current() -> Span` — [`Span`](#span)

- <span id="span-make-with"></span>`fn make_with(meta: &'static Metadata<'static>, new_span: Attributes<'_>, dispatch: &Dispatch) -> Span` — [`Metadata`](../index.md#metadata), [`Attributes`](#attributes), [`Dispatch`](../dispatcher/index.md#dispatch), [`Span`](#span)

- <span id="span-enter"></span>`fn enter(&self) -> Entered<'_>` — [`Entered`](#entered)

- <span id="span-entered"></span>`fn entered(self) -> EnteredSpan` — [`EnteredSpan`](#enteredspan)

- <span id="span-or-current"></span>`fn or_current(self) -> Self`

- <span id="span-do-enter"></span>`fn do_enter(&self)`

- <span id="span-do-exit"></span>`fn do_exit(&self)`

- <span id="span-in-scope"></span>`fn in_scope<F: FnOnce() -> T, T>(&self, f: F) -> T`

- <span id="span-field"></span>`fn field<Q: field::AsField + ?Sized>(&self, field: &Q) -> Option<field::Field>`

- <span id="span-has-field"></span>`fn has_field<Q: field::AsField + ?Sized>(&self, field: &Q) -> bool`

- <span id="span-record"></span>`fn record<Q: field::AsField + ?Sized, V: field::Value>(&self, field: &Q, value: V) -> &Self`

- <span id="span-is-disabled"></span>`fn is_disabled(&self) -> bool`

- <span id="span-is-none"></span>`fn is_none(&self) -> bool`

- <span id="span-follows-from"></span>`fn follows_from(&self, from: impl Into<Option<Id>>) -> &Self` — [`Id`](#id)

- <span id="span-id"></span>`fn id(&self) -> Option<Id>` — [`Id`](#id)

- <span id="span-metadata"></span>`fn metadata(&self) -> Option<&'static Metadata<'static>>` — [`Metadata`](../index.md#metadata)

- <span id="span-with-subscriber"></span>`fn with_subscriber<T>(&self, f: impl FnOnce((&Id, &Dispatch)) -> T) -> Option<T>` — [`Id`](#id), [`Dispatch`](../dispatcher/index.md#dispatch)

#### Trait Implementations

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl Debug for Span`

- <span id="span-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Span`

- <span id="span-drop"></span>`fn drop(&mut self)`

##### `impl Hash for Span`

- <span id="span-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl Instrument for Span`

##### `impl PartialEq for Span`

- <span id="span-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl WithSubscriber for Span`

### `Inner`

```rust
struct Inner {
    id: Id,
    subscriber: crate::dispatcher::Dispatch,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:367-376`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L367-L376)*

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

- <span id="inner-follows-from"></span>`fn follows_from(&self, from: &Id)` — [`Id`](#id)

- <span id="inner-id"></span>`fn id(&self) -> Id` — [`Id`](#id)

- <span id="inner-record"></span>`fn record(&self, values: &Record<'_>)` — [`Record`](#record)

- <span id="inner-new"></span>`fn new(id: Id, subscriber: &Dispatch) -> Self` — [`Id`](#id), [`Dispatch`](../dispatcher/index.md#dispatch)

#### Trait Implementations

##### `impl Clone for Inner`

- <span id="inner-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Inner`

- <span id="inner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Hash for Inner`

- <span id="inner-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl Instrument for Inner`

##### `impl PartialEq for Inner`

- <span id="inner-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl WithSubscriber for Inner`

### `Entered<'a>`

```rust
struct Entered<'a> {
    span: &'a Span,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:388-390`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L388-L390)*

A guard representing a span which has been entered and is currently
executing.

When the guard is dropped, the span will be exited.

This is returned by the `Span::enter` function.


#### Trait Implementations

##### `impl Debug for Entered<'a>`

- <span id="entered-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Entered<'_>`

- <span id="entered-drop"></span>`fn drop(&mut self)`

##### `impl Instrument for Entered<'a>`

##### `impl WithSubscriber for Entered<'a>`

### `EnteredSpan`

```rust
struct EnteredSpan {
    span: Span,
    _not_send: PhantomNotSend,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:402-412`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L402-L412)*

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

- <span id="enteredspan-id"></span>`fn id(&self) -> Option<Id>` — [`Id`](#id)

- <span id="enteredspan-exit"></span>`fn exit(self) -> Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Debug for EnteredSpan`

- <span id="enteredspan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for EnteredSpan`

- <span id="enteredspan-type-target"></span>`type Target = Span`

- <span id="enteredspan-deref"></span>`fn deref(&self) -> &Span` — [`Span`](#span)

##### `impl Drop for EnteredSpan`

- <span id="enteredspan-drop"></span>`fn drop(&mut self)`

##### `impl Instrument for EnteredSpan`

##### `impl Receiver for EnteredSpan`

- <span id="enteredspan-type-target"></span>`type Target = T`

##### `impl WithSubscriber for EnteredSpan`

### `PhantomNotSend`

```rust
struct PhantomNotSend {
    ghost: core::marker::PhantomData<*mut ()>,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:1594-1596`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L1594-L1596)*

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

- <span id="phantomnotsend-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for PhantomNotSend`

##### `impl Sync for PhantomNotSend`

##### `impl WithSubscriber for PhantomNotSend`

## Traits

### `AsId`

```rust
trait AsId: crate::sealed::Sealed { ... }
```

*Defined in [`tracing-0.1.43/src/span.rs:336-340`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L336-L340)*

Trait implemented by types which have a span `Id`.

#### Required Methods

- `fn as_id(&self) -> Option<&Id>`

  Returns the `Id` of the span that `self` corresponds to, or `None` if

## Functions

*Defined in [`tracing-0.1.43/src/span.rs:321`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L321)*

## Constants

### `PhantomNotSend`
```rust
const PhantomNotSend: PhantomNotSend;
```

*Defined in [`tracing-0.1.43/src/span.rs:1599`](../../../.source_1765210505/tracing-0.1.43/src/span.rs#L1599)*

