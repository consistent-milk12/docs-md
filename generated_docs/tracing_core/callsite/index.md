*[tracing_core](../index.md) / [callsite](index.md)*

---

# Module `callsite`

Callsites represent the source locations from which spans or events
originate.

# What Are Callsites?

Every span or event in `tracing` is associated with a [`Callsite`](#callsite). A
callsite is a small `static` value that is responsible for the following:

* Storing the span or event's [`Metadata`](../metadata/index.md),
* Uniquely [identifying](Identifier) the span or event definition,
* Caching the subscriber's [`Interest`][^1] in that span or event, to avoid
  re-evaluating filters.

# Registering Callsites

When a span or event is recorded for the first time, its callsite
[`register`](#register)s itself with the global callsite registry. Registering a
callsite calls the [`Subscriber::register_callsite`]`register_callsite`
method with that callsite's [`Metadata`](../metadata/index.md) on every currently active
subscriber. This serves two primary purposes: informing subscribers of the
callsite's existence, and performing static filtering.

## Callsite Existence

If a [`Subscriber`](../subscriber/index.md) implementation wishes to allocate storage for each
unique span/event location in the program, or pre-compute some value
that will be used to record that span or event in the future, it can
do so in its `register_callsite` method.

## Performing Static Filtering

The `register_callsite` method returns an [`Interest`](../subscriber/index.md) value,
which indicates that the subscriber either [always] wishes to record
that span or event, [sometimes] wishes to record it based on a
dynamic filter evaluation, or [never] wishes to record it.

When registering a new callsite, the [`Interest`](../subscriber/index.md)s returned by every
currently active subscriber are combined, and the result is stored at
each callsite. This way, when the span or event occurs in the
future, the cached [`Interest`](../subscriber/index.md) value can be checked efficiently
to determine if the span or event should be recorded, without
needing to perform expensive filtering (i.e. calling the
`Subscriber::enabled` method every time a span or event occurs).

### Rebuilding Cached Interest

When a new [`Dispatch`](../dispatcher/index.md) is created (i.e. a new subscriber becomes
active), any previously cached [`Interest`](../subscriber/index.md) values are re-evaluated
for all callsites in the program. This way, if the new subscriber
will enable a callsite that was not previously enabled, the
[`Interest`](../subscriber/index.md) in that callsite is updated. Similarly, when a
subscriber is dropped, the interest cache is also re-evaluated, so
that any callsites enabled only by that subscriber are disabled.

In addition, the [`rebuild_interest_cache`](#rebuild-interest-cache) function in this module can be
used to manually invalidate all cached interest and re-register those
callsites. This function is useful in situations where a subscriber's
interest can change, but it does so relatively infrequently. The subscriber
may wish for its interest to be cached most of the time, and return
[`Interest::always`][always] or [`Interest::never`][never] in its
`register_callsite` method, so that its `Subscriber::enabled` method
doesn't need to be evaluated every time a span or event is recorded.
However, when the configuration changes, the subscriber can call
[`rebuild_interest_cache`](#rebuild-interest-cache) to re-evaluate the entire interest cache with its
new configuration. This is a relatively costly operation, but if the
configuration changes infrequently, it may be more efficient than calling
`Subscriber::enabled` frequently.

# Implementing Callsites

In most cases, instrumenting code using `tracing` should *not* require
implementing the [`Callsite`](#callsite) trait directly. When using the [`tracing`
crate's macros][`macros`](../../aho_corasick/macros/index.md) or the [`#[instrument]` attribute][`instrument`](../../tracing/instrument/index.md), a
`Callsite` is automatically generated.

However, code which provides alternative forms of `tracing` instrumentation
may need to interact with the callsite system directly. If
instrumentation-side code needs to produce a `Callsite` to emit spans or
events, the [`DefaultCallsite`](#defaultcallsite) struct provided in this module is a
ready-made `Callsite` implementation that is suitable for most uses. When
possible, the use of `DefaultCallsite` should be preferred over implementing
[`Callsite`](#callsite) for user types, as `DefaultCallsite` may benefit from
additional performance optimizations.

[^1]: Returned by the [`Subscriber::register_callsite`]`register_callsite`
    method.












## Contents

- [Modules](#modules)
  - [`private`](#private)
  - [`dispatchers`](#dispatchers)
- [Structs](#structs)
  - [`Identifier`](#identifier)
  - [`DefaultCallsite`](#defaultcallsite)
  - [`Callsites`](#callsites)
- [Traits](#traits)
  - [`Callsite`](#callsite)
- [Functions](#functions)
  - [`rebuild_interest_cache`](#rebuild-interest-cache)
  - [`register`](#register)
  - [`register_dispatch`](#register-dispatch)
  - [`rebuild_callsite_interest`](#rebuild-callsite-interest)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`dispatchers`](#dispatchers) | mod |  |
| [`Identifier`](#identifier) | struct | Uniquely identifies a [`Callsite`] |
| [`DefaultCallsite`](#defaultcallsite) | struct | A default [`Callsite`] implementation. |
| [`Callsites`](#callsites) | struct |  |
| [`Callsite`](#callsite) | trait | Trait implemented by callsites. |
| [`rebuild_interest_cache`](#rebuild-interest-cache) | fn | Clear and reregister interest on every [`Callsite`] |
| [`register`](#register) | fn | Register a new [`Callsite`] with the global registry. |
| [`register_dispatch`](#register-dispatch) | fn |  |
| [`rebuild_callsite_interest`](#rebuild-callsite-interest) | fn |  |

## Modules

- [`private`](private/index.md)
- [`dispatchers`](dispatchers/index.md)

## Structs

### `Identifier`

```rust
struct Identifier();
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:178-188`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L178-L188)*

Uniquely identifies a [`Callsite`](#callsite)

Two `Identifier`s are equal if they both refer to the same callsite.


#### Trait Implementations

##### `impl Clone for Identifier`

- <span id="identifier-clone"></span>`fn clone(&self) -> Identifier` — [`Identifier`](#identifier)

##### `impl Debug for Identifier`

- <span id="identifier-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Identifier`

##### `impl Hash for Identifier`

- <span id="identifier-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for Identifier`

- <span id="identifier-eq"></span>`fn eq(&self, other: &Identifier) -> bool` — [`Identifier`](#identifier)

### `DefaultCallsite`

```rust
struct DefaultCallsite {
    interest: core::sync::atomic::AtomicU8,
    registration: core::sync::atomic::AtomicU8,
    meta: &'static crate::metadata::Metadata<'static>,
    next: core::sync::atomic::AtomicPtr<Self>,
}
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:192-197`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L192-L197)*

A default [`Callsite`](#callsite) implementation.

#### Implementations

- <span id="defaultcallsite-const-unregistered"></span>`const UNREGISTERED: u8`

- <span id="defaultcallsite-const-registering"></span>`const REGISTERING: u8`

- <span id="defaultcallsite-const-registered"></span>`const REGISTERED: u8`

- <span id="defaultcallsite-const-interest-never"></span>`const INTEREST_NEVER: u8`

- <span id="defaultcallsite-const-interest-sometimes"></span>`const INTEREST_SOMETIMES: u8`

- <span id="defaultcallsite-const-interest-always"></span>`const INTEREST_ALWAYS: u8`

- <span id="defaultcallsite-new"></span>`const fn new(meta: &'static Metadata<'static>) -> Self` — [`Metadata`](../metadata/index.md#metadata)

- <span id="defaultcallsite-register"></span>`fn register(self: &'static Self) -> Interest` — [`Interest`](../subscriber/index.md#interest)

- <span id="defaultcallsite-interest"></span>`fn interest(self: &'static Self) -> Interest` — [`Interest`](../subscriber/index.md#interest)

#### Trait Implementations

##### `impl Callsite for DefaultCallsite`

- <span id="defaultcallsite-set-interest"></span>`fn set_interest(&self, interest: Interest)` — [`Interest`](../subscriber/index.md#interest)

- <span id="defaultcallsite-metadata"></span>`fn metadata(&self) -> &Metadata<'static>` — [`Metadata`](../metadata/index.md#metadata)

##### `impl Debug for DefaultCallsite`

- <span id="defaultcallsite-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Callsites`

```rust
struct Callsites {
    list_head: core::sync::atomic::AtomicPtr<DefaultCallsite>,
    has_locked_callsites: core::sync::atomic::AtomicBool,
}
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:264-267`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L264-L267)*

#### Implementations

- <span id="callsites-rebuild-interest"></span>`fn rebuild_interest(&self, dispatchers: dispatchers::Rebuilder<'_>)` — [`Rebuilder`](dispatchers/index.md#rebuilder)

- <span id="callsites-push-dyn"></span>`fn push_dyn(&self, callsite: &'static dyn Callsite)` — [`Callsite`](#callsite)

- <span id="callsites-push-default"></span>`fn push_default(&self, callsite: &'static DefaultCallsite)` — [`DefaultCallsite`](#defaultcallsite)

- <span id="callsites-for-each"></span>`fn for_each(&self, f: impl FnMut(&'static dyn Callsite))` — [`Callsite`](#callsite)

## Traits

### `Callsite`

```rust
trait Callsite: Sync { ... }
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:125-170`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L125-L170)*

Trait implemented by callsites.

These functions are only intended to be called by the callsite registry, which
correctly handles determining the common interest between all subscribers.

See the [module-level documentation](crate::callsite) for details on
callsites.

#### Required Methods

- `fn set_interest(&self, interest: Interest)`

  Sets the [`Interest`](../subscriber/index.md) for this callsite.

- `fn metadata(&self) -> &Metadata<'_>`

  Returns the [`metadata`](../metadata/index.md) associated with the callsite.

#### Implementors

- [`DefaultCallsite`](#defaultcallsite)

## Functions

### `rebuild_interest_cache`

```rust
fn rebuild_interest_cache()
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:222-224`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L222-L224)*

Clear and reregister interest on every [`Callsite`](#callsite)

This function is intended for runtime reconfiguration of filters on traces
when the filter recalculation is much less frequent than trace events are.
The alternative is to have the [`Subscriber`](../subscriber/index.md) that supports runtime
reconfiguration of filters always return `Interest::sometimes()` so that
`enabled` is evaluated for every event.

This function will also re-compute the global maximum level as determined by
the `max_level_hint` method. If a [`Subscriber`](../subscriber/index.md)
implementation changes the value returned by its `max_level_hint`
implementation at runtime, then it **must** call this function after that
value changes, in order for the change to be reflected.

See the [documentation on callsite interest caching][cache-docs] for
additional information on this function's usage.







### `register`

```rust
fn register(callsite: &'static dyn Callsite)
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:236-253`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L236-L253)*

Register a new [`Callsite`](#callsite) with the global registry.

This should be called once per callsite after the callsite has been
constructed.

See the [documentation on callsite registration][reg-docs] for details
on the global callsite registry.



### `register_dispatch`

```rust
fn register_dispatch(dispatch: &crate::dispatcher::Dispatch)
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:484-488`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L484-L488)*

### `rebuild_callsite_interest`

```rust
fn rebuild_callsite_interest(callsite: &'static dyn Callsite, dispatchers: &dispatchers::Rebuilder<'_>)
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:490-507`](../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L490-L507)*

