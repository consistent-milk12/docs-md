*[rayon](../../index.md) / [iter](../index.md) / [collect](index.md)*

---

# Module `collect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`consumer`](#consumer) | mod |  |
| [`collect_into_vec`](#collect-into-vec) | fn | Collects the results of the exact iterator into the specified vector. |
| [`special_extend`](#special-extend) | fn | Collects the results of the iterator into the specified vector. |
| [`unzip_into_vecs`](#unzip-into-vecs) | fn | Unzips the results of the exact iterator into the specified vectors. |
| [`collect_with_consumer`](#collect-with-consumer) | fn | Create a consumer on the slice of memory we are collecting into. |

## Modules

- [`consumer`](consumer/index.md)

## Functions

### `collect_into_vec`

```rust
fn collect_into_vec<I, T>(pi: I, v: &mut Vec<T>)
where
    I: IndexedParallelIterator<Item = T>,
    T: Send
```

*Defined in [`rayon-1.11.0/src/iter/collect/mod.rs:13-21`](../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/mod.rs#L13-L21)*

Collects the results of the exact iterator into the specified vector.

This is called by `IndexedParallelIterator::collect_into_vec`.

### `special_extend`

```rust
fn special_extend<I, T>(pi: I, len: usize, v: &mut Vec<T>)
where
    I: ParallelIterator<Item = T>,
    T: Send
```

*Defined in [`rayon-1.11.0/src/iter/collect/mod.rs:34-40`](../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/mod.rs#L34-L40)*

Collects the results of the iterator into the specified vector.

Technically, this only works for `IndexedParallelIterator`, but we're faking a
bit of specialization here until Rust can do that natively.  Callers are
using `opt_len` to find the length before calling this, and only exact
iterators will return anything but `None` there.

Since the type system doesn't understand that contract, we have to allow
*any* `ParallelIterator` here, and `CollectConsumer` has to also implement
`UnindexedConsumer`.  That implementation panics `unreachable!` in case
there's a bug where we actually do try to use this unindexed.

### `unzip_into_vecs`

```rust
fn unzip_into_vecs<I, A, B>(pi: I, left: &mut Vec<A>, right: &mut Vec<B>)
where
    I: IndexedParallelIterator<Item = (A, B)>,
    A: Send,
    B: Send
```

*Defined in [`rayon-1.11.0/src/iter/collect/mod.rs:45-65`](../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/mod.rs#L45-L65)*

Unzips the results of the exact iterator into the specified vectors.

This is called by `IndexedParallelIterator::unzip_into_vecs`.

### `collect_with_consumer`

```rust
fn collect_with_consumer<T, F>(vec: &mut Vec<T>, len: usize, scope_fn: F)
where
    T: Send,
    F: FnOnce(self::consumer::CollectConsumer<'_, T>) -> self::consumer::CollectResult<'_, T>
```

*Defined in [`rayon-1.11.0/src/iter/collect/mod.rs:75-114`](../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/mod.rs#L75-L114)*

Create a consumer on the slice of memory we are collecting into.

The consumer needs to be used inside the scope function, and the
complete collect result passed back.

This method will verify the collect result, and panic if the slice
was not fully written into. Otherwise, in the successful case,
the vector is complete with the collected result.

