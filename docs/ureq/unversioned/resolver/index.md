*[ureq](../../index.md) / [unversioned](../index.md) / [resolver](index.md)*

---

# Module `resolver`

Name resolvers.

**NOTE resolver does not (yet) [follow semver][super](#super)
.**

_NOTE: Resolver is deep configuration of ureq and is not required for regular use._

Name resolving is pluggable. The resolver's duty is to take a URI and translate it
to a socket address (IP + port). This is done as a separate step in regular ureq use.
The hostname is looked up and provided to the [`Connector`](crate::transport::Connector).

In some situations it might be desirable to not do this lookup, or to use another system
than DNS for it.

## Structs

### `DefaultResolver`

```rust
struct DefaultResolver {
    // [REDACTED: Private Fields]
}
```

Default resolver implementation.

Uses std::net [`ToSocketAddrs`](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html) to
do the lookup. Can optionally spawn a thread to abort lookup if the relevant timeout is set.

#### Implementations

- `fn host_and_port(scheme: &Scheme, authority: &Authority) -> Option<String>`
  Helper to combine scheme host and port to a single string.

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

##### `impl Resolver`

- `fn resolve(self: &Self, uri: &Uri, config: &Config, timeout: NextTimeout) -> Result<ResolvedSocketAddrs, Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> DefaultResolver`

## Traits

### `Resolver`

```rust
trait Resolver: Debug + Send + Sync + 'static { ... }
```

Trait for name resolvers.

#### Required Methods

- `fn resolve(self: &Self, uri: &Uri, config: &Config, timeout: NextTimeout) -> Result<ResolvedSocketAddrs, Error>`

  Resolve the URI to a socket address.

- `fn empty(self: &Self) -> ResolvedSocketAddrs`

  Produce an empty array of addresses.

## Type Aliases

### `ResolvedSocketAddrs`

```rust
type ResolvedSocketAddrs = ArrayVec<std::net::SocketAddr, MAX_ADDRS>;
```

Addresses as returned by the resolver.

