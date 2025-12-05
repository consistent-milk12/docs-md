*[ureq_proto](../index.md) / [client](index.md)*

---

# Module `client`

HTTP/1.1 client protocol

Sans-IO protocol impl, which means "writing" and "reading" are made via buffers
rather than the Write/Read std traits.

The [`Call`](#call) object attempts to encode correct HTTP/1.1 handling using
state variables, for example `Call<'a, SendRequest>` to represent the
lifecycle stage where we are to send the request.

The states are:

* **Prepare** - Preparing a request means 1) adding headers such as
  cookies. 2) acquiring the connection from a pool or opening a new
  socket (potentially wrappping in TLS)
* **SendRequest** - Send the first row, which is the method, path
  and version as well as the request headers
* **SendBody** - Send the request body
* **Await100** - If there is an `Expect: 100-continue` header, the
  client should pause before sending the body
* **RecvResponse** - Receive the response, meaning the status and
  version and the response headers
* **RecvBody** - Receive the response body
* **Redirect** - Handle redirects, potentially spawning new requests
* **Cleanup** - Return the connection to the pool or close it


```text
                           ┌──────────────────┐
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ▶│     Prepare      │
                           └──────────────────┘
│                                    │
                                     ▼
│                          ┌──────────────────┐
                        ┌──│   SendRequest    │──────────────┐
│                       │  └──────────────────┘              │
                        │            │                       │
│                       │            ▼                       ▼
                        │  ┌──────────────────┐    ┌──────────────────┐
│                       │  │     SendBody     │◀───│     Await100     │
                        │  └──────────────────┘    └──────────────────┘
│                       │            │                       │
                        │            ▼                       │
│                       └─▶┌──────────────────┐◀─────────────┘
             ┌─────────────│   RecvResponse   │──┐
│            │             └──────────────────┘  │
             │                       │           │
│            ▼                       ▼           │
   ┌──────────────────┐    ┌──────────────────┐  │
└ ─│     Redirect     │◀───│     RecvBody     │  │
   └──────────────────┘    └──────────────────┘  │
             │                       │           │
             │                       ▼           │
             │             ┌──────────────────┐  │
             └────────────▶│     Cleanup      │◀─┘
                           └──────────────────┘
```

# Example

```rust
use ureq_proto::client::*;
use ureq_proto::http::Request;

let request = Request::put("https://example.test/my-path")
    .header("Expect", "100-continue")
    .header("x-foo", "bar")
    .body(())
    .unwrap();

// ********************************** Prepare

let mut call = Call::new(request).unwrap();

// Prepare with state from cookie jar. The uri
// is used to key the cookies.
let uri = call.uri();

// call.header("Cookie", "my_cookie1=value1");
// call.header("Cookie", "my_cookie2=value2");

// Obtain a connection for the uri, either a
// pooled connection from a previous http/1.1
// keep-alive, or open a new. The connection
// must be TLS wrapped if the scheme so indicate.
// let connection = todo!();

// Sans-IO means it does not use any
// Write trait or similar. Requests and request
// bodies are written to a buffer that in turn
// should be sent via the connection.
let mut output = vec![0_u8; 1024];

// ********************************** SendRequest

// Proceed to the next state writing the request.
let mut call = call.proceed();

let output_used = call.write(&mut output).unwrap();
assert_eq!(output_used, 107);

assert_eq!(&output[..output_used], b"\
    PUT /my-path HTTP/1.1\r\n\
    host: example.test\r\n\
    transfer-encoding: chunked\r\n\
    expect: 100-continue\r\n\
    x-foo: bar\r\n\
    \r\n");

// Check we can continue to send the body
assert!(call.can_proceed());

// ********************************** Await100

// In this example, we know the next state is Await100.
// A real client needs to match on the variants.
let mut call = match call.proceed() {
    Ok(Some(SendRequestResult::Await100(v))) => v,
    _ => panic!(),
};

// When awaiting 100, the client should run a timer and
// proceed to sending the body either when the server
// indicates it can receive the body, or the timer runs out.

// This boolean can be checked whether there's any point
// in keeping waiting for the timer to run out.
assert!(call.can_keep_await_100());

let input = b"HTTP/1.1 100 Continue\r\n\r\n";
let input_used = call.try_read_100(input).unwrap();

assert_eq!(input_used, 25);
assert!(!call.can_keep_await_100());

// ********************************** SendBody

// Proceeding is possible regardless of whether the
// can_keep_await_100() is true or false.
// A real client needs to match on the variants.
let mut call = match call.proceed() {
    Ok(Await100Result::SendBody(v)) => v,
    _ => panic!(),
};

let (input_used, o1) =
    call.write(b"hello", &mut output).unwrap();

assert_eq!(input_used, 5);

// When doing transfer-encoding: chunked,
// the end of body must be signaled with
// an empty input. This is also valid for
// regular content-length body.
assert!(!call.can_proceed());

let (_, o2) = call.write(&[], &mut output[o1..]).unwrap();

let output_used = o1 + o2;
assert_eq!(output_used, 15);

assert_eq!(&output[..output_used], b"\
    5\r\n\
    hello\
    \r\n\
    0\r\n\
    \r\n");

assert!(call.can_proceed());

// ********************************** RecvRequest

// Proceed to read the request.
let mut call = call.proceed().unwrap();

let part = b"HTTP/1.1 200 OK\r\nContent-Len";
let full = b"HTTP/1.1 200 OK\r\nContent-Length: 9\r\n\r\n";

// try_response can be used repeatedly until we
// get enough content including all headers.
let (input_used, maybe_response) =
    call.try_response(part, false).unwrap();

assert_eq!(input_used, 0);
assert!(maybe_response.is_none());

let (input_used, maybe_response) =
    call.try_response(full, false).unwrap();

assert_eq!(input_used, 38);
let response = maybe_response.unwrap();

// ********************************** RecvBody

// It's not possible to proceed until we
// have read a response.
let mut call = match call.proceed() {
    Some(RecvResponseResult::RecvBody(v)) => v,
    _ => panic!(),
};

let(input_used, output_used) =
    call.read(b"hi there!", &mut output).unwrap();

assert_eq!(input_used, 9);
assert_eq!(output_used, 9);

assert_eq!(&output[..output_used], b"hi there!");

// ********************************** Cleanup

let call = match call.proceed() {
    Some(RecvBodyResult::Cleanup(v)) => v,
    _ => panic!(),
};

if call.must_close_connection() {
    // connection.close();
} else {
    // connection.return_to_pool();
}

```

