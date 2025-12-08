*[tracing_subscriber](../index.md) / [layer](index.md)*

---

# Module `layer`

 The [`Layer`](#layer) trait, a composable abstraction for building [`Subscriber`](../fmt/index.md)s.

 The [`Subscriber`](../fmt/index.md) trait in `tracing-core` represents the _complete_ set of
 functionality required to consume `tracing` instrumentation. This means that
 a single `Subscriber` instance is a self-contained implementation of a
 complete strategy for collecting traces; but it _also_ means that the
 `Subscriber` trait cannot easily be composed with other `Subscriber`s.

 In particular, [`Subscriber`](../fmt/index.md)s are responsible for generating [span IDs] and
 assigning them to spans. Since these IDs must uniquely identify a span
 within the context of the current trace, this means that there may only be
 a single `Subscriber` for a given thread at any point in time &mdash;
 otherwise, there would be no authoritative source of span IDs.

 On the other hand, the majority of the [`Subscriber`](../fmt/index.md) trait's functionality
 is composable: any number of subscribers may _observe_ events, span entry
 and exit, and so on, provided that there is a single authoritative source of
 span IDs. The [`Layer`](#layer) trait represents this composable subset of the
 [`Subscriber`](../fmt/index.md) behavior; it can _observe_ events and spans, but does not
 assign IDs.

 # Composing Layers

 Since a [`Layer`](#layer) does not implement a complete strategy for collecting
 traces, it must be composed with a `Subscriber` in order to be used. The
 [`Layer`](#layer) trait is generic over a type parameter (called `S` in the trait
 definition), representing the types of `Subscriber` they can be composed
 with. Thus, a [`Layer`](#layer) may be implemented that will only compose with a
 particular `Subscriber` implementation, or additional trait bounds may be
 added to constrain what types implementing `Subscriber` a `Layer` can wrap.

 `Layer`s may be added to a `Subscriber` by using the `SubscriberExt::with`
 method, which is provided by `tracing-subscriber`'s [`prelude`](../prelude/index.md). This method
 returns a [`Layered`](#layered) struct that implements `Subscriber` by composing the
 `Layer` with the `Subscriber`.

 For example:
 ```rust
 use tracing_subscriber::Layer;
 use tracing_subscriber::prelude::*;
 use tracing::Subscriber;

 pub struct MyLayer {
     // ...
 }

 impl<S: Subscriber> Layer<S> for MyLayer {
     // ...
 }

 pub struct MySubscriber {
     // ...
 }

 use tracing_core::{span::{Id, Attributes, Record}, Metadata, Event};
 impl Subscriber for MySubscriber {
     // ...
   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(1) }
   fn record(&self, _: &Id, _: &Record) {}
   fn event(&self, _: &Event) {}
   fn record_follows_from(&self, _: &Id, _: &Id) {}
   fn enabled(&self, _: &Metadata) -> bool { false }
   fn enter(&self, _: &Id) {}
   fn exit(&self, _: &Id) {}
 }
 impl MyLayer {
 fn new() -> Self { Self {} }
 }
 impl MySubscriber {
 fn new() -> Self { Self {} }
 }

 let subscriber = MySubscriber::new()
     .with(MyLayer::new());

 tracing::subscriber::set_global_default(subscriber);
 ```

 Multiple `Layer`s may be composed in the same manner:
 ```rust
 use tracing_subscriber::{Layer, layer::SubscriberExt};
 use tracing::Subscriber;
 pub struct MyOtherLayer {
     // ...
 }

 impl<S: Subscriber> Layer<S> for MyOtherLayer {
     // ...
 }

 pub struct MyThirdLayer {
     // ...
 }

 impl<S: Subscriber> Layer<S> for MyThirdLayer {
     // ...
 }
 pub struct MyLayer {}
 impl<S: Subscriber> Layer<S> for MyLayer {}
 pub struct MySubscriber { }
 use tracing_core::{span::{Id, Attributes, Record}, Metadata, Event};
 impl Subscriber for MySubscriber {
   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(1) }
   fn record(&self, _: &Id, _: &Record) {}
   fn event(&self, _: &Event) {}
   fn record_follows_from(&self, _: &Id, _: &Id) {}
   fn enabled(&self, _: &Metadata) -> bool { false }
   fn enter(&self, _: &Id) {}
   fn exit(&self, _: &Id) {}
 }
 impl MyLayer {
 fn new() -> Self { Self {} }
 }
 impl MyOtherLayer {
 fn new() -> Self { Self {} }
 }
 impl MyThirdLayer {
 fn new() -> Self { Self {} }
 }
 impl MySubscriber {
 fn new() -> Self { Self {} }
 }

 let subscriber = MySubscriber::new()
     .with(MyLayer::new())
     .with(MyOtherLayer::new())
     .with(MyThirdLayer::new());

 tracing::subscriber::set_global_default(subscriber);
 ```

 The `Layer::with_subscriber` constructs the [`Layered`](#layered) type from a
 [`Layer`](#layer) and [`Subscriber`](../fmt/index.md), and is called by `SubscriberExt::with`. In
 general, it is more idiomatic to use `SubscriberExt::with`, and treat
 `Layer::with_subscriber` as an implementation detail, as `with_subscriber`
 calls must be nested, leading to less clear code for the reader.

 ## Runtime Configuration With `Layer`s

 In some cases, a particular [`Layer`](#layer) may be enabled or disabled based on
 runtime configuration. This can introduce challenges, because the type of a
 layered [`Subscriber`](../fmt/index.md) depends on which layers are added to it: if an `if`
 or `match` expression adds some [`Layer`](#layer) implementation in one branch,
 and other layers in another, the [`Subscriber`](../fmt/index.md) values returned by those
 branches will have different types. For example, the following _will not_
 work:

 ```compile_fail
 fn docs() -> Result<(), Box<dyn std::error::Error + 'static>> {
 struct Config {
    is_prod: bool,
    path: &'static str,
 }
 let cfg = Config { is_prod: false, path: "debug.log" };
 use std::fs::File;
 use tracing_subscriber::{Registry, prelude::*};

 let stdout_log = tracing_subscriber::fmt::layer().pretty();
 let subscriber = Registry::default().with(stdout_log);

 // The compile error will occur here because the if and else
 // branches have different (and therefore incompatible) types.
 let subscriber = if cfg.is_prod {
     let file = File::create(cfg.path)?;
     let layer = tracing_subscriber::fmt::layer()
         .json()
         .with_writer(Arc::new(file));
     layer.with(subscriber)
 } else {
     layer
 };

 tracing::subscriber::set_global_default(subscriber)
     .expect("Unable to set global subscriber");
 Ok(()) }
 ```

 However, a [`Layer`](#layer) wrapped in an [`Option`](../../clap_derive/index.md) [also implements the `Layer`
 trait][option-impl]. This allows individual layers to be enabled or disabled at
 runtime while always producing a [`Subscriber`](../fmt/index.md) of the same type. For
 example:

 ```rust
 fn docs() -> Result<(), Box<dyn std::error::Error + 'static>> {
 struct Config {
    is_prod: bool,
    path: &'static str,
 }
 let cfg = Config { is_prod: false, path: "debug.log" };
 use std::fs::File;
 use tracing_subscriber::{Registry, prelude::*};

 let stdout_log = tracing_subscriber::fmt::layer().pretty();
 let subscriber = Registry::default().with(stdout_log);

 // if `cfg.is_prod` is true, also log JSON-formatted logs to a file.
 let json_log = if cfg.is_prod {
     let file = File::create(cfg.path)?;
     let json_log = tracing_subscriber::fmt::layer()
         .json()
         .with_writer(file);
     Some(json_log)
 } else {
     None
 };

 // If `cfg.is_prod` is false, then `json` will be `None`, and this layer
 // will do nothing. However, the subscriber will still have the same type
 // regardless of whether the `Option`'s value is `None` or `Some`.
 let subscriber = subscriber.with(json_log);

 tracing::subscriber::set_global_default(subscriber)
    .expect("Unable to set global subscriber");
 Ok(()) }
 ```

 If a [`Layer`](#layer) may be one of several different types, note that [`Box<dyn
 Layer<S> + Send + Sync>` implements `Layer`][box-impl].
 This may be used to erase the type of a [`Layer`](#layer).

 For example, a function that configures a [`Layer`](#layer) to log to one of
 several outputs might return a `Box<dyn Layer<S> + Send + Sync + 'static>`:
 ```rust
 use tracing_subscriber::{
     Layer,
     registry::LookupSpan,
     prelude::*,
 };
 use std::{path::PathBuf, fs::File, io};

 /// Configures whether logs are emitted to a file, to stdout, or to stderr.
 pub enum LogConfig {
     File(PathBuf),
     Stdout,
     Stderr,
 }

 impl LogConfig {
     pub fn layer<S>(self) -> Box<dyn Layer<S> + Send + Sync + 'static>
     where
         S: tracing_core::Subscriber,
         for<'a> S: LookupSpan<'a>,
     {
         // Shared configuration regardless of where logs are output to.
         let fmt = tracing_subscriber::fmt::layer()
             .with_target(true)
             .with_thread_names(true);

         // Configure the writer based on the desired log target:
         match self {
             LogConfig::File(path) => {
                 let file = File::create(path).expect("failed to create log file");
                 Box::new(fmt.with_writer(file))
             },
             LogConfig::Stdout => Box::new(fmt.with_writer(io::stdout)),
             LogConfig::Stderr => Box::new(fmt.with_writer(io::stderr)),
         }
     }
 }

 let config = LogConfig::Stdout;
 tracing_subscriber::registry()
     .with(config.layer())
     .init();
 ```

 The `Layer::boxed` method is provided to make boxing a `Layer`
 more convenient, but `Box::new` may be used as well.

 When the number of `Layer`s varies at runtime, note that a
 [`Vec<L> where L: Layer` also implements `Layer`][vec-impl]. This
 can be used to add a variable number of `Layer`s to a `Subscriber`:

 ```rust
 use tracing_subscriber::{Layer, prelude::*};
 struct MyLayer {
     // ...
 }
 impl MyLayer { fn new() -> Self { Self {} }}

 impl<S: tracing_core::Subscriber> Layer<S> for MyLayer {
     // ...
 }

 /// Returns how many layers we need
 fn how_many_layers() -> usize {
     // ...
     3
 }

 // Create a variable-length `Vec` of layers
 let mut layers = Vec::new();
 for _ in 0..how_many_layers() {
     layers.push(MyLayer::new());
 }

 tracing_subscriber::registry()
     .with(layers)
     .init();
 ```

 If a variable number of `Layer` is needed and those `Layer`s have
 different types, a `Vec` of [boxed `Layer` trait objects][box-impl] may
 be used. For example:

 ```rust
 use tracing_subscriber::{filter::LevelFilter, Layer, prelude::*};
 use std::fs::File;
 fn main() -> Result<(), Box<dyn std::error::Error>> {
 struct Config {
     enable_log_file: bool,
     enable_stdout: bool,
     enable_stderr: bool,
     // ...
 }
 impl Config {
    fn from_config_file()-> Result<Self, Box<dyn std::error::Error>> {
         // don't enable the log file so that the example doesn't actually create it
         Ok(Self { enable_log_file: false, enable_stdout: true, enable_stderr: true })
    }
 }

 let cfg = Config::from_config_file()?;

 // Based on our dynamically loaded config file, create any number of layers:
 let mut layers = Vec::new();

 if cfg.enable_log_file {
     let file = File::create("myapp.log")?;
     let layer = tracing_subscriber::fmt::layer()
         .with_thread_names(true)
         .with_target(true)
         .json()
         .with_writer(file)
         // Box the layer as a type-erased trait object, so that it can
         // be pushed to the `Vec`.
         .boxed();
     layers.push(layer);
 }

 if cfg.enable_stdout {
     let layer = tracing_subscriber::fmt::layer()
         .pretty()
         .with_filter(LevelFilter::INFO)
         // Box the layer as a type-erased trait object, so that it can
         // be pushed to the `Vec`.
         .boxed();
     layers.push(layer);
 }

 if cfg.enable_stdout {
     let layer = tracing_subscriber::fmt::layer()
         .with_target(false)
         .with_filter(LevelFilter::WARN)
         // Box the layer as a type-erased trait object, so that it can
         // be pushed to the `Vec`.
         .boxed();
     layers.push(layer);
 }

 tracing_subscriber::registry()
     .with(layers)
     .init();
Ok(()) }
 ```

 Finally, if the number of layers _changes_ at runtime, a `Vec` of
 subscribers can be used alongside the [`reload`](crate::reload) module to
 add or remove subscribers dynamically at runtime.




 # Recording Traces

 The [`Layer`](#layer) trait defines a set of methods for consuming notifications from
 tracing instrumentation, which are generally equivalent to the similarly
 named methods on [`Subscriber`](../fmt/index.md). Unlike [`Subscriber`](../fmt/index.md), the methods on
 `Layer` are additionally passed a [`Context`](#context) type, which exposes additional
 information provided by the wrapped subscriber (such as [the current span])
 to the layer.

 # Filtering with `Layer`s

 As well as strategies for handling trace events, the `Layer` trait may also
 be used to represent composable _filters_. This allows the determination of
 what spans and events should be recorded to be decoupled from _how_ they are
 recorded: a filtering layer can be applied to other layers or
 subscribers. `Layer`s can be used to implement _global filtering_, where a
 `Layer` provides a filtering strategy for the entire subscriber.
 Additionally, individual recording `Layer`s or sets of `Layer`s may be
 combined with _per-layer filters_ that control what spans and events are
 recorded by those layers.

 ## Global Filtering

 A `Layer` that implements a filtering strategy should override the
 `register_callsite` and/or `enabled` methods. It may also choose to implement
 methods such as `on_enter`, if it wishes to filter trace events based on
 the current span context.

 Note that the `Layer::register_callsite` and `Layer::enabled` methods
 determine whether a span or event is enabled *globally*. Thus, they should
 **not** be used to indicate whether an individual layer wishes to record a
 particular span or event. Instead, if a layer is only interested in a subset
 of trace data, but does *not* wish to disable other spans and events for the
 rest of the layer stack should ignore those spans and events in its
 notification methods.

 The filtering methods on a stack of `Layer`s are evaluated in a top-down
 order, starting with the outermost `Layer` and ending with the wrapped
 [`Subscriber`](../fmt/index.md). If any layer returns `false` from its `enabled` method, or
 `Interest::never()` from its `register_callsite` method, filter
 evaluation will short-circuit and the span or event will be disabled.

 ### Enabling Interest

 Whenever an tracing event (or span) is emitted, it goes through a number of
 steps to determine how and how much it should be processed. The earlier an
 event is disabled, the less work has to be done to process the event, so
 `Layer`s that implement filtering should attempt to disable unwanted
 events as early as possible. In order, each event checks:

 - `register_callsite`, once per callsite (roughly: once per time that
   `event!` or `span!` is written in the source code; this is cached at the
   callsite). See `Subscriber::register_callsite` and
   [`tracing_core::callsite`](../../tracing_core/callsite/index.md) for a summary of how this behaves.
 - `enabled`, once per emitted event (roughly: once per time that `event!`
   or `span!` is *executed*), and only if `register_callsite` registers an
   `Interest::sometimes`. This is the main customization point to globally
   filter events based on their [`Metadata`](../../tracing_core/metadata/index.md). If an event can be disabled
   based only on [`Metadata`](../../tracing_core/metadata/index.md), it should be, as this allows the construction
   of the actual `Event`/`Span` to be skipped.
 - For events only (and not spans), `event_enabled` is called just before
   processing the event. This gives layers one last chance to say that
   an event should be filtered out, now that the event's fields are known.

 ## Per-Layer Filtering

 **Note**: per-layer filtering APIs currently require the [`"registry"` crate
 feature flag][feat] to be enabled.

 Sometimes, it may be desirable for one `Layer` to record a particular subset
 of spans and events, while a different subset of spans and events are
 recorded by other `Layer`s. For example:

 - A layer that records metrics may wish to observe only events including
   particular tracked values, while a logging layer ignores those events.
 - If recording a distributed trace is expensive, it might be desirable to
   only send spans with `INFO` and lower verbosity to the distributed tracing
   system, while logging more verbose spans to a file.
 - Spans and events with a particular target might be recorded differently
   from others, such as by generating an HTTP access log from a span that
   tracks the lifetime of an HTTP request.

 The [`Filter`](#filter) trait is used to control what spans and events are
 observed by an individual `Layer`, while still allowing other `Layer`s to
 potentially record them. The `Layer::with_filter` method combines a
 `Layer` with a [`Filter`](#filter), returning a [`Filtered`](../filter/index.md) layer.

 This crate's [`filter`](../filter/index.md) module provides a number of types which implement
 the [`Filter`](#filter) trait, such as [`LevelFilter`](../filter/level/index.md), [`Targets`](../filter/targets/index.md), and
 [`FilterFn`](../filter/index.md). These [`Filter`](#filter)s provide ready-made implementations of
 common forms of filtering. For custom filtering policies, the [`FilterFn`](../filter/index.md)
 and [`DynFilterFn`](../filter/index.md) types allow implementing a [`Filter`](#filter) with a closure or
 function pointer. In addition, when more control is required, the [`Filter`](#filter)
 trait may also be implemented for user-defined types.

 [`Option<Filter>`](../../clap_derive/index.md) also implements [`Filter`](#filter), which allows for an optional
 filter. `None` filters out _nothing_ (that is, allows everything through). For
 example:

 ```rust
 use tracing_subscriber::{filter::filter_fn, Layer};
 use tracing_core::{Metadata, subscriber::Subscriber};
 struct MyLayer<S>(std::marker::PhantomData<S>);
 impl<S> MyLayer<S> { fn new() -> Self { Self(std::marker::PhantomData)} }
 impl<S: Subscriber> Layer<S> for MyLayer<S> {}
 fn my_filter(_: &str) -> impl Fn(&Metadata) -> bool { |_| true  }
 fn setup_tracing<S: Subscriber>(filter_config: Option<&str>) {
     let layer = MyLayer::<S>::new()
         .with_filter(filter_config.map(|config| filter_fn(my_filter(config))));
 //...
 }
 ```

 <pre class="compile_fail" style="white-space:normal;font:inherit;">
     <strong>Warning</strong>: Currently, the <a href="../struct.Registry.html">
     <code>Registry</code></a> type defined in this crate is the only root
     <code>Subscriber</code> capable of supporting <code>Layer</code>s with
     per-layer filters. In the future, new APIs will be added to allow other
     root <code>Subscriber</code>s to support per-layer filters.
 </pre>

 For example, to generate an HTTP access log based on spans with
 the `http_access` target, while logging other spans and events to
 standard out, a [`Filter`](#filter) can be added to the access log layer:

 ```rust
 use tracing_subscriber::{filter, prelude::*};

 // Generates an HTTP access log.
 let access_log = // ...
     filter::LevelFilter::INFO;

 // Add a filter to the access log layer so that it only observes
 // spans and events with the `http_access` target.
 let access_log = access_log.with_filter(filter::filter_fn(|metadata| {
     // Returns `true` if and only if the span or event's target is
     // "http_access".
     metadata.target() == "http_access"
 }));

 // A general-purpose logging layer.
 let fmt_layer = tracing_subscriber::fmt::layer();

 // Build a subscriber that combines the access log and stdout log
 // layers.
 tracing_subscriber::registry()
     .with(fmt_layer)
     .with(access_log)
     .init();
 ```

 Multiple layers can have their own, separate per-layer filters. A span or
 event will be recorded if it is enabled by _any_ per-layer filter, but it
 will be skipped by the layers whose filters did not enable it. Building on
 the previous example:

 ```rust
 use tracing_subscriber::{filter::{filter_fn, LevelFilter}, prelude::*};

 let access_log = // ...
     LevelFilter::INFO;
 let fmt_layer = tracing_subscriber::fmt::layer();

 tracing_subscriber::registry()
     // Add the filter for the "http_access" target to the access
     // log layer, like before.
     .with(access_log.with_filter(filter_fn(|metadata| {
         metadata.target() == "http_access"
     })))
     // Add a filter for spans and events with the INFO level
     // and below to the logging layer.
     .with(fmt_layer.with_filter(LevelFilter::INFO))
     .init();

 // Neither layer will observe this event
 tracing::debug!(does_anyone_care = false, "a tree fell in the forest");

 // This event will be observed by the logging layer, but not
 // by the access log layer.
 tracing::warn!(dose_roentgen = %3.8, "not great, but not terrible");

 // This event will be observed only by the access log layer.
 tracing::trace!(target: "http_access", "HTTP request started");

 // Both layers will observe this event.
 tracing::error!(target: "http_access", "HTTP request failed with a very bad error!");
 ```

 A per-layer filter can be applied to multiple [`Layer`](#layer)s at a time, by
 combining them into a [`Layered`](#layered) layer using `Layer::and_then`, and then
 calling `Layer::with_filter` on the resulting [`Layered`](#layered) layer.

 Consider the following:
 - `layer_a` and `layer_b`, which should only receive spans and events at
   the `INFO` [`level`](../filter/level/index.md) and above.
 - A third layer, `layer_c`, which should receive spans and events at
   the `DEBUG` [`level`](../filter/level/index.md) as well.

 The layers and filters would be composed thusly:

 ```rust
 use tracing_subscriber::{filter::LevelFilter, prelude::*};

 let layer_a = // ...
 LevelFilter::INFO;
 let layer_b =  // ...
 LevelFilter::INFO;
 let layer_c =  // ...
 LevelFilter::INFO;

 let info_layers = layer_a
     // Combine `layer_a` and `layer_b` into a `Layered` layer:
     .and_then(layer_b)
     // ...and then add an `INFO` `LevelFilter` to that layer:
     .with_filter(LevelFilter::INFO);

 tracing_subscriber::registry()
     // Add `layer_c` with a `DEBUG` filter.
     .with(layer_c.with_filter(LevelFilter::DEBUG))
     .with(info_layers)
     .init();
```

 If a [`Filtered`](../filter/index.md) [`Layer`](#layer) is combined with another [`Layer`](#layer)
 `Layer::and_then`, and a filter is added to the [`Layered`](#layered) layer, that
 layer will be filtered by *both* the inner filter and the outer filter.
 Only spans and events that are enabled by *both* filters will be
 observed by that layer. This can be used to implement complex filtering
 trees.

 As an example, consider the following constraints:
 - Suppose that a particular [`target`](../../tracing_attributes/attr/kw/index.md) is used to indicate events that
   should be counted as part of a metrics system, which should be only
   observed by a layer that collects metrics.
 - A log of high-priority events (`INFO` and above) should be logged
   to stdout, while more verbose events should be logged to a debugging log file.
 - Metrics-focused events should *not* be included in either log output.

 In that case, it is possible to apply a filter to both logging layers to
 exclude the metrics events, while additionally adding a [`LevelFilter`](../filter/level/index.md)
 to the stdout log:

 ```rust
 // wrap this in a function so we don't actually create `debug.log` when
 // running the doctests..
 fn docs() -> Result<(), Box<dyn std::error::Error + 'static>> {
 use tracing_subscriber::{filter, prelude::*};
 use std::{fs::File, sync::Arc};

 // A layer that logs events to stdout using the human-readable "pretty"
 // format.
 let stdout_log = tracing_subscriber::fmt::layer()
     .pretty();

 // A layer that logs events to a file.
 let file = File::create("debug.log")?;
 let debug_log = tracing_subscriber::fmt::layer()
     .with_writer(Arc::new(file));

 // A layer that collects metrics using specific events.
 let metrics_layer = /* ... */ filter::LevelFilter::INFO;

 tracing_subscriber::registry()
     .with(
         stdout_log
             // Add an `INFO` filter to the stdout logging layer
             .with_filter(filter::LevelFilter::INFO)
             // Combine the filtered `stdout_log` layer with the
             // `debug_log` layer, producing a new `Layered` layer.
             .and_then(debug_log)
             // Add a filter to *both* layers that rejects spans and
             // events whose targets start with `metrics`.
             .with_filter(filter::filter_fn(|metadata| {
                 !metadata.target().starts_with("metrics")
             }))
     )
     .with(
         // Add a filter to the metrics label that *only* enables
         // events whose targets start with `metrics`.
         metrics_layer.with_filter(filter::filter_fn(|metadata| {
             metadata.target().starts_with("metrics")
         }))
     )
     .init();

 // This event will *only* be recorded by the metrics layer.
 tracing::info!(target: "metrics::cool_stuff_count", value = 42);

 // This event will only be seen by the debug log file layer:
 tracing::debug!("this is a message, and part of a system of messages");

 // This event will be seen by both the stdout log layer *and*
 // the debug log file layer, but not by the metrics layer.
 tracing::warn!("the message is a warning about danger!");
 Ok(()) }
 ```






















## Structs

### `Identity`

```rust
struct Identity {
    _p: (),
}
```

A layer that does nothing.

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for Identity`

- `fn clone(self: &Self) -> Identity` — [`Identity`](#identity)

##### `impl Debug for Identity`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Identity`

- `fn default() -> Identity` — [`Identity`](#identity)

##### `impl<T> Instrument for Identity`

##### `impl<S: Subscriber> Layer for Identity`

##### `impl<T> WithSubscriber for Identity`

### `Context<'a, S>`

```rust
struct Context<'a, S> {
    subscriber: Option<&'a S>,
    filter: crate::filter::FilterId,
}
```

Represents information about the current context provided to [`Layer`](../fmt/fmt_layer/index.md)s by the
wrapped [`Subscriber`](../fmt/index.md).

To access [stored data] keyed by a span ID, implementors of the `Layer`
trait should ensure that the `Subscriber` type parameter is *also* bound by the

use tracing::Subscriber;
use tracing_subscriber::{Layer, registry::LookupSpan};

pub struct MyLayer;

impl<S> Layer<S> for MyLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    // ...
}
```rust





#### Fields

- **`filter`**: `crate::filter::FilterId`

  The bitmask of all [`Filtered`](../filter/index.md) layers that currently apply in this
  context. If there is only a single [`Filtered`](../filter/index.md) wrapping the layer that
  produced this context, then this is that filter's ID. Otherwise, if we
  are in a nested tree with multiple filters, this is produced by
  `and`-ing together the [`FilterId`](../filter/index.md)s of each of the filters that wrap
  the current layer.
  
  
  

#### Implementations

- `fn new(subscriber: &'a S) -> Self`

- `fn current_span(self: &Self) -> span::Current`

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool`

- `fn event(self: &Self, event: &Event<'_>)`

- `fn event_span(self: &Self, event: &Event<'_>) -> Option<SpanRef<'_, S>>` — [`SpanRef`](../registry/index.md)

- `fn metadata(self: &Self, id: &span::Id) -> Option<&'static Metadata<'static>>`

- `fn span(self: &Self, id: &span::Id) -> Option<registry::SpanRef<'_, S>>` — [`SpanRef`](../registry/index.md)

- `fn exists(self: &Self, id: &span::Id) -> bool`

- `fn lookup_current(self: &Self) -> Option<registry::SpanRef<'_, S>>` — [`SpanRef`](../registry/index.md)

- `fn lookup_current_filtered<'lookup>(self: &Self, subscriber: &'lookup S) -> Option<registry::SpanRef<'lookup, S>>` — [`SpanRef`](../registry/index.md)

- `fn span_scope(self: &Self, id: &span::Id) -> Option<registry::Scope<'_, S>>` — [`Scope`](../registry/index.md)

- `fn event_scope(self: &Self, event: &Event<'_>) -> Option<registry::Scope<'_, S>>` — [`Scope`](../registry/index.md)

- `fn with_filter(self: Self, filter: FilterId) -> Self` — [`FilterId`](../filter/index.md)

- `fn is_enabled_for(self: &Self, span: &span::Id, filter: FilterId) -> bool` — [`FilterId`](../filter/index.md)

- `fn if_enabled_for(self: Self, span: &span::Id, filter: FilterId) -> Option<Self>` — [`FilterId`](../filter/index.md)

- `fn is_enabled_inner(self: &Self, span: &span::Id, filter: FilterId) -> Option<bool>` — [`FilterId`](../filter/index.md)

#### Trait Implementations

##### `impl<S> Clone for Context<'_, S>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, S: $crate::fmt::Debug> Debug for Context<'a, S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Context<'a, S>`

##### `impl<T> WithSubscriber for Context<'a, S>`

### `Layered<L, I, S>`

```rust
struct Layered<L, I, S> {
    layer: L,
    inner: I,
    inner_is_registry: bool,
    has_layer_filter: bool,
    inner_has_layer_filter: bool,
    _s: core::marker::PhantomData<fn(S)>,
}
```

A [`Subscriber`](../fmt/index.md) composed of a `Subscriber` wrapped by one or more
[`Layer`](../fmt/fmt_layer/index.md)s.



#### Fields

- **`layer`**: `L`

  The layer.

- **`inner`**: `I`

  The inner value that `self.layer` was layered onto.
  
  If this is also a `Layer`, then this `Layered` will implement `Layer`.
  If this is a `Subscriber`, then this `Layered` will implement
  `Subscriber` instead.

- **`inner_is_registry`**: `bool`

  Is `self.inner` a `Registry`?
  
  If so, when combining `Interest`s, we want to "bubble up" its
  `Interest`.

- **`has_layer_filter`**: `bool`

  Does `self.layer` have per-layer filters?
  
  This will be true if:
  - `self.inner` is a `Filtered`.
  - `self.inner` is a tree of `Layered`s where _all_ arms of those
    `Layered`s have per-layer filters.
  
  Otherwise, if it's a `Layered` with one per-layer filter in one branch,
  but a non-per-layer-filtered layer in the other branch, this will be
  _false_, because the `Layered` is already handling the combining of
  per-layer filter `Interest`s and max level hints with its non-filtered
  `Layer`.

- **`inner_has_layer_filter`**: `bool`

  Does `self.inner` have per-layer filters?
  
  This is determined according to the same rules as
  `has_layer_filter` above.

#### Implementations

- `fn ctx(self: &Self) -> Context<'_, S>` — [`Context`](#context)

#### Trait Implementations

##### `impl<L: $crate::clone::Clone, I: $crate::clone::Clone, S: $crate::clone::Clone> Clone for Layered<L, I, S>`

- `fn clone(self: &Self) -> Layered<L, I, S>` — [`Layered`](#layered)

##### `impl<A, B, S> Debug for Layered<A, B, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for Layered<L, I, S>`

##### `impl<S, A, B> Layer for Layered<A, B, S>`

- `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`

- `fn on_layer(self: &mut Self, subscriber: &mut S)`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool` — [`Context`](#context)

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](#context)

- `fn on_record(self: &Self, span: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)` — [`Context`](#context)

- `fn on_follows_from(self: &Self, span: &span::Id, follows: &span::Id, ctx: Context<'_, S>)` — [`Context`](#context)

- `fn event_enabled(self: &Self, event: &Event<'_>, ctx: Context<'_, S>) -> bool` — [`Context`](#context)

- `fn on_event(self: &Self, event: &Event<'_>, ctx: Context<'_, S>)` — [`Context`](#context)

- `fn on_enter(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](#context)

- `fn on_exit(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](#context)

- `fn on_close(self: &Self, id: span::Id, ctx: Context<'_, S>)` — [`Context`](#context)

- `fn on_id_change(self: &Self, old: &span::Id, new: &span::Id, ctx: Context<'_, S>)` — [`Context`](#context)

##### `impl<'a, L, S> LookupSpan for Layered<L, S>`

- `type Data = <S as LookupSpan>::Data`

- `fn span_data(self: &'a Self, id: &span::Id) -> Option<<Self as >::Data>` — [`LookupSpan`](../registry/index.md)

- `fn register_filter(self: &mut Self) -> FilterId` — [`FilterId`](../filter/index.md)

##### `impl<S> Sealed for Layered<L, I, S>`

##### `impl<L, S> Subscriber for Layered<L, S>`

- `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool`

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

- `fn new_span(self: &Self, span: &span::Attributes<'_>) -> span::Id`

- `fn record(self: &Self, span: &span::Id, values: &span::Record<'_>)`

- `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)`

- `fn event_enabled(self: &Self, event: &Event<'_>) -> bool`

- `fn event(self: &Self, event: &Event<'_>)`

- `fn enter(self: &Self, span: &span::Id)`

- `fn exit(self: &Self, span: &span::Id)`

- `fn clone_span(self: &Self, old: &span::Id) -> span::Id`

- `fn drop_span(self: &Self, id: span::Id)`

- `fn try_close(self: &Self, id: span::Id) -> bool`

- `fn current_span(self: &Self) -> span::Current`

##### `impl<S> SubscriberExt for Layered<L, I, S>`

##### `impl<T> SubscriberInitExt for Layered<L, I, S>`

##### `impl<T> WithSubscriber for Layered<L, I, S>`

## Traits

### `Layer<S>`

```rust
trait Layer<S>
where
    S: Subscriber,
    Self: 'static { ... }
```

A composable handler for `tracing` events.

A `Layer` implements a behavior for recording or collecting traces that can
be composed together with other `Layer`s to build a [`Subscriber`](../fmt/index.md). See the
[module-level documentation](crate::layer) for details.


#### Required Methods

- `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`

  Performs late initialization when installing this layer as a

- `fn on_layer(self: &mut Self, subscriber: &mut S)`

  Performs late initialization when attaching a `Layer` to a

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

  Registers a new callsite with this layer, returning whether or not

- `fn enabled(self: &Self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool`

  Returns `true` if this layer is interested in a span or event with the

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)`

  Notifies this layer that a new span was constructed with the given

- `fn on_record(self: &Self, _span: &span::Id, _values: &span::Record<'_>, _ctx: Context<'_, S>)`

  Notifies this layer that a span with the given `Id` recorded the given

- `fn on_follows_from(self: &Self, _span: &span::Id, _follows: &span::Id, _ctx: Context<'_, S>)`

  Notifies this layer that a span with the ID `span` recorded that it

- `fn event_enabled(self: &Self, _event: &Event<'_>, _ctx: Context<'_, S>) -> bool`

  Called before `on_event`, to determine if `on_event` should be called.

- `fn on_event(self: &Self, _event: &Event<'_>, _ctx: Context<'_, S>)`

  Notifies this layer that an event has occurred.

- `fn on_enter(self: &Self, _id: &span::Id, _ctx: Context<'_, S>)`

  Notifies this layer that a span with the given ID was entered.

- `fn on_exit(self: &Self, _id: &span::Id, _ctx: Context<'_, S>)`

  Notifies this layer that the span with the given ID was exited.

- `fn on_close(self: &Self, _id: span::Id, _ctx: Context<'_, S>)`

  Notifies this layer that the span with the given ID has been closed.

- `fn on_id_change(self: &Self, _old: &span::Id, _new: &span::Id, _ctx: Context<'_, S>)`

  Notifies this layer that a span ID has been cloned, and that the

- `fn and_then<L>(self: Self, layer: L) -> Layered<L, Self, S>`

  Composes this layer around the given `Layer`, returning a `Layered`

- `fn with_subscriber(self: Self, inner: S) -> Layered<Self, S>`

   Composes this `Layer` with the given [`Subscriber`](../fmt/index.md), returning a

- `fn with_filter<F>(self: Self, filter: F) -> filter::Filtered<Self, F, S>`

  Combines `self` with a [`Filter`](#filter), returning a [`Filtered`](../filter/index.md) layer.

- `fn boxed(self: Self) -> Box<dyn Layer<S> + Send + Sync>`

  Erases the type of this [`Layer`](#layer), returning a [`Box`](../../allocator_api2/stable/boxed/index.md)ed `dyn

### `Filter<S>`

```rust
trait Filter<S> { ... }
```

A per-[`Layer`](#layer) filter that determines whether a span or event is enabled
for an individual layer.

See [the module-level documentation][plf] for details on using [`Filter`](#filter)s.


#### Required Methods

- `fn enabled(self: &Self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool`

  Returns `true` if this layer is interested in a span or event with the

- `fn callsite_enabled(self: &Self, meta: &'static Metadata<'static>) -> Interest`

  Returns an `Interest` indicating whether this layer will [always],

- `fn event_enabled(self: &Self, event: &Event<'_>, cx: &Context<'_, S>) -> bool`

  Called before the filtered [`Layer]'s `on_event`, to determine if

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

  Returns an optional hint of the highest [verbosity level][`level`](../filter/level/index.md) that

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)`

  Notifies this filter that a new span was constructed with the given

- `fn on_record(self: &Self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)`

  Notifies this filter that a span with the given `Id` recorded the given

- `fn on_enter(self: &Self, id: &span::Id, ctx: Context<'_, S>)`

  Notifies this filter that a span with the given ID was entered.

- `fn on_exit(self: &Self, id: &span::Id, ctx: Context<'_, S>)`

  Notifies this filter that a span with the given ID was exited.

- `fn on_close(self: &Self, id: span::Id, ctx: Context<'_, S>)`

  Notifies this filter that a span with the given ID has been closed.

### `SubscriberExt`

```rust
trait SubscriberExt: Subscriber + crate::sealed::Sealed { ... }
```

Extension trait adding a `with(Layer)` combinator to `Subscriber`s.

#### Required Methods

- `fn with<L>(self: Self, layer: L) -> Layered<L, Self>`

  Wraps `self` with the provided `layer`.

