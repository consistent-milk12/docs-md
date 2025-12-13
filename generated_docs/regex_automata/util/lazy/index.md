*[regex_automata](../../index.md) / [util](../index.md) / [lazy](index.md)*

---

# Module `lazy`

A lazily initialized value for safe sharing between threads.

The principal type in this module is `Lazy`, which makes it easy to construct
values that are shared safely across multiple threads simultaneously.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`lazy`](#lazy) | mod |  |
| [`Lazy`](#lazy) | struct | A lazily initialized value that implements `Deref` for `T`. |

## Modules

- [`lazy`](lazy/index.md)

## Structs

### `Lazy<T, F>`

```rust
struct Lazy<T, F>(lazy::Lazy<T, F>);
```

*Defined in [`regex-automata-0.4.13/src/util/lazy.rs:52`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/lazy.rs#L52)*

A lazily initialized value that implements `Deref` for `T`.

A `Lazy` takes an initialization function and permits callers from any
thread to access the result of that initialization function in a safe
manner. In effect, this permits one-time initialization of global resources
in a (possibly) multi-threaded program.

This type and its functionality are available even when neither the `alloc`
nor the `std` features are enabled. In exchange, a `Lazy` does **not**
guarantee that the given `create` function is called at most once. It
might be called multiple times. Moreover, a call to `Lazy::get` (either
explicitly or implicitly via `Lazy`'s `Deref` impl) may block until a `T`
is available.

This is very similar to `lazy_static` or `once_cell`, except it doesn't
guarantee that the initialization function will be run once and it works
in no-alloc no-std environments. With that said, if you need stronger
guarantees or a more flexible API, then it is recommended to use either
`lazy_static` or `once_cell`.

# Warning: may use a spin lock

When this crate is compiled _without_ the `alloc` feature, then this type
may used a spin lock internally. This can have subtle effects that may
be undesirable. See [Spinlocks Considered Harmful][spinharm] for a more
thorough treatment of this topic.

# Example

This type is useful for creating regexes once, and then using them from
multiple threads simultaneously without worrying about synchronization.

```rust
use regex_automata::{dfa::regex::Regex, util::lazy::Lazy, Match};

static RE: Lazy<Regex> = Lazy::new(|| Regex::new("foo[0-9]+bar").unwrap());

let expected = Some(Match::must(0, 3..14));
assert_eq!(expected, RE.find(b"zzzfoo12345barzzz"));
```

#### Implementations

- <span id="lazy-new"></span>`const fn new(create: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

  Create a new `Lazy` value that is initialized via the given function.

  

  The `T` type is automatically inferred from the return type of the

  `create` function given.

#### Trait Implementations

##### `impl<T> Any for Lazy<T, F>`

- <span id="lazy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lazy<T, F>`

- <span id="lazy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lazy<T, F>`

- <span id="lazy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, F: Fn() -> T> Debug for Lazy<T, F>`

- <span id="lazy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F: Fn() -> T> Deref for Lazy<T, F>`

- <span id="lazy-deref-type-target"></span>`type Target = T`

- <span id="lazy-deref"></span>`fn deref(&self) -> &T`

##### `impl<T> From for Lazy<T, F>`

- <span id="lazy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Lazy<T, F>`

- <span id="lazy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Receiver for Lazy<T, F>`

- <span id="lazy-receiver-type-target"></span>`type Target = T`

##### `impl<T, U> TryFrom for Lazy<T, F>`

- <span id="lazy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Lazy<T, F>`

- <span id="lazy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

