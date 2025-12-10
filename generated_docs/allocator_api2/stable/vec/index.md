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
  - [`into_iter`](#into_iter)
  - [`partial_eq`](#partial_eq)
  - [`set_len_on_drop`](#set_len_on_drop)
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
| [`into_iter`](#into_iter) | mod |  |
| [`partial_eq`](#partial_eq) | mod |  |
| [`set_len_on_drop`](#set_len_on_drop) | mod |  |
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

*Defined in [`allocator-api2-0.2.21/src/stable/vec/splice.rs:21-24`](../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/splice.rs#L21-L24)*

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

##### `impl<'a, I: fmt::Debug + Iterator + 'a, A: fmt::Debug + Allocator + 'a> Debug for Splice<'a, I, A>`

- <span id="splice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator, A: Allocator> DoubleEndedIterator for Splice<'_, I, A>`

- <span id="splice-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I: Iterator, A: Allocator> Drop for Splice<'_, I, A>`

- <span id="splice-drop"></span>`fn drop(&mut self)`

##### `impl<I: Iterator, A: Allocator> ExactSizeIterator for Splice<'_, I, A>`

##### `impl<I> IntoIterator for Splice<'a, I, A>`

- <span id="splice-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-type-intoiter"></span>`type IntoIter = I`

- <span id="splice-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator, A: Allocator> Iterator for Splice<'_, I, A>`

- <span id="splice-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="splice-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Drain<'a, T: 'a, A: Allocator + 'a>`

```rust
struct Drain<'a, T: 'a, A: Allocator + 'a> {
    tail_start: usize,
    tail_len: usize,
    iter: slice::Iter<'a, T>,
    vec: core::ptr::NonNull<super::Vec<T, A>>,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/drain.rs:22-30`](../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/drain.rs#L22-L30)*

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

- <span id="superdrain-move-tail"></span>`unsafe fn move_tail(&mut self, additional: usize)`

#### Trait Implementations

##### `impl<'a, T, A: Allocator> AsRef for Drain<'a, T, A>`

- <span id="drain-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Drain<'_, T, A>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A>`

- <span id="drain-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for Drain<'_, T, A>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<I> IntoIterator for Drain<'a, T, A>`

- <span id="drain-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- <span id="drain-type-item"></span>`type Item = T`

- <span id="drain-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A>`

##### `impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A>`

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

*Defined in [`allocator-api2-0.2.21/src/stable/vec/into_iter.rs:27-36`](../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/into_iter.rs#L27-L36)*

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

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

- <span id="intoiter-allocator"></span>`fn allocator(&self) -> &A`

- <span id="intoiter-as-raw-mut-slice"></span>`fn as_raw_mut_slice(&mut self) -> *mut [T]`

#### Trait Implementations

##### `impl<T, A: Allocator> AsRef for IntoIter<T, A>`

- <span id="intoiter-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug, A: Allocator> Debug for IntoIter<T, A>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A>`

- <span id="intoiter-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for IntoIter<T, A>`

- <span id="intoiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A>`

##### `impl<T, A: Allocator> FusedIterator for IntoIter<T, A>`

##### `impl<I> IntoIterator for IntoIter<T, A>`

- <span id="intoiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for IntoIter<T, A>`

- <span id="intoiter-type-item"></span>`type Item = T`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="intoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-count"></span>`fn count(self) -> usize`

##### `impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A>`

##### `impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A>`

### `Vec<T, A: Allocator>`

```rust
struct Vec<T, A: Allocator> {
    buf: super::raw_vec::RawVec<T, A>,
    len: usize,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/mod.rs:348-351`](../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/mod.rs#L348-L351)*

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
when you just want to provide read access. The same goes for [`String`](#string) and
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

- <span id="vec-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="vec-from-raw-parts"></span>`unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self`

#### Trait Implementations

##### `impl<T, A: Allocator> AsMut for Vec<T, A>`

- <span id="vec-as-mut"></span>`fn as_mut(&mut self) -> &mut Vec<T, A>` — [`Vec`](#vec)

##### `impl<T, A: Allocator> AsRef for Vec<T, A>`

- <span id="vec-as-ref"></span>`fn as_ref(&self) -> &Vec<T, A>` — [`Vec`](#vec)

##### `impl<T: Clone, A: Allocator + Clone> Clone for Vec<T, A>`

- <span id="vec-clone"></span>`fn clone(&self) -> Self`

- <span id="vec-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Vec<T, A>`

- <span id="vec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Vec<T>`

- <span id="vec-default"></span>`fn default() -> Vec<T>` — [`Vec`](#vec)

##### `impl<T, A: Allocator> Deref for Vec<T, A>`

- <span id="vec-type-target"></span>`type Target = [T]`

- <span id="vec-deref"></span>`fn deref(&self) -> &[T]`

##### `impl<T, A: Allocator> DerefMut for Vec<T, A>`

- <span id="vec-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [T]`

##### `impl<T, A: Allocator> Drop for Vec<T, A>`

- <span id="vec-drop"></span>`fn drop(&mut self)`

##### `impl<T: Eq, A: Allocator> Eq for Vec<T, A>`

##### `impl<T, A: Allocator> Extend for Vec<T, A>`

- <span id="vec-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T> FromIterator for Vec<T>`

- <span id="vec-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T>` — [`Vec`](#vec)

##### `impl<T: Hash, A: Allocator> Hash for Vec<T, A>`

- <span id="vec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T, I: SliceIndex<[T]>, A: Allocator> Index for Vec<T, A>`

- <span id="vec-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="vec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut for Vec<T, A>`

- <span id="vec-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<T, A: Allocator> IntoIterator for Vec<T, A>`

- <span id="vec-type-item"></span>`type Item = T`

- <span id="vec-type-intoiter"></span>`type IntoIter = IntoIter<T, A>`

- <span id="vec-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T: Ord, A: Allocator> Ord for Vec<T, A>`

- <span id="vec-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T, U, A1: Allocator, A2: Allocator> PartialEq for super::Vec<T, A1>`

- <span id="supervec-eq"></span>`fn eq(&self, other: &Vec<U, A2>) -> bool` — [`Vec`](#vec)

- <span id="supervec-ne"></span>`fn ne(&self, other: &Vec<U, A2>) -> bool` — [`Vec`](#vec)

##### `impl<T: PartialOrd, A: Allocator> PartialOrd for Vec<T, A>`

- <span id="vec-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<P, T> Receiver for Vec<T, A>`

- <span id="vec-type-target"></span>`type Target = T`

### `ExtendElement<T>`

```rust
struct ExtendElement<T>(T);
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/mod.rs:2481`](../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/mod.rs#L2481)*

#### Trait Implementations

##### `impl<T: Clone> ExtendWith for ExtendElement<T>`

- <span id="extendelement-next"></span>`fn next(&mut self) -> T`

- <span id="extendelement-last"></span>`fn last(self) -> T`

## Traits

### `ExtendWith<T>`

```rust
trait ExtendWith<T> { ... }
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/mod.rs:2476-2479`](../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/mod.rs#L2476-L2479)*

#### Required Methods

- `fn next(&mut self) -> T`

- `fn last(self) -> T`

#### Implementors

- [`ExtendElement`](#extendelement)

