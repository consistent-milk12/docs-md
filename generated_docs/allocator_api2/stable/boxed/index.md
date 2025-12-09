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

For non-zero-sized values, a [`Box`](#box) will use the [`Global`](../alloc/index.md) allocator for
its allocation. It is valid to convert both ways between a [`Box`](#box) and a
raw pointer allocated with the [`Global`](../alloc/index.md) allocator, given that the
[`Layout`](../alloc/index.md) used with the allocator is correct for the type. More precisely,
a `value: *mut T` that has been allocated with the [`Global`](../alloc/index.md) allocator
with `Layout::for_value(&*value)` may be converted into a box using
`Box::<T>::from_raw(value)`. Conversely, the memory backing a `value: *mut
T` obtained from `Box::<T>::into_raw` may be deallocated using the
[`Global`](../alloc/index.md) allocator with `Layout::for_value(&*value)`.

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

A pointer type for heap allocation.

See the [module-level documentation](../../std/boxed/index.html) for more.

#### Implementations

- <span id="box-downcast"></span>`fn downcast<T: Any>(self) -> Result<Box<T, A>, Self>` — [`Box`](#box)

- <span id="box-downcast-unchecked"></span>`unsafe fn downcast_unchecked<T: Any>(self) -> Box<T, A>` — [`Box`](#box)

#### Trait Implementations

##### `impl<T: ?Sized, A: Allocator> AsMut for Box<T, A>`

- <span id="box-as-mut"></span>`fn as_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized, A: Allocator> AsRef for Box<T, A>`

- <span id="box-as-ref"></span>`fn as_ref(&self) -> &T`

##### `impl<I: Iterator + ?Sized, A: Allocator> BoxIter for Box<I, A>`

- <span id="box-item"></span>`type Item = <I as Iterator>::Item`

- <span id="box-last"></span>`fn last(self) -> Option<<I as >::Item>`

##### `impl<T: Clone, A: Allocator + Clone> Clone for Box<T, A>`

- <span id="box-clone"></span>`fn clone(&self) -> Self`

- <span id="box-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T: fmt::Debug + ?Sized, A: Allocator> Debug for Box<T, A>`

- <span id="box-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator + Default> Default for Box<[T], A>`

- <span id="box-default"></span>`fn default() -> Self`

##### `impl<T: ?Sized, A: Allocator> Deref for Box<T, A>`

- <span id="box-target"></span>`type Target = T`

- <span id="box-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized, A: Allocator> DerefMut for Box<T, A>`

- <span id="box-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Display + ?Sized, A: Allocator> Display for Box<T, A>`

- <span id="box-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: DoubleEndedIterator + ?Sized, A: Allocator> DoubleEndedIterator for Box<I, A>`

- <span id="box-next-back"></span>`fn next_back(&mut self) -> Option<<I as >::Item>`

- <span id="box-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<I as >::Item>`

##### `impl<T: ?Sized, A: Allocator> Drop for Box<T, A>`

- <span id="box-drop"></span>`fn drop(&mut self)`

##### `impl<T: ?Sized + Eq, A: Allocator> Eq for Box<T, A>`

##### `impl<I: ExactSizeIterator + ?Sized, A: Allocator> ExactSizeIterator for Box<I, A>`

- <span id="box-len"></span>`fn len(&self) -> usize`

##### `impl<I> FromIterator for Box<[I]>`

- <span id="box-from-iter"></span>`fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self`

##### `impl<I: FusedIterator + ?Sized, A: Allocator> FusedIterator for Box<I, A>`

##### `impl<F: ?Sized + Future + Unpin, A> Future for Box<F, A>`

- <span id="box-output"></span>`type Output = <F as Future>::Output`

- <span id="box-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T: ?Sized + Hash, A: Allocator> Hash for Box<T, A>`

- <span id="box-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T: ?Sized + Hasher, A: Allocator> Hasher for Box<T, A>`

- <span id="box-finish"></span>`fn finish(&self) -> u64`

- <span id="box-write"></span>`fn write(&mut self, bytes: &[u8])`

- <span id="box-write-u8"></span>`fn write_u8(&mut self, i: u8)`

- <span id="box-write-u16"></span>`fn write_u16(&mut self, i: u16)`

- <span id="box-write-u32"></span>`fn write_u32(&mut self, i: u32)`

- <span id="box-write-u64"></span>`fn write_u64(&mut self, i: u64)`

- <span id="box-write-u128"></span>`fn write_u128(&mut self, i: u128)`

- <span id="box-write-usize"></span>`fn write_usize(&mut self, i: usize)`

- <span id="box-write-i8"></span>`fn write_i8(&mut self, i: i8)`

- <span id="box-write-i16"></span>`fn write_i16(&mut self, i: i16)`

- <span id="box-write-i32"></span>`fn write_i32(&mut self, i: i32)`

- <span id="box-write-i64"></span>`fn write_i64(&mut self, i: i64)`

- <span id="box-write-i128"></span>`fn write_i128(&mut self, i: i128)`

- <span id="box-write-isize"></span>`fn write_isize(&mut self, i: isize)`

##### `impl<F> IntoFuture for Box<T, A>`

- <span id="box-output"></span>`type Output = <F as Future>::Output`

- <span id="box-intofuture"></span>`type IntoFuture = F`

- <span id="box-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<I> IntoIterator for Box<T, A>`

- <span id="box-item"></span>`type Item = <I as Iterator>::Item`

- <span id="box-intoiter"></span>`type IntoIter = I`

- <span id="box-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator + ?Sized, A: Allocator> Iterator for Box<I, A>`

- <span id="box-item"></span>`type Item = <I as Iterator>::Item`

- <span id="box-next"></span>`fn next(&mut self) -> Option<<I as >::Item>`

- <span id="box-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="box-nth"></span>`fn nth(&mut self, n: usize) -> Option<<I as >::Item>`

- <span id="box-last"></span>`fn last(self) -> Option<<I as >::Item>`

##### `impl<T: ?Sized + Ord, A: Allocator> Ord for Box<T, A>`

- <span id="box-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T: ?Sized + PartialEq, A: Allocator> PartialEq for Box<T, A>`

- <span id="box-eq"></span>`fn eq(&self, other: &Self) -> bool`

- <span id="box-ne"></span>`fn ne(&self, other: &Self) -> bool`

##### `impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Box<T, A>`

- <span id="box-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

- <span id="box-lt"></span>`fn lt(&self, other: &Self) -> bool`

- <span id="box-le"></span>`fn le(&self, other: &Self) -> bool`

- <span id="box-ge"></span>`fn ge(&self, other: &Self) -> bool`

- <span id="box-gt"></span>`fn gt(&self, other: &Self) -> bool`

##### `impl<T: ?Sized, A: Allocator> Pointer for Box<T, A>`

- <span id="box-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<P, T> Receiver for Box<T, A>`

- <span id="box-target"></span>`type Target = T`

##### `impl<T, A> Send for Box<T, A>`

##### `impl<T, A> Sync for Box<T, A>`

##### `impl<T> ToString for Box<T, A>`

- <span id="box-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T: ?Sized, A> Unpin for Box<T, A>`

## Traits

### `BoxIter`

```rust
trait BoxIter { ... }
```

#### Associated Types

- `type Item`

#### Required Methods

- `fn last(self) -> Option<<Self as >::Item>`

#### Implementors

- [`Box`](#box)

