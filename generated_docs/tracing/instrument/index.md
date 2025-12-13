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

*Defined in [`tracing-0.1.43/src/instrument.rs:236-252`](../../../.source_1765521767/tracing-0.1.43/src/instrument.rs#L236-L252)*

A `Future` that has been instrumented with a `tracing` [`Subscriber`](../../tracing_core/subscriber/index.md).

This type is returned by the [`WithSubscriber`](#withsubscriber) extension trait. See that
trait's documentation for details.



#### Implementations

- <span id="withdispatch-dispatcher"></span>`fn dispatcher(&self) -> &Dispatch` — [`Dispatch`](../dispatcher/index.md#dispatch)

  Borrows the [`Dispatch`](../dispatcher/index.md) that is entered when this type is polled.

- <span id="withdispatch-inner"></span>`fn inner(&self) -> &T`

  Borrows the wrapped type.

- <span id="withdispatch-inner-mut"></span>`fn inner_mut(&mut self) -> &mut T`

  Mutably borrows the wrapped type.

- <span id="withdispatch-inner-pin-ref"></span>`fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`

  Get a pinned reference to the wrapped type.

- <span id="withdispatch-inner-pin-mut"></span>`fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`

  Get a pinned mutable reference to the wrapped type.

- <span id="withdispatch-into-inner"></span>`fn into_inner(self) -> T`

  Consumes the `Instrumented`, returning the wrapped type.

  

  Note that this drops the span.

#### Trait Implementations

##### `impl<T> Any for WithDispatch<T>`

- <span id="withdispatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithDispatch<T>`

- <span id="withdispatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithDispatch<T>`

- <span id="withdispatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for WithDispatch<T>`

- <span id="withdispatch-clone"></span>`fn clone(&self) -> WithDispatch<T>` — [`WithDispatch`](#withdispatch)

##### `impl<T> CloneToUninit for WithDispatch<T>`

- <span id="withdispatch-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for WithDispatch<T>`

- <span id="withdispatch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WithDispatch<T>`

- <span id="withdispatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Future> Future for WithDispatch<T>`

- <span id="withdispatch-future-type-output"></span>`type Output = <T as Future>::Output`

- <span id="withdispatch-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T> Instrument for WithDispatch<T>`

##### `impl<T, U> Into for WithDispatch<T>`

- <span id="withdispatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoFuture for WithDispatch<T>`

- <span id="withdispatch-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="withdispatch-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="withdispatch-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> ToOwned for WithDispatch<T>`

- <span id="withdispatch-toowned-type-owned"></span>`type Owned = T`

- <span id="withdispatch-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="withdispatch-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for WithDispatch<T>`

- <span id="withdispatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withdispatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for WithDispatch<T>`

- <span id="withdispatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withdispatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> Unpin for WithDispatch<T>`

##### `impl<T> WithSubscriber for WithDispatch<T>`

### `Instrumented<T>`

```rust
struct Instrumented<T> {
    inner: core::mem::ManuallyDrop<T>,
    span: crate::span::Span,
}
```

*Defined in [`tracing-0.1.43/src/instrument.rs:254-288`](../../../.source_1765521767/tracing-0.1.43/src/instrument.rs#L254-L288)*

A `Future` that has been instrumented with a `tracing` [`Span`](../span/index.md).

This type is returned by the [`Instrument`](#instrument) extension trait. See that
trait's documentation for details.



#### Implementations

- <span id="instrumented-span"></span>`fn span(&self) -> &Span` — [`Span`](../span/index.md#span)

  Borrows the `Span` that this type is instrumented by.

- <span id="instrumented-span-mut"></span>`fn span_mut(&mut self) -> &mut Span` — [`Span`](../span/index.md#span)

  Mutably borrows the `Span` that this type is instrumented by.

- <span id="instrumented-inner"></span>`fn inner(&self) -> &T`

  Borrows the wrapped type.

- <span id="instrumented-inner-mut"></span>`fn inner_mut(&mut self) -> &mut T`

  Mutably borrows the wrapped type.

- <span id="instrumented-inner-pin-ref"></span>`fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T>`

  Get a pinned reference to the wrapped type.

- <span id="instrumented-inner-pin-mut"></span>`fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`

  Get a pinned mutable reference to the wrapped type.

- <span id="instrumented-into-inner"></span>`fn into_inner(self) -> T`

  Consumes the `Instrumented`, returning the wrapped type.

  

  Note that this drops the span.

#### Trait Implementations

##### `impl<T> Any for Instrumented<T>`

- <span id="instrumented-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Instrumented<T>`

- <span id="instrumented-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Instrumented<T>`

- <span id="instrumented-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Instrumented<T>`

- <span id="instrumented-clone"></span>`fn clone(&self) -> Instrumented<T>` — [`Instrumented`](#instrumented)

##### `impl<T> CloneToUninit for Instrumented<T>`

- <span id="instrumented-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Instrumented<T>`

- <span id="instrumented-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Instrumented<T>`

- <span id="instrumented-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Instrumented<T>`

- <span id="instrumented-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Future> Future for Instrumented<T>`

- <span id="instrumented-future-type-output"></span>`type Output = <T as Future>::Output`

- <span id="instrumented-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T> Instrument for Instrumented<T>`

##### `impl<T, U> Into for Instrumented<T>`

- <span id="instrumented-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoFuture for Instrumented<T>`

- <span id="instrumented-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="instrumented-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="instrumented-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> ToOwned for Instrumented<T>`

- <span id="instrumented-toowned-type-owned"></span>`type Owned = T`

- <span id="instrumented-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="instrumented-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Instrumented<T>`

- <span id="instrumented-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="instrumented-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Instrumented<T>`

- <span id="instrumented-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="instrumented-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> Unpin for Instrumented<T>`

##### `impl<T> WithSubscriber for Instrumented<T>`

## Traits

### `Instrument`

```rust
trait Instrument: Sized { ... }
```

*Defined in [`tracing-0.1.43/src/instrument.rs:20-131`](../../../.source_1765521767/tracing-0.1.43/src/instrument.rs#L20-L131)*

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

*Defined in [`tracing-0.1.43/src/instrument.rs:136-234`](../../../.source_1765521767/tracing-0.1.43/src/instrument.rs#L136-L234)*

Extension trait allowing futures to be instrumented with
a `tracing` [`Subscriber`](crate::Subscriber).

#### Provided Methods

- `fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>`

  Attaches the provided [`Subscriber`](../../tracing_core/subscriber/index.md) to this type, returning a

- `fn with_current_subscriber(self) -> WithDispatch<Self>`

  Attaches the current [`default`](../../crossbeam_epoch/default/index.md) [`Subscriber`](../../tracing_core/subscriber/index.md) to this type, returning a

#### Implementors

- `T`

