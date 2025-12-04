# Crate `ureq`

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/ureq">
    <img src="https://img.shields.io/crates/v/ureq.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Docs -->
  <a href="https://docs.rs/ureq">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/ureq">
    <img src="https://img.shields.io/crates/d/ureq.svg?style=flat-square"
      alt="Crates.io downloads" />
  </a>
</div>

 A simple, safe HTTP client.

 Ureq's first priority is being easy for you to use. It's great for
 anyone who wants a low-overhead HTTP client that just gets the job done. Works
 very well with HTTP APIs. Its features include cookies, JSON, HTTP proxies,
 HTTPS, charset decoding, and is based on the API of the `http` crate.

 Ureq is in pure Rust for safety and ease of understanding. It avoids using
 `unsafe` directly. It uses blocking I/O instead of async I/O, because that keeps
 the API simple and keeps dependencies to a minimum. For TLS, ureq uses
 rustls or native-tls.

 See the [changelog](#changelog)
 for details of recent releases.

 [changelog](#changelog)
: https://github.com/algesten/ureq/blob/main/CHANGELOG.md

 # Usage

 In its simplest form, ureq looks like this:

 ```rust
 let body: String = ureq::get("http://example.com")
     .header("Example-Header", "header value")
     .call()?
     .body_mut()
     .read_to_string()?;
 # Ok::<(), ureq::Error>(())
 ```

 For more involved tasks, you'll want to create an [`Agent`](#agent). An Agent
 holds a connection pool for reuse, and a cookie store if you use the
 **cookies** feature. An Agent can be cheaply cloned due to internal
 [`Arc`](#arc) and all clones of an Agent share state among each other. Creating
 an Agent also allows setting options like the TLS configuration.

 ```rust
 # fn no_run() -> Result<(), ureq::Error> {
 use ureq::Agent;
 use std::time::Duration;

 let mut config = Agent::config_builder()
     .timeout_global(Some(Duration::from_secs(5)))
     .build();

 let agent: Agent = config.into();

 let body: String = agent.get("http://example.com/page")
     .call()?
     .body_mut()
     .read_to_string()?;

 // Reuses the connection from previous request.
 let response: String = agent.put("http://example.com/upload")
     .header("Authorization", "example-token")
     .send("some body data")?
     .body_mut()
     .read_to_string()?;
 # Ok(())}
 ```

 ## JSON

 Ureq supports sending and receiving json, if you enable the **json** feature:

 ```rust
 # #[cfg(feature = "json")]
 # fn no_run() -> Result<(), ureq::Error> {
 use serde::{Serialize, Deserialize};

 #[derive(Serialize)]
 struct MySendBody {
    thing: String,
 }

 #[derive(Deserialize)]
 struct MyRecvBody {
    other: String,
 }

 let send_body = MySendBody { thing: "yo".to_string() };

 // Requires the `json` feature enabled.
 let recv_body = ureq::post("http://example.com/post/ingest")
     .header("X-My-Header", "Secret")
     .send_json(&send_body)?
     .body_mut()
     .read_json::<MyRecvBody>()?;
 # Ok(())}
 ```

 ## Error handling

 ureq returns errors via `Result<T, ureq::Error>`. That includes I/O errors,
 protocol errors. By default, also HTTP status code errors (when the
 server responded 4xx or 5xx) results in [`Error`](#error).

 This behavior can be turned off via [`http_status_as_error()`](#http-status-as-error)

 ```rust
 use ureq::Error;

 # fn no_run() -> Result<(), ureq::Error> {
 match ureq::get("http://mypage.example.com/").call() {
     Ok(response) => { /* it worked */},
     Err(Error::StatusCode(code)) => {
         /* the server returned an unexpected status
            code (such as 400, 500 etc) */
     }
     Err(_) => { /* some kind of io/transport/etc error */ }
 }
 # Ok(())}
 ```

 # Features

 To enable a minimal dependency tree, some features are off by default.
 You can control them when including ureq as a dependency.

 `ureq = { version = "3", features = ["socks-proxy", "charset"] }`

 The default enabled features are: **rustls** and **gzip**.

 * **rustls** enables the rustls TLS implementation. This is the default for the the crate level
   convenience calls (`ureq::get` etc). It currently uses `ring` as the TLS provider.
 * **native-tls** enables the native tls backend for TLS. Due to the risk of diamond dependencies
   accidentally switching on an unwanted TLS implementation, `native-tls` is never picked up as
   a default or used by the crate level convenience calls (`ureq::get` etc) – it must be configured
   on the agent
 * **platform-verifier** enables verifying the server certificates using a method native to the
   platform ureq is executing on. See [rustls-platform-verifier] crate
 * **socks-proxy** enables proxy config using the `socks4://`, `socks4a://`, `socks5://`
   and `socks://` (equal to `socks5://`) prefix
 * **cookies** enables cookies
 * **gzip** enables requests of gzip-compressed responses and decompresses them
 * **brotli** enables requests brotli-compressed responses and decompresses them
 * **charset** enables interpreting the charset part of the Content-Type header
   (e.g.  `Content-Type: text/plain; charset=iso-8859-1`). Without this, the
   library defaults to Rust's built in `utf-8`
 * **json** enables JSON sending and receiving via serde_json
 * **multipart** enables multipart/form-data sending via `unversioned::multipart`

 ### Unstable

 These features are unstable and might change in a minor version.

 * **rustls-no-provider** Enables rustls, but does not enable any [`CryptoProvider`](#cryptoprovider) such as `ring`.
   Providers other than the default (currently `ring`) are never picked up from feature flags alone.
   It must be configured on the agent.

 * **vendored** compiles and statically links to a copy of non-Rust vendors (e.g. OpenSSL from `native-tls`)

 # TLS (https)

 ## rustls

 By default, ureq uses [`rustls` crate] with the `ring` cryptographic provider.
 As of Sep 2024, the `ring` provider has a higher chance of compiling successfully. If the user
 installs another process [default provider], that choice is respected.

 ureq does not guarantee to default to ring indefinitely. `rustls` as a feature flag will always
 work, but the specific crypto backend might change in a minor version.

 ```
 # #[cfg(feature = "rustls")]
 # {
 // This uses rustls
 ureq::get("https://www.google.com/").call().unwrap();
 # } Ok::<_, ureq::Error>(())
 ```

 ### rustls without ring

 ureq never changes TLS backend from feature flags alone. It is possible to compile ureq
 without ring, but it requires specific feature flags and configuring the [`Agent`](#agent).

 Since rustls is not semver 1.x, this requires non-semver-guaranteed API. I.e. ureq might
 change this behavior without a major version bump.

 Read more at `TlsConfigBuilder::unversioned_rustls_crypto_provider`.

 ## native-tls

 As an alternative, ureq ships with [`native-tls`](#native-tls) as a TLS provider. This must be
 enabled using the **native-tls** feature. Due to the risk of diamond dependencies
 accidentally switching on an unwanted TLS implementation, `native-tls` is never picked
 up as a default or used by the crate level convenience calls (`ureq::get` etc) – it
 must be configured on the agent.

 ```
 # #[cfg(feature = "native-tls")]
 # {
 use ureq::config::Config;
 use ureq::tls::{TlsConfig, TlsProvider};

 let mut config = Config::builder()
     .tls_config(
         TlsConfig::builder()
             // requires the native-tls feature
             .provider(TlsProvider::NativeTls)
             .build()
     )
     .build();

 let agent = config.new_agent();

 agent.get("https://www.google.com/").call().unwrap();
 # } Ok::<_, ureq::Error>(())
 ```

 ## Root certificates

 ### webpki-roots

 By default, ureq uses Mozilla's root certificates via the [webpki-roots] crate. This is a static
 bundle of root certificates that do not update automatically. It also circumvents whatever root
 certificates are installed on the host running ureq, which might be a good or a bad thing depending
 on your perspective. There is also no mechanism for [SCT], [CRL]s or other revocations.
 To maintain a "fresh" list of root certs, you need to bump the ureq dependency from time to time.

 The main reason for chosing this as the default is to minimize the number of dependencies. More
 details about this decision can be found at [PR 818].

 If your use case for ureq is talking to a limited number of servers with high trust, the
 default setting is likely sufficient. If you use ureq with a high number of servers, or servers
 you don't trust, we recommend using the platform verifier (see below).

 ### platform-verifier

 The [rustls-platform-verifier] crate provides access to natively checking the certificate via your OS.
 To use this verifier, you need to enable it using feature flag **platform-verifier** as well as
 configure an agent to use it.

 ```
 # #[cfg(all(feature = "rustls", feature="platform-verifier"))]
 # {
 use ureq::Agent;
 use ureq::tls::{TlsConfig, RootCerts};

 let agent = Agent::config_builder()
     .tls_config(
         TlsConfig::builder()
             .root_certs(RootCerts::PlatformVerifier)
             .build()
     )
     .build()
     .new_agent();

 let response = agent.get("https://httpbin.org/get").call()?;
 # } Ok::<_, ureq::Error>(())
 ```

 Setting `RootCerts::PlatformVerifier` together with `TlsProvider::NativeTls` means
 also native-tls will use the OS roots instead of [webpki-roots] crate. Whether that
 results in a config that has CRLs and revocations is up to whatever native-tls links to.

 # JSON

 By enabling the **json** feature, the library supports serde json.

 This is enabled by default.

 * [`request.send_json()`](#requestsend-json) send body as json.
 * [`body.read_json()`](#bodyread-json) transform response to json.

 # Sending body data

 HTTP/1.1 has two ways of transfering body data. Either of a known size with
 the `Content-Length` HTTP header, or unknown size with the
 `Transfer-Encoding: chunked` header. ureq supports both and will use the
 appropriate method depending on which body is being sent.

 ureq has a [`AsSendBody`](#assendbody) trait that is implemented for many well known types
 of data that we might want to send. The request body can thus be anything
 from a `String` to a `File`, see below.

 ## Content-Length

 The library will send a `Content-Length` header on requests with bodies of
 known size, in other words, if the body to send is one of:

 * `&[u8](#u8)
`
 * `&[u8; N]`
 * `&str`
 * `String`
 * `&String`
 * `Vec<u8>`
 * `&Vec<u8>)`
 * `SendBody::from_json()` (implicitly via [`request.send_json()`](#requestsend-json))

 ## Transfer-Encoding: chunked

 ureq will send a `Transfer-Encoding: chunked` header on requests where the body
 is of unknown size. The body is automatically converted to an `std::io::Read`
 when the type is one of:

 * `File`
 * `&File`
 * `TcpStream`
 * `&TcpStream`
 * `Stdin`
 * `UnixStream` (not on windows)

 ### From readers

 The chunked method also applies for bodies constructed via:

 * `SendBody::from_reader()`
 * `SendBody::from_owned_reader()`

 ## Proxying a response body

 As a special case, when ureq sends a [`Body`](#body) from a previous http call, the
 use of `Content-Length` or `chunked` depends on situation. For input such as
 gzip decoding (**gzip** feature) or charset transformation (**charset** feature),
 the output body might not match the input, which means ureq is forced to use
 the `chunked` method.

 * `Response<Body>`

 ## Sending form data

 [`request.send_form()`](#requestsend-form) provides a way to send `application/x-www-form-urlencoded`
 encoded data. The key/values provided will be URL encoded.

 ## Overriding

 If you set your own Content-Length or Transfer-Encoding header before
 sending the body, ureq will respect that header by not overriding it,
 and by encoding the body or not, as indicated by the headers you set.

 ```
 let resp = ureq::put("https://httpbin.org/put")
     .header("Transfer-Encoding", "chunked")
     .send("Hello world")?;
 # Ok::<_, ureq::Error>(())
 ```

 # Character encoding

 By enabling the **charset** feature, the library supports receiving other
 character sets than `utf-8`.

 For `Body::read_to_string()` we read the header like:

 `Content-Type: text/plain; charset=iso-8859-1`

 and if it contains a charset specification, we try to decode the body using that
 encoding. In the absence of, or failing to interpret the charset, we fall back on `utf-8`.

 Currently ureq does not provide a way to encode when sending request bodies.

 ## Lossy utf-8

 When reading text bodies (with a `Content-Type` starting `text/` as in `text/plain`,
 `text/html`, etc), ureq can ensure the body is possible to read as a `String` also if
 it contains characters that are not valid for utf-8. Invalid characters are replaced
 with a question mark `?` (NOT the utf-8 replacement character).

 For `Body::read_to_string()` this is turned on by default, but it can be disabled
 and conversely for `Body::as_reader()` it is not enabled, but can be.

 To precisely configure the behavior use `Body::with_config()`.

 # Proxying

 ureq supports two kinds of proxies,  [`HTTP`](#http) ([`CONNECT`](#connect)), [`SOCKS4`](#socks4)/[`SOCKS5`](#socks5),
 the former is always available while the latter must be enabled using the feature
 **socks-proxy**.

 Proxies settings are configured on an [`Agent`](#agent). All request sent through the agent will be proxied.

 ## Environment Variables

 ureq automatically reads proxy configuration from environment variables when creating
 a default [`Agent`](#agent). Proxy variables are checked in order: `ALL_PROXY`, `HTTPS_PROXY`,
 then `HTTP_PROXY` (with lowercase variants).

 `NO_PROXY` specifies hosts that bypass the proxy, supporting exact hosts, wildcard
 suffixes (`*.example.com`), dot suffixes (`.example.com`), and match-all (`*`).

 ## Example using HTTP

 ```rust
 use ureq::{Agent, Proxy};
 # fn no_run() -> std::result::Result<(), ureq::Error> {
 // Configure an http connect proxy.
 let proxy = Proxy::new("http://user:password@cool.proxy:9090")?;
 let agent: Agent = Agent::config_builder()
     .proxy(Some(proxy))
     .build()
     .into();

 // This is proxied.
 let resp = agent.get("http://cool.server").call()?;
 # Ok(())}
 # fn main() {}
 ```

 ## Example using SOCKS5

 ```rust
 use ureq::{Agent, Proxy};
 # #[cfg(feature = "socks-proxy")]
 # fn no_run() -> std::result::Result<(), ureq::Error> {
 // Configure a SOCKS proxy.
 let proxy = Proxy::new("socks5://user:password@cool.proxy:9090")?;
 let agent: Agent = Agent::config_builder()
     .proxy(Some(proxy))
     .build()
     .into();

 // This is proxied.
 let resp = agent.get("http://cool.server").call()?;
 # Ok(())}
 ```

 # Log levels

 ureq uses the log crate. These are the definitions of the log levels, however we
 do not guarantee anything for dependencies such as `http` and `rustls`.

 * `ERROR` - nothing
 * `WARN` - if we detect a user configuration problem.
 * `INFO` - nothing
 * `DEBUG` - uri, state changes, transport, resolver and selected request/response headers
 * `TRACE` - wire level debug. NOT REDACTED!

 The request/response headers on DEBUG levels are allow-listed to only include headers that
 are considered safe. The code has the [allow list](https://github.com/algesten/ureq/blob/81127cfc38516903330dc1b9c618122372f8dc29/src/util.rs#L184-L198).

 # Versioning

 ## Semver and `unversioned`

 ureq follows semver. From ureq 3.x we strive to have a much closer adherence to semver than 2.x.
 The main mistake in 2.x was to re-export crates that were not yet semver 1.0. In ureq 3.x TLS and
 cookie configuration is shimmed using our own types.

 ureq 3.x is trying out two new traits that had no equivalent in 2.x, [`Transport`](unversioned/transport/index.md) and [`Resolver`](unversioned/resolver/index.md).
 These allow the user write their own bespoke transports and (DNS name) resolver. The API:s for
 these parts are not yet solidified. They live under the [`unversioned`](unversioned/index.md) module, and do not
 follow semver. See module doc for more info.

 ## Breaking changes in dependencies

 ureq relies on non-semver 1.x crates such as `rustls` and `native-tls`. Some scenarios, such
 as configuring `rustls` to not use `ring`, a user of ureq might need to interact with these
 crates directly instead of going via ureq's provided API.

 Such changes can break when ureq updates dependencies. This is not considered a breaking change
 for ureq and will not be reflected by a major version bump.

 We strive to mark ureq's API with the word "unversioned" to identify places where this risk arises.

 ## Minimum Supported Rust Version (MSRV)

 From time to time we will need to update our minimum supported Rust version (MSRV). This is not
 something we do lightly; our ambition is to be as conservative with MSRV as possible.

 * For some dependencies, we will opt for pinning the version of the dep instead
   of bumping our MSRV.
 * For important dependencies, like the TLS libraries, we cannot hold back our MSRV if they change.
 * We do not consider MSRV changes to be breaking for the purposes of semver.
 * We will not make MSRV changes in patch releases.
 * MSRV changes will get their own minor release, and not be co-mingled with other changes.




 [`rustls` crate]: https://crates.io/crates/rustls
 [default provider]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#method.install_default

 [rustls-platform-verifier]: https://crates.io/crates/rustls-platform-verifier
 [webpki-roots]: https://crates.io/crates/webpki-roots




 [SCT]: https://en.wikipedia.org/wiki/Certificate_Transparency
 [CRL]: https://en.wikipedia.org/wiki/Certificate_revocation_list
 [PR 818]: https://github.com/algesten/ureq/pull/818


















## Modules

- [`config`](config/index.md) - Agent configuration
- [`unversioned`](unversioned/index.md) - API that does not (yet) follow semver.
- [`middleware`](middleware/index.md) - Chained interception to modify the request or response.
- [`tls`](tls/index.md) - TLS for handling `https`.
- [`typestate`](typestate/index.md) - Typestate variables.

## Structs

### `Body`

```rust
struct Body {
    // [REDACTED: Private Fields]
}
```

A response body returned as `http::Response<Body>`.

# Default size limit

Methods like `read_to_string()`, `read_to_vec()`, and `read_json()` have a **default 10MB limit**
to prevent memory exhaustion. To download larger files, use `with_config().limit(new_size)`:

```
// Download a 20MB file
let bytes = ureq::get("http://httpbin.org/bytes/200000000")
    .call()?
    .body_mut()
    .with_config()
    .limit(20 * 1024 * 1024) // 20MB
    .read_to_vec()?;
# Ok::<_, ureq::Error>(())
```

# Body lengths

HTTP/1.1 has two major modes of transfering body data. Either a `Content-Length`
header defines exactly how many bytes to transfer, or `Transfer-Encoding: chunked`
facilitates a streaming style when the size is not known up front.

To protect against a problem called [request smuggling], ureq has heuristics for
how to interpret a server sending both `Transfer-Encoding` and `Content-Length` headers.

1. `chunked` takes precedence if there both headers are present (not for HTTP/1.0)
2. `content-length` is used if there is no chunked
3. If there are no headers, fall back on "close delimited" meaning the socket
   must close to end the body

When a `Content-Length` header is used, ureq will ensure the received body is _EXACTLY_
as many bytes as declared (it cannot be less). This mechanic is in `ureq-proto`
and is different to the `BodyWithConfig::limit()` below.

# Pool reuse

To return a connection (aka `Transport`)
to the Agent's pool, the body must be read to end. If `BodyWithConfig::limit()` is set
shorter size than the actual response body, the connection will not be reused.

# Example

```
use std::io::Read;
let mut res = ureq::get("http://httpbin.org/bytes/100")
    .call()?;

assert!(res.headers().contains_key("Content-Length"));
let len: usize = res.headers().get("Content-Length")
    .unwrap().to_str().unwrap().parse().unwrap();

let mut bytes: Vec<u8> = Vec::with_capacity(len);
res.body_mut().as_reader()
    .read_to_end(&mut bytes)?;

assert_eq!(bytes.len(), len);
# Ok::<_, ureq::Error>(())
```

[request smuggling]: https://en.wikipedia.org/wiki/HTTP_request_smuggling

#### Implementations

- `fn builder() -> BodyBuilder`
  Builder for creating a body

- `fn mime_type(self: &Self) -> Option<&str>`
  The mime-type of the `content-type` header.

- `fn charset(self: &Self) -> Option<&str>`
  The charset of the `content-type` header.

- `fn content_length(self: &Self) -> Option<u64>`
  The content length of the body.

- `fn as_reader(self: &mut Self) -> BodyReader<'_>`
  Handle this body as a shared `impl Read` of the body.

- `fn into_reader(self: Self) -> BodyReader<'static>`
  Turn this response into an owned `impl Read` of the body.

- `fn read_to_string(self: &mut Self) -> Result<String, Error>`
  Read the response as a string.

- `fn read_to_vec(self: &mut Self) -> Result<Vec<u8>, Error>`
  Read the response to a vec.

- `fn with_config(self: &mut Self) -> BodyWithConfig<'_>`
  Read the body data with configuration.

- `fn into_with_config(self: Self) -> BodyWithConfig<'static>`
  Consume self and read the body with configuration.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsSendBody`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BodyBuilder`

```rust
struct BodyBuilder {
    // [REDACTED: Private Fields]
}
```

Builder for creating a response body.

This is useful for testing, or for `Middleware` that
returns another body than the requested one.

# Example

```
use ureq::Body;
use ureq::http::Response;

let body = Body::builder()
    .mime_type("text/plain")
    .charset("utf-8")
    .data("Hello world!");

let mut response = Response::builder()
    .status(200)
    .header("content-type", "text/plain; charset=utf-8")
    .body(body)?;

let text = response
    .body_mut()
    .read_to_string()?;

assert_eq!(text, "Hello world!");
# Ok::<_, ureq::Error>(())
```

#### Implementations

- `fn mime_type(self: Self, mime_type: impl Into<String>) -> Self`
  Set the mime type of the body.

- `fn charset(self: Self, charset: impl Into<String>) -> Self`
  Set the mime type of the body

- `fn limit(self: Self, l: u64) -> Self`
  Limit how much data is to be released from the body.

- `fn data(self: Self, data: impl Into<Vec<u8>>) -> Body`
  Creates the body data turned into bytes.

- `fn reader(self: Self, data: impl io::Read + Send + Sync + 'static) -> Body`
  Creates a body from a streaming reader.

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

### `BodyReader<'a>`

```rust
struct BodyReader<'a> {
    // [REDACTED: Private Fields]
}
```

A reader of the response data.

1. If `Transfer-Encoding: chunked`, the returned reader will unchunk it
   and any `Content-Length` header is ignored.
2. If `Content-Encoding: gzip` (or `br`) and the corresponding feature
   flag is enabled (**gzip** and **brotli**), decompresses the body data.
3. Given a header like `Content-Type: text/plain; charset=ISO-8859-1`
   and the **charset** feature enabled, will translate the body to utf-8.
   This mechanic need two components a mime-type starting `text/` and
   a non-utf8 charset indication.
4. If `Content-Length` is set, the returned reader is limited to this byte
   length regardless of how many bytes the server sends.
5. If no length header, the reader is until server stream end.
6. The limit in the body method used to obtain the reader.

Note: The reader is also limited by the `Body::as_reader` and
`Body::into_reader` calls. If that limit is set very high, a malicious
server might return enough bytes to exhaust available memory. If you're
making requests to untrusted servers, you should use set that
limit accordingly.

# Example

```
use std::io::Read;
let mut res = ureq::get("http://httpbin.org/bytes/100")
    .call()?;

assert!(res.headers().contains_key("Content-Length"));
let len: usize = res.headers().get("Content-Length")
    .unwrap().to_str().unwrap().parse().unwrap();

let mut bytes: Vec<u8> = Vec::with_capacity(len);
res.body_mut().as_reader()
    .read_to_end(&mut bytes)?;

assert_eq!(bytes.len(), len);
# Ok::<_, ureq::Error>(())
```

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

##### `impl Read<'a>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `BodyWithConfig<'a>`

```rust
struct BodyWithConfig<'a> {
    // [REDACTED: Private Fields]
}
```

Configuration of how to read the body.

Obtained via one of:

* [Body::with_config()]
* [Body::into_with_config()]

# Handling large responses

The `BodyWithConfig` is the primary way to increase the default 10MB size limit
when downloading large files to memory:

```
// Download a 50MB file
let large_data = ureq::get("http://httpbin.org/bytes/200000000")
    .call()?
    .body_mut()
    .with_config()
    .limit(50 * 1024 * 1024) // 50MB
    .read_to_vec()?;
# Ok::<_, ureq::Error>(())
```

#### Implementations

- `fn limit(self: Self, value: u64) -> Self`
  Limit the response body.

- `fn lossy_utf8(self: Self, value: bool) -> Self`
  Replace invalid utf-8 chars.

- `fn reader(self: Self) -> BodyReader<'a>`
  Creates a reader.

- `fn read_to_string(self: Self) -> Result<String, Error>`
  Read into string.

- `fn read_to_vec(self: Self) -> Result<Vec<u8>, Error>`
  Read into vector.

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

### `Proxy`

```rust
struct Proxy {
    // [REDACTED: Private Fields]
}
```

Proxy server settings

This struct represents a proxy server configuration that can be used to route HTTP/HTTPS
requests through a proxy server. It supports various proxy protocols including HTTP CONNECT,
HTTPS CONNECT, SOCKS4, SOCKS4A, and SOCKS5.

# Protocol Support

* `HTTP`: HTTP CONNECT proxy
* `HTTPS`: HTTPS CONNECT proxy (requires a TLS provider)
* `SOCKS4`: SOCKS4 proxy (requires **socks-proxy** feature)
* `SOCKS4A`: SOCKS4A proxy (requires **socks-proxy** feature)
* `SOCKS5`: SOCKS5 proxy (requires **socks-proxy** feature)

# DNS Resolution

The `resolve_target` setting controls where DNS resolution happens:

* When `true`: DNS resolution happens locally before connecting to the proxy.
  The resolved IP address is sent to the proxy.
* When `false`: The hostname is sent to the proxy, which performs DNS resolution.

Default behavior:
* For SOCKS4: `true` (local resolution required)
* For all other protocols: `false` (proxy performs resolution)

# Examples

```rust
use ureq::{Proxy, ProxyProtocol};

// Create a proxy from a URI string
let proxy = Proxy::new("http://localhost:8080").unwrap();

// Create a proxy using the builder pattern
let proxy = Proxy::builder(ProxyProtocol::Socks5)
    .host("proxy.example.com")
    .port(1080)
    .username("user")
    .password("pass")
    .resolve_target(true)  // Force local DNS resolution
    .build()
    .unwrap();

// Read proxy settings from environment variables
if let Some(proxy) = Proxy::try_from_env() {
    // Use proxy from environment
}
```

#### Implementations

- `fn new(proxy: &str) -> Result<Self, Error>`
  Create a proxy from a uri.

- `fn builder(p: ProxyProtocol) -> ProxyBuilder`
  Creates a proxy config using a builder.

- `fn try_from_env() -> Option<Self>`
  Read proxy settings from environment variables.

- `fn protocol(self: &Self) -> ProxyProtocol`
  The configured protocol.

- `fn uri(self: &Self) -> &Uri`
  The proxy uri

- `fn host(self: &Self) -> &str`
  The host part of the proxy uri

- `fn port(self: &Self) -> u16`
  The port of the proxy uri

- `fn username(self: &Self) -> Option<&str>`
  The username of the proxy uri

- `fn password(self: &Self) -> Option<&str>`
  The password of the proxy uri

- `fn is_from_env(self: &Self) -> bool`
  Whether this proxy setting was created manually or from

- `fn resolve_target(self: &Self) -> bool`
  Whether to resolve target locally before calling the proxy.

- `fn is_no_proxy(self: &Self, uri: &Uri) -> bool`
  Tells if this entry matches anything on the NO_PROXY list.

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

- `fn clone(self: &Self) -> Proxy`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Proxy) -> bool`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ProxyBuilder`

```rust
struct ProxyBuilder {
    // [REDACTED: Private Fields]
}
```

Builder for configuring a proxy.

Obtained via `Proxy::builder()`.

#### Implementations

- `fn host(self: Self, host: &str) -> Self`
  Set the proxy hostname

- `fn port(self: Self, port: u16) -> Self`
  Set the proxy port

- `fn username(self: Self, v: &str) -> Self`
  Set the username

- `fn password(self: Self, v: &str) -> Self`
  Set the password

- `fn resolve_target(self: Self, do_resolve: bool) -> Self`
  Whether to resolve the target host locally before calling the proxy.

- `fn no_proxy(self: Self, expr: &str) -> Self`
  Add a NO_PROXY expression to not route proxy through.

- `fn build(self: Self) -> Result<Proxy, Error>`
  Construct the [`Proxy`]

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

### `RequestBuilder<B>`

```rust
struct RequestBuilder<B> {
    // [REDACTED: Private Fields]
}
```

Transparent wrapper around `http::request::Builder`.

The purpose is to provide the `.call()` and `.send()`
and additional helpers for query parameters like `.query()` functions to
make an API for sending requests.

#### Implementations

- `fn method_ref(self: &Self) -> Option<&Method>`
  Get the HTTP Method for this request.

- `fn header<K, V>(self: Self, key: K, value: V) -> Self`
  Appends a header to this request builder.

- `fn headers_ref(self: &Self) -> Option<&HeaderMap<HeaderValue>>`
  Get header on this request builder.

- `fn headers_mut(self: &mut Self) -> Option<&mut HeaderMap<HeaderValue>>`
  Get headers on this request builder.

- `fn query<K, V>(self: Self, key: K, value: V) -> Self`
  Add a query parameter to the URL.

- `fn query_raw<K, V>(self: Self, key: K, value: V) -> Self`
  Add a query parameter to the URL without percent-encoding.

- `fn query_pairs<I, K, V>(self: Self, iter: I) -> Self`
  Set multi query parameters.

- `fn query_pairs_raw<I, K, V>(self: Self, iter: I) -> Self`
  Set multi query parameters without percent-encoding.

- `fn uri<T>(self: Self, uri: T) -> Self`
  Overrides the URI for this request.

- `fn uri_ref(self: &Self) -> Option<&Uri>`
  Get the URI for this request

- `fn version(self: Self, version: Version) -> Self`
  Set the HTTP version for this request.

- `fn version_ref(self: &Self) -> Option<&Version>`
  Get the HTTP version for this request

- `fn config(self: Self) -> ConfigBuilder<RequestScope<Any>>`
  Override agent level config on the request level.

- `fn extension<T>(self: Self, extension: T) -> Self`
  Adds an extension to this builder

- `fn extensions_ref(self: &Self) -> Option<&Extensions>`
  Get a reference to the extensions for this request builder.

- `fn extensions_mut(self: &mut Self) -> Option<&mut Extensions>`
  Get a mutable reference to the extensions for this request builder.

- `fn call(self: Self) -> Result<Response<Body>, Error>`
  Sends the request and blocks the caller until we receive a response.

- `fn force_send_body(self: Self) -> RequestBuilder<WithBody>`
  Force sending a body.

- `fn content_type<V>(self: Self, content_type: V) -> Self`
  Set the content-type header.

- `fn send(self: Self, data: impl AsSendBody) -> Result<Response<Body>, Error>`
  Send body data and blocks the caller until we receive response.

- `fn send_empty(self: Self) -> Result<Response<Body>, Error>`
  Send an empty body.

- `fn send_form<I, K, V>(self: Self, iter: I) -> Result<Response<Body>, Error>`
  Send form encoded data.

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Agent`

```rust
struct Agent {
    // [REDACTED: Private Fields]
}
```

Agents keep state between requests.

By default, no state, such as cookies, is kept between requests.
But by creating an agent as entry point for the request, we
can keep a state.

# Example

```no_run
let mut agent = ureq::agent();

agent
    .post("http://example.com/post/login")
    .send(b"my password")?;

let secret = agent
    .get("http://example.com/get/my-protected-page")
    .call()?
    .body_mut()
    .read_to_string()?;

  println!("Secret is: {}", secret);
# Ok::<_, ureq::Error>(())
```

# About threads and cloning

Agent uses inner [`Arc`](#arc). Cloning an Agent results in an instance
that shares the same underlying connection pool and other state.

The connection pool contains an inner `Mutex` which is (briefly)
held when borrowing a pooled connection, or returning a connection to the pool.

All request functions in ureq have a signature similar to this:

```
# use ureq::{http, Body, AsSendBody, Error};
fn run(request: http::Request<impl AsSendBody>) -> Result<http::Response<Body>, Error> {
    // <something>
# todo!()
}
```

It follows that:

* An Agent is borrowed for the duration of:
    1. Sending the request header (`http::Request`)
    2. Sending the request body ([`SendBody`](#sendbody))
    3. Receiving the response header (`http::Response`)
* The [`Body`](#body) of the response is not bound to the lifetime of the Agent.

A response [`Body`](#body) can be streamed (for instance via `Body::into_reader()`). The [`Body`](#body)
implements [`Send`](#send), which means it's possible to read the response body on another thread than
the one that run the request. Behind the scenes, the [`Body`](#body) retains the connection to the remote
server and it is returned to the agent's pool, once the Body instance (or reader) is dropped.

There is an asymmetry in that sending a request body will borrow the Agent instance, while receiving
the response body does not. This inconvenience is somewhat mitigated by that `Agent::run()` (or
going via the methods such as `Agent::get()`), borrows `&self`, i.e. not exclusive `mut` borrows.

That cloning the agent shares the connection pool is considered a feature. It is often useful to
retain a single pool for the entire process, while dispatching requests from different threads.
And if we want separate pools, we can create multiple agents via one of the constructors
(such as `Agent::new_with_config()`).

Note that both `Config::clone()` and `Agent::clone()` are  "cheap" meaning they should not
incur any heap allocation.

#### Implementations

- `fn new_with_defaults() -> Self`
  Creates an agent with defaults.

- `fn new_with_config(config: Config) -> Self`
  Creates an agent with config.

- `fn config_builder() -> ConfigBuilder<AgentScope>`
  Shortcut to reach a [`ConfigBuilder`]

- `fn with_parts(config: Config, connector: impl Connector, resolver: impl Resolver) -> Self`
  Creates an agent with a bespoke transport and resolver.

- `fn run(self: &Self, request: Request<impl AsSendBody>) -> Result<Response<Body>, Error>`
  Run a [`http::Request<impl AsSendBody>`].

- `fn config(self: &Self) -> &Config`
  Get the config for this agent.

- `fn configure_request<S: AsSendBody>(self: &Self, request: Request<S>) -> ConfigBuilder<HttpCrateScope<S>>`
  Alter the configuration for an http crate request.

- `fn get<T>(self: &Self, uri: T) -> RequestBuilder<WithoutBody>`
  Make a GET request using this agent.

- `fn post<T>(self: &Self, uri: T) -> RequestBuilder<WithBody>`
  Make a POST request using this agent.

- `fn put<T>(self: &Self, uri: T) -> RequestBuilder<WithBody>`
  Make a PUT request using this agent.

- `fn delete<T>(self: &Self, uri: T) -> RequestBuilder<WithoutBody>`
  Make a DELETE request using this agent.

- `fn head<T>(self: &Self, uri: T) -> RequestBuilder<WithoutBody>`
  Make a HEAD request using this agent.

- `fn options<T>(self: &Self, uri: T) -> RequestBuilder<WithoutBody>`
  Make an OPTIONS request using this agent.

- `fn connect<T>(self: &Self, uri: T) -> RequestBuilder<WithoutBody>`
  Make a CONNECT request using this agent.

- `fn patch<T>(self: &Self, uri: T) -> RequestBuilder<WithBody>`
  Make a PATCH request using this agent.

- `fn trace<T>(self: &Self, uri: T) -> RequestBuilder<WithoutBody>`
  Make a TRACE request using this agent.

#### Trait Implementations

##### `impl From`

- `fn from(value: Config) -> Self`

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

- `fn clone(self: &Self) -> Agent`

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

### `SendBody<'a>`

```rust
struct SendBody<'a> {
    // [REDACTED: Private Fields]
}
```

Request body for sending data via POST, PUT and PATCH.

Typically not interacted with directly since the trait [`AsSendBody`](#assendbody) is implemented
for the majority of the types of data a user might want to send to a remote server.
That means if you want to send things like `String`, `&str` or `[u8](#u8)
`, they can be
used directly. See documentation for [`AsSendBody`](#assendbody).

The exception is when using [`Read`](#read) trait bodies, in which case we wrap the request
body directly. See below `SendBody::from_reader`.


#### Implementations

- `fn none() -> SendBody<'static>`
  Creates an empty body.

- `fn from_reader(reader: &'a mut dyn Read) -> SendBody<'a>`
  Creates a body from a shared [`Read`] impl.

- `fn from_owned_reader(reader: impl Read + 'static) -> SendBody<'static>`
  Creates a body from an owned [`Read`] impl.

- `fn into_reader(self: Self) -> impl Sized + io::Read + 'a`
  Turn this `SendBody` into a reader.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from((size, inner): (Option<u64>, BodyInner<'a>)) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsSendBody<'a>`

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

### `ProxyProtocol`

```rust
enum ProxyProtocol {
    Http,
    Https,
    Socks4,
    Socks4A,
    Socks5,
}
```

Proxy protocol

#### Variants

- **`Http`**

  CONNECT proxy over HTTP

- **`Https`**

  CONNECT proxy over HTTPS

- **`Socks4`**

  A SOCKS4 proxy

- **`Socks4A`**

  A SOCKS4a proxy (proxy can resolve domain name)

- **`Socks5`**

  SOCKS5 proxy

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

- `fn clone(self: &Self) -> ProxyProtocol`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ProxyProtocol) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom`

- `type Error = Error`

- `fn try_from(scheme: &str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Error`

```rust
enum Error {
    StatusCode(u16),
    Http(http::Error),
    BadUri(String),
    Protocol(ureq_proto::Error),
    Io(io::Error),
    Timeout(crate::Timeout),
    HostNotFound,
    RedirectFailed,
    InvalidProxyUrl,
    ConnectionFailed,
    BodyExceedsLimit(u64),
    TooManyRedirects,
    Tls(&'static str),
    Pem(rustls_pki_types::pem::Error),
    Rustls(rustls::Error),
    RequireHttpsOnly(String),
    LargeResponseHeader(usize, usize),
    Decompress(&'static str, io::Error),
    ConnectProxyFailed(String),
    TlsRequired,
    Other(Box<dyn std::error::Error + Send + Sync>),
}
```

Errors from ureq.

#### Variants

- **`StatusCode`**

  When [`http_status_as_error()`](crate::config::ConfigBuilder::http_status_as_error) is true,
  4xx and 5xx response status codes are translated to this error.
  
  This is the default behavior.

- **`Http`**

  Errors arising from the http-crate.
  
  These errors happen for things like invalid characters in header names.

- **`BadUri`**

  Error if the URI is missing scheme or host.

- **`Protocol`**

  An HTTP/1.1 protocol error.
  
  This can happen if the remote server ends incorrect HTTP data like
  missing version or invalid chunked transfer.

- **`Io`**

  Error in io such as the TCP socket.

- **`Timeout`**

  Error raised if the request hits any configured timeout.
  
  By default no timeouts are set, which means this error can't happen.

- **`HostNotFound`**

  Error when resolving a hostname fails.

- **`RedirectFailed`**

  A redirect failed.
  
  This happens when ureq encounters a redirect when sending a request body
  such as a POST request, and receives a 307/308 response. ureq refuses to
  redirect the POST body and instead raises this error.

- **`InvalidProxyUrl`**

  Error when creating proxy settings.

- **`ConnectionFailed`**

  A connection failed.
  
  This is a fallback error when there is no other explanation as to
  why a connector chain didn't produce a connection. The idea is that the connector
  chain would return some other [`Error`](#error) rather than rely om this value.
  
  Typically bespoke connector chains should, as far as possible, map their underlying
  errors to `Error::Io` and use the `io::ErrorKind` to provide a reason.
  
  A bespoke chain is allowed to map to this value, but that provides very little
  information to the user as to why the connection failed. One way to mitigate that
  would be to rely on the `log` crate to provide additional information.

- **`BodyExceedsLimit`**

  A send body (Such as `&str`) is larger than the `content-length` header.

- **`TooManyRedirects`**

  Too many redirects.
  
  The error can be turned off by setting
  [`max_redirects_will_error()`](crate::config::ConfigBuilder::max_redirects_will_error)
  to false. When turned off, the last response will be returned instead of causing
  an error, even if it is a redirect.
  
  The number of redirects is limited to 10 by default.

- **`Tls`**

  Some error with TLS.

- **`Pem`**

  Error in reading PEM certificates/private keys.
  
  *Note:* The wrapped error struct is not considered part of ureq API.
  Breaking changes in that struct will not be reflected in ureq
  major versions.

- **`Rustls`**

  An error originating in Rustls.
  
  *Note:* The wrapped error struct is not considered part of ureq API.
  Breaking changes in that struct will not be reflected in ureq
  major versions.

- **`RequireHttpsOnly`**

  The setting [`https_only`](crate::config::ConfigBuilder::https_only) is true and
  the URI is not https.

- **`LargeResponseHeader`**

  The response header, from status up until body, is too big.

- **`Decompress`**

  Body decompression failed (gzip or brotli).

- **`ConnectProxyFailed`**

  Attempt to connect to a CONNECT proxy failed.

- **`TlsRequired`**

  The protocol requires TLS (https), but the connector did not
  create a TLS secured transport.
  
  This typically indicates a fault in bespoke `Connector` chains.

- **`Other`**

  Some other error occured.
  
  This is an escape hatch for bespoke connector chains having errors that don't naturally
  map to any other error. For connector chains we recommend:
  
  1. Map to `Error::Io` as far as possible.
  2. Map to other [`Error`](#error) where reasonable.
  3. Fall back on `Error::Other`.
  4. As a last resort `Error::ConnectionFailed`.
  
  ureq does not produce this error using the default connectors.

#### Implementations

- `fn into_io(self: Self) -> io::Error`
  Convert the error into a [`std::io::Error`].

#### Trait Implementations

##### `impl From`

- `fn from(value: ureq_proto::Error) -> Self`

##### `impl From`

- `fn from(e: io::Error) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: http::Error) -> Self`

##### `impl From`

- `fn from(value: rustls::Error) -> Self`

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

### `Timeout`

```rust
enum Timeout {
    Global,
    PerCall,
    Resolve,
    Connect,
    SendRequest,
    SendBody,
    RecvResponse,
    RecvBody,
}
```

The various timeouts.

Each enum corresponds to a value in
`ConfigBuilder::timeout_xxx`.

#### Variants

- **`Global`**

  Timeout for entire operation.

- **`PerCall`**

  Timeout for the current call (when redirected).

- **`Resolve`**

  Timeout in the resolver.

- **`Connect`**

  Timeout while opening the connection.

- **`SendRequest`**

  Timeout while sending the request headers.

- **`SendBody`**

  Timeout when sending then request body.

- **`RecvResponse`**

  Timeout while receiving the response headers.

- **`RecvBody`**

  Timeout while receiving the response body.

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

- `fn clone(self: &Self) -> Timeout`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Timeout) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

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

## Traits

## Functions

### `run`

```rust
fn run(request: http::Request<impl AsSendBody>) -> Result<http::Response<Body>, Error>
```

Run a `http::Request<impl AsSendBody>`.

### `agent`

```rust
fn agent() -> Agent
```

A new [Agent] with default configuration

Agents are used to hold configuration and keep state between requests.

### `get`

```rust
fn get<T>(uri: T) -> RequestBuilder<request::WithoutBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a GET request.

Run on a use-once [`Agent`](#agent).

### `post`

```rust
fn post<T>(uri: T) -> RequestBuilder<request::WithBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a POST request.

Run on a use-once [`Agent`](#agent).

### `put`

```rust
fn put<T>(uri: T) -> RequestBuilder<request::WithBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a PUT request.

Run on a use-once [`Agent`](#agent).

### `delete`

```rust
fn delete<T>(uri: T) -> RequestBuilder<request::WithoutBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a DELETE request.

Run on a use-once [`Agent`](#agent).

### `head`

```rust
fn head<T>(uri: T) -> RequestBuilder<request::WithoutBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a HEAD request.

Run on a use-once [`Agent`](#agent).

### `options`

```rust
fn options<T>(uri: T) -> RequestBuilder<request::WithoutBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make an OPTIONS request.

Run on a use-once [`Agent`](#agent).

### `connect`

```rust
fn connect<T>(uri: T) -> RequestBuilder<request::WithoutBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a CONNECT request.

Run on a use-once [`Agent`](#agent).

### `patch`

```rust
fn patch<T>(uri: T) -> RequestBuilder<request::WithBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a PATCH request.

Run on a use-once [`Agent`](#agent).

### `trace`

```rust
fn trace<T>(uri: T) -> RequestBuilder<request::WithoutBody>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom>::Error: Into<http::Error>
```

Make a TRACE request.

Run on a use-once [`Agent`](#agent).

