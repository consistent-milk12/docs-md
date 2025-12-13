*[allocator_api2](../../index.md) / [stable](../index.md) / [boxed](index.md)*

---

# Module `boxed`

The `Box<T>` type for heap allocation.

[`Box<T>`](#box), casually referred to as a 'box', provides the simplest form of
heap allocation in Rust. Boxes provide ownership for this allocation, and
drop their contents when they go out of scope. Boxes also ensure that they
never allocate more than `isize::MAX` bytes.

# Examples

Move a value from the stack to the heap by creating a [`Box`](#box):

```rust
let val: u8 = 5;
let boxed: Box<u8> = Box::new(val);
```

Move a value from a [`Box`](#box) back to the stack by [dereferencing]:

```rust
let boxed: Box<u8> = Box::new(5);
let val: u8 = *boxed;
```

Creating a recursive data structure:

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
println!("{list:?}");
```

This will print `Cons(1, Cons(2, Nil))`.

Recursive structures must be boxed, because if the definition of `Cons`
looked like this:

```compile_fail,E0072
enum List<T> {
Cons(T, List<T>),
}
```

It wouldn't work. This is because the size of a `List` depends on how many
elements are in the list, and so we don't know how much memory to allocate
for a `Cons`. By introducing a [`Box<T>`](#box), which has a defined size, we know how
big `Cons` needs to be.

# Memory layout

For non-zero-sized values, a [`Box`](#box) will use the [`Global`](../alloc/global/index.md) allocator for
its allocation. It is valid to convert both ways between a [`Box`](#box) and a
raw pointer allocated with the [`Global`](../alloc/global/index.md) allocator, given that the
[`Layout`](../alloc/index.md) used with the allocator is correct for the type. More precisely,
a `value: *mut T` that has been allocated with the [`Global`](../alloc/global/index.md) allocator
with `Layout::for_value(&*value)` may be converted into a box using
`Box::<T>::from_raw(value)`. Conversely, the memory backing a `value: *mut
T` obtained from `Box::<T>::into_raw` may be deallocated using the
[`Global`](../alloc/global/index.md) allocator with `Layout::for_value(&*value)`.

For zero-sized values, the `Box` pointer still has to be [`valid`](../../../thiserror_impl/valid/index.md) for reads
and writes and sufficiently aligned. In particular, casting any aligned
non-zero integer literal to a raw pointer produces a valid pointer, but a
pointer pointing into previously allocated memory that since got freed is
not valid. The recommended way to build a Box to a ZST if `Box::new` cannot
be used is to use `ptr::NonNull::dangling`.

So long as `T: Sized`, a `Box<T>` is guaranteed to be represented
as a single pointer and is also ABI-compatible with C pointers
(i.e. the C type `T*`). This means that if you have extern "C"
Rust functions that will be called from C, you can define those
Rust functions using `Box<T>` types, and use `T*` as corresponding
type on the C side. As an example, consider this C header which
declares functions that create and destroy some kind of `Foo`
value:

```c
/* C header */

/* Returns ownership to the caller */
struct Foo* foo_new(void);

/* Takes ownership from the caller; no-op when invoked with null */
void foo_delete(struct Foo*);
```

These two functions might be implemented in Rust as follows. Here, the
`struct Foo*` type from C is translated to `Box<Foo>`, which captures
the ownership constraints. Note also that the nullable argument to
`foo_delete` is represented in Rust as `Option<Box<Foo>>`, since `Box<Foo>`
cannot be null.

```rust
#[repr(C)]
pub struct Foo;

#[no_mangle]
pub extern "C" fn foo_new() -> Box<Foo> {
    Box::new(Foo)
}

#[no_mangle]
pub extern "C" fn foo_delete(_: Option<Box<Foo>>) {}
```

Even though `Box<T>` has the same representation and C ABI as a C pointer,
this does not mean that you can convert an arbitrary `T*` into a `Box<T>`
and expect things to work. `Box<T>` values will always be fully aligned,
non-null pointers. Moreover, the destructor for `Box<T>` will attempt to
free the value with the global allocator. In general, the best practice
is to only use `Box<T>` for pointers that originated from the global
allocator.

**Important.** At least at present, you should avoid using
`Box<T>` types for functions that are defined in C but invoked
from Rust. In those cases, you should directly mirror the C types
as closely as possible. Using types like `Box<T>` where the C
definition is just using `T*` can lead to undefined behavior, as
described in [rust-lang/unsafe-code-guidelines#198][ucg#198].

# Considerations for unsafe code

**Warning: This section is not normative and is subject to change, possibly
being relaxed in the future! It is a simplified summary of the rules
currently implemented in the compiler.**

The aliasing rules for `Box<T>` are the same as for `&mut T`. `Box<T>`
asserts uniqueness over its content. Using raw pointers derived from a box
after that box has been mutated through, moved or borrowed as `&mut T`
is not allowed. For more guidance on working with box from unsafe code, see
[rust-lang/unsafe-code-guidelines#326][ucg#326].









## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Box`](#box) | struct | A pointer type for heap allocation. |
| [`BoxIter`](#boxiter) | trait |  |

## Structs

### `Box<T: ?Sized, A: Allocator>`

```rust
struct Box<T: ?Sized, A: Allocator>(super::unique::Unique<T>, A);
```

*Defined in [`allocator-api2-0.2.21/src/stable/boxed.rs:177`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/boxed.rs#L177)*

A pointer type for heap allocation.

See the [module-level documentation](../../std/boxed/index.html) for more.

#### Implementations

- <span id="box-new"></span>`fn new(x: T) -> Self`

  Allocates memory on the heap and then places `x` into it.

  

  This doesn't actually allocate if `T` is zero-sized.

  

  # Examples

  

  ```rust

  let five = Box::new(5);

  ```

- <span id="box-new-uninit"></span>`fn new_uninit() -> Box<mem::MaybeUninit<T>>` — [`Box`](#box)

  Constructs a new box with uninitialized contents.

  

  # Examples

  

  ```rust

  #![feature(new_uninit)]

  

  let mut five = Box::<u32>::new_uninit();

  

  let five = unsafe {

      // Deferred initialization:

      five.as_mut_ptr().write(5);

  

      five.assume_init()

  };

  

  assert_eq!(*five, 5)

  ```

- <span id="box-new-zeroed"></span>`fn new_zeroed() -> Box<mem::MaybeUninit<T>>` — [`Box`](#box)

  Constructs a new `Box` with uninitialized contents, with the memory

  being filled with `0` bytes.

  

  See [`MaybeUninit::zeroed`][zeroed] for examples of correct and incorrect usage

  of this method.

  

  # Examples

  

  ```rust

  #![feature(new_uninit)]

  

  let zero = Box::<u32>::new_zeroed();

  let zero = unsafe { zero.assume_init() };

  

  assert_eq!(*zero, 0)

  ```

- <span id="box-pin"></span>`fn pin(x: T) -> Pin<Box<T>>` — [`Box`](#box)

  Constructs a new `Pin<Box<T>>`. If `T` does not implement `Unpin`, then

  `x` will be pinned in memory and unable to be moved.

  

  Constructing and pinning of the `Box` can also be done in two steps: `Box::pin(x)`

  does the same as <code>[Box::into_pin]\([Box::new]\(x))</code>. Consider using

  [`into_pin`](Box::into_pin) if you already have a `Box<T>`, or if you want to

  construct a (pinned) `Box` in a different way than with `Box::new`.

- <span id="box-try-new"></span>`fn try_new(x: T) -> Result<Self, AllocError>` — [`AllocError`](../alloc/index.md#allocerror)

  Allocates memory on the heap then places `x` into it,

  returning an error if the allocation fails

  

  This doesn't actually allocate if `T` is zero-sized.

  

  # Examples

  

  ```rust

  #![feature(allocator_api)]

  

  let five = Box::try_new(5)?;

  Ok::<(), std::alloc::AllocError>(())

  ```

- <span id="box-try-new-uninit"></span>`fn try_new_uninit() -> Result<Box<mem::MaybeUninit<T>>, AllocError>` — [`Box`](#box), [`AllocError`](../alloc/index.md#allocerror)

  Constructs a new box with uninitialized contents on the heap,

  returning an error if the allocation fails

  

  # Examples

  

  ```rust

  #![feature(allocator_api, new_uninit)]

  

  let mut five = Box::<u32>::try_new_uninit()?;

  

  let five = unsafe {

      // Deferred initialization:

      five.as_mut_ptr().write(5);

  

      five.assume_init()

  };

  

  assert_eq!(*five, 5);

  Ok::<(), std::alloc::AllocError>(())

  ```

- <span id="box-try-new-zeroed"></span>`fn try_new_zeroed() -> Result<Box<mem::MaybeUninit<T>>, AllocError>` — [`Box`](#box), [`AllocError`](../alloc/index.md#allocerror)

  Constructs a new `Box` with uninitialized contents, with the memory

  being filled with `0` bytes on the heap

  

  See [`MaybeUninit::zeroed`][zeroed] for examples of correct and incorrect usage

  of this method.

  

  # Examples

  

  ```rust

  #![feature(allocator_api, new_uninit)]

  

  let zero = Box::<u32>::try_new_zeroed()?;

  let zero = unsafe { zero.assume_init() };

  

  assert_eq!(*zero, 0);

  Ok::<(), std::alloc::AllocError>(())

  ```

#### Trait Implementations

##### `impl<T> Any for Box<T, A>`

- <span id="box-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized, A: Allocator> AsMut for Box<T, A>`

- <span id="box-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized, A: Allocator> AsRef for Box<T, A>`

- <span id="box-asref-as-ref"></span>`fn as_ref(&self) -> &T`

##### `impl<T> Borrow for Box<T, A>`

- <span id="box-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Box<T, A>`

- <span id="box-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: Iterator + ?Sized, A: Allocator> BoxIter for Box<I, A>`

- <span id="box-boxiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="box-boxiter-last"></span>`fn last(self) -> Option<<I as >::Item>`

##### `impl<T: Clone, A: Allocator + Clone> Clone for Box<T, A>`

- <span id="box-clone"></span>`fn clone(&self) -> Self`

  Returns a new box with a `clone()` of this box's contents.

  

  # Examples

  

  ```rust

  let x = Box::new(5);

  let y = x.clone();

  

  // The value is the same

  assert_eq!(x, y);

  

  // But they are unique objects

  assert_ne!(&*x as *const i32, &*y as *const i32);

  ```

- <span id="box-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

  Copies `source`'s contents into `self` without creating a new allocation.

  

  # Examples

  

  ```rust

  let x = Box::new(5);

  let mut y = Box::new(10);

  let yp: *const i32 = &*y;

  

  y.clone_from(&x);

  

  // The value is the same

  assert_eq!(x, y);

  

  // And no allocation occurred

  assert_eq!(yp, &*y);

  ```

##### `impl<T> CloneToUninit for Box<T, A>`

- <span id="box-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug + ?Sized, A: Allocator> Debug for Box<T, A>`

- <span id="box-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for Box<T>`

- <span id="box-default"></span>`fn default() -> Self`

  Creates a `Box<T>`, with the `Default` value for T.

##### `impl<T: ?Sized, A: Allocator> Deref for Box<T, A>`

- <span id="box-deref-type-target"></span>`type Target = T`

- <span id="box-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized, A: Allocator> DerefMut for Box<T, A>`

- <span id="box-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Display + ?Sized, A: Allocator> Display for Box<T, A>`

- <span id="box-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: DoubleEndedIterator + ?Sized, A: Allocator> DoubleEndedIterator for Box<I, A>`

- <span id="box-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<I as >::Item>`

- <span id="box-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<I as >::Item>`

##### `impl<T: ?Sized, A: Allocator> Drop for Box<T, A>`

- <span id="box-drop"></span>`fn drop(&mut self)`

##### `impl<T: ?Sized + Eq, A: Allocator> Eq for Box<T, A>`

##### `impl<I: ExactSizeIterator + ?Sized, A: Allocator> ExactSizeIterator for Box<I, A>`

- <span id="box-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<A: Allocator> Extend for alloc_crate::string::String`

- <span id="alloc-cratestringstring-extend"></span>`fn extend<I: IntoIterator<Item = Box<str, A>>>(&mut self, iter: I)`

##### `impl<T> From for Box<T, A>`

- <span id="box-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FromIterator for Box<[I]>`

- <span id="box-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self`

##### `impl<I: FusedIterator + ?Sized, A: Allocator> FusedIterator for Box<I, A>`

##### `impl<F: ?Sized + Future + Unpin, A> Future for Box<F, A>`

- <span id="box-future-type-output"></span>`type Output = <F as Future>::Output`

- <span id="box-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T: ?Sized + Hash, A: Allocator> Hash for Box<T, A>`

- <span id="box-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T: ?Sized + Hasher, A: Allocator> Hasher for Box<T, A>`

- <span id="box-hasher-finish"></span>`fn finish(&self) -> u64`

- <span id="box-hasher-write"></span>`fn write(&mut self, bytes: &[u8])`

- <span id="box-hasher-write-u8"></span>`fn write_u8(&mut self, i: u8)`

- <span id="box-hasher-write-u16"></span>`fn write_u16(&mut self, i: u16)`

- <span id="box-hasher-write-u32"></span>`fn write_u32(&mut self, i: u32)`

- <span id="box-hasher-write-u64"></span>`fn write_u64(&mut self, i: u64)`

- <span id="box-hasher-write-u128"></span>`fn write_u128(&mut self, i: u128)`

- <span id="box-hasher-write-usize"></span>`fn write_usize(&mut self, i: usize)`

- <span id="box-hasher-write-i8"></span>`fn write_i8(&mut self, i: i8)`

- <span id="box-hasher-write-i16"></span>`fn write_i16(&mut self, i: i16)`

- <span id="box-hasher-write-i32"></span>`fn write_i32(&mut self, i: i32)`

- <span id="box-hasher-write-i64"></span>`fn write_i64(&mut self, i: i64)`

- <span id="box-hasher-write-i128"></span>`fn write_i128(&mut self, i: i128)`

- <span id="box-hasher-write-isize"></span>`fn write_isize(&mut self, i: isize)`

##### `impl<T, U> Into for Box<T, A>`

- <span id="box-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoFuture for Box<T, A>`

- <span id="box-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="box-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="box-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl IntoIterator for Box<T, A>`

- <span id="box-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="box-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="box-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator + ?Sized, A: Allocator> Iterator for Box<I, A>`

- <span id="box-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="box-iterator-next"></span>`fn next(&mut self) -> Option<<I as >::Item>`

- <span id="box-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="box-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<I as >::Item>`

- <span id="box-iterator-last"></span>`fn last(self) -> Option<<I as >::Item>`

##### `impl<T: ?Sized + Ord, A: Allocator> Ord for Box<T, A>`

- <span id="box-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T: ?Sized + PartialEq, A: Allocator> PartialEq for Box<T, A>`

- <span id="box-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

- <span id="box-partialeq-ne"></span>`fn ne(&self, other: &Self) -> bool`

##### `impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Box<T, A>`

- <span id="box-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

- <span id="box-partialord-lt"></span>`fn lt(&self, other: &Self) -> bool`

- <span id="box-partialord-le"></span>`fn le(&self, other: &Self) -> bool`

- <span id="box-partialord-ge"></span>`fn ge(&self, other: &Self) -> bool`

- <span id="box-partialord-gt"></span>`fn gt(&self, other: &Self) -> bool`

##### `impl<T: ?Sized, A: Allocator> Pointer for Box<T, A>`

- <span id="box-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Receiver for Box<T, A>`

- <span id="box-receiver-type-target"></span>`type Target = T`

##### `impl<T, A> Send for Box<T, A>`

##### `impl<T, A> Sync for Box<T, A>`

##### `impl<T> ToOwned for Box<T, A>`

- <span id="box-toowned-type-owned"></span>`type Owned = T`

- <span id="box-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="box-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T> ToString for Box<T, A>`

- <span id="box-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for Box<T, A>`

- <span id="box-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="box-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Box<T, A>`

- <span id="box-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="box-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized, A> Unpin for Box<T, A>`

## Traits

### `BoxIter`

```rust
trait BoxIter { ... }
```

*Defined in [`allocator-api2-0.2.21/src/stable/boxed.rs:1903-1906`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/boxed.rs#L1903-L1906)*

#### Associated Types

- `type Item`

#### Required Methods

- `fn last(self) -> Option<<Self as >::Item>`

#### Implementors

- [`Box`](#box)

