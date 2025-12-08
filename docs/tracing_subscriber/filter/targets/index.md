*[tracing_subscriber](../../index.md) / [filter](../index.md) / [targets](index.md)*

---

# Module `targets`

A [`filter`](../index.md) that enables or disables spans and events based on their [`target`](../../../tracing_attributes/attr/kw/index.md) and [`level`](../level/index.md).

See [`Targets`](#targets) for details.




## Structs

### `Targets`

```rust
struct Targets(crate::filter::directive::DirectiveSet<crate::filter::directive::StaticDirective>);
```

 A filter that enables or disables spans and events based on their [`target`](../../../tracing_attributes/attr/kw/index.md)
 and [`level`](../level/index.md).

 Targets are typically equal to the Rust module path of the code where the
 span or event was recorded, although they may be overridden.

 This type can be used for both [per-layer filtering][plf] (using its
 [`Filter`](../../layer/index.md) implementation) and [global filtering][`global`](../../../allocator_api2/stable/alloc/global/index.md) (using its
 [`Layer`](../../fmt/fmt_layer/index.md) implementation).

 See the [documentation on filtering with layers][filtering] for details.

 # Filtering With `Targets`

 A `Targets` filter consists of one or more [`target`](../../../tracing_attributes/attr/kw/index.md) prefixes, paired with
 [`LevelFilter`](../../../tracing_core/metadata/index.md)s. If a span or event's [`target`](../../../tracing_attributes/attr/kw/index.md) begins with one of those
 prefixes, and its [`level`](../level/index.md) is at or below the [`LevelFilter`](../../../tracing_core/metadata/index.md) enabled for
 that prefix, then the span or event will be enabled.

 This is similar to the behavior implemented by the [`env_logger` crate] in
 the `log` ecosystem.

 The [`EnvFilter`](../index.md) type also provided by this crate is very similar to `Targets`,
 but is capable of a more sophisticated form of filtering where events may
 also be enabled or disabled based on the span they are recorded in.
 `Targets` can be thought of as a lighter-weight form of [`EnvFilter`](../index.md) that
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
 [global filter][`global`](../../../allocator_api2/stable/alloc/global/index.md):

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

- `fn iter(self: &Self) -> Iter<'_>` — [`Iter`](#iter)

- `fn interested(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn would_enable(self: &Self, target: &str, level: &Level) -> bool`

#### Trait Implementations

##### `impl Clone for Targets`

- `fn clone(self: &Self) -> Targets` — [`Targets`](#targets)

##### `impl Debug for Targets`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Targets`

- `fn default() -> Targets` — [`Targets`](#targets)

##### `impl Display for Targets`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, L> Extend for Targets`

- `fn extend<I: IntoIterator<Item = (T, L)>>(self: &mut Self, iter: I)`

##### `impl<S> Filter for Targets`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, _: &layer::Context<'_, S>) -> bool` — [`Context`](../../layer/index.md)

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

- `fn enabled(self: &Self, metadata: &Metadata<'_>, _: layer::Context<'_, S>) -> bool` — [`Context`](../../layer/index.md)

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

##### `impl PartialEq for Targets`

- `fn eq(self: &Self, other: &Targets) -> bool` — [`Targets`](#targets)

##### `impl StructuralPartialEq for Targets`

##### `impl<T> ToString for Targets`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for Targets`

### `IntoIter`

```rust
struct IntoIter(core::iter::FilterMap<<crate::filter::directive::DirectiveSet<crate::filter::directive::StaticDirective> as IntoIterator>::IntoIter, fn(crate::filter::directive::StaticDirective) -> Option<(alloc::string::String, crate::filter::LevelFilter)>>);
```

An owning iterator over the [`target`](../../../tracing_attributes/attr/kw/index.md)-[`level`](../level/index.md) pairs of a `Targets` filter.

This struct is created by the `IntoIterator` trait implementation of [`Targets`](#targets).

# Examples

Merge the targets from one `Targets` with another:

```rust
use tracing_subscriber::filter::Targets;
use tracing_core::Level;

let mut filter = Targets::new().with_target("my_crate", Level::INFO);
let overrides = Targets::new().with_target("my_crate::interesting_module", Level::DEBUG);

filter.extend(overrides);
drop(filter);
```



#### Implementations

- `fn new(targets: Targets) -> Self` — [`Targets`](#targets)

#### Trait Implementations

##### `impl Debug for IntoIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for IntoIter`

##### `impl<I> IntoIterator for IntoIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for IntoIter`

- `type Item = (String, LevelFilter)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> WithSubscriber for IntoIter`

### `Iter<'a>`

```rust
struct Iter<'a>(core::iter::FilterMap<slice::Iter<'a, crate::filter::directive::StaticDirective>, fn(&'a crate::filter::directive::StaticDirective) -> Option<(&'a str, crate::filter::LevelFilter)>>);
```

A borrowing iterator over the [`target`](../../../tracing_attributes/attr/kw/index.md)-[`level`](../level/index.md) pairs of a `Targets` filter.

This struct is created by [`iter`](#iter) method of [`Targets`](#targets), or from the `IntoIterator`
implementation for `&Targets`.




#### Implementations

- `fn new(targets: &'a Targets) -> Self` — [`Targets`](#targets)

#### Trait Implementations

##### `impl<'a> Debug for Iter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Iter<'a>`

##### `impl<I> IntoIterator for Iter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Iter<'a>`

- `type Item = (&'a str, LevelFilter)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> WithSubscriber for Iter<'a>`

