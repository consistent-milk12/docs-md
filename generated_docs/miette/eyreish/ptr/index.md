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

##### `impl<T> Clone for Own<T>`

- <span id="own-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Copy for Own<T>`

##### `impl OwoColorize for Own<T>`

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

##### `impl<T> Clone for Ref<'_, T>`

- <span id="ref-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Copy for Ref<'_, T>`

##### `impl OwoColorize for Ref<'a, T>`

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

##### `impl<T> Clone for Mut<'_, T>`

- <span id="mut-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Copy for Mut<'_, T>`

##### `impl OwoColorize for Mut<'a, T>`

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

