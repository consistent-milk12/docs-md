*[regex_automata](../../index.md) / [util](../index.md) / [pool](index.md)*

---

# Module `pool`

A thread safe memory pool.

The principal type in this module is a [`Pool`](#pool). It main use case is for
holding a thread safe collection of mutable scratch spaces (usually called
`Cache` in this crate) that regex engines need to execute a search. This then
permits sharing the same read-only regex object across multiple threads while
having a quick way of reusing scratch space in a thread safe way. This avoids
needing to re-create the scratch space for every search, which could wind up
being quite expensive.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inner`](#inner) | mod |  |
| [`Pool`](#pool) | struct | A thread safe pool that works in an `alloc`-only context. |
| [`PoolGuard`](#poolguard) | struct | A guard that is returned when a caller requests a value from the pool. |

## Modules

- [`inner`](inner/index.md)

## Structs

### `Pool<T, F>`

```rust
struct Pool<T, F>(alloc::boxed::Box<inner::Pool<T, F>>);
```

A thread safe pool that works in an `alloc`-only context.

Getting a value out comes with a guard. When that guard is dropped, the
value is automatically put back in the pool. The guard provides both a
`Deref` and a `DerefMut` implementation for easy access to an underlying
`T`.

A `Pool` impls `Sync` when `T` is `Send` (even if `T` is not `Sync`). This
is possible because a pool is guaranteed to provide a value to exactly one
thread at any time.

Currently, a pool never contracts in size. Its size is proportional to the
maximum number of simultaneous uses. This may change in the future.

A `Pool` is a particularly useful data structure for this crate because
many of the regex engines require a mutable "cache" in order to execute
a search. Since regexes themselves tend to be global, the problem is then:
how do you get a mutable cache to execute a search? You could:

1. Use a `thread_local!`, which requires the standard library and requires
that the regex pattern be statically known.
2. Use a `Pool`.
3. Make the cache an explicit dependency in your code and pass it around.
4. Put the cache state in a `Mutex`, but this means only one search can
execute at a time.
5. Create a new cache for every search.

A `thread_local!` is perhaps the best choice if it works for your use case.
Putting the cache in a mutex or creating a new cache for every search are
perhaps the worst choices. Of the remaining two choices, whether you use
this `Pool` or thread through a cache explicitly in your code is a matter
of taste and depends on your code architecture.

# Warning: may use a spin lock

When this crate is compiled _without_ the `std` feature, then this type
may used a spin lock internally. This can have subtle effects that may
be undesirable. See [Spinlocks Considered Harmful][spinharm] for a more
thorough treatment of this topic.

# Example

This example shows how to share a single hybrid regex among multiple
threads, while also safely getting exclusive access to a hybrid's
[`Cache`](crate::hybrid::regex::Cache) without preventing other searches
from running while your thread uses the `Cache`.

```rust
use regex_automata::{
    hybrid::regex::{Cache, Regex},
    util::{lazy::Lazy, pool::Pool},
    Match,
};

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("foo[0-9]+bar").unwrap());
static CACHE: Lazy<Pool<Cache>> =
    Lazy::new(|| Pool::new(|| RE.create_cache()));

let expected = Some(Match::must(0, 3..14));
assert_eq!(expected, RE.find(&mut CACHE.get(), b"zzzfoo12345barzzz"));
```

#### Implementations

- <span id="pool-get"></span>`fn get(&self) -> PoolGuard<'_, T, F>` — [`PoolGuard`](#poolguard)

#### Trait Implementations

##### `impl<T: core::fmt::Debug, F> Debug for Pool<T, F>`

- <span id="pool-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `PoolGuard<'a, T: Send, F: Fn() -> T>`

```rust
struct PoolGuard<'a, T: Send, F: Fn() -> T>(inner::PoolGuard<'a, T, F>);
```

A guard that is returned when a caller requests a value from the pool.

The purpose of the guard is to use RAII to automatically put the value
back in the pool once it's dropped.

#### Implementations

- <span id="poolguard-put"></span>`fn put(this: PoolGuard<'_, T, F>)` — [`PoolGuard`](#poolguard)

#### Trait Implementations

##### `impl<'a, T: Send + core::fmt::Debug, F: Fn() -> T> Debug for PoolGuard<'a, T, F>`

- <span id="poolguard-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<'a, T: Send, F: Fn() -> T> Deref for PoolGuard<'a, T, F>`

- <span id="poolguard-target"></span>`type Target = T`

- <span id="poolguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<'a, T: Send, F: Fn() -> T> DerefMut for PoolGuard<'a, T, F>`

- <span id="poolguard-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<P, T> Receiver for PoolGuard<'a, T, F>`

- <span id="poolguard-target"></span>`type Target = T`

