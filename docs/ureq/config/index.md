*[ureq](../index.md) / [config](index.md)*

---

# Module `config`

Agent configuration

## Structs

### `Config`

```rust
struct Config {
    // [REDACTED: Private Fields]
}
```

Config primarily for the [`Agent`](index.md), but also per-request.

Config objects are cheap to clone and should not incur any heap allocations.

# Agent level config

## Example

```
use ureq::Agent;
use std::time::Duration;

let config = Agent::config_builder()
    .timeout_global(Some(Duration::from_secs(10)))
    .https_only(true)
    .build();

let agent = Agent::new_with_config(config);
```


# Request level config

The config can also be change per-request. Since every request ultimately executes
using an [`Agent`](index.md) (also the root-level `ureq::get(...)` have an implicit agent),
a request level config clones the agent level config.

There are two ways of getting a request level config.

## Request builder example

The first way is via `RequestBuilder::config()`.

```
use ureq::Agent;

let agent: Agent = Agent::config_builder()
    .https_only(false)
    .build()
    .into();

let response = agent.get("http://httpbin.org/get")
    .config()
    // override agent level setting for this request
    .https_only(true)
    .build()
    .call();
```

## HTTP request example

The second way is via `Agent::configure_request()`.
This is used when working with the http crate `http::Request` type directly.

```
use ureq::{http, Agent};

let agent: Agent = Agent::config_builder()
    .https_only(false)
    .build()
    .into();

let request = http::Request::get("http://httpbin.org/get")
    .body(()).unwrap();

let request = agent.configure_request(request)
    // override agent level setting for this request
    .https_only(true)
    .build();

let response = agent.run(request);
```


#### Implementations

- `fn http_status_as_error(self: &Self) -> bool`
  Whether to treat 4xx and 5xx HTTP status codes as

- `fn https_only(self: &Self) -> bool`
  Whether to limit requests (including redirects) to https only

- `fn ip_family(self: &Self) -> IpFamily`
  Configuration of IPv4/IPv6.

- `fn tls_config(self: &Self) -> &TlsConfig`
  Config for TLS.

- `fn proxy(self: &Self) -> Option<&Proxy>`
  Proxy configuration.

- `fn no_delay(self: &Self) -> bool`
  Disable Nagle's algorithm

- `fn max_redirects(self: &Self) -> u32`
  The max number of redirects to follow before giving up.

- `fn max_redirects_will_error(self: &Self) -> bool`
  If we should error when max redirects are reached.

- `fn redirect_auth_headers(self: &Self) -> RedirectAuthHeaders`
  How to handle `Authorization` headers when following redirects

- `fn save_redirect_history(self: &Self) -> bool`
  If we should record a history of every redirect location,

- `fn user_agent(self: &Self) -> &AutoHeaderValue`
  Value to use for the `User-Agent` header.

- `fn accept(self: &Self) -> &AutoHeaderValue`
  Value to use for the `Accept` header.

- `fn accept_encoding(self: &Self) -> &AutoHeaderValue`
  Value to use for the `Accept-Encoding` header.

- `fn timeouts(self: &Self) -> Timeouts`
  All configured timeouts.

- `fn max_response_header_size(self: &Self) -> usize`
  Max size of the HTTP response header.

- `fn input_buffer_size(self: &Self) -> usize`
  Default size of the input buffer

- `fn output_buffer_size(self: &Self) -> usize`
  Default size of the output buffer.

- `fn max_idle_connections(self: &Self) -> usize`
  Max number of idle pooled connections overall.

- `fn max_idle_connections_per_host(self: &Self) -> usize`
  Max number of idle pooled connections per host/port combo.

- `fn max_idle_age(self: &Self) -> Duration`
  Max duration to keep an idle connection in the pool

- `fn allow_non_standard_methods(self: &Self) -> bool`
  Whether to allow non-standard HTTP methods.

- `fn builder() -> ConfigBuilder<AgentScope>`
  A builder to make a bespoke configuration.

- `fn new_agent(self: &Self) -> Agent`
  Creates a new agent by cloning this config.

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

- `fn clone(self: &Self) -> Config`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Default`

- `fn default() -> Self`

### `ConfigBuilder<Scope: private::ConfigScope>`

```rust
struct ConfigBuilder<Scope: private::ConfigScope>();
```

Builder of [`Config`](config/index.md)

#### Implementations

- `fn build(self: Self) -> Config`
  Finalize the config

- `fn build(self: Self) -> RequestBuilder<Any>`
  Finalize the config

- `fn build(self: Self) -> WithAgent<'a, S>`
  Finalize the config

- `fn run(self: Self) -> Result<Response<Body>, Error>`
  Run the request with the agent in the ConfigBuilder

- `fn http_status_as_error(self: Self, v: bool) -> Self`
  Whether to treat 4xx and 5xx HTTP status codes as

- `fn https_only(self: Self, v: bool) -> Self`
  Whether to limit requests (including redirects) to https only

- `fn ip_family(self: Self, v: IpFamily) -> Self`
  Configuration of IPv4/IPv6.

- `fn tls_config(self: Self, v: TlsConfig) -> Self`
  Config for TLS.

- `fn proxy(self: Self, v: Option<Proxy>) -> Self`
  Proxy configuration.

- `fn no_delay(self: Self, v: bool) -> Self`
  Disable Nagle's algorithm

- `fn max_redirects(self: Self, v: u32) -> Self`
  The max number of redirects to follow before giving up.

- `fn max_redirects_will_error(self: Self, v: bool) -> Self`
  If we should error when max redirects are reached.

- `fn redirect_auth_headers(self: Self, v: RedirectAuthHeaders) -> Self`
  How to handle `Authorization` headers when following redirects

