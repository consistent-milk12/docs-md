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
callsite calls the [`Subscriber::register_callsite`][`register_callsite`](#register-callsite)
method with that callsite's [`Metadata`](../metadata/index.md) on every currently active
subscriber. This serves two primary purposes: informing subscribers of the
callsite's existence, and performing static filtering.

## Callsite Existence

If a [`Subscriber`](../subscriber/index.md) implementation wishes to allocate storage for each
unique span/event location in the program, or pre-compute some value
that will be used to record that span or event in the future, it can
do so in its [`register_callsite`](#register-callsite) method.

## Performing Static Filtering

The [`register_callsite`](#register-callsite) method returns an [`Interest`](../subscriber/index.md) value,
which indicates that the subscriber either [always](#always)
 wishes to record
that span or event, [sometimes](#sometimes)
 wishes to record it based on a
dynamic filter evaluation, or [never](#never)
 wishes to record it.

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
[`Interest::always`][always](#always)
 or [`Interest::never`][never](#never)
 in its
[`register_callsite`](#register-callsite) method, so that its `Subscriber::enabled` method
doesn't need to be evaluated every time a span or event is recorded.
However, when the configuration changes, the subscriber can call
[`rebuild_interest_cache`](#rebuild-interest-cache) to re-evaluate the entire interest cache with its
new configuration. This is a relatively costly operation, but if the
configuration changes infrequently, it may be more efficient than calling
`Subscriber::enabled` frequently.

# Implementing Callsites

In most cases, instrumenting code using `tracing` should *not* require
implementing the [`Callsite`](#callsite) trait directly. When using the [`tracing`
crate's macros][macros](#macros)
 or the [`#[instrument](#instrument)
` attribute][instrument](#instrument)
, a
`Callsite` is automatically generated.

However, code which provides alternative forms of `tracing` instrumentation
may need to interact with the callsite system directly. If
instrumentation-side code needs to produce a `Callsite` to emit spans or
events, the [`DefaultCallsite`](#defaultcallsite) struct provided in this module is a
ready-made `Callsite` implementation that is suitable for most uses. When
possible, the use of `DefaultCallsite` should be preferred over implementing
[`Callsite`](#callsite) for user types, as `DefaultCallsite` may benefit from
additional performance optimizations.

[^1]: Returned by the [`Subscriber::register_callsite`][`register_callsite`](#register-callsite)
    method.





[always](#always)
: crate::subscriber::Interest::always
[sometimes](#sometimes)
: crate::subscriber::Interest::sometimes
[never](#never)
: crate::subscriber::Interest::never

[macros](#macros)
: https://docs.rs/tracing/latest/tracing/#macros
[instrument](#instrument)
: https://docs.rs/tracing/latest/tracing/attr.instrument.html

## Structs

### `Identifier`

```rust
struct Identifier();
```

Uniquely identifies a [`Callsite`](#callsite)

Two `Identifier`s are equal if they both refer to the same callsite.


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

##### `impl Clone`

- `fn clone(self: &Self) -> Identifier`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Identifier) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DefaultCallsite`

```rust
struct DefaultCallsite {
    // [REDACTED: Private Fields]
}
```

A default [`Callsite`](#callsite) implementation.

#### Implementations

- `const fn new(meta: &'static Metadata<'static>) -> Self`
  Returns a new `DefaultCallsite` with the specified `Metadata`.

- `fn register(self: &'static Self) -> Interest`
  Registers this callsite with the global callsite registry.

- `fn interest(self: &'static Self) -> Interest`
  Returns the callsite's cached `Interest`, or registers it for the

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

##### `impl Callsite`

- `fn set_interest(self: &Self, interest: Interest)`

- `fn metadata(self: &Self) -> &Metadata<'static>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Callsite`

```rust
trait Callsite: Sync { ... }
```

Trait implemented by callsites.

These functions are only intended to be called by the callsite registry, which
correctly handles determining the common interest between all subscribers.

See the [module-level documentation](crate::callsite) for details on
callsites.

#### Required Methods

- `fn set_interest(self: &Self, interest: Interest)`

  Sets the [`Interest`](../subscriber/index.md) for this callsite.

- `fn metadata(self: &Self) -> &Metadata<'_>`

  Returns the [metadata](#metadata)

## Functions

### `rebuild_interest_cache`

```rust
fn rebuild_interest_cache()
```

Clear and reregister interest on every [`Callsite`](#callsite)

This function is intended for runtime reconfiguration of filters on traces
when the filter recalculation is much less frequent than trace events are.
The alternative is to have the [`Subscriber`](../subscriber/index.md) that supports runtime
reconfiguration of filters always return `Interest::sometimes()` so that
[`enabled`](#enabled) is evaluated for every event.

This function will also re-compute the global maximum level as determined by
the [`max_level_hint`](#max-level-hint) method. If a [`Subscriber`](../subscriber/index.md)
implementation changes the value returned by its `max_level_hint`
implementation at runtime, then it **must** call this function after that
value changes, in order for the change to be reflected.

See the [documentation on callsite interest caching][cache-docs] for
additional information on this function's usage.





[cache-docs]: crate::callsite#rebuilding-cached-interest

### `register`

```rust
fn register(callsite: &'static dyn Callsite)
```

Register a new [`Callsite`](#callsite) with the global registry.

This should be called once per callsite after the callsite has been
constructed.

See the [documentation on callsite registration][reg-docs] for details
on the global callsite registry.

[reg-docs]: crate::callsite#registering-callsites

