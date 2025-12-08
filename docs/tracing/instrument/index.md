*[tracing](../index.md) / [instrument](index.md)*

---

# Module `instrument`

Attach a span to a `std::future::Future`.

## Structs

### `WithDispatch<T>`

```rust
struct WithDispatch<T> {
    inner: T,
    dispatcher: crate::dispatcher::Dispatch,
}
```

A `Future` that has been instrumented with a `tracing` [`Subscriber`](../../tracing_core/index.md).

This type is returned by the [`WithSubscriber`](#withsubscriber) extension trait. See that
trait's documentation for details.



#### Implementations

- `fn dispatcher(self: &Self) -> &Dispatch` — [`Dispatch`](../index.md)

- `fn inner(self: &Self) -> &T`

- `fn inner_mut(self: &mut Self) -> &mut T`

- `fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`

- `fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`

- `fn into_inner(self: Self) -> T`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for WithDispatch<T>`

- `fn clone(self: &Self) -> WithDispatch<T>` — [`WithDispatch`](#withdispatch)

##### `impl<T: $crate::fmt::Debug> Debug for WithDispatch<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Future> Future for WithDispatch<T>`

- `type Output = <T as Future>::Output`

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T> Instrument for WithDispatch<T>`

##### `impl<F> IntoFuture for WithDispatch<T>`

- `type Output = <F as Future>::Output`

- `type IntoFuture = F`

- `fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture`

##### `impl<'__pin, T> Unpin for WithDispatch<T>`

##### `impl<T> WithSubscriber for WithDispatch<T>`

### `Instrumented<T>`

```rust
struct Instrumented<T> {
    inner: core::mem::ManuallyDrop<T>,
    span: crate::span::Span,
}
```

A `Future` that has been instrumented with a `tracing` [`Span`](../index.md).

This type is returned by the [`Instrument`](../index.md) extension trait. See that
trait's documentation for details.



#### Implementations

- `fn span(self: &Self) -> &Span` — [`Span`](../index.md)

- `fn span_mut(self: &mut Self) -> &mut Span` — [`Span`](../index.md)

- `fn inner(self: &Self) -> &T`

- `fn inner_mut(self: &mut Self) -> &mut T`

- `fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`

- `fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`

- `fn into_inner(self: Self) -> T`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Instrumented<T>`

- `fn clone(self: &Self) -> Instrumented<T>` — [`Instrumented`](#instrumented)

##### `impl<T: $crate::fmt::Debug> Debug for Instrumented<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Drop for Instrumented<T>`

- `fn drop(self: &mut Self)`

##### `impl<T: Future> Future for Instrumented<T>`

- `type Output = <T as Future>::Output`

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T> Instrument for Instrumented<T>`

##### `impl<F> IntoFuture for Instrumented<T>`

- `type Output = <F as Future>::Output`

- `type IntoFuture = F`

- `fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture`

##### `impl<'__pin, T> Unpin for Instrumented<T>`

##### `impl<T> WithSubscriber for Instrumented<T>`

## Traits

### `Instrument`

```rust
trait Instrument: Sized { ... }
```

Attaches spans to a `std::future::Future`.

Extension trait allowing futures to be
instrumented with a `tracing` [`span`](../span/index.md).


#### Required Methods

- `fn instrument(self: Self, span: Span) -> Instrumented<Self>`

  Instruments this type with the provided [`Span`](../index.md), returning an

- `fn in_current_span(self: Self) -> Instrumented<Self>`

  Instruments this type with the [current] [`Span`](../index.md), returning an

### `WithSubscriber`

```rust
trait WithSubscriber: Sized { ... }
```

Extension trait allowing futures to be instrumented with
a `tracing` [`Subscriber`](crate::Subscriber).

#### Required Methods

- `fn with_subscriber<S>(self: Self, subscriber: S) -> WithDispatch<Self>`

  Attaches the provided [`Subscriber`](../../tracing_core/index.md) to this type, returning a

- `fn with_current_subscriber(self: Self) -> WithDispatch<Self>`

  Attaches the current [`default`](../../crossbeam_epoch/default/index.md) [`Subscriber`](../../tracing_core/index.md) to this type, returning a