- `fn save_redirect_history(self: Self, v: bool) -> Self`
  If we should record a history of every redirect location,

- `fn user_agent(self: Self, v: impl Into<AutoHeaderValue>) -> Self`
  Value to use for the `User-Agent` header.

- `fn accept(self: Self, v: impl Into<AutoHeaderValue>) -> Self`
  Value to use for the `Accept` header.

- `fn accept_encoding(self: Self, v: impl Into<AutoHeaderValue>) -> Self`
  Value to use for the `Accept-Encoding` header.

- `fn max_response_header_size(self: Self, v: usize) -> Self`
  Max size of the HTTP response header.

- `fn input_buffer_size(self: Self, v: usize) -> Self`
  Default size of the input buffer

- `fn output_buffer_size(self: Self, v: usize) -> Self`
  Default size of the output buffer.

- `fn max_idle_connections(self: Self, v: usize) -> Self`
  Max number of idle pooled connections overall.

- `fn max_idle_connections_per_host(self: Self, v: usize) -> Self`
  Max number of idle pooled connections per host/port combo.

- `fn max_idle_age(self: Self, v: Duration) -> Self`
  Max duration to keep an idle connection in the pool

- `fn allow_non_standard_methods(self: Self, v: bool) -> Self`
  Whether to allow non-standard HTTP methods.

- `fn middleware(self: Self, v: impl Middleware) -> Self`
  Add middleware to use for each request in this agent.

- `fn timeout_global(self: Self, v: Option<Duration>) -> Self`
  Timeout for the entire call

- `fn timeout_per_call(self: Self, v: Option<Duration>) -> Self`
  Timeout for call-by-call when following redirects

- `fn timeout_resolve(self: Self, v: Option<Duration>) -> Self`
  Max duration for doing the DNS lookup when establishing the connection

- `fn timeout_connect(self: Self, v: Option<Duration>) -> Self`
  Max duration for establishing the connection

- `fn timeout_send_request(self: Self, v: Option<Duration>) -> Self`
  Max duration for sending the request, but not the request body.

- `fn timeout_await_100(self: Self, v: Option<Duration>) -> Self`
  Max duration for awaiting a 100-continue response.

- `fn timeout_send_body(self: Self, v: Option<Duration>) -> Self`
  Max duration for sending a request body (if there is one)

- `fn timeout_recv_response(self: Self, v: Option<Duration>) -> Self`
  Max duration for receiving the response headers, but not the body

- `fn timeout_recv_body(self: Self, v: Option<Duration>) -> Self`
  Max duration for receving the response body.

- `fn build(self: Self) -> http::Request<S>`
  Finalize the config

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

### `Timeouts`

```rust
struct Timeouts {
    pub global: Option<std::time::Duration>,
    pub per_call: Option<std::time::Duration>,
    pub resolve: Option<std::time::Duration>,
    pub connect: Option<std::time::Duration>,
    pub send_request: Option<std::time::Duration>,
    pub await_100: Option<std::time::Duration>,
    pub send_body: Option<std::time::Duration>,
    pub recv_response: Option<std::time::Duration>,
    pub recv_body: Option<std::time::Duration>,
}
```

Request timeout configuration.

This can be configured both on Agent level as well as per request.

#### Fields

- **`global`**: `Option<std::time::Duration>`

  Timeout for the entire call

- **`per_call`**: `Option<std::time::Duration>`

  Timeout for call-by-call when following redirects

- **`resolve`**: `Option<std::time::Duration>`

  Max duration for doing the DNS lookup when establishing the connection

- **`connect`**: `Option<std::time::Duration>`

  Max duration for establishing the connection

- **`send_request`**: `Option<std::time::Duration>`

  Max duration for sending the request, but not the request body.

- **`await_100`**: `Option<std::time::Duration>`

  Max duration for awaiting a 100-continue response.

- **`send_body`**: `Option<std::time::Duration>`

  Max duration for sending a request body (if there is one)

- **`recv_response`**: `Option<std::time::Duration>`

  Max duration for receiving the response headers, but not the body

- **`recv_body`**: `Option<std::time::Duration>`

  Max duration for receving the response body.

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

- `fn clone(self: &Self) -> Timeouts`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Self`

## Enums

### `AutoHeaderValue`

```rust
enum AutoHeaderValue {
    None,
    Default,
    Provided(std::sync::Arc<String>),
}
```

Possible config values for headers.

* `None` no automatic header
* `Default` default behavior. I.e. for user-agent something like `ureq/3.1.2`
* `Provided` is a user provided header

#### Variants

- **`None`**

  No automatic header.

- **`Default`**

  Default behavior.
  
  I.e. for user-agent something like `ureq/3.1.2`.

- **`Provided`**

  User provided header value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<S: AsRef<str>>`

- `fn from(value: S) -> Self`

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

- `fn clone(self: &Self) -> AutoHeaderValue`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Default`

- `fn default() -> AutoHeaderValue`

### `IpFamily`

```rust
enum IpFamily {
    Any,
    Ipv4Only,
    Ipv6Only,
}
```

Configuration of IP family to use.

Used to limit the IP to either IPv4, IPv6 or any.

#### Variants

- **`Any`**

  Both Ipv4 and Ipv6

- **`Ipv4Only`**

  Just Ipv4

- **`Ipv6Only`**

  Just Ipv6

#### Implementations

- `fn keep_wanted<'a>(self: &'a Self, iter: impl Iterator<Item = SocketAddr> + 'a) -> impl Iterator<Item = SocketAddr> + 'a`
  Filter the socket addresses to the family of IP.

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

- `fn clone(self: &Self) -> IpFamily`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &IpFamily) -> bool`

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

