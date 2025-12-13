*[miette](../../index.md) / [eyreish](../index.md) / [ptr](index.md)*

---

# Module `ptr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Own`](#own) | struct | A raw pointer that owns its pointee |
| [`Ref`](#ref) | struct | A raw pointer that represents a shared borrow of its pointee |
| [`Mut`](#mut) | struct | A raw pointer that represents a unique borrow of its pointee |
| [`CastTo`](#castto) | trait |  |

## Structs

### `Own<T>`

```rust
struct Own<T>
where
    T: ?Sized {
    ptr: std::ptr::NonNull<T>,
}
```

*Defined in [`miette-7.6.0/src/eyreish/ptr.rs:5-10`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/ptr.rs#L5-L10)*

A raw pointer that owns its pointee

#### Implementations

- <span id="own-new"></span>`fn new(ptr: Box<T>) -> Self`

- <span id="own-cast"></span>`fn cast<U: CastTo>(self) -> Own<<U as >::Target>` — [`Own`](#own), [`CastTo`](#castto)

- <span id="own-boxed"></span>`unsafe fn boxed(self) -> Box<T>`

- <span id="own-by-ref"></span>`const fn by_ref<'a>(&self) -> Ref<'a, T>` — [`Ref`](#ref)

- <span id="own-by-mut"></span>`fn by_mut<'a>(self) -> Mut<'a, T>` — [`Mut`](#mut)

#### Trait Implementations

##### `impl<T> Any for Own<T>`

- <span id="own-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Own<T>`

- <span id="own-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Own<T>`

- <span id="own-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Own<T>`

- <span id="own-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Own<T>`

- <span id="own-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Copy for Own<T>`

##### `impl<T> From for Own<T>`

- <span id="own-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Own<T>`

- <span id="own-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Own<T>`

##### `impl<T> Send for Own<T>`

##### `impl<T> Sync for Own<T>`

##### `impl<T> ToOwned for Own<T>`

- <span id="own-toowned-type-owned"></span>`type Owned = T`

- <span id="own-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="own-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Own<T>`

- <span id="own-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="own-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Own<T>`

- <span id="own-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="own-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ref<'a, T>`

```rust
struct Ref<'a, T>
where
    T: ?Sized {
    ptr: std::ptr::NonNull<T>,
    lifetime: std::marker::PhantomData<&'a T>,
}
```

*Defined in [`miette-7.6.0/src/eyreish/ptr.rs:64-70`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/ptr.rs#L64-L70)*

A raw pointer that represents a shared borrow of its pointee

#### Implementations

- <span id="ref-new"></span>`fn new(ptr: &'a T) -> Self`

- <span id="ref-from-raw"></span>`const fn from_raw(ptr: NonNull<T>) -> Self`

- <span id="ref-cast"></span>`fn cast<U: CastTo>(self) -> Ref<'a, <U as >::Target>` — [`Ref`](#ref), [`CastTo`](#castto)

- <span id="ref-by-mut"></span>`fn by_mut(self) -> Mut<'a, T>` — [`Mut`](#mut)

- <span id="ref-as-ptr"></span>`const fn as_ptr(self) -> *const T`

- <span id="ref-deref"></span>`unsafe fn deref(self) -> &'a T`

#### Trait Implementations

##### `impl<T> Any for Ref<'a, T>`

- <span id="ref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ref<'a, T>`

- <span id="ref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ref<'a, T>`

- <span id="ref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Ref<'_, T>`

- <span id="ref-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Ref<'a, T>`

- <span id="ref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Copy for Ref<'_, T>`

##### `impl<T> From for Ref<'a, T>`

- <span id="ref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Ref<'a, T>`

- <span id="ref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Ref<'a, T>`

##### `impl<T> ToOwned for Ref<'a, T>`

- <span id="ref-toowned-type-owned"></span>`type Owned = T`

- <span id="ref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Ref<'a, T>`

- <span id="ref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Ref<'a, T>`

- <span id="ref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Mut<'a, T>`

```rust
struct Mut<'a, T>
where
    T: ?Sized {
    ptr: std::ptr::NonNull<T>,
    lifetime: std::marker::PhantomData<&'a mut T>,
}
```

*Defined in [`miette-7.6.0/src/eyreish/ptr.rs:127-133`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/ptr.rs#L127-L133)*

A raw pointer that represents a unique borrow of its pointee

#### Implementations

- <span id="mut-cast"></span>`fn cast<U: CastTo>(self) -> Mut<'a, <U as >::Target>` — [`Mut`](#mut), [`CastTo`](#castto)

- <span id="mut-by-ref"></span>`const fn by_ref(self) -> Ref<'a, T>` — [`Ref`](#ref)

- <span id="mut-extend"></span>`fn extend<'b>(self) -> Mut<'b, T>` — [`Mut`](#mut)

- <span id="mut-deref-mut"></span>`unsafe fn deref_mut(self) -> &'a mut T`

#### Trait Implementations

##### `impl<T> Any for Mut<'a, T>`

- <span id="mut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mut<'a, T>`

- <span id="mut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mut<'a, T>`

- <span id="mut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Mut<'_, T>`

- <span id="mut-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Mut<'a, T>`

- <span id="mut-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Copy for Mut<'_, T>`

##### `impl<T> From for Mut<'a, T>`

- <span id="mut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Mut<'a, T>`

- <span id="mut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Mut<'a, T>`

##### `impl<T> ToOwned for Mut<'a, T>`

- <span id="mut-toowned-type-owned"></span>`type Owned = T`

- <span id="mut-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mut-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Mut<'a, T>`

- <span id="mut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Mut<'a, T>`

- <span id="mut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `CastTo`

```rust
trait CastTo { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/ptr.rs:182-184`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/ptr.rs#L182-L184)*

#### Associated Types

- `type Target`

#### Implementors

- `T`

