*[tracing_log](../index.md) / [log_tracer](index.md)*

---

# Module `log_tracer`

An adapter for converting [`log`](../../log/index.md) records into `tracing` `Event`s.

This module provides the [`LogTracer`](#logtracer) type which implements `log`'s [logger
interface] by recording log records as `tracing` `Event`s. This is intended for
use in conjunction with a `tracing` `Subscriber` to consume events from
dependencies that emit [`log`](../../log/index.md) records within a trace context.

# Usage

To create and initialize a `LogTracer` with the default configurations, use:

* `init` if you want to convert all logs, regardless of log level,
  allowing the tracing `Subscriber` to perform any filtering
* `init_with_filter` to convert all logs up to a specified log level

In addition, a [`builder`](../../aho_corasick/packed/teddy/builder/index.md) is available for cases where more advanced
configuration is required. In particular, the builder can be used to [ignore
log records][ignore] emitted by particular crates. This is useful in cases
such as when a crate emits both `tracing` diagnostics _and_ log records by
default.






## Modules

- [`SetLoggerError`](SetLoggerError/index.md) - 

## Structs

### `LogTracer`

```rust
struct LogTracer {
    ignore_crates: Box<[String]>,
}
```

A simple "logger" that converts all log records into `tracing` `Event`s.

#### Implementations

- `fn builder() -> Builder` — [`Builder`](#builder)

- `fn new() -> Self`

- `fn init_with_filter(level: log::LevelFilter) -> Result<(), SetLoggerError>`

- `fn init() -> Result<(), SetLoggerError>`

#### Trait Implementations

##### `impl Debug for LogTracer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LogTracer`

- `fn default() -> Self`

##### `impl Log for LogTracer`

- `fn enabled(self: &Self, metadata: &log::Metadata<'_>) -> bool`

- `fn log(self: &Self, record: &log::Record<'_>)`

- `fn flush(self: &Self)`

### `Builder`

```rust
struct Builder {
    ignore_crates: Vec<String>,
    filter: log::LevelFilter,
}
```

Configures a new `LogTracer`.

#### Implementations

- `fn new() -> Self`

- `fn with_max_level(self: Self, filter: impl Into<log::LevelFilter>) -> Self`

- `fn ignore_crate(self: Self, name: impl Into<String>) -> Self`

- `fn ignore_all<I>(self: Self, crates: impl IntoIterator<Item = I>) -> Self`

- `fn init(self: Self) -> Result<(), SetLoggerError>`

#### Trait Implementations

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Self`

