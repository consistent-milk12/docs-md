*[allocator_api2](../../index.md) / [stable](../index.md) / [unique](index.md)*

---

# Module `unique`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Unique`](#unique) | struct | A wrapper around a raw non-null `*mut T` that indicates that the possessor of this wrapper owns the referent. |

## Structs

### `Unique<T: ?Sized>`

```rust
struct Unique<T: ?Sized> {
    pointer: core::ptr::NonNull<T>,
    _marker: core::marker::PhantomData<T>,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/unique.rs:22-25`](../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/unique.rs#L22-L25)*

A wrapper around a raw non-null `*mut T` that indicates that the possessor
of this wrapper owns the referent. Useful for building abstractions like
`Box<T>`, `Vec<T>`, `String`, and `HashMap<K, V>`.

Unlike `*mut T`, `Unique<T>` behaves "as if" it were an instance of `T`.
It implements `Send`/`Sync` if `T` is `Send`/`Sync`. It also implies
the kind of strong aliasing guarantees an instance of `T` can expect:
the referent of the pointer should not be modified without a unique path to
its owning Unique.

If you're uncertain of whether it's correct to use `Unique` for your purposes,
consider using `NonNull`, which has weaker semantics.

Unlike `*mut T`, the pointer must always be non-null, even if the pointer
is never dereferenced. This is so that enums may use this forbidden value
as a discriminant -- `Option<Unique<T>>` has the same size as `Unique<T>`.
However the pointer may still dangle if it isn't dereferenced.

Unlike `*mut T`, `Unique<T>` is covariant over `T`. This should always be correct
for any type which upholds Unique's aliasing requirements.

#### Implementations

- <span id="unique-new-unchecked"></span>`const unsafe fn new_unchecked(ptr: *mut T) -> Self`

- <span id="unique-as-ptr"></span>`const fn as_ptr(self) -> *mut T`

- <span id="unique-as-non-null-ptr"></span>`const fn as_non_null_ptr(self) -> NonNull<T>`

- <span id="unique-as-ref"></span>`const unsafe fn as_ref(&self) -> &T`

- <span id="unique-as-mut"></span>`unsafe fn as_mut(&mut self) -> &mut T`

#### Trait Implementations

##### `impl<T: ?Sized> Clone for Unique<T>`

- <span id="unique-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: ?Sized> Copy for Unique<T>`

##### `impl<T: Send + ?Sized> Send for Unique<T>`

##### `impl<T: Sync + ?Sized> Sync for Unique<T>`

