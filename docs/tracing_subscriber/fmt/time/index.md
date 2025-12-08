*[tracing_subscriber](../../index.md) / [fmt](../index.md) / [time](index.md)*

---

# Module `time`

Formatters for event timestamps.

## Structs

### `SystemTime`

```rust
struct SystemTime;
```

Retrieve and print the current wall-clock time.

#### Trait Implementations

##### `impl Clone for SystemTime`

- `fn clone(self: &Self) -> SystemTime` — [`SystemTime`](#systemtime)

##### `impl Copy for SystemTime`

##### `impl Debug for SystemTime`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for SystemTime`

- `fn default() -> SystemTime` — [`SystemTime`](#systemtime)

##### `impl Eq for SystemTime`

##### `impl FormatTime for SystemTime`

- `fn format_time(self: &Self, w: &mut Writer<'_>) -> fmt::Result` — [`Writer`](../format/index.md)

##### `impl<T> Instrument for SystemTime`

##### `impl PartialEq for SystemTime`

- `fn eq(self: &Self, other: &SystemTime) -> bool` — [`SystemTime`](#systemtime)

##### `impl StructuralPartialEq for SystemTime`

##### `impl<T> WithSubscriber for SystemTime`

### `Uptime`

```rust
struct Uptime {
    epoch: std::time::Instant,
}
```

Retrieve and print the relative elapsed wall-clock time since an epoch.

The `Default` implementation for `Uptime` makes the epoch the current time.

#### Trait Implementations

##### `impl Clone for Uptime`

- `fn clone(self: &Self) -> Uptime` — [`Uptime`](#uptime)

##### `impl Copy for Uptime`

##### `impl Debug for Uptime`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Uptime`

- `fn default() -> Self`

##### `impl Eq for Uptime`

##### `impl FormatTime for Uptime`

- `fn format_time(self: &Self, w: &mut Writer<'_>) -> fmt::Result` — [`Writer`](../format/index.md)

##### `impl<T> Instrument for Uptime`

##### `impl PartialEq for Uptime`

- `fn eq(self: &Self, other: &Uptime) -> bool` — [`Uptime`](#uptime)

##### `impl StructuralPartialEq for Uptime`

##### `impl<T> WithSubscriber for Uptime`

## Traits

### `FormatTime`

```rust
trait FormatTime { ... }
```

A type that can measure and format the current time.

This trait is used by `Format` to include a timestamp with each `Event` when it is logged.

Notable default implementations of this trait are `SystemTime` and `()`. The former prints the
current time as reported by `std::time::SystemTime`, and the latter does not print the current
time at all. `FormatTime` is also automatically implemented for any function pointer with the
appropriate signature.

The full list of provided implementations can be found in [`time`](#time).


#### Required Methods

- `fn format_time(self: &Self, w: &mut Writer<'_>) -> fmt::Result`

  Measure and write out the current time.

## Functions

### `time`

```rust
fn time() -> SystemTime
```

Returns a new `SystemTime` timestamp provider.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
fn timer() -> tracing_subscriber::fmt::time::SystemTime {
tracing_subscriber::fmt::time::SystemTime::default()
}
```

### `uptime`

```rust
fn uptime() -> Uptime
```

Returns a new `Uptime` timestamp provider.

With this timer, timestamps will be formatted with the amount of time
elapsed since the timestamp provider was constructed.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
fn timer() -> tracing_subscriber::fmt::time::Uptime {
tracing_subscriber::fmt::time::Uptime::default()
}
```

