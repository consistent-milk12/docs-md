*[tracing_subscriber](../index.md) / [filter](index.md)*

---

# Module `filter`

[`Layer`](../fmt/fmt_layer/index.md)s that control which spans and events are enabled by the wrapped
subscriber.

This module contains a number of types that provide implementations of
various strategies for filtering which spans and events are enabled. For
details on filtering spans and events using [`Layer`](../fmt/fmt_layer/index.md)s, see the
[`layer` module's documentation].



## Modules

- [`targets`](targets/index.md) - A [filter] that enables or disables spans and events based on their [target] and [level].
- [`combinator`](combinator/index.md) - Filter combinators

## Structs

### `Targets`

```rust
struct Targets(crate::filter::directive::DirectiveSet<crate::filter::directive::StaticDirective>);
```

 A filter that enables or disables spans and events based on their [`target`](../../tracing_attributes/attr/kw/index.md)
 and [`level`](level/index.md).

 Targets are typically equal to the Rust module path of the code where the
 span or event was recorded, although they may be overridden.

 This type can be used for both [per-layer filtering][plf] (using its
 [`Filter`](../layer/index.md) implementation) and [global filtering][`global`](../../allocator_api2/stable/alloc/global/index.md) (using its
 [`Layer`](../fmt/fmt_layer/index.md) implementation).

 See the [documentation on filtering with layers][filtering] for details.

 # Filtering With `Targets`

 A `Targets` filter consists of one or more [`target`](../../tracing_attributes/attr/kw/index.md) prefixes, paired with
 [`LevelFilter`](../../tracing_core/metadata/index.md)s. If a span or event's [`target`](../../tracing_attributes/attr/kw/index.md) begins with one of those
 prefixes, and its [`level`](level/index.md) is at or below the [`LevelFilter`](../../tracing_core/metadata/index.md) enabled for
 that prefix, then the span or event will be enabled.

 This is similar to the behavior implemented by the [`env_logger` crate] in
 the `log` ecosystem.

 The [`EnvFilter`](#envfilter) type also provided by this crate is very similar to `Targets`,
 but is capable of a more sophisticated form of filtering where events may
 also be enabled or disabled based on the span they are recorded in.
 `Targets` can be thought of as a lighter-weight form of [`EnvFilter`](#envfilter) that
 can be used instead when this dynamic filtering is not required.

 # Examples

 A `Targets` filter can be constructed by programmatically adding targets and
 levels to enable:

 ```rust
 use tracing_subscriber::{filter, prelude::*};
 use tracing_core::Level;

 let filter = filter::Targets::new()
     // Enable the `INFO` level for anything in `my_crate`
     .with_target("my_crate", Level::INFO)
     // Enable the `DEBUG` level for a specific module.
     .with_target("my_crate::interesting_module", Level::DEBUG);

 // Build a new subscriber with the `fmt` layer using the `Targets`
 // filter we constructed above.
 tracing_subscriber::registry()
     .with(tracing_subscriber::fmt::layer())
     .with(filter)
     .init();
 ```

 `LevelFilter::OFF` can be used to disable a particular target:
 ```rust
 use tracing_subscriber::filter::{Targets, LevelFilter};
 use tracing_core::Level;

 let filter = Targets::new()
     .with_target("my_crate", Level::INFO)
     // Disable all traces from `annoying_module`.
     .with_target("my_crate::annoying_module", LevelFilter::OFF);
 drop(filter);
 ```

 Alternatively, `Targets` implements `std::str::FromStr`, allowing it to be
 parsed from a comma-delimited list of `target=level` pairs. For example:

 ```rust
 fn main() -> Result<(), Box<dyn std::error::Error>> {
 use tracing_subscriber::filter;
 use tracing_core::Level;

 let filter = "my_crate=info,my_crate::interesting_module=trace,other_crate=debug"
     .parse::<filter::Targets>()?;

 // The parsed filter is identical to a filter constructed using `with_target`:
 assert_eq!(
     filter,
     filter::Targets::new()
         .with_target("my_crate", Level::INFO)
         .with_target("my_crate::interesting_module", Level::TRACE)
         .with_target("other_crate", Level::DEBUG)
 );
 Ok(()) }
 ```

 This is particularly useful when the list of enabled targets is configurable
 by the user at runtime.

 The `Targets` filter can be used as a [per-layer filter][plf] *and* as a
 [global filter][`global`](../../allocator_api2/stable/alloc/global/index.md):

 ```rust
 use tracing_subscriber::{
     fmt,
     filter::{Targets, LevelFilter},
     prelude::*,
 };
 use tracing_core::Level;
 use std::{sync::Arc, fs::File};
 fn docs() -> Result<(), Box<dyn std::error::Error>> {

 // A layer that logs events to stdout using the human-readable "pretty"
 // format.
 let stdout_log = fmt::layer().pretty();

 // A layer that logs events to a file, using the JSON format.
 let file = File::create("debug_log.json")?;
 let debug_log = fmt::layer()
     .with_writer(Arc::new(file))
     .json();

 tracing_subscriber::registry()
     // Only log INFO and above to stdout, unless the span or event
     // has the `my_crate::cool_module` target prefix.
     .with(stdout_log
         .with_filter(
             Targets::default()
                 .with_target("my_crate::cool_module", Level::DEBUG)
                 .with_default(Level::INFO)
        )
     )
     // Log everything enabled by the global filter to `debug_log.json`.
     .with(debug_log)
     // Configure a global filter for the whole subscriber stack. This will
     // control what spans and events are recorded by both the `debug_log`
     // and the `stdout_log` layers, and `stdout_log` will *additionally* be
     // filtered by its per-layer filter.
     .with(
         Targets::default()
             .with_target("my_crate", Level::TRACE)
             .with_target("other_crate", Level::INFO)
             .with_target("other_crate::annoying_module", LevelFilter::OFF)
             .with_target("third_crate", Level::DEBUG)
     ).init();
 Ok(()) }
```










#### Implementations

- `fn new() -> Self`

- `fn with_target(self: Self, target: impl Into<String>, level: impl Into<LevelFilter>) -> Self`

- `fn with_targets<T, L>(self: Self, targets: impl IntoIterator<Item = (T, L)>) -> Self`

- `fn with_default(self: Self, level: impl Into<LevelFilter>) -> Self`

- `fn default_level(self: &Self) -> Option<LevelFilter>`

- `fn iter(self: &Self) -> Iter<'_>` — [`Iter`](targets/index.md)

- `fn interested(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn would_enable(self: &Self, target: &str, level: &Level) -> bool`

#### Trait Implementations

##### `impl Clone for Targets`

- `fn clone(self: &Self) -> Targets` — [`Targets`](targets/index.md)

##### `impl Debug for Targets`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Targets`

- `fn default() -> Targets` — [`Targets`](targets/index.md)

##### `impl Display for Targets`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, L> Extend for Targets`

- `fn extend<I: IntoIterator<Item = (T, L)>>(self: &mut Self, iter: I)`

##### `impl<S> Filter for Targets`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, _: &layer::Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

##### `impl<F, S> FilterExt for Targets`

##### `impl<T, L> FromIterator for Targets`

- `fn from_iter<I: IntoIterator<Item = (T, L)>>(iter: I) -> Self`

##### `impl FromStr for Targets`

- `type Err = ParseError`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<T> Instrument for Targets`

##### `impl IntoIterator for Targets`

- `type Item = (String, LevelFilter)`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<S> Layer for Targets`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, _: layer::Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

##### `impl PartialEq for Targets`

- `fn eq(self: &Self, other: &Targets) -> bool` — [`Targets`](targets/index.md)

##### `impl StructuralPartialEq for Targets`

##### `impl<T> ToString for Targets`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for Targets`

### `ParseError`

```rust
struct ParseError {
    kind: ParseErrorKind,
}
```

Indicates that a string could not be parsed as a filtering directive.

#### Implementations

- `fn new() -> Self`

- `fn msg(s: &'static str) -> Self`

#### Trait Implementations

##### `impl Debug for ParseError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ParseError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseError`

- `fn description(self: &Self) -> &str`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl<T> Instrument for ParseError`

##### `impl<T> ToString for ParseError`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for ParseError`

### `EnvFilter`

```rust
struct EnvFilter {
    statics: crate::filter::directive::DirectiveSet<crate::filter::directive::StaticDirective>,
    dynamics: crate::filter::directive::DirectiveSet<Directive>,
    has_dynamics: bool,
    by_id: std::sync::RwLock<std::collections::HashMap<span::Id, MatchSet<field::SpanMatch>>>,
    by_cs: std::sync::RwLock<std::collections::HashMap<callsite::Identifier, MatchSet<field::CallsiteMatch>>>,
    scope: thread_local::ThreadLocal<core::cell::RefCell<alloc::vec::Vec<crate::filter::LevelFilter>>>,
    regex: bool,
}
```

A [`Layer`](../layer/index.md) which filters spans and events based on a set of filter
directives.

`EnvFilter` implements both the [`Layer`](#impl-Layer<S>) and [`Filter`](../layer/index.md) traits, so it may
be used for both [global filtering][`global`](../../allocator_api2/stable/alloc/global/index.md) and [per-layer filtering][plf],
respectively. See [the documentation on filtering with `Layer`s][filtering]
for details.

The [`Targets`](targets/index.md) type implements a similar form of filtering, but without the
ability to dynamically enable events based on the current span context, and
without filtering on field values. When these features are not required,
[`Targets`](targets/index.md) provides a lighter-weight alternative to [`EnvFilter`](#envfilter).

# Directives

A filter consists of one or more comma-separated directives which match on [`Span`](#span)s and `Event`s.
Each directive may have a corresponding maximum verbosity [`level`](level/index.md) which
enables (e.g., _selects for_) spans and events that match. Like `log`,
`tracing` considers less exclusive levels (like `trace` or `info`) to be more
verbose than more exclusive levels (like `error` or `warn`).

The directive syntax is similar to that of `env_logger`'s. At a high level, the syntax for directives
consists of several parts:

```text
target[span{field=value}]=level
```

Each component (`target`, `span`, `field`, `value`, and `level`) will be covered in turn.

- `target` matches the event or span's target. In general, this is the module path and/or crate name.
  Examples of targets `h2`, `tokio::net`, or `tide::server`. For more information on targets,
  please refer to `Metadata`'s documentation.
- `span` matches on the span's name. If a `span` directive is provided alongside a `target`,
  the `span` directive will match on spans _within_ the `target`.
- `field` matches on [`fields`](../../tracing_attributes/attr/kw/index.md) within spans. Field names can also be supplied without a `value`
  and will match on any [`Span`](#span) or `Event` that has a field with that name.
  For example: `[span{field=\"value\"}]=debug`, `[{field}]=trace`.
- `value` matches on the value of a span's field. If a value is a numeric literal or a bool,
  it will match _only_ on that value. Otherwise, this filter matches the
  [`std::fmt::Debug`](../../docs_md/index.md) output from the value.
- `level` sets a maximum verbosity level accepted by this directive.

When a field value directive (`[{<FIELD NAME>=<FIELD_VALUE>}]=...`) matches a
value's [`std::fmt::Debug`](../../docs_md/index.md) output (i.e., the field value in the directive
is not a `bool`, `i64`, `u64`, or `f64` literal), the matched pattern may be
interpreted as either a regular expression or as the precise expected
output of the field's [`std::fmt::Debug`](../../docs_md/index.md) implementation. By default, these
filters are interpreted as regular expressions, but this can be disabled
using the `Builder::with_regex` builder method to use precise matching
instead.

When field value filters are interpreted as regular expressions, the
[`regex` crate's regular expression syntax][re-syntax] is supported.

**Note**: When filters are constructed from potentially untrusted inputs,
[disabling regular expression matching](Builder::with_regex) is strongly
recommended.

## Usage Notes

- The portion of the directive which is included within the square brackets is `tracing`-specific.
- Any portion of the directive can be omitted.
    - The sole exception are the `field` and `value` directives. If a `value` is provided,
      a `field` must _also_ be provided. However, the converse does not hold, as fields can
      be matched without a value.
- If only a level is provided, it will set the maximum level for all `Span`s and `Event`s
  that are not enabled by other filters.
- A directive without a level will enable anything that it matches. This is equivalent to `=trace`.
- When a crate has a dash in its name, the default target for events will be the
  crate's module path as it appears in Rust. This means every dash will be replaced
  with an underscore.
- A dash in a target will only appear when being specified explicitly:
  `tracing::info!(target: "target-name", ...);`

## Example Syntax

- `tokio::net=info` will enable all spans or events that:
   - have the `tokio::net` target,
   - at the level `info` or above.
- `warn,tokio::net=info` will enable all spans and events that:
   - are at the level `warn` or above, *or*
   - have the `tokio::net` target at the level `info` or above.
- `my_crate[span_a]=trace` will enable all spans and events that:
   - are within the `span_a` span or named `span_a` _if_ `span_a` has the target `my_crate`,
   - at the level `trace` or above.
- `[span_b{name=\"bob\"}]` will enable all spans or event that:
   - have _any_ target,
   - are inside a span named `span_b`,
   - which has a field named `name` with value `bob`,
   - at _any_ level.

# Examples

Parsing an `EnvFilter` from the [default environment
variable](EnvFilter::from_default_env) (`RUST_LOG`):

```rust
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_default_env())
    .init();
```

Parsing an `EnvFilter` [from a user-provided environment
variable](EnvFilter::from_env):

```rust
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_env("MYAPP_LOG"))
    .init();
```

Using `EnvFilter` as a [per-layer filter][plf] to filter only a single

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// Parse an `EnvFilter` configuration from the `RUST_LOG`
// environment variable.
let filter = EnvFilter::from_default_env();

// Apply the filter to this layer *only*.
let filtered_layer = fmt::layer().with_filter(filter);

// Some other layer, whose output we don't want to filter.
let unfiltered_layer = // ...
    # fmt::layer();

tracing_subscriber::registry()
    .with(filtered_layer)
    .with(unfiltered_layer)
    .init();
```rust
Constructing `EnvFilter`s

An `EnvFilter` is be constructed by parsing a string containing one or more
directives. The `EnvFilter::new` constructor parses an `EnvFilter` from a
string, ignoring any invalid directives, while `EnvFilter::try_new`
returns an error if invalid directives are encountered. Similarly, the
`EnvFilter::from_env` and `EnvFilter::try_from_env` constructors parse
an `EnvFilter` from the value of the provided environment variable, with
lossy and strict validation, respectively.

A [builder](EnvFilter::builder) interface is available to set additional
configuration options prior to parsing an `EnvFilter`. See the [`Builder`
type's documentation](Builder) for details on the options that can be
configured using the builder.













#### Implementations

- `const DEFAULT_ENV: &'static str`

- `fn builder() -> Builder` — [`Builder`](env/builder/index.md)

- `fn from_default_env() -> Self`

- `fn from_env<A: AsRef<str>>(env: A) -> Self`

- `fn new<S: AsRef<str>>(directives: S) -> Self`

- `fn try_new<S: AsRef<str>>(dirs: S) -> Result<Self, crate::filter::directive::ParseError>` — [`ParseError`](directive/index.md)

- `fn try_from_default_env() -> Result<Self, FromEnvError>` — [`FromEnvError`](#fromenverror)

- `fn try_from_env<A: AsRef<str>>(env: A) -> Result<Self, FromEnvError>` — [`FromEnvError`](#fromenverror)

- `fn add_directive(self: Self, directive: Directive) -> Self` — [`Directive`](env/directive/index.md)

- `fn enabled<S>(self: &Self, metadata: &Metadata<'_>, _: Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

- `fn on_new_span<S>(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, _: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_enter<S>(self: &Self, id: &span::Id, _: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_exit<S>(self: &Self, id: &span::Id, _: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_close<S>(self: &Self, id: span::Id, _: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_record<S>(self: &Self, id: &span::Id, values: &span::Record<'_>, _: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn cares_about_span(self: &Self, span: &span::Id) -> bool`

- `fn base_interest(self: &Self) -> Interest`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

#### Trait Implementations

##### `impl Clone for EnvFilter`

- `fn clone(self: &Self) -> EnvFilter` — [`EnvFilter`](#envfilter)

##### `impl Debug for EnvFilter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for EnvFilter`

- `fn default() -> Self`

##### `impl Display for EnvFilter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S> Filter for EnvFilter`

- `fn enabled(self: &Self, meta: &Metadata<'_>, ctx: &Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn callsite_enabled(self: &Self, meta: &'static Metadata<'static>) -> Interest`

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_record(self: &Self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_enter(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_exit(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_close(self: &Self, id: span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

##### `impl<F, S> FilterExt for EnvFilter`

##### `impl FromStr for EnvFilter`

- `type Err = ParseError`

- `fn from_str(spec: &str) -> Result<Self, <Self as >::Err>`

##### `impl<T> Instrument for EnvFilter`

##### `impl<S: Subscriber> Layer for EnvFilter`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_record(self: &Self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_enter(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_exit(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_close(self: &Self, id: span::Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

##### `impl<T> ToString for EnvFilter`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for EnvFilter`

### `FromEnvError`

```rust
struct FromEnvError {
    kind: ErrorKind,
}
```

Indicates that an error occurred while parsing a `EnvFilter` from an
environment variable.

#### Trait Implementations

##### `impl Debug for FromEnvError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for FromEnvError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for FromEnvError`

- `fn source(self: &Self) -> Option<&dyn Error>`

##### `impl<T> Instrument for FromEnvError`

##### `impl<T> ToString for FromEnvError`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for FromEnvError`

### `Filtered<L, F, S>`

```rust
struct Filtered<L, F, S> {
    filter: F,
    layer: L,
    id: MagicPlfDowncastMarker,
    _s: core::marker::PhantomData<fn(S)>,
}
```

A [`Layer`](../layer/index.md) that wraps an inner [`Layer`](../layer/index.md) and adds a [`Filter`](../layer/index.md) which
controls what spans and events are enabled for that layer.

This is returned by the `Layer::with_filter` method. See the
[documentation on per-layer filtering][plf] for details.



#### Implementations

- `fn new(layer: L, filter: F) -> Self`

- `fn id(self: &Self) -> FilterId` — [`FilterId`](#filterid)

- `fn did_enable(self: &Self, f: impl FnOnce())`

- `fn filter(self: &Self) -> &F`

- `fn filter_mut(self: &mut Self) -> &mut F`

- `fn inner(self: &Self) -> &L`

- `fn inner_mut(self: &mut Self) -> &mut L`

#### Trait Implementations

##### `impl<L: $crate::clone::Clone, F: $crate::clone::Clone, S: $crate::clone::Clone> Clone for Filtered<L, F, S>`

- `fn clone(self: &Self) -> Filtered<L, F, S>` — [`Filtered`](#filtered)

##### `impl<F, L, S> Debug for Filtered<F, L, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for Filtered<L, F, S>`

##### `impl<S, L, F> Layer for Filtered<L, F, S>`

- `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`

- `fn on_layer(self: &mut Self, subscriber: &mut S)`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, cx: Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_record(self: &Self, span: &span::Id, values: &span::Record<'_>, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_follows_from(self: &Self, span: &span::Id, follows: &span::Id, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn event_enabled(self: &Self, event: &Event<'_>, cx: Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn on_event(self: &Self, event: &Event<'_>, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_enter(self: &Self, id: &span::Id, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_exit(self: &Self, id: &span::Id, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_close(self: &Self, id: span::Id, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_id_change(self: &Self, old: &span::Id, new: &span::Id, cx: Context<'_, S>)` — [`Context`](../layer/index.md)

##### `impl<T> WithSubscriber for Filtered<L, F, S>`

### `FilterId`

```rust
struct FilterId(u64);
```

Uniquely identifies an individual [`Filter`](../layer/index.md) instance in the context of
a [`Subscriber`](../fmt/index.md).

When adding a [`Filtered`](#filtered) [`Layer`](../fmt/fmt_layer/index.md) to a [`Subscriber`](../fmt/index.md), the [`Subscriber`](../fmt/index.md)
generates a `FilterId` for that [`Filtered`](#filtered) layer. The [`Filtered`](#filtered) layer
will then use the generated ID to query whether a particular span was
previously enabled by that layer's [`Filter`](../layer/index.md).

**Note**: Currently, the [`Registry`](../registry/sharded/index.md) type provided by this crate is the
**only** [`Subscriber`](../fmt/index.md) implementation capable of participating in per-layer
filtering. Therefore, the `FilterId` type cannot currently be constructed by
code outside of `tracing-subscriber`. In the future, new APIs will be added to `tracing-subscriber` to
allow non-Registry [`Subscriber`](../fmt/index.md)s to also participate in per-layer
filtering. When those APIs are added, subscribers will be responsible
for generating and assigning `FilterId`s.





#### Implementations

- `const fn disabled() -> Self`

- `const fn none() -> Self`

- `fn new(id: u8) -> Self`

- `fn and(self: Self, FilterId: Self) -> Self`

- `fn is_disabled(self: Self) -> bool`

#### Trait Implementations

##### `impl Binary for FilterId`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Clone for FilterId`

- `fn clone(self: &Self) -> FilterId` — [`FilterId`](#filterid)

##### `impl Copy for FilterId`

##### `impl Debug for FilterId`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for FilterId`

##### `impl<T> WithSubscriber for FilterId`

### `FilterFn<F>`

```rust
struct FilterFn<F> {
    enabled: F,
    max_level_hint: Option<crate::filter::LevelFilter>,
}
```

A filter implemented by a closure or function pointer that
determines whether a given span or event is enabled, based on its
`Metadata`.

This type can be used for both [per-layer filtering][plf] (using its
[`Filter`](../layer/index.md) implementation) and [global filtering][`global`](../../allocator_api2/stable/alloc/global/index.md) (using its
[`Layer`](../fmt/fmt_layer/index.md) implementation).

See the [documentation on filtering with layers][filtering] for details.







#### Implementations

- `fn new(enabled: F) -> Self`

- `fn with_max_level_hint(self: Self, max_level_hint: impl Into<LevelFilter>) -> Self`

- `fn is_enabled(self: &Self, metadata: &Metadata<'_>) -> bool`

- `fn is_callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn is_below_max_level(self: &Self, metadata: &Metadata<'_>) -> bool`

#### Trait Implementations

##### `impl<F: $crate::clone::Clone> Clone for FilterFn<F>`

- `fn clone(self: &Self) -> FilterFn<F>` — [`FilterFn`](#filterfn)

##### `impl<F> Debug for FilterFn<F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, F> Filter for FilterFn<F>`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, _: &Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

##### `impl<F, S> FilterExt for FilterFn<F>`

##### `impl<T> Instrument for FilterFn<F>`

##### `impl<S, F> Layer for FilterFn<F>`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, _: Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

##### `impl<T> WithSubscriber for FilterFn<F>`

### `DynFilterFn<S, F, R>`

```rust
struct DynFilterFn<S, F, R> {
    enabled: F,
    register_callsite: Option<R>,
    max_level_hint: Option<crate::filter::LevelFilter>,
    _s: core::marker::PhantomData<fn(S)>,
}
```

A filter implemented by a closure or function pointer that
determines whether a given span or event is enabled _dynamically_,
potentially based on the current [span context].

This type can be used for both [per-layer filtering][plf] (using its
[`Filter`](../layer/index.md) implementation) and [global filtering][`global`](../../allocator_api2/stable/alloc/global/index.md) (using its
[`Layer`](../fmt/fmt_layer/index.md) implementation).

See the [documentation on filtering with layers][filtering] for details.







#### Implementations

- `fn new(enabled: F) -> Self`

#### Trait Implementations

##### `impl<S, F, R> Clone for DynFilterFn<S, F, R>`

- `fn clone(self: &Self) -> Self`

##### `impl<S, F, R> Debug for DynFilterFn<S, F, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, F, R> Filter for DynFilterFn<S, F, R>`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, cx: &Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

##### `impl<F, S> FilterExt for DynFilterFn<S, F, R>`

##### `impl<T> Instrument for DynFilterFn<S, F, R>`

##### `impl<S, F, R> Layer for DynFilterFn<S, F, R>`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, cx: Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

##### `impl<T> WithSubscriber for DynFilterFn<S, F, R>`

## Traits

### `FilterExt<S>`

```rust
trait FilterExt<S>: layer::Filter<S> { ... }
```

Extension trait adding [combinators] for combining [`Filter`](../layer/index.md).



#### Required Methods

- `fn and<B>(self: Self, other: B) -> combinator::And<Self, B, S>`

  Combines this [`Filter`](../layer/index.md) with another [`Filter`](../layer/index.md) s so that spans and

- `fn or<B>(self: Self, other: B) -> combinator::Or<Self, B, S>`

  Combines two [`Filter`](../layer/index.md)s so that spans and events are enabled if *either* filter

- `fn not(self: Self) -> combinator::Not<Self, S>`

  Inverts `self`, returning a filter that enables spans and events only if

- `fn boxed(self: Self) -> Box<dyn layer::Filter<S> + Send + Sync>`

  [Boxes] `self`, erasing its concrete type.

## Functions

### `filter_fn`

```rust
fn filter_fn<F>(f: F) -> FilterFn<F>
where
    F: Fn(&tracing_core::Metadata<'_>) -> bool
```

Constructs a [`FilterFn`](#filterfn), from a function or closure that returns `true` if
a span or event should be enabled, based on its `Metadata`.

The returned [`FilterFn`](#filterfn) can be used for both [per-layer filtering][plf]
(using its [`Filter`](../layer/index.md) implementation) and [global filtering][`global`](../../allocator_api2/stable/alloc/global/index.md) (using
its  [`Layer`](../fmt/fmt_layer/index.md) implementation).

See the [documentation on filtering with layers][filtering] for details.

This is equivalent to calling `FilterFn::new`.






# Examples

```rust
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter,
    util::SubscriberInitExt,
};

let my_filter = filter::filter_fn(|metadata| {
    // Only enable spans or events with the target "interesting_things"
    metadata.target() == "interesting_things"
});

let my_layer = tracing_subscriber::fmt::layer();

tracing_subscriber::registry()
    .with(my_layer.with_filter(my_filter))
    .init();

// This event will not be enabled.
tracing::warn!("something important but uninteresting happened!");

// This event will be enabled.
tracing::debug!(target: "interesting_things", "an interesting minor detail...");
```

### `dynamic_filter_fn`

```rust
fn dynamic_filter_fn<S, F>(f: F) -> DynFilterFn<S, F>
where
    F: Fn(&tracing_core::Metadata<'_>, &crate::layer::Context<'_, S>) -> bool
```

Constructs a [`DynFilterFn`](#dynfilterfn) from a function or closure that returns `true`
if a span or event should be enabled within a particular [span context][`Context`](../layer/index.md).

This is equivalent to calling `DynFilterFn::new`.

Unlike [`filter_fn`](#filter-fn), this function takes a closure or function pointer
taking the `Metadata` for a span or event *and* the current [`Context`](../layer/index.md).
This means that a [`DynFilterFn`](#dynfilterfn) can choose whether to enable spans or
events based on information about the _current_ span (or its parents).

If this is *not* necessary, use [`filter_fn`](#filter-fn) instead.

The returned [`DynFilterFn`](#dynfilterfn) can be used for both [per-layer filtering][plf]
(using its [`Filter`](../layer/index.md) implementation) and [global filtering][`global`](../../allocator_api2/stable/alloc/global/index.md) (using
its  [`Layer`](../fmt/fmt_layer/index.md) implementation).

See the [documentation on filtering with layers][filtering] for details.

# Examples

```rust
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter,
    util::SubscriberInitExt,
};

// Only enable spans or events within a span named "interesting_span".
let my_filter = filter::dynamic_filter_fn(|metadata, cx| {
    // If this *is* "interesting_span", make sure to enable it.
    if metadata.is_span() && metadata.name() == "interesting_span" {
        return true;
    }

    // Otherwise, are we in an interesting span?
    if let Some(current_span) = cx.lookup_current() {
        return current_span.name() == "interesting_span";
    }

    false
});

let my_layer = tracing_subscriber::fmt::layer();

tracing_subscriber::registry()
    .with(my_layer.with_filter(my_filter))
    .init();

// This event will not be enabled.
tracing::info!("something happened");

tracing::info_span!("interesting_span").in_scope(|| {
    // This event will be enabled.
    tracing::debug!("something else happened");
});
```








