*[proc_macro2](../index.md) / [rcvec](index.md)*

---

# Module `rcvec`

## Structs

### `RcVec<T>`

```rust
struct RcVec<T> {
    inner: alloc::rc::Rc<Vec<T>>,
}
```

#### Implementations

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn iter(self: &Self) -> slice::Iter<'_, T>`

- `fn make_mut(self: &mut Self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- `fn get_mut(self: &mut Self) -> Option<RcVecMut<'_, T>>` — [`RcVecMut`](#rcvecmut)

- `fn make_owned(self: Self) -> RcVecBuilder<T>` — [`RcVecBuilder`](#rcvecbuilder)

#### Trait Implementations

##### `impl<T> Clone for RcVec<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> RefUnwindSafe for RcVec<T>`

### `RcVecBuilder<T>`

```rust
struct RcVecBuilder<T> {
    inner: Vec<T>,
}
```

#### Implementations

- `fn new() -> Self`

- `fn with_capacity(cap: usize) -> Self`

- `fn push(self: &mut Self, element: T)`

- `fn extend(self: &mut Self, iter: impl IntoIterator<Item = T>)`

- `fn as_mut(self: &mut Self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- `fn build(self: Self) -> RcVec<T>` — [`RcVec`](#rcvec)

#### Trait Implementations

##### `impl<T> IntoIterator for RcVecBuilder<T>`

- `type Item = T`

- `type IntoIter = RcVecIntoIter<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

### `RcVecMut<'a, T>`

```rust
struct RcVecMut<'a, T> {
    inner: &'a mut Vec<T>,
}
```

#### Implementations

- `fn push(self: &mut Self, element: T)`

- `fn extend(self: &mut Self, iter: impl IntoIterator<Item = T>)`

- `fn as_mut(self: &mut Self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- `fn take(self: Self) -> RcVecBuilder<T>` — [`RcVecBuilder`](#rcvecbuilder)

### `RcVecIntoIter<T>`

```rust
struct RcVecIntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RcVecIntoIter<T>`

- `fn clone(self: &Self) -> RcVecIntoIter<T>` — [`RcVecIntoIter`](#rcvecintoiter)

##### `impl<I> IntoIterator for RcVecIntoIter<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for RcVecIntoIter<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

