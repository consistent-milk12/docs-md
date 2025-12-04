# Crate `rustls`

# Rustls - a modern TLS library

Rustls is a TLS library that aims to provide a good level of cryptographic security,
requires no configuration to achieve that security, and provides no unsafe features or
obsolete cryptography by default.

Rustls implements TLS1.2 and TLS1.3 for both clients and servers. See [the full
list of protocol features](manual::_04_features).

### Platform support

While Rustls itself is platform independent, by default it uses [`aws-lc-rs`](#aws-lc-rs) for implementing
the cryptography in TLS.  See [the aws-lc-rs FAQ][aws-lc-rs-platforms-faq] for more details of the
platform/architecture support constraints in aws-lc-rs.

[`ring`](rustls/crypto/ring/index.md) is also available via the `ring` crate feature: see
[the supported `ring` target platforms][ring-target-platforms].

By providing a custom instance of the [`crypto::CryptoProvider`](#cryptoprovider) struct, you
can replace all cryptography dependencies of rustls.  This is a route to being portable
to a wider set of architectures and environments, or compliance requirements.  See the
[`crypto::CryptoProvider`](#cryptoprovider) documentation for more details.

Specifying `default-features = false` when depending on rustls will remove the implicit
dependency on aws-lc-rs.

Rustls requires Rust 1.71 or later. It has an optional dependency on zlib-rs which requires 1.75 or later.

[ring-target-platforms]: https://github.com/briansmith/ring/blob/2e8363b433fa3b3962c877d9ed2e9145612f3160/include/ring-core/target.h#L18-L64


[aws-lc-rs-platforms-faq]: https://aws.github.io/aws-lc-rs/faq.html#can-i-run-aws-lc-rs-on-x-platform-or-architecture

### Cryptography providers

Since Rustls 0.22 it has been possible to choose the provider of the cryptographic primitives
that Rustls uses. This may be appealing if you have specific platform, compliance or feature
requirements that aren't met by the default provider, [`aws-lc-rs`](#aws-lc-rs).

Users that wish to customize the provider in use can do so when constructing `ClientConfig`
and `ServerConfig` instances using the `with_crypto_provider` method on the respective config
builder types. See the [`crypto::CryptoProvider`](#cryptoprovider) documentation for more details.

#### Built-in providers

Rustls ships with two built-in providers controlled by associated crate features:

  * [`aws-lc-rs`](#aws-lc-rs) - enabled by default, available with the `aws_lc_rs` crate feature enabled.
  * [`ring`](rustls/crypto/ring/index.md) - available with the `ring` crate feature enabled.

See the documentation for [`crypto::CryptoProvider`](#cryptoprovider) for details on how providers are
selected.

#### Third-party providers

The community has also started developing third-party providers for Rustls:

  * [`boring-rustls-provider`](#boring-rustls-provider) - a work-in-progress provider that uses [`boringssl`](#boringssl) for
    cryptography.
  * [`rustls-graviola`](#rustls-graviola) - a provider that uses [`graviola`](#graviola) for cryptography.
  * [`rustls-mbedtls-provider`](#rustls-mbedtls-provider) - a provider that uses [`mbedtls`](#mbedtls) for cryptography.
  * [`rustls-openssl`](#rustls-openssl) - a provider that uses [OpenSSL] for cryptography.
  * [`rustls-rustcrypto`](#rustls-rustcrypto) - an experimental provider that uses the crypto primitives
    from [`RustCrypto`](#rustcrypto) for cryptography.
  * [`rustls-symcrypt`](#rustls-symcrypt) - a provider that uses Microsoft's [SymCrypt] library.
  * [`rustls-wolfcrypt-provider`](#rustls-wolfcrypt-provider) - a work-in-progress provider that uses [`wolfCrypt`](#wolfcrypt) for cryptography.





[OpenSSL]: https://openssl-library.org/

[SymCrypt]: https://github.com/microsoft/SymCrypt






#### Custom provider

We also provide a simple example of writing your own provider in the [custom provider example].
This example implements a minimal provider using parts of the [`RustCrypto`](#rustcrypto) ecosystem.

See the [Making a custom CryptoProvider] section of the documentation for more information
on this topic.

[custom provider example]: https://github.com/rustls/rustls/tree/main/provider-example/

[Making a custom CryptoProvider]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#making-a-custom-cryptoprovider

## Design overview

Rustls is a low-level library. If your goal is to make HTTPS connections you may prefer
to use a library built on top of Rustls like [hyper](#hyper)
 or [ureq](#ureq)
.

[hyper](#hyper)
: https://crates.io/crates/hyper
[ureq](#ureq)
: https://crates.io/crates/ureq

### Rustls does not take care of network IO
It doesn't make or accept TCP connections, or do DNS, or read or write files.

Our [examples](#examples)
 directory contains demos that show how to handle I/O using the
[`stream::Stream`](#stream) helper, as well as more complex asynchronous I/O using [`mio`](#mio).
If you're already using Tokio for an async runtime you may prefer to use [`tokio-rustls`](#tokio-rustls) instead
of interacting with rustls directly.

[examples](#examples)
: https://github.com/rustls/rustls/tree/main/examples

### Rustls provides encrypted pipes
These are the [`ServerConnection`](#serverconnection) and [`ClientConnection`](#clientconnection) types.  You supply raw TLS traffic
on the left (via the [`read_tls()`](#read-tls) and [`write_tls()`](#write-tls) methods) and then read/write the
plaintext on the right:


```text
         TLS                                   Plaintext
         ===                                   =========
    read_tls()      +-----------------------+      reader() as io::Read
                    |                       |
          +--------->   ClientConnection    +--------->
                    |          or           |
          <---------+   ServerConnection    <---------+
                    |                       |
    write_tls()     +-----------------------+      writer() as io::Write
```

### Rustls takes care of server certificate verification
You do not need to provide anything other than a set of root certificates to trust.
Certificate verification cannot be turned off or disabled in the main API.

## Getting started
This is the minimum you need to do to make a TLS client connection.

First we load some root certificates.  These are used to authenticate the server.
The simplest way is to depend on the [`webpki_roots`](../webpki_roots/index.md) crate which contains
the Mozilla set of root certificates.

```rust,no_run
# #[cfg(feature = "aws-lc-rs")] {
let root_store = rustls::RootCertStore::from_iter(
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned(),
);
# }
```

Next, we make a `ClientConfig`.  You're likely to make one of these per process,
and use it for all connections made by that process.

```rust,no_run
# #[cfg(feature = "aws_lc_rs")] {
# let root_store: rustls::RootCertStore = panic!();
let config = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();
# }
```

Now we can make a connection.  You need to provide the server's hostname so we
know what to expect to find in the server's certificate.

```rust
# #[cfg(feature = "aws_lc_rs")] {
# use rustls;
# use webpki;
# use std::sync::Arc;
# rustls::crypto::aws_lc_rs::default_provider().install_default();
# let root_store = rustls::RootCertStore::from_iter(
#  webpki_roots::TLS_SERVER_ROOTS
#      .iter()
#      .cloned(),
# );
# let config = rustls::ClientConfig::builder()
#     .with_root_certificates(root_store)
#     .with_no_client_auth();
let rc_config = Arc::new(config);
let example_com = "example.com".try_into().unwrap();
let mut client = rustls::ClientConnection::new(rc_config, example_com);
# }
```

Now you should do appropriate IO for the `client` object.  If `client.wants_read()` yields
true, you should call `client.read_tls()` when the underlying connection has data.
Likewise, if `client.wants_write()` yields true, you should call `client.write_tls()`
when the underlying connection is able to send data.  You should continue doing this
as long as the connection is valid.

The return types of `read_tls()` and `write_tls()` only tell you if the IO worked.  No
parsing or processing of the TLS messages is done.  After each `read_tls()` you should
therefore call `client.process_new_packets()` which parses and processes the messages.
Any error returned from `process_new_packets` is fatal to the connection, and will tell you
why.  For example, if the server's certificate is expired `process_new_packets` will
return `Err(InvalidCertificate(Expired))`.  From this point on,
`process_new_packets` will not do any new work and will return that error continually.

You can extract newly received data by calling `client.reader()` (which implements the
`io::Read` trait).  You can send data to the peer by calling `client.writer()` (which
implements `io::Write` trait).  Note that `client.writer().write()` buffers data you
send if the TLS connection is not yet established: this is useful for writing (say) a
HTTP request, but this is buffered so avoid large amounts of data.

The following code uses a fictional socket IO API for illustration, and does not handle
errors.

```rust,no_run
# #[cfg(feature = "aws_lc_rs")] {
# let mut client = rustls::ClientConnection::new(panic!(), panic!()).unwrap();
# struct Socket { }
# impl Socket {
#   fn ready_for_write(&self) -> bool { false }
#   fn ready_for_read(&self) -> bool { false }
#   fn wait_for_something_to_happen(&self) { }
# }
#
# use std::io::{Read, Write, Result};
# impl Read for Socket {
#   fn read(&mut self, buf: &mut [u8](#u8)
) -> Result<usize> { panic!() }
# }
# impl Write for Socket {
#   fn write(&mut self, buf: &[u8](#u8)
) -> Result<usize> { panic!() }
#   fn flush(&mut self) -> Result<()> { panic!() }
# }
#
# fn connect(_address: &str, _port: u16) -> Socket {
#   panic!();
# }
use std::io;
use rustls::Connection;

client.writer().write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut socket = connect("example.com", 443);
loop {
  if client.wants_read() && socket.ready_for_read() {
    client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();

    let mut plaintext = Vec::new();
    client.reader().read_to_end(&mut plaintext).unwrap();
    io::stdout().write(&plaintext).unwrap();
  }

  if client.wants_write() && socket.ready_for_write() {
    client.write_tls(&mut socket).unwrap();
  }

  socket.wait_for_something_to_happen();
}
# }
```

# Examples

You can find several client and server examples of varying complexity in the [examples](#examples)

directory, including [`tlsserver-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsserver-mio.rs)
and [`tlsclient-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsclient-mio.rs)
\- full worked examples using [`mio`](#mio).

# Manual

The [rustls manual](crate::manual) explains design decisions and includes how-to guidance.

# Crate features
Here's a list of what features are exposed by the rustls crate and what
they mean.

- `std` (enabled by default): enable the high-level (buffered) Connection API and other functionality
  which relies on the `std` library.

- `aws_lc_rs` (enabled by default): makes the rustls crate depend on the [`aws-lc-rs`](#aws-lc-rs) crate.
  Use `rustls::crypto::aws_lc_rs::default_provider().install_default()` to
  use it as the default `CryptoProvider`, or provide it explicitly
  when making a `ClientConfig` or `ServerConfig`.

  Note that aws-lc-rs has additional build-time dependencies like cmake.
  See [the documentation](https://aws.github.io/aws-lc-rs/requirements/index.html) for details.

- `ring`: makes the rustls crate depend on the *ring* crate for cryptography.
  Use `rustls::crypto::ring::default_provider().install_default()` to
  use it as the default `CryptoProvider`, or provide it explicitly
  when making a `ClientConfig` or `ServerConfig`.

- `fips`: enable support for FIPS140-3-approved cryptography, via the [`aws-lc-rs`](#aws-lc-rs) crate.
  This feature enables the `aws_lc_rs` crate feature, which makes the rustls crate depend
  on [aws-lc-rs](https://github.com/aws/aws-lc-rs).  It also changes the default
  for [`ServerConfig::require_ems`](#require-ems) and [`ClientConfig::require_ems`](#require-ems).

  See [manual::_06_fips] for more details.

- `prefer-post-quantum` (enabled by default): for the [`aws-lc-rs`](#aws-lc-rs)-backed provider,
  prioritizes post-quantum secure key exchange by default (using X25519MLKEM768).
  This feature merely alters the order of `rustls::crypto::aws_lc_rs::DEFAULT_KX_GROUPS`.
  See [the manual][x25519mlkem768-manual] for more details.

- `custom-provider`: disables implicit use of built-in providers (`aws-lc-rs` or `ring`). This forces
  applications to manually install one, for instance, when using a custom `CryptoProvider`.

- `tls12` (enabled by default): enable support for TLS version 1.2. Note that, due to the
  additive nature of Cargo features and because it is enabled by default, other crates
  in your dependency graph could re-enable it for your application. If you want to disable
  TLS 1.2 for security reasons, consider explicitly enabling TLS 1.3 only in the config
  builder API.

- `logging` (enabled by default): make the rustls crate depend on the `log` crate.
  rustls outputs interesting protocol-level messages at `trace!` and `debug!` level,
  and protocol-level errors at `warn!` and `error!` level.  The log messages do not
  contain secret key data, and so are safe to archive without affecting session security.

- `read_buf`: when building with Rust Nightly, adds support for the unstable
  `std::io::ReadBuf` and related APIs. This reduces costs from initializing
  buffers. Will do nothing on non-Nightly releases.

- `brotli`: uses the `brotli` crate for RFC8879 certificate compression support.

- `zlib`: uses the `zlib-rs` crate for RFC8879 certificate compression support.

[x25519mlkem768-manual]: manual::_05_defaults#about-the-post-quantum-secure-key-exchange-x25519mlkem768

## Modules

- [`compress`](compress/index.md) - Certificate compression and decompression support
- [`crypto`](crypto/index.md) - Crypto provider interface.
- [`unbuffered`](unbuffered/index.md) - Unbuffered connection API
- [`client`](client/index.md) - Items for use in a client.
- [`server`](server/index.md) - Items for use in a server.
- [`version`](version/index.md) - All defined protocol versions appear in this module.
- [`pki_types`](pki_types/index.md) - Re-exports the contents of the [rustls-pki-types](https://docs.rs/rustls-pki-types) crate for easy access
- [`sign`](sign/index.md) - Message signing interfaces.
- [`quic`](quic/index.md) - APIs for implementing QUIC TLS
- [`ticketer`](ticketer/index.md) - APIs for implementing TLS tickets
- [`manual`](manual/index.md) -  This is the rustls manual.
- [`time_provider`](time_provider/index.md) - The library's source of time.
- [`lock`](lock/index.md) - APIs abstracting over locking primitives.
- [`kernel`](kernel/index.md) - 
- [`ffdhe_groups`](ffdhe_groups/index.md) - 

## Structs

### `ConfigBuilder<Side: ConfigSide, State>`

```rust
struct ConfigBuilder<Side: ConfigSide, State> {
}
```

A [builder](#builder)
 for [`ServerConfig`](#serverconfig) or [`ClientConfig`](#clientconfig) values.

To get one of these, call [`ServerConfig::builder()`](#builder) or [`ClientConfig::builder()`](#builder).

To build a config, you must make at least two decisions (in order):

- How should this client or server verify certificates provided by its peer?
- What certificates should this client or server present to its peer?

For settings besides these, see the fields of [`ServerConfig`](#serverconfig) and [`ClientConfig`](#clientconfig).

The usual choice for protocol primitives is to call
[`ClientConfig::builder`](#builder)/[`ServerConfig::builder`](#builder)
which will use rustls' default cryptographic provider and safe defaults for ciphersuites and
supported protocol versions.

```
# #[cfg(feature = "aws_lc_rs")] {
# rustls::crypto::aws_lc_rs::default_provider().install_default();
use rustls::{ClientConfig, ServerConfig};
ClientConfig::builder()
//  ...
# ;

ServerConfig::builder()
//  ...
# ;
# }
```

You may also override the choice of protocol versions:

```no_run
# #[cfg(feature = "aws_lc_rs")] {
# rustls::crypto::aws_lc_rs::default_provider().install_default();
# use rustls::ServerConfig;
ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
//  ...
# ;
# }
```

Overriding the default cryptographic provider introduces a `Result` that must be unwrapped,
because the config builder checks for consistency of the choices made. For instance, it's an error to
configure only TLS 1.2 cipher suites while specifying that TLS 1.3 should be the only supported protocol
version.

If you configure a smaller set of protocol primitives than the default, you may get a smaller binary,
since the code for the unused ones can be optimized away at link time.

After choosing protocol primitives, you must choose (a) how to verify certificates and (b) what certificates
(if any) to send to the peer. The methods to do this are specific to whether you're building a ClientConfig
or a ServerConfig, as tracked by the [`ConfigSide`](#configside) type parameter on the various impls of ConfigBuilder.

# ClientConfig certificate configuration

For a client, _certificate verification_ must be configured either by calling one of:
 - [`ConfigBuilder::with_root_certificates`](#with-root-certificates) or
 - [`ConfigBuilder::dangerous()`](#dangerous) and [`DangerousClientConfigBuilder::with_custom_certificate_verifier`](#with-custom-certificate-verifier)

Next, _certificate sending_ (also known as "client authentication", "mutual TLS", or "mTLS") must be configured
or disabled using one of:
- [`ConfigBuilder::with_no_client_auth`](#with-no-client-auth) - to not send client authentication (most common)
- [`ConfigBuilder::with_client_auth_cert`](#with-client-auth-cert) - to always send a specific certificate
- [`ConfigBuilder::with_client_cert_resolver`](#with-client-cert-resolver) - to send a certificate chosen dynamically

For example:

```
# #[cfg(feature = "aws_lc_rs")] {
# rustls::crypto::aws_lc_rs::default_provider().install_default();
# use rustls::ClientConfig;
# let root_certs = rustls::RootCertStore::empty();
ClientConfig::builder()
    .with_root_certificates(root_certs)
    .with_no_client_auth();
# }
```

# ServerConfig certificate configuration

For a server, _certificate verification_ must be configured by calling one of:
- [`ConfigBuilder::with_no_client_auth`](#with-no-client-auth) - to not require client authentication (most common)
- [`ConfigBuilder::with_client_cert_verifier`](#with-client-cert-verifier) - to use a custom verifier

Next, _certificate sending_ must be configured by calling one of:
- [`ConfigBuilder::with_single_cert`](#with-single-cert) - to send a specific certificate
- [`ConfigBuilder::with_single_cert_with_ocsp`](#with-single-cert-with-ocsp) - to send a specific certificate, plus stapled OCSP
- [`ConfigBuilder::with_cert_resolver`](#with-cert-resolver) - to send a certificate chosen dynamically

For example:

```no_run
# #[cfg(feature = "aws_lc_rs")] {
# rustls::crypto::aws_lc_rs::default_provider().install_default();
# use rustls::ServerConfig;
# let certs = vec![];
# let private_key = pki_types::PrivateKeyDer::from(
#    pki_types::PrivatePkcs8KeyDer::from(vec![])
# );
ServerConfig::builder()
    .with_no_client_auth()
    .with_single_cert(certs, private_key)
    .expect("bad certificate/key");
# }
```

# Types

ConfigBuilder uses the [typestate](#typestate)
 pattern to ensure at compile time that each required
configuration item is provided exactly once. This is tracked in the `State` type parameter,
which can have these values:

- [`WantsVersions`](#wantsversions)
- [`WantsVerifier`](#wantsverifier)
- [`WantsClientCert`](#wantsclientcert)
- [`WantsServerCert`](#wantsservercert)

The other type parameter is `Side`, which is either `ServerConfig` or `ClientConfig`
depending on whether the ConfigBuilder was built with [`ServerConfig::builder()`](#builder) or
[`ClientConfig::builder()`](#builder).

You won't need to write out either of these type parameters explicitly. If you write a
correct chain of configuration calls they will be used automatically. If you write an
incorrect chain of configuration calls you will get an error message from the compiler
mentioning some of these types.

Additionally, ServerConfig and ClientConfig carry a private field containing a
[`CryptoProvider`](rustls/crypto/index.md), from [`ClientConfig::builder_with_provider()`](#builder-with-provider) or
[`ServerConfig::builder_with_provider()`](#builder-with-provider). This determines which cryptographic backend
is used. The default is [the process-default provider](`CryptoProvider::get_default`).

[builder](#builder)
: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
[typestate](#typestate)
: http://cliffle.com/blog/rust-typestate/














#### Implementations

- `fn crypto_provider(self: &Self) -> &alloc::sync::Arc<CryptoProvider>`
  Return the crypto provider used to construct this builder.

- `fn with_single_cert(self: Self, cert_chain: Vec<CertificateDer<'static>>, key_der: PrivateKeyDer<'static>) -> Result<ServerConfig, Error>`
  Sets a single certificate chain and matching private key.  This

- `fn with_single_cert_with_ocsp(self: Self, cert_chain: Vec<CertificateDer<'static>>, key_der: PrivateKeyDer<'static>, ocsp: Vec<u8>) -> Result<ServerConfig, Error>`
  Sets a single certificate chain, matching private key and optional OCSP

- `fn with_cert_resolver(self: Self, cert_resolver: alloc::sync::Arc<dyn ResolvesServerCert>) -> ServerConfig`
  Sets a custom [`ResolvesServerCert`].

- `fn with_client_cert_verifier(self: Self, client_cert_verifier: alloc::sync::Arc<dyn ClientCertVerifier>) -> ConfigBuilder<ServerConfig, WantsServerCert>`
  Choose how to verify client certificates.

- `fn with_no_client_auth(self: Self) -> ConfigBuilder<ServerConfig, WantsServerCert>`
  Disable client authentication.

- `fn with_client_auth_cert(self: Self, cert_chain: Vec<CertificateDer<'static>>, key_der: PrivateKeyDer<'static>) -> Result<ClientConfig, Error>`
  Sets a single certificate chain and matching private key for use

- `fn with_no_client_auth(self: Self) -> ClientConfig`
  Do not support client auth.

- `fn with_client_cert_resolver(self: Self, client_auth_cert_resolver: alloc::sync::Arc<dyn ResolvesClientCert>) -> ClientConfig`
  Sets a custom [`ResolvesClientCert`].

- `fn with_ech(self: Self, mode: EchMode) -> Result<ConfigBuilder<ClientConfig, WantsVerifier>, Error>`
  Enable Encrypted Client Hello (ECH) in the given mode.

- `fn with_root_certificates(self: Self, root_store: impl Into<alloc::sync::Arc<webpki::RootCertStore>>) -> ConfigBuilder<ClientConfig, WantsClientCert>`
  Choose how to verify server certificates.

- `fn with_webpki_verifier(self: Self, verifier: alloc::sync::Arc<WebPkiServerVerifier>) -> ConfigBuilder<ClientConfig, WantsClientCert>`
  Choose how to verify server certificates using a webpki verifier.

- `fn dangerous(self: Self) -> danger::DangerousClientConfigBuilder`
  Access configuration options whose use is dangerous and requires

- `fn with_safe_default_protocol_versions(self: Self) -> Result<ConfigBuilder<S, WantsVerifier>, Error>`
  Accept the default protocol versions: both TLS1.2 and TLS1.3 are enabled.

- `fn with_protocol_versions(self: Self, versions: &[&'static versions::SupportedProtocolVersion]) -> Result<ConfigBuilder<S, WantsVerifier>, Error>`
  Use a specific set of protocol versions.

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

##### `impl Clone<Side: $crate::clone::Clone + ConfigSide, State: $crate::clone::Clone>`

- `fn clone(self: &Self) -> ConfigBuilder<Side, State>`

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

##### `impl Debug<Side: ConfigSide, State: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WantsVerifier`

```rust
struct WantsVerifier {
}
```

Config builder state where the caller must supply a verifier.

For more information, see the [`ConfigBuilder`](../ureq/ureq/config/index.md) documentation.

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

- `fn clone(self: &Self) -> WantsVerifier`

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

### `WantsVersions`

```rust
struct WantsVersions {
}
```

Config builder state where the caller must supply TLS protocol versions.

For more information, see the [`ConfigBuilder`](../ureq/ureq/config/index.md) documentation.

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

- `fn clone(self: &Self) -> WantsVersions`

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

### `CommonState`

```rust
struct CommonState {
}
```

Connection state common to both client and server connections.

#### Implementations

- `fn wants_write(self: &Self) -> bool`
  Returns true if the caller should call [`Connection::write_tls`] as soon as possible.

- `fn is_handshaking(self: &Self) -> bool`
  Returns true if the connection is currently performing the TLS handshake.

- `fn peer_certificates(self: &Self) -> Option<&[CertificateDer<'static>]>`
  Retrieves the certificate chain or the raw public key used by the peer to authenticate.

- `fn alpn_protocol(self: &Self) -> Option<&[u8]>`
  Retrieves the protocol agreed with the peer via ALPN.

- `fn negotiated_cipher_suite(self: &Self) -> Option<SupportedCipherSuite>`
  Retrieves the ciphersuite agreed with the peer.

- `fn negotiated_key_exchange_group(self: &Self) -> Option<&'static dyn SupportedKxGroup>`
  Retrieves the key exchange group agreed with the peer.

- `fn protocol_version(self: &Self) -> Option<ProtocolVersion>`
  Retrieves the protocol version agreed with the peer.

- `fn handshake_kind(self: &Self) -> Option<HandshakeKind>`
  Which kind of handshake was performed.

- `fn send_close_notify(self: &mut Self)`
  Queues a `close_notify` warning alert to be sent in the next

- `fn wants_read(self: &Self) -> bool`
  Returns true if the caller should call [`Connection::read_tls`] as soon

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

### `IoState`

```rust
struct IoState {
}
```

Values of this structure are returned from [`Connection::process_new_packets`](#process-new-packets)
and tell the caller the current I/O state of the TLS connection.


#### Implementations

- `fn tls_bytes_to_write(self: &Self) -> usize`
  How many bytes could be written by [`Connection::write_tls`] if called

- `fn plaintext_bytes_to_read(self: &Self) -> usize`
  How many plaintext bytes could be obtained via [`std::io::Read`]

- `fn peer_has_closed(self: &Self) -> bool`
  True if the peer has sent us a close_notify alert.  This is

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &IoState) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Reader<'a>`

```rust
struct Reader<'a> {
}
```

A structure that implements [`std::io::Read`](#read) for reading plaintext.

#### Implementations

- `fn into_first_chunk(self: Self) -> io::Result<&'a [u8]>`
  Obtain a chunk of plaintext data received from the peer over this TLS connection.

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

##### `impl BufRead`

- `fn fill_buf(self: &mut Self) -> io::Result<&[u8]>`
  Obtain a chunk of plaintext data received from the peer over this TLS connection.

- `fn consume(self: &mut Self, amt: usize)`

##### `impl Read`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`
  Obtain plaintext data received from the peer over this TLS connection.

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Writer<'a>`

```rust
struct Writer<'a> {
}
```

A structure that implements [`std::io::Write`](#write) for writing plaintext.

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

##### `impl Write`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`
  Send the plaintext `buf` to the peer, encrypting

- `fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

### `ConnectionCommon<Data>`

```rust
struct ConnectionCommon<Data> {
}
```

Interface shared by client and server connections.

#### Implementations

- `fn reader(self: &mut Self) -> Reader<'_>`
  Returns an object that allows reading plaintext.

- `fn writer(self: &mut Self) -> Writer<'_>`
  Returns an object that allows writing plaintext.

- `fn complete_io<T>(self: &mut Self, io: &mut T) -> Result<(usize, usize), io::Error>`
  This function uses `io` to complete any outstanding IO for

- `fn read_tls(self: &mut Self, rd: &mut dyn io::Read) -> Result<usize, io::Error>`
  Read TLS content from `rd` into the internal buffer.

- `fn write_tls(self: &mut Self, wr: &mut dyn io::Write) -> Result<usize, io::Error>`
  Writes TLS messages to `wr`.

- `fn process_new_packets(self: &mut Self) -> Result<IoState, Error>`
  Processes any new packets read by a previous call to

- `fn export_keying_material<T: AsMut<[u8]>>(self: &Self, output: T, label: &[u8], context: Option<&[u8]>) -> Result<T, Error>`
  Derives key material from the agreed connection secrets.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

- `fn set_buffer_limit(self: &mut Self, limit: Option<usize>)`
  Sets a limit on the internal buffers used to buffer

- `fn refresh_traffic_keys(self: &mut Self) -> Result<(), Error>`
  Sends a TLS1.3 `key_update` message to refresh a connection's keys.

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

##### `impl DerefMut<T>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `OtherError`

```rust
struct OtherError(alloc::sync::Arc<dyn StdError + Send + Sync>);
```

Any other error that cannot be expressed by a more specific [`Error`](../docs_md/docs_md/error/index.md) variant.

For example, an `OtherError` could be produced by a custom crypto provider
exposing a provider specific error.

Enums holding this type will never compare equal to each other.

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

- `fn clone(self: &Self) -> OtherError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

- `fn source(self: &Self) -> Option<&dyn StdError>`

##### `impl PartialEq`

- `fn eq(self: &Self, _other: &Self) -> bool`

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

### `NoKeyLog`

```rust
struct NoKeyLog;
```

KeyLog that does exactly nothing.

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

##### `impl KeyLog`

- `fn log(self: &Self, _: &str, _: &[u8], _: &[u8])`

- `fn will_log(self: &Self, _label: &str) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `KeyLogFile`

```rust
struct KeyLogFile();
```

[`KeyLog`](#keylog) implementation that opens a file whose name is
given by the `SSLKEYLOGFILE` environment variable, and writes
keys into it.

If `SSLKEYLOGFILE` is not set, this does nothing.

If such a file cannot be opened, or cannot be written then
this does nothing but logs errors at warning-level.

#### Implementations

- `fn new() -> Self`
  Makes a new `KeyLogFile`.  The environment variable is

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

##### `impl KeyLog`

- `fn log(self: &Self, label: &str, client_random: &[u8], secret: &[u8])`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `DistinguishedName`

```rust
struct DistinguishedName();
```

A `DistinguishedName` is a `Vec<u8>` wrapped in internal types.

It contains the DER or BER encoded [`Subject` field from RFC 5280](https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.6)
for a single certificate. The Subject field is [encoded as an RFC 5280 `Name`](https://datatracker.ietf.org/doc/html/rfc5280#page-116).
It can be decoded using [x509-parser's FromDer trait](https://docs.rs/x509-parser/latest/x509_parser/prelude/trait.FromDer.html).

```ignore
for name in distinguished_names {
    use x509_parser::prelude::FromDer;
    println!("{}", x509_parser::x509::X509Name::from_der(&name.0)?.1);
}
```

The TLS encoding is defined in RFC5246: `opaque DistinguishedName<1..2^16-1>;`

#### Implementations

- `fn in_sequence(bytes: &[u8]) -> Self`
  Create a [`DistinguishedName`] after prepending its outer SEQUENCE encoding.

#### Trait Implementations

##### `impl From`

- `fn from(v: Vec<u8>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> DistinguishedName`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, InvalidMessage>`

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

### `Stream<'a, C: 'a + ?Sized, T: 'a + Read + Write + ?Sized>`

```rust
struct Stream<'a, C: 'a + ?Sized, T: 'a + Read + Write + ?Sized> {
    pub conn: &'a mut C,
    pub sock: &'a mut T,
}
```

This type implements `io::Read` and `io::Write`, encapsulating
a Connection `C` and an underlying transport `T`, such as a socket.

Relies on [`ConnectionCommon::complete_io()`](#complete-io) to perform the necessary I/O.

This allows you to use a rustls Connection like a normal stream.

#### Fields

- **`conn`**: `&'a mut C`

  Our TLS connection

- **`sock`**: `&'a mut T`

  The underlying transport, like a socket

#### Implementations

- `fn new(conn: &'a mut C, sock: &'a mut T) -> Self`
  Make a new Stream using the Connection `conn` and socket-like object

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

##### `impl BufRead<'a, C, T, S>`

- `fn fill_buf(self: &mut Self) -> Result<&[u8]>`

- `fn consume(self: &mut Self, amt: usize)`

##### `impl Read<'a, C, T, S>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<'a, C, T, S>`

- `fn write(self: &mut Self, buf: &[u8]) -> Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[IoSlice<'_>]) -> Result<usize>`

- `fn flush(self: &mut Self) -> Result<()>`

##### `impl Debug<'a, C: $crate::fmt::Debug + 'a + ?Sized, T: $crate::fmt::Debug + 'a + Read + Write + ?Sized>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StreamOwned<C: Sized, T: Read + Write + Sized>`

```rust
struct StreamOwned<C: Sized, T: Read + Write + Sized> {
    pub conn: C,
    pub sock: T,
}
```

This type implements `io::Read` and `io::Write`, encapsulating
and owning a Connection `C` and an underlying transport `T`, such as a socket.

Relies on [`ConnectionCommon::complete_io()`](#complete-io) to perform the necessary I/O.

This allows you to use a rustls Connection like a normal stream.

#### Fields

- **`conn`**: `C`

  Our connection

- **`sock`**: `T`

  The underlying transport, like a socket

#### Implementations

- `fn new(conn: C, sock: T) -> Self`
  Make a new StreamOwned taking the Connection `conn` and socket-like

- `fn get_ref(self: &Self) -> &T`
  Get a reference to the underlying socket

- `fn get_mut(self: &mut Self) -> &mut T`
  Get a mutable reference to the underlying socket

- `fn into_parts(self: Self) -> (C, T)`
  Extract the `conn` and `sock` parts from the `StreamOwned`

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

##### `impl BufRead<C, T, S>`

- `fn fill_buf(self: &mut Self) -> Result<&[u8]>`

- `fn consume(self: &mut Self, amt: usize)`

##### `impl Read<C, T, S>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<C, T, S>`

- `fn write(self: &mut Self, buf: &[u8]) -> Result<usize>`

- `fn flush(self: &mut Self) -> Result<()>`

##### `impl Debug<C: $crate::fmt::Debug + Sized, T: $crate::fmt::Debug + Read + Write + Sized>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CipherSuiteCommon`

```rust
struct CipherSuiteCommon {
    pub suite: crate::enums::CipherSuite,
    pub hash_provider: &'static dyn crypto::hash::Hash,
    pub confidentiality_limit: u64,
}
```

Common state for cipher suites (both for TLS 1.2 and TLS 1.3)

#### Fields

- **`suite`**: `crate::enums::CipherSuite`

  The TLS enumeration naming this cipher suite.

- **`hash_provider`**: `&'static dyn crypto::hash::Hash`

  Which hash function the suite uses.

- **`confidentiality_limit`**: `u64`

  Number of TCP-TLS messages that can be safely encrypted with a single key of this type
  
  Once a `MessageEncrypter` produced for this suite has encrypted more than
  `confidentiality_limit` messages, an attacker gains an advantage in distinguishing it
  from an ideal pseudorandom permutation (PRP).
  
  This is to be set on the assumption that messages are maximally sized --
  each is 2<sup>14</sup> bytes. It **does not** consider confidentiality limits for
  QUIC connections - see the [`quic::PacketKey::confidentiality_limit`](#confidentiality-limit) field for
  this context.
  
  For AES-GCM implementations, this should be set to 2<sup>24</sup> to limit attack
  probability to one in 2<sup>60</sup>.  See [AEBounds] (Table 1) and [draft-irtf-aead-limits-08]:
  
  ```python
  >>> p = 2 ** -60
  >>> L = (2 ** 14 // 16) + 1
  >>> qlim = (math.sqrt(p) * (2 ** (129 // 2)) - 1) / (L + 1)
  >>> print(int(qlim).bit_length())
  24
  ```
  [AEBounds]: https://eprint.iacr.org/2024/051.pdf
  [draft-irtf-aead-limits-08]: https://www.ietf.org/archive/id/draft-irtf-cfrg-aead-limits-08.html#section-5.1.1
  
  For chacha20-poly1305 implementations, this should be set to `u64::MAX`:
  see <https://www.ietf.org/archive/id/draft-irtf-cfrg-aead-limits-08.html#section-5.2.1>

#### Implementations

- `fn fips(self: &Self) -> bool`
  Return `true` if this is backed by a FIPS-approved implementation.

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

### `ExtractedSecrets`

```rust
struct ExtractedSecrets {
    pub tx: (u64, ConnectionTrafficSecrets),
    pub rx: (u64, ConnectionTrafficSecrets),
}
```

Secrets for transmitting/receiving data over a TLS session.

After performing a handshake with rustls, these secrets can be extracted
to configure kTLS for a socket, and have the kernel take over encryption
and/or decryption.

#### Fields

- **`tx`**: `(u64, ConnectionTrafficSecrets)`

  sequence number and secrets for the "tx" (transmit) direction

- **`rx`**: `(u64, ConnectionTrafficSecrets)`

  sequence number and secrets for the "rx" (receive) direction

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

### `TicketRotator`

```rust
struct TicketRotator {
}
```

A ticketer that has a 'current' sub-ticketer and a single
'previous' ticketer.  It creates a new ticketer every so
often, demoting the current ticketer.

#### Implementations

- `fn new(lifetime: u32, generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>) -> Result<Self, Error>`
  Creates a new `TicketRotator`, which rotates through sub-ticketers

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

##### `impl ProducesTickets`

- `fn lifetime(self: &Self) -> u32`

- `fn enabled(self: &Self) -> bool`

- `fn encrypt(self: &Self, message: &[u8]) -> Option<Vec<u8>>`

- `fn decrypt(self: &Self, ciphertext: &[u8]) -> Option<Vec<u8>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `TicketSwitcher`

```rust
struct TicketSwitcher {
}
```

A ticketer that has a 'current' sub-ticketer and a single
'previous' ticketer.  It creates a new ticketer every so
often, demoting the current ticketer.

#### Implementations

- `fn new(lifetime: u32, generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>) -> Result<Self, Error>`
  Creates a new `TicketSwitcher`, which rotates through sub-ticketers

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

##### `impl ProducesTickets`

- `fn lifetime(self: &Self) -> u32`

- `fn enabled(self: &Self) -> bool`

- `fn encrypt(self: &Self, message: &[u8]) -> Option<Vec<u8>>`

- `fn decrypt(self: &Self, ciphertext: &[u8]) -> Option<Vec<u8>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Tls12CipherSuite`

```rust
struct Tls12CipherSuite {
    pub common: crate::suites::CipherSuiteCommon,
    pub prf_provider: &'static dyn crypto::tls12::Prf,
    pub kx: crate::msgs::handshake::KeyExchangeAlgorithm,
    pub sign: &'static [crate::enums::SignatureScheme],
    pub aead_alg: &'static dyn Tls12AeadAlgorithm,
}
```

A TLS 1.2 cipher suite supported by rustls.

#### Fields

- **`common`**: `crate::suites::CipherSuiteCommon`

  Common cipher suite fields.

- **`prf_provider`**: `&'static dyn crypto::tls12::Prf`

  How to compute the TLS1.2 PRF for the suite's hash function.
  
  If you have a TLS1.2 PRF implementation, you should directly implement the [`crypto::tls12::Prf`](#prf) trait.
  
  If not, you can implement the [`crypto::hmac::Hmac`](#hmac) trait (and associated), and then use
  [`crypto::tls12::PrfUsingHmac`](#prfusinghmac).

- **`kx`**: `crate::msgs::handshake::KeyExchangeAlgorithm`

  How to exchange/agree keys.
  
  In TLS1.2, the key exchange method (eg, Elliptic Curve Diffie-Hellman with Ephemeral keys -- ECDHE)
  is baked into the cipher suite, but the details to achieve it are negotiated separately.
  
  This controls how protocol messages (like the `ClientKeyExchange` message) are interpreted
  once this cipher suite has been negotiated.

- **`sign`**: `&'static [crate::enums::SignatureScheme]`

  How to sign messages for authentication.
  
  This is a set of [`SignatureScheme`](#signaturescheme)s that are usable once this cipher suite has been
  negotiated.
  
  The precise scheme used is then chosen from this set by the selected authentication key.

- **`aead_alg`**: `&'static dyn Tls12AeadAlgorithm`

  How to produce a [`MessageDecrypter`](rustls/crypto/cipher/index.md) or [`MessageEncrypter`](rustls/crypto/cipher/index.md)
  from raw key material.

#### Implementations

- `fn resolve_sig_schemes(self: &Self, offered: &[SignatureScheme]) -> Vec<SignatureScheme>`
  Resolve the set of supported [`SignatureScheme`]s from the

- `fn fips(self: &Self) -> bool`
  Return `true` if this is backed by a FIPS-approved implementation.

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

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Tls13CipherSuite`

```rust
struct Tls13CipherSuite {
    pub common: crate::suites::CipherSuiteCommon,
    pub hkdf_provider: &'static dyn crypto::tls13::Hkdf,
    pub aead_alg: &'static dyn crypto::cipher::Tls13AeadAlgorithm,
    pub quic: Option<&'static dyn crate::quic::Algorithm>,
}
```

A TLS 1.3 cipher suite supported by rustls.

#### Fields

- **`common`**: `crate::suites::CipherSuiteCommon`

  Common cipher suite fields.

- **`hkdf_provider`**: `&'static dyn crypto::tls13::Hkdf`

  How to complete HKDF with the suite's hash function.
  
  If you have a HKDF implementation, you should directly implement the `crypto::tls13::Hkdf`
  trait (and associated).
  
  If not, you can implement the [`crypto::hmac::Hmac`](#hmac) trait (and associated), and then use
  [`crypto::tls13::HkdfUsingHmac`](#hkdfusinghmac).

- **`aead_alg`**: `&'static dyn crypto::cipher::Tls13AeadAlgorithm`

  How to produce a [MessageDecrypter] or [MessageEncrypter]
  from raw key material.
  
  [MessageDecrypter]: crate::crypto::cipher::MessageDecrypter
  [MessageEncrypter]: crate::crypto::cipher::MessageEncrypter

- **`quic`**: `Option<&'static dyn crate::quic::Algorithm>`

  How to create QUIC header and record protection algorithms
  for this suite.
  
  Provide `None` to opt out of QUIC support for this suite.  It will
  not be offered in QUIC handshakes.

#### Implementations

- `fn can_resume_from(self: &Self, prev: &'static Self) -> Option<&'static Self>`
  Can a session using suite self resume from suite prev?

- `fn fips(self: &Self) -> bool`
  Return `true` if this is backed by a FIPS-approved implementation.

- `fn quic_suite(self: &'static Self) -> Option<crate::quic::Suite>`
  Returns a `quic::Suite` for the ciphersuite, if supported.

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

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DigitallySignedStruct`

```rust
struct DigitallySignedStruct {
    pub scheme: crate::enums::SignatureScheme,
}
```

This type combines a [`SignatureScheme`](#signaturescheme) and a signature payload produced with that scheme.

#### Fields

- **`scheme`**: `crate::enums::SignatureScheme`

  The [`SignatureScheme`](#signaturescheme) used to produce the signature.

#### Implementations

- `fn signature(self: &Self) -> &[u8]`
  Get the signature.

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

- `fn clone(self: &Self) -> DigitallySignedStruct`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, InvalidMessage>`

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

### `SupportedProtocolVersion`

```rust
struct SupportedProtocolVersion {
    pub version: crate::enums::ProtocolVersion,
}
```

A TLS protocol version supported by rustls.

All possible instances of this class are provided by the library in
the [`ALL_VERSIONS`](#all-versions) array, as well as individually as [`TLS12`](#tls12)
and [`TLS13`](#tls13).

#### Fields

- **`version`**: `crate::enums::ProtocolVersion`

  The TLS enumeration naming this version.

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SupportedProtocolVersion) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RootCertStore`

```rust
struct RootCertStore {
    pub roots: alloc::vec::Vec<pki_types::TrustAnchor<'static>>,
}
```

A container for root certificates able to provide a root-of-trust
for connection authentication.

#### Fields

- **`roots`**: `alloc::vec::Vec<pki_types::TrustAnchor<'static>>`

  The list of roots.

#### Implementations

- `fn empty() -> Self`
  Make a new, empty `RootCertStore`.

- `fn add_parsable_certificates<'a>(self: &mut Self, der_certs: impl IntoIterator<Item = CertificateDer<'a>>) -> (usize, usize)`
  Parse the given DER-encoded certificates and add all that can be parsed

- `fn add(self: &mut Self, der: CertificateDer<'_>) -> Result<(), Error>`
  Add a single DER-encoded certificate to the store.

- `fn subjects(self: &Self) -> Vec<DistinguishedName>`
  Return the DER encoded [`DistinguishedName`] of each trust anchor subject in the root

- `fn is_empty(self: &Self) -> bool`
  Return true if there are no certificates.

- `fn len(self: &Self) -> usize`
  Say how many certificates are in the container.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator`

- `fn from_iter<T: IntoIterator<Item = TrustAnchor<'static>>>(iter: T) -> Self`

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

- `fn clone(self: &Self) -> RootCertStore`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Extend`

- `fn extend<T: IntoIterator<Item = TrustAnchor<'static>>>(self: &mut Self, iter: T)`

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

### `ClientConfig`

```rust
struct ClientConfig {
    pub alpn_protocols: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    pub resumption: Resumption,
    pub max_fragment_size: Option<usize>,
    pub client_auth_cert_resolver: alloc::sync::Arc<dyn ResolvesClientCert>,
    pub enable_sni: bool,
    pub key_log: alloc::sync::Arc<dyn KeyLog>,
    pub enable_secret_extraction: bool,
    pub enable_early_data: bool,
    pub require_ems: bool,
    pub time_provider: alloc::sync::Arc<dyn TimeProvider>,
    pub cert_decompressors: alloc::vec::Vec<&'static dyn compress::CertDecompressor>,
    pub cert_compressors: alloc::vec::Vec<&'static dyn compress::CertCompressor>,
    pub cert_compression_cache: alloc::sync::Arc<compress::CompressionCache>,
}
```

Common configuration for (typically) all connections made by a program.

Making one of these is cheap, though one of the inputs may be expensive: gathering trust roots
from the operating system to add to the [`RootCertStore`](#rootcertstore) passed to `with_root_certificates()`
(the rustls-native-certs crate is often used for this) may take on the order of a few hundred
milliseconds.

These must be created via the [`ClientConfig::builder()`](#builder) or [`ClientConfig::builder_with_provider()`](#builder-with-provider)
function.

Note that using [`ConfigBuilder<ClientConfig, WantsVersions>::with_ech()`](#with-ech) will produce a common
configuration specific to the provided [`crate::client::EchConfig`](#echconfig) that may not be appropriate
for all connections made by the program. In this case the configuration should only be shared
by connections intended for domains that offer the provided [`crate::client::EchConfig`](#echconfig) in
their DNS zone.

# Defaults

* [`ClientConfig::max_fragment_size`](#max-fragment-size): the default is `None` (meaning 16kB).
* [`ClientConfig::resumption`](#resumption): supports resumption with up to 256 server names, using session
  ids or tickets, with a max of eight tickets per server.
* [`ClientConfig::alpn_protocols`](#alpn-protocols): the default is empty -- no ALPN protocol is negotiated.
* [`ClientConfig::key_log`](#key-log): key material is not logged.
* [`ClientConfig::cert_decompressors`](#cert-decompressors): depends on the crate features, see [`compress::default_cert_decompressors()`](#default-cert-decompressors).
* [`ClientConfig::cert_compressors`](#cert-compressors): depends on the crate features, see [`compress::default_cert_compressors()`](#default-cert-compressors).
* [`ClientConfig::cert_compression_cache`](#cert-compression-cache): caches the most recently used 4 compressions


#### Fields

- **`alpn_protocols`**: `alloc::vec::Vec<alloc::vec::Vec<u8>>`

  Which ALPN protocols we include in our client hello.
  If empty, no ALPN extension is sent.

- **`resumption`**: `Resumption`

  How and when the client can resume a previous session.
  
  # Sharing `resumption` between `ClientConfig`s
  In a program using many `ClientConfig`s it may improve resumption rates
  (which has a significant impact on connection performance) if those
  configs share a single `Resumption`.
  
  However, resumption is only allowed between two `ClientConfig`s if their
  `client_auth_cert_resolver` (ie, potential client authentication credentials)
  and `verifier` (ie, server certificate verification settings) are
  the same (according to `Arc::ptr_eq`).
  
  To illustrate, imagine two `ClientConfig`s `A` and `B`.  `A` fully validates
  the server certificate, `B` does not.  If `A` and `B` shared a resumption store,
  it would be possible for a session originated by `B` to be inserted into the
  store, and then resumed by `A`.  This would give a false impression to the user
  of `A` that the server certificate is fully validated.

- **`max_fragment_size`**: `Option<usize>`

  The maximum size of plaintext input to be emitted in a single TLS record.
  A value of None is equivalent to the [TLS maximum] of 16 kB.
  
  rustls enforces an arbitrary minimum of 32 bytes for this field.
  Out of range values are reported as errors from [ClientConnection::new].
  
  Setting this value to a little less than the TCP MSS may improve latency
  for stream-y workloads.
  
  [TLS maximum]: https://datatracker.ietf.org/doc/html/rfc8446#section-5.1
  [ClientConnection::new]: crate::client::ClientConnection::new

- **`client_auth_cert_resolver`**: `alloc::sync::Arc<dyn ResolvesClientCert>`

  How to decide what client auth certificate/keys to use.

- **`enable_sni`**: `bool`

  Whether to send the Server Name Indication (SNI) extension
  during the client handshake.
  
  The default is true.

- **`key_log`**: `alloc::sync::Arc<dyn KeyLog>`

  How to output key material for debugging.  The default
  does nothing.

- **`enable_secret_extraction`**: `bool`

  Allows traffic secrets to be extracted after the handshake,
  e.g. for kTLS setup.

- **`enable_early_data`**: `bool`

  Whether to send data on the first flight ("early data") in
  TLS 1.3 handshakes.
  
  The default is false.

- **`require_ems`**: `bool`

  If set to `true`, requires the server to support the extended
  master secret extraction method defined in [RFC 7627].
  
  The default is `true` if the `fips` crate feature is enabled,
  `false` otherwise.
  
  It must be set to `true` to meet FIPS requirement mentioned in section
  **D.Q Transition of the TLS 1.2 KDF to Support the Extended Master
  Secret** from [FIPS 140-3 IG.pdf].
  
  [RFC 7627]: https://datatracker.ietf.org/doc/html/rfc7627
  [FIPS 140-3 IG.pdf]: https://csrc.nist.gov/csrc/media/Projects/cryptographic-module-validation-program/documents/fips%20140-3/FIPS%20140-3%20IG.pdf

- **`time_provider`**: `alloc::sync::Arc<dyn TimeProvider>`

  Provides the current system time

- **`cert_decompressors`**: `alloc::vec::Vec<&'static dyn compress::CertDecompressor>`

  How to decompress the server's certificate chain.
  
  If this is non-empty, the [RFC8779] certificate compression
  extension is offered, and any compressed certificates are
  transparently decompressed during the handshake.
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

- **`cert_compressors`**: `alloc::vec::Vec<&'static dyn compress::CertCompressor>`

  How to compress the client's certificate chain.
  
  If a server supports this extension, and advertises support
  for one of the compression algorithms included here, the
  client certificate will be compressed according to [RFC8779].
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

- **`cert_compression_cache`**: `alloc::sync::Arc<compress::CompressionCache>`

  Caching for compressed certificates.
  
  This is optional: [`compress::CompressionCache::Disabled`](#disabled) gives
  a cache that does no caching.

#### Implementations

- `fn builder() -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a client configuration with

- `fn builder_with_protocol_versions(versions: &[&'static versions::SupportedProtocolVersion]) -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a client configuration with

- `fn builder_with_provider(provider: alloc::sync::Arc<CryptoProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a client configuration with a specific [`CryptoProvider`].

- `fn builder_with_details(provider: alloc::sync::Arc<CryptoProvider>, time_provider: alloc::sync::Arc<dyn TimeProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a client configuration with no default implementation details.

- `fn fips(self: &Self) -> bool`
  Return true if connections made with this `ClientConfig` will

- `fn crypto_provider(self: &Self) -> &alloc::sync::Arc<CryptoProvider>`
  Return the crypto provider used to construct this client configuration.

- `fn dangerous(self: &mut Self) -> danger::DangerousClientConfig<'_>`
  Access configuration options whose use is dangerous and requires

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

- `fn clone(self: &Self) -> ClientConfig`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ConfigSide`

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

### `ClientConnection`

```rust
struct ClientConnection {
}
```

This represents a single TLS client connection.

#### Implementations

- `fn new(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>) -> Result<Self, Error>`
  Make a new ClientConnection.  `config` controls how

- `fn new_with_alpn(config: alloc::sync::Arc<ClientConfig>, name: ServerName<'static>, alpn_protocols: Vec<Vec<u8>>) -> Result<Self, Error>`
  Make a new ClientConnection with custom ALPN protocols.

- `fn early_data(self: &mut Self) -> Option<WriteEarlyData<'_>>`
  Returns an `io::Write` implementer you can write bytes to

- `fn is_early_data_accepted(self: &Self) -> bool`
  Returns True if the server signalled it will process early data.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

- `fn ech_status(self: &Self) -> EchStatus`
  Return the connection's Encrypted Client Hello (ECH) status.

- `fn tls13_tickets_received(self: &Self) -> u32`
  Returns the number of TLS1.3 tickets that have been received.

- `fn fips(self: &Self) -> bool`
  Return true if the connection was made with a `ClientConfig` that is FIPS compatible.

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref`

- `type Target = ConnectionCommon<ClientConnectionData>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `ServerConfig`

```rust
struct ServerConfig {
    pub ignore_client_order: bool,
    pub max_fragment_size: Option<usize>,
    pub session_storage: alloc::sync::Arc<dyn StoresServerSessions>,
    pub ticketer: alloc::sync::Arc<dyn ProducesTickets>,
    pub cert_resolver: alloc::sync::Arc<dyn ResolvesServerCert>,
    pub alpn_protocols: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    pub key_log: alloc::sync::Arc<dyn KeyLog>,
    pub enable_secret_extraction: bool,
    pub max_early_data_size: u32,
    pub send_half_rtt_data: bool,
    pub send_tls13_tickets: usize,
    pub require_ems: bool,
    pub time_provider: alloc::sync::Arc<dyn TimeProvider>,
    pub cert_compressors: alloc::vec::Vec<&'static dyn compress::CertCompressor>,
    pub cert_compression_cache: alloc::sync::Arc<compress::CompressionCache>,
    pub cert_decompressors: alloc::vec::Vec<&'static dyn compress::CertDecompressor>,
}
```

Common configuration for a set of server sessions.

Making one of these is cheap, though one of the inputs may be expensive: gathering trust roots
from the operating system to add to the [`RootCertStore`](#rootcertstore) passed to a `ClientCertVerifier`
builder may take on the order of a few hundred milliseconds.

These must be created via the [`ServerConfig::builder()`](#builder) or [`ServerConfig::builder_with_provider()`](#builder-with-provider)
function.

# Defaults

* [`ServerConfig::max_fragment_size`](#max-fragment-size): the default is `None` (meaning 16kB).
* [`ServerConfig::session_storage`](#session-storage): if the `std` feature is enabled, the default stores 256
  sessions in memory. If the `std` feature is not enabled, the default is to not store any
  sessions. In a no-std context, by enabling the `hashbrown` feature you may provide your
  own `session_storage` using [`ServerSessionMemoryCache`](#serversessionmemorycache) and a `crate::lock::MakeMutex`
  implementation.
* [`ServerConfig::alpn_protocols`](#alpn-protocols): the default is empty -- no ALPN protocol is negotiated.
* [`ServerConfig::key_log`](#key-log): key material is not logged.
* [`ServerConfig::send_tls13_tickets`](#send-tls13-tickets): 2 tickets are sent.
* [`ServerConfig::cert_compressors`](#cert-compressors): depends on the crate features, see [`compress::default_cert_compressors()`](#default-cert-compressors).
* [`ServerConfig::cert_compression_cache`](#cert-compression-cache): caches the most recently used 4 compressions
* [`ServerConfig::cert_decompressors`](#cert-decompressors): depends on the crate features, see [`compress::default_cert_decompressors()`](#default-cert-decompressors).

# Sharing resumption storage between `ServerConfig`s

In a program using many `ServerConfig`s it may improve resumption rates
(which has a significant impact on connection performance) if those
configs share [`ServerConfig::session_storage`](#session-storage) or [`ServerConfig::ticketer`](#ticketer).

However, caution is needed: other fields influence the security of a session
and resumption between them can be surprising.  If sharing
[`ServerConfig::session_storage`](#session-storage) or [`ServerConfig::ticketer`](#ticketer) between two
`ServerConfig`s, you should also evaluate the following fields and ensure
they are equivalent:

* `ServerConfig::verifier` -- client authentication requirements,
* [`ServerConfig::cert_resolver`](#cert-resolver) -- server identities.

To illustrate, imagine two `ServerConfig`s `A` and `B`.  `A` requires
client authentication, `B` does not.  If `A` and `B` shared a resumption store,
it would be possible for a session originated by `B` (that is, an unauthenticated client)
to be inserted into the store, and then resumed by `A`.  This would give a false
impression to the user of `A` that the client was authenticated.  This is possible
whether the resumption is performed statefully (via [`ServerConfig::session_storage`](#session-storage))
or statelessly (via [`ServerConfig::ticketer`](#ticketer)).

_Unlike_ `ClientConfig`, rustls does not enforce any policy here.



#### Fields

- **`ignore_client_order`**: `bool`

  Ignore the client's ciphersuite order. Instead,
  choose the top ciphersuite in the server list
  which is supported by the client.

- **`max_fragment_size`**: `Option<usize>`

  The maximum size of plaintext input to be emitted in a single TLS record.
  A value of None is equivalent to the [TLS maximum] of 16 kB.
  
  rustls enforces an arbitrary minimum of 32 bytes for this field.
  Out of range values are reported as errors from [ServerConnection::new].
  
  Setting this value to a little less than the TCP MSS may improve latency
  for stream-y workloads.
  
  [TLS maximum]: https://datatracker.ietf.org/doc/html/rfc8446#section-5.1
  [ServerConnection::new]: crate::server::ServerConnection::new

- **`session_storage`**: `alloc::sync::Arc<dyn StoresServerSessions>`

  How to store client sessions.
  
  See [ServerConfig#sharing-resumption-storage-between-serverconfigs]
  for a warning related to this field.

- **`ticketer`**: `alloc::sync::Arc<dyn ProducesTickets>`

  How to produce tickets.
  
  See [ServerConfig#sharing-resumption-storage-between-serverconfigs]
  for a warning related to this field.

- **`cert_resolver`**: `alloc::sync::Arc<dyn ResolvesServerCert>`

  How to choose a server cert and key. This is usually set by
  [ConfigBuilder::with_single_cert] or [ConfigBuilder::with_cert_resolver].
  For async applications, see also [Acceptor].

- **`alpn_protocols`**: `alloc::vec::Vec<alloc::vec::Vec<u8>>`

  Protocol names we support, most preferred first.
  If empty we don't do ALPN at all.

- **`key_log`**: `alloc::sync::Arc<dyn KeyLog>`

  How to output key material for debugging.  The default
  does nothing.

- **`enable_secret_extraction`**: `bool`

  Allows traffic secrets to be extracted after the handshake,
  e.g. for kTLS setup.

- **`max_early_data_size`**: `u32`

  Amount of early data to accept for sessions created by
  this config.  Specify 0 to disable early data.  The
  default is 0.
  
  Read the early data via [`ServerConnection::early_data`](#early-data).
  
  The units for this are _both_ plaintext bytes, _and_ ciphertext
  bytes, depending on whether the server accepts a client's early_data
  or not.  It is therefore recommended to include some slop in
  this value to account for the unknown amount of ciphertext
  expansion in the latter case.

- **`send_half_rtt_data`**: `bool`

  Whether the server should send "0.5RTT" data.  This means the server
  sends data after its first flight of handshake messages, without
  waiting for the client to complete the handshake.
  
  This can improve TTFB latency for either server-speaks-first protocols,
  or client-speaks-first protocols when paired with "0RTT" data.  This
  comes at the cost of a subtle weakening of the normal handshake
  integrity guarantees that TLS provides.  Note that the initial
  `ClientHello` is indirectly authenticated because it is included
  in the transcript used to derive the keys used to encrypt the data.
  
  This only applies to TLS1.3 connections.  TLS1.2 connections cannot
  do this optimisation and this setting is ignored for them.  It is
  also ignored for TLS1.3 connections that even attempt client
  authentication.
  
  This defaults to false.  This means the first application data
  sent by the server comes after receiving and validating the client's
  handshake up to the `Finished` message.  This is the safest option.

- **`send_tls13_tickets`**: `usize`

  How many TLS1.3 tickets to send immediately after a successful
  handshake.
  
  Because TLS1.3 tickets are single-use, this allows
  a client to perform multiple resumptions.
  
  The default is 2.
  
  If this is 0, no tickets are sent and clients will not be able to
  do any resumption.

- **`require_ems`**: `bool`

  If set to `true`, requires the client to support the extended
  master secret extraction method defined in [RFC 7627].
  
  The default is `true` if the "fips" crate feature is enabled,
  `false` otherwise.
  
  It must be set to `true` to meet FIPS requirement mentioned in section
  **D.Q Transition of the TLS 1.2 KDF to Support the Extended Master
  Secret** from [FIPS 140-3 IG.pdf].
  
  [RFC 7627]: https://datatracker.ietf.org/doc/html/rfc7627
  [FIPS 140-3 IG.pdf]: https://csrc.nist.gov/csrc/media/Projects/cryptographic-module-validation-program/documents/fips%20140-3/FIPS%20140-3%20IG.pdf

- **`time_provider`**: `alloc::sync::Arc<dyn TimeProvider>`

  Provides the current system time

- **`cert_compressors`**: `alloc::vec::Vec<&'static dyn compress::CertCompressor>`

  How to compress the server's certificate chain.
  
  If a client supports this extension, and advertises support
  for one of the compression algorithms included here, the
  server certificate will be compressed according to [RFC8779].
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

- **`cert_compression_cache`**: `alloc::sync::Arc<compress::CompressionCache>`

  Caching for compressed certificates.
  
  This is optional: [`compress::CompressionCache::Disabled`](#disabled) gives
  a cache that does no caching.

- **`cert_decompressors`**: `alloc::vec::Vec<&'static dyn compress::CertDecompressor>`

  How to decompress the clients's certificate chain.
  
  If this is non-empty, the [RFC8779] certificate compression
  extension is offered when requesting client authentication,
  and any compressed certificates are transparently decompressed
  during the handshake.
  
  This only applies to TLS1.3 connections.  It is ignored for
  TLS1.2 connections.
  
  [RFC8779]: https://datatracker.ietf.org/doc/rfc8879/

#### Implementations

- `fn builder() -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a server configuration with

- `fn builder_with_protocol_versions(versions: &[&'static versions::SupportedProtocolVersion]) -> ConfigBuilder<Self, WantsVerifier>`
  Create a builder for a server configuration with

- `fn builder_with_provider(provider: alloc::sync::Arc<CryptoProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a server configuration with a specific [`CryptoProvider`].

- `fn builder_with_details(provider: alloc::sync::Arc<CryptoProvider>, time_provider: alloc::sync::Arc<dyn TimeProvider>) -> ConfigBuilder<Self, WantsVersions>`
  Create a builder for a server configuration with no default implementation details.

- `fn fips(self: &Self) -> bool`
  Return `true` if connections made with this `ServerConfig` will

- `fn crypto_provider(self: &Self) -> &alloc::sync::Arc<CryptoProvider>`
  Return the crypto provider used to construct this client configuration.

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

- `fn clone(self: &Self) -> ServerConfig`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ConfigSide`

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

### `ServerConnection`

```rust
struct ServerConnection {
}
```

This represents a single TLS server connection.

Send TLS-protected data to the peer using the `io::Write` trait implementation.
Read data from the peer using the `io::Read` trait implementation.

#### Implementations

- `fn new(config: alloc::sync::Arc<ServerConfig>) -> Result<Self, Error>`
  Make a new ServerConnection.  `config` controls how

- `fn server_name(self: &Self) -> Option<&str>`
  Retrieves the server name, if any, used to select the certificate and

- `fn received_resumption_data(self: &Self) -> Option<&[u8]>`
  Application-controlled portion of the resumption ticket supplied by the client, if any.

- `fn set_resumption_data(self: &mut Self, data: &[u8])`
  Set the resumption data to embed in future resumption tickets supplied to the client.

- `fn reject_early_data(self: &mut Self)`
  Explicitly discard early data, notifying the client

- `fn early_data(self: &mut Self) -> Option<ReadEarlyData<'_>>`
  Returns an `io::Read` implementer you can read bytes from that are

- `fn fips(self: &Self) -> bool`
  Return true if the connection was made with a `ServerConfig` that is FIPS compatible.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Deref`

- `type Target = ConnectionCommon<ServerConnectionData>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

## Enums

### `HandshakeKind`

```rust
enum HandshakeKind {
    Full,
    FullWithHelloRetryRequest,
    Resumed,
}
```

Describes which sort of handshake happened.

#### Variants

- **`Full`**

  A full handshake.
  
  This is the typical TLS connection initiation process when resumption is
  not yet unavailable, and the initial `ClientHello` was accepted by the server.

- **`FullWithHelloRetryRequest`**

  A full TLS1.3 handshake, with an extra round-trip for a `HelloRetryRequest`.
  
  The server can respond with a `HelloRetryRequest` if the initial `ClientHello`
  is unacceptable for several reasons, the most likely if no supported key
  shares were offered by the client.

- **`Resumed`**

  A resumed handshake.
  
  Resumed handshakes involve fewer round trips and less cryptography than
  full ones, but can only happen when the peers have previously done a full
  handshake together, and then remember data about it.

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

- `fn clone(self: &Self) -> HandshakeKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HandshakeKind) -> bool`

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

### `Side`

```rust
enum Side {
    Client,
    Server,
}
```

Side of the connection.

#### Variants

- **`Client`**

  A client initiates the connection.

- **`Server`**

  A server waits for a client to connect.

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

- `fn clone(self: &Self) -> Side`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Side) -> bool`

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

### `Connection`

```rust
enum Connection {
    Client(crate::client::ClientConnection),
    Server(crate::server::ServerConnection),
}
```

A client or server connection.

#### Variants

- **`Client`**

  A client connection

- **`Server`**

  A server connection

#### Implementations

- `fn read_tls(self: &mut Self, rd: &mut dyn Read) -> Result<usize, io::Error>`
  Read TLS content from `rd`.

- `fn write_tls(self: &mut Self, wr: &mut dyn io::Write) -> Result<usize, io::Error>`
  Writes TLS messages to `wr`.

- `fn reader(self: &mut Self) -> Reader<'_>`
  Returns an object that allows reading plaintext.

- `fn writer(self: &mut Self) -> Writer<'_>`
  Returns an object that allows writing plaintext.

- `fn process_new_packets(self: &mut Self) -> Result<IoState, Error>`
  Processes any new packets read by a previous call to [`Connection::read_tls`].

- `fn export_keying_material<T: AsMut<[u8]>>(self: &Self, output: T, label: &[u8], context: Option<&[u8]>) -> Result<T, Error>`
  Derives key material from the agreed connection secrets.

- `fn complete_io<T>(self: &mut Self, io: &mut T) -> Result<(usize, usize), io::Error>`
  This function uses `io` to complete any outstanding IO for this connection.

- `fn dangerous_extract_secrets(self: Self) -> Result<ExtractedSecrets, Error>`
  Extract secrets, so they can be used when configuring kTLS, for example.

- `fn set_buffer_limit(self: &mut Self, limit: Option<usize>)`
  Sets a limit on the internal buffers

- `fn refresh_traffic_keys(self: &mut Self) -> Result<(), Error>`
  Sends a TLS1.3 `key_update` message to refresh a connection's keys

#### Trait Implementations

##### `impl From`

- `fn from(conn: ServerConnection) -> Self`

##### `impl From`

- `fn from(conn: ClientConnection) -> Self`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Deref`

- `type Target = CommonState`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

### `AlertDescription`

```rust
enum AlertDescription {
    CloseNotify,
    UnexpectedMessage,
    BadRecordMac,
    DecryptionFailed,
    RecordOverflow,
    DecompressionFailure,
    HandshakeFailure,
    NoCertificate,
    BadCertificate,
    UnsupportedCertificate,
    CertificateRevoked,
    CertificateExpired,
    CertificateUnknown,
    IllegalParameter,
    UnknownCA,
    AccessDenied,
    DecodeError,
    DecryptError,
    ExportRestriction,
    ProtocolVersion,
    InsufficientSecurity,
    InternalError,
    InappropriateFallback,
    UserCanceled,
    NoRenegotiation,
    MissingExtension,
    UnsupportedExtension,
    CertificateUnobtainable,
    UnrecognisedName,
    BadCertificateStatusResponse,
    BadCertificateHashValue,
    UnknownPSKIdentity,
    CertificateRequired,
    NoApplicationProtocol,
    EncryptedClientHelloRequired,
    Unknown(u8),
}
```

The `AlertDescription` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 1]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From`

- `fn from(e: InvalidMessage) -> Self`

##### `impl From`

- `fn from(x: u8) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: CertificateError) -> Self`

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

- `fn clone(self: &Self) -> AlertDescription`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AlertDescription) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CertificateCompressionAlgorithm`

```rust
enum CertificateCompressionAlgorithm {
    Zlib,
    Brotli,
    Zstd,
    Unknown(u16),
}
```

The "TLS Certificate Compression Algorithm IDs" TLS protocol enum.
Values in this enum are taken from [RFC8879].

[RFC8879]: https://www.rfc-editor.org/rfc/rfc8879.html#section-7.3

#### Implementations

- `fn to_array(self: Self) -> [u8; 2]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From`

- `fn from(x: u16) -> Self`

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

- `fn clone(self: &Self) -> CertificateCompressionAlgorithm`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CertificateCompressionAlgorithm) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CipherSuite`

```rust
enum CipherSuite {
    TLS_NULL_WITH_NULL_NULL,
    TLS_PSK_WITH_AES_128_GCM_SHA256,
    TLS_PSK_WITH_AES_256_GCM_SHA384,
    TLS_EMPTY_RENEGOTIATION_INFO_SCSV,
    TLS13_AES_128_GCM_SHA256,
    TLS13_AES_256_GCM_SHA384,
    TLS13_CHACHA20_POLY1305_SHA256,
    TLS13_AES_128_CCM_SHA256,
    TLS13_AES_128_CCM_8_SHA256,
    TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA,
    TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA,
    TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA,
    TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA,
    TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256,
    TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384,
    TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256,
    TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384,
    TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
    TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
    TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
    TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
    TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
    TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
    TLS_RSA_WITH_NULL_MD5,
    TLS_RSA_WITH_NULL_SHA,
    TLS_RSA_EXPORT_WITH_RC4_40_MD5,
    TLS_RSA_WITH_RC4_128_MD5,
    TLS_RSA_WITH_RC4_128_SHA,
    TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5,
    TLS_RSA_WITH_IDEA_CBC_SHA,
    TLS_RSA_EXPORT_WITH_DES40_CBC_SHA,
    TLS_RSA_WITH_DES_CBC_SHA,
    TLS_RSA_WITH_3DES_EDE_CBC_SHA,
    TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA,
    TLS_DH_DSS_WITH_DES_CBC_SHA,
    TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA,
    TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA,
    TLS_DH_RSA_WITH_DES_CBC_SHA,
    TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA,
    TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA,
    TLS_DHE_DSS_WITH_DES_CBC_SHA,
    TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA,
    TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA,
    TLS_DHE_RSA_WITH_DES_CBC_SHA,
    TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA,
    TLS_DH_anon_EXPORT_WITH_RC4_40_MD5,
    TLS_DH_anon_WITH_RC4_128_MD5,
    TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA,
    TLS_DH_anon_WITH_DES_CBC_SHA,
    TLS_DH_anon_WITH_3DES_EDE_CBC_SHA,
    SSL_FORTEZZA_KEA_WITH_NULL_SHA,
    SSL_FORTEZZA_KEA_WITH_FORTEZZA_CBC_SHA,
    TLS_KRB5_WITH_DES_CBC_SHA_or_SSL_FORTEZZA_KEA_WITH_RC4_128_SHA,
    TLS_KRB5_WITH_3DES_EDE_CBC_SHA,
    TLS_KRB5_WITH_RC4_128_SHA,
    TLS_KRB5_WITH_IDEA_CBC_SHA,
    TLS_KRB5_WITH_DES_CBC_MD5,
    TLS_KRB5_WITH_3DES_EDE_CBC_MD5,
    TLS_KRB5_WITH_RC4_128_MD5,
    TLS_KRB5_WITH_IDEA_CBC_MD5,
    TLS_KRB5_EXPORT_WITH_DES_CBC_40_SHA,
    TLS_KRB5_EXPORT_WITH_RC2_CBC_40_SHA,
    TLS_KRB5_EXPORT_WITH_RC4_40_SHA,
    TLS_KRB5_EXPORT_WITH_DES_CBC_40_MD5,
    TLS_KRB5_EXPORT_WITH_RC2_CBC_40_MD5,
    TLS_KRB5_EXPORT_WITH_RC4_40_MD5,
    TLS_PSK_WITH_NULL_SHA,
    TLS_DHE_PSK_WITH_NULL_SHA,
    TLS_RSA_PSK_WITH_NULL_SHA,
    TLS_RSA_WITH_AES_128_CBC_SHA,
    TLS_DH_DSS_WITH_AES_128_CBC_SHA,
    TLS_DH_RSA_WITH_AES_128_CBC_SHA,
    TLS_DHE_DSS_WITH_AES_128_CBC_SHA,
    TLS_DHE_RSA_WITH_AES_128_CBC_SHA,
    TLS_DH_anon_WITH_AES_128_CBC_SHA,
    TLS_RSA_WITH_AES_256_CBC_SHA,
    TLS_DH_DSS_WITH_AES_256_CBC_SHA,
    TLS_DH_RSA_WITH_AES_256_CBC_SHA,
    TLS_DHE_DSS_WITH_AES_256_CBC_SHA,
    TLS_DHE_RSA_WITH_AES_256_CBC_SHA,
    TLS_DH_anon_WITH_AES_256_CBC_SHA,
    TLS_RSA_WITH_NULL_SHA256,
    TLS_RSA_WITH_AES_128_CBC_SHA256,
    TLS_RSA_WITH_AES_256_CBC_SHA256,
    TLS_DH_DSS_WITH_AES_128_CBC_SHA256,
    TLS_DH_RSA_WITH_AES_128_CBC_SHA256,
    TLS_DHE_DSS_WITH_AES_128_CBC_SHA256,
    TLS_RSA_WITH_CAMELLIA_128_CBC_SHA,
    TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA,
    TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA,
    TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA,
    TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA,
    TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA,
    TLS_ECDH_ECDSA_WITH_NULL_SHA_draft,
    TLS_ECDH_ECDSA_WITH_RC4_128_SHA_draft,
    TLS_ECDH_ECDSA_WITH_DES_CBC_SHA_draft,
    TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA_draft,
    TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA_draft,
    TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA_draft,
    TLS_ECDH_ECNRA_WITH_DES_CBC_SHA_draft,
    TLS_ECDH_ECNRA_WITH_3DES_EDE_CBC_SHA_draft,
    TLS_ECMQV_ECDSA_NULL_SHA_draft,
    TLS_ECMQV_ECDSA_WITH_RC4_128_SHA_draft,
    TLS_ECMQV_ECDSA_WITH_DES_CBC_SHA_draft,
    TLS_ECMQV_ECDSA_WITH_3DES_EDE_CBC_SHA_draft,
    TLS_ECMQV_ECNRA_NULL_SHA_draft,
    TLS_ECMQV_ECNRA_WITH_RC4_128_SHA_draft,
    TLS_ECMQV_ECNRA_WITH_DES_CBC_SHA_draft,
    TLS_ECMQV_ECNRA_WITH_3DES_EDE_CBC_SHA_draft,
    TLS_ECDH_anon_NULL_WITH_SHA_draft,
    TLS_ECDH_anon_WITH_RC4_128_SHA_draft,
    TLS_ECDH_anon_WITH_DES_CBC_SHA_draft,
    TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA_draft,
    TLS_ECDH_anon_EXPORT_WITH_DES40_CBC_SHA_draft,
    TLS_ECDH_anon_EXPORT_WITH_RC4_40_SHA_draft,
    TLS_RSA_EXPORT1024_WITH_RC4_56_MD5,
    TLS_RSA_EXPORT1024_WITH_RC2_CBC_56_MD5,
    TLS_RSA_EXPORT1024_WITH_DES_CBC_SHA,
    TLS_DHE_DSS_EXPORT1024_WITH_DES_CBC_SHA,
    TLS_RSA_EXPORT1024_WITH_RC4_56_SHA,
    TLS_DHE_DSS_EXPORT1024_WITH_RC4_56_SHA,
    TLS_DHE_DSS_WITH_RC4_128_SHA,
    TLS_DHE_RSA_WITH_AES_128_CBC_SHA256,
    TLS_DH_DSS_WITH_AES_256_CBC_SHA256,
    TLS_DH_RSA_WITH_AES_256_CBC_SHA256,
    TLS_DHE_DSS_WITH_AES_256_CBC_SHA256,
    TLS_DHE_RSA_WITH_AES_256_CBC_SHA256,
    TLS_DH_anon_WITH_AES_128_CBC_SHA256,
    TLS_DH_anon_WITH_AES_256_CBC_SHA256,
    TLS_DHE_DSS_WITH_3DES_EDE_CBC_RMD,
    TLS_DHE_DSS_WITH_AES_128_CBC_RMD,
    TLS_DHE_DSS_WITH_AES_256_CBC_RMD,
    TLS_DHE_RSA_WITH_3DES_EDE_CBC_RMD,
    TLS_DHE_RSA_WITH_AES_128_CBC_RMD,
    TLS_DHE_RSA_WITH_AES_256_CBC_RMD,
    TLS_RSA_WITH_3DES_EDE_CBC_RMD,
    TLS_RSA_WITH_AES_128_CBC_RMD,
    TLS_RSA_WITH_AES_256_CBC_RMD,
    TLS_GOSTR341094_WITH_28147_CNT_IMIT,
    TLS_GOSTR341001_WITH_28147_CNT_IMIT,
    TLS_GOSTR341094_WITH_NULL_GOSTR3411,
    TLS_GOSTR341001_WITH_NULL_GOSTR3411,
    TLS_RSA_WITH_CAMELLIA_256_CBC_SHA,
    TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA,
    TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA,
    TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA,
    TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA,
    TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA,
    TLS_PSK_WITH_RC4_128_SHA,
    TLS_PSK_WITH_3DES_EDE_CBC_SHA,
    TLS_PSK_WITH_AES_128_CBC_SHA,
    TLS_PSK_WITH_AES_256_CBC_SHA,
    TLS_DHE_PSK_WITH_RC4_128_SHA,
    TLS_DHE_PSK_WITH_3DES_EDE_CBC_SHA,
    TLS_DHE_PSK_WITH_AES_128_CBC_SHA,
    TLS_DHE_PSK_WITH_AES_256_CBC_SHA,
    TLS_RSA_PSK_WITH_RC4_128_SHA,
    TLS_RSA_PSK_WITH_3DES_EDE_CBC_SHA,
    TLS_RSA_PSK_WITH_AES_128_CBC_SHA,
    TLS_RSA_PSK_WITH_AES_256_CBC_SHA,
    TLS_RSA_WITH_SEED_CBC_SHA,
    TLS_DH_DSS_WITH_SEED_CBC_SHA,
    TLS_DH_RSA_WITH_SEED_CBC_SHA,
    TLS_DHE_DSS_WITH_SEED_CBC_SHA,
    TLS_DHE_RSA_WITH_SEED_CBC_SHA,
    TLS_DH_anon_WITH_SEED_CBC_SHA,
    TLS_RSA_WITH_AES_128_GCM_SHA256,
    TLS_RSA_WITH_AES_256_GCM_SHA384,
    TLS_DHE_RSA_WITH_AES_128_GCM_SHA256,
    TLS_DHE_RSA_WITH_AES_256_GCM_SHA384,
    TLS_DH_RSA_WITH_AES_128_GCM_SHA256,
    TLS_DH_RSA_WITH_AES_256_GCM_SHA384,
    TLS_DHE_DSS_WITH_AES_128_GCM_SHA256,
    TLS_DHE_DSS_WITH_AES_256_GCM_SHA384,
    TLS_DH_DSS_WITH_AES_128_GCM_SHA256,
    TLS_DH_DSS_WITH_AES_256_GCM_SHA384,
    TLS_DH_anon_WITH_AES_128_GCM_SHA256,
    TLS_DH_anon_WITH_AES_256_GCM_SHA384,
    TLS_DHE_PSK_WITH_AES_128_GCM_SHA256,
    TLS_DHE_PSK_WITH_AES_256_GCM_SHA384,
    TLS_RSA_PSK_WITH_AES_128_GCM_SHA256,
    TLS_RSA_PSK_WITH_AES_256_GCM_SHA384,
    TLS_PSK_WITH_AES_128_CBC_SHA256,
    TLS_PSK_WITH_AES_256_CBC_SHA384,
    TLS_PSK_WITH_NULL_SHA256,
    TLS_PSK_WITH_NULL_SHA384,
    TLS_DHE_PSK_WITH_AES_128_CBC_SHA256,
    TLS_DHE_PSK_WITH_AES_256_CBC_SHA384,
    TLS_DHE_PSK_WITH_NULL_SHA256,
    TLS_DHE_PSK_WITH_NULL_SHA384,
    TLS_RSA_PSK_WITH_AES_128_CBC_SHA256,
    TLS_RSA_PSK_WITH_AES_256_CBC_SHA384,
    TLS_RSA_PSK_WITH_NULL_SHA256,
    TLS_RSA_PSK_WITH_NULL_SHA384,
    TLS_RSA_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_RSA_WITH_CAMELLIA_256_CBC_SHA256,
    TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256,
    TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256,
    TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256,
    TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256,
    TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA256,
    TLS_ECDH_ECDSA_WITH_NULL_SHA,
    TLS_ECDH_ECDSA_WITH_RC4_128_SHA,
    TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA,
    TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA,
    TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA,
    TLS_ECDHE_ECDSA_WITH_NULL_SHA,
    TLS_ECDHE_ECDSA_WITH_RC4_128_SHA,
    TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA,
    TLS_ECDH_RSA_WITH_NULL_SHA,
    TLS_ECDH_RSA_WITH_RC4_128_SHA,
    TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA,
    TLS_ECDH_RSA_WITH_AES_128_CBC_SHA,
    TLS_ECDH_RSA_WITH_AES_256_CBC_SHA,
    TLS_ECDHE_RSA_WITH_NULL_SHA,
    TLS_ECDHE_RSA_WITH_RC4_128_SHA,
    TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA,
    TLS_ECDH_anon_WITH_NULL_SHA,
    TLS_ECDH_anon_WITH_RC4_128_SHA,
    TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA,
    TLS_ECDH_anon_WITH_AES_128_CBC_SHA,
    TLS_ECDH_anon_WITH_AES_256_CBC_SHA,
    TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA,
    TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA,
    TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA,
    TLS_SRP_SHA_WITH_AES_128_CBC_SHA,
    TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA,
    TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA,
    TLS_SRP_SHA_WITH_AES_256_CBC_SHA,
    TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA,
    TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA,
    TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256,
    TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384,
    TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256,
    TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384,
    TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256,
    TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384,
    TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256,
    TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384,
    TLS_ECDHE_PSK_WITH_RC4_128_SHA,
    TLS_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA,
    TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA,
    TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA,
    TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256,
    TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384,
    TLS_ECDHE_PSK_WITH_NULL_SHA,
    TLS_ECDHE_PSK_WITH_NULL_SHA256,
    TLS_ECDHE_PSK_WITH_NULL_SHA384,
    TLS_RSA_WITH_ARIA_128_CBC_SHA256,
    TLS_RSA_WITH_ARIA_256_CBC_SHA384,
    TLS_DH_DSS_WITH_ARIA_128_CBC_SHA256,
    TLS_DH_DSS_WITH_ARIA_256_CBC_SHA384,
    TLS_DH_RSA_WITH_ARIA_128_CBC_SHA256,
    TLS_DH_RSA_WITH_ARIA_256_CBC_SHA384,
    TLS_DHE_DSS_WITH_ARIA_128_CBC_SHA256,
    TLS_DHE_DSS_WITH_ARIA_256_CBC_SHA384,
    TLS_DHE_RSA_WITH_ARIA_128_CBC_SHA256,
    TLS_DHE_RSA_WITH_ARIA_256_CBC_SHA384,
    TLS_DH_anon_WITH_ARIA_128_CBC_SHA256,
    TLS_DH_anon_WITH_ARIA_256_CBC_SHA384,
    TLS_ECDHE_ECDSA_WITH_ARIA_128_CBC_SHA256,
    TLS_ECDHE_ECDSA_WITH_ARIA_256_CBC_SHA384,
    TLS_ECDH_ECDSA_WITH_ARIA_128_CBC_SHA256,
    TLS_ECDH_ECDSA_WITH_ARIA_256_CBC_SHA384,
    TLS_ECDHE_RSA_WITH_ARIA_128_CBC_SHA256,
    TLS_ECDHE_RSA_WITH_ARIA_256_CBC_SHA384,
    TLS_ECDH_RSA_WITH_ARIA_128_CBC_SHA256,
    TLS_ECDH_RSA_WITH_ARIA_256_CBC_SHA384,
    TLS_RSA_WITH_ARIA_128_GCM_SHA256,
    TLS_RSA_WITH_ARIA_256_GCM_SHA384,
    TLS_DHE_RSA_WITH_ARIA_128_GCM_SHA256,
    TLS_DHE_RSA_WITH_ARIA_256_GCM_SHA384,
    TLS_DH_RSA_WITH_ARIA_128_GCM_SHA256,
    TLS_DH_RSA_WITH_ARIA_256_GCM_SHA384,
    TLS_DHE_DSS_WITH_ARIA_128_GCM_SHA256,
    TLS_DHE_DSS_WITH_ARIA_256_GCM_SHA384,
    TLS_DH_DSS_WITH_ARIA_128_GCM_SHA256,
    TLS_DH_DSS_WITH_ARIA_256_GCM_SHA384,
    TLS_DH_anon_WITH_ARIA_128_GCM_SHA256,
    TLS_DH_anon_WITH_ARIA_256_GCM_SHA384,
    TLS_ECDHE_ECDSA_WITH_ARIA_128_GCM_SHA256,
    TLS_ECDHE_ECDSA_WITH_ARIA_256_GCM_SHA384,
    TLS_ECDH_ECDSA_WITH_ARIA_128_GCM_SHA256,
    TLS_ECDH_ECDSA_WITH_ARIA_256_GCM_SHA384,
    TLS_ECDHE_RSA_WITH_ARIA_128_GCM_SHA256,
    TLS_ECDHE_RSA_WITH_ARIA_256_GCM_SHA384,
    TLS_ECDH_RSA_WITH_ARIA_128_GCM_SHA256,
    TLS_ECDH_RSA_WITH_ARIA_256_GCM_SHA384,
    TLS_PSK_WITH_ARIA_128_CBC_SHA256,
    TLS_PSK_WITH_ARIA_256_CBC_SHA384,
    TLS_DHE_PSK_WITH_ARIA_128_CBC_SHA256,
    TLS_DHE_PSK_WITH_ARIA_256_CBC_SHA384,
    TLS_RSA_PSK_WITH_ARIA_128_CBC_SHA256,
    TLS_RSA_PSK_WITH_ARIA_256_CBC_SHA384,
    TLS_PSK_WITH_ARIA_128_GCM_SHA256,
    TLS_PSK_WITH_ARIA_256_GCM_SHA384,
    TLS_DHE_PSK_WITH_ARIA_128_GCM_SHA256,
    TLS_DHE_PSK_WITH_ARIA_256_GCM_SHA384,
    TLS_RSA_PSK_WITH_ARIA_128_GCM_SHA256,
    TLS_RSA_PSK_WITH_ARIA_256_GCM_SHA384,
    TLS_ECDHE_PSK_WITH_ARIA_128_CBC_SHA256,
    TLS_ECDHE_PSK_WITH_ARIA_256_CBC_SHA384,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_RSA_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_RSA_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_DHE_RSA_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_DHE_RSA_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_DH_RSA_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_DH_RSA_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_DHE_DSS_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_DHE_DSS_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_DH_DSS_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_DH_DSS_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_DH_anon_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_DH_anon_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_ECDHE_RSA_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_ECDHE_RSA_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_ECDH_RSA_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_ECDH_RSA_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_PSK_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_PSK_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_DHE_PSK_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_DHE_PSK_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_RSA_PSK_WITH_CAMELLIA_128_GCM_SHA256,
    TLS_RSA_PSK_WITH_CAMELLIA_256_GCM_SHA384,
    TLS_PSK_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_PSK_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256,
    TLS_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384,
    TLS_RSA_WITH_AES_128_CCM,
    TLS_RSA_WITH_AES_256_CCM,
    TLS_DHE_RSA_WITH_AES_128_CCM,
    TLS_DHE_RSA_WITH_AES_256_CCM,
    TLS_RSA_WITH_AES_128_CCM_8,
    TLS_RSA_WITH_AES_256_CCM_8,
    TLS_DHE_RSA_WITH_AES_128_CCM_8,
    TLS_DHE_RSA_WITH_AES_256_CCM_8,
    TLS_PSK_WITH_AES_128_CCM,
    TLS_PSK_WITH_AES_256_CCM,
    TLS_DHE_PSK_WITH_AES_128_CCM,
    TLS_DHE_PSK_WITH_AES_256_CCM,
    TLS_PSK_WITH_AES_128_CCM_8,
    TLS_PSK_WITH_AES_256_CCM_8,
    TLS_PSK_DHE_WITH_AES_128_CCM_8,
    TLS_PSK_DHE_WITH_AES_256_CCM_8,
    TLS_ECDHE_ECDSA_WITH_AES_128_CCM,
    TLS_ECDHE_ECDSA_WITH_AES_256_CCM,
    TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8,
    TLS_ECDHE_ECDSA_WITH_AES_256_CCM_8,
    TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
    TLS_PSK_WITH_CHACHA20_POLY1305_SHA256,
    TLS_ECDHE_PSK_WITH_CHACHA20_POLY1305_SHA256,
    TLS_DHE_PSK_WITH_CHACHA20_POLY1305_SHA256,
    TLS_RSA_PSK_WITH_CHACHA20_POLY1305_SHA256,
    SSL_RSA_FIPS_WITH_DES_CBC_SHA,
    SSL_RSA_FIPS_WITH_3DES_EDE_CBC_SHA,
    Unknown(u16),
}
```

The `CipherSuite` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 2]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(x: u16) -> Self`

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

- `fn clone(self: &Self) -> CipherSuite`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CipherSuite) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ContentType`

```rust
enum ContentType {
    ChangeCipherSpec,
    Alert,
    Handshake,
    ApplicationData,
    Heartbeat,
    Unknown(u8),
}
```

The `ContentType` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 1]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(x: u8) -> Self`

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

- `fn clone(self: &Self) -> ContentType`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ContentType) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `HandshakeType`

```rust
enum HandshakeType {
    HelloRequest,
    ClientHello,
    ServerHello,
    HelloVerifyRequest,
    NewSessionTicket,
    EndOfEarlyData,
    HelloRetryRequest,
    EncryptedExtensions,
    Certificate,
    ServerKeyExchange,
    CertificateRequest,
    ServerHelloDone,
    CertificateVerify,
    ClientKeyExchange,
    Finished,
    CertificateURL,
    CertificateStatus,
    KeyUpdate,
    CompressedCertificate,
    MessageHash,
    Unknown(u8),
}
```

The `HandshakeType` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 1]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(x: u8) -> Self`

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

- `fn clone(self: &Self) -> HandshakeType`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HandshakeType) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ProtocolVersion`

```rust
enum ProtocolVersion {
    SSLv2,
    SSLv3,
    TLSv1_0,
    TLSv1_1,
    TLSv1_2,
    TLSv1_3,
    DTLSv1_0,
    DTLSv1_2,
    DTLSv1_3,
    Unknown(u16),
}
```

The `ProtocolVersion` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 2]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From`

- `fn from(x: u16) -> Self`

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

- `fn clone(self: &Self) -> ProtocolVersion`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ProtocolVersion) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SignatureAlgorithm`

```rust
enum SignatureAlgorithm {
    Anonymous,
    RSA,
    DSA,
    ECDSA,
    ED25519,
    ED448,
    Unknown(u8),
}
```

The `SignatureAlgorithm` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 1]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From`

- `fn from(x: u8) -> Self`

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

- `fn clone(self: &Self) -> SignatureAlgorithm`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SignatureAlgorithm) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SignatureScheme`

```rust
enum SignatureScheme {
    RSA_PKCS1_SHA1,
    ECDSA_SHA1_Legacy,
    RSA_PKCS1_SHA256,
    ECDSA_NISTP256_SHA256,
    RSA_PKCS1_SHA384,
    ECDSA_NISTP384_SHA384,
    RSA_PKCS1_SHA512,
    ECDSA_NISTP521_SHA512,
    RSA_PSS_SHA256,
    RSA_PSS_SHA384,
    RSA_PSS_SHA512,
    ED25519,
    ED448,
    ML_DSA_44,
    ML_DSA_65,
    ML_DSA_87,
    Unknown(u16),
}
```

The `SignatureScheme` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 2]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(x: u16) -> Self`

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

- `fn clone(self: &Self) -> SignatureScheme`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SignatureScheme) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CertRevocationListError`

```rust
enum CertRevocationListError {
    BadSignature,
    UnsupportedSignatureAlgorithm,
    UnsupportedSignatureAlgorithmContext {
        signature_algorithm_id: alloc::vec::Vec<u8>,
        supported_algorithms: alloc::vec::Vec<pki_types::AlgorithmIdentifier>,
    },
    UnsupportedSignatureAlgorithmForPublicKeyContext {
        signature_algorithm_id: alloc::vec::Vec<u8>,
        public_key_algorithm_id: alloc::vec::Vec<u8>,
    },
    InvalidCrlNumber,
    InvalidRevokedCertSerialNumber,
    IssuerInvalidForCrl,
    Other(OtherError),
    ParseError,
    UnsupportedCrlVersion,
    UnsupportedCriticalExtension,
    UnsupportedDeltaCrl,
    UnsupportedIndirectCrl,
    UnsupportedRevocationReason,
}
```

The ways in which a certificate revocation list (CRL) can be invalid.

#### Variants

- **`BadSignature`**

  The CRL had a bad signature from its issuer.

- **`UnsupportedSignatureAlgorithm`**

  The CRL had an unsupported signature from its issuer.

- **`UnsupportedSignatureAlgorithmContext`**

  A signature inside a certificate or on a handshake was made with an unsupported algorithm.

- **`UnsupportedSignatureAlgorithmForPublicKeyContext`**

  A signature was made with an algorithm that doesn't match the relevant public key.

- **`InvalidCrlNumber`**

  The CRL contained an invalid CRL number.

- **`InvalidRevokedCertSerialNumber`**

  The CRL contained a revoked certificate with an invalid serial number.

- **`IssuerInvalidForCrl`**

  The CRL issuer does not specify the cRLSign key usage.

- **`Other`**

  The CRL is invalid for some other reason.
  
  Enums holding this variant will never compare equal to each other.

- **`ParseError`**

  The CRL is not correctly encoded.

- **`UnsupportedCrlVersion`**

  The CRL is not a v2 X.509 CRL.

- **`UnsupportedCriticalExtension`**

  The CRL, or a revoked certificate in the CRL, contained an unsupported critical extension.

- **`UnsupportedDeltaCrl`**

  The CRL is an unsupported delta CRL, containing only changes relative to another CRL.

- **`UnsupportedIndirectCrl`**

  The CRL is an unsupported indirect CRL, containing revoked certificates issued by a CA
  other than the issuer of the CRL.

- **`UnsupportedRevocationReason`**

  The CRL contained a revoked certificate with an unsupported revocation reason.
  See RFC 5280 Section 5.3.1[^1] for a list of supported revocation reasons.
  
  [^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1>

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

- `fn clone(self: &Self) -> CertRevocationListError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

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

### `CertificateError`

```rust
enum CertificateError {
    BadEncoding,
    Expired,
    ExpiredContext {
        time: pki_types::UnixTime,
        not_after: pki_types::UnixTime,
    },
    NotValidYet,
    NotValidYetContext {
        time: pki_types::UnixTime,
        not_before: pki_types::UnixTime,
    },
    Revoked,
    UnhandledCriticalExtension,
    UnknownIssuer,
    UnknownRevocationStatus,
    ExpiredRevocationList,
    ExpiredRevocationListContext {
        time: pki_types::UnixTime,
        next_update: pki_types::UnixTime,
    },
    BadSignature,
    UnsupportedSignatureAlgorithm,
    UnsupportedSignatureAlgorithmContext {
        signature_algorithm_id: alloc::vec::Vec<u8>,
        supported_algorithms: alloc::vec::Vec<pki_types::AlgorithmIdentifier>,
    },
    UnsupportedSignatureAlgorithmForPublicKeyContext {
        signature_algorithm_id: alloc::vec::Vec<u8>,
        public_key_algorithm_id: alloc::vec::Vec<u8>,
    },
    NotValidForName,
    NotValidForNameContext {
        expected: pki_types::ServerName<'static>,
        presented: alloc::vec::Vec<alloc::string::String>,
    },
    InvalidPurpose,
    InvalidPurposeContext {
        required: ExtendedKeyPurpose,
        presented: alloc::vec::Vec<ExtendedKeyPurpose>,
    },
    InvalidOcspResponse,
    ApplicationVerificationFailure,
    Other(OtherError),
}
```

The ways in which certificate validators can express errors.

Note that the rustls TLS protocol code interprets specifically these
error codes to send specific TLS alerts.  Therefore, if a
custom certificate validator uses incorrect errors the library as
a whole will send alerts that do not match the standard (this is usually
a minor issue, but could be misleading).

#### Variants

- **`BadEncoding`**

  The certificate is not correctly encoded.

- **`Expired`**

  The current time is after the `notAfter` time in the certificate.

- **`ExpiredContext`**

  The current time is after the `notAfter` time in the certificate.
  
  This variant is semantically the same as `Expired`, but includes
  extra data to improve error reports.

- **`NotValidYet`**

  The current time is before the `notBefore` time in the certificate.

- **`NotValidYetContext`**

  The current time is before the `notBefore` time in the certificate.
  
  This variant is semantically the same as `NotValidYet`, but includes
  extra data to improve error reports.

- **`Revoked`**

  The certificate has been revoked.

- **`UnhandledCriticalExtension`**

  The certificate contains an extension marked critical, but it was
  not processed by the certificate validator.

- **`UnknownIssuer`**

  The certificate chain is not issued by a known root certificate.

- **`UnknownRevocationStatus`**

  The certificate's revocation status could not be determined.

- **`ExpiredRevocationList`**

  The certificate's revocation status could not be determined, because the CRL is expired.

- **`ExpiredRevocationListContext`**

  The certificate's revocation status could not be determined, because the CRL is expired.
  
  This variant is semantically the same as `ExpiredRevocationList`, but includes
  extra data to improve error reports.

- **`BadSignature`**

  A certificate is not correctly signed by the key of its alleged
  issuer.

- **`UnsupportedSignatureAlgorithm`**

  A signature inside a certificate or on a handshake was made with an unsupported algorithm.

- **`UnsupportedSignatureAlgorithmContext`**

  A signature inside a certificate or on a handshake was made with an unsupported algorithm.

- **`UnsupportedSignatureAlgorithmForPublicKeyContext`**

  A signature was made with an algorithm that doesn't match the relevant public key.

- **`NotValidForName`**

  The subject names in an end-entity certificate do not include
  the expected name.

- **`NotValidForNameContext`**

  The subject names in an end-entity certificate do not include
  the expected name.
  
  This variant is semantically the same as `NotValidForName`, but includes
  extra data to improve error reports.

- **`InvalidPurpose`**

  The certificate is being used for a different purpose than allowed.

- **`InvalidPurposeContext`**

  The certificate is being used for a different purpose than allowed.
  
  This variant is semantically the same as `InvalidPurpose`, but includes
  extra data to improve error reports.

- **`InvalidOcspResponse`**

  The OCSP response provided to the verifier was invalid.
  
  This should be returned from [`ServerCertVerifier::verify_server_cert()`](#verify-server-cert)
  when a verifier checks its `ocsp_response` parameter and finds it invalid.
  
  This maps to [`AlertDescription::BadCertificateStatusResponse`](#badcertificatestatusresponse).
  

- **`ApplicationVerificationFailure`**

  The certificate is valid, but the handshake is rejected for other
  reasons.

- **`Other`**

  Any other error.
  
  This can be used by custom verifiers to expose the underlying error
  (where they are not better described by the more specific errors
  above).
  
  It is also used by the default verifier in case its error is
  not covered by the above common cases.
  
  Enums holding this variant will never compare equal to each other.

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

- `fn clone(self: &Self) -> CertificateError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

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

### `EncryptedClientHelloError`

```rust
enum EncryptedClientHelloError {
    InvalidConfigList,
    NoCompatibleConfig,
    SniRequired,
}
```

An error that occurred while handling Encrypted Client Hello (ECH).

#### Variants

- **`InvalidConfigList`**

  The provided ECH configuration list was invalid.

- **`NoCompatibleConfig`**

  No compatible ECH configuration.

- **`SniRequired`**

  The client configuration has server name indication (SNI) disabled.

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

- `fn clone(self: &Self) -> EncryptedClientHelloError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &EncryptedClientHelloError) -> bool`

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

### `Error`

```rust
enum Error {
    InappropriateMessage {
        expect_types: alloc::vec::Vec<crate::enums::ContentType>,
        got_type: crate::enums::ContentType,
    },
    InappropriateHandshakeMessage {
        expect_types: alloc::vec::Vec<crate::enums::HandshakeType>,
        got_type: crate::enums::HandshakeType,
    },
    InvalidEncryptedClientHello(EncryptedClientHelloError),
    InvalidMessage(InvalidMessage),
    NoCertificatesPresented,
    UnsupportedNameType,
    DecryptError,
    EncryptError,
    PeerIncompatible(PeerIncompatible),
    PeerMisbehaved(PeerMisbehaved),
    AlertReceived(crate::enums::AlertDescription),
    InvalidCertificate(CertificateError),
    InvalidCertRevocationList(CertRevocationListError),
    General(alloc::string::String),
    FailedToGetCurrentTime,
    FailedToGetRandomBytes,
    HandshakeNotComplete,
    PeerSentOversizedRecord,
    NoApplicationProtocol,
    BadMaxFragmentSize,
    InconsistentKeys(InconsistentKeys),
    Other(OtherError),
}
```

rustls reports protocol errors using this type.

#### Variants

- **`InappropriateMessage`**

  We received a TLS message that isn't valid right now.
  `expect_types` lists the message types we can expect right now.
  `got_type` is the type we found.  This error is typically
  caused by a buggy TLS stack (the peer or this one), a broken
  network, or an attack.

- **`InappropriateHandshakeMessage`**

  We received a TLS handshake message that isn't valid right now.
  `expect_types` lists the handshake message types we can expect
  right now.  `got_type` is the type we found.

- **`InvalidEncryptedClientHello`**

  An error occurred while handling Encrypted Client Hello (ECH).

- **`InvalidMessage`**

  The peer sent us a TLS message with invalid contents.

- **`NoCertificatesPresented`**

  The peer didn't give us any certificates.

- **`UnsupportedNameType`**

  The certificate verifier doesn't support the given type of name.

- **`DecryptError`**

  We couldn't decrypt a message.  This is invariably fatal.

- **`EncryptError`**

  We couldn't encrypt a message because it was larger than the allowed message size.
  This should never happen if the application is using valid record sizes.

- **`PeerIncompatible`**

  The peer doesn't support a protocol version/feature we require.
  The parameter gives a hint as to what version/feature it is.

- **`PeerMisbehaved`**

  The peer deviated from the standard TLS protocol.
  The parameter gives a hint where.

- **`AlertReceived`**

  We received a fatal alert.  This means the peer is unhappy.

- **`InvalidCertificate`**

  We saw an invalid certificate.
  
  The contained error is from the certificate validation trait
  implementation.

- **`InvalidCertRevocationList`**

  A provided certificate revocation list (CRL) was invalid.

- **`General`**

  A catch-all error for unlikely errors.

- **`FailedToGetCurrentTime`**

  We failed to figure out what time it currently is.

- **`FailedToGetRandomBytes`**

  We failed to acquire random bytes from the system.

- **`HandshakeNotComplete`**

  This function doesn't work until the TLS handshake
  is complete.

- **`PeerSentOversizedRecord`**

  The peer sent an oversized record/fragment.

- **`NoApplicationProtocol`**

  An incoming connection did not support any known application protocol.

- **`BadMaxFragmentSize`**

  The `max_fragment_size` value supplied in configuration was too small,
  or too large.

- **`InconsistentKeys`**

  Specific failure cases from [`keys_match`](#keys-match) or a [`crate::crypto::signer::SigningKey`](#signingkey) that cannot produce a corresponding public key.
  

- **`Other`**

  Any other error.
  
  This variant should only be used when the error is not better described by a more
  specific variant. For example, if a custom crypto provider returns a
  provider specific error.
  
  Enums holding this variant will never compare equal to each other.

#### Trait Implementations

##### `impl From`

- `fn from(e: InconsistentKeys) -> Self`

##### `impl From`

- `fn from(e: PeerMisbehaved) -> Self`

##### `impl From`

- `fn from(e: CertificateError) -> Self`

##### `impl From`

- `fn from(_: rand::GetRandomFailed) -> Self`

##### `impl From`

- `fn from(e: EncryptedClientHelloError) -> Self`

##### `impl From`

- `fn from(e: InvalidMessage) -> Self`

##### `impl From`

- `fn from(_: SystemTimeError) -> Self`

##### `impl From`

- `fn from(value: UnsupportedOperationError) -> Self`

##### `impl From`

- `fn from(value: OtherError) -> Self`

##### `impl From`

- `fn from(e: PeerIncompatible) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(e: CertRevocationListError) -> Self`

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

- `fn clone(self: &Self) -> Error`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool`

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

### `ExtendedKeyPurpose`

```rust
enum ExtendedKeyPurpose {
    ClientAuth,
    ServerAuth,
    Other(alloc::vec::Vec<usize>),
}
```

Extended Key Usage (EKU) purpose values.

These are usually represented as OID values in the certificate's extension (if present), but
we represent the values that are most relevant to rustls as named enum variants.

#### Variants

- **`ClientAuth`**

  Client authentication

- **`ServerAuth`**

  Server authentication

- **`Other`**

  Other EKU values
  
  Represented here as a `Vec<usize>` for human readability.

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

- `fn clone(self: &Self) -> ExtendedKeyPurpose`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ExtendedKeyPurpose) -> bool`

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

### `InconsistentKeys`

```rust
enum InconsistentKeys {
    KeyMismatch,
    Unknown,
}
```

Specific failure cases from [`keys_match`](#keys-match) or a [`crate::crypto::signer::SigningKey`](#signingkey) that cannot produce a corresponding public key.


#### Variants

- **`KeyMismatch`**

  The public key returned by the [`SigningKey`](#signingkey) does not match the public key information in the certificate.
  

- **`Unknown`**

  The [`SigningKey`](#signingkey) cannot produce its corresponding public key.
  

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

- `fn clone(self: &Self) -> InconsistentKeys`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &InconsistentKeys) -> bool`

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

### `InvalidMessage`

```rust
enum InvalidMessage {
    CertificatePayloadTooLarge,
    HandshakePayloadTooLarge,
    InvalidCcs,
    InvalidContentType,
    InvalidCertificateStatusType,
    InvalidCertRequest,
    InvalidDhParams,
    InvalidEmptyPayload,
    InvalidKeyUpdate,
    InvalidServerName,
    MessageTooLarge,
    MessageTooShort,
    MissingData(&'static str),
    MissingKeyExchange,
    NoSignatureSchemes,
    TrailingData(&'static str),
    UnexpectedMessage(&'static str),
    UnknownProtocolVersion,
    UnsupportedCompression,
    UnsupportedCurveType,
    UnsupportedKeyExchangeAlgorithm(crate::msgs::handshake::KeyExchangeAlgorithm),
    EmptyTicketValue,
    IllegalEmptyList(&'static str),
    IllegalEmptyValue,
    DuplicateExtension(u16),
    PreSharedKeyIsNotFinalExtension,
    UnknownHelloRetryRequestExtension,
    UnknownCertificateExtension,
}
```

A corrupt TLS message payload that resulted in an error.

#### Variants

- **`CertificatePayloadTooLarge`**

  A certificate payload exceeded rustls's 64KB limit

- **`HandshakePayloadTooLarge`**

  An advertised message was larger then expected.

- **`InvalidCcs`**

  The peer sent us a syntactically incorrect ChangeCipherSpec payload.

- **`InvalidContentType`**

  An unknown content type was encountered during message decoding.

- **`InvalidCertificateStatusType`**

  A peer sent an invalid certificate status type

- **`InvalidCertRequest`**

  Context was incorrectly attached to a certificate request during a handshake.

- **`InvalidDhParams`**

  A peer's DH params could not be decoded

- **`InvalidEmptyPayload`**

  A message was zero-length when its record kind forbids it.

- **`InvalidKeyUpdate`**

  A peer sent an unexpected key update request.

- **`InvalidServerName`**

  A peer's server name could not be decoded

- **`MessageTooLarge`**

  A TLS message payload was larger then allowed by the specification.

- **`MessageTooShort`**

  Message is shorter than the expected length

- **`MissingData`**

  Missing data for the named handshake payload value

- **`MissingKeyExchange`**

  A peer did not advertise its supported key exchange groups.

- **`NoSignatureSchemes`**

  A peer sent an empty list of signature schemes

- **`TrailingData`**

  Trailing data found for the named handshake payload value

- **`UnexpectedMessage`**

  A peer sent an unexpected message type.

- **`UnknownProtocolVersion`**

  An unknown TLS protocol was encountered during message decoding.

- **`UnsupportedCompression`**

  A peer sent a non-null compression method.

- **`UnsupportedCurveType`**

  A peer sent an unknown elliptic curve type.

- **`UnsupportedKeyExchangeAlgorithm`**

  A peer sent an unsupported key exchange algorithm.

- **`EmptyTicketValue`**

  A server sent an empty ticket

- **`IllegalEmptyList`**

  A peer sent an empty list of items, but a non-empty list is required.
  
  The argument names the context.

- **`IllegalEmptyValue`**

  A peer sent an empty value, but a non-empty value is required.

- **`DuplicateExtension`**

  A peer sent a message where a given extension type was repeated

- **`PreSharedKeyIsNotFinalExtension`**

  A peer sent a message with a PSK offer extension in wrong position

- **`UnknownHelloRetryRequestExtension`**

  A server sent a HelloRetryRequest with an unknown extension

- **`UnknownCertificateExtension`**

  The peer sent a TLS1.3 Certificate with an unknown extension

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

- `fn clone(self: &Self) -> InvalidMessage`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &InvalidMessage) -> bool`

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

### `PeerIncompatible`

```rust
enum PeerIncompatible {
    EcPointsExtensionRequired,
    ExtendedMasterSecretExtensionRequired,
    IncorrectCertificateTypeExtension,
    KeyShareExtensionRequired,
    NamedGroupsExtensionRequired,
    NoCertificateRequestSignatureSchemesInCommon,
    NoCipherSuitesInCommon,
    NoEcPointFormatsInCommon,
    NoKxGroupsInCommon,
    NoSignatureSchemesInCommon,
    NullCompressionRequired,
    ServerDoesNotSupportTls12Or13,
    ServerSentHelloRetryRequestWithUnknownExtension,
    ServerTlsVersionIsDisabledByOurConfig,
    SignatureAlgorithmsExtensionRequired,
    SupportedVersionsExtensionRequired,
    Tls12NotOffered,
    Tls12NotOfferedOrEnabled,
    Tls13RequiredForQuic,
    UncompressedEcPointsRequired,
    UnsolicitedCertificateTypeExtension,
    ServerRejectedEncryptedClientHello(Option<alloc::vec::Vec<crate::msgs::handshake::EchConfigPayload>>),
}
```

The set of cases where we failed to make a connection because a peer
doesn't support a TLS version/feature we require.

This is `non_exhaustive`: we might add or stop using items here in minor
versions.

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

- `fn clone(self: &Self) -> PeerIncompatible`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PeerIncompatible) -> bool`

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

### `PeerMisbehaved`

```rust
enum PeerMisbehaved {
    AttemptedDowngradeToTls12WhenTls13IsSupported,
    BadCertChainExtensions,
    DisallowedEncryptedExtension,
    DuplicateClientHelloExtensions,
    DuplicateEncryptedExtensions,
    DuplicateHelloRetryRequestExtensions,
    DuplicateNewSessionTicketExtensions,
    DuplicateServerHelloExtensions,
    DuplicateServerNameTypes,
    EarlyDataAttemptedInSecondClientHello,
    EarlyDataExtensionWithoutResumption,
    EarlyDataOfferedWithVariedCipherSuite,
    HandshakeHashVariedAfterRetry,
    IllegalHelloRetryRequestWithEmptyCookie,
    IllegalHelloRetryRequestWithNoChanges,
    IllegalHelloRetryRequestWithOfferedGroup,
    IllegalHelloRetryRequestWithUnofferedCipherSuite,
    IllegalHelloRetryRequestWithUnofferedNamedGroup,
    IllegalHelloRetryRequestWithUnsupportedVersion,
    IllegalHelloRetryRequestWithWrongSessionId,
    IllegalHelloRetryRequestWithInvalidEch,
    IllegalMiddleboxChangeCipherSpec,
    IllegalTlsInnerPlaintext,
    IncorrectBinder,
    InvalidCertCompression,
    InvalidMaxEarlyDataSize,
    InvalidKeyShare,
    KeyEpochWithPendingFragment,
    KeyUpdateReceivedInQuicConnection,
    MessageInterleavedWithHandshakeMessage,
    MissingBinderInPskExtension,
    MissingKeyShare,
    MissingPskModesExtension,
    MissingQuicTransportParameters,
    OfferedDuplicateCertificateCompressions,
    OfferedDuplicateKeyShares,
    OfferedEarlyDataWithOldProtocolVersion,
    OfferedEmptyApplicationProtocol,
    OfferedIncorrectCompressions,
    PskExtensionMustBeLast,
    PskExtensionWithMismatchedIdsAndBinders,
    RefusedToFollowHelloRetryRequest,
    RejectedEarlyDataInterleavedWithHandshakeMessage,
    ResumptionAttemptedWithVariedEms,
    ResumptionOfferedWithVariedCipherSuite,
    ResumptionOfferedWithVariedEms,
    ResumptionOfferedWithIncompatibleCipherSuite,
    SelectedDifferentCipherSuiteAfterRetry,
    SelectedInvalidPsk,
    SelectedTls12UsingTls13VersionExtension,
    SelectedUnofferedApplicationProtocol,
    SelectedUnofferedCertCompression,
    SelectedUnofferedCipherSuite,
    SelectedUnofferedCompression,
    SelectedUnofferedKxGroup,
    SelectedUnofferedPsk,
    SelectedUnusableCipherSuiteForVersion,
    ServerEchoedCompatibilitySessionId,
    ServerHelloMustOfferUncompressedEcPoints,
    ServerNameDifferedOnRetry,
    ServerNameMustContainOneHostName,
    SignedKxWithWrongAlgorithm,
    SignedHandshakeWithUnadvertisedSigScheme,
    TooManyEmptyFragments,
    TooManyKeyUpdateRequests,
    TooManyRenegotiationRequests,
    TooManyWarningAlertsReceived,
    TooMuchEarlyDataReceived,
    UnexpectedCleartextExtension,
    UnsolicitedCertExtension,
    UnsolicitedEncryptedExtension,
    UnsolicitedSctList,
    UnsolicitedServerHelloExtension,
    WrongGroupForKeyShare,
    UnsolicitedEchExtension,
}
```

The set of cases where we failed to make a connection because we thought
the peer was misbehaving.

This is `non_exhaustive`: we might add or stop using items here in minor
versions.  We also don't document what they mean.  Generally a user of
rustls shouldn't vary its behaviour on these error codes, and there is
nothing it can do to improve matters.

Please file a bug against rustls if you see `Error::PeerMisbehaved` in
the wild.

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

- `fn clone(self: &Self) -> PeerMisbehaved`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PeerMisbehaved) -> bool`

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

### `NamedGroup`

```rust
enum NamedGroup {
    secp256r1,
    secp384r1,
    secp521r1,
    X25519,
    X448,
    FFDHE2048,
    FFDHE3072,
    FFDHE4096,
    FFDHE6144,
    FFDHE8192,
    MLKEM512,
    MLKEM768,
    MLKEM1024,
    secp256r1MLKEM768,
    X25519MLKEM768,
    Unknown(u16),
}
```

The `NamedGroup` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn key_exchange_algorithm(self: Self) -> KeyExchangeAlgorithm`
  Return the key exchange algorithm associated with this `NamedGroup`

- `fn to_array(self: Self) -> [u8; 2]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From`

- `fn from(x: u16) -> Self`

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

- `fn clone(self: &Self) -> NamedGroup`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &NamedGroup) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ConnectionTrafficSecrets`

```rust
enum ConnectionTrafficSecrets {
    Aes128Gcm {
        key: crate::crypto::cipher::AeadKey,
        iv: crate::crypto::cipher::Iv,
    },
    Aes256Gcm {
        key: crate::crypto::cipher::AeadKey,
        iv: crate::crypto::cipher::Iv,
    },
    Chacha20Poly1305 {
        key: crate::crypto::cipher::AeadKey,
        iv: crate::crypto::cipher::Iv,
    },
}
```

Secrets used to encrypt/decrypt data in a TLS session.

These can be used to configure kTLS for a socket in one direction.
The only other piece of information needed is the sequence number,
which is in [ExtractedSecrets].

#### Variants

- **`Aes128Gcm`**

  Secrets for the AES_128_GCM AEAD algorithm

- **`Aes256Gcm`**

  Secrets for the AES_256_GCM AEAD algorithm

- **`Chacha20Poly1305`**

  Secrets for the CHACHA20_POLY1305 AEAD algorithm

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

### `SupportedCipherSuite`

```rust
enum SupportedCipherSuite {
    Tls12(&'static crate::tls12::Tls12CipherSuite),
    Tls13(&'static crate::tls13::Tls13CipherSuite),
}
```

A cipher suite supported by rustls.

This type carries both configuration and implementation. Compare with
[`CipherSuite`](#ciphersuite), which carries solely a cipher suite identifier.

#### Variants

- **`Tls12`**

  A TLS 1.2 cipher suite

- **`Tls13`**

  A TLS 1.3 cipher suite

#### Implementations

- `fn suite(self: &Self) -> CipherSuite`
  The cipher suite's identifier

- `fn tls13(self: &Self) -> Option<&'static Tls13CipherSuite>`
  Return the inner `Tls13CipherSuite` for this suite, if it is a TLS1.3 suite.

- `fn version(self: &Self) -> &'static SupportedProtocolVersion`
  Return supported protocol version for the cipher suite.

- `fn usable_for_signature_algorithm(self: &Self, _sig_alg: SignatureAlgorithm) -> bool`
  Return true if this suite is usable for a key only offering `sig_alg`

- `fn fips(self: &Self) -> bool`
  Return `true` if this is backed by a FIPS-approved implementation.

#### Trait Implementations

##### `impl From`

- `fn from(s: &'static Tls13CipherSuite) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(s: &'static Tls12CipherSuite) -> Self`

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

- `fn clone(self: &Self) -> SupportedCipherSuite`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SupportedCipherSuite) -> bool`

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

## Traits

