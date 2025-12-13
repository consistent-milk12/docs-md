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

*Defined in [`allocator-api2-0.2.21/src/stable/unique.rs:22-25`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/unique.rs#L22-L25)*

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

  Creates a new `Unique`.

  

  # Safety

  

  `ptr` must be non-null.

- <span id="unique-as-ptr"></span>`const fn as_ptr(self) -> *mut T`

  Acquires the underlying `*mut` pointer.

- <span id="unique-as-non-null-ptr"></span>`const fn as_non_null_ptr(self) -> NonNull<T>`

  Acquires the underlying `*mut` pointer.

- <span id="unique-as-ref"></span>`const unsafe fn as_ref(&self) -> &T`

  Dereferences the content.

  

  The resulting lifetime is bound to self so this behaves "as if"

  it were actually an instance of T that is getting borrowed. If a longer

  (unbound) lifetime is needed, use `&*my_ptr.as_ptr()`.

- <span id="unique-as-mut"></span>`unsafe fn as_mut(&mut self) -> &mut T`

  Mutably dereferences the content.

  

  The resulting lifetime is bound to self so this behaves "as if"

  it were actually an instance of T that is getting borrowed. If a longer

  (unbound) lifetime is needed, use `&mut *my_ptr.as_ptr()`.

#### Trait Implementations

##### `impl<T> Any for Unique<T>`

- <span id="unique-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unique<T>`

- <span id="unique-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unique<T>`

- <span id="unique-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized> Clone for Unique<T>`

- <span id="unique-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Unique<T>`

- <span id="unique-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: ?Sized> Copy for Unique<T>`

##### `impl<T> From for Unique<T>`

- <span id="unique-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Unique<T>`

- <span id="unique-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Send + ?Sized> Send for Unique<T>`

##### `impl<T: Sync + ?Sized> Sync for Unique<T>`

##### `impl<T> ToOwned for Unique<T>`

- <span id="unique-toowned-type-owned"></span>`type Owned = T`

- <span id="unique-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unique-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Unique<T>`

- <span id="unique-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unique-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Unique<T>`

- <span id="unique-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unique-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

