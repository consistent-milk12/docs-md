*[tracing](../index.md) / [subscriber](index.md)*

---

# Module `subscriber`

Collects and records trace data.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DefaultGuard`](#defaultguard) | trait |  |
| [`with_default`](#with_default) | fn | Sets this [`Subscriber`] as the default for the current thread for the |
| [`set_global_default`](#set_global_default) | fn | Sets this subscriber as the global default for the duration of the entire program. |
| [`set_default`](#set_default) | fn | Sets the [`Subscriber`] as the default for the current thread for the |

## Traits

### `DefaultGuard`

```rust
trait DefaultGuard: Automaton + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { ... }
```

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

Sets this [`Subscriber`](../../tracing_core/index.md) as the default for the current thread for the
duration of a closure.

The default subscriber is used when creating a new [`Span`](../index.md) or
[`Event`](../../tracing_core/index.md).




### `set_global_default`

```rust
fn set_global_default<S>(subscriber: S) -> Result<(), SetGlobalDefaultError>
where
    S: Subscriber + Send + Sync + 'static
```

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

Sets the [`Subscriber`](../../tracing_core/index.md) as the default for the current thread for the
duration of the lifetime of the returned [`DefaultGuard`](../../tracing_core/dispatcher/index.md).

The default subscriber is used when creating a new [`Span`](../index.md) or [`Event`](../../tracing_core/index.md).





