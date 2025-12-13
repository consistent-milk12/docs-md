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

*Defined in [`addr2line-0.25.1/src/lib.rs:95-99`](../../../.source_1765521767/addr2line-0.25.1/src/lib.rs#L95-L99)*

*Re-exported from `addr2line`*

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

#### Implementations

- <span id="context-find-dwarf-and-unit"></span>`fn find_dwarf_and_unit(&self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Option<gimli::UnitRef<'_, R>>, Buf = R>>` — [`DefaultGuard`](../dispatcher/index.md#defaultguard)

  Find the DWARF unit corresponding to the given virtual memory address.

- <span id="context-find-location"></span>`fn find_location(&self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>` — [`Record`](#record), [`DefaultGuard`](../dispatcher/index.md#defaultguard), [`set_global_default`](../dispatcher/index.md#set-global-default)

  Find the source file and line corresponding to the given virtual memory address.

- <span id="context-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64) -> Result<LocationRangeIter<'_, R>, gimli::Error>` — [`Record`](#record)

  Return source file and lines for a range of addresses. For each location it also

  returns the address and size of the range of the underlying instructions.

- <span id="context-find-frames"></span>`fn find_frames(&self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Result<FrameIter<'_, R>, gimli::Error>, Buf = R>>` — [`Record`](#record)

  Return an iterator for the function frames corresponding to the given virtual

  memory address.

  

  If the probe address is not for an inline function then only one frame is

  returned.

  

  If the probe address is for an inline function then the first frame corresponds

  to the innermost inline function.  Subsequent frames contain the caller and call

  location, until an non-inline caller is reached.

- <span id="context-preload-units"></span>`fn preload_units(&self, probe: u64) -> impl Iterator<Item = (SplitDwarfLoad<R>, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(), gimli::Error> + '_)>` — [`DefaultGuard`](../subscriber/index.md#defaultguard), [`DefaultGuard`](../dispatcher/index.md#defaultguard), [`Record`](#record)

  Preload units for `probe`.

  

  The iterator returns pairs of `SplitDwarfLoad`s containing the

  information needed to locate and load split DWARF for `probe` and

  a matching callback to invoke once that data is available.

  

  If this method is called, and all of the returned closures are invoked,

  addr2line guarantees that any future API call for the address `probe`

  will not require the loading of any split DWARF.

  

  ```no_run

    use addr2line::*;

    use std::sync::Arc;

    let ctx: Context<gimli::EndianSlice<gimli::RunTimeEndian>> = todo!();

    let do_split_dwarf_load = |load: SplitDwarfLoad<gimli::EndianSlice<gimli::RunTimeEndian>>| -> Option<Arc<gimli::Dwarf<gimli::EndianSlice<gimli::RunTimeEndian>>>> { None };

    const ADDRESS: u64 = 0xdeadbeef;

    ctx.preload_units(ADDRESS).for_each(|(load, callback)| {

      let dwo = do_split_dwarf_load(load);

      callback(dwo);

    });

  

    let frames_iter = match ctx.find_frames(ADDRESS) {

      LookupResult::Output(result) => result,

      LookupResult::Load { .. } => unreachable!("addr2line promised we wouldn't get here"),

    };

  

    // ...

  ```

- <span id="context-from-sections"></span>`fn from_sections(debug_abbrev: gimli::DebugAbbrev<R>, debug_addr: gimli::DebugAddr<R>, debug_aranges: gimli::DebugAranges<R>, debug_info: gimli::DebugInfo<R>, debug_line: gimli::DebugLine<R>, debug_line_str: gimli::DebugLineStr<R>, debug_ranges: gimli::DebugRanges<R>, debug_rnglists: gimli::DebugRngLists<R>, debug_str: gimli::DebugStr<R>, debug_str_offsets: gimli::DebugStrOffsets<R>, default_section: R) -> Result<Self, gimli::Error>` — [`Record`](#record)

  Construct a new `Context` from DWARF sections.

  

  This method does not support using a supplementary object file.

- <span id="context-from-dwarf"></span>`fn from_dwarf(sections: gimli::Dwarf<R>) -> Result<Context<R>, gimli::Error>` — [`Record`](#record), [`Id`](#id)

  Construct a new `Context` from an existing [`gimli::Dwarf`](../../miniz_oxide/index.md) object.

- <span id="context-from-arc-dwarf"></span>`fn from_arc_dwarf(sections: Arc<gimli::Dwarf<R>>) -> Result<Context<R>, gimli::Error>` — [`Record`](#record), [`Id`](#id)

  Construct a new `Context` from an existing [`gimli::Dwarf`](../../miniz_oxide/index.md) object.

- <span id="context-find-unit"></span>`fn find_unit(&self, offset: gimli::DebugInfoOffset<<R as >::Offset>, file: DebugFile) -> Result<(&gimli::Unit<R>, gimli::UnitOffset<<R as >::Offset>), gimli::Error>` — [`Id`](#id), [`Record`](#record)

#### Trait Implementations

##### `impl Any for Context<R>`

- <span id="context-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Context<R>`

- <span id="context-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Context<R>`

- <span id="context-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Context<R>`

- <span id="context-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Context<R>`

- <span id="context-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Context<R>`

- <span id="context-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="context-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`Record`](#record), [`Attributes`](#attributes)

##### `impl<U> TryInto for Context<R>`

- <span id="context-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="context-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`Record`](#record), [`Attributes`](#attributes)

### `Span`

```rust
struct Span {
    inner: Option<Inner>,
    meta: Option<&'static crate::Metadata<'static>>,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:349-359`](../../../.source_1765521767/tracing-0.1.43/src/span.rs#L349-L359)*

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

  Constructs a new `Span` with the given [`metadata`](../../tracing_core/metadata/index.md) and set of

  [field values].

  

  The new span will be constructed by the currently-active [`Subscriber`](../../tracing_core/subscriber/index.md),

  with the current span as its parent (if one exists).

  

  After the span is constructed, [field values] and/or `follows_from`

  annotations may be added to it.

  

  

  

- <span id="span-new-root"></span>`fn new_root(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Metadata`](../index.md#metadata), [`Span`](#span)

  Constructs a new `Span` as the root of its own trace tree, with the

  given [`metadata`](../../tracing_core/metadata/index.md) and set of [field values].

  

  After the span is constructed, [field values] and/or `follows_from`

  annotations may be added to it.

  

  

- <span id="span-child-of"></span>`fn child_of(parent: impl Into<Option<Id>>, meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Id`](#id), [`Metadata`](../index.md#metadata), [`Span`](#span)

  Constructs a new `Span` as child of the given parent span, with the

  given [`metadata`](../../tracing_core/metadata/index.md) and set of [field values].

  

  After the span is constructed, [field values] and/or `follows_from`

  annotations may be added to it.

  

  

- <span id="span-new-disabled"></span>`fn new_disabled(meta: &'static Metadata<'static>) -> Span` — [`Metadata`](../index.md#metadata), [`Span`](#span)

  Constructs a new disabled span with the given `Metadata`.

  

  This should be used when a span is constructed from a known callsite,

  but the subscriber indicates that it is disabled.

  

  Entering, exiting, and recording values on this span will not notify the

  `Subscriber` but _may_ record log messages if the `log` feature flag is

  enabled.

- <span id="span-none"></span>`const fn none() -> Span` — [`Span`](#span)

  Constructs a new span that is *completely disabled*.

  

  This can be used rather than `Option<Span>` to represent cases where a

  span is not present.

  

  Entering, exiting, and recording values on this span will do nothing.

- <span id="span-current"></span>`fn current() -> Span` — [`Span`](#span)

  Returns a handle to the span [considered by the `Subscriber`] to be the

  current span.

  

  If the subscriber indicates that it does not track the current span, or

  that the thread from which this function is called is not currently

  inside a span, the returned span will be disabled.

- <span id="span-make-with"></span>`fn make_with(meta: &'static Metadata<'static>, new_span: Attributes<'_>, dispatch: &Dispatch) -> Span` — [`Metadata`](../index.md#metadata), [`Attributes`](#attributes), [`Dispatch`](../dispatcher/index.md#dispatch), [`Span`](#span)

- <span id="span-enter"></span>`fn enter(&self) -> Entered<'_>` — [`Entered`](#entered)

  Enters this span, returning a guard that will exit the span when dropped.

  

  If this span is enabled by the current subscriber, then this function will

  call `Subscriber::enter` with the span's [`Id`](../../tracing_core/span/index.md), and dropping the guard

  will call `Subscriber::exit`. If the span is disabled, this does

  nothing.

  

  # In Asynchronous Code

  

  **Warning**: in asynchronous code that uses [async/await syntax][`syntax`](../../regex_automata/util/syntax/index.md),

  `Span::enter` should be used very carefully or avoided entirely. Holding

  the drop guard returned by `Span::enter` across `.await` points will

  result in incorrect traces. For example,

  

  ```rust

  use tracing::info_span;

  async fn some_other_async_function() {}

  async fn my_async_function() {

      let span = info_span!("my_async_function");

  

      // WARNING: This span will remain entered until this

      // guard is dropped...

      let _enter = span.enter();

      // ...but the `await` keyword may yield, causing the

      // runtime to switch to another task, while remaining in

      // this span!

      some_other_async_function().await

  

      // ...

  }

  ```

  

  The drop guard returned by `Span::enter` exits the span when it is

  dropped. When an async function or async block yields at an `.await`

  point, the current scope is _exited_, but values in that scope are

  **not** dropped (because the async block will eventually resume

  execution from that await point). This means that _another_ task will

  begin executing while _remaining_ in the entered span. This results in

  an incorrect trace.

  

  Instead of using `Span::enter` in asynchronous code, prefer the

  following:

  

  * To enter a span for a synchronous section of code within an async

    block or function, prefer `Span::in_scope`. Since `in_scope` takes a

    synchronous closure and exits the span when the closure returns, the

    span will always be exited before the next await point. For example:

    ```rust

    use tracing::info_span;

    async fn some_other_async_function(_: ()) {}

    async fn my_async_function() {

        let span = info_span!("my_async_function");

  

        let some_value = span.in_scope(|| {

            // run some synchronous code inside the span...

        });

  

        // This is okay! The span has already been exited before we reach

        // the await point.

        some_other_async_function(some_value).await;

  

        // ...

    }

    ```

  * For instrumenting asynchronous code, `tracing` provides the

    [`Future::instrument` combinator][`instrument`](../instrument/index.md) for

    attaching a span to a future (async function or block). This will

    enter the span _every_ time the future is polled, and exit it whenever

    the future yields.

  

    `Instrument` can be used with an async block inside an async function:

    ```ignore

    use tracing::info_span;

    use tracing::Instrument;

  

    async fn some_other_async_function() {}

    async fn my_async_function() {

        let span = info_span!("my_async_function");

        async move {

           // This is correct! If we yield here, the span will be exited,

           // and re-entered when we resume.

           some_other_async_function().await;

  

           //more asynchronous code inside the span...

  

        }

          // instrument the async block with the span...

          .instrument(span)

          // ...and await it.

          .await

    }

    ```

  

    It can also be used to instrument calls to async functions at the

    callsite:

    ```ignore

    use tracing::debug_span;

    use tracing::Instrument;

  

    async fn some_other_async_function() {}

    async fn my_async_function() {

        let some_value = some_other_async_function()

           .instrument(debug_span!("some_other_async_function"))

           .await;

  

        // ...

    }

    ```

  

  * The [`#[instrument]` attribute macro][`attr`](../../clap_derive/attr/index.md) can automatically generate

    correct code when used on an async function:

  

    ```ignore

    async fn some_other_async_function() {}

    #[tracing::instrument(level = "info")]

    async fn my_async_function() {

  

        // This is correct! If we yield here, the span will be exited,

        // and re-entered when we resume.

        some_other_async_function().await;

  

        // ...

  

    }

    ```

  

  

  

  

  # Examples

  

  ```rust

  use tracing::{span, Level};

  let span = span!(Level::INFO, "my_span");

  let guard = span.enter();

  

  // code here is within the span

  

  drop(guard);

  

  // code here is no longer within the span

  

  ```

  

  Guards need not be explicitly dropped:

  

  ```rust

  use tracing::trace_span;

  fn my_function() -> String {

      // enter a span for the duration of this function.

      let span = trace_span!("my_function");

      let _enter = span.enter();

  

      // anything happening in functions we call is still inside the span...

      my_other_function();

  

      // returning from the function drops the guard, exiting the span.

      return "Hello world".to_owned();

  }

  

  fn my_other_function() {

      // ...

  }

  ```

  

  Sub-scopes may be created to limit the duration for which the span is

  entered:

  

  ```rust

  use tracing::{info, info_span};

  let span = info_span!("my_great_span");

  

  {

      let _enter = span.enter();

  

      // this event occurs inside the span.

      info!("i'm in the span!");

  

      // exiting the scope drops the guard, exiting the span.

  }

  

  // this event is not inside the span.

  info!("i'm outside the span!")

  ```

  

  

- <span id="span-entered"></span>`fn entered(self) -> EnteredSpan` — [`EnteredSpan`](#enteredspan)

  Enters this span, consuming it and returning a [guard][`EnteredSpan`](#enteredspan)

  that will exit the span when dropped.

  

  <pre class="compile_fail" style="white-space:normal;font:inherit;">

      <strong>Warning</strong>: In asynchronous code that uses async/await syntax,

      <code>Span::entered</code> may produce incorrect traces if the returned drop

      guard is held across an await point. See <a href="#in-asynchronous-code">the

      <code>Span::enter</code> documentation</a> for details.

  </pre>

  

  

  If this span is enabled by the current subscriber, then this function will

  call `Subscriber::enter` with the span's [`Id`](../../tracing_core/span/index.md), and dropping the guard

  will call `Subscriber::exit`. If the span is disabled, this does

  nothing.

  

  This is similar to the `Span::enter` method, except that it moves the

  span by value into the returned guard, rather than borrowing it.

  Therefore, this method can be used to create and enter a span in a

  single expression, without requiring a `let`-binding. For example:

  

  ```rust

  use tracing::info_span;

  let _span = info_span!("something_interesting").entered();

  ```

  rather than:

  ```rust

  use tracing::info_span;

  let span = info_span!("something_interesting");

  let _e = span.enter();

  ```

  

  Furthermore, `entered` may be used when the span must be stored in some

  other struct or be passed to a function while remaining entered.

  

  <pre class="ignore" style="white-space:normal;font:inherit;">

      <strong>Note</strong>: The returned <a href="../struct.EnteredSpan.html">

      <code>EnteredSpan</code></a> guard does not implement <code>Send</code>.

      Dropping the guard will exit <em>this</em> span, and if the guard is sent

      to another thread and dropped there, that thread may never have entered

      this span. Thus, <code>EnteredSpan</code>s should not be sent between threads.

  </pre>

  

  # Examples

  

  The returned guard can be `explicitly exited`,

  returning the un-entered span:

  

  ```rust

  use tracing::{Level, span};

  let span = span!(Level::INFO, "doing_something").entered();

  

  // code here is within the span

  

  // explicitly exit the span, returning it

  let span = span.exit();

  

  // code here is no longer within the span

  

  // enter the span again

  let span = span.entered();

  

  // now we are inside the span once again

  ```

  

  Guards need not be explicitly dropped:

  

  ```rust

  use tracing::trace_span;

  fn my_function() -> String {

      // enter a span for the duration of this function.

      let span = trace_span!("my_function").entered();

  

      // anything happening in functions we call is still inside the span...

      my_other_function();

  

      // returning from the function drops the guard, exiting the span.

      return "Hello world".to_owned();

  }

  

  fn my_other_function() {

      // ...

  }

  ```

  

  Since the [`EnteredSpan`](#enteredspan) guard can dereference to the [`Span`](#span) itself,

  the span may still be accessed while entered. For example:

  

  ```rust

  use tracing::info_span;

  use tracing::field;

  

  // create the span with an empty field, and enter it.

  let span = info_span!("my_span", some_field = field::Empty).entered();

  

  // we can still record a value for the field while the span is entered.

  span.record("some_field", &"hello world!");

  ```

  

  

- <span id="span-or-current"></span>`fn or_current(self) -> Self`

  Returns this span, if it was [`enabled`](../index.md) by the current [`Subscriber`](../../tracing_core/subscriber/index.md), or

  the [current span] (whose lexical distance may be further than expected),

   if this span [is disabled].

  

  This method can be useful when propagating spans to spawned threads or

  [async tasks]. Consider the following:

  

  ```rust

  let _parent_span = tracing::info_span!("parent").entered();

  

  // ...

  

  let child_span = tracing::debug_span!("child");

  

  std::thread::spawn(move || {

      let _entered = child_span.entered();

  

      tracing::info!("spawned a thread!");

  

      // ...

  });

  ```

  

  If the current [`Subscriber`](../../tracing_core/subscriber/index.md) enables the `DEBUG` level, then both

  the "parent" and "child" spans will be enabled. Thus, when the "spawned

  a thread!" event occurs, it will be inside of the "child" span. Because

  "parent" is the parent of "child", the event will _also_ be inside of

  "parent".

  

  However, if the [`Subscriber`](../../tracing_core/subscriber/index.md) only enables the `INFO` level, the "child"

  span will be disabled. When the thread is spawned, the

  `child_span.entered()` call will do nothing, since "child" is not

  enabled. In this case, the "spawned a thread!" event occurs outside of

  *any* span, since the "child" span was responsible for propagating its

  parent to the spawned thread.

  

  If this is not the desired behavior, `Span::or_current` can be used to

  ensure that the "parent" span is propagated in both cases, either as a

  parent of "child" _or_ directly. For example:

  

  ```rust

  let _parent_span = tracing::info_span!("parent").entered();

  

  // ...

  

  // If DEBUG is enabled, then "child" will be enabled, and `or_current`

  // returns "child". Otherwise, if DEBUG is not enabled, "child" will be

  // disabled, and `or_current` returns "parent".

  let child_span = tracing::debug_span!("child").or_current();

  

  std::thread::spawn(move || {

      let _entered = child_span.entered();

  

      tracing::info!("spawned a thread!");

  

      // ...

  });

  ```

  

  When spawning [asynchronous tasks][async tasks], `Span::or_current` can

  be used similarly, in combination with [`instrument`](../instrument/index.md):

  

  ```rust

  use tracing::Instrument;

  // lol

  mod tokio {

      pub(super) fn spawn(_: impl std::future::Future) {}

  }

  

  let _parent_span = tracing::info_span!("parent").entered();

  

  // ...

  

  let child_span = tracing::debug_span!("child");

  

  tokio::spawn(

      async {

          tracing::info!("spawned a task!");

  

          // ...

  

      }.instrument(child_span.or_current())

  );

  ```

  

  In general, `or_current` should be preferred over nesting an

  [`instrument`](../instrument/index.md)  call inside of an `in_current_span` call, as using

  `or_current` will be more efficient.

  

  ```rust

  use tracing::Instrument;

  // lol

  mod tokio {

      pub(super) fn spawn(_: impl std::future::Future) {}

  }

  async fn my_async_fn() {

      // ...

  }

  

  let _parent_span = tracing::info_span!("parent").entered();

  

  // Do this:

  tokio::spawn(

      my_async_fn().instrument(tracing::debug_span!("child").or_current())

  );

  

  // ...rather than this:

  tokio::spawn(

      my_async_fn()

          .instrument(tracing::debug_span!("child"))

          .in_current_span()

  );

  ```

  

  

  

  

  

  

  

  

- <span id="span-do-enter"></span>`fn do_enter(&self)`

- <span id="span-do-exit"></span>`fn do_exit(&self)`

- <span id="span-in-scope"></span>`fn in_scope<F: FnOnce() -> T, T>(&self, f: F) -> T`

  Executes the given function in the context of this span.

  

  If this span is enabled, then this function enters the span, invokes `f`

  and then exits the span. If the span is disabled, `f` will still be

  invoked, but in the context of the currently-executing span (if there is

  one).

  

  Returns the result of evaluating `f`.

  

  # Examples

  

  ```rust

  use tracing::{trace, span, Level};

  let my_span = span!(Level::TRACE, "my_span");

  

  my_span.in_scope(|| {

      // this event occurs within the span.

      trace!("i'm in the span!");

  });

  

  // this event occurs outside the span.

  trace!("i'm not in the span!");

  ```

  

  Calling a function and returning the result:

  ```rust

  use tracing::{info_span, Level};

  fn hello_world() -> String {

      "Hello world!".to_owned()

  }

  

  let span = info_span!("hello_world");

  // the span will be entered for the duration of the call to

  // `hello_world`.

  let a_string = span.in_scope(hello_world);

- <span id="span-field"></span>`fn field<Q: field::AsField + ?Sized>(&self, field: &Q) -> Option<field::Field>`

  Returns a `Field` for the field with the

  given `name`, if one exists,

- <span id="span-has-field"></span>`fn has_field<Q: field::AsField + ?Sized>(&self, field: &Q) -> bool`

  Returns true if this `Span` has a field for the given

  `Field` or field name.

- <span id="span-record"></span>`fn record<Q: field::AsField + ?Sized, V: field::Value>(&self, field: &Q, value: V) -> &Self`

  Records that the field described by `field` has the value `value`.

  

  This may be used with `field::Empty` to declare fields whose values

  are not known when the span is created, and record them later:

  ```rust

  use tracing::{trace_span, field};

  

  // Create a span with two fields: `greeting`, with the value "hello world", and

  // `parting`, without a value.

  let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);

  

  // ...

  

  // Now, record a value for parting as well.

  // (note that the field name is passed as a string slice)

  span.record("parting", "goodbye world!");

  ```

  However, it may also be used to record a _new_ value for a field whose

  value was already recorded:

  ```rust

  use tracing::info_span;

  fn do_something() -> Result<(), ()> { Err(()) }

  

  // Initially, let's assume that our attempt to do something is going okay...

  let span = info_span!("doing_something", is_okay = true);

  let _e = span.enter();

  

  match do_something() {

      Ok(something) => {

          // ...

      }

      Err(_) => {

          // Things are no longer okay!

          span.record("is_okay", false);

      }

  }

  ```

  

  <pre class="ignore" style="white-space:normal;font:inherit;">

      <strong>Note</strong>: The fields associated with a span are part

      of its <a href="../struct.Metadata.html"><code>Metadata</code></a>.

      The <a href="../struct.Metadata.html"><code>Metadata</code></a>

      describing a particular span is constructed statically when the span

      is created and cannot be extended later to add new fields. Therefore,

      you cannot record a value for a field that was not specified when the

      span was created:

  </pre>

  

  ```rust

  use tracing::{trace_span, field};

  

  // Create a span with two fields: `greeting`, with the value "hello world", and

  // `parting`, without a value.

  let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);

  

  // ...

  

  // Now, you try to record a value for a new field, `new_field`, which was not

  // declared as `Empty` or populated when you created `span`.

  // You won't get any error, but the assignment will have no effect!

  span.record("new_field", "interesting_value_you_really_need");

  

  // Instead, all fields that may be recorded after span creation should be declared up front,

  // using field::Empty when a value is not known, as we did for `parting`.

  // This `record` call will indeed replace field::Empty with "you will be remembered".

  span.record("parting", "you will be remembered");

  ```

  

  <div class="example-wrap" style="display:inline-block">

  <pre class="ignore" style="white-space:normal;font:inherit;">

  **Note**: To record several values in just one call, see the [`record_all!`](crate::record_all!) macro.

  </pre></div>

  

- <span id="span-is-disabled"></span>`fn is_disabled(&self) -> bool`

  Returns `true` if this span was disabled by the subscriber and does not

  exist.

  

  See also `is_none`.

- <span id="span-is-none"></span>`fn is_none(&self) -> bool`

  Returns `true` if this span was constructed by `Span::none` and is

  empty.

  

  If `is_none` returns `true` for a given span, then `is_disabled` will

  also return `true`. However, when a span is disabled by the subscriber

  rather than constructed by `Span::none`, this method will return

  `false`, while `is_disabled` will return `true`.

  

- <span id="span-follows-from"></span>`fn follows_from(&self, from: impl Into<Option<Id>>) -> &Self` — [`Id`](#id)

  Indicates that the span with the given ID has an indirect causal

  relationship with this span.

  

  This relationship differs somewhat from the parent-child relationship: a

  span may have any number of prior spans, rather than a single one; and

  spans are not considered to be executing _inside_ of the spans they

  follow from. This means that a span may close even if subsequent spans

  that follow from it are still open, and time spent inside of a

  subsequent span should not be included in the time its precedents were

  executing. This is used to model causal relationships such as when a

  single future spawns several related background tasks, et cetera.

  

  If this span is disabled, or the resulting follows-from relationship

  would be invalid, this function will do nothing.

  

  # Examples

  

  Setting a `follows_from` relationship with a `Span`:

  ```rust

  use tracing::{span, Id, Level, Span};

  let span1 = span!(Level::INFO, "span_1");

  let span2 = span!(Level::DEBUG, "span_2");

  span2.follows_from(span1);

  ```

  

  Setting a `follows_from` relationship with the current span:

  ```rust

  use tracing::{span, Id, Level, Span};

  let span = span!(Level::INFO, "hello!");

  span.follows_from(Span::current());

  ```

  

  Setting a `follows_from` relationship with a `Span` reference:

  ```rust

  use tracing::{span, Id, Level, Span};

  let span = span!(Level::INFO, "hello!");

  let curr = Span::current();

  span.follows_from(&curr);

  ```

  

  Setting a `follows_from` relationship with an `Id`:

  ```rust

  use tracing::{span, Id, Level, Span};

  let span = span!(Level::INFO, "hello!");

  let id = span.id();

  span.follows_from(id);

  ```

- <span id="span-id"></span>`fn id(&self) -> Option<Id>` — [`Id`](#id)

  Returns this span's `Id`, if it is enabled.

- <span id="span-metadata"></span>`fn metadata(&self) -> Option<&'static Metadata<'static>>` — [`Metadata`](../index.md#metadata)

  Returns this span's `Metadata`, if it is enabled.

- <span id="span-with-subscriber"></span>`fn with_subscriber<T>(&self, f: impl FnOnce((&Id, &Dispatch)) -> T) -> Option<T>` — [`Id`](#id), [`Dispatch`](../dispatcher/index.md#dispatch)

  Invokes a function with a reference to this span's ID and subscriber.

  

  if this span is enabled, the provided function is called, and the result is returned.

  If the span is disabled, the function is not called, and this method returns `None`

  instead.

#### Trait Implementations

##### `impl Any for Span`

- <span id="span-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Span`

- <span id="span-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Span`

- <span id="span-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl CloneToUninit for Span`

- <span id="span-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Span`

- <span id="span-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Span`

- <span id="span-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Span`

- <span id="span-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Span`

- <span id="span-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl Instrument for Span`

##### `impl<U> Into for Span`

- <span id="span-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Span`

- <span id="span-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for Span`

- <span id="span-toowned-type-owned"></span>`type Owned = T`

- <span id="span-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="span-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Span`

- <span id="span-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="span-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Span`

- <span id="span-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="span-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for Span`

### `Inner`

```rust
struct Inner {
    id: Id,
    subscriber: crate::dispatcher::Dispatch,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:367-376`](../../../.source_1765521767/tracing-0.1.43/src/span.rs#L367-L376)*

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

  Indicates that the span with the given ID has an indirect causal

  relationship with this span.

  

  This relationship differs somewhat from the parent-child relationship: a

  span may have any number of prior spans, rather than a single one; and

  spans are not considered to be executing _inside_ of the spans they

  follow from. This means that a span may close even if subsequent spans

  that follow from it are still open, and time spent inside of a

  subsequent span should not be included in the time its precedents were

  executing. This is used to model causal relationships such as when a

  single future spawns several related background tasks, et cetera.

  

  If this span is disabled, this function will do nothing. Otherwise, it

  returns `Ok(())` if the other span was added as a precedent of this

  span, or an error if this was not possible.

- <span id="inner-id"></span>`fn id(&self) -> Id` — [`Id`](#id)

  Returns the span's ID.

- <span id="inner-record"></span>`fn record(&self, values: &Record<'_>)` — [`Record`](#record)

- <span id="inner-new"></span>`fn new(id: Id, subscriber: &Dispatch) -> Self` — [`Id`](#id), [`Dispatch`](../dispatcher/index.md#dispatch)

#### Trait Implementations

##### `impl Any for Inner`

- <span id="inner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inner`

- <span id="inner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inner`

- <span id="inner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Inner`

- <span id="inner-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Inner`

- <span id="inner-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Inner`

- <span id="inner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Inner`

- <span id="inner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Inner`

- <span id="inner-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl Instrument for Inner`

##### `impl<U> Into for Inner`

- <span id="inner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Inner`

- <span id="inner-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for Inner`

- <span id="inner-toowned-type-owned"></span>`type Owned = T`

- <span id="inner-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="inner-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Inner`

- <span id="inner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Inner`

- <span id="inner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for Inner`

### `Entered<'a>`

```rust
struct Entered<'a> {
    span: &'a Span,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:388-390`](../../../.source_1765521767/tracing-0.1.43/src/span.rs#L388-L390)*

A guard representing a span which has been entered and is currently
executing.

When the guard is dropped, the span will be exited.

This is returned by the `Span::enter` function.


#### Trait Implementations

##### `impl Any for Entered<'a>`

- <span id="entered-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entered<'a>`

- <span id="entered-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entered<'a>`

- <span id="entered-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Entered<'a>`

- <span id="entered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Entered<'_>`

- <span id="entered-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Entered<'a>`

- <span id="entered-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for Entered<'a>`

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

##### `impl WithSubscriber for Entered<'a>`

### `EnteredSpan`

```rust
struct EnteredSpan {
    span: Span,
    _not_send: PhantomNotSend,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:402-412`](../../../.source_1765521767/tracing-0.1.43/src/span.rs#L402-L412)*

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

  Returns this span's `Id`, if it is enabled.

- <span id="enteredspan-exit"></span>`fn exit(self) -> Span` — [`Span`](#span)

  Exits this span, returning the underlying [`Span`](#span).

#### Trait Implementations

##### `impl Any for EnteredSpan`

- <span id="enteredspan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EnteredSpan`

- <span id="enteredspan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EnteredSpan`

- <span id="enteredspan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for EnteredSpan`

- <span id="enteredspan-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for EnteredSpan`

- <span id="enteredspan-deref-type-target"></span>`type Target = Span`

- <span id="enteredspan-deref"></span>`fn deref(&self) -> &Span` — [`Span`](#span)

##### `impl Drop for EnteredSpan`

- <span id="enteredspan-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for EnteredSpan`

- <span id="enteredspan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for EnteredSpan`

##### `impl<U> Into for EnteredSpan`

- <span id="enteredspan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for EnteredSpan`

- <span id="enteredspan-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for EnteredSpan`

- <span id="enteredspan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enteredspan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EnteredSpan`

- <span id="enteredspan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enteredspan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for EnteredSpan`

### `PhantomNotSend`

```rust
struct PhantomNotSend {
    ghost: core::marker::PhantomData<*mut ()>,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:1594-1596`](../../../.source_1765521767/tracing-0.1.43/src/span.rs#L1594-L1596)*

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

##### `impl Any for PhantomNotSend`

- <span id="phantomnotsend-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PhantomNotSend`

- <span id="phantomnotsend-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PhantomNotSend`

- <span id="phantomnotsend-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PhantomNotSend`

- <span id="phantomnotsend-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PhantomNotSend`

- <span id="phantomnotsend-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for PhantomNotSend`

##### `impl<U> Into for PhantomNotSend`

- <span id="phantomnotsend-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for PhantomNotSend`

##### `impl<U> TryFrom for PhantomNotSend`

- <span id="phantomnotsend-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="phantomnotsend-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PhantomNotSend`

- <span id="phantomnotsend-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="phantomnotsend-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for PhantomNotSend`

## Traits

### `AsId`

```rust
trait AsId: crate::sealed::Sealed { ... }
```

*Defined in [`tracing-0.1.43/src/span.rs:336-340`](../../../.source_1765521767/tracing-0.1.43/src/span.rs#L336-L340)*

Trait implemented by types which have a span `Id`.

#### Required Methods

- `fn as_id(&self) -> Option<&Id>`

  Returns the `Id` of the span that `self` corresponds to, or `None` if

## Functions

### `Attributes`

```rust
fn Attributes(self) -> Result<U, <U as TryFrom>::Error>
```

## Constants

### `PhantomNotSend`
```rust
const PhantomNotSend: PhantomNotSend;
```

*Defined in [`tracing-0.1.43/src/span.rs:1599`](../../../.source_1765521767/tracing-0.1.43/src/span.rs#L1599)*

