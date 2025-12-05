*[regex_automata](../../index.md) / [util](../index.md) / [lazy](index.md)*

---

# Module `lazy`

A lazily initialized value for safe sharing between threads.

The principal type in this module is `Lazy`, which makes it easy to construct
values that are shared safely across multiple threads simultaneously.

## Structs

### `Lazy<T, F>`

```rust
struct Lazy<T, F>(lazy::Lazy<T, F>);
```

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
be undesirable. See [Spinlocks Considered Harmful][spinharm](#spinharm) for a more
thorough treatment of this topic.

[spinharm](#spinharm): https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html

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

- `fn get(this: &Lazy<T, F>) -> &T` — [`Lazy`](../../../util/lazy/index.md)

#### Trait Implementations

##### `impl Debug<T: fmt::Debug, F: Fn() -> T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref<T, F: Fn() -> T>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl Receiver<P, T>`

- `type Target = T`

