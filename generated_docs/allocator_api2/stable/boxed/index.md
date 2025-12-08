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









## Structs

### `Box<T: ?Sized, A: Allocator>`

```rust
struct Box<T: ?Sized, A: Allocator>(super::unique::Unique<T>, A);
```

A pointer type for heap allocation.

See the [module-level documentation](../../std/boxed/index.html) for more.

#### Implementations

- `fn downcast<T: Any>(self: Self) -> Result<Box<T, A>, Self>` — [`Box`](#box)

- `unsafe fn downcast_unchecked<T: Any>(self: Self) -> Box<T, A>` — [`Box`](#box)

#### Trait Implementations

##### `impl<T: ?Sized, A: Allocator> AsMut for Box<T, A>`

- `fn as_mut(self: &mut Self) -> &mut T`

##### `impl<T: ?Sized, A: Allocator> AsRef for Box<T, A>`

- `fn as_ref(self: &Self) -> &T`

##### `impl<I: Iterator + ?Sized, A: Allocator> BoxIter for Box<I, A>`

- `type Item = <I as Iterator>::Item`

- `fn last(self: Self) -> Option<<I as >::Item>`

##### `impl<T: Clone, A: Allocator + Clone> Clone for Box<[T], A>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, other: &Self)`

##### `impl<T: fmt::Debug + ?Sized, A: Allocator> Debug for Box<T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: Allocator + Default> Default for Box<str, A>`

- `fn default() -> Self`

##### `impl<T: ?Sized, A: Allocator> Deref for Box<T, A>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl<T: ?Sized, A: Allocator> DerefMut for Box<T, A>`

- `fn deref_mut(self: &mut Self) -> &mut T`

##### `impl<T: fmt::Display + ?Sized, A: Allocator> Display for Box<T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: DoubleEndedIterator + ?Sized, A: Allocator> DoubleEndedIterator for Box<I, A>`

- `fn next_back(self: &mut Self) -> Option<<I as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<I as >::Item>`

##### `impl<T: ?Sized, A: Allocator> Drop for Box<T, A>`

- `fn drop(self: &mut Self)`

##### `impl<T: ?Sized + Eq, A: Allocator> Eq for Box<T, A>`

##### `impl<I: ExactSizeIterator + ?Sized, A: Allocator> ExactSizeIterator for Box<I, A>`

- `fn len(self: &Self) -> usize`

##### `impl<I> FromIterator for Box<[I]>`

- `fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self`

##### `impl<I: FusedIterator + ?Sized, A: Allocator> FusedIterator for Box<I, A>`

##### `impl<F: ?Sized + Future + Unpin, A> Future for Box<F, A>`

- `type Output = <F as Future>::Output`

- `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl<T: ?Sized + Hash, A: Allocator> Hash for Box<T, A>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<T: ?Sized + Hasher, A: Allocator> Hasher for Box<T, A>`

- `fn finish(self: &Self) -> u64`

- `fn write(self: &mut Self, bytes: &[u8])`

- `fn write_u8(self: &mut Self, i: u8)`

- `fn write_u16(self: &mut Self, i: u16)`

- `fn write_u32(self: &mut Self, i: u32)`

- `fn write_u64(self: &mut Self, i: u64)`

- `fn write_u128(self: &mut Self, i: u128)`

- `fn write_usize(self: &mut Self, i: usize)`

- `fn write_i8(self: &mut Self, i: i8)`

- `fn write_i16(self: &mut Self, i: i16)`

- `fn write_i32(self: &mut Self, i: i32)`

- `fn write_i64(self: &mut Self, i: i64)`

- `fn write_i128(self: &mut Self, i: i128)`

- `fn write_isize(self: &mut Self, i: isize)`

##### `impl<F> IntoFuture for Box<T, A>`

- `type Output = <F as Future>::Output`

- `type IntoFuture = F`

- `fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture`

##### `impl<I> IntoIterator for Box<T, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator + ?Sized, A: Allocator> Iterator for Box<I, A>`

- `type Item = <I as Iterator>::Item`

- `fn next(self: &mut Self) -> Option<<I as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn nth(self: &mut Self, n: usize) -> Option<<I as >::Item>`

- `fn last(self: Self) -> Option<<I as >::Item>`

##### `impl<T: ?Sized + Ord, A: Allocator> Ord for Box<T, A>`

- `fn cmp(self: &Self, other: &Self) -> Ordering`

##### `impl<T: ?Sized + PartialEq, A: Allocator> PartialEq for Box<T, A>`

- `fn eq(self: &Self, other: &Self) -> bool`

- `fn ne(self: &Self, other: &Self) -> bool`

##### `impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Box<T, A>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

- `fn lt(self: &Self, other: &Self) -> bool`

- `fn le(self: &Self, other: &Self) -> bool`

- `fn ge(self: &Self, other: &Self) -> bool`

- `fn gt(self: &Self, other: &Self) -> bool`

##### `impl<T: ?Sized, A: Allocator> Pointer for Box<T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<P, T> Receiver for Box<T, A>`

- `type Target = T`

##### `impl<T, A> Send for Box<T, A>`

##### `impl<T, A> Sync for Box<T, A>`

##### `impl<T> ToString for Box<T, A>`

- `fn to_string(self: &Self) -> String`

##### `impl<T: ?Sized, A> Unpin for Box<T, A>`

## Traits

### `BoxIter`

```rust
trait BoxIter { ... }
```

#### Required Methods

- `type Item`

- `fn last(self: Self) -> Option<<Self as >::Item>`

