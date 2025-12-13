*[tinyvec](../index.md) / [tinyvec](index.md)*

---

# Module `tinyvec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TinyVecSplice`](#tinyvecsplice) | struct | Splicing iterator for `TinyVec` See [`TinyVec::splice`](TinyVec::<A>::splice) |
| [`TinyVec`](#tinyvec) | enum | A vector that starts inline, but can automatically move to the heap. |
| [`TinyVecDrain`](#tinyvecdrain) | enum | Draining iterator for `TinyVecDrain` |
| [`TinyVecIterator`](#tinyveciterator) | enum | Iterator for consuming an `TinyVec` and returning owned elements. |

## Structs

### `TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>>`

```rust
struct TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>> {
    parent: &'p mut TinyVec<A>,
    removal_start: usize,
    removal_end: usize,
    replacement: I,
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1215-1220`](../../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L1215-L1220)*

Splicing iterator for `TinyVec`
See [`TinyVec::splice`](TinyVec::<A>::splice)

#### Trait Implementations

##### `impl Any for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A, I> DoubleEndedIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

##### `impl<A: Array, I: Iterator<Item = <A as >::Item>> Drop for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-drop"></span>`fn drop(&mut self)`

##### `impl<A, I> ExactSizeIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, I> FusedIterator for TinyVecSplice<'p, A, I>`

##### `impl<U> Into for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecsplice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecsplice-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A, I> Iterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvecsplice-iterator-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

- <span id="tinyvecsplice-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyvecsplice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyvecsplice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `TinyVec<A: Array>`

```rust
enum TinyVec<A: Array> {
    Inline(ArrayVec<A>),
    Heap(alloc::vec::Vec<<A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:96-101`](../../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L96-L101)*

A vector that starts inline, but can automatically move to the heap.

* Requires the `alloc` feature

A `TinyVec` is either an Inline([`ArrayVec`](crate::ArrayVec::<A>)) or
Heap([`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)). The
interface for the type as a whole is a bunch of methods that just match on
the enum variant and then call the same method on the inner vec.

## Construction

Because it's an enum, you can construct a `TinyVec` simply by making an
`ArrayVec` or `Vec` and then putting it into the enum.

There is also a macro

```rust
use tinyvec::*;
let empty_tv = tiny_vec!([u8; 16]);
let some_ints = tiny_vec!([i32; 4] => 1, 2, 3);
```

#### Implementations

- <span id="tinyvec-is-heap"></span>`fn is_heap(&self) -> bool`

  Returns whether elements are on heap

- <span id="tinyvec-is-inline"></span>`fn is_inline(&self) -> bool`

  Returns whether elements are on stack

- <span id="tinyvec-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

  Shrinks the capacity of the vector as much as possible.\

  It is inlined if length is less than `A::CAPACITY`.

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 2] => 1, 2, 3);

  assert!(tv.is_heap());

  let _ = tv.pop();

  assert!(tv.is_heap());

  tv.shrink_to_fit();

  assert!(tv.is_inline());

  ```

- <span id="tinyvec-move-to-the-heap"></span>`fn move_to_the_heap(&mut self)`

  Moves the content of the TinyVec to the heap, if it's inline.

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3);

  assert!(tv.is_inline());

  tv.move_to_the_heap();

  assert!(tv.is_heap());

  ```

- <span id="tinyvec-move-to-the-heap-and-reserve"></span>`fn move_to_the_heap_and_reserve(&mut self, n: usize)`

  If TinyVec is inline, moves the content of it to the heap.

  Also reserves additional space.

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3);

  assert!(tv.is_inline());

  tv.move_to_the_heap_and_reserve(32);

  assert!(tv.is_heap());

  assert!(tv.capacity() >= 35);

  ```

- <span id="tinyvec-reserve"></span>`fn reserve(&mut self, n: usize)`

  Reserves additional space.

  Moves to the heap if array can't hold `n` more items

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3, 4);

  assert!(tv.is_inline());

  tv.reserve(1);

  assert!(tv.is_heap());

  assert!(tv.capacity() >= 5);

  ```

- <span id="tinyvec-reserve-exact"></span>`fn reserve_exact(&mut self, n: usize)`

  Reserves additional space.

  Moves to the heap if array can't hold `n` more items

  

  From [Vec::reserve_exact](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve_exact)

  ```text

  Note that the allocator may give the collection more space than it requests.

  Therefore, capacity can not be relied upon to be precisely minimal.

  Prefer `reserve` if future insertions are expected.

  ```

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3, 4);

  assert!(tv.is_inline());

  tv.reserve_exact(1);

  assert!(tv.is_heap());

  assert!(tv.capacity() >= 5);

  ```

- <span id="tinyvec-with-capacity"></span>`fn with_capacity(cap: usize) -> Self`

  Makes a new TinyVec with _at least_ the given capacity.

  

  If the requested capacity is less than or equal to the array capacity you

  get an inline vec. If it's greater than you get a heap vec.

  ```rust

  use tinyvec::*;

  let t = TinyVec::<[u8; 10]>::with_capacity(5);

  assert!(t.is_inline());

  assert!(t.capacity() >= 5);

  

  let t = TinyVec::<[u8; 10]>::with_capacity(20);

  assert!(t.is_heap());

  assert!(t.capacity() >= 20);

  ```

- <span id="tinyvec-into-boxed-slice"></span>`fn into_boxed_slice(self) -> alloc::boxed::Box<[<A as >::Item]>` — [`Array`](../index.md#array)

  Converts a `TinyVec<[T; N]>` into a `Box<[T]>`.

  

  - For `TinyVec::Heap(Vec<T>)`, it takes the `Vec<T>` and converts it into

    a `Box<[T]>` without heap reallocation.

  - For `TinyVec::Inline(inner_data)`, it first converts the `inner_data` to

    `Vec<T>`, then into a `Box<[T]>`. Requiring only a single heap

    allocation.

  

  ## Example

  

  ```rust

  use core::mem::size_of_val as mem_size_of;

  use tinyvec::TinyVec;

  

  // Initialize TinyVec with 256 elements (exceeding inline capacity)

  let v: TinyVec<[_; 128]> = (0u8..=255).collect();

  

  assert!(v.is_heap());

  assert_eq!(mem_size_of(&v), 136); // mem size of TinyVec<[u8; N]>: N+8

  assert_eq!(v.len(), 256);

  

  let boxed = v.into_boxed_slice();

  assert_eq!(mem_size_of(&boxed), 16); // mem size of Box<[u8]>: 16 bytes (fat pointer)

  assert_eq!(boxed.len(), 256);

  ```

- <span id="tinyvec-into-vec"></span>`fn into_vec(self) -> Vec<<A as >::Item>` — [`Array`](../index.md#array)

  Converts a `TinyVec<[T; N]>` into a `Vec<T>`.

  

  `v.into_vec()` is equivalent to `Into::<Vec<_>>::into(v)`.

  

  - For `TinyVec::Inline(_)`, `.into_vec()` **does not** offer a performance

    advantage over `.to_vec()`.

  - For `TinyVec::Heap(vec_data)`, `.into_vec()` will take `vec_data`

    without heap reallocation.

  

  ## Example

  

  ```rust

  use tinyvec::TinyVec;

  

  let v = TinyVec::from([0u8; 8]);

  let v2 = v.clone();

  

  let vec = v.into_vec();

  let vec2: Vec<_> = v2.into();

  

  assert_eq!(vec, vec2);

  ```

#### Trait Implementations

##### `impl Any for TinyVec<A>`

- <span id="tinyvec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<A: Array> AsMut for TinyVec<A>`

- <span id="tinyvec-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md#array)

##### `impl<A: Array> AsRef for TinyVec<A>`

- <span id="tinyvec-asref-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

##### `impl<A: Array> Binary for TinyVec<A>`

- <span id="tinyvec-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Borrow for TinyVec<A>`

- <span id="tinyvec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVec<A>`

- <span id="tinyvec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A> Clone for TinyVec<A>`

- <span id="tinyvec-clone"></span>`fn clone(&self) -> Self`

- <span id="tinyvec-clone-clone-from"></span>`fn clone_from(&mut self, o: &Self)`

##### `impl CloneToUninit for TinyVec<A>`

- <span id="tinyvec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: Array> Debug for TinyVec<A>`

- <span id="tinyvec-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Default for TinyVec<A>`

- <span id="tinyvec-default"></span>`fn default() -> Self`

##### `impl<A: Array> Deref for TinyVec<A>`

- <span id="tinyvec-deref-type-target"></span>`type Target = [<A as Array>::Item]`

- <span id="tinyvec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<A: Array> DerefMut for TinyVec<A>`

- <span id="tinyvec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<A: Array> Display for TinyVec<A>`

- <span id="tinyvec-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Eq for TinyVec<A>`

##### `impl<A: Array> Extend for TinyVec<A>`

- <span id="tinyvec-extend"></span>`fn extend<T: IntoIterator<Item = <A as >::Item>>(&mut self, iter: T)`

##### `impl<T> From for TinyVec<A>`

- <span id="tinyvec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: Array> FromIterator for TinyVec<A>`

- <span id="tinyvec-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = <A as >::Item>>(iter: T) -> Self`

##### `impl<A: Array> Hash for TinyVec<A>`

- <span id="tinyvec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for TinyVec<A>`

- <span id="tinyvec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="tinyvec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for TinyVec<A>`

- <span id="tinyvec-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<U> Into for TinyVec<A>`

- <span id="tinyvec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<A: Array> IntoIterator for TinyVec<A>`

- <span id="tinyvec-intoiterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvec-intoiterator-type-intoiter"></span>`type IntoIter = TinyVecIterator<A>`

- <span id="tinyvec-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<A: Array> LowerExp for TinyVec<A>`

- <span id="tinyvec-lowerexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> LowerHex for TinyVec<A>`

- <span id="tinyvec-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Octal for TinyVec<A>`

- <span id="tinyvec-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Ord for TinyVec<A>`

- <span id="tinyvec-ord-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<A: Array> PartialEq for TinyVec<A>`

- <span id="tinyvec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<A: Array> PartialOrd for TinyVec<A>`

- <span id="tinyvec-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<A: Array> Pointer for TinyVec<A>`

- <span id="tinyvec-pointer-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl Receiver for TinyVec<A>`

- <span id="tinyvec-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for TinyVec<A>`

- <span id="tinyvec-toowned-type-owned"></span>`type Owned = T`

- <span id="tinyvec-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tinyvec-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for TinyVec<A>`

- <span id="tinyvec-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for TinyVec<A>`

- <span id="tinyvec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyvec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVec<A>`

- <span id="tinyvec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyvec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<A: Array> UpperExp for TinyVec<A>`

- <span id="tinyvec-upperexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> UpperHex for TinyVec<A>`

- <span id="tinyvec-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `TinyVecDrain<'p, A: Array>`

```rust
enum TinyVecDrain<'p, A: Array> {
    Inline(ArrayVecDrain<'p, <A as >::Item>),
    Heap(vec::Drain<'p, <A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1166-1171`](../../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L1166-L1171)*

Draining iterator for `TinyVecDrain`

See [`TinyVecDrain::drain`](TinyVecDrain::<A>::drain)

#### Trait Implementations

##### `impl Any for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: Array> DoubleEndedIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T> From for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecdrain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvecdrain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tinyvecdrain-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-iterator-count"></span>`fn count(self) -> usize`

- <span id="tinyvecdrain-iterator-for-each"></span>`fn for_each<F: FnMut(<Self as >::Item)>(self, f: F)`

##### `impl<U> TryFrom for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyvecdrain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyvecdrain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TinyVecIterator<A: Array>`

```rust
enum TinyVecIterator<A: Array> {
    Inline(ArrayVecIterator<A>),
    Heap(alloc::vec::IntoIter<<A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1483-1488`](../../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L1483-L1488)*

Iterator for consuming an `TinyVec` and returning owned elements.

#### Implementations

- <span id="tinyveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

  Returns the remaining items of this iterator as a slice.

#### Trait Implementations

##### `impl Any for TinyVecIterator<A>`

- <span id="tinyveciterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TinyVecIterator<A>`

- <span id="tinyveciterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVecIterator<A>`

- <span id="tinyveciterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: Array> Debug for TinyVecIterator<A>`

- <span id="tinyveciterator-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for TinyVecIterator<A>`

- <span id="tinyveciterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: Array> FusedIterator for TinyVecIterator<A>`

##### `impl<U> Into for TinyVecIterator<A>`

- <span id="tinyveciterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyveciterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyveciterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for TinyVecIterator<A>`

- <span id="tinyveciterator-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyveciterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tinyveciterator-iterator-count"></span>`fn count(self) -> usize`

- <span id="tinyveciterator-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

##### `impl<U> TryFrom for TinyVecIterator<A>`

- <span id="tinyveciterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyveciterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVecIterator<A>`

- <span id="tinyveciterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyveciterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

