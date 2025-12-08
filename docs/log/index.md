# Crate `log`

A lightweight logging facade.

The `log` crate provides a single logging API that abstracts over the
actual logging implementation. Libraries can use the logging API provided
by this crate, and the consumer of those libraries can choose the logging
implementation that is most suitable for its use case.

If no logging implementation is selected, the facade falls back to a "noop"
implementation that ignores all log messages. The overhead in this case
is very small - just an integer load, comparison and jump.

A log request consists of a _target_, a _level_, and a _body_. A target is a
string which defaults to the module path of the location of the log request,
though that default may be overridden. Logger implementations typically use
the target to filter requests based on some user configuration.

# Usage

The basic use of the log crate is through the five logging macros: `error!`,
`warn!`, `info!`, `debug!` and `trace!`
where `error!` represents the highest-priority log messages
and `trace!` the lowest. The log messages are filtered by configuring
the log level to exclude messages with a lower priority.
Each of these macros accept format strings similarly to `println!`.






Avoid writing expressions with side-effects in log statements. They may not be evaluated.

## In libraries

Libraries should link only to the `log` crate, and use the provided
macros to log whatever information will be useful to downstream consumers.

### Examples

```rust
#[derive(Debug)] pub struct Yak(String);
impl Yak { fn shave(&mut self, _: u32) {} }
fn find_a_razor() -> Result<u32, u32> { Ok(1) }
use log::{info, warn};

pub fn shave_the_yak(yak: &mut Yak) {
    info!(target: "yak_events", "Commencing yak shaving for {yak:?}");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!("Razor located: {razor}");
                yak.shave(razor);
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {err}, retrying");
            }
        }
    }
}
fn main() {}
```

## In executables

Executables should choose a logging implementation and initialize it early in the
runtime of the program. Logging implementations will typically include a
function to do this. Any log messages generated before
the implementation is initialized will be ignored.

The executable itself may use the `log` crate to log as well.

### Warning

The logging system may only be initialized once.

## Structured logging

If you enable the `kv` feature you can associate structured values
with your log records. If we take the example from before, we can include
some additional context besides what's in the formatted message:

```rust
use serde::Serialize;
#[derive(Debug, Serialize)] pub struct Yak(String);
impl Yak { fn shave(&mut self, _: u32) {} }
fn find_a_razor() -> Result<u32, std::io::Error> { Ok(1) }
#[cfg(feature = "kv_serde")]
fn main() {
use log::{info, warn};

pub fn shave_the_yak(yak: &mut Yak) {
    info!(target: "yak_events", yak:serde; "Commencing yak shaving");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!(razor; "Razor located");
                yak.shave(razor);
                break;
            }
            Err(e) => {
                warn!(e:err; "Unable to locate a razor, retrying");
            }
        }
    }
}
}
#[cfg(not(feature = "kv_serde"))]
fn main() {}
```

See the `kv` module documentation for more details.

# Available logging implementations

In order to produce log output executables have to use
a logger implementation compatible with the facade.
There are many available implementations to choose from,
here are some of the most popular ones:

* Simple minimal loggers:
    * [env_logger]
    * [colog]
    * [simple_logger]
    * [simplelog]
    * [pretty_env_logger]
    * [stderrlog]
    * [flexi_logger]
    * [call_logger]
    * [std-logger]
    * [structured-logger]
    * [clang_log]
    * [ftail]
* Complex configurable frameworks:
    * [log4rs]
    * [logforth]
    * [fern]
    * [spdlog-rs]
* Adaptors for other facilities:
    * [`syslog`](../libc/index.md)
    * [slog-stdlog]
    * [systemd-journal-logger]
    * [android_log]
    * [win_dbg_logger]
    * [db_logger]
    * [log-to-defmt]
    * [logcontrol-log]
* For WebAssembly binaries:
    * [console_log]
* For dynamic libraries:
    * You may need to construct an FFI-safe wrapper over `log` to initialize in your libraries
* Utilities:
    * [log_err]
    * [log-reload]
    * [alterable_logger]

# Implementing a Logger

