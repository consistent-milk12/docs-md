*[tracing](../index.md) / [subscriber](index.md)*

---

# Module `subscriber`

Collects and records trace data.

## Functions

### `with_default`

```rust
fn with_default<T, S>(subscriber: S, f: impl FnOnce() -> T) -> T
where
    S: Subscriber + Send + Sync + 'static
```

Sets this [`Subscriber`](../index.md) as the default for the current thread for the
duration of a closure.

The default subscriber is used when creating a new [`Span`](../span/index.md) or
[`Event`](../index.md).




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

Sets the [`Subscriber`](../index.md) as the default for the current thread for the
duration of the lifetime of the returned [`DefaultGuard`](../dispatcher/index.md).

The default subscriber is used when creating a new [`Span`](../span/index.md) or [`Event`](../index.md).





