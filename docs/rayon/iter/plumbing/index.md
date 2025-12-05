*[rayon](../../index.md) / [iter](../index.md) / [plumbing](index.md)*

---

# Module `plumbing`

Traits and functions used to implement parallel iteration.  These are
low-level details -- users of parallel iterators should not need to
interact with them directly.  See [the `plumbing` README][r](#r) for a general overview.

[r](#r): https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md

## Traits

### `ProducerCallback<T>`

```rust
trait ProducerCallback<T> { ... }
```

The `ProducerCallback` trait is a kind of generic closure,
[analogous to `FnOnce`][FnOnce]. See [the corresponding section in
the plumbing README][r](#r) for more details.

[r](#r): https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md#producer-callback
[FnOnce]: std::ops::FnOnce

#### Required Methods

- `type Output`

- `fn callback<P>(self: Self, producer: P) -> <Self as >::Output`

  Invokes the callback with the given producer as argument. The

### `Producer`

```rust
trait Producer: Send + Sized { ... }
```

A `Producer` is effectively a "splittable `IntoIterator`". That
is, a producer is a value which can be converted into an iterator
at any time: at that point, it simply produces items on demand,
like any iterator. But what makes a `Producer` special is that,
*before* we convert to an iterator, we can also **split** it at a
particular point using the `split_at` method. This will yield up
two producers, one producing the items before that point, and one
producing the items after that point (these two producers can then
independently be split further, or be converted into iterators).
In Rayon, this splitting is used to divide between threads.
See [the `plumbing` README][r](#r) for further details.

Note that each producer will always produce a fixed number of
items N. However, this number N is not queryable through the API;
the consumer is expected to track it.

NB. You might expect `Producer` to extend the `IntoIterator`
trait.  However, [rust-lang/rust#20671][20671] prevents us from
declaring the DoubleEndedIterator and ExactSizeIterator
constraints on a required IntoIterator trait, so we inline
IntoIterator here until that issue is fixed.

[r](#r): https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
[20671]: https://github.com/rust-lang/rust/issues/20671

#### Required Methods

- `type Item`

- `type IntoIter: 3`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

  Convert `self` into an iterator; at this point, no more parallel splits

- `fn min_len(self: &Self) -> usize`

  The minimum number of items that we will process

- `fn max_len(self: &Self) -> usize`

  The maximum number of items that we will process

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

  Split into two producers; one produces items `0..index`, the

- `fn fold_with<F>(self: Self, folder: F) -> F`

  Iterate the producer, feeding each element to `folder`, and

### `Consumer<Item>`

```rust
trait Consumer<Item>: Send + Sized { ... }
```

A consumer is effectively a [generalized "fold" operation][fold](#fold),
and in fact each consumer will eventually be converted into a
[`Folder`](#folder). What makes a consumer special is that, like a
[`Producer`](#producer), it can be **split** into multiple consumers using
the `split_at` method. When a consumer is split, it produces two
consumers, as well as a **reducer**. The two consumers can be fed
items independently, and when they are done the reducer is used to
combine their two results into one. See [the `plumbing`
README][r](#r) for further details.

[r](#r): https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
[fold](#fold): Iterator::fold()

#### Required Methods

- `type Folder: 1`

- `type Reducer: 1`

- `type Result: 1`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)`

  Divide the consumer into two consumers, one processing items

- `fn into_folder(self: Self) -> <Self as >::Folder`

  Convert the consumer into a folder that can consume items

- `fn full(self: &Self) -> bool`

  Hint whether this `Consumer` would like to stop processing

### `Folder<Item>`

```rust
trait Folder<Item>: Sized { ... }
```

The `Folder` trait encapsulates [the standard fold
operation][fold](#fold).  It can be fed many items using the `consume`
method. At the end, once all items have been consumed, it can then
be converted (using `complete`) into a final value.

[fold](#fold): Iterator::fold()

#### Required Methods

- `type Result`

- `fn consume(self: Self, item: Item) -> Self`

  Consume next item and return new sequential state.

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

  Consume items from the iterator until full, and return new sequential state.

- `fn complete(self: Self) -> <Self as >::Result`

  Finish consuming items, produce final result.

- `fn full(self: &Self) -> bool`

  Hint whether this `Folder` would like to stop processing

### `Reducer<Result>`

```rust
trait Reducer<Result> { ... }
```

The reducer is the final step of a `Consumer` -- after a consumer
has been split into two parts, and each of those parts has been
fully processed, we are left with two results. The reducer is then
used to combine those two results into one. See [the `plumbing`
README][r](#r) for further details.

[r](#r): https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md

#### Required Methods

- `fn reduce(self: Self, left: Result, right: Result) -> Result`

  Reduce two final results into one; this is executed after a

### `UnindexedConsumer<I>`

```rust
trait UnindexedConsumer<I>: Consumer<I> { ... }
```

A stateless consumer can be freely copied. These consumers can be
used like regular consumers, but they also support a
`split_off_left` method that does not take an index to split, but
simply splits at some arbitrary point (`for_each`, for example,
produces an unindexed consumer).

#### Required Methods

- `fn split_off_left(self: &Self) -> Self`

  Splits off a "left" consumer and returns it. The `self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer`

  Creates a reducer that can be used to combine the results from

### `UnindexedProducer`

```rust
trait UnindexedProducer: Send + Sized { ... }
```

A variant on `Producer` which does not know its exact length or
cannot represent it in a `usize`. These producers act like
ordinary producers except that they cannot be told to split at a
particular point. Instead, you just ask them to split 'somewhere'.

(In principle, `Producer` could extend this trait; however, it
does not because to do so would require producers to carry their
own length with them.)

#### Required Methods

- `type Item`

- `fn split(self: Self) -> (Self, Option<Self>)`

  Split midway into a new producer if possible, otherwise return `None`.

- `fn fold_with<F>(self: Self, folder: F) -> F`

  Iterate the producer, feeding each element to `folder`, and

## Functions

### `bridge`

```rust
fn bridge<I, C>(par_iter: I, consumer: C) -> <C as >::Result
where
    I: IndexedParallelIterator,
    C: Consumer<<I as >::Item>
```

This helper function is used to "connect" a parallel iterator to a
consumer. It will convert the `par_iter` into a producer P and
then pull items from P and feed them to `consumer`, splitting and
creating parallel threads as needed.

This is useful when you are implementing your own parallel
iterators: it is often used as the definition of the
[`drive_unindexed`](#drive-unindexed) or [`drive`](#drive) methods.



### `bridge_producer_consumer`

```rust
fn bridge_producer_consumer<P, C>(len: usize, producer: P, consumer: C) -> <C as >::Result
where
    P: Producer,
    C: Consumer<<P as >::Item>
```

This helper function is used to "connect" a producer and a
consumer. You may prefer to call [`bridge()`](#bridge), which wraps this
function. This function will draw items from `producer` and feed
them to `consumer`, splitting and creating parallel tasks when
needed.

This is useful when you are implementing your own parallel
iterators: it is often used as the definition of the
[`drive_unindexed`](#drive-unindexed) or [`drive`](#drive) methods.



### `bridge_unindexed`

```rust
fn bridge_unindexed<P, C>(producer: P, consumer: C) -> <C as >::Result
where
    P: UnindexedProducer,
    C: UnindexedConsumer<<P as >::Item>
```

A variant of [`bridge_producer_consumer()`](#bridge-producer-consumer) where the producer is an unindexed producer.

