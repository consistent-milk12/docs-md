# Crate `tracing`

 A scoped, structured logging and diagnostics system.

 # Overview

 `tracing` is a framework for instrumenting Rust programs to collect
 structured, event-based diagnostic information.

 In asynchronous systems like Tokio, interpreting traditional log messages can
 often be quite challenging. Since individual tasks are multiplexed on the same
 thread, associated events and log lines are intermixed making it difficult to
 trace the logic flow. `tracing` expands upon logging-style diagnostics by
 allowing libraries and applications to record structured events with additional
 information about *temporality* and *causality* — unlike a log message, a span
 in `tracing` has a beginning and end time, may be entered and exited by the
 flow of execution, and may exist within a nested tree of similar spans. In
 addition, `tracing` spans are *structured*, with the ability to record typed
 data as well as textual messages.

 The `tracing` crate provides the APIs necessary for instrumenting libraries
 and applications to emit trace data.

 *Compiler support: [requires `rustc` 1.65+][msrv]*

 # Core Concepts

 The core of `tracing`'s API is composed of _spans_, _events_ and
 _subscribers_. We'll cover these in turn.

 ## Spans

 To record the flow of execution through a program, `tracing` introduces the
 concept of [spans]. Unlike a log line that represents a _moment in
 time_, a span represents a _period of time_ with a beginning and an end. When a
 program begins executing in a context or performing a unit of work, it
 _enters_ that context's span, and when it stops executing in that context,
 it _exits_ the span. The span in which a thread is currently executing is
 referred to as that thread's _current_ span.

 For example:
 ```rust
 use tracing::{span, Level};
 fn main() {
 let span = span!(Level::TRACE, "my_span");
 // `enter` returns a RAII guard which, when dropped, exits the span. this
 // indicates that we are in the span for the current lexical scope.
 let _enter = span.enter();
 // perform some work in the context of `my_span`...
 }
```

 The [`span` module][`span`](span/index.md)'s documentation provides further details on how to
 use spans.

 <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">

  **Warning**: In asynchronous code that uses async/await syntax,
  `Span::enter` may produce incorrect traces if the returned drop
  guard is held across an await point. See
  [the method documentation][Span#in-asynchronous-code] for details.

 </pre></div>

 ## Events

 An [`Event`](#event) represents a _moment_ in time. It signifies something that
 happened while a trace was being recorded. `Event`s are comparable to the log
 records emitted by unstructured logging code, but unlike a typical log line,
 an `Event` may occur within the context of a span.

 For example:
 ```rust
 use tracing::{event, span, Level};

 fn main() {
 // records an event outside of any span context:
 event!(Level::INFO, "something happened");

 let span = span!(Level::INFO, "my_span");
 let _guard = span.enter();

 // records an event within "my_span".
 event!(Level::DEBUG, "something happened inside my_span");
 }
```

 In general, events should be used to represent points in time _within_ a
 span — a request returned with a given status code, _n_ new items were
 taken from a queue, and so on.

 The [`Event` struct][`Event`](#event) documentation provides further details on using
 events.

 ## Subscribers

 As `Span`s and `Event`s occur, they are recorded or aggregated by
 implementations of the [`Subscriber`](../tracing_core/subscriber/index.md) trait. `Subscriber`s are notified
 when an `Event` takes place and when a `Span` is entered or exited. These
 notifications are represented by the following `Subscriber` trait methods:

 + `event`, called when an `Event` takes place,
 + `enter`, called when execution enters a `Span`,
 + [`exit`](#exit), called when execution exits a `Span`

 In addition, subscribers may implement the [`enabled`](#enabled) function to _filter_
 the notifications they receive based on [`metadata`](../tracing_core/metadata/index.md) describing each `Span`
 or `Event`. If a call to `Subscriber::enabled` returns `false` for a given
 set of metadata, that `Subscriber` will *not* be notified about the
 corresponding `Span` or `Event`. For performance reasons, if no currently
 active subscribers express interest in a given set of metadata by returning
 `true`, then the corresponding `Span` or `Event` will never be constructed.

 # Usage

 First, add this to your `Cargo.toml`:

 ```toml
 [dependencies]
 tracing = "0.1"
 ```

 ## Recording Spans and Events

 Spans and events are recorded using macros.

 ### Spans

 The [`span!`](#span) macro expands to a [`Span` struct][`Span`](span/index.md) which is used to
 record a span. The `Span::enter` method on that struct records that the
 span has been entered, and returns a [RAII] guard object, which will exit
 the span when dropped.

 For example:

 ```rust
 use tracing::{span, Level};
 fn main() {
 // Construct a new span named "my span" with trace log level.
 let span = span!(Level::TRACE, "my span");

 // Enter the span, returning a guard object.
 let _enter = span.enter();

 // Any trace events that occur before the guard is dropped will occur
 // within the span.

 // Dropping the guard will exit the span.
 }
 ```

 The [`#[instrument]`][`instrument`](instrument/index.md) attribute provides an easy way to
 add `tracing` spans to functions. A function annotated with `#[instrument]`
 will create and enter a span with that function's name every time the
 function is called, with arguments to that function will be recorded as
 fields using `fmt::Debug`.

 For example:
 ```ignore
 // this doctest is ignored because we don't have a way to say
 // that it should only be run with cfg(feature = "attributes")
 use tracing::{Level, event, instrument};

 #[instrument]
 pub fn my_function(my_arg: usize) {
     // This event will be recorded inside a span named `my_function` with the
     // field `my_arg`.
     event!(Level::INFO, "inside my_function!");
     // ...
 }
 fn main() {}
 ```

 For functions which don't have built-in tracing support and can't have
 the `#[instrument]` attribute applied (such as from an external crate),
 the [`Span` struct][`Span`](span/index.md) has a [`in_scope()` method]`in_scope`
 which can be used to easily wrap synchronous code in a span.

 For example:
 ```rust
 use tracing::info_span;

 fn doc() -> Result<(), ()> {
 mod serde_json {
    pub(crate) fn from_slice(buf: &[u8]) -> Result<(), ()> { Ok(()) }
 }
 let buf: [u8; 0] = [];
 let json = info_span!("json.parse").in_scope(|| serde_json::from_slice(&buf))?;
 let _ = json; // suppress unused variable warning
 Ok(())
 }
 ```

 You can find more examples showing how to use this crate [here][examples].


 ### Events

 [`Event`](#event)s are recorded using the [`event!`](#event) macro:

 ```rust
 fn main() {
 use tracing::{event, Level};
 event!(Level::INFO, "something has happened!");
 }
 ```

 ## Using the Macros

 The [`span!`](#span) and [`event!`](#event) macros as well as the `#[instrument]` attribute
 use fairly similar syntax, with some exceptions.

 ### Configuring Attributes

 Both macros require a [`Level`](#level) specifying the verbosity of the span or
 event. Optionally, the, [`target`](../tracing_attributes/attr/kw/index.md) and [parent span] may be overridden. If the
 target and parent span are not overridden, they will default to the
 module path where the macro was invoked and the current span (as determined
 by the subscriber), respectively.

 For example:

 ```rust
 use tracing::{span, event, Level};
 fn main() {
 span!(target: "app_spans", Level::TRACE, "my span");
 event!(target: "app_events", Level::INFO, "something has happened!");
 }
 ```
 ```rust
 use tracing::{span, event, Level};
 fn main() {
 let span = span!(Level::TRACE, "my span");
 event!(parent: &span, Level::INFO, "something has happened!");
 }
 ```

 The span macros also take a string literal after the level, to set the name
 of the span (as above).  In the case of the event macros, the name of the event can
 be overridden (the default is `event file:line`) using the `name:` specifier.

 ```rust
 use tracing::{span, event, Level};
 fn main() {
 span!(Level::TRACE, "my span");
 event!(name: "some_info", Level::INFO, "something has happened!");
 }
 ```

 ### Recording Fields

 Structured fields on spans and events are specified using the syntax
 `field_name = field_value`. Fields are separated by commas.

 ```rust
 use tracing::{event, Level};
 fn main() {
 // records an event with two fields:
 //  - "answer", with the value 42
 //  - "question", with the value "life, the universe and everything"
 event!(Level::INFO, answer = 42, question = "life, the universe, and everything");
 }
 ```

 As shorthand, local variables may be used as field values without an
 assignment, similar to [struct initializers]. For example:

 ```rust
 use tracing::{span, Level};
 fn main() {
 let user = "ferris";

 span!(Level::TRACE, "login", user);
 // is equivalent to:
 span!(Level::TRACE, "login", user = user);
 }
```

 Field names can include dots, but should not be terminated by them:
 ```rust
 use tracing::{span, Level};
 fn main() {
 let user = "ferris";
 let email = "ferris@rust-lang.org";
 span!(Level::TRACE, "login", user, user.email = email);
 }
```

 Since field names can include dots, fields on local structs can be used
 using the local variable shorthand:
 ```rust
 use tracing::{span, Level};
 fn main() {
 struct User {
    name: &'static str,
    email: &'static str,
 }
 let user = User {
     name: "ferris",
     email: "ferris@rust-lang.org",
 };
 // the span will have the fields `user.name = "ferris"` and
 // `user.email = "ferris@rust-lang.org"`.
 span!(Level::TRACE, "login", user.name, user.email);
 }
```

 Fields with names that are not Rust identifiers, or with names that are Rust reserved words,
 may be created using quoted string literals. However, this may not be used with the local
 variable shorthand.
 ```rust
 use tracing::{span, Level};
 fn main() {
 // records an event with fields whose names are not Rust identifiers
 //  - "guid:x-request-id", containing a `:`, with the value "abcdef"
 //  - "type", which is a reserved word, with the value "request"
 span!(Level::TRACE, "api", "guid:x-request-id" = "abcdef", "type" = "request");
 }
```

 Constant expressions can also be used as field names. Constants
 must be enclosed in curly braces (`{}`) to indicate that the *value*
 of the constant is to be used as the field name, rather than the
 constant's name. For example:
 ```rust
 use tracing::{span, Level};
 fn main() {
 const RESOURCE_NAME: &str = "foo";
 // this span will have the field `foo = "some_id"`
 span!(Level::TRACE, "get", { RESOURCE_NAME } = "some_id");
 }
```

 The `?` sigil is shorthand that specifies a field should be recorded using
 its `fmt::Debug` implementation:
 ```rust
 use tracing::{event, Level};
 fn main() {
 #[derive(Debug)]
 struct MyStruct {
     field: &'static str,
 }

 let my_struct = MyStruct {
     field: "Hello world!"
 };

 // `my_struct` will be recorded using its `fmt::Debug` implementation.
 event!(Level::TRACE, greeting = ?my_struct);
 // is equivalent to:
 event!(Level::TRACE, greeting = tracing::field::debug(&my_struct));
 }
 ```

 The `%` sigil operates similarly, but indicates that the value should be
 recorded using its `fmt::Display` implementation:
 ```rust
 use tracing::{event, Level};
 fn main() {
 #[derive(Debug)]
 struct MyStruct {
     field: &'static str,
 }

 let my_struct = MyStruct {
     field: "Hello world!"
 };
 // `my_struct.field` will be recorded using its `fmt::Display` implementation.
 event!(Level::TRACE, greeting = %my_struct.field);
 // is equivalent to:
 event!(Level::TRACE, greeting = tracing::field::display(&my_struct.field));
 }
 ```

 The `%` and `?` sigils may also be used with local variable shorthand:

 ```rust
 use tracing::{event, Level};
 fn main() {
 #[derive(Debug)]
 struct MyStruct {
     field: &'static str,
 }

 let my_struct = MyStruct {
     field: "Hello world!"
 };
 // `my_struct.field` will be recorded using its `fmt::Display` implementation.
 event!(Level::TRACE, %my_struct.field);
 }
 ```

 Additionally, a span may declare fields with the special value [`Empty`](#empty),
 which indicates that that the value for that field does not currently exist
 but may be recorded later. For example:

 ```rust
 use tracing::{trace_span, field};

 // Create a span with two fields: `greeting`, with the value "hello world", and
 // `parting`, without a value.
 let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);

 // ...

 // Now, record a value for parting as well.
 span.record("parting", &"goodbye world!");
 ```

 Finally, events may also include human-readable messages, in the form of a
 [format string][`fmt`](../anstream/fmt/index.md) and (optional) arguments, **after** the event's
 key-value fields. If a format string and arguments are provided,
 they will implicitly create a new field named `message` whose value is the
 provided set of format arguments.

 For example:

 ```rust
 use tracing::{event, Level};
 fn main() {
 let question = "the ultimate question of life, the universe, and everything";
 let answer = 42;
 // records an event with the following fields:
 // - `question.answer` with the value 42,
 // - `question.tricky` with the value `true`,
 // - "message", with the value "the answer to the ultimate question of life, the
 //    universe, and everything is 42."
 event!(
     Level::DEBUG,
     question.answer = answer,
     question.tricky = true,
     "the answer to {} is {}.", question, answer
 );
 }
 ```

 Specifying a formatted message in this manner does not allocate by default.








 ### Shorthand Macros

 `tracing` also offers a number of macros with preset verbosity levels.
 The [`trace!`](#trace), [`debug!`](#debug), [`info!`](#info), [`warn!`](#warn), and [`error!`](#error) behave
 similarly to the [`event!`](#event) macro, but with the [`Level`](#level) argument already
 specified, while the corresponding [`trace_span!`](#trace-span), [`debug_span!`](#debug-span),
 [`info_span!`](#info-span), [`warn_span!`](#warn-span), and [`error_span!`](#error-span) macros are the same,
 but for the [`span!`](#span) macro.

 These are intended both as a shorthand, and for compatibility with the [`log`](#log)
 crate (see the next section).












 ### For `log` Users

 Users of the [`log`](#log) crate should note that `tracing` exposes a set of
 macros for creating `Event`s (`trace!`, `debug!`, `info!`, `warn!`, and
 `error!`) which may be invoked with the same syntax as the similarly-named
 macros from the `log` crate. Often, the process of converting a project to
 use `tracing` can begin with a simple drop-in replacement.

 Let's consider the `log` crate's yak-shaving example:

 ```rust,ignore
 use std::{error::Error, io};
 use tracing::{debug, error, info, span, warn, Level};

 // the `#[tracing::instrument]` attribute creates and enters a span
 // every time the instrumented function is called. The span is named after the
 // the function or method. Parameters passed to the function are recorded as fields.
 #[tracing::instrument]
 pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
     // this creates an event at the DEBUG level with two fields:
     // - `excitement`, with the key "excitement" and the value "yay!"
     // - `message`, with the key "message" and the value "hello! I'm gonna shave a yak."
     //
     // unlike other fields, `message`'s shorthand initialization is just the string itself.
     debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
     if yak == 3 {
         warn!("could not locate yak!");
         // note that this is intended to demonstrate `tracing`'s features, not idiomatic
         // error handling! in a library or application, you should consider returning
         // a dedicated `YakError`. libraries like snafu or thiserror make this easy.
         return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
     } else {
         debug!("yak shaved successfully");
     }
     Ok(())
 }

 pub fn shave_all(yaks: usize) -> usize {
     // Constructs a new span named "shaving_yaks" at the TRACE level,
     // and a field whose key is "yaks". This is equivalent to writing:
     //
     // let span = span!(Level::TRACE, "shaving_yaks", yaks = yaks);
     //
     // local variables (`yaks`) can be used as field values
     // without an assignment, similar to struct initializers.
     let _span = span!(Level::TRACE, "shaving_yaks", yaks).entered();

     info!("shaving yaks");

     let mut yaks_shaved = 0;
     for yak in 1..=yaks {
         let res = shave(yak);
         debug!(yak, shaved = res.is_ok());

         if let Err(ref error) = res {
             // Like spans, events can also use the field initialization shorthand.
             // In this instance, `yak` is the field being initalized.
             error!(yak, error = error.as_ref(), "failed to shave yak!");
         } else {
             yaks_shaved += 1;
         }
         debug!(yaks_shaved);
     }

     yaks_shaved
 }
 ```

 ## In libraries

 Libraries should link only to the `tracing` crate, and use the provided
 macros to record whatever information will be useful to downstream
 consumers.

 ## In executables

 In order to record trace events, executables have to use a `Subscriber`
 implementation compatible with `tracing`. A `Subscriber` implements a
 way of collecting trace data, such as by logging it to standard output.

 This library does not contain any `Subscriber` implementations; these are
 provided by [other crates](#related-crates).

 The simplest way to use a subscriber is to call the [`set_global_default`](subscriber/index.md)
 function:

 ```rust
 extern crate tracing;
 pub struct FooSubscriber;
 use tracing::{span::{Id, Attributes, Record}, Metadata};
 impl tracing::Subscriber for FooSubscriber {
   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
   fn record(&self, _: &Id, _: &Record) {}
   fn event(&self, _: &tracing::Event) {}
   fn record_follows_from(&self, _: &Id, _: &Id) {}
   fn enabled(&self, _: &Metadata) -> bool { false }
   fn enter(&self, _: &Id) {}
   fn exit(&self, _: &Id) {}
 }
 impl FooSubscriber {
   fn new() -> Self { FooSubscriber }
 }
 fn main() {

 let my_subscriber = FooSubscriber::new();
 tracing::subscriber::set_global_default(my_subscriber)
     .expect("setting tracing default failed");
 }
 ```

 <pre class="compile_fail" style="white-space:normal;font:inherit;">
     <strong>Warning</strong>: In general, libraries should <em>not</em> call
     <code>set_global_default()</code>! Doing so will cause conflicts when
     executables that depend on the library try to set the default later.
 </pre>

 This subscriber will be used as the default in all threads for the
 remainder of the duration of the program, similar to setting the logger
 in the `log` crate.

 In addition, the default subscriber can be set through using the
 [`with_default`](subscriber/index.md) function. This follows the `tokio` pattern of using
 closures to represent executing code in a context that is exited at the end
 of the closure. For example:

 ```rust
 pub struct FooSubscriber;
 use tracing::{span::{Id, Attributes, Record}, Metadata};
 impl tracing::Subscriber for FooSubscriber {
   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
   fn record(&self, _: &Id, _: &Record) {}
   fn event(&self, _: &tracing::Event) {}
   fn record_follows_from(&self, _: &Id, _: &Id) {}
   fn enabled(&self, _: &Metadata) -> bool { false }
   fn enter(&self, _: &Id) {}
   fn exit(&self, _: &Id) {}
 }
 impl FooSubscriber {
   fn new() -> Self { FooSubscriber }
 }
 fn main() {

 let my_subscriber = FooSubscriber::new();
 #[cfg(feature = "std")]
 tracing::subscriber::with_default(my_subscriber, || {
     // Any trace events generated in this closure or by functions it calls
     // will be collected by `my_subscriber`.
 })
 }
 ```

 This approach allows trace data to be collected by multiple subscribers
 within different contexts in the program. Note that the override only applies to the
 currently executing thread; other threads will not see the change from with_default.

 Any trace events generated outside the context of a subscriber will not be collected.

 Once a subscriber has been set, instrumentation points may be added to the
 executable using the `tracing` crate's macros.

 ## `log` Compatibility

 The [`log`](#log) crate provides a simple, lightweight logging facade for Rust.
 While `tracing` builds upon `log`'s foundation with richer structured
 diagnostic data, `log`'s simplicity and ubiquity make it the "lowest common
 denominator" for text-based logging in Rust — a vast majority of Rust
 libraries and applications either emit or consume `log` records. Therefore,
 `tracing` provides multiple forms of interoperability with `log`: `tracing`
 instrumentation can emit `log` records, and a compatibility layer enables
 `tracing` [`Subscriber`](../tracing_core/subscriber/index.md)s to consume `log` records as `tracing` [`Event`](#event)s.

 ### Emitting `log` Records

 This crate provides two feature flags, "log" and "log-always", which will
 cause [spans] and [events] to emit `log` records. When the "log" feature is
 enabled, if no `tracing` `Subscriber` is active, invoking an event macro or
 creating a span with fields will emit a `log` record. This is intended
 primarily for use in libraries which wish to emit diagnostics that can be
 consumed by applications using `tracing` *or* `log`, without paying the
 additional overhead of emitting both forms of diagnostics when `tracing` is
 in use.

 Enabling the "log-always" feature will cause `log` records to be emitted
 even if a `tracing` `Subscriber` _is_ set. This is intended to be used in
 applications where a `log` `Logger` is being used to record a textual log,
 and `tracing` is used only to record other forms of diagnostics (such as
 metrics, profiling, or distributed tracing data). Unlike the "log" feature,
 libraries generally should **not** enable the "log-always" feature, as doing
 so will prevent applications from being able to opt out of the `log` records.

 See [here][`flags`](../portable_atomic/imp/atomic128/x86_64/detect/index.md) for more details on this crate's feature flags.

 The generated `log` records' messages will be a string representation of the
 span or event's fields, and all additional information recorded by `log`
 (target, verbosity level, module path, file, and line number) will also be
 populated. Additionally, `log` records are also generated when spans are
 entered, exited, and closed. Since these additional span lifecycle logs have
 the potential to be very verbose, and don't include additional fields, they
 will always be emitted at the `Trace` level, rather than inheriting the
 level of the span that generated them. Furthermore, they are categorized
 under a separate `log` target, "tracing::span" (and its sub-target,
 "tracing::span::active", for the logs on entering and exiting a span), which
 may be enabled or disabled separately from other `log` records emitted by
 `tracing`.

 ### Consuming `log` Records

 The `tracing-log` crate provides a compatibility layer which
 allows a `tracing` [`Subscriber`](../tracing_core/subscriber/index.md) to consume `log` records as though they
 were `tracing` [events]. This allows applications using `tracing` to record
 the logs emitted by dependencies using `log` as events within the context of
 the application's trace tree. See [that crate's documentation][log-tracer]
 for details.

 ## Related Crates

 In addition to `tracing` and `tracing-core`, the `tokio-rs/tracing` repository
 contains several additional crates designed to be used with the `tracing` ecosystem.
 This includes a collection of `Subscriber` implementations, as well as utility
 and adapter crates to assist in writing `Subscriber`s and instrumenting
 applications.

 In particular, the following crates are likely to be of interest:

  - `tracing-futures` provides a compatibility layer with the `futures`
    crate, allowing spans to be attached to `Future`s, `Stream`s, and `Executor`s.
  - `tracing-subscriber` provides `Subscriber` implementations and
    utilities for working with `Subscriber`s. This includes a `FmtSubscriber`
    `FmtSubscriber` for logging formatted trace data to stdout, with similar
    filtering and formatting to the `env_logger` crate.
  - `tracing-log` provides a compatibility layer with the [`log`](#log) crate,
    allowing log messages to be recorded as `tracing` `Event`s within the
    trace tree. This is useful when a project using `tracing` have
    dependencies which use `log`. Note that if you're using
    `tracing-subscriber`'s `FmtSubscriber`, you don't need to depend on
    `tracing-log` directly.
  - `tracing-appender` provides utilities for outputting tracing data,
    including a file appender and non blocking writer.

 Additionally, there are also several third-party crates which are not
 maintained by the `tokio` project. These include:

  - `tracing-timing` implements inter-event timing metrics on top of `tracing`.
    It provides a subscriber that records the time elapsed between pairs of
    `tracing` events and generates histograms.
  - `tracing-opentelemetry` provides a subscriber for emitting traces to
    [OpenTelemetry]-compatible distributed tracing systems.
  - `tracing-honeycomb` Provides a layer that reports traces spanning multiple machines to [honeycomb.io]. Backed by `tracing-distributed`.
  - `tracing-distributed` Provides a generic implementation of a layer that reports traces spanning multiple machines to some backend.
  - `tracing-actix-web` provides `tracing` integration for the `actix-web` web framework.
  - `tracing-actix` provides `tracing` integration for the `actix` actor
    framework.
  - `axum-insights` provides `tracing` integration and Application insights export for the `axum` web framework.
  - `tracing-gelf` implements a subscriber for exporting traces in Greylog
    GELF format.
  - `tracing-coz` provides integration with the [coz] causal profiler
    (Linux-only).
  - `tracing-bunyan-formatter` provides a layer implementation that reports events and spans
    in [bunyan] format, enriched with timing information.
  - `tracing-wasm` provides a `Subscriber`/`Layer` implementation that reports
    events and spans via browser `console.log` and [User Timing API (`window.performance`)].
  - `tracing-web` provides a layer implementation of level-aware logging of events
    to web browsers' `console.*` and span events to the [User Timing API (`window.performance`)].
  - `tide-tracing` provides a [tide] middleware to trace all incoming requests and responses.
  - `test-log` takes care of initializing `tracing` for tests, based on
    environment variables with an `env_logger` compatible syntax.
  - `tracing-unwrap` provides convenience methods to report failed unwraps
    on `Result` or `Option` types to a `Subscriber`.
  - `diesel-tracing` provides integration with `diesel` database connections.
  - `tracing-tracy` provides a way to collect [Tracy] profiles in instrumented
    applications.
  - `tracing-elastic-apm` provides a layer for reporting traces to [Elastic APM].
  - `tracing-etw` provides a layer for emitting Windows [ETW] events.
  - `tracing-fluent-assertions` provides a fluent assertions-style testing
    framework for validating the behavior of `tracing` spans.
  - `sentry-tracing` provides a layer for reporting events and traces to [Sentry].
  - `tracing-forest` provides a subscriber that preserves contextual coherence by
    grouping together logs from the same spans during writing.
  - `tracing-loki` provides a layer for shipping logs to [Grafana Loki].
  - `tracing-logfmt` provides a layer that formats events and spans into the logfmt format.
  - `reqwest-tracing` provides a middleware to trace `reqwest` HTTP requests.
  - `tracing-cloudwatch` provides a layer that sends events to AWS CloudWatch Logs.
  - `clippy-tracing` provides a tool to add, remove and check for `tracing::instrument`.
  - `json-subscriber` provides a subscriber for emitting JSON logs. The output can be customized much more than with `tracing-subscriber`'s JSON output.

 If you're the maintainer of a `tracing` ecosystem crate not listed above,
 please let us know! We'd love to add your project to the list!








































 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: Some of these ecosystem crates are currently
     unreleased and/or in earlier stages of development. They may be less stable
     than <code>tracing</code> and <code>tracing-core</code>.
 </pre>

 ## Crate Feature Flags

 The following crate [feature flags] are available:

 * A set of features controlling the [static verbosity level].
 * `log`: causes trace instrumentation points to emit [`log`](#log) records as well
   as trace events, if a default `tracing` subscriber has not been set. This
   is intended for use in libraries whose users may be using either `tracing`
   or `log`.
 * `log-always`: Emit `log` records from all `tracing` spans and events, even
   if a `tracing` subscriber has been set. This should be set only by
   applications which intend to collect traces and logs separately; if an
   adapter is used to convert `log` records into `tracing` events, this will
   cause duplicate events to occur.
 * `attributes`: Includes support for the `#[instrument]` attribute.
   This is on by default, but does bring in the `syn` crate as a dependency,
   which may add to the compile time of crates that do not already use it.
 * `std`: Depend on the Rust standard library (enabled by default).

   `no_std` users may disable this feature with `default-features = false`:

   ```toml
   [dependencies]
   tracing = { version = "0.1.38", default-features = false }
   ```

 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: <code>tracing</code>'s <code>no_std</code> support
     requires <code>liballoc</code>.
 </pre>

 ### Unstable Features

 These feature flags enable **unstable** features. The public API may break in 0.1.x
 releases. To enable these features, the `--cfg tracing_unstable` must be passed to
 `rustc` when compiling.

 The following unstable feature flags are currently available:

 * `valuable`: Enables support for recording [field values] using the
   `valuable` crate.

 #### Enabling Unstable Features

 The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
 env variable when running `cargo` commands:

 ```shell
 RUSTFLAGS="--cfg tracing_unstable" cargo build
 ```
 Alternatively, the following can be added to the `.cargo/config` file in a
 project to automatically enable the cfg flag for that project:

 ```toml
 [build]
 rustflags = ["--cfg", "tracing_unstable"]
 ```



 ## Supported Rust Versions

 Tracing is built against the latest stable release. The minimum supported
 version is 1.65. The current Tracing version is not guaranteed to build on
 Rust versions earlier than the minimum supported version.

 Tracing follows the same compiler support policies as the rest of the Tokio
 project. The current stable Rust compiler and the three most recent minor
 versions before it will always be supported. For example, if the current
 stable compiler version is 1.69, the minimum supported version will not be
 increased past 1.66, three minor versions prior. Increasing the minimum
 supported compiler version is not considered a semver breaking change as
 long as doing so complies with this policy.





























## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`dispatcher`](#dispatcher)
  - [`field`](#field)
  - [`instrument`](#instrument)
  - [`level_filters`](#level-filters)
  - [`span`](#span)
  - [`subscriber`](#subscriber)
  - [`sealed`](#sealed)
- [Structs](#structs)
  - [`Span`](#span)
- [Traits](#traits)
  - [`Instrument`](#instrument)
- [Functions](#functions)
  - [`Value`](#value)
  - [`Subscriber`](#subscriber)
- [Macros](#macros)
  - [`event!`](#event)
  - [`span!`](#span)
  - [`record_all!`](#record-all)
  - [`trace_span!`](#trace-span)
  - [`debug_span!`](#debug-span)
  - [`info_span!`](#info-span)
  - [`warn_span!`](#warn-span)
  - [`error_span!`](#error-span)
  - [`event_enabled!`](#event-enabled)
  - [`span_enabled!`](#span-enabled)
  - [`enabled!`](#enabled)
  - [`trace!`](#trace)
  - [`debug!`](#debug)
  - [`info!`](#info)
  - [`warn!`](#warn)
  - [`error!`](#error)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`dispatcher`](#dispatcher) | mod | Dispatches trace events to [`Subscriber`]s. |
| [`field`](#field) | mod | `Span` and `Event` key-value data. |
| [`instrument`](#instrument) | mod | Attach a span to a `std::future::Future`. |
| [`level_filters`](#level-filters) | mod | Trace verbosity level filtering. |
| [`span`](#span) | mod | Spans represent periods of time in which a program was executing in a particular context. |
| [`subscriber`](#subscriber) | mod | Collects and records trace data. |
| [`sealed`](#sealed) | mod |  |
| [`Span`](#span) | struct |  |
| [`Instrument`](#instrument) | trait |  |
| [`Value`](#value) | fn |  |
| [`Subscriber`](#subscriber) | fn |  |
| [`event!`](#event) | macro | Constructs a new `Event`. |
| [`span!`](#span) | macro | Constructs a new span. |
| [`record_all!`](#record-all) | macro | Records multiple values on a span in a single call. |
| [`trace_span!`](#trace-span) | macro | Constructs a span at the trace level. |
| [`debug_span!`](#debug-span) | macro | Constructs a span at the debug level. |
| [`info_span!`](#info-span) | macro | Constructs a span at the info level. |
| [`warn_span!`](#warn-span) | macro | Constructs a span at the warn level. |
| [`error_span!`](#error-span) | macro | Constructs a span at the error level. |
| [`event_enabled!`](#event-enabled) | macro | Tests whether an event with the specified level and target would be enabled. |
| [`span_enabled!`](#span-enabled) | macro | Tests whether a span with the specified level and target would be enabled. |
| [`enabled!`](#enabled) | macro | Checks whether a span or event is [enabled] based on the provided [metadata]. |
| [`trace!`](#trace) | macro | Constructs an event at the trace level. |
| [`debug!`](#debug) | macro | Constructs an event at the debug level. |
| [`info!`](#info) | macro | Constructs an event at the info level. |
| [`warn!`](#warn) | macro | Constructs an event at the warn level. |
| [`error!`](#error) | macro | Constructs an event at the error level. |

## Modules

- [`macros`](macros/index.md)
- [`dispatcher`](dispatcher/index.md) — Dispatches trace events to [`Subscriber`]s.
- [`field`](field/index.md) — `Span` and `Event` key-value data.
- [`instrument`](instrument/index.md) — Attach a span to a `std::future::Future`.
- [`level_filters`](level_filters/index.md) — Trace verbosity level filtering.
- [`span`](span/index.md) —  Spans represent periods of time in which a program was executing in a
- [`subscriber`](subscriber/index.md) — Collects and records trace data.
- [`sealed`](sealed/index.md)

## Structs

### `Span`

```rust
struct Span {
    inner: Option<Inner>,
    meta: Option<&'static crate::Metadata<'static>>,
}
```

*Defined in [`tracing-0.1.43/src/span.rs:349-359`](../../.source_1765633015/tracing-0.1.43/src/span.rs#L349-L359)*

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

- <span id="span-new"></span>`fn new(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Metadata`](#metadata), [`Span`](span/index.md#span)

  Constructs a new `Span` with the given [`metadata`](../tracing_core/metadata/index.md) and set of

  [field values].

  

  The new span will be constructed by the currently-active [`Subscriber`](../tracing_core/subscriber/index.md),

  with the current span as its parent (if one exists).

  

  After the span is constructed, [field values] and/or `follows_from`

  annotations may be added to it.

  

  

  

- <span id="span-new-root"></span>`fn new_root(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Metadata`](#metadata), [`Span`](span/index.md#span)

  Constructs a new `Span` as the root of its own trace tree, with the

  given [`metadata`](../tracing_core/metadata/index.md) and set of [field values].

  

  After the span is constructed, [field values] and/or `follows_from`

  annotations may be added to it.

  

  

- <span id="span-child-of"></span>`fn child_of(parent: impl Into<Option<Id>>, meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span` — [`Id`](span/index.md#id), [`Metadata`](#metadata), [`Span`](span/index.md#span)

  Constructs a new `Span` as child of the given parent span, with the

  given [`metadata`](../tracing_core/metadata/index.md) and set of [field values].

  

  After the span is constructed, [field values] and/or `follows_from`

  annotations may be added to it.

  

  

- <span id="span-new-disabled"></span>`fn new_disabled(meta: &'static Metadata<'static>) -> Span` — [`Metadata`](#metadata), [`Span`](span/index.md#span)

  Constructs a new disabled span with the given `Metadata`.

  

  This should be used when a span is constructed from a known callsite,

  but the subscriber indicates that it is disabled.

  

  Entering, exiting, and recording values on this span will not notify the

  `Subscriber` but _may_ record log messages if the `log` feature flag is

  enabled.

- <span id="span-none"></span>`const fn none() -> Span` — [`Span`](span/index.md#span)

  Constructs a new span that is *completely disabled*.

  

  This can be used rather than `Option<Span>` to represent cases where a

  span is not present.

  

  Entering, exiting, and recording values on this span will do nothing.

- <span id="span-current"></span>`fn current() -> Span` — [`Span`](span/index.md#span)

  Returns a handle to the span [considered by the `Subscriber`] to be the

  current span.

  

  If the subscriber indicates that it does not track the current span, or

  that the thread from which this function is called is not currently

  inside a span, the returned span will be disabled.

- <span id="span-make-with"></span>`fn make_with(meta: &'static Metadata<'static>, new_span: Attributes<'_>, dispatch: &Dispatch) -> Span` — [`Metadata`](#metadata), [`Attributes`](span/index.md#attributes), [`Dispatch`](dispatcher/index.md#dispatch), [`Span`](span/index.md#span)

- <span id="span-enter"></span>`fn enter(&self) -> Entered<'_>` — [`Entered`](span/index.md#entered)

  Enters this span, returning a guard that will exit the span when dropped.

  

  If this span is enabled by the current subscriber, then this function will

  call `Subscriber::enter` with the span's [`Id`](../tracing_core/span/index.md), and dropping the guard

  will call `Subscriber::exit`. If the span is disabled, this does

  nothing.

  

  # In Asynchronous Code

  

  **Warning**: in asynchronous code that uses [async/await syntax][`syntax`](../regex_automata/util/syntax/index.md),

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

    [`Future::instrument` combinator][`instrument`](instrument/index.md) for

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

  

  * The [`#[instrument]` attribute macro][`attr`](../clap_derive/attr/index.md) can automatically generate

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

  

  

- <span id="span-entered"></span>`fn entered(self) -> EnteredSpan` — [`EnteredSpan`](span/index.md#enteredspan)

  Enters this span, consuming it and returning a [guard][`EnteredSpan`](span/index.md)

  that will exit the span when dropped.

  

  <pre class="compile_fail" style="white-space:normal;font:inherit;">

      <strong>Warning</strong>: In asynchronous code that uses async/await syntax,

      <code>Span::entered</code> may produce incorrect traces if the returned drop

      guard is held across an await point. See <a href="#in-asynchronous-code">the

      <code>Span::enter</code> documentation</a> for details.

  </pre>

  

  

  If this span is enabled by the current subscriber, then this function will

  call `Subscriber::enter` with the span's [`Id`](../tracing_core/span/index.md), and dropping the guard

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

  

  Since the [`EnteredSpan`](span/index.md) guard can dereference to the [`Span`](span/index.md) itself,

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

  Returns this span, if it was [enabled](#enabled) by the current [`Subscriber`](../tracing_core/subscriber/index.md), or

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

  

  If the current [`Subscriber`](../tracing_core/subscriber/index.md) enables the `DEBUG` level, then both

  the "parent" and "child" spans will be enabled. Thus, when the "spawned

  a thread!" event occurs, it will be inside of the "child" span. Because

  "parent" is the parent of "child", the event will _also_ be inside of

  "parent".

  

  However, if the [`Subscriber`](../tracing_core/subscriber/index.md) only enables the `INFO` level, the "child"

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

  be used similarly, in combination with [`instrument`](instrument/index.md):

  

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

  [`instrument`](instrument/index.md)  call inside of an `in_current_span` call, as using

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

  

- <span id="span-follows-from"></span>`fn follows_from(&self, from: impl Into<Option<Id>>) -> &Self` — [`Id`](span/index.md#id)

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

- <span id="span-id"></span>`fn id(&self) -> Option<Id>` — [`Id`](span/index.md#id)

  Returns this span's `Id`, if it is enabled.

- <span id="span-metadata"></span>`fn metadata(&self) -> Option<&'static Metadata<'static>>` — [`Metadata`](#metadata)

  Returns this span's `Metadata`, if it is enabled.

- <span id="span-with-subscriber"></span>`fn with_subscriber<T>(&self, f: impl FnOnce((&Id, &Dispatch)) -> T) -> Option<T>` — [`Id`](span/index.md#id), [`Dispatch`](dispatcher/index.md#dispatch)

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

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](span/index.md#span)

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

## Traits

### `Instrument`

```rust
trait Instrument: Sized { ... }
```

*Defined in [`tracing-0.1.43/src/instrument.rs:20-131`](../../.source_1765633015/tracing-0.1.43/src/instrument.rs#L20-L131)*

Attaches spans to a `std::future::Future`.

Extension trait allowing futures to be
instrumented with a `tracing` [`span`](span/index.md).


#### Provided Methods

- `fn instrument(self, span: Span) -> Instrumented<Self>`

  Instruments this type with the provided [`Span`](span/index.md), returning an

- `fn in_current_span(self) -> Instrumented<Self>`

  Instruments this type with the [current] [`Span`](span/index.md), returning an

#### Implementors

- `T`

## Functions

### `Value`

```rust
fn Value(t: T) -> T
```

Returns the argument unchanged.

### `Subscriber`

```rust
fn Subscriber(&self) -> &T
```

## Macros

### `event!`

*Defined in [`tracing-0.1.43/src/macros.rs:615-1053`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L615-L1053)*

Constructs a new `Event`.

The event macro is invoked with a `Level` and up to 32 key-value fields.
Optionally, a format string and arguments may follow the fields; this will
be used to construct an implicit field named "message".

See [the top-level documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.

# Examples

```rust
use tracing::{event, Level};

fn main() {
let data = (42, "forty-two");
let private_data = "private";
let error = "a bad error";

event!(Level::ERROR, %error, "Received error");
event!(
    target: "app_events",
    Level::WARN,
    private_data,
    ?data,
    "App warning: {}",
    error
);
event!(name: "answer", Level::INFO, the_answer = data.0);
event!(Level::INFO, the_answer = data.0);
}
```


### `span!`

*Defined in [`tracing-0.1.43/src/macros.rs:20-131`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L20-L131)*

Constructs a new span.

See [the top-level documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.

# Examples

Creating a new span:
```rust
use tracing::{span, Level};
fn main() {
let span = span!(Level::TRACE, "my span");
let _enter = span.enter();
// do work inside the span...
}
```

### `record_all!`

*Defined in [`tracing-0.1.43/src/macros.rs:152-161`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L152-L161)*

Records multiple values on a span in a single call. As with recording
individual values, all fields must be declared when the span is created.

This macro supports two optional sigils:
- `%` uses the Display implementation.
- `?` uses the Debug implementation.

For more details, see the [top-level documentation][`lib`](../serde/lib/index.md).

# Examples

```rust
use tracing::{field, info_span, record_all};
let span = info_span!("my span", field1 = field::Empty, field2 = field::Empty, field3 = field::Empty).entered();
record_all!(span, field1 = ?"1", field2 = %"2", field3 = 3);
```

### `trace_span!`

*Defined in [`tracing-0.1.43/src/macros.rs:197-242`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L197-L242)*

Constructs a span at the trace level.

[Fields] and [`attributes`](../object/read/elf/attributes/index.md) are set using the same syntax as the `span!`
macro.

See [the top-level documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.




# Examples

```rust
use tracing::{trace_span, span, Level};
fn main() {
trace_span!("my_span");
// is equivalent to:
span!(Level::TRACE, "my_span");
}
```

```rust
use tracing::{trace_span, span, Level};
fn main() {
let span = trace_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
}
```

### `debug_span!`

*Defined in [`tracing-0.1.43/src/macros.rs:278-323`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L278-L323)*

Constructs a span at the debug level.

[Fields] and [`attributes`](../object/read/elf/attributes/index.md) are set using the same syntax as the `span!`
macro.

See [the top-level documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.




# Examples

```rust
use tracing::{debug_span, span, Level};
fn main() {
debug_span!("my_span");
// is equivalent to:
span!(Level::DEBUG, "my_span");
}
```

```rust
use tracing::debug_span;
fn main() {
let span = debug_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
}
```

### `info_span!`

*Defined in [`tracing-0.1.43/src/macros.rs:359-404`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L359-L404)*

Constructs a span at the info level.

[Fields] and [`attributes`](../object/read/elf/attributes/index.md) are set using the same syntax as the `span!`
macro.

See [the top-level documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.




# Examples

```rust
use tracing::{span, info_span, Level};
fn main() {
info_span!("my_span");
// is equivalent to:
span!(Level::INFO, "my_span");
}
```

```rust
use tracing::info_span;
fn main() {
let span = info_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
}
```

### `warn_span!`

*Defined in [`tracing-0.1.43/src/macros.rs:440-485`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L440-L485)*

Constructs a span at the warn level.

[Fields] and [`attributes`](../object/read/elf/attributes/index.md) are set using the same syntax as the `span!`
macro.

See [the top-level documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.




# Examples

```rust
use tracing::{warn_span, span, Level};
fn main() {
warn_span!("my_span");
// is equivalent to:
span!(Level::WARN, "my_span");
}
```

```rust
use tracing::warn_span;
fn main() {
let span = warn_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
}
```

### `error_span!`

*Defined in [`tracing-0.1.43/src/macros.rs:520-565`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L520-L565)*

Constructs a span at the error level.

[Fields] and [`attributes`](../object/read/elf/attributes/index.md) are set using the same syntax as the `span!`
macro.

See [the top-level documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.




# Examples

```rust
use tracing::{span, error_span, Level};
fn main() {
error_span!("my_span");
// is equivalent to:
span!(Level::ERROR, "my_span");
}
```

```rust
use tracing::error_span;
fn main() {
let span = error_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
}
```

### `event_enabled!`

*Defined in [`tracing-0.1.43/src/macros.rs:1083-1087`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L1083-L1087)*

Tests whether an event with the specified level and target would be enabled.

This is similar to `enabled!`, but queries the current subscriber specifically for
an event, whereas `enabled!` queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also `span_enabled!`.

# Examples

```rust
use tracing::{event_enabled, Level};
if event_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if event_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if event_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```



### `span_enabled!`

*Defined in [`tracing-0.1.43/src/macros.rs:1117-1121`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L1117-L1121)*

Tests whether a span with the specified level and target would be enabled.

This is similar to `enabled!`, but queries the current subscriber specifically for
an event, whereas `enabled!` queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also `span_enabled!`.

# Examples

```rust
use tracing::{span_enabled, Level};
if span_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if span_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if span_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```



### `enabled!`

*Defined in [`tracing-0.1.43/src/macros.rs:1214-1294`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L1214-L1294)*

Checks whether a span or event is [enabled](#enabled) based on the provided [`metadata`](../tracing_core/metadata/index.md).


This macro is a specialized tool: it is intended to be used prior
to an expensive computation required *just* for that event, but
*cannot* be done as part of an argument to that event, such as
when multiple events are emitted (e.g., iterating over a collection
and emitting an event for each item).

# Usage

[Subscribers] can make filtering decisions based all the data included in a
span or event's [`Metadata`](../tracing_core/metadata/index.md). This means that it is possible for `enabled!`
to return a _false positive_ (indicating that something would be enabled
when it actually would not be) or a _false negative_ (indicating that
something would be disabled when it would actually be enabled).


This occurs when a subscriber is using a _more specific_ filter than the
metadata provided to the `enabled!` macro. Some situations that can result
in false positives or false negatives include:

- If a subscriber is using a filter which may enable a span or event based
  on field names, but `enabled!` is invoked without listing field names,
  `enabled!` may return a false negative if a specific field name would
  cause the subscriber to enable something that would otherwise be disabled.
- If a subscriber is using a filter which enables or disables specific events by
  file path and line number,  a particular event may be enabled/disabled
  even if an `enabled!` invocation with the same level, target, and fields
  indicated otherwise.
- The subscriber can choose to enable _only_ spans or _only_ events, which `enabled`
  will not reflect.

`enabled!()` requires a [level](crate::Level) argument, an optional `target:`
argument, and an optional set of field names. If the fields are not provided,
they are considered to be unknown. `enabled!` attempts to match the
syntax of `event!()` as closely as possible, which can be seen in the
examples below.

# Examples

If the current subscriber is interested in recording `DEBUG`-level spans and
events in the current file and module path, this will evaluate to true:
```rust
use tracing::{enabled, Level};

if enabled!(Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", and at the
level  `DEBUG`, this will evaluate to true:
```rust
use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", at
the level `DEBUG`, and with a field named "hello", this will evaluate
to true:

```rust
use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG, hello) {
    // some expensive work...
}
```

# Alternatives

`enabled!` queries subscribers with [`Metadata`](../tracing_core/metadata/index.md) where
`is_event` and `is_span` both return `false`. Alternatively,
use [`event_enabled!`](#event-enabled) or `span_enabled!` to ensure one of these
returns true.






### `trace!`

*Defined in [`tracing-0.1.43/src/macros.rs:1333-1583`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L1333-L1583)*

Constructs an event at the trace level.

This functions similarly to the `event!` macro. See [the top-level
documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.


# Examples

```rust
use tracing::trace;
#[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
impl Position {
const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
fn dist(&self, other: Position) -> f32 {
   let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
   (x + y).sqrt()
}
}
fn main() {
let pos = Position { x: 3.234, y: -1.223 };
let origin_dist = pos.dist(Position::ORIGIN);

trace!(position = ?pos, ?origin_dist);
trace!(
    target: "app_events",
    position = ?pos,
    "x is {} and y is {}",
    if pos.x >= 0.0 { "positive" } else { "negative" },
    if pos.y >= 0.0 { "positive" } else { "negative" }
);
trace!(name: "completed", position = ?pos);
}
```

### `debug!`

*Defined in [`tracing-0.1.43/src/macros.rs:1609-1859`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L1609-L1859)*

Constructs an event at the debug level.

This functions similarly to the `event!` macro. See [the top-level
documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.


# Examples

```rust
use tracing::debug;
fn main() {
#[derive(Debug)] struct Position { x: f32, y: f32 }

let pos = Position { x: 3.234, y: -1.223 };

debug!(?pos.x, ?pos.y);
debug!(target: "app_events", position = ?pos, "New position");
debug!(name: "completed", position = ?pos);
}
```

### `info!`

*Defined in [`tracing-0.1.43/src/macros.rs:1896-2146`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L1896-L2146)*

Constructs an event at the info level.

This functions similarly to the `event!` macro. See [the top-level
documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.


# Examples

```rust
use tracing::info;
// this is so the test will still work in no-std mode
#[derive(Debug)]
pub struct Ipv4Addr;
impl Ipv4Addr { fn new(o1: u8, o2: u8, o3: u8, o4: u8) -> Self { Self } }
fn main() {
struct Connection { port: u32, speed: f32 }
use tracing::field;

let addr = Ipv4Addr::new(127, 0, 0, 1);
let conn = Connection { port: 40, speed: 3.20 };

info!(conn.port, "connected to {:?}", addr);
info!(
    target: "connection_events",
    ip = ?addr,
    conn.port,
    ?conn.speed,
);
info!(name: "completed", "completed connection to {:?}", addr);
}
```

### `warn!`

*Defined in [`tracing-0.1.43/src/macros.rs:2176-2426`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L2176-L2426)*

Constructs an event at the warn level.

This functions similarly to the `event!` macro. See [the top-level
documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.


# Examples

```rust
use tracing::warn;
fn main() {

let warn_description = "Invalid Input";
let input = &[0x27, 0x45];

warn!(?input, warning = warn_description);
warn!(
    target: "input_events",
    warning = warn_description,
    "Received warning for input: {:?}", input,
);
warn!(name: "invalid", ?input);
}
```

### `error!`

*Defined in [`tracing-0.1.43/src/macros.rs:2452-2702`](../../.source_1765633015/tracing-0.1.43/src/macros.rs#L2452-L2702)*

Constructs an event at the error level.

This functions similarly to the `event!` macro. See [the top-level
documentation][`lib`](../serde/lib/index.md) for details on the syntax accepted by
this macro.


# Examples

```rust
use tracing::error;
fn main() {

let (err_info, port) = ("No connection", 22);

error!(port, error = %err_info);
error!(target: "app_events", "App Error: {}", err_info);
error!({ info = err_info }, "error on port: {}", port);
error!(name: "invalid_input", "Invalid input: {}", err_info);
}
```