## Modules

- [`state`](state/index.md) - State types for the Call state machine.

## Structs

### `Call<State>`

```rust
struct Call<State> {
    // [REDACTED: Private Fields]
}
```

A state machine for an HTTP request/response cycle.

This type represents a state machine that transitions through various
states during the lifecycle of an HTTP request/response.

The type parameters are:
- `State`: The current state of the state machine (e.g., `Prepare`, `SendRequest`, etc.)

See the `state graph` in the client module documentation for a
visual representation of the state transitions.

#### Implementations

- `fn write(self: &mut Self, input: &[u8], output: &mut [u8]) -> Result<(usize, usize), Error>`
  Write request body from `input` to `output`.

- `fn consume_direct_write(self: &mut Self, amount: usize) -> Result<(), Error>`
  Helper to avoid copying memory.

- `fn calculate_max_input(self: &Self, output_len: usize) -> usize`
  Calculate the max amount of input we can transfer to fill the `output_len`.

- `fn is_chunked(self: &Self) -> bool`
  Test if call is chunked.

- `fn can_proceed(self: &Self) -> bool`
  Check whether the request body is fully sent.

- `fn proceed(self: Self) -> Option<Call<RecvResponse>>`
  Proceed to the next state.

- `fn write(self: &mut Self, output: &mut [u8]) -> Result<usize, Error>`
  Write the request to the buffer.

- `fn method(self: &Self) -> &Method`
  The configured method.

- `fn uri(self: &Self) -> &Uri`
  The uri being requested.

- `fn version(self: &Self) -> Version`
  Version of the request.

- `fn headers_map(self: &mut Self) -> Result<HeaderMap, Error>`
  The configured headers.

- `fn can_proceed(self: &Self) -> bool`
  Check whether the entire request has been sent.

- `fn proceed(self: Self) -> Result<Option<SendRequestResult>, Error>`
  Attempt to proceed from this state to the next.

- `fn as_new_call(self: &mut Self, redirect_auth_headers: RedirectAuthHeaders) -> Result<Option<Call<Prepare>>, Error>`
  Construct a new `Call` by following the redirect.

- `fn status(self: &Self) -> StatusCode`
  The redirect status code.

- `fn must_close_connection(self: &Self) -> bool`
  Whether we must close the connection corresponding to the current call.

- `fn close_reason(self: &Self) -> Option<&'static str>`
  If we are closing the connection, give a reason why.

- `fn proceed(self: Self) -> Call<Cleanup>`
  Proceed to the cleanup state.

- `fn try_response(self: &mut Self, input: &[u8], allow_partial_redirect: bool) -> Result<(usize, Option<Response<()>>), Error>`
  Try reading a response from the input.

- `fn can_proceed(self: &Self) -> bool`
  Tell if we have finished receiving the response.

- `fn proceed(self: Self) -> Option<RecvResponseResult>`
  Proceed to the next state.

- `fn force_recv_body(self: &mut Self)`
  Convert the state to receive a body despite method.

- `fn try_read_100(self: &mut Self, input: &[u8]) -> Result<usize, Error>`
  Attempt to read a 100-continue response.

- `fn can_keep_await_100(self: &Self) -> bool`
  Tell if there is any point in waiting for more data from the server.

- `fn proceed(self: Self) -> Result<Await100Result, Error>`
  Proceed to the next state.

- `fn must_close_connection(self: &Self) -> bool`
  Tell if we must close the connection.

- `fn close_reason(self: &Self) -> Option<&'static str>`
  If we are closing the connection, give a reason.

- `fn new(request: Request<()>) -> Result<Self, Error>`
  Create a new Call instance from an HTTP request.

