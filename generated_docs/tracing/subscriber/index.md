*[tracing](../index.md) / [subscriber](index.md)*

---

# Module `subscriber`

Collects and records trace data.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DefaultGuard`](#defaultguard) | trait |  |
| [`with_default`](#with-default) | fn | Sets this [`Subscriber`] as the default for the current thread for the duration of a closure. |
| [`set_global_default`](#set-global-default) | fn | Sets this subscriber as the global default for the duration of the entire program. |
| [`set_default`](#set-default) | fn | Sets the [`Subscriber`] as the default for the current thread for the duration of the lifetime of the returned [`DefaultGuard`]. |

## Traits

### `DefaultGuard`

```rust
trait DefaultGuard: Automaton + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { ... }
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2643-2646`](../../../.source_1765210505/aho-corasick-1.1.4/src/ahocorasick.rs#L2643-L2646)*

A trait that effectively gives us practical dynamic dispatch over anything
that impls `Automaton`, but without needing to add a bunch of bounds to
the core `Automaton` trait. Basically, we provide all of the marker traits
that our automatons have, in addition to `Debug` impls and requiring that
there is no borrowed data. Without these, the main `AhoCorasick` type would
not be able to meaningfully impl `Debug` or the marker traits without also
requiring that all impls of `Automaton` do so, which would be not great.

## Functions

### `with_default`

```rust
fn with_default<T, S>(subscriber: S, f: impl FnOnce() -> T) -> T
where
    S: Subscriber + Send + Sync + 'static
```

*Defined in [`tracing-0.1.43/src/subscriber.rs:20-25`](../../../.source_1765210505/tracing-0.1.43/src/subscriber.rs#L20-L25)*

Sets this [`Subscriber`](../../tracing_core/subscriber/index.md) as the default for the current thread for the
duration of a closure.

The default subscriber is used when creating a new [`Span`](../span/index.md) or
[`Event`](../../tracing_core/event/index.md).




### `set_global_default`

```rust
fn set_global_default<S>(subscriber: S) -> Result<(), SetGlobalDefaultError>
where
    S: Subscriber + Send + Sync + 'static
```

*Defined in [`tracing-0.1.43/src/subscriber.rs:38-43`](../../../.source_1765210505/tracing-0.1.43/src/subscriber.rs#L38-L43)*

Sets this subscriber as the global default for the duration of the entire program.
Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail.
Returns whether the initialization was successful.

Note: Libraries should *NOT* call `set_global_default()`! That will cause conflicts when
executables try to set them later.



### `set_default`

```rust
fn set_default<S>(subscriber: S) -> DefaultGuard
where
    S: Subscriber + Send + Sync + 'static
```

*Defined in [`tracing-0.1.43/src/subscriber.rs:57-62`](../../../.source_1765210505/tracing-0.1.43/src/subscriber.rs#L57-L62)*

Sets the [`Subscriber`](../../tracing_core/subscriber/index.md) as the default for the current thread for the
duration of the lifetime of the returned [`DefaultGuard`](../../tracing_core/dispatcher/index.md).

The default subscriber is used when creating a new [`Span`](../span/index.md) or [`Event`](../../tracing_core/event/index.md).