Loggers implement the [`Log`](#log) trait. Here's a very basic example that simply
logs all messages at the [`Error`][level_link], [`Warn`][level_link] or
[`Info`][level_link] levels to stdout:

```rust
use log::{Record, Level, Metadata};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() {}
```

Loggers are installed by calling the [`set_logger`](#set-logger) function. The maximum
log level also needs to be adjusted via the [`set_max_level`](#set-max-level) function. The
logging facade uses this as an optimization to improve performance of log
messages at levels that are disabled. It's important to set it, as it
defaults to [`Off`][filter_link], so no log messages will ever be captured!
In the case of our example logger, we'll want to set the maximum log level
to [`Info`][filter_link], since we ignore any [`Debug`][level_link] or
[`Trace`][level_link] level log messages. A logging implementation should
provide a function that wraps a call to [`set_logger`](#set-logger) and
[`set_max_level`](#set-max-level), handling initialization of the logger:

```rust
use log::{Level, Metadata};
struct SimpleLogger;
impl log::Log for SimpleLogger {
  fn enabled(&self, _: &Metadata) -> bool { false }
  fn log(&self, _: &log::Record) {}
  fn flush(&self) {}
}
fn main() {}
use log::{SetLoggerError, LevelFilter};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

Implementations that adjust their configurations at runtime should take care
to adjust the maximum log level as well.

# Use with `std`

`set_logger` requires you to provide a `&'static Log`, which can be hard to
obtain if your logger depends on some runtime configuration. The
`set_boxed_logger` function is available with the `std` Cargo feature. It is
identical to `set_logger` except that it takes a `Box<Log>` rather than a
`&'static Log`:

```rust
use log::{Level, LevelFilter, Log, SetLoggerError, Metadata};
struct SimpleLogger;
impl log::Log for SimpleLogger {
  fn enabled(&self, _: &Metadata) -> bool { false }
  fn log(&self, _: &log::Record) {}
  fn flush(&self) {}
}
fn main() {}
#[cfg(feature = "std")]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger))
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

# Compile time filters

Log levels can be statically disabled at compile time by enabling one of these Cargo features:

* `max_level_off`
* `max_level_error`
* `max_level_warn`
* `max_level_info`
* `max_level_debug`
* `max_level_trace`

Log invocations at disabled levels will be skipped and will not even be present in the
resulting binary. These features control the value of the `STATIC_MAX_LEVEL` constant. The
logging macros check this value before logging a message. By default, no levels are disabled.

It is possible to override this level for release builds only with the following features:

* `release_max_level_off`
* `release_max_level_error`
* `release_max_level_warn`
* `release_max_level_info`
* `release_max_level_debug`
* `release_max_level_trace`

Libraries should avoid using the max level features because they're global and can't be changed
once they're set.

For example, a crate can disable trace level logs in debug builds and trace, debug, and info
level logs in release builds with the following configuration:

```toml
[dependencies]
log = { version = "0.4", features = ["max_level_debug", "release_max_level_warn"] }
```
# Crate Feature Flags

The following crate feature flags are available in addition to the filters. They are
configured in your `Cargo.toml`.

* `std` allows use of `std` crate instead of the default `core`. Enables using `std::error` and
  `set_boxed_logger` functionality.
* `serde` enables support for serialization and deserialization of `Level` and `LevelFilter`.

```toml
[dependencies]
log = { version = "0.4", features = ["std", "serde"] }
```

# Version compatibility

The 0.3 and 0.4 versions of the `log` crate are almost entirely compatible. Log messages
made using `log` 0.3 will forward transparently to a logger implementation using `log` 0.4. Log
messages made using `log` 0.4 will forward to a logger implementation using `log` 0.3, but the
module path and file name information associated with the message will unfortunately be lost.




































## Structs

### `Record<'a>`

```rust
struct Record<'a> {
    metadata: Metadata<'a>,
    args: fmt::Arguments<'a>,
    module_path: Option<MaybeStaticStr<'a>>,
    file: Option<MaybeStaticStr<'a>>,
    line: Option<u32>,
}
```

The "payload" of a log message.

# Use

`Record` structures are passed as parameters to the [`log`][method.log]
method of the [`Log`](#log) trait. Logger implementors manipulate these
structures in order to display log messages. `Record`s are automatically
created by the `log!` macro and so are not seen by log users.

Note that the `level()` and `target()` accessors are equivalent to
`self.metadata().level()` and `self.metadata().target()` respectively.
These methods are provided as a convenience for users of this structure.

# Example

The following example shows a simple logger that displays the level,
module path, and message of any `Record` that is passed to it.

```rust
struct SimpleLogger;

impl log::Log for SimpleLogger {
   fn enabled(&self, _metadata: &log::Metadata) -> bool {
       true
   }

   fn log(&self, record: &log::Record) {
       if !self.enabled(record.metadata()) {
           return;
       }

       println!("{}:{} -- {}",
                record.level(),
                record.target(),
                record.args());
   }
   fn flush(&self) {}
}
```






#### Implementations

- `fn builder() -> RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn args(self: &Self) -> &fmt::Arguments<'a>`

- `fn metadata(self: &Self) -> &Metadata<'a>` — [`Metadata`](#metadata)

- `fn level(self: &Self) -> Level` — [`Level`](#level)

- `fn target(self: &Self) -> &'a str`

- `fn module_path(self: &Self) -> Option<&'a str>`

- `fn module_path_static(self: &Self) -> Option<&'static str>`

- `fn file(self: &Self) -> Option<&'a str>`

- `fn file_static(self: &Self) -> Option<&'static str>`

- `fn line(self: &Self) -> Option<u32>`

#### Trait Implementations

##### `impl<'a> AsTrace for log::Record<'a>`

- `fn from_str(level: &str) -> Result<Level, <Self as >::Err>` — [`Level`](#level)

##### `impl<'a> Clone for Record<'a>`

- `fn clone(self: &Self) -> Record<'a>` — [`Record`](#record)

##### `impl<'a> Debug for Record<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> Sealed for log::Record<'a>`

### `RecordBuilder<'a>`

```rust
struct RecordBuilder<'a> {
    record: Record<'a>,
}
```

Builder for [`Record`](#record).

Typically should only be used by log library creators or for testing and "shim loggers".
The `RecordBuilder` can set the different parameters of `Record` object, and returns
the created object when `build` is called.

# Examples

```rust
use log::{Level, Record};

let record = Record::builder()
                .args(format_args!("Error!"))
                .level(Level::Error)
                .target("myApp")
                .file(Some("server.rs"))
                .line(Some(144))
                .module_path(Some("server"))
                .build();
```

Alternatively, use [`MetadataBuilder`](#metadatabuilder):

```rust
use log::{Record, Level, MetadataBuilder};

let error_metadata = MetadataBuilder::new()
                        .target("myApp")
                        .level(Level::Error)
                        .build();

let record = Record::builder()
                .metadata(error_metadata)
                .args(format_args!("Error!"))
                .line(Some(433))
                .file(Some("app.rs"))
                .module_path(Some("server"))
                .build();
```

#### Implementations

- `fn new() -> RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn args(self: &mut Self, args: fmt::Arguments<'a>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn metadata(self: &mut Self, metadata: Metadata<'a>) -> &mut RecordBuilder<'a>` — [`Metadata`](#metadata), [`RecordBuilder`](#recordbuilder)

- `fn level(self: &mut Self, level: Level) -> &mut RecordBuilder<'a>` — [`Level`](#level), [`RecordBuilder`](#recordbuilder)

- `fn target(self: &mut Self, target: &'a str) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn module_path(self: &mut Self, path: Option<&'a str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn module_path_static(self: &mut Self, path: Option<&'static str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn file(self: &mut Self, file: Option<&'a str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn file_static(self: &mut Self, file: Option<&'static str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn line(self: &mut Self, line: Option<u32>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

- `fn build(self: &Self) -> Record<'a>` — [`Record`](#record)

#### Trait Implementations

##### `impl<'a> Debug for RecordBuilder<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RecordBuilder<'_>`

- `fn default() -> Self`

### `Metadata<'a>`

```rust
struct Metadata<'a> {
    level: Level,
    target: &'a str,
}
```

Metadata about a log message.

# Use

`Metadata` structs are created when users of the library use
logging macros.

They are consumed by implementations of the `Log` trait in the
`enabled` method.

`Record`s use `Metadata` to determine the log message's severity
and target.

Users should use the `log_enabled!` macro in their code to avoid
constructing expensive log messages.

# Examples

```rust
use log::{Record, Level, Metadata};

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

fn main(){}
```

#### Implementations

- `fn builder() -> MetadataBuilder<'a>` — [`MetadataBuilder`](#metadatabuilder)

- `fn level(self: &Self) -> Level` — [`Level`](#level)

- `fn target(self: &Self) -> &'a str`

#### Trait Implementations

##### `impl<'a> AsTrace for log::Metadata<'a>`

- `type Err = ParseLevelError`

##### `impl<'a> Clone for Metadata<'a>`

- `fn clone(self: &Self) -> Metadata<'a>` — [`Metadata`](#metadata)

##### `impl<'a> Debug for Metadata<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> Eq for Metadata<'a>`

##### `impl<'a> Hash for Metadata<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'a> Ord for Metadata<'a>`

- `fn cmp(self: &Self, other: &Metadata<'a>) -> $crate::cmp::Ordering` — [`Metadata`](#metadata)

##### `impl<'a> PartialEq for Metadata<'a>`

- `fn eq(self: &Self, other: &Metadata<'a>) -> bool` — [`Metadata`](#metadata)

##### `impl<'a> PartialOrd for Metadata<'a>`

- `fn partial_cmp(self: &Self, other: &Metadata<'a>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Metadata`](#metadata)

##### `impl<'a> Sealed for log::Metadata<'a>`

##### `impl<'a> StructuralPartialEq for Metadata<'a>`

### `MetadataBuilder<'a>`

```rust
struct MetadataBuilder<'a> {
    metadata: Metadata<'a>,
}
```

Builder for [`Metadata`](#metadata).

Typically should only be used by log library creators or for testing and "shim loggers".
The `MetadataBuilder` can set the different parameters of a `Metadata` object, and returns
the created object when `build` is called.

# Example

```rust
let target = "myApp";
use log::{Level, MetadataBuilder};
let metadata = MetadataBuilder::new()
                    .level(Level::Debug)
                    .target(target)
                    .build();
```

#### Implementations

- `fn new() -> MetadataBuilder<'a>` — [`MetadataBuilder`](#metadatabuilder)

- `fn level(self: &mut Self, arg: Level) -> &mut MetadataBuilder<'a>` — [`Level`](#level), [`MetadataBuilder`](#metadatabuilder)

- `fn target(self: &mut Self, target: &'a str) -> &mut MetadataBuilder<'a>` — [`MetadataBuilder`](#metadatabuilder)

- `fn build(self: &Self) -> Metadata<'a>` — [`Metadata`](#metadata)

#### Trait Implementations

##### `impl<'a> Debug for MetadataBuilder<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MetadataBuilder<'_>`

- `fn default() -> Self`

##### `impl<'a> Eq for MetadataBuilder<'a>`

##### `impl<'a> Hash for MetadataBuilder<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'a> Ord for MetadataBuilder<'a>`

- `fn cmp(self: &Self, other: &MetadataBuilder<'a>) -> $crate::cmp::Ordering` — [`MetadataBuilder`](#metadatabuilder)

##### `impl<'a> PartialEq for MetadataBuilder<'a>`

- `fn eq(self: &Self, other: &MetadataBuilder<'a>) -> bool` — [`MetadataBuilder`](#metadatabuilder)

##### `impl<'a> PartialOrd for MetadataBuilder<'a>`

- `fn partial_cmp(self: &Self, other: &MetadataBuilder<'a>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`MetadataBuilder`](#metadatabuilder)

##### `impl<'a> StructuralPartialEq for MetadataBuilder<'a>`

### `SetLoggerError`

```rust
struct SetLoggerError(());
```

The type returned by [`set_logger`](#set-logger) if [`set_logger`](#set-logger) has already been called.


#### Trait Implementations

##### `impl Debug for SetLoggerError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SetLoggerError`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for SetLoggerError`

##### `impl<T> ToString for SetLoggerError`

- `fn to_string(self: &Self) -> String`

### `ParseLevelError`

```rust
struct ParseLevelError(());
```

The type returned by `from_str` when the string doesn't match any of the log levels.


#### Trait Implementations

##### `impl Debug for ParseLevelError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ParseLevelError`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseLevelError`

##### `impl Error for ParseLevelError`

##### `impl PartialEq for ParseLevelError`

- `fn eq(self: &Self, other: &ParseLevelError) -> bool` — [`ParseLevelError`](#parselevelerror)

##### `impl StructuralPartialEq for ParseLevelError`

##### `impl<T> ToString for ParseLevelError`

- `fn to_string(self: &Self) -> String`

## Enums

### `Level`

```rust
enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
```

An enum representing the available verbosity levels of the logger.

Typical usage includes: checking if a certain `Level` is enabled with
[`log_enabled!`](#log_enabled), specifying the `Level` of
[`log!`](#log), and comparing a `Level` directly to a
[`LevelFilter`](#levelfilter).

#### Variants

- **`Error`**

  The "error" level.
  
  Designates very serious errors.

- **`Warn`**

  The "warn" level.
  
  Designates hazardous situations.

- **`Info`**

  The "info" level.
  
  Designates useful information.

- **`Debug`**

  The "debug" level.
  
  Designates lower priority information.

- **`Trace`**

  The "trace" level.
  
  Designates very low priority, often extremely verbose, information.

#### Implementations

- `fn from_usize(u: usize) -> Option<Level>` — [`Level`](#level)

- `fn max() -> Level` — [`Level`](#level)

- `fn to_level_filter(self: &Self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

- `fn as_str(self: &Self) -> &'static str`

- `fn iter() -> impl Iterator<Item = Self>`

- `fn increment_severity(self: &Self) -> Self`

- `fn decrement_severity(self: &Self) -> Self`

#### Trait Implementations

##### `impl AsTrace for log::Level`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Clone for Level`

- `fn clone(self: &Self) -> Level` — [`Level`](#level)

##### `impl Copy for Level`

##### `impl Debug for Level`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Level`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Level`

##### `impl FromStr for Level`

- `type Err = ParseLevelError`

- `fn from_str(level: &str) -> Result<Level, <Self as >::Err>` — [`Level`](#level)

##### `impl Hash for Level`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Level`

- `fn cmp(self: &Self, other: &Level) -> $crate::cmp::Ordering` — [`Level`](#level)

##### `impl PartialEq for Level`

- `fn eq(self: &Self, other: &Level) -> bool` — [`Level`](#level)

##### `impl PartialOrd for Level`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering>` — [`LevelFilter`](#levelfilter)

##### `impl Sealed for log::Level`

##### `impl StructuralPartialEq for Level`

##### `impl<T> ToString for Level`

- `fn to_string(self: &Self) -> String`

### `LevelFilter`

```rust
enum LevelFilter {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
```

An enum representing the available verbosity level filters of the logger.

A `LevelFilter` may be compared directly to a [`Level`](#level). Use this type
to get and set the maximum log level with `max_level()` and [`set_max_level`](#set-max-level).




#### Variants

- **`Off`**

  A level lower than all log levels.

- **`Error`**

  Corresponds to the `Error` log level.

- **`Warn`**

  Corresponds to the `Warn` log level.

- **`Info`**

  Corresponds to the `Info` log level.

- **`Debug`**

  Corresponds to the `Debug` log level.

- **`Trace`**

  Corresponds to the `Trace` log level.

#### Implementations

- `fn from_usize(u: usize) -> Option<LevelFilter>` — [`LevelFilter`](#levelfilter)

- `fn max() -> LevelFilter` — [`LevelFilter`](#levelfilter)

- `fn to_level(self: &Self) -> Option<Level>` — [`Level`](#level)

- `fn as_str(self: &Self) -> &'static str`

- `fn iter() -> impl Iterator<Item = Self>`

- `fn increment_severity(self: &Self) -> Self`

- `fn decrement_severity(self: &Self) -> Self`

#### Trait Implementations

##### `impl AsTrace for log::LevelFilter`

- `fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering>` — [`Level`](#level)

##### `impl Clone for LevelFilter`

- `fn clone(self: &Self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

##### `impl Copy for LevelFilter`

##### `impl Debug for LevelFilter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for LevelFilter`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LevelFilter`

##### `impl FromStr for LevelFilter`

- `type Err = ParseLevelError`

- `fn from_str(level: &str) -> Result<LevelFilter, <Self as >::Err>` — [`LevelFilter`](#levelfilter)

##### `impl Hash for LevelFilter`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for LevelFilter`

- `fn cmp(self: &Self, other: &LevelFilter) -> $crate::cmp::Ordering` — [`LevelFilter`](#levelfilter)

##### `impl PartialEq for LevelFilter`

- `fn eq(self: &Self, other: &Level) -> bool` — [`Level`](#level)

##### `impl PartialOrd for LevelFilter`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> $crate::option::Option<$crate::cmp::Ordering>` — [`LevelFilter`](#levelfilter)

##### `impl Sealed for log::LevelFilter`

##### `impl StructuralPartialEq for LevelFilter`

##### `impl<T> ToString for LevelFilter`

- `fn to_string(self: &Self) -> String`

## Traits

### `Log`

```rust
trait Log: Sync + Send { ... }
```

A trait encapsulating the operations required of a logger.

#### Required Methods

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool`

  Determines if a log message with the specified metadata would be

- `fn log(self: &Self, record: &Record<'_>)`

  Logs the `Record`.

- `fn flush(self: &Self)`

  Flushes any buffered records.

## Functions

### `set_max_level`

```rust
fn set_max_level(level: LevelFilter)
```

Sets the global maximum log level.

Generally, this should only be called by the active logging implementation.

Note that `Trace` is the maximum level, because it provides the maximum amount of detail in the emitted logs.

### `set_max_level_racy`

```rust
unsafe fn set_max_level_racy(level: LevelFilter)
```

A thread-unsafe version of [`set_max_level`](#set-max-level).

This function is available on all platforms, even those that do not have
support for atomics that is needed by [`set_max_level`](#set-max-level).

In almost all cases, [`set_max_level`](#set-max-level) should be preferred.

# Safety

This function is only safe to call when it cannot race with any other
calls to `set_max_level` or `set_max_level_racy`.

This can be upheld by (for example) making sure that **there are no other
threads**, and (on embedded) that **interrupts are disabled**.

It is safe to use all other logging functions while this function runs
(including all logging macros).


### `max_level`

```rust
fn max_level() -> LevelFilter
```

Returns the current maximum log level.

The `log!`, `error!`, `warn!`, `info!`, `debug!`, and `trace!` macros check
this value and discard any message logged at a higher level. The maximum
log level is set by the [`set_max_level`](#set-max-level) function.








### `set_boxed_logger`

```rust
fn set_boxed_logger(logger: Box<dyn Log>) -> Result<(), SetLoggerError>
```

Sets the global logger to a `Box<Log>`.

This is a simple convenience wrapper over `set_logger`, which takes a
`Box<Log>` rather than a `&'static Log`. See the documentation for
[`set_logger`](#set-logger) for more details.

Requires the `std` feature.

# Errors

An error is returned if a logger has already been set.


### `set_logger`

```rust
fn set_logger(logger: &'static dyn Log) -> Result<(), SetLoggerError>
```

Sets the global logger to a `&'static Log`.

This function may only be called once in the lifetime of a program. Any log
events that occur before the call to `set_logger` completes will be ignored.

This function does not typically need to be called manually. Logger
implementations should provide an initialization method that installs the
logger internally.

# Availability

This method is available even when the `std` feature is disabled. However,
it is currently unavailable on `thumbv6` targets, which lack support for
some atomic operations which are used by this function. Even on those
targets, [`set_logger_racy`](#set-logger-racy) will be available.

# Errors

An error is returned if a logger has already been set.

# Examples

```rust
use log::{error, info, warn, Record, Level, Metadata, LevelFilter};

static MY_LOGGER: MyLogger = MyLogger;

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

fn main(){
log::set_logger(&MY_LOGGER).unwrap();
log::set_max_level(LevelFilter::Info);

info!("hello log");
warn!("warning");
error!("oops");
}
```


### `set_logger_racy`

```rust
unsafe fn set_logger_racy(logger: &'static dyn Log) -> Result<(), SetLoggerError>
```

A thread-unsafe version of [`set_logger`](#set-logger).

This function is available on all platforms, even those that do not have
support for atomics that is needed by [`set_logger`](#set-logger).

In almost all cases, [`set_logger`](#set-logger) should be preferred.

# Safety

This function is only safe to call when it cannot race with any other
calls to `set_logger` or `set_logger_racy`.

This can be upheld by (for example) making sure that **there are no other
threads**, and (on embedded) that **interrupts are disabled**.

It is safe to use other logging functions while this function runs
(including all logging macros).


### `logger`

```rust
fn logger() -> &'static dyn Log
```

Returns a reference to the logger.

If a logger has not been set, a no-op implementation is returned.

## Constants

### `STATIC_MAX_LEVEL`

```rust
const STATIC_MAX_LEVEL: LevelFilter;
```

The statically resolved maximum log level.

See the crate level documentation for information on how to configure this.

This value is checked by the log macros, but not by the `Log`ger returned by
the [`logger`](#logger) function. Code that manually calls functions on that value
should compare the level against this value.


## Macros

### `error!`

Logs a message at the error level.

# Examples

```rust
use log::error;

let my_logger = log::__private_api::GlobalLogger;
let (err_info, port) = ("No connection", 22);

error!("Error: {err_info} on port {port}");
error!(target: "app_events", "App Error: {err_info}, Port: {port}");
error!(logger: my_logger, "App Error: {err_info}, Port: {port}");
```

### `log_enabled!`

Determines if a message logged at the specified level in that module will
be logged.

This can be used to avoid expensive computation of log message arguments if
the message would be ignored anyway.

# Examples

```rust
use log::{debug, log_enabled, Level};

struct Data { x: u32, y: u32 }
fn expensive_call() -> Data { Data { x: 0, y: 0 } }
let my_logger = log::__private_api::GlobalLogger;
if log_enabled!(Level::Debug) {
    let data = expensive_call();
    debug!("expensive debug data: {} {}", data.x, data.y);
}

if log_enabled!(target: "Global", Level::Debug) {
   let data = expensive_call();
   debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
}

if log_enabled!(logger: my_logger, Level::Debug) {
   let data = expensive_call();
   debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
}
```

This macro accepts the same `target` and `logger` arguments as [`macro@log`](#macrolog).

### `log!`

The standard logging macro.

This macro will generically log with the specified `Level` and `format!`
based argument list.

```rust
use log::{log, Level};

let data = (42, "Forty-two");
let private_data = "private";

log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
```

Optionally, you can specify a `target` argument to attach a specific target
to the log record. By default, the target is the module path of the caller.

```rust
use log::{log, Level};

let data = (42, "Forty-two");
let private_data = "private";

log!(
    target: "app_events",
    Level::Error,
    "Received errors: {}, {}",
    data.0, data.1
);
```

And optionally, you can specify a `logger` argument to use a specific logger
instead of the default global logger.

```rust
struct MyLogger {}
impl Log for MyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        false
    }
    fn log(&self, _record: &log::Record) {}
    fn flush(&self) {}
}
use log::{log, Level, Log};

let data = (42, "Forty-two");
let private_data = "private";

let my_logger = MyLogger {};
log!(
    logger: my_logger,
    Level::Error,
    "Received errors: {}, {}",
    data.0, data.1
);
```

The `logger` argument accepts a value that implements the `Log` trait. The value
will be borrowed within the macro.

Note that the global level set via Cargo features, or through `set_max_level` will
still apply, even when a custom logger is supplied with the `logger` argument.

### `warn!`

Logs a message at the warn level.

# Examples

```rust
use log::warn;

let my_logger = log::__private_api::GlobalLogger;
let warn_description = "Invalid Input";

warn!("Warning! {warn_description}!");
warn!(target: "input_events", "App received warning: {warn_description}");
warn!(logger: my_logger, "App received warning: {warn_description}");
```

### `info!`

Logs a message at the info level.

# Examples

```rust
use log::info;

let my_logger = log::__private_api::GlobalLogger;
struct Connection { port: u32, speed: f32 }
let conn_info = Connection { port: 40, speed: 3.20 };

info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
info!(
    target: "connection_events",
    "Successful connection, port: {}, speed: {}",
    conn_info.port, conn_info.speed
);
info!(
    logger: my_logger,
    "Successful connection, port: {}, speed: {}",
    conn_info.port, conn_info.speed
);
```

### `debug!`

Logs a message at the debug level.

# Examples

```rust
use log::debug;

let my_logger = log::__private_api::GlobalLogger;
struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

debug!("New position: x: {}, y: {}", pos.x, pos.y);
debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
debug!(logger: my_logger, "New position: x: {}, y: {}", pos.x, pos.y);
```

### `trace!`

Logs a message at the trace level.

# Examples

```rust
use log::trace;

let my_logger = log::__private_api::GlobalLogger;
struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

trace!("Position is: x: {}, y: {}", pos.x, pos.y);
trace!(target: "app_events", "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
trace!(logger: my_logger, "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
```

