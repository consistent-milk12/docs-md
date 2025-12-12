*[tracing](../index.md) / [instrument](index.md)*

---

# Module `instrument`

Attach a span to a `std::future::Future`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WithDispatch`](#withdispatch) | struct | A [`Future`] that has been instrumented with a `tracing` [`Subscriber`]. |
| [`Instrumented`](#instrumented) | struct | A [`Future`] that has been instrumented with a `tracing` [`Span`]. |
| [`Instrument`](#instrument) | trait | Attaches spans to a [`std::future::Future`]. |
| [`WithSubscriber`](#withsubscriber) | trait | Extension trait allowing futures to be instrumented with a `tracing` [`Subscriber`](crate::Subscriber). |

## Structs

### `WithDispatch<T>`

```rust
struct WithDispatch<T> {
    inner: T,
    dispatcher: crate::dispatcher::Dispatch,
}
```

*Defined in [`tracing-0.1.43/src/instrument.rs:236-252`](../../../.source_1765210505/tracing-0.1.43/src/instrument.rs#L236-L252)*

A `Future` that has been instrumented with a `tracing` [`Subscriber`](../../tracing_core/subscriber/index.md).

This type is returned by the [`WithSubscriber`](#withsubscriber) extension trait. See that
trait's documentation for details.



#### Implementations

- <span id="withdispatch-dispatcher"></span>`fn dispatcher(&self) -> &Dispatch` — [`Dispatch`](../dispatcher/index.md#dispatch)

- <span id="withdispatch-inner"></span>`fn inner(&self) -> &T`

- <span id="withdispatch-inner-mut"></span>`fn inner_mut(&mut self) -> &mut T`

- <span id="withdispatch-inner-pin-ref"></span>`fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`

- <span id="withdispatch-inner-pin-mut"></span>`fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`

- <span id="withdispatch-into-inner"></span>`fn into_inner(self) -> T`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for WithDispatch<T>`

- <span id="withdispatch-clone"></span>`fn clone(&self) -> WithDispatch<T>` — [`WithDispatch`](#withdispatch)

##### `impl<T: fmt::Debug> Debug for WithDispatch<T>`

- <span id="withdispatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Future> Future for WithDispatch<T>`

- <span id="withdispatch-future-type-output"></span>`type Output = <T as Future>::Output`

- <span id="withdispatch-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T> Instrument for WithDispatch<T>`

##### `impl IntoFuture for WithDispatch<T>`

- <span id="withdispatch-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="withdispatch-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="withdispatch-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> Unpin for WithDispatch<T>`

##### `impl<T> WithSubscriber for WithDispatch<T>`

### `Instrumented<T>`

```rust
struct Instrumented<T> {
    inner: core::mem::ManuallyDrop<T>,
    span: crate::span::Span,
}
```

*Defined in [`tracing-0.1.43/src/instrument.rs:254-288`](../../../.source_1765210505/tracing-0.1.43/src/instrument.rs#L254-L288)*

A `Future` that has been instrumented with a `tracing` [`Span`](../span/index.md).

This type is returned by the [`Instrument`](#instrument) extension trait. See that
trait's documentation for details.



#### Implementations

- <span id="instrumented-span"></span>`fn span(&self) -> &Span` — [`Span`](../span/index.md#span)

- <span id="instrumented-span-mut"></span>`fn span_mut(&mut self) -> &mut Span` — [`Span`](../span/index.md#span)

- <span id="instrumented-inner"></span>`fn inner(&self) -> &T`

- <span id="instrumented-inner-mut"></span>`fn inner_mut(&mut self) -> &mut T`

- <span id="instrumented-inner-pin-ref"></span>`fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`

- <span id="instrumented-inner-pin-mut"></span>`fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`

- <span id="instrumented-into-inner"></span>`fn into_inner(self) -> T`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Instrumented<T>`

- <span id="instrumented-clone"></span>`fn clone(&self) -> Instrumented<T>` — [`Instrumented`](#instrumented)

##### `impl<T: fmt::Debug> Debug for Instrumented<T>`

- <span id="instrumented-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Instrumented<T>`

- <span id="instrumented-drop"></span>`fn drop(&mut self)`

##### `impl<T: Future> Future for Instrumented<T>`

- <span id="instrumented-future-type-output"></span>`type Output = <T as Future>::Output`

- <span id="instrumented-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T> Instrument for Instrumented<T>`

##### `impl IntoFuture for Instrumented<T>`

- <span id="instrumented-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="instrumented-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="instrumented-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> Unpin for Instrumented<T>`

##### `impl<T> WithSubscriber for Instrumented<T>`

## Traits

### `Instrument`

```rust
trait Instrument: Sized { ... }
```

*Defined in [`tracing-0.1.43/src/instrument.rs:20-131`](../../../.source_1765210505/tracing-0.1.43/src/instrument.rs#L20-L131)*

Attaches spans to a `std::future::Future`.

Extension trait allowing futures to be
instrumented with a `tracing` [`span`](../span/index.md).


#### Provided Methods

- `fn instrument(self, span: Span) -> Instrumented<Self>`

  Instruments this type with the provided [`Span`](../span/index.md), returning an

- `fn in_current_span(self) -> Instrumented<Self>`

  Instruments this type with the [current] [`Span`](../span/index.md), returning an

#### Implementors

- `T`

### `WithSubscriber`

```rust
trait WithSubscriber: Sized { ... }
```

*Defined in [`tracing-0.1.43/src/instrument.rs:136-234`](../../../.source_1765210505/tracing-0.1.43/src/instrument.rs#L136-L234)*

Extension trait allowing futures to be instrumented with
a `tracing` [`Subscriber`](crate::Subscriber).

#### Provided Methods

- `fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>`

  Attaches the provided [`Subscriber`](../../tracing_core/subscriber/index.md) to this type, returning a

- `fn with_current_subscriber(self) -> WithDispatch<Self>`

  Attaches the current [`default`](../../crossbeam_epoch/default/index.md) [`Subscriber`](../../tracing_core/subscriber/index.md) to this type, returning a

#### Implementors

- `T`

