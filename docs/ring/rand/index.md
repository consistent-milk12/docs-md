*[ring](../index.md) / [rand](index.md)*

---

# Module `rand`

Cryptographic pseudo-random number generation.

*ring* functions that generate random bytes take a `&dyn SecureRandom`
parameter to make it clear which functions are non-deterministic.

## Structs

### `Random<T: RandomlyConstructable>`

```rust
struct Random<T: RandomlyConstructable>();
```

A random value constructed from a `SecureRandom` that hasn't been exposed
through any safe Rust interface.

Intentionally does not implement any traits other than `Sized`.

#### Implementations

- `fn expose(self: Self) -> T`
  Expose the random value.

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

### `SystemRandom`

```rust
struct SystemRandom();
```

A secure random number generator where the random values come directly
from the operating system.

"Directly from the operating system" here presently means "whatever the
`getrandom` crate does" but that may change in the future. That roughly
means calling libc's `getrandom` function or whatever is analogous to that;
see the `getrandom` crate's documentation for more info.

A single `SystemRandom` may be shared across multiple threads safely.

`new()` is guaranteed to always succeed and to have low latency; it won't
try to open or read from a file or do similar things. The first call to
`fill()` may block a substantial amount of time since any and all
initialization is deferred to it. Therefore, it may be a good idea to call
`fill()` once at a non-latency-sensitive time to minimize latency for
future calls.

#### Implementations

- `fn new() -> Self`
  Constructs a new `SystemRandom`.

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

- `fn clone(self: &Self) -> SystemRandom`

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

##### `impl SecureRandom<T>`

- `fn fill(self: &Self, dest: &mut [u8]) -> Result<(), Unspecified>`

## Traits

### `SecureRandom`

```rust
trait SecureRandom: sealed::SecureRandom { ... }
```

A secure random number generator.

#### Required Methods

- `fn fill(self: &Self, dest: &mut [u8]) -> Result<(), error::Unspecified>`

  Fills `dest` with random bytes.

### `RandomlyConstructable`

```rust
trait RandomlyConstructable: sealed::RandomlyConstructable { ... }
```

A type that can be returned by `ring::rand::generate()`.

## Functions

### `generate`

```rust
fn generate<T: RandomlyConstructable>(rng: &dyn SecureRandom) -> Result<Random<T>, error::Unspecified>
```

Generate the new random value using `rng`.

