*[allocator_api2](../../index.md) / [stable](../index.md) / [vec](index.md)*

---

# Module `vec`

A contiguous growable array type with heap-allocated contents, written
`Vec<T>`.

Vectors have *O*(1) indexing, amortized *O*(1) push (to the end) and
*O*(1) pop (from the end).

Vectors ensure they never allocate more than `isize::MAX` bytes.

# Examples

You can explicitly create a [`Vec`](#vec) with `Vec::new`:

```rust
let v: Vec<i32> = Vec::new();
```

...or by using the `vec!` macro:

```rust
let v: Vec<i32> = vec![];

let v = vec![1, 2, 3, 4, 5];

let v = vec![0; 10]; // ten zeroes
```

You can `push` values onto the end of a vector (which will grow the vector
as needed):

```rust
let mut v = vec![1, 2];

v.push(3);
```

Popping values works in much the same way:

```rust
let mut v = vec![1, 2];

let two = v.pop();
```

Vectors also support indexing (through the [`Index`](../../../clap_builder/index.md) and `IndexMut` traits):

```rust
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;
```


## Contents

- [Modules](#modules)
  - [`splice`](#splice)
  - [`drain`](#drain)
  - [`into_iter`](#into-iter)
  - [`partial_eq`](#partial-eq)
  - [`set_len_on_drop`](#set-len-on-drop)
- [Structs](#structs)
  - [`Splice`](#splice)
  - [`Drain`](#drain)
  - [`IntoIter`](#intoiter)
  - [`Vec`](#vec)
  - [`ExtendElement`](#extendelement)
- [Traits](#traits)
  - [`ExtendWith`](#extendwith)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`splice`](#splice) | mod |  |
| [`drain`](#drain) | mod |  |
| [`into_iter`](#into-iter) | mod |  |
| [`partial_eq`](#partial-eq) | mod |  |
| [`set_len_on_drop`](#set-len-on-drop) | mod |  |
| [`Splice`](#splice) | struct |  |
| [`Drain`](#drain) | struct |  |
| [`IntoIter`](#intoiter) | struct |  |
| [`Vec`](#vec) | struct | A contiguous growable array type, written as `Vec<T>`, short for 'vector'. |
| [`ExtendElement`](#extendelement) | struct |  |
| [`ExtendWith`](#extendwith) | trait |  |

## Modules

- [`splice`](splice/index.md)
- [`drain`](drain/index.md)
- [`into_iter`](into_iter/index.md)
- [`partial_eq`](partial_eq/index.md)
- [`set_len_on_drop`](set_len_on_drop/index.md)

## Structs

### `Splice<'a, I: Iterator + 'a, A: Allocator + 'a>`

```rust
struct Splice<'a, I: Iterator + 'a, A: Allocator + 'a> {
    drain: super::Drain<'a, <I as >::Item, A>,
    replace_with: I,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/splice.rs:21-24`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/splice.rs#L21-L24)*

A splicing iterator for `Vec`.

This struct is created by `Vec::splice()`.
See its documentation for more.

# Example

```rust
let mut v = vec![0, 1, 2];
let new = [7, 8];
let iter: std::vec::Splice<_> = v.splice(1.., new);
```

#### Trait Implementations

##### `impl Any for Splice<'a, I, A>`

- <span id="splice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Splice<'a, I, A>`

- <span id="splice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Splice<'a, I, A>`

- <span id="splice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: fmt::Debug + Iterator + 'a, A: fmt::Debug + Allocator + 'a> Debug for Splice<'a, I, A>`

- <span id="splice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator, A: Allocator> DoubleEndedIterator for Splice<'_, I, A>`

- <span id="splice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I: Iterator, A: Allocator> Drop for Splice<'_, I, A>`

- <span id="splice-drop"></span>`fn drop(&mut self)`

##### `impl<I: Iterator, A: Allocator> ExactSizeIterator for Splice<'_, I, A>`

##### `impl<T> From for Splice<'a, I, A>`

- <span id="splice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Splice<'a, I, A>`

- <span id="splice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Splice<'a, I, A>`

- <span id="splice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splice-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator, A: Allocator> Iterator for Splice<'_, I, A>`

- <span id="splice-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="splice-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for Splice<'a, I, A>`

- <span id="splice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Splice<'a, I, A>`

- <span id="splice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'a, T: 'a, A: Allocator + 'a>`

```rust
struct Drain<'a, T: 'a, A: Allocator + 'a> {
    tail_start: usize,
    tail_len: usize,
    iter: slice::Iter<'a, T>,
    vec: core::ptr::NonNull<super::Vec<T, A>>,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/drain.rs:22-30`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/drain.rs#L22-L30)*

A draining iterator for `Vec<T>`.

This `struct` is created by `Vec::drain`.
See its documentation for more.

# Example

```rust
let mut v = vec![0, 1, 2];
let iter: std::vec::Drain<_> = v.drain(..);
```

#### Fields

- **`tail_start`**: `usize`

  Index of tail to preserve

- **`tail_len`**: `usize`

  Length of tail

- **`iter`**: `slice::Iter<'a, T>`

  Current remaining range to remove

#### Implementations

- <span id="superdrain-fill"></span>`unsafe fn fill<I: Iterator<Item = T>>(&mut self, replace_with: &mut I) -> bool`

  The range from `self.vec.len` to `self.tail_start` contains elements

  that have been moved out.

  Fill that range as much as possible with new elements from the `replace_with` iterator.

  Returns `true` if we filled the entire range. (`replace_with.next()` didn’t return `None`.)

- <span id="superdrain-move-tail"></span>`unsafe fn move_tail(&mut self, additional: usize)`

  Makes room for inserting more elements before the tail.

#### Trait Implementations

##### `impl<T> Any for Drain<'a, T, A>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T, A: Allocator> AsRef for Drain<'a, T, A>`

- <span id="drain-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T> Borrow for Drain<'a, T, A>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a, T, A>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Drain<'_, T, A>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for Drain<'_, T, A>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

##### `impl<T> From for Drain<'a, T, A>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<T, U> Into for Drain<'a, T, A>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Drain<'a, T, A>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- <span id="drain-iterator-type-item"></span>`type Item = T`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A>`

##### `impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A>`

##### `impl<T, U> TryFrom for Drain<'a, T, A>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Drain<'a, T, A>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter<T, A: Allocator>`

```rust
struct IntoIter<T, A: Allocator> {
    buf: core::ptr::NonNull<T>,
    phantom: core::marker::PhantomData<T>,
    cap: usize,
    alloc: core::mem::ManuallyDrop<A>,
    ptr: *const T,
    end: *const T,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/into_iter.rs:27-36`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/into_iter.rs#L27-L36)*

An iterator that moves out of a vector.

This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)
(provided by the `IntoIterator` trait).

# Example

```rust
let v = vec![0, 1, 2];
let iter: std::vec::IntoIter<_> = v.into_iter();
```

#### Implementations

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Returns the remaining items of this iterator as a slice.

  

  # Examples

  

  ```rust

  let vec = vec!['a', 'b', 'c'];

  let mut into_iter = vec.into_iter();

  assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);

  let _ = into_iter.next().unwrap();

  assert_eq!(into_iter.as_slice(), &['b', 'c']);

  ```

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

  Returns the remaining items of this iterator as a mutable slice.

  

  # Examples

  

  ```rust

  let vec = vec!['a', 'b', 'c'];

  let mut into_iter = vec.into_iter();

  assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);

  into_iter.as_mut_slice()[2] = 'z';

  assert_eq!(into_iter.next().unwrap(), 'a');

  assert_eq!(into_iter.next().unwrap(), 'b');

  assert_eq!(into_iter.next().unwrap(), 'z');

  ```

- <span id="intoiter-allocator"></span>`fn allocator(&self) -> &A`

  Returns a reference to the underlying allocator.

- <span id="intoiter-as-raw-mut-slice"></span>`fn as_raw_mut_slice(&mut self) -> *mut [T]`

#### Trait Implementations

##### `impl<T> Any for IntoIter<T, A>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T, A: Allocator> AsRef for IntoIter<T, A>`

- <span id="intoiter-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T> Borrow for IntoIter<T, A>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<T, A>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for IntoIter<T, A>`

- <span id="intoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug, A: Allocator> Debug for IntoIter<T, A>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for IntoIter<T, A>`

- <span id="intoiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A>`

##### `impl<T> From for IntoIter<T, A>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, A: Allocator> FusedIterator for IntoIter<T, A>`

##### `impl<T, U> Into for IntoIter<T, A>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter<T, A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for IntoIter<T, A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-count"></span>`fn count(self) -> usize`

##### `impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A>`

##### `impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A>`

##### `impl<T> ToOwned for IntoIter<T, A>`

- <span id="intoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="intoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IntoIter<T, A>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntoIter<T, A>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Vec<T, A: Allocator>`

```rust
struct Vec<T, A: Allocator> {
    buf: super::raw_vec::RawVec<T, A>,
    len: usize,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/mod.rs:348-351`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/mod.rs#L348-L351)*

A contiguous growable array type, written as `Vec<T>`, short for 'vector'.

# Examples

```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1, 2, 3].iter().copied());

for x in &vec {
    println!("{x}");
}
assert_eq!(vec, [7, 1, 2, 3]);
```

The `vec!` macro is provided for convenient initialization:

```rust
let mut vec1 = vec![1, 2, 3];
vec1.push(4);
let vec2 = Vec::from([1, 2, 3, 4]);
assert_eq!(vec1, vec2);
```

It can also initialize each element of a `Vec<T>` with a given value.
This may be more efficient than performing allocation and initialization
in separate steps, especially when initializing a vector of zeros:

```rust
let vec = vec![0; 5];
assert_eq!(vec, [0, 0, 0, 0, 0]);

// The following is equivalent, but potentially slower:
let mut vec = Vec::with_capacity(5);
vec.resize(5, 0);
assert_eq!(vec, [0, 0, 0, 0, 0]);
```

For more information, see
[Capacity and Reallocation](#capacity-and-reallocation).

Use a `Vec<T>` as an efficient stack:

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    // Prints 3, 2, 1
    println!("{top}");
}
```

# Indexing

The `Vec` type allows to access values by index, because it implements the
[`Index`](../../../clap_builder/index.md) trait. An example will be more explicit:

```rust
let v = vec![0, 2, 4, 6];
println!("{}", v[1]); // it will display '2'
```

However be careful: if you try to access an index which isn't in the `Vec`,
your software will panic! You cannot do this:

```should_panic
let v = vec![0, 2, 4, 6];
println!("{}", v[6]); // it will panic!
```

Use [`get`](#get) and `get_mut` if you want to check whether the index is in
the `Vec`.

# Slicing

A `Vec` can be mutable. On the other hand, slices are read-only objects.
To get a [slice][prim@slice], use `&`. Example:

```rust
fn read_slice(slice: &[usize]) {
    // ...
}

let v = vec![0, 1];
read_slice(&v);

// ... and that's all!
// you can also do it like this:
let u: &[usize] = &v;
// or like this:
let u: &[_] = &v;
```

In Rust, it's more common to pass slices as arguments rather than vectors
when you just want to provide read access. The same goes for `String` and
`&str`.

# Capacity and reallocation

The capacity of a vector is the amount of space allocated for any future
elements that will be added onto the vector. This is not to be confused with
the *length* of a vector, which specifies the number of actual elements
within the vector. If a vector's length exceeds its capacity, its capacity
will automatically be increased, but its elements will have to be
reallocated.

For example, a vector with capacity 10 and length 0 would be an empty vector
with space for 10 more elements. Pushing 10 or fewer elements onto the
vector will not change its capacity or cause reallocation to occur. However,
if the vector's length is increased to 11, it will have to reallocate, which
can be slow. For this reason, it is recommended to use `Vec::with_capacity`
whenever possible to specify how big the vector is expected to get.

# Guarantees

Due to its incredibly fundamental nature, `Vec` makes a lot of guarantees
about its design. This ensures that it's as low-overhead as possible in
the general case, and can be correctly manipulated in primitive ways
by unsafe code. Note that these guarantees refer to an unqualified `Vec<T>`.
If additional type parameters are added (e.g., to support custom allocators),
overriding their defaults may change the behavior.

Most fundamentally, `Vec` is and always will be a (pointer, capacity, length)
triplet. No more, no less. The order of these fields is completely
unspecified, and you should use the appropriate methods to modify these.
The pointer will never be null, so this type is null-pointer-optimized.

However, the pointer might not actually point to allocated memory. In particular,
if you construct a `Vec` with capacity 0 via `Vec::new`, [`vec![]`]`vec!`,
[`Vec::with_capacity(0)`]`Vec::with_capacity`, or by calling `shrink_to_fit`
on an empty Vec, it will not allocate memory. Similarly, if you store zero-sized
types inside a `Vec`, it will not allocate space for them. *Note that in this case
the `Vec` might not report a `capacity` of 0*. `Vec` will allocate if and only
if <code>[mem::size_of::\<T>]\() * [`capacity`](../../../compact_str/repr/capacity/index.md)\() > 0</code>. In general, `Vec`'s allocation
details are very subtle --- if you intend to allocate memory using a `Vec`
and use it for something else (either to pass to unsafe code, or to build your
own memory-backed collection), be sure to deallocate this memory by using
`from_raw_parts` to recover the `Vec` and then dropping it.

If a `Vec` *has* allocated memory, then the memory it points to is on the heap
(as defined by the allocator Rust is configured to use by default), and its
pointer points to `len` initialized, contiguous elements in order (what
you would see if you coerced it to a slice), followed by <code>[`capacity`](../../../compact_str/repr/capacity/index.md) - [`len`](../../../rayon/iter/len/index.md)</code>
logically uninitialized, contiguous elements.

A vector containing the elements `'a'` and `'b'` with capacity 4 can be
visualized as below. The top part is the `Vec` struct, it contains a
pointer to the head of the allocation in the heap, length and capacity.
The bottom part is the allocation on the heap, a contiguous memory block.

```text
            ptr      len  capacity
       +--------+--------+--------+
       | 0x0123 |      2 |      4 |
       +--------+--------+--------+
            |
            v
Heap   +--------+--------+--------+--------+
       |    'a' |    'b' | uninit | uninit |
       +--------+--------+--------+--------+
```

- **uninit** represents memory that is not initialized, see `MaybeUninit`.
- Note: the ABI is not stable and `Vec` makes no guarantees about its memory
  layout (including the order of fields).

`Vec` will never perform a "small optimization" where elements are actually
stored on the stack for two reasons:

* It would make it more difficult for unsafe code to correctly manipulate
  a `Vec`. The contents of a `Vec` wouldn't have a stable address if it were
  only moved, and it would be more difficult to determine if a `Vec` had
  actually allocated memory.

* It would penalize the general case, incurring an additional branch
  on every access.

`Vec` will never automatically shrink itself, even if completely empty. This
ensures no unnecessary allocations or deallocations occur. Emptying a `Vec`
and then filling it back up to the same `len` should incur no calls to
the allocator. If you wish to free up unused memory, use
`shrink_to_fit` or `shrink_to`.

`push` and `insert` will never (re)allocate if the reported capacity is
sufficient. `push` and `insert` *will* (re)allocate if
<code>[`len`](../../../rayon/iter/len/index.md) == [`capacity`](../../../compact_str/repr/capacity/index.md)</code>. That is, the reported capacity is completely
accurate, and can be relied on. It can even be used to manually free the memory
allocated by a `Vec` if desired. Bulk insertion methods *may* reallocate, even
when not necessary.

`Vec` does not guarantee any particular growth strategy when reallocating
when full, nor when `reserve` is called. The current strategy is basic
and it may prove desirable to use a non-constant growth factor. Whatever
strategy is used will of course guarantee *O*(1) amortized `push`.

`vec![x; n]`, `vec![a, b, c, d]`, and
[`Vec::with_capacity(n)`]`Vec::with_capacity`, will all produce a `Vec`
with exactly the requested capacity. If <code>[`len`](../../../rayon/iter/len/index.md) == [`capacity`](../../../compact_str/repr/capacity/index.md)</code>,
(as is the case for the `vec!` macro), then a `Vec<T>` can be converted to
and from a [`Box<[T]>`][owned slice] without reallocating or moving the elements.

`Vec` will not specifically overwrite any data that is removed from it,
but also won't specifically preserve it. Its uninitialized memory is
scratch space that it may use however it wants. It will generally just do
whatever is most efficient or otherwise easy to implement. Do not rely on
removed data to be erased for security purposes. Even if you drop a `Vec`, its
buffer may simply be reused by another allocation. Even if you zero a `Vec`'s memory
first, that might not actually happen because the optimizer does not consider
this a side-effect that must be preserved. There is one case which we will
not break, however: using `unsafe` code to write to the excess capacity,
and then increasing the length to match, is always valid.

Currently, `Vec` does not guarantee the order in which elements are dropped.
The order has changed in the past and may change again.

















#### Implementations

- <span id="vec-new"></span>`const fn new() -> Self`

  Constructs a new, empty `Vec<T>`.

  

  The vector will not allocate until elements are pushed onto it.

  

  # Examples

  

  ```rust

  #![allow(unused_mut)]

  let mut vec: Vec<i32> = Vec::new();

  ```

- <span id="vec-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Constructs a new, empty `Vec<T>` with at least the specified capacity.

  

  The vector will be able to hold at least `capacity` elements without

  reallocating. This method is allowed to allocate for more elements than

  `capacity`. If `capacity` is 0, the vector will not allocate.

  

  It is important to note that although the returned vector has the

  minimum *capacity* specified, the vector will have a zero *length*. For

  an explanation of the difference between length and capacity, see

  *[Capacity and reallocation]*.

  

  If it is important to know the exact allocated capacity of a `Vec`,

  always use the `capacity` method after construction.

  

  For `Vec<T>` where `T` is a zero-sized type, there will be no allocation

  and the capacity will always be `usize::MAX`.

  

  

  # Panics

  

  Panics if the new capacity exceeds `isize::MAX` bytes.

  

  # Examples

  

  ```rust

  let mut vec = Vec::with_capacity(10);

  

  // The vector contains no items, even though it has capacity for more

  assert_eq!(vec.len(), 0);

  assert!(vec.capacity() >= 10);

  

  // These are all done without reallocating...

  for i in 0..10 {

      vec.push(i);

  }

  assert_eq!(vec.len(), 10);

  assert!(vec.capacity() >= 10);

  

  // ...but this may make the vector reallocate

  vec.push(11);

  assert_eq!(vec.len(), 11);

  assert!(vec.capacity() >= 11);

  

  // A vector of a zero-sized type will always over-allocate, since no

  // allocation is necessary

  let vec_units = Vec::<()>::with_capacity(10);

  assert_eq!(vec_units.capacity(), usize::MAX);

  ```

- <span id="vec-from-raw-parts"></span>`unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self`

  Creates a `Vec<T>` directly from a pointer, a capacity, and a length.

  

  # Safety

  

  This is highly unsafe, due to the number of invariants that aren't

  checked:

  

  * `T` needs to have the same alignment as what `ptr` was allocated with.

    (`T` having a less strict alignment is not sufficient, the alignment really

    needs to be equal to satisfy the [`dealloc`](../alloc/index.md) requirement that memory must be

    allocated and deallocated with the same layout.)

  * The size of `T` times the `capacity` (ie. the allocated size in bytes) needs

    to be the same size as the pointer was allocated with. (Because similar to

    alignment, [`dealloc`](../alloc/index.md) must be called with the same layout `size`.)

  * `length` needs to be less than or equal to `capacity`.

  * The first `length` values must be properly initialized values of type `T`.

  * `capacity` needs to be the capacity that the pointer was allocated with.

  * The allocated size in bytes must be no larger than `isize::MAX`.

    See the safety documentation of [`pointer::offset`](https://doc.rust-lang.org/nightly/std/primitive.pointer.html#method.offset).

  

  These requirements are always upheld by any `ptr` that has been allocated

  via `Vec<T>`. Other allocation sources are allowed if the invariants are

  upheld.

  

  Violating these may cause problems like corrupting the allocator's

  internal data structures. For example it is normally **not** safe

  to build a `Vec<u8>` from a pointer to a C `char` array with length

  `size_t`, doing so is only safe if the array was initially allocated by

  a `Vec` or `String`.

  It's also not safe to build one from a `Vec<u16>` and its length, because

  the allocator cares about the alignment, and these two types have different

  alignments. The buffer was allocated with alignment 2 (for `u16`), but after

  turning it into a `Vec<u8>` it'll be deallocated with alignment 1. To avoid

  these issues, it is often preferable to do casting/transmuting using

  `slice::from_raw_parts` instead.

  

  The ownership of `ptr` is effectively transferred to the

  `Vec<T>` which may then deallocate, reallocate or change the

  contents of memory pointed to by the pointer at will. Ensure

  that nothing else uses the pointer after calling this

  function.

  

  

  # Examples

  

  ```rust

  use std::ptr;

  use std::mem;

  

  let v = vec![1, 2, 3];

  

  // Prevent running `v`'s destructor so we are in complete control

  // of the allocation.

  let mut v = mem::ManuallyDrop::new(v);

  

  // Pull out the various important pieces of information about `v`

  let p = v.as_mut_ptr();

  let len = v.len();

  let cap = v.capacity();

  

  unsafe {

      // Overwrite memory with 4, 5, 6

      for i in 0..len {

          ptr::write(p.add(i), 4 + i);

      }

  

      // Put everything back together into a Vec

      let rebuilt = Vec::from_raw_parts(p, len, cap);

      assert_eq!(rebuilt, [4, 5, 6]);

  }

  ```

  

  Using memory that was allocated elsewhere:

  

  ```rust

  #![feature(allocator_api)]

  

  use std::alloc::{AllocError, Allocator, Global, Layout};

  

  fn main() {

      let layout = Layout::array::<u32>(16).expect("overflow cannot happen");

  

      let vec = unsafe {

          let mem = match Global.allocate(layout) {

              Ok(mem) => mem.cast::<u32>().as_ptr(),

              Err(AllocError) => return,

          };

  

          mem.write(1_000_000);

  

          Vec::from_raw_parts_in(mem, 1, 16, Global)

      };

  

      assert_eq!(vec, &[1_000_000]);

      assert_eq!(vec.capacity(), 16);

  }

  ```

#### Trait Implementations

##### `impl<T> Any for Vec<T, A>`

- <span id="vec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T, A: Allocator> AsMut for Vec<T, A>`

- <span id="vec-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut Vec<T, A>` — [`Vec`](#vec)

##### `impl<T, A: Allocator> AsRef for Vec<T, A>`

- <span id="vec-asref-as-ref"></span>`fn as_ref(&self) -> &Vec<T, A>` — [`Vec`](#vec)

##### `impl<T> Borrow for Vec<T, A>`

- <span id="vec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Vec<T, A>`

- <span id="vec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone, A: Allocator + Clone> Clone for Vec<T, A>`

- <span id="vec-clone"></span>`fn clone(&self) -> Self`

- <span id="vec-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<T> CloneToUninit for Vec<T, A>`

- <span id="vec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Vec<T, A>`

- <span id="vec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Vec<T>`

- <span id="vec-default"></span>`fn default() -> Vec<T>` — [`Vec`](#vec)

  Creates an empty `Vec<T>`.

  

  The vector will not allocate until elements are pushed onto it.

##### `impl<T, A: Allocator> Deref for Vec<T, A>`

- <span id="vec-deref-type-target"></span>`type Target = [T]`

- <span id="vec-deref"></span>`fn deref(&self) -> &[T]`

##### `impl<T, A: Allocator> DerefMut for Vec<T, A>`

- <span id="vec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [T]`

##### `impl<T, A: Allocator> Drop for Vec<T, A>`

- <span id="vec-drop"></span>`fn drop(&mut self)`

##### `impl<T: Eq, A: Allocator> Eq for Vec<T, A>`

##### `impl<T, A: Allocator> Extend for Vec<T, A>`

- <span id="vec-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T> From for Vec<T, A>`

- <span id="vec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FromIterator for Vec<T>`

- <span id="vec-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T>` — [`Vec`](#vec)

##### `impl<T: Hash, A: Allocator> Hash for Vec<T, A>`

- <span id="vec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T, I: SliceIndex<[T]>, A: Allocator> Index for Vec<T, A>`

- <span id="vec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="vec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut for Vec<T, A>`

- <span id="vec-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<T, U> Into for Vec<T, A>`

- <span id="vec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, A: Allocator> IntoIterator for Vec<T, A>`

- <span id="vec-intoiterator-type-item"></span>`type Item = T`

- <span id="vec-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<T, A>`

- <span id="vec-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

  Creates a consuming iterator, that is, one that moves each value out of

  the vector (from start to end). The vector cannot be used after calling

  this.

  

  # Examples

  

  ```rust

  let v = vec!["a".to_string(), "b".to_string()];

  let mut v_iter = v.into_iter();

  

  let first_element: Option<String> = v_iter.next();

  

  assert_eq!(first_element, Some("a".to_string()));

  assert_eq!(v_iter.next(), Some("b".to_string()));

  assert_eq!(v_iter.next(), None);

  ```

##### `impl<T: Ord, A: Allocator> Ord for Vec<T, A>`

- <span id="vec-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T, U, A1: Allocator, A2: Allocator> PartialEq for super::Vec<T, A1>`

- <span id="supervec-partialeq-eq"></span>`fn eq(&self, other: &Vec<U, A2>) -> bool` — [`Vec`](#vec)

- <span id="supervec-partialeq-ne"></span>`fn ne(&self, other: &Vec<U, A2>) -> bool` — [`Vec`](#vec)

##### `impl<T: PartialOrd, A: Allocator> PartialOrd for Vec<T, A>`

- <span id="vec-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Receiver for Vec<T, A>`

- <span id="vec-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToOwned for Vec<T, A>`

- <span id="vec-toowned-type-owned"></span>`type Owned = T`

- <span id="vec-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="vec-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Vec<T, A>`

- <span id="vec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Vec<T, A>`

- <span id="vec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExtendElement<T>`

```rust
struct ExtendElement<T>(T);
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/mod.rs:2481`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/mod.rs#L2481)*

#### Trait Implementations

##### `impl<T> Any for ExtendElement<T>`

- <span id="extendelement-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExtendElement<T>`

- <span id="extendelement-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExtendElement<T>`

- <span id="extendelement-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone> ExtendWith for ExtendElement<T>`

- <span id="extendelement-extendwith-next"></span>`fn next(&mut self) -> T`

- <span id="extendelement-extendwith-last"></span>`fn last(self) -> T`

##### `impl<T> From for ExtendElement<T>`

- <span id="extendelement-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ExtendElement<T>`

- <span id="extendelement-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for ExtendElement<T>`

- <span id="extendelement-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extendelement-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ExtendElement<T>`

- <span id="extendelement-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extendelement-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ExtendWith<T>`

```rust
trait ExtendWith<T> { ... }
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/mod.rs:2476-2479`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/mod.rs#L2476-L2479)*

#### Required Methods

- `fn next(&mut self) -> T`

- `fn last(self) -> T`

#### Implementors

- [`ExtendElement`](#extendelement)

