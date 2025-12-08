*[docs_md](../index.md) / [logger](index.md)*

---

# Module `logger`

Logging related utils.

## Structs

### `Logger`

```rust
struct Logger;
```

Logger

#### Implementations

- `fn init_logging(log_level: LogLevel, log_file: Option<&PathBuf>) -> Result<()>` — [`LogLevel`](#loglevel)

#### Trait Implementations

##### `impl<T> Instrument for Logger`

##### `impl<T> IntoEither for Logger`

##### `impl<D> OwoColorize for Logger`

##### `impl<T> Pointable for Logger`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for Logger`

## Enums

### `LogLevel`

```rust
enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
```

Log level for debug output

#### Variants

- **`Off`**

  No logging output

- **`Error`**

  Errors only

- **`Warn`**

  Errors and warnings

- **`Info`**

  Informational messages

- **`Debug`**

  Debug messages (verbose)

- **`Trace`**

  Trace messages (very verbose)

#### Implementations

- `const fn to_tracing_level(self: Self) -> LevelFilter`

#### Trait Implementations

##### `impl Clone for LogLevel`

- `fn clone(self: &Self) -> LogLevel` — [`LogLevel`](#loglevel)

##### `impl Copy for LogLevel`

##### `impl Debug for LogLevel`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LogLevel`

- `fn default() -> LogLevel` — [`LogLevel`](#loglevel)

##### `impl<T> Instrument for LogLevel`

##### `impl<T> IntoEither for LogLevel`

##### `impl<D> OwoColorize for LogLevel`

##### `impl<T> Pointable for LogLevel`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl ValueEnum for LogLevel`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value<'a>(self: &Self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl<T> WithSubscriber for LogLevel`

