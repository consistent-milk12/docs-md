*[ureq](../../index.md) / [unversioned](../index.md) / [transport](index.md)*

---

# Module `transport`

HTTP/1.1 data transport.

**NOTE: transport does not (yet) [follow semver][super](#super)
.**

_NOTE: Transport is deep configuration of ureq and is not required for regular use._

ureq provides a pluggable transport layer making it possible to write bespoke
transports using the HTTP/1.1 protocol from point A to B. The
[`Agent::with_parts()`](crate::Agent::with_parts) constructor takes an implementation
of the [`Connector`](ureq/unversioned/transport/index.md) trait which is used for all connections made using that
agent.

The [DefaultConnector] covers the regular needs for HTTP/1.1:

* TCP Sockets
* SOCKS-proxy sockets
* HTTPS/TLS using rustls (feature flag **rustls**)
* HTTPS/TLS using native-tls (feature flag **native-tls** + [config](crate::tls::TlsProvider::NativeTls))

The [`Connector`](ureq/unversioned/transport/index.md) trait anticipates a chain of connectors that each decide
whether to help perform the connection or not. It is for instance possible to make a
connector handling other schemes than `http`/`https` without affecting "regular" connections.

## Modules

- [`time`](time/index.md) - Internal time wrappers

## Structs

### `LazyBuffers`

```rust
struct LazyBuffers {
    // [REDACTED: Private Fields]
}
```

Default buffer implementation.

The buffers are lazy such that no allocations are made until needed. That means
a [`Transport`](crate::transport::Transport) implementation can freely instantiate
the `LazyBuffers`.

#### Implementations

- `fn new(input_size: usize, output_size: usize) -> Self`
  Create a new buffer.

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

##### `impl Buffers`

- `fn output(self: &mut Self) -> &mut [u8]`

- `fn input(self: &Self) -> &[u8]`

- `fn input_append_buf(self: &mut Self) -> &mut [u8]`

- `fn tmp_and_output(self: &mut Self) -> (&mut [u8], &mut [u8])`

- `fn input_appended(self: &mut Self, amount: usize)`

- `fn input_consume(self: &mut Self, amount: usize)`

- `fn can_use_input(self: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TcpConnector`

```rust
struct TcpConnector();
```

Connector for regular TCP sockets.

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

##### `impl Connector<In: Transport>`

- `type Out = Either<In, TcpTransport>`

- `fn connect(self: &Self, details: &ConnectionDetails<'_>, chained: Option<In>) -> Result<Option<<Self as >::Out>, Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> TcpConnector`

### `TransportAdapter<T: Transport>`

```rust
struct TransportAdapter<T: Transport> {
    // [REDACTED: Private Fields]
}
```

Helper to turn a [`Transport`](ureq/unversioned/transport/index.md) into a std::io [`Read`](io::Read) and [`Write`](io::Write).

This is useful when integrating with components that expect a regular `Read`/`Write`. In
ureq this is used both for the [`RustlsConnector`](crate::unversioned::transport::RustlsConnector) and the
[`NativeTlsConnector`](crate::unversioned::transport::NativeTlsConnector).

#### Implementations

- `fn new(transport: T) -> Self`
  Creates a new adapter

- `fn set_timeout(self: &mut Self, timeout: NextTimeout)`
  Set a new value of the timeout.

- `fn get_ref(self: &Self) -> &dyn Transport`
  Reference to the adapted transport

- `fn get_mut(self: &mut Self) -> &mut dyn Transport`
  Mut reference to the adapted transport

- `fn inner(self: &Self) -> &dyn Transport`
  Reference to the inner transport.

- `fn into_inner(self: Self) -> T`
  Turn the adapter back into the wrapped transport

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

##### `impl Read<T: Transport>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<T: Transport>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

### `ChainedConnector<In, First, Second>`

```rust
struct ChainedConnector<In, First, Second>();
```

Two chained connectors called one after another.

Created by calling [`Connector::chain`](#chain) on the first connector.

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

##### `impl Clone<In, First, Second>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Connector<In, First, Second>`

- `type Out = <Second as Connector>::Out`

- `fn connect(self: &Self, details: &super::ConnectionDetails<'_>, chained: Option<In>) -> Result<Option<<Self as >::Out>, crate::Error>`

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

##### `impl Debug<In, First, Second>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ConnectProxyConnector`

```rust
struct ConnectProxyConnector();
```

Connector for CONNECT proxy settings.

This operates on the previous chained transport typically a TcpConnector optionally
wrapped in TLS.

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

##### `impl Connector<In: Transport>`

- `type Out = Either<In, Box<dyn Transport>>`

- `fn connect(self: &Self, details: &ConnectionDetails<'_>, chained: Option<In>) -> Result<Option<<Self as >::Out>, Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> ConnectProxyConnector`

### `RustlsConnector`

```rust
struct RustlsConnector {
    // [REDACTED: Private Fields]
}
```

Wrapper for TLS using rustls.

Requires feature flag **rustls**.

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

##### `impl Connector<In: Transport>`

- `type Out = Either<In, RustlsTransport>`

- `fn connect(self: &Self, details: &ConnectionDetails<'_>, chained: Option<In>) -> Result<Option<<Self as >::Out>, Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> RustlsConnector`

### `NextTimeout`

```rust
struct NextTimeout {
    pub after: crate::transport::time::Duration,
    pub reason: Timeout,
}
```

A pair of [`Duration`](ureq/unversioned/transport/time/index.md) and [`Timeout`](#timeout).

#### Fields

- **`after`**: `crate::transport::time::Duration`

  Duration until next timeout.

- **`reason`**: `Timeout`

  The name of the next timeout.s

#### Implementations

- `fn not_zero(self: &Self) -> Option<Duration>`
  Returns the duration of the timeout if the timeout must happen, but avoid instant timeouts

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

- `fn clone(self: &Self) -> NextTimeout`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &NextTimeout) -> bool`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ConnectionDetails<'a>`

```rust
struct ConnectionDetails<'a> {
    pub uri: &'a http::Uri,
    pub addrs: super::resolver::ResolvedSocketAddrs,
    pub config: &'a crate::config::Config,
    pub request_level: bool,
    pub resolver: &'a dyn Resolver,
    pub now: self::time::Instant,
    pub timeout: NextTimeout,
    pub run_connector: std::sync::Arc<dyn Fn(&ConnectionDetails<'_>) -> Result<Box<dyn Transport>, crate::Error> + Send + Sync>,
}
```

The parameters needed to create a [`Transport`](ureq/unversioned/transport/index.md).

#### Fields

- **`uri`**: `&'a http::Uri`

  Full uri that is being requested.
  
  In the case of CONNECT (HTTP) proxy, this is the URI of the
  proxy, and the actual URI is in the `proxied` field.

- **`addrs`**: `super::resolver::ResolvedSocketAddrs`

  The resolved IP address + port for the uri being requested. See [`Resolver`](ureq/unversioned/resolver/index.md).
  
  For proxies, whetherh this holds real addresses depends on
  [`Proxy::resolve_target()`](crate::Proxy::resolve_target).

- **`config`**: `&'a crate::config::Config`

  The configuration.
  
  Agent or Request level.

- **`request_level`**: `bool`

  Whether the config is request level.

- **`resolver`**: `&'a dyn Resolver`

  The resolver configured on [`Agent`](crate::Agent).
  
  Typically the IP address of the host in the uri is already resolved to the `addr`
  property. However there might be cases where additional DNS lookups need to be
  made in the connector itself, such as resolving a SOCKS proxy server.

- **`now`**: `self::time::Instant`

  Current time.

- **`timeout`**: `NextTimeout`

  The next timeout for making the connection.

- **`run_connector`**: `std::sync::Arc<dyn Fn(&ConnectionDetails<'_>) -> Result<Box<dyn Transport>, crate::Error> + Send + Sync>`

  Run the connector chain.
  
  Used for CONNECT proxy to establish a connection to the proxy server itself.

#### Implementations

- `fn needs_tls(self: &Self) -> bool`
  Tell if the requested socket need TLS wrapping.

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `DefaultConnector`

```rust
struct DefaultConnector {
    // [REDACTED: Private Fields]
}
```

Default connector providing TCP sockets, TLS and SOCKS proxy.

This connector is the following chain:

1. [`SocksConnector`](#socksconnector) to handle proxy settings if set.
2. [`TcpConnector`](#tcpconnector) to open a socket directly if a proxy is not used.
3. [`RustlsConnector`](#rustlsconnector) which wraps the
   connection from 1 or 2 in TLS if the scheme is `https` and the
   [`TlsConfig`](crate::tls::TlsConfig) indicate we are using **rustls**.
   This is the default TLS provider.
4. [`NativeTlsConnector`](#nativetlsconnector) which wraps
   the connection from 1 or 2 in TLS if the scheme is `https` and
   [`TlsConfig`](crate::tls::TlsConfig) indicate we are using **native-tls**.


#### Implementations

- `fn new() -> Self`
  Creates a default connector.

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

##### `impl Connector`

- `type Out = Box<dyn Transport>`

- `fn connect(self: &Self, details: &ConnectionDetails<'_>, chained: Option<()>) -> Result<Option<<Self as >::Out>, Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

## Enums

### `Either<A, B>`

```rust
enum Either<A, B> {
    A(A),
    B(B),
}
```

A selection between two transports.

#### Variants

- **`A`**

  The first transport.

- **`B`**

  The second transport.

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

##### `impl Transport<A: Transport, B: Transport>`

- `fn buffers(self: &mut Self) -> &mut dyn super::Buffers`

- `fn transmit_output(self: &mut Self, amount: usize, timeout: super::NextTimeout) -> Result<(), crate::Error>`

- `fn await_input(self: &mut Self, timeout: super::NextTimeout) -> Result<bool, crate::Error>`

- `fn is_open(self: &mut Self) -> bool`

- `fn is_tls(self: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<A: $crate::fmt::Debug, B: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Connector<In: Transport>`

```rust
trait Connector<In: Transport>: Debug + Send + Sync + 'static { ... }
```

Trait for components providing some aspect of connecting.

A connector instance is reused to produce multiple [`Transport`](ureq/unversioned/transport/index.md) instances (where `Transport`
instance would typically be a socket connection).

A connector can be part of a chain of connectors. The [`DefaultConnector`](ureq/unversioned/transport/index.md) provides a chain that
first tries to make a concrete socket connection (using [`TcpConnector`](#tcpconnector)) and then pass the
resulting [`Transport`](ureq/unversioned/transport/index.md) to a TLS wrapping connector
(see [`RustlsConnector`](#rustlsconnector)). This makes it possible combine connectors
in new ways. A user of ureq could implement bespoke connector (such as SCTP) and still use
the `RustlsConnector` to wrap the underlying transport in TLS.

The built-in [`DefaultConnector`](ureq/unversioned/transport/index.md) provides SOCKS, TCP sockets and TLS wrapping.

# Errors

When writing a bespoke connector chain we recommend handling errors like this:

1. Map to [`Error::Io`](#io) as far as possible.
2. Map to any other [`Error`](../../../docs_md/docs_md/error/index.md) where reasonable.
3. Fall back on [`Error::Other`](#other) preserving the original error.
4. As a last resort [`Error::ConnectionFailed`](#connectionfailed) + logging.

# Example

```
# #[cfg(all(feature = "rustls", not(feature = "_test")))] {
use ureq::{Agent, config::Config};

// These types are not covered by the promises of semver (yet)
use ureq::unversioned::transport::{Connector, TcpConnector, RustlsConnector};
use ureq::unversioned::resolver::DefaultResolver;

// A connector chain that opens a TCP transport, then wraps it in a TLS.
let connector = ()
    .chain(TcpConnector::default())
    .chain(RustlsConnector::default());

let config = Config::default();
let resolver = DefaultResolver::default();

// Creates an agent with a bespoke connector
let agent = Agent::with_parts(config, connector, resolver);

let mut res = agent.get("https://httpbin.org/get").call().unwrap();
let body = res.body_mut().read_to_string().unwrap();
# }
```

#### Required Methods

- `type Out: 1`

- `fn connect(self: &Self, details: &ConnectionDetails<'_>, chained: Option<In>) -> Result<Option<<Self as >::Out>, Error>`

  Use this connector to make a [`Transport`](ureq/unversioned/transport/index.md).

- `fn chain<Next: Connector<<Self as >::Out>>(self: Self, next: Next) -> ChainedConnector<In, Self, Next>`

  Chain this connector to another connector.

### `Transport`

```rust
trait Transport: Debug + Send + Sync + 'static { ... }
```

Transport of HTTP/1.1 as created by a [`Connector`](ureq/unversioned/transport/index.md).

In ureq, [`Transport`](ureq/unversioned/transport/index.md) and [`Buffers`](#buffers) go hand in hand. The rest of ureq tries to minimize
the allocations, and the transport is responsible for providing the buffers required
to perform the request. Unless the transport requires special buffer handling, the
[`LazyBuffers`](#lazybuffers) implementation can be used.

For sending data, the order of calls are:

1. [`Transport::buffers()`](#buffers) to obtain the buffers.
2. [`Buffers::output()`](#output) or [`Buffers::tmp_and_output`](#tmp-and-output)
   depending where in the life cycle of the request ureq is.
3. [`Transport::transmit_output()`](#transmit-output) to ask the transport to send/flush the `amount` of
   buffers used in 2.

For receiving data, the order of calls are:

1. [`Transport::maybe_await_input()`](#maybe-await-input)
2. The transport impl itself uses [`Buffers::input_append_buf()`](#input-append-buf) to fill a number
   of bytes from the underlying transport and use [`Buffers::input_appended()`](#input-appended) to
   tell the buffer how much been filled.
3. [`Transport::buffers()`](#buffers) to obtain the buffers
4. [`Buffers::input()`](#input) followed by [`Buffers::input_consume()`](#input-consume). It's important to retain the
   unconsumed bytes for the next call to `maybe_await_input()`. This is handled by [`LazyBuffers`](#lazybuffers).
   It's important to call [`Buffers::input_consume()`](#input-consume) also with 0 consumed bytes since that's
   how we keep track of whether the input is making progress.


#### Required Methods

- `fn buffers(self: &mut Self) -> &mut dyn Buffers`

  Provide buffers for this transport.

- `fn transmit_output(self: &mut Self, amount: usize, timeout: NextTimeout) -> Result<(), Error>`

  Transmit `amount` of the output buffer. ureq will always transmit the entirety

- `fn await_input(self: &mut Self, timeout: NextTimeout) -> Result<bool, Error>`

  Wait for input and fill the buffer.

- `fn is_open(self: &mut Self) -> bool`

  Tell whether this transport is still functional. This must provide an accurate answer

- `fn is_tls(self: &Self) -> bool`

  Whether the transport is TLS.

