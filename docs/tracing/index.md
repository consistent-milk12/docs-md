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

 *Compiler support: [requires `rustc` 1.65+][msrv](#msrv)*

 [msrv](#msrv): #supported-rust-versions
 # Core Concepts

 The core of `tracing`'s API is composed of _spans_, _events_ and
 _subscribers_. We'll cover these in turn.

 ## Spans

 To record the flow of execution through a program, `tracing` introduces the
 concept of [spans](#spans). Unlike a log line that represents a _moment in
 time_, a span represents a _period of time_ with a beginning and an end. When a
 program begins executing in a context or performing a unit of work, it
 _enters_ that context's span, and when it stops executing in that context,
 it _exits_ the span. The span in which a thread is currently executing is
 referred to as that thread's _current_ span.

 For example:
 ```
 use tracing::{span, Level};
 # fn main() {
 let span = span!(Level::TRACE, "my_span");
 // `enter` returns a RAII guard which, when dropped, exits the span. this
 // indicates that we are in the span for the current lexical scope.
 let _enter = span.enter();
 // perform some work in the context of `my_span`...
 # }
```

 The [`span` module][span](#span)'s documentation provides further details on how to
 use spans.

 <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">

  **Warning**: In asynchronous code that uses async/await syntax,
  `Span::enter` may produce incorrect traces if the returned drop
  guard is held across an await point. See
  [the method documentation][Span#in-asynchronous-code] for details.

 </pre></div>

 ## Events

 An [`Event`](../tracing_core/tracing_core/event/index.md) represents a _moment_ in time. It signifies something that
 happened while a trace was being recorded. `Event`s are comparable to the log
 records emitted by unstructured logging code, but unlike a typical log line,
 an `Event` may occur within the context of a span.

 For example:
 ```
 use tracing::{event, span, Level};

 # fn main() {
 // records an event outside of any span context:
 event!(Level::INFO, "something happened");

 let span = span!(Level::INFO, "my_span");
 let _guard = span.enter();

 // records an event within "my_span".
 event!(Level::DEBUG, "something happened inside my_span");
 # }
```

 In general, events should be used to represent points in time _within_ a
 span — a request returned with a given status code, _n_ new items were
 taken from a queue, and so on.

 The [`Event` struct][`Event`](../tracing_core/tracing_core/event/index.md) documentation provides further details on using
 events.

 ## Subscribers

 As `Span`s and `Event`s occur, they are recorded or aggregated by
 implementations of the [`Subscriber`](../tracing_core/tracing_core/subscriber/index.md) trait. `Subscriber`s are notified
 when an `Event` takes place and when a `Span` is entered or exited. These
 notifications are represented by the following `Subscriber` trait methods:

 + [`event`](#event), called when an `Event` takes place,
 + [`enter`](#enter), called when execution enters a `Span`,
 + [`exit`](../rustix/rustix/not_implemented/libc_internals/index.md), called when execution exits a `Span`

 In addition, subscribers may implement the [`enabled`](tracing/index.md) function to _filter_
 the notifications they receive based on [metadata](#metadata) describing each `Span`
 or `Event`. If a call to `Subscriber::enabled` returns `false` for a given
 set of metadata, that `Subscriber` will *not* be notified about the
 corresponding `Span` or `Event`. For performance reasons, if no currently
 active subscribers express interest in a given set of metadata by returning
 `true`, then the corresponding `Span` or `Event` will never be constructed.

 # Usage

 First, add this to your `Cargo.toml`:

 ```toml
 [dependencies](#dependencies)
 tracing = "0.1"
 ```

 ## Recording Spans and Events

 Spans and events are recorded using macros.

 ### Spans

 The [`span`](tracing/index.md) macro expands to a [`Span` struct][`Span`](tracing/span/index.md) which is used to
 record a span. The [`Span::enter`](#enter) method on that struct records that the
 span has been entered, and returns a [RAII] guard object, which will exit
 the span when dropped.

 For example:

 ```rust
 use tracing::{span, Level};
 # fn main() {
 // Construct a new span named "my span" with trace log level.
 let span = span!(Level::TRACE, "my span");

 // Enter the span, returning a guard object.
 let _enter = span.enter();

 // Any trace events that occur before the guard is dropped will occur
 // within the span.

 // Dropping the guard will exit the span.
 # }
 ```

 The [`#[instrument](#instrument)`][instrument](#instrument) attribute provides an easy way to
 add `tracing` spans to functions. A function annotated with `#[instrument](#instrument)`
 will create and enter a span with that function's name every time the
 function is called, with arguments to that function will be recorded as
 fields using `fmt::Debug`.

 For example:
 ```ignore
 # // this doctest is ignored because we don't have a way to say
 # // that it should only be run with cfg(feature = "attributes")
 use tracing::{Level, event, instrument};

 #[instrument](#instrument)
 pub fn my_function(my_arg: usize) {
     // This event will be recorded inside a span named `my_function` with the
     // field `my_arg`.
     event!(Level::INFO, "inside my_function!");
     // ...
 }
 # fn main() {}
 ```

 For functions which don't have built-in tracing support and can't have
 the `#[instrument](#instrument)` attribute applied (such as from an external crate),
 the [`Span` struct][`Span`](tracing/span/index.md) has a [`in_scope()` method][`in_scope`](#in-scope)
 which can be used to easily wrap synchronous code in a span.

 For example:
 ```rust
 use tracing::info_span;

 # fn doc() -> Result<(), ()> {
 # mod serde_json {
 #    pub(crate) fn from_slice(buf: &[u8](#u8)) -> Result<(), ()> { Ok(()) }
 # }
 # let buf: [u8; 0] = [];
 let json = info_span!("json.parse").in_scope(|| serde_json::from_slice(&buf))?;
 # let _ = json; // suppress unused variable warning
 # Ok(())
 # }
 ```

 You can find more examples showing how to use this crate [here][examples](#examples).

 [RAII]: https://github.com/rust-unofficial/patterns/blob/main/src/patterns/behavioural/RAII.md
 [examples](#examples): https://github.com/tokio-rs/tracing/tree/main/examples

 ### Events

 [`Event`](../tracing_core/tracing_core/event/index.md)s are recorded using the [`event`](tracing/index.md) macro:

 ```rust
 # fn main() {
 use tracing::{event, Level};
 event!(Level::INFO, "something has happened!");
 # }
 ```

 ## Using the Macros

 The [`span`](tracing/index.md) and [`event`](tracing/index.md) macros as well as the `#[instrument](#instrument)` attribute
 use fairly similar syntax, with some exceptions.

 ### Configuring Attributes

 Both macros require a [`Level`](../log/log/index.md) specifying the verbosity of the span or
 event. Optionally, the, [target](#target) and [parent span] may be overridden. If the
 target and parent span are not overridden, they will default to the
 module path where the macro was invoked and the current span (as determined
 by the subscriber), respectively.

 For example:

 ```
 # use tracing::{span, event, Level};
 # fn main() {
 span!(target: "app_spans", Level::TRACE, "my span");
 event!(target: "app_events", Level::INFO, "something has happened!");
 # }
 ```
 ```
 # use tracing::{span, event, Level};
 # fn main() {
 let span = span!(Level::TRACE, "my span");
 event!(parent: &span, Level::INFO, "something has happened!");
 # }
 ```

 The span macros also take a string literal after the level, to set the name
 of the span (as above).  In the case of the event macros, the name of the event can
 be overridden (the default is `event file:line`) using the `name:` specifier.

 ```
 # use tracing::{span, event, Level};
 # fn main() {
 span!(Level::TRACE, "my span");
 event!(name: "some_info", Level::INFO, "something has happened!");
 # }
 ```

 ### Recording Fields

 Structured fields on spans and events are specified using the syntax
 `field_name = field_value`. Fields are separated by commas.

 ```
 # use tracing::{event, Level};
 # fn main() {
 // records an event with two fields:
 //  - "answer", with the value 42
 //  - "question", with the value "life, the universe and everything"
 event!(Level::INFO, answer = 42, question = "life, the universe, and everything");
 # }
 ```

 As shorthand, local variables may be used as field values without an
 assignment, similar to [struct initializers]. For example:

 ```
 # use tracing::{span, Level};
 # fn main() {
 let user = "ferris";

 span!(Level::TRACE, "login", user);
 // is equivalent to:
 span!(Level::TRACE, "login", user = user);
 # }
```

 Field names can include dots, but should not be terminated by them:
 ```
 # use tracing::{span, Level};
 # fn main() {
 let user = "ferris";
 let email = "ferris@rust-lang.org";
 span!(Level::TRACE, "login", user, user.email = email);
 # }
```

 Since field names can include dots, fields on local structs can be used
 using the local variable shorthand:
 ```
 # use tracing::{span, Level};
 # fn main() {
 # struct User {
 #    name: &'static str,
 #    email: &'static str,
 # }
 let user = User {
     name: "ferris",
     email: "ferris@rust-lang.org",
 };
 // the span will have the fields `user.name = "ferris"` and
 // `user.email = "ferris@rust-lang.org"`.
 span!(Level::TRACE, "login", user.name, user.email);
 # }
```

 Fields with names that are not Rust identifiers, or with names that are Rust reserved words,
 may be created using quoted string literals. However, this may not be used with the local
 variable shorthand.
 ```
 # use tracing::{span, Level};
 # fn main() {
 // records an event with fields whose names are not Rust identifiers
 //  - "guid:x-request-id", containing a `:`, with the value "abcdef"
 //  - "type", which is a reserved word, with the value "request"
 span!(Level::TRACE, "api", "guid:x-request-id" = "abcdef", "type" = "request");
 # }
```

 Constant expressions can also be used as field names. Constants
 must be enclosed in curly braces (`{}`) to indicate that the *value*
 of the constant is to be used as the field name, rather than the
 constant's name. For example:
 ```
 # use tracing::{span, Level};
 # fn main() {
 const RESOURCE_NAME: &str = "foo";
 // this span will have the field `foo = "some_id"`
 span!(Level::TRACE, "get", { RESOURCE_NAME } = "some_id");
 # }
```

 The `?` sigil is shorthand that specifies a field should be recorded using
 its [`fmt::Debug`](#debug) implementation:
 ```
 # use tracing::{event, Level};
 # fn main() {
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
 # }
 ```

 The `%` sigil operates similarly, but indicates that the value should be
 recorded using its [`fmt::Display`](#display) implementation:
 ```
 # use tracing::{event, Level};
 # fn main() {
 # #[derive(Debug)]
 # struct MyStruct {
 #     field: &'static str,
 # }
 #
 # let my_struct = MyStruct {
 #     field: "Hello world!"
 # };
 // `my_struct.field` will be recorded using its `fmt::Display` implementation.
 event!(Level::TRACE, greeting = %my_struct.field);
 // is equivalent to:
 event!(Level::TRACE, greeting = tracing::field::display(&my_struct.field));
 # }
 ```

 The `%` and `?` sigils may also be used with local variable shorthand:

 ```
 # use tracing::{event, Level};
 # fn main() {
 # #[derive(Debug)]
 # struct MyStruct {
 #     field: &'static str,
 # }
 #
 # let my_struct = MyStruct {
 #     field: "Hello world!"
 # };
 // `my_struct.field` will be recorded using its `fmt::Display` implementation.
 event!(Level::TRACE, %my_struct.field);
 # }
 ```

 Additionally, a span may declare fields with the special value [`Empty`](../tracing_core/tracing_core/field/index.md),
 which indicates that that the value for that field does not currently exist
 but may be recorded later. For example:

 ```
 use tracing::{trace_span, field};

 // Create a span with two fields: `greeting`, with the value "hello world", and
 // `parting`, without a value.
 let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);

 // ...

 // Now, record a value for parting as well.
 span.record("parting", &"goodbye world!");
 ```

 Finally, events may also include human-readable messages, in the form of a
 [format string][fmt](#fmt) and (optional) arguments, **after** the event's
 key-value fields. If a format string and arguments are provided,
 they will implicitly create a new field named `message` whose value is the
 provided set of format arguments.

 For example:

 ```
 # use tracing::{event, Level};
 # fn main() {
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
 # }
 ```

 Specifying a formatted message in this manner does not allocate by default.

 [struct initializers]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name
 [target](#target): Metadata::target
 [parent span]: span::Attributes::parent
 [determined contextually]: span::Attributes::is_contextual


 [fmt](#fmt): std::fmt#usage

 ### Shorthand Macros

 `tracing` also offers a number of macros with preset verbosity levels.
 The [`trace`](tracing/index.md), [`debug`](tracing/index.md), [`info`](tracing/index.md), [`warn`](tracing/index.md), and [`error`](tracing/index.md) behave
 similarly to the [`event`](tracing/index.md) macro, but with the [`Level`](../log/log/index.md) argument already
 specified, while the corresponding [`trace_span`](tracing/index.md), [`debug_span`](tracing/index.md),
 [`info_span`](tracing/index.md), [`warn_span`](tracing/index.md), and [`error_span`](tracing/index.md) macros are the same,
 but for the [`span`](tracing/index.md) macro.

 These are intended both as a shorthand, and for compatibility with the [`log`](../log/index.md)
 crate (see the next section).












 ### For `log` Users

 Users of the [`log`](../log/index.md) crate should note that `tracing` exposes a set of
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

 The simplest way to use a subscriber is to call the [`set_global_default`](tracing/subscriber/index.md)
 function:

 ```
 extern crate tracing;
 # pub struct FooSubscriber;
 # use tracing::{span::{Id, Attributes, Record}, Metadata};
 # impl tracing::Subscriber for FooSubscriber {
 #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
 #   fn record(&self, _: &Id, _: &Record) {}
 #   fn event(&self, _: &tracing::Event) {}
 #   fn record_follows_from(&self, _: &Id, _: &Id) {}
 #   fn enabled(&self, _: &Metadata) -> bool { false }
 #   fn enter(&self, _: &Id) {}
 #   fn exit(&self, _: &Id) {}
 # }
 # impl FooSubscriber {
 #   fn new() -> Self { FooSubscriber }
 # }
 # fn main() {

 let my_subscriber = FooSubscriber::new();
 tracing::subscriber::set_global_default(my_subscriber)
     .expect("setting tracing default failed");
 # }
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
 [`with_default`](tracing/subscriber/index.md) function. This follows the `tokio` pattern of using
 closures to represent executing code in a context that is exited at the end
 of the closure. For example:

 ```rust
 # pub struct FooSubscriber;
 # use tracing::{span::{Id, Attributes, Record}, Metadata};
 # impl tracing::Subscriber for FooSubscriber {
 #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
 #   fn record(&self, _: &Id, _: &Record) {}
 #   fn event(&self, _: &tracing::Event) {}
 #   fn record_follows_from(&self, _: &Id, _: &Id) {}
 #   fn enabled(&self, _: &Metadata) -> bool { false }
 #   fn enter(&self, _: &Id) {}
 #   fn exit(&self, _: &Id) {}
 # }
 # impl FooSubscriber {
 #   fn new() -> Self { FooSubscriber }
 # }
 # fn main() {

 let my_subscriber = FooSubscriber::new();
 # #[cfg(feature = "std")]
 tracing::subscriber::with_default(my_subscriber, || {
     // Any trace events generated in this closure or by functions it calls
     // will be collected by `my_subscriber`.
 })
 # }
 ```

 This approach allows trace data to be collected by multiple subscribers
 within different contexts in the program. Note that the override only applies to the
 currently executing thread; other threads will not see the change from with_default.

 Any trace events generated outside the context of a subscriber will not be collected.

 Once a subscriber has been set, instrumentation points may be added to the
 executable using the `tracing` crate's macros.

 ## `log` Compatibility

 The [`log`](../log/index.md) crate provides a simple, lightweight logging facade for Rust.
 While `tracing` builds upon `log`'s foundation with richer structured
 diagnostic data, `log`'s simplicity and ubiquity make it the "lowest common
 denominator" for text-based logging in Rust — a vast majority of Rust
 libraries and applications either emit or consume `log` records. Therefore,
 `tracing` provides multiple forms of interoperability with `log`: `tracing`
 instrumentation can emit `log` records, and a compatibility layer enables
 `tracing` [`Subscriber`](../tracing_core/tracing_core/subscriber/index.md)s to consume `log` records as `tracing` [`Event`](../tracing_core/tracing_core/event/index.md)s.

 ### Emitting `log` Records

 This crate provides two feature flags, "log" and "log-always", which will
 cause [spans](#spans) and [events](#events) to emit `log` records. When the "log" feature is
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

 See [here][flags](#flags) for more details on this crate's feature flags.

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

 The [`tracing-log`](#tracing-log) crate provides a compatibility layer which
 allows a `tracing` [`Subscriber`](../tracing_core/tracing_core/subscriber/index.md) to consume `log` records as though they
 were `tracing` [events](#events). This allows applications using `tracing` to record
 the logs emitted by dependencies using `log` as events within the context of
 the application's trace tree. See [that crate's documentation][log-tracer]
 for details.

 [log-tracer]: https://docs.rs/tracing-log/latest/tracing_log/#convert-log-records-to-tracing-events

 ## Related Crates

 In addition to `tracing` and `tracing-core`, the [`tokio-rs/tracing`](#tokio-rstracing) repository
 contains several additional crates designed to be used with the `tracing` ecosystem.
 This includes a collection of `Subscriber` implementations, as well as utility
 and adapter crates to assist in writing `Subscriber`s and instrumenting
 applications.

 In particular, the following crates are likely to be of interest:

  - [`tracing-futures`](#tracing-futures) provides a compatibility layer with the `futures`
    crate, allowing spans to be attached to `Future`s, `Stream`s, and `Executor`s.
  - [`tracing-subscriber`](#tracing-subscriber) provides `Subscriber` implementations and
    utilities for working with `Subscriber`s. This includes a [`FmtSubscriber`](#fmtsubscriber)
    `FmtSubscriber` for logging formatted trace data to stdout, with similar
    filtering and formatting to the [`env_logger`](#env-logger) crate.
  - [`tracing-log`](#tracing-log) provides a compatibility layer with the [`log`](../log/index.md) crate,
    allowing log messages to be recorded as `tracing` `Event`s within the
    trace tree. This is useful when a project using `tracing` have
    dependencies which use `log`. Note that if you're using
    `tracing-subscriber`'s `FmtSubscriber`, you don't need to depend on
    `tracing-log` directly.
  - [`tracing-appender`](#tracing-appender) provides utilities for outputting tracing data,
    including a file appender and non blocking writer.

 Additionally, there are also several third-party crates which are not
 maintained by the `tokio` project. These include:

  - [`tracing-timing`](#tracing-timing) implements inter-event timing metrics on top of `tracing`.
    It provides a subscriber that records the time elapsed between pairs of
    `tracing` events and generates histograms.
  - [`tracing-opentelemetry`](#tracing-opentelemetry) provides a subscriber for emitting traces to
    [OpenTelemetry]-compatible distributed tracing systems.
  - [`tracing-honeycomb`](#tracing-honeycomb) Provides a layer that reports traces spanning multiple machines to [honeycomb.io]. Backed by [`tracing-distributed`](#tracing-distributed).
  - [`tracing-distributed`](#tracing-distributed) Provides a generic implementation of a layer that reports traces spanning multiple machines to some backend.
  - [`tracing-actix-web`](#tracing-actix-web) provides `tracing` integration for the `actix-web` web framework.
  - [`tracing-actix`](#tracing-actix) provides `tracing` integration for the `actix` actor
    framework.
  - [`axum-insights`](#axum-insights) provides `tracing` integration and Application insights export for the `axum` web framework.
  - [`tracing-gelf`](#tracing-gelf) implements a subscriber for exporting traces in Greylog
    GELF format.
  - [`tracing-coz`](#tracing-coz) provides integration with the [coz](#coz) causal profiler
    (Linux-only).
  - [`tracing-bunyan-formatter`](#tracing-bunyan-formatter) provides a layer implementation that reports events and spans
    in [bunyan](#bunyan) format, enriched with timing information.
  - [`tracing-wasm`](#tracing-wasm) provides a `Subscriber`/`Layer` implementation that reports
    events and spans via browser `console.log` and [User Timing API (`window.performance`)].
  - [`tracing-web`](#tracing-web) provides a layer implementation of level-aware logging of events
    to web browsers' `console.*` and span events to the [User Timing API (`window.performance`)].
  - [`tide-tracing`](#tide-tracing) provides a [tide](#tide) middleware to trace all incoming requests and responses.
  - [`test-log`](#test-log) takes care of initializing `tracing` for tests, based on
    environment variables with an `env_logger` compatible syntax.
  - [`tracing-unwrap`](#tracing-unwrap) provides convenience methods to report failed unwraps
    on `Result` or `Option` types to a `Subscriber`.
  - [`diesel-tracing`](#diesel-tracing) provides integration with [`diesel`](#diesel) database connections.
  - [`tracing-tracy`](#tracing-tracy) provides a way to collect [Tracy] profiles in instrumented
    applications.
  - [`tracing-elastic-apm`](#tracing-elastic-apm) provides a layer for reporting traces to [Elastic APM].
  - [`tracing-etw`](#tracing-etw) provides a layer for emitting Windows [ETW] events.
  - [`tracing-fluent-assertions`](#tracing-fluent-assertions) provides a fluent assertions-style testing
    framework for validating the behavior of `tracing` spans.
  - [`sentry-tracing`](#sentry-tracing) provides a layer for reporting events and traces to [Sentry].
  - [`tracing-forest`](#tracing-forest) provides a subscriber that preserves contextual coherence by
    grouping together logs from the same spans during writing.
  - [`tracing-loki`](#tracing-loki) provides a layer for shipping logs to [Grafana Loki].
  - [`tracing-logfmt`](#tracing-logfmt) provides a layer that formats events and spans into the logfmt format.
  - [`reqwest-tracing`](#reqwest-tracing) provides a middleware to trace [`reqwest`](#reqwest) HTTP requests.
  - [`tracing-cloudwatch`](#tracing-cloudwatch) provides a layer that sends events to AWS CloudWatch Logs.
  - [`clippy-tracing`](#clippy-tracing) provides a tool to add, remove and check for `tracing::instrument`.
  - [`json-subscriber`](#json-subscriber) provides a subscriber for emitting JSON logs. The output can be customized much more than with [`tracing-subscriber`](#tracing-subscriber)'s JSON output.

 If you're the maintainer of a `tracing` ecosystem crate not listed above,
 please let us know! We'd love to add your project to the list!

 [OpenTelemetry]: https://opentelemetry.io/


 [honeycomb.io]: https://www.honeycomb.io/





 [coz](#coz): https://github.com/plasma-umass/coz

 [bunyan](#bunyan): https://github.com/trentm/node-bunyan


 [User Timing API (`window.performance`)]: https://developer.mozilla.org/en-US/docs/Web/API/User_Timing_API

 [tide](#tide): https://crates.io/crates/tide





 [Tracy]: https://github.com/wolfpld/tracy

 [Elastic APM]: https://www.elastic.co/apm

 [ETW]: https://docs.microsoft.com/en-us/windows/win32/etw/about-event-tracing


 [Sentry]: https://sentry.io/welcome/


 [Grafana Loki]: https://grafana.com/oss/loki/






 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: Some of these ecosystem crates are currently
     unreleased and/or in earlier stages of development. They may be less stable
     than <code>tracing</code> and <code>tracing-core</code>.
 </pre>

 ## Crate Feature Flags

 The following crate [feature flags] are available:

 * A set of features controlling the [static verbosity level].
 * `log`: causes trace instrumentation points to emit [`log`](../log/index.md) records as well
   as trace events, if a default `tracing` subscriber has not been set. This
   is intended for use in libraries whose users may be using either `tracing`
   or `log`.
 * `log-always`: Emit `log` records from all `tracing` spans and events, even
   if a `tracing` subscriber has been set. This should be set only by
   applications which intend to collect traces and logs separately; if an
   adapter is used to convert `log` records into `tracing` events, this will
   cause duplicate events to occur.
 * `attributes`: Includes support for the `#[instrument](#instrument)` attribute.
   This is on by default, but does bring in the `syn` crate as a dependency,
   which may add to the compile time of crates that do not already use it.
 * `std`: Depend on the Rust standard library (enabled by default).

   `no_std` users may disable this feature with `default-features = false`:

   ```toml
   [dependencies](#dependencies)
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
   [`valuable`](#valuable) crate.

 #### Enabling Unstable Features

 The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
 env variable when running `cargo` commands:

 ```shell
 RUSTFLAGS="--cfg tracing_unstable" cargo build
 ```
 Alternatively, the following can be added to the `.cargo/config` file in a
 project to automatically enable the cfg flag for that project:

 ```toml
 [build](#build)
 rustflags = ["--cfg", "tracing_unstable"]
 ```

 [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
 [field values]: crate::field

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

 [span](#span): mod@span
 [spans](#spans): mod@span


 [event](#event): Event
 [events](#events): Event

 [Subscriber::event]: subscriber::Subscriber::event



 [metadata](#metadata): Metadata












 [static verbosity level]: level_filters#compile-time-filters
 [instrument](#instrument): https://docs.rs/tracing-attributes/latest/tracing_attributes/attr.instrument.html
 [flags](#flags): #crate-feature-flags

## Modules

- [`dispatcher`](dispatcher/index.md) - Dispatches trace events to [`Subscriber`]s.
- [`field`](field/index.md) - `Span` and `Event` key-value data.
- [`instrument`](instrument/index.md) - Attach a span to a `std::future::Future`.
- [`level_filters`](level_filters/index.md) - Trace verbosity level filtering.
- [`span`](span/index.md) -  Spans represent periods of time in which a program was executing in a
- [`subscriber`](subscriber/index.md) - Collects and records trace data.

## Structs

### `Metadata`

```rust
struct Metadata {
}
```

An automaton for searching multiple strings in linear time.

The `AhoCorasick` type supports a few basic ways of constructing an
automaton, with the default being [`AhoCorasick::new`](#new). However, there
are a fair number of configurable options that can be set by using
[`AhoCorasickBuilder`](#ahocorasickbuilder) instead. Such options include, but are not limited
to, how matches are determined, simple case insensitivity, whether to use a
DFA or not and various knobs for controlling the space-vs-time trade offs
taken when building the automaton.

# Resource usage

Aho-Corasick automatons are always constructed in `O(p)` time, where
`p` is the combined length of all patterns being searched. With that
said, building an automaton can be fairly costly because of high constant
factors, particularly when enabling the [DFA](AhoCorasickKind::DFA) option
with [`AhoCorasickBuilder::kind`](#kind). For this reason, it's generally a good
idea to build an automaton once and reuse it as much as possible.

Aho-Corasick automatons can also use a fair bit of memory. To get
a concrete idea of how much memory is being used, try using the
[`AhoCorasick::memory_usage`](#memory-usage) method.

To give a quick idea of the differences between Aho-Corasick
implementations and their resource usage, here's a sample of construction
times and heap memory used after building an automaton from 100,000
randomly selected titles from Wikipedia:

* 99MB for a [`noncontiguous::NFA`](#nfa) in 240ms.
* 21MB for a [`contiguous::NFA`](#nfa) in 275ms.
* 1.6GB for a [`dfa::DFA`](#dfa) in 1.88s.

(Note that the memory usage above reflects the size of each automaton and
not peak memory usage. For example, building a contiguous NFA requires
first building a noncontiguous NFA. Once the contiguous NFA is built, the
noncontiguous NFA is freed.)

This experiment very strongly argues that a contiguous NFA is often the
best balance in terms of resource usage. It takes a little longer to build,
but its memory usage is quite small. Its search speed (not listed) is
also often faster than a noncontiguous NFA, but a little slower than a
DFA. Indeed, when no specific [`AhoCorasickKind`](#ahocorasickkind) is used (which is the
default), a contiguous NFA is used in most cases.

The only "catch" to using a contiguous NFA is that, because of its variety
of compression tricks, it may not be able to support automatons as large as
what the noncontiguous NFA supports. In which case, building a contiguous
NFA will fail and (by default) `AhoCorasick` will automatically fall
back to a noncontiguous NFA. (This typically only happens when building
automatons from millions of patterns.) Otherwise, the small additional time
for building a contiguous NFA is almost certainly worth it.

# Cloning

The `AhoCorasick` type uses thread safe reference counting internally. It
is guaranteed that it is cheap to clone.

# Search configuration

Most of the search routines accept anything that can be cheaply converted
to an [`Input`](#input). This includes `&[u8](#u8)`, `&str` and `Input` itself.

# Construction failure

It is generally possible for building an Aho-Corasick automaton to fail.
Construction can fail in generally one way: when the inputs provided are
too big. Whether that's a pattern that is too long, too many patterns
or some combination of both. A first approximation for the scale at which
construction can fail is somewhere around "millions of patterns."

For that reason, if you're building an Aho-Corasick automaton from
untrusted input (or input that doesn't have any reasonable bounds on its
size), then it is strongly recommended to handle the possibility of an
error.

If you're constructing an Aho-Corasick automaton from static or trusted
data, then it is likely acceptable to panic (by calling `unwrap()` or
`expect()`) if construction fails.

# Fallibility

The `AhoCorasick` type provides a number of methods for searching, as one
might expect. Depending on how the Aho-Corasick automaton was built and
depending on the search configuration, it is possible for a search to
return an error. Since an error is _never_ dependent on the actual contents
of the haystack, this type provides both infallible and fallible methods
for searching. The infallible methods panic if an error occurs, and can be
used for convenience and when you know the search will never return an
error.

For example, the [`AhoCorasick::find_iter`](#find-iter) method is the infallible
version of the [`AhoCorasick::try_find_iter`](#try-find-iter) method.

Examples of errors that can occur:

* Running a search that requires [`MatchKind::Standard`](#standard) semantics (such
as a stream or overlapping search) with an automaton that was built with
[`MatchKind::LeftmostFirst`](#leftmostfirst) or [`MatchKind::LeftmostLongest`](#leftmostlongest) semantics.
* Running an anchored search with an automaton that only supports
unanchored searches. (By default, `AhoCorasick` only supports unanchored
searches. But this can be toggled with [`AhoCorasickBuilder::start_kind`](#start-kind).)
* Running an unanchored search with an automaton that only supports
anchored searches.

The common thread between the different types of errors is that they are
all rooted in the automaton construction and search configurations. If
those configurations are a static property of your program, then it is
reasonable to call infallible routines since you know an error will never
occur. And if one _does_ occur, then it's a bug in your program.

To re-iterate, if the patterns, build or search configuration come from
user or untrusted data, then you should handle errors at build or search
time. If only the haystack comes from user or untrusted data, then there
should be no need to handle errors anywhere and it is generally encouraged
to `unwrap()` (or `expect()`) both build and search time calls.

# Examples

This example shows how to search for occurrences of multiple patterns
simultaneously in a case insensitive fashion. Each match includes the
pattern that matched along with the byte offsets of the match.

```
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::builder()
    .ascii_case_insensitive(true)
    .build(patterns)
    .unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

This example shows how to replace matches with some other string:

```
use aho_corasick::AhoCorasick;

let patterns = &["fox", "brown", "quick"];
let haystack = "The quick brown fox.";
let replace_with = &["sloth", "grey", "slow"];

let ac = AhoCorasick::new(patterns).unwrap();
let result = ac.replace_all(haystack, replace_with);
assert_eq!(result, "The slow grey sloth.");
```

### `Span`

```rust
struct Span {
}
```

A handle representing a span, with the capability to enter the span if it
exists.

If the span was rejected by the current `Subscriber`'s filter, entering the
span will silently do nothing. Thus, the handle can be used in the same
manner regardless of whether or not the trace is currently being collected.

#### Implementations

- `fn new(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span`
  Constructs a new `Span` with the given [metadata] and set of

- `fn new_root(meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span`
  Constructs a new `Span` as the root of its own trace tree, with the

- `fn child_of(parent: impl Into<Option<Id>>, meta: &'static Metadata<'static>, values: &field::ValueSet<'_>) -> Span`
  Constructs a new `Span` as child of the given parent span, with the

- `fn new_disabled(meta: &'static Metadata<'static>) -> Span`
  Constructs a new disabled span with the given `Metadata`.

- `const fn none() -> Span`
  Constructs a new span that is *completely disabled*.

- `fn current() -> Span`
  Returns a handle to the span [considered by the `Subscriber`] to be the

- `fn enter(self: &Self) -> Entered<'_>`
  Enters this span, returning a guard that will exit the span when dropped.

- `fn entered(self: Self) -> EnteredSpan`
  Enters this span, consuming it and returning a [guard][`EnteredSpan`]

- `fn or_current(self: Self) -> Self`
  Returns this span, if it was [enabled] by the current [`Subscriber`], or

- `fn in_scope<F: FnOnce() -> T, T>(self: &Self, f: F) -> T`
  Executes the given function in the context of this span.

- `fn field<Q: field::AsField + ?Sized>(self: &Self, field: &Q) -> Option<field::Field>`
  Returns a [`Field`][super::field::Field] for the field with the

- `fn has_field<Q: field::AsField + ?Sized>(self: &Self, field: &Q) -> bool`
  Returns true if this `Span` has a field for the given

- `fn record<Q: field::AsField + ?Sized, V: field::Value>(self: &Self, field: &Q, value: V) -> &Self`
  Records that the field described by `field` has the value `value`.

- `fn is_disabled(self: &Self) -> bool`
  Returns `true` if this span was disabled by the subscriber and does not

- `fn is_none(self: &Self) -> bool`
  Returns `true` if this span was constructed by [`Span::none`] and is

- `fn follows_from(self: &Self, from: impl Into<Option<Id>>) -> &Self`
  Indicates that the span with the given ID has an indirect causal

- `fn id(self: &Self) -> Option<Id>`
  Returns this span's `Id`, if it is enabled.

- `fn metadata(self: &Self) -> Option<&'static Metadata<'static>>`
  Returns this span's `Metadata`, if it is enabled.

- `fn with_subscriber<T>(self: &Self, f: impl FnOnce((&Id, &Dispatch)) -> T) -> Option<T>`
  Invokes a function with a reference to this span's ID and subscriber.

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

- `fn clone(self: &Self) -> Span`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, hasher: &mut H)`

##### `impl Instrument<T>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

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

##### `impl WithSubscriber<T>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

## Functions

## Macros

### `event!`

Constructs a new `Event`.

The event macro is invoked with a `Level` and up to 32 key-value fields.
Optionally, a format string and arguments may follow the fields; this will
be used to construct an implicit field named "message".

See [the top-level documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros

# Examples

```rust
use tracing::{event, Level};

# fn main() {
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
# }
```


### `span!`

Constructs a new span.

See [the top-level documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros

# Examples

Creating a new span:
```
# use tracing::{span, Level};
# fn main() {
let span = span!(Level::TRACE, "my span");
let _enter = span.enter();
// do work inside the span...
# }
```

### `record_all!`

Records multiple values on a span in a single call. As with recording
individual values, all fields must be declared when the span is created.

This macro supports two optional sigils:
- `%` uses the Display implementation.
- `?` uses the Debug implementation.

For more details, see the [top-level documentation][lib](#lib).

[lib](#lib): tracing/#recording-fields

# Examples

```
# use tracing::{field, info_span, record_all};
let span = info_span!("my span", field1 = field::Empty, field2 = field::Empty, field3 = field::Empty).entered();
record_all!(span, field1 = ?"1", field2 = %"2", field3 = 3);
```

### `trace_span!`

Constructs a span at the trace level.

[Fields] and [attributes](#attributes) are set using the same syntax as the [`span!`](#span)
macro.

See [the top-level documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros
[attributes](#attributes): crate#configuring-attributes
[Fields]: crate#recording-fields

# Examples

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
trace_span!("my_span");
// is equivalent to:
span!(Level::TRACE, "my_span");
# }
```

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
let span = trace_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

### `debug_span!`

Constructs a span at the debug level.

[Fields] and [attributes](#attributes) are set using the same syntax as the [`span!`](#span)
macro.

See [the top-level documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros
[attributes](#attributes): crate#configuring-attributes
[Fields]: crate#recording-fields

# Examples

```rust
# use tracing::{debug_span, span, Level};
# fn main() {
debug_span!("my_span");
// is equivalent to:
span!(Level::DEBUG, "my_span");
# }
```

```rust
# use tracing::debug_span;
# fn main() {
let span = debug_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

### `info_span!`

Constructs a span at the info level.

[Fields] and [attributes](#attributes) are set using the same syntax as the [`span!`](#span)
macro.

See [the top-level documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros
[attributes](#attributes): crate#configuring-attributes
[Fields]: crate#recording-fields

# Examples

```rust
# use tracing::{span, info_span, Level};
# fn main() {
info_span!("my_span");
// is equivalent to:
span!(Level::INFO, "my_span");
# }
```

```rust
# use tracing::info_span;
# fn main() {
let span = info_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

### `warn_span!`

Constructs a span at the warn level.

[Fields] and [attributes](#attributes) are set using the same syntax as the [`span!`](#span)
macro.

See [the top-level documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros
[attributes](#attributes): crate#configuring-attributes
[Fields]: crate#recording-fields

# Examples

```rust
# use tracing::{warn_span, span, Level};
# fn main() {
warn_span!("my_span");
// is equivalent to:
span!(Level::WARN, "my_span");
# }
```

```rust
use tracing::warn_span;
# fn main() {
let span = warn_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

### `error_span!`

Constructs a span at the error level.

[Fields] and [attributes](#attributes) are set using the same syntax as the [`span!`](#span)
macro.

See [the top-level documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros
[attributes](#attributes): crate#configuring-attributes
[Fields]: crate#recording-fields

# Examples

```rust
# use tracing::{span, error_span, Level};
# fn main() {
error_span!("my_span");
// is equivalent to:
span!(Level::ERROR, "my_span");
# }
```

```rust
# use tracing::error_span;
# fn main() {
let span = error_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

### `event_enabled!`

Tests whether an event with the specified level and target would be enabled.

This is similar to [`enabled!`](#enabled), but queries the current subscriber specifically for
an event, whereas [`enabled!`](#enabled) queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`](#span-enabled).

# Examples

```rust
# use tracing::{event_enabled, Level};
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

Tests whether a span with the specified level and target would be enabled.

This is similar to [`enabled!`](#enabled), but queries the current subscriber specifically for
an event, whereas [`enabled!`](#enabled) queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`](#span-enabled).

# Examples

```rust
# use tracing::{span_enabled, Level};
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

Checks whether a span or event is [enabled](#enabled) based on the provided [metadata](#metadata).

[enabled](#enabled): crate::Subscriber::enabled
[metadata](#metadata): crate::Metadata

This macro is a specialized tool: it is intended to be used prior
to an expensive computation required *just* for that event, but
*cannot* be done as part of an argument to that event, such as
when multiple events are emitted (e.g., iterating over a collection
and emitting an event for each item).

# Usage

[Subscribers] can make filtering decisions based all the data included in a
span or event's [`Metadata`](../log/log/index.md). This means that it is possible for `enabled!`
to return a _false positive_ (indicating that something would be enabled
when it actually would not be) or a _false negative_ (indicating that
something would be disabled when it would actually be enabled).

[Subscribers]: crate::subscriber::Subscriber

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
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", at
the level `DEBUG`, and with a field named "hello", this will evaluate
to true:

```rust
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG, hello) {
    // some expensive work...
}
```

# Alternatives

`enabled!` queries subscribers with [`Metadata`](../log/log/index.md) where
[`is_event`](#is-event) and [`is_span`](#is-span) both return `false`. Alternatively,
use [`event_enabled!`](#event-enabled) or [`span_enabled!`](#span-enabled) to ensure one of these
returns true.






### `trace!`

Constructs an event at the trace level.

This functions similarly to the [`event!`](#event) macro. See [the top-level
documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros

# Examples

```rust
use tracing::trace;
# #[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
# impl Position {
# const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
# fn dist(&self, other: Position) -> f32 {
#    let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
#    (x + y).sqrt()
# }
# }
# fn main() {
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
# }
```

### `debug!`

Constructs an event at the debug level.

This functions similarly to the [`event!`](#event) macro. See [the top-level
documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros

# Examples

```rust
use tracing::debug;
# fn main() {
# #[derive(Debug)] struct Position { x: f32, y: f32 }

let pos = Position { x: 3.234, y: -1.223 };

debug!(?pos.x, ?pos.y);
debug!(target: "app_events", position = ?pos, "New position");
debug!(name: "completed", position = ?pos);
# }
```

### `info!`

Constructs an event at the info level.

This functions similarly to the [`event!`](#event) macro. See [the top-level
documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros

# Examples

```rust
use tracing::info;
# // this is so the test will still work in no-std mode
# #[derive(Debug)]
# pub struct Ipv4Addr;
# impl Ipv4Addr { fn new(o1: u8, o2: u8, o3: u8, o4: u8) -> Self { Self } }
# fn main() {
# struct Connection { port: u32, speed: f32 }
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
# }
```

### `warn!`

Constructs an event at the warn level.

This functions similarly to the [`event!`](#event) macro. See [the top-level
documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros

# Examples

```rust
use tracing::warn;
# fn main() {

let warn_description = "Invalid Input";
let input = &[0x27, 0x45];

warn!(?input, warning = warn_description);
warn!(
    target: "input_events",
    warning = warn_description,
    "Received warning for input: {:?}", input,
);
warn!(name: "invalid", ?input);
# }
```

### `error!`

Constructs an event at the error level.

This functions similarly to the [`event!`](#event) macro. See [the top-level
documentation][lib](#lib) for details on the syntax accepted by
this macro.

[lib](#lib): crate#using-the-macros

# Examples

```rust
use tracing::error;
# fn main() {

let (err_info, port) = ("No connection", 22);

error!(port, error = %err_info);
error!(target: "app_events", "App Error: {}", err_info);
error!({ info = err_info }, "error on port: {}", port);
error!(name: "invalid_input", "Invalid input: {}", err_info);
# }
```

