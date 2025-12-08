*[allocator_api2](../../index.md) / [stable](../index.md) / [unique](index.md)*

---

# Module `unique`

## Structs

### `Unique<T: ?Sized>`

```rust
struct Unique<T: ?Sized> {
    pointer: core::ptr::NonNull<T>,
    _marker: core::marker::PhantomData<T>,
}
```

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

- `const unsafe fn new_unchecked(ptr: *mut T) -> Self`

- `const fn as_ptr(self: Self) -> *mut T`

- `const fn as_non_null_ptr(self: Self) -> NonNull<T>`

- `const unsafe fn as_ref(self: &Self) -> &T`

- `unsafe fn as_mut(self: &mut Self) -> &mut T`

#### Trait Implementations

##### `impl<T: ?Sized> Clone for Unique<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: ?Sized> Copy for Unique<T>`

##### `impl<T: Send + ?Sized> Send for Unique<T>`

##### `impl<T: Sync + ?Sized> Sync for Unique<T>`

