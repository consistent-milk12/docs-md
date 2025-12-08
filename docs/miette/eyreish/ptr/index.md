*[miette](../../index.md) / [eyreish](../index.md) / [ptr](index.md)*

---

# Module `ptr`

## Structs

### `Own<T>`

```rust
struct Own<T>
where
    T: ?Sized {
    ptr: std::ptr::NonNull<T>,
}
```

A raw pointer that owns its pointee

#### Implementations

- `fn new(ptr: Box<T>) -> Self`

- `fn cast<U: CastTo>(self: Self) -> Own<<U as >::Target>` — [`Own`](#own), [`CastTo`](#castto)

- `unsafe fn boxed(self: Self) -> Box<T>`

- `const fn by_ref<'a>(self: &Self) -> Ref<'a, T>` — [`Ref`](#ref)

- `fn by_mut<'a>(self: Self) -> Mut<'a, T>` — [`Mut`](#mut)

#### Trait Implementations

##### `impl<T> Clone for Own<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Copy for Own<T>`

##### `impl<D> OwoColorize for Own<T>`

##### `impl<T> Send for Own<T>`

##### `impl<T> Sync for Own<T>`

### `Ref<'a, T>`

```rust
struct Ref<'a, T>
where
    T: ?Sized {
    ptr: std::ptr::NonNull<T>,
    lifetime: std::marker::PhantomData<&'a T>,
}
```

A raw pointer that represents a shared borrow of its pointee

#### Implementations

- `fn new(ptr: &'a T) -> Self`

- `const fn from_raw(ptr: NonNull<T>) -> Self`

- `fn cast<U: CastTo>(self: Self) -> Ref<'a, <U as >::Target>` — [`Ref`](#ref), [`CastTo`](#castto)

- `fn by_mut(self: Self) -> Mut<'a, T>` — [`Mut`](#mut)

- `const fn as_ptr(self: Self) -> *const T`

- `unsafe fn deref(self: Self) -> &'a T`

#### Trait Implementations

##### `impl<T> Clone for Ref<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Copy for Ref<'_, T>`

##### `impl<D> OwoColorize for Ref<'a, T>`

### `Mut<'a, T>`

```rust
struct Mut<'a, T>
where
    T: ?Sized {
    ptr: std::ptr::NonNull<T>,
    lifetime: std::marker::PhantomData<&'a mut T>,
}
```

A raw pointer that represents a unique borrow of its pointee

#### Implementations

- `fn cast<U: CastTo>(self: Self) -> Mut<'a, <U as >::Target>` — [`Mut`](#mut), [`CastTo`](#castto)

- `const fn by_ref(self: Self) -> Ref<'a, T>` — [`Ref`](#ref)

- `fn extend<'b>(self: Self) -> Mut<'b, T>` — [`Mut`](#mut)

- `unsafe fn deref_mut(self: Self) -> &'a mut T`

#### Trait Implementations

##### `impl<T> Clone for Mut<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Copy for Mut<'_, T>`

##### `impl<D> OwoColorize for Mut<'a, T>`

## Traits

### `CastTo`

```rust
trait CastTo { ... }
```

#### Required Methods

- `type Target`

