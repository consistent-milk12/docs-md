# Crate `tinyvec`

`tinyvec` provides 100% safe vec-like data structures.

## Provided Types
With no features enabled, this crate provides the [`ArrayVec`](arrayvec/index.md) type, which
is an array-backed storage. You can push values into the array and pop them
out of the array and so on. If the array is made to overflow it will panic.

Similarly, there is also a [`SliceVec`](slicevec/index.md) type available, which is a vec-like
that's backed by a slice you provide. You can add and remove elements, but
if you overflow the slice it will panic.

With the `alloc` feature enabled, the crate also has a [`TinyVec`](tinyvec/index.md) type.
This is an enum type which is either an `Inline(ArrayVec)` or a `Heap(Vec)`.
If a `TinyVec` is `Inline` and would overflow it automatically transitions
itself into being `Heap` mode instead of a panic.

All of this is done with no `unsafe` code within the crate. Technically the
`Vec` type from the standard library uses `unsafe` internally, but *this
crate* introduces no new `unsafe` code into your project.

The limitation is that the element type of a vec from this crate must
support the [`Default`](#default) trait. This means that this crate isn't suitable for
all situations, but a very surprising number of types do support `Default`.

## Other Features
* `grab_spare_slice` lets you get access to the "inactive" portions of an
  ArrayVec.
* `serde` provides a `Serialize` and `Deserialize` implementation for
  [`TinyVec`](tinyvec/index.md) and [`ArrayVec`](arrayvec/index.md) types, provided the inner item also has an
  implementation.
* `borsh` provides a `BorshSerialize` and `BorshDeserialize` implementation
  for [`TinyVec`](tinyvec/index.md) and [`ArrayVec`](arrayvec/index.md) types, provided the inner item also has
  an implementation.

## API
The general goal of the crate is that, as much as possible, the vecs here
should be a "drop in" replacement for the standard library `Vec` type. We
strive to provide all of the `Vec` methods with the same names and
signatures. The exception is that the element type of some methods will have
a `Default` bound that's not part of the normal `Vec` type.

The vecs here also have a few additional methods that aren't on the `Vec`
type. In this case, the names tend to be fairly long so that they are
unlikely to clash with any future methods added to `Vec`.

## Macros

### `array_vec!`

Helper to make an `ArrayVec`.

You specify the backing array type, and optionally give all the elements you
want to initially place into the array.

```rust
use tinyvec::*;

// The backing array type can be specified in the macro call
let empty_av = array_vec!([u8; 16]);
let some_ints = array_vec!([i32; 4] => 1, 2, 3);

// Or left to inference
let empty_av: ArrayVec<[u8; 10]> = array_vec!();
let some_ints: ArrayVec<[u8; 10]> = array_vec!(5, 6, 7, 8);
```

### `tiny_vec!`

Helper to make a `TinyVec`.

You specify the backing array type, and optionally give all the elements you
want to initially place into the array.

```rust
use tinyvec::*;

// The backing array type can be specified in the macro call
let empty_tv = tiny_vec!([u8; 16]);
let some_ints = tiny_vec!([i32; 4] => 1, 2, 3);
let many_ints = tiny_vec!([i32; 4] => 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

// Or left to inference
let empty_tv: TinyVec<[u8; 16]> = tiny_vec!();
let some_ints: TinyVec<[i32; 4]> = tiny_vec!(1, 2, 3);
let many_ints: TinyVec<[i32; 4]> = tiny_vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
```

