*[rustls](../index.md) / [unbuffered](index.md)*

---

# Module `unbuffered`

Unbuffered connection API

This is an alternative to the `crate::ConnectionCommon` API that does not internally buffer
TLS nor plaintext data. Instead those buffers are managed by the API user so they have
control over when and how to allocate, resize and dispose of them.

This API is lower level than the `ConnectionCommon` API and is built around a state machine
interface where the API user must handle each state to advance and complete the
handshake process.

Like the `ConnectionCommon` API, no IO happens internally so all IO must be handled by the API
user. Unlike the `ConnectionCommon` API, this API does not make use of the `std::io::Read` and
`std::io::Write` traits so it's usable in no-std context.

The entry points into this API are `crate::client::UnbufferedClientConnection::new`,
`crate::server::UnbufferedServerConnection::new` and
`unbuffered::UnbufferedConnectionCommon::process_tls_records`. The state machine API is
documented in `unbuffered::ConnectionState`.

# Examples

[`unbuffered-client`](#unbuffered-client) and [`unbuffered-server`](#unbuffered-server) are examples that fully exercise the API in
std, non-async context.



## Structs

### `UnbufferedConnectionCommon<Data>`

```rust
struct UnbufferedConnectionCommon<Data> {
    // [REDACTED: Private Fields]
}
```

Interface shared by unbuffered client and server connections.

#### Implementations

- `fn process_tls_records<'c, 'i>(self: &'c mut Self, incoming_tls: &'i mut [u8]) -> UnbufferedStatus<'c, 'i, ClientConnectionData>`
  Processes the TLS records in `incoming_tls` buffer until a new [`UnbufferedStatus`] is

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

- `fn process_tls_records<'c, 'i>(self: &'c mut Self, incoming_tls: &'i mut [u8]) -> UnbufferedStatus<'c, 'i, ServerConnectionData>`
  Processes the TLS records in `incoming_tls` buffer until a new [`UnbufferedStatus`] is

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

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Deref<T>`

- `type Target = CommonState`

- `fn deref(self: &Self) -> &<Self as >::Target`

### `AppDataRecord<'i>`

```rust
struct AppDataRecord<'i> {
    pub discard: usize,
    pub payload: &'i [u8],
}
```

A decrypted application-data record

#### Fields

- **`discard`**: `usize`

  Number of additional bytes to discard
  
  This number MUST be added to the value of `UnbufferedStatus::discard` *prior* to the
  discard operation. See `UnbufferedStatus::discard` for more details

- **`payload`**: `&'i [u8]`

  The payload of the app-data record

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

### `EncodeTlsData<'c, Data>`

```rust
struct EncodeTlsData<'c, Data> {
    // [REDACTED: Private Fields]
}
```

A handshake record must be encoded

#### Implementations

- `fn encode(self: &mut Self, outgoing_tls: &mut [u8]) -> Result<usize, EncodeError>`
  Encodes a handshake record into the `outgoing_tls` buffer

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

### `InsufficientSizeError`

```rust
struct InsufficientSizeError {
    pub required_size: usize,
}
```

Provided buffer was too small

#### Fields

- **`required_size`**: `usize`

  buffer must be at least this size

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

- `fn clone(self: &Self) -> InsufficientSizeError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

### `ReadEarlyData<'c, 'i, Data>`

```rust
struct ReadEarlyData<'c, 'i, Data> {
    // [REDACTED: Private Fields]
}
```

Early application-data is available.

#### Implementations

- `fn next_record(self: &mut Self) -> Option<Result<AppDataRecord<'_>, Error>>`
  decrypts and returns the next available app-data record

- `fn peek_len(self: &Self) -> Option<NonZeroUsize>`
  returns the payload size of the next app-data record *without* decrypting it

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

### `ReadTraffic<'c, 'i, Data>`

```rust
struct ReadTraffic<'c, 'i, Data> {
    // [REDACTED: Private Fields]
}
```

Application data is available

#### Implementations

- `fn next_record(self: &mut Self) -> Option<Result<AppDataRecord<'_>, Error>>`
  Decrypts and returns the next available app-data record

- `fn peek_len(self: &Self) -> Option<NonZeroUsize>`
  Returns the payload size of the next app-data record *without* decrypting it

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

### `TransmitTlsData<'c, Data>`

```rust
struct TransmitTlsData<'c, Data> {
    // [REDACTED: Private Fields]
}
```

Previously encoded TLS data must be transmitted

#### Implementations

- `fn may_encrypt_early_data(self: &mut Self) -> Option<MayEncryptEarlyData<'_>>`
  returns an adapter that allows encrypting early (RTT-0) data before transmitting the

- `fn done(self: Self)`
  Signals that the previously encoded TLS data has been transmitted

- `fn may_encrypt_app_data(self: &mut Self) -> Option<WriteTraffic<'_, Data>>`
  Returns an adapter that allows encrypting application data

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

### `UnbufferedStatus<'c, 'i, Data>`

```rust
struct UnbufferedStatus<'c, 'i, Data> {
    pub discard: usize,
    pub state: Result<ConnectionState<'c, 'i, Data>, crate::Error>,
}
```

The current status of the `UnbufferedConnection*`

#### Fields

- **`discard`**: `usize`

  Number of bytes to discard
  
  After the `state` field of this object has been handled, `discard` bytes must be
  removed from the *front* of the `incoming_tls` buffer that was passed to
  the `UnbufferedConnectionCommon::process_tls_records` call that returned this object.
  
  This discard operation MUST happen *before*
  `UnbufferedConnectionCommon::process_tls_records` is called again.

- **`state`**: `Result<ConnectionState<'c, 'i, Data>, crate::Error>`

  The current state of the handshake process
  
  This value MUST be handled prior to calling
  `UnbufferedConnectionCommon::process_tls_records` again. See the documentation on the
  variants of [`ConnectionState`](../index.md) for more details.

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

##### `impl Debug<'c, 'i, Data: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `WriteTraffic<'c, Data>`

```rust
struct WriteTraffic<'c, Data> {
    // [REDACTED: Private Fields]
}
```

Allows encrypting app-data

#### Implementations

- `fn encrypt(self: &mut Self, application_data: &[u8], outgoing_tls: &mut [u8]) -> Result<usize, EncryptError>`
  Encrypts `application_data` into the `outgoing_tls` buffer

- `fn queue_close_notify(self: &mut Self, outgoing_tls: &mut [u8]) -> Result<usize, EncryptError>`
  Encrypts a close_notify warning alert in `outgoing_tls`

- `fn refresh_traffic_keys(self: Self) -> Result<(), Error>`
  Arranges for a TLS1.3 `key_update` to be sent.

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

## Enums

### `ConnectionState<'c, 'i, Data>`

```rust
enum ConnectionState<'c, 'i, Data> {
    ReadTraffic(ReadTraffic<'c, 'i, Data>),
    PeerClosed,
    Closed,
    ReadEarlyData(ReadEarlyData<'c, 'i, Data>),
    EncodeTlsData(EncodeTlsData<'c, Data>),
    TransmitTlsData(TransmitTlsData<'c, Data>),
    BlockedHandshake,
    WriteTraffic(WriteTraffic<'c, Data>),
}
```

The state of the [`UnbufferedConnectionCommon`](../index.md) object

#### Variants

- **`ReadTraffic`**

  One, or more, application data records are available
  
  See [`ReadTraffic`](../index.md) for more details on how to use the enclosed object to access
  the received data.

- **`PeerClosed`**

  Connection has been cleanly closed by the peer.
  
  This state is encountered at most once by each connection -- it is
  "edge" triggered, rather than "level" triggered.
  
  It delimits the data received from the peer, meaning you can be sure you
  have received all the data the peer sent.
  
  No further application data will be received from the peer, so no further
  `ReadTraffic` states will be produced.
  
  However, it is possible to _send_ further application data via `WriteTraffic`
  states, or close the connection cleanly by calling
  `WriteTraffic::queue_close_notify()`.

- **`Closed`**

  Connection has been cleanly closed by both us and the peer.
  
  This is a terminal state.  No other states will be produced for this
  connection.

- **`ReadEarlyData`**

  One, or more, early (RTT-0) data records are available

- **`EncodeTlsData`**

  A Handshake record is ready for encoding
  
  Call `EncodeTlsData::encode` on the enclosed object, providing an `outgoing_tls`
  buffer to store the encoding

- **`TransmitTlsData`**

  Previously encoded handshake records need to be transmitted
  
  Transmit the contents of the `outgoing_tls` buffer that was passed to previous
  `EncodeTlsData::encode` calls to the peer.
  
  After transmitting the contents, call `TransmitTlsData::done` on the enclosed object.
  The transmitted contents MUST not be sent to the peer more than once so they SHOULD be
  discarded at this point.
  
  At some stages of the handshake process, it's possible to send application-data alongside
  handshake records. Call `TransmitTlsData::may_encrypt_app_data` on the enclosed
  object to probe if that's allowed.

- **`BlockedHandshake`**

  More TLS data is needed to continue with the handshake
  
  Request more data from the peer and append the contents to the `incoming_tls` buffer that
  was passed to `UnbufferedConnectionCommon::process_tls_records`.

- **`WriteTraffic`**

  The handshake process has been completed.
  
  `WriteTraffic::encrypt` can be called on the enclosed object to encrypt application
  data into an `outgoing_tls` buffer. Similarly, `WriteTraffic::queue_close_notify` can
  be used to encrypt a close_notify alert message into a buffer to signal the peer that the
  connection is being closed. Data written into `outgoing_buffer` by either method MAY be
  transmitted to the peer during this state.
  
  Once this state has been reached, data MAY be requested from the peer and appended to an
  `incoming_tls` buffer that will be passed to a future
  `UnbufferedConnectionCommon::process_tls_records` invocation. When enough data has been
  appended to `incoming_tls`, `UnbufferedConnectionCommon::process_tls_records` will yield
  the `ConnectionState::ReadTraffic` state.

#### Trait Implementations

##### `impl From<'c, Data>`

- `fn from(v: TransmitTlsData<'c, Data>) -> Self`

##### `impl From<'c, Data>`

- `fn from(v: EncodeTlsData<'c, Data>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'c, 'i, Data>`

- `fn from(v: ReadTraffic<'c, 'i, Data>) -> Self`

##### `impl From<'c, 'i, Data>`

- `fn from(v: ReadEarlyData<'c, 'i, Data>) -> Self`

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

##### `impl Debug<Data>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EncodeError`

```rust
enum EncodeError {
    InsufficientSize(InsufficientSizeError),
    AlreadyEncoded,
}
```

Errors that may arise when encoding a handshake record

#### Variants

- **`InsufficientSize`**

  Provided buffer was too small

- **`AlreadyEncoded`**

  The handshake record has already been encoded; do not call `encode` again

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(v: InsufficientSizeError) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `EncryptError`

```rust
enum EncryptError {
    InsufficientSize(InsufficientSizeError),
    EncryptExhausted,
}
```

Errors that may arise when encrypting application data

#### Variants

- **`InsufficientSize`**

  Provided buffer was too small

- **`EncryptExhausted`**

  Encrypter has been exhausted

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(v: InsufficientSizeError) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

