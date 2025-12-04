*[tracing](../index.md) / [instrument](index.md)*

---

# Module `instrument`

Attach a span to a `std::future::Future`.

## Structs

### `WithDispatch<T>`

```rust
struct WithDispatch<T> {
    // [REDACTED: Private Fields]
}
```

A [`Future`](#future) that has been instrumented with a `tracing` [`Subscriber`](../index.md).

This type is returned by the [`WithSubscriber`](#withsubscriber) extension trait. See that
trait's documentation for details.



#### Implementations

- `fn dispatcher(self: &Self) -> &Dispatch`
  Borrows the [`Dispatch`] that is entered when this type is polled.

- `fn inner(self: &Self) -> &T`
  Borrows the wrapped type.

- `fn inner_mut(self: &mut Self) -> &mut T`
  Mutably borrows the wrapped type.

- `fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`
  Get a pinned reference to the wrapped type.

- `fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`
  Get a pinned mutable reference to the wrapped type.

- `fn into_inner(self: Self) -> T`
  Consumes the `Instrumented`, returning the wrapped type.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoFuture<F>`

- `type Output = <F as Future>::Output`

- `type IntoFuture = F`

- `fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> WithDispatch<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Future<T: Future>`

- `type Output = <T as Future>::Output`

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl Instrument<T>`

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

##### `impl Unpin<'__pin, T>`

##### `impl WithSubscriber<T>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Instrumented<T>`

```rust
struct Instrumented<T> {
    // [REDACTED: Private Fields]
}
```

A [`Future`](#future) that has been instrumented with a `tracing` [`Span`](../span/index.md).

This type is returned by the [`Instrument`](#instrument) extension trait. See that
trait's documentation for details.



#### Implementations

- `fn span(self: &Self) -> &Span`
  Borrows the `Span` that this type is instrumented by.

- `fn span_mut(self: &mut Self) -> &mut Span`
  Mutably borrows the `Span` that this type is instrumented by.

- `fn inner(self: &Self) -> &T`
  Borrows the wrapped type.

- `fn inner_mut(self: &mut Self) -> &mut T`
  Mutably borrows the wrapped type.

- `fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`
  Get a pinned reference to the wrapped type.

- `fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`
  Get a pinned mutable reference to the wrapped type.

- `fn into_inner(self: Self) -> T`
  Consumes the `Instrumented`, returning the wrapped type.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoFuture<F>`

- `type Output = <F as Future>::Output`

- `type IntoFuture = F`

- `fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Instrumented<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop<T>`

- `fn drop(self: &mut Self)`

##### `impl Future<T: Future>`

- `type Output = <T as Future>::Output`

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl Instrument<T>`

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

##### `impl Unpin<'__pin, T>`

##### `impl WithSubscriber<T>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Instrument`

```rust
trait Instrument: Sized { ... }
```

Attaches spans to a `std::future::Future`.

Extension trait allowing futures to be
instrumented with a `tracing` [span](#span)
.

[span](#span)
: super::Span

#### Required Methods

- `fn instrument(self: Self, span: Span) -> Instrumented<Self>`

  Instruments this type with the provided [`Span`](../span/index.md), returning an

- `fn in_current_span(self: Self) -> Instrumented<Self>`

  Instruments this type with the [current](#current)

### `WithSubscriber`

```rust
trait WithSubscriber: Sized { ... }
```

Extension trait allowing futures to be instrumented with
a `tracing` [`Subscriber`](crate::Subscriber).

#### Required Methods

- `fn with_subscriber<S>(self: Self, subscriber: S) -> WithDispatch<Self>`

  Attaches the provided [`Subscriber`](../index.md) to this type, returning a

- `fn with_current_subscriber(self: Self) -> WithDispatch<Self>`

  Attaches the current [default](#default)

