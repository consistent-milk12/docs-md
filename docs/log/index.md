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

The basic use of the log crate is through the five logging macros: [`error!`](#error),
[`warn!`](#warn), [`info!`](#info), [`debug!`](#debug) and [`trace!`](#trace)
where `error!` represents the highest-priority log messages
and `trace!` the lowest. The log messages are filtered by configuring
the log level to exclude messages with a lower priority.
Each of these macros accept format strings similarly to [`println!`](#println).






Avoid writing expressions with side-effects in log statements. They may not be evaluated.

## In libraries

Libraries should link only to the `log` crate, and use the provided
macros to log whatever information will be useful to downstream consumers.

### Examples

```
# #[derive(Debug)] pub struct Yak(String);
# impl Yak { fn shave(&mut self, _: u32) {} }
# fn find_a_razor() -> Result<u32, u32> { Ok(1) }
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
# fn main() {}
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

```
# use serde::Serialize;
# #[derive(Debug, Serialize)] pub struct Yak(String);
# impl Yak { fn shave(&mut self, _: u32) {} }
# fn find_a_razor() -> Result<u32, std::io::Error> { Ok(1) }
# #[cfg(feature = "kv_serde")]
# fn main() {
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
# }
# #[cfg(not(feature = "kv_serde"))]
# fn main() {}
```

See the [`kv`](#kv) module documentation for more details.

# Available logging implementations

In order to produce log output executables have to use
a logger implementation compatible with the facade.
There are many available implementations to choose from,
here are some of the most popular ones:

* Simple minimal loggers:
    * [env_logger](#env-logger)

    * [colog](#colog)

    * [simple_logger](#simple-logger)

    * [simplelog](#simplelog)

    * [pretty_env_logger](#pretty-env-logger)

    * [stderrlog](#stderrlog)

    * [flexi_logger](#flexi-logger)

    * [call_logger](#call-logger)

    * [std-logger]
    * [structured-logger]
    * [clang_log](#clang-log)

    * [ftail](#ftail)

* Complex configurable frameworks:
    * [log4rs](#log4rs)

    * [logforth](#logforth)

    * [fern](#fern)

    * [spdlog-rs]
* Adaptors for other facilities:
    * [syslog](#syslog)

    * [slog-stdlog]
    * [systemd-journal-logger]
    * [android_log](#android-log)

    * [win_dbg_logger](#win-dbg-logger)

    * [db_logger](#db-logger)

    * [log-to-defmt]
    * [logcontrol-log]
* For WebAssembly binaries:
    * [console_log](#console-log)

* For dynamic libraries:
    * You may need to construct an FFI-safe wrapper over `log` to initialize in your libraries
* Utilities:
    * [log_err](#log-err)

    * [log-reload]
    * [alterable_logger](#alterable-logger)


# Implementing a Logger

Loggers implement the [`Log`](#log) trait. Here's a very basic example that simply
logs all messages at the [`Error`][level_link](#level-link)
, [`Warn`][level_link](#level-link)
 or
[`Info`][level_link](#level-link)
 levels to stdout:

```
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

# fn main() {}
```

Loggers are installed by calling the [`set_logger`](#set-logger) function. The maximum
log level also needs to be adjusted via the [`set_max_level`](#set-max-level) function. The
logging facade uses this as an optimization to improve performance of log
messages at levels that are disabled. It's important to set it, as it
defaults to [`Off`][filter_link](#filter-link)
, so no log messages will ever be captured!
In the case of our example logger, we'll want to set the maximum log level
to [`Info`][filter_link](#filter-link)
, since we ignore any [`Debug`][level_link](#level-link)
 or
[`Trace`][level_link](#level-link)
 level log messages. A logging implementation should
provide a function that wraps a call to [`set_logger`](#set-logger) and
[`set_max_level`](#set-max-level), handling initialization of the logger:

```
# use log::{Level, Metadata};
# struct SimpleLogger;
# impl log::Log for SimpleLogger {
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn log(&self, _: &log::Record) {}
#   fn flush(&self) {}
# }
# fn main() {}
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

```
# use log::{Level, LevelFilter, Log, SetLoggerError, Metadata};
# struct SimpleLogger;
# impl log::Log for SimpleLogger {
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn log(&self, _: &log::Record) {}
#   fn flush(&self) {}
# }
# fn main() {}
# #[cfg(feature = "std")]
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
[dependencies](#dependencies)

log = { version = "0.4", features = ["max_level_debug", "release_max_level_warn"] }
```
# Crate Feature Flags

The following crate feature flags are available in addition to the filters. They are
configured in your `Cargo.toml`.

* `std` allows use of `std` crate instead of the default `core`. Enables using `std::error` and
  `set_boxed_logger` functionality.
* `serde` enables support for serialization and deserialization of `Level` and `LevelFilter`.

```toml
[dependencies](#dependencies)

log = { version = "0.4", features = ["std", "serde"] }
```

# Version compatibility

The 0.3 and 0.4 versions of the `log` crate are almost entirely compatible. Log messages
made using `log` 0.3 will forward transparently to a logger implementation using `log` 0.4. Log
messages made using `log` 0.4 will forward to a logger implementation using `log` 0.3, but the
module path and file name information associated with the message will unfortunately be lost.

[level_link](#level-link)
: enum.Level.html
[filter_link](#filter-link)
: enum.LevelFilter.html




[env_logger](#env-logger)
: https://docs.rs/env_logger/*/env_logger/
[colog](#colog)
: https://docs.rs/colog/*/colog/
[simple_logger](#simple-logger)
: https://github.com/borntyping/rust-simple_logger
[simplelog](#simplelog)
: https://github.com/drakulix/simplelog.rs
[pretty_env_logger](#pretty-env-logger)
: https://docs.rs/pretty_env_logger/*/pretty_env_logger/
[stderrlog](#stderrlog)
: https://docs.rs/stderrlog/*/stderrlog/
[flexi_logger](#flexi-logger)
: https://docs.rs/flexi_logger/*/flexi_logger/
[call_logger](#call-logger)
: https://docs.rs/call_logger/*/call_logger/
[std-logger]: https://docs.rs/std-logger/*/std_logger/
[syslog](#syslog)
: https://docs.rs/syslog/*/syslog/
[slog-stdlog]: https://docs.rs/slog-stdlog/*/slog_stdlog/
[log4rs](#log4rs)
: https://docs.rs/log4rs/*/log4rs/
[logforth](#logforth)
: https://docs.rs/logforth/*/logforth/
[fern](#fern)
: https://docs.rs/fern/*/fern/
[spdlog-rs]: https://docs.rs/spdlog-rs/*/spdlog/
[systemd-journal-logger]: https://docs.rs/systemd-journal-logger/*/systemd_journal_logger/
[android_log](#android-log)
: https://docs.rs/android_log/*/android_log/
[win_dbg_logger](#win-dbg-logger)
: https://docs.rs/win_dbg_logger/*/win_dbg_logger/
[db_logger](#db-logger)
: https://docs.rs/db_logger/*/db_logger/
[log-to-defmt]: https://docs.rs/log-to-defmt/*/log_to_defmt/
[console_log](#console-log)
: https://docs.rs/console_log/*/console_log/
[structured-logger]: https://docs.rs/structured-logger/latest/structured_logger/
[logcontrol-log]: https://docs.rs/logcontrol-log/*/logcontrol_log/
[log_err](#log-err)
: https://docs.rs/log_err/*/log_err/
[log-reload]: https://docs.rs/log-reload/*/log_reload/
[alterable_logger](#alterable-logger)
: https://docs.rs/alterable_logger/*/alterable_logger
[clang_log](#clang-log)
: https://docs.rs/clang_log/latest/clang_log
[ftail](#ftail)
: https://docs.rs/ftail/latest/ftail

## Structs

### `Record<'a>`

```rust
struct Record<'a> {
    // [REDACTED: Private Fields]
}
```

The "payload" of a log message.

# Use

`Record` structures are passed as parameters to the [`log`][method.log]
method of the [`Log`](#log) trait. Logger implementors manipulate these
structures in order to display log messages. `Record`s are automatically
created by the [`log!`](#log) macro and so are not seen by log users.

Note that the [`level()`](#level) and [`target()`](#target) accessors are equivalent to
`self.metadata().level()` and `self.metadata().target()` respectively.
These methods are provided as a convenience for users of this structure.

# Example

The following example shows a simple logger that displays the level,
module path, and message of any `Record` that is passed to it.

```
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

[method.log]: trait.Log.html#tymethod.log





#### Implementations

- `fn builder() -> RecordBuilder<'a>`
  Returns a new builder.

- `fn args(self: &Self) -> &fmt::Arguments<'a>`
  The message body.

- `fn metadata(self: &Self) -> &Metadata<'a>`
  Metadata about the log directive.

- `fn level(self: &Self) -> Level`
  The verbosity level of the message.

- `fn target(self: &Self) -> &'a str`
  The name of the target of the directive.

- `fn module_path(self: &Self) -> Option<&'a str>`
  The module path of the message.

- `fn module_path_static(self: &Self) -> Option<&'static str>`
  The module path of the message, if it is a `'static` string.

- `fn file(self: &Self) -> Option<&'a str>`
  The source file containing the message.

- `fn file_static(self: &Self) -> Option<&'static str>`
  The source file containing the message, if it is a `'static` string.

- `fn line(self: &Self) -> Option<u32>`
  The line containing the message.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Record<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RecordBuilder<'a>`

```rust
struct RecordBuilder<'a> {
    // [REDACTED: Private Fields]
}
```

Builder for [`Record`](#record).

Typically should only be used by log library creators or for testing and "shim loggers".
The `RecordBuilder` can set the different parameters of `Record` object, and returns
the created object when `build` is called.

# Examples

```
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

```
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

- `fn new() -> RecordBuilder<'a>`
  Construct new `RecordBuilder`.

- `fn args(self: &mut Self, args: fmt::Arguments<'a>) -> &mut RecordBuilder<'a>`
  Set [`args`](struct.Record.html#method.args).

- `fn metadata(self: &mut Self, metadata: Metadata<'a>) -> &mut RecordBuilder<'a>`
  Set [`metadata`](struct.Record.html#method.metadata). Construct a `Metadata` object with [`MetadataBuilder`](struct.MetadataBuilder.html).

- `fn level(self: &mut Self, level: Level) -> &mut RecordBuilder<'a>`
  Set [`Metadata::level`](struct.Metadata.html#method.level).

- `fn target(self: &mut Self, target: &'a str) -> &mut RecordBuilder<'a>`
  Set [`Metadata::target`](struct.Metadata.html#method.target)

- `fn module_path(self: &mut Self, path: Option<&'a str>) -> &mut RecordBuilder<'a>`
  Set [`module_path`](struct.Record.html#method.module_path)

- `fn module_path_static(self: &mut Self, path: Option<&'static str>) -> &mut RecordBuilder<'a>`
  Set [`module_path`](struct.Record.html#method.module_path) to a `'static` string

- `fn file(self: &mut Self, file: Option<&'a str>) -> &mut RecordBuilder<'a>`
  Set [`file`](struct.Record.html#method.file)

- `fn file_static(self: &mut Self, file: Option<&'static str>) -> &mut RecordBuilder<'a>`
  Set [`file`](struct.Record.html#method.file) to a `'static` string.

- `fn line(self: &mut Self, line: Option<u32>) -> &mut RecordBuilder<'a>`
  Set [`line`](struct.Record.html#method.line)

- `fn build(self: &Self) -> Record<'a>`
  Invoke the builder and return a `Record`

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `Metadata<'a>`

```rust
struct Metadata<'a> {
    // [REDACTED: Private Fields]
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

```
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

# fn main(){}
```

#### Implementations

- `fn builder() -> MetadataBuilder<'a>`
  Returns a new builder.

- `fn level(self: &Self) -> Level`
  The verbosity level of the message.

- `fn target(self: &Self) -> &'a str`
  The name of the target of the directive.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Metadata<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<'a>`

- `fn cmp(self: &Self, other: &Metadata<'a>) -> $crate::cmp::Ordering`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &Metadata<'a>) -> bool`

##### `impl PartialOrd<'a>`

- `fn partial_cmp(self: &Self, other: &Metadata<'a>) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MetadataBuilder<'a>`

```rust
struct MetadataBuilder<'a> {
    // [REDACTED: Private Fields]
}
```

Builder for [`Metadata`](#metadata).

Typically should only be used by log library creators or for testing and "shim loggers".
The `MetadataBuilder` can set the different parameters of a `Metadata` object, and returns
the created object when `build` is called.

# Example

```
let target = "myApp";
use log::{Level, MetadataBuilder};
let metadata = MetadataBuilder::new()
                    .level(Level::Debug)
                    .target(target)
                    .build();
```

#### Implementations

- `fn new() -> MetadataBuilder<'a>`
  Construct a new `MetadataBuilder`.

- `fn level(self: &mut Self, arg: Level) -> &mut MetadataBuilder<'a>`
  Setter for [`level`](struct.Metadata.html#method.level).

- `fn target(self: &mut Self, target: &'a str) -> &mut MetadataBuilder<'a>`
  Setter for [`target`](struct.Metadata.html#method.target).

- `fn build(self: &Self) -> Metadata<'a>`
  Returns a `Metadata` object.

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

##### `impl Eq<'a>`

##### `impl Hash<'a>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<'a>`

- `fn cmp(self: &Self, other: &MetadataBuilder<'a>) -> $crate::cmp::Ordering`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &MetadataBuilder<'a>) -> bool`

##### `impl PartialOrd<'a>`

- `fn partial_cmp(self: &Self, other: &MetadataBuilder<'a>) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `SetLoggerError`

```rust
struct SetLoggerError();
```

The type returned by [`set_logger`](#set-logger) if [`set_logger`](#set-logger) has already been called.


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

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ParseLevelError`

```rust
struct ParseLevelError();
```

The type returned by [`from_str`](#from-str) when the string doesn't match any of the log levels.


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

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ParseLevelError) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn max() -> Level`
  Returns the most verbose logging level.

- `fn to_level_filter(self: &Self) -> LevelFilter`
  Converts the `Level` to the equivalent `LevelFilter`.

- `fn as_str(self: &Self) -> &'static str`
  Returns the string representation of the `Level`.

- `fn iter() -> impl Iterator<Item = Self>`
  Iterate through all supported logging levels.

- `fn increment_severity(self: &Self) -> Self`
  Get the next-highest `Level` from this one.

- `fn decrement_severity(self: &Self) -> Self`
  Get the next-lowest `Level` from this one.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = ParseLevelError`

- `fn from_str(level: &str) -> Result<Level, <Self as >::Err>`

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

- `fn clone(self: &Self) -> Level`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Level) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LevelFilter) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Level) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Level) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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
to get and set the maximum log level with [`max_level()`](#max-level) and [`set_max_level`](#set-max-level).




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

- `fn max() -> LevelFilter`
  Returns the most verbose logging level filter.

- `fn to_level(self: &Self) -> Option<Level>`
  Converts `self` to the equivalent `Level`.

- `fn as_str(self: &Self) -> &'static str`
  Returns the string representation of the `LevelFilter`.

- `fn iter() -> impl Iterator<Item = Self>`
  Iterate through all supported filtering levels.

- `fn increment_severity(self: &Self) -> Self`
  Get the next-highest `LevelFilter` from this one.

- `fn decrement_severity(self: &Self) -> Self`
  Get the next-lowest `LevelFilter` from this one.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = ParseLevelError`

- `fn from_str(level: &str) -> Result<LevelFilter, <Self as >::Err>`

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

- `fn clone(self: &Self) -> LevelFilter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &LevelFilter) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LevelFilter) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Level) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

The [`log!`](#log), [`error!`](#error), [`warn!`](#warn), [`info!`](#info), [`debug!`](#debug), and [`trace!`](#trace) macros check
this value and discard any message logged at a higher level. The maximum
log level is set by the [`set_max_level`](#set-max-level) function.








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

```
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

# fn main(){
log::set_logger(&MY_LOGGER).unwrap();
log::set_max_level(LevelFilter::Info);

info!("hello log");
warn!("warning");
error!("oops");
# }
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

### `log_enabled!`

Determines if a message logged at the specified level in that module will
be logged.

This can be used to avoid expensive computation of log message arguments if
the message would be ignored anyway.

# Examples

```
use log::{debug, log_enabled, Level};

# struct Data { x: u32, y: u32 }
# fn expensive_call() -> Data { Data { x: 0, y: 0 } }
# let my_logger = log::__private_api::GlobalLogger;
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

```
use log::{log, Level};

let data = (42, "Forty-two");
let private_data = "private";

log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
```

Optionally, you can specify a `target` argument to attach a specific target
to the log record. By default, the target is the module path of the caller.

```
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

```
# struct MyLogger {}
# impl Log for MyLogger {
#     fn enabled(&self, _metadata: &log::Metadata) -> bool {
#         false
#     }
#     fn log(&self, _record: &log::Record) {}
#     fn flush(&self) {}
# }
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

### `error!`

Logs a message at the error level.

# Examples

```
use log::error;

# let my_logger = log::__private_api::GlobalLogger;
let (err_info, port) = ("No connection", 22);

error!("Error: {err_info} on port {port}");
error!(target: "app_events", "App Error: {err_info}, Port: {port}");
error!(logger: my_logger, "App Error: {err_info}, Port: {port}");
```

### `warn!`

Logs a message at the warn level.

# Examples

```
use log::warn;

# let my_logger = log::__private_api::GlobalLogger;
let warn_description = "Invalid Input";

warn!("Warning! {warn_description}!");
warn!(target: "input_events", "App received warning: {warn_description}");
warn!(logger: my_logger, "App received warning: {warn_description}");
```

### `info!`

Logs a message at the info level.

# Examples

```
use log::info;

# let my_logger = log::__private_api::GlobalLogger;
# struct Connection { port: u32, speed: f32 }
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

```
use log::debug;

# let my_logger = log::__private_api::GlobalLogger;
# struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

debug!("New position: x: {}, y: {}", pos.x, pos.y);
debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
debug!(logger: my_logger, "New position: x: {}, y: {}", pos.x, pos.y);
```

### `trace!`

Logs a message at the trace level.

# Examples

```
use log::trace;

# let my_logger = log::__private_api::GlobalLogger;
# struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

trace!("Position is: x: {}, y: {}", pos.x, pos.y);
trace!(target: "app_events", "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
trace!(logger: my_logger, "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
```

