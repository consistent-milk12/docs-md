*[tracing_subscriber](../index.md) / [util](index.md)*

---

# Module `util`

Extension traits and other utilities to make working with subscribers more
ergonomic.

## Structs

### `TryInitError`

```rust
struct TryInitError {
    inner: alloc::boxed::Box<dyn Error + Send + Sync>,
}
```

Error returned by [`try_init`](SubscriberInitExt::try_init) if a global default subscriber could not be initialized.

#### Implementations

- `fn new(e: impl Into<Box<dyn Error + Send + Sync>>) -> Self`

#### Trait Implementations

##### `impl Debug for TryInitError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryInitError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for TryInitError`

- `fn source(self: &Self) -> Option<&dyn Error>`

##### `impl<T> Instrument for TryInitError`

##### `impl<T> ToString for TryInitError`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for TryInitError`

## Traits

### `SubscriberInitExt`

```rust
trait SubscriberInitExt
where
    Self: Into<tracing_core::dispatcher::Dispatch> { ... }
```

Extension trait adding utility methods for subscriber initialization.

This trait provides extension methods to make configuring and setting a
[default subscriber] more ergonomic. It is automatically implemented for all
types that can be converted into a [trace dispatcher]. Since `Dispatch`
implements `From<T>` for all `T: Subscriber`, all `Subscriber`
implementations will implement this extension trait as well. Types which
can be converted into `Subscriber`s, such as builders that construct a
`Subscriber`, may implement `Into<Dispatch>`, and will also receive an
implementation of this trait.



#### Required Methods

- `fn set_default(self: Self) -> dispatcher::DefaultGuard`

  Sets `self` as the [default subscriber] in the current scope, returning a

- `fn try_init(self: Self) -> Result<(), TryInitError>`

  Attempts to set `self` as the [global default subscriber] in the current

- `fn init(self: Self)`

  Attempts to set `self` as the [global default subscriber] in the current