- `fn method(self: &Self) -> &Method`
  Inspect call method

- `fn uri(self: &Self) -> &Uri`
  Inspect call URI

- `fn version(self: &Self) -> Version`
  Inspect call HTTP version

- `fn headers(self: &Self) -> &HeaderMap`
  Inspect call headers

- `fn allow_non_standard_methods(self: &mut Self, v: bool)`
  Set whether to allow non-standard HTTP methods.

- `fn header<K, V>(self: &mut Self, key: K, value: V) -> Result<(), Error>`
  Add more headers to the call

- `fn force_send_body(self: &mut Self)`
  Convert the state to send body despite method.

- `fn proceed(self: Self) -> Call<SendRequest>`
  Continue to the next call state.

- `fn read(self: &mut Self, input: &[u8], output: &mut [u8]) -> Result<(usize, usize), Error>`
  Read the response body from `input` to `output`.

- `fn stop_on_chunk_boundary(self: &mut Self, enabled: bool)`
  Set if we are stopping on chunk boundaries.

- `fn is_on_chunk_boundary(self: &Self) -> bool`
  Tell if the reading is on a chunk boundary.

- `fn body_mode(self: &Self) -> BodyMode`
  Tell which kind of mode the response body is.

- `fn can_proceed(self: &Self) -> bool`
  Check if the response body has been fully received.

- `fn is_ended_chunked(self: &Self) -> bool`
  Tell if we got an end chunk when reading chunked.

- `fn proceed(self: Self) -> Option<RecvBodyResult>`
  Proceed to the next state.

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

##### `impl Debug<State: Named>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `SendRequestResult`

```rust
enum SendRequestResult {
    Await100(Call<Await100>),
    SendBody(Call<SendBody>),
    RecvResponse(Call<RecvResponse>),
}
```

Possible state transitions after sending a request.

After sending the request headers, the call can transition to one of three states:
- `Await100`: If the request included an `Expect: 100-continue` header
- `SendBody`: If the request has a body to send
- `RecvResponse`: If the request has no body (e.g., GET, HEAD)

See the `state graph` for a visual representation.

#### Variants

- **`Await100`**

  Expect-100/Continue mechanic.

- **`SendBody`**

  Send the request body.

- **`RecvResponse`**

  Receive the response.

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

### `Await100Result`

```rust
enum Await100Result {
    SendBody(Call<SendBody>),
    RecvResponse(Call<RecvResponse>),
}
```

Possible state transitions after awaiting a 100 Continue response.

After awaiting a 100 Continue response, the call can transition to one of two states:
- `SendBody`: If the server sent a 100 Continue response or the timeout expired
- `RecvResponse`: If the server sent a different response

See the `state graph` for a visual representation.

#### Variants

- **`SendBody`**

  Send the request body.

- **`RecvResponse`**

  Receive server response.

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

### `RecvResponseResult`

```rust
enum RecvResponseResult {
    RecvBody(Call<RecvBody>),
    Redirect(Call<Redirect>),
    Cleanup(Call<Cleanup>),
}
```

Possible state transitions after receiving a response.

After receiving a response (status and headers), the call can transition to one of three states:
- `RecvBody`: If the response has a body to receive
- `Redirect`: If the response is a redirect
- `Cleanup`: If the response has no body and is not a redirect

See the `state graph` for a visual representation.

#### Variants

- **`RecvBody`**

  Receive a response body.

- **`Redirect`**

  Follow a redirect.

- **`Cleanup`**

  Run cleanup.

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

### `RecvBodyResult`

```rust
enum RecvBodyResult {
    Redirect(Call<Redirect>),
    Cleanup(Call<Cleanup>),
}
```

Possible state transitions after receiving a response body.

After receiving a response body, the call can transition to one of two states:
- `Redirect`: If the response is a redirect
- `Cleanup`: If the response is not a redirect

See the `state graph` for a visual representation.

#### Variants

- **`Redirect`**

  Follow a redirect

- **`Cleanup`**

  Go to cleanup

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

### `RedirectAuthHeaders`

```rust
enum RedirectAuthHeaders {
    Never,
    SameHost,
}
```

Strategy for preserving authorization headers during redirects.

This enum defines how authorization headers should be handled when following
redirects:

* `Never`: Never preserve the `authorization` header in redirects. This is the default.
* `SameHost`: Preserve the `authorization` header when the redirect is to the same host
  and uses the same scheme (or switches to a more secure one, i.e., from HTTP to HTTPS,
  but not the reverse).

#### Variants

- **`Never`**

  Never preserve the `authorization` header on redirect. This is the default.

- **`SameHost`**

  Preserve the `authorization` header when the redirect is to the same host. Both hosts must use
  the same scheme (or switch to a more secure one, i.e we can redirect from `http` to `https`,
  but not the reverse).

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

- `fn clone(self: &Self) -> RedirectAuthHeaders`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RedirectAuthHeaders) -> bool`

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

## Constants

### `MAX_RESPONSE_HEADERS`

```rust
const MAX_RESPONSE_HEADERS: usize = 128usize;
```

Max number of headers to parse from an HTTP response

