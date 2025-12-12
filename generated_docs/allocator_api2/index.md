# Crate `allocator_api2`

allocator-api2 crate.

## Contents

- [Modules](#modules)
  - [`stable`](#stable)
  - [`alloc`](#alloc)
  - [`boxed`](#boxed)
  - [`raw_vec`](#raw-vec)
  - [`vec`](#vec)
  - [`macros`](#macros)
  - [`slice`](#slice)
  - [`unique`](#unique)
  - [`collections`](#collections)
- [Functions](#functions)
  - [`assume`](#assume)
  - [`addr`](#addr)
  - [`invalid_mut`](#invalid-mut)
- [Macros](#macros)
  - [`vec!`](#vec)
  - [`unsize_box!`](#unsize-box)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`stable`](#stable) | mod |  |
| [`alloc`](#alloc) | mod | Memory allocation APIs |
| [`boxed`](#boxed) | mod | The `Box<T>` type for heap allocation. |
| [`raw_vec`](#raw-vec) | mod |  |
| [`vec`](#vec) | mod | A contiguous growable array type with heap-allocated contents, written `Vec<T>`. |
| [`macros`](#macros) | mod |  |
| [`slice`](#slice) | mod |  |
| [`unique`](#unique) | mod |  |
| [`collections`](#collections) | mod |  |
| [`assume`](#assume) | fn |  |
| [`addr`](#addr) | fn |  |
| [`invalid_mut`](#invalid-mut) | fn |  |
| [`vec!`](#vec) | macro | Creates a [`Vec`] containing the arguments. |
| [`unsize_box!`](#unsize-box) | macro | Allows turning a [`Box<T: Sized, A>`][boxed::Box] into a [`Box<U: ?Sized, A>`][boxed::Box] where `T` can be unsizing-coerced into a `U`. |

## Modules

- [`stable`](stable/index.md)
- [`alloc`](alloc/index.md) — Memory allocation APIs
- [`boxed`](boxed/index.md) — The `Box<T>` type for heap allocation.
- [`raw_vec`](raw_vec/index.md)
- [`vec`](vec/index.md) — A contiguous growable array type with heap-allocated contents, written
- [`macros`](macros/index.md)
- [`slice`](slice/index.md)
- [`unique`](unique/index.md)
- [`collections`](collections/index.md)

## Functions

### `assume`

```rust
unsafe fn assume(v: bool)
```

*Defined in [`allocator-api2-0.2.21/src/stable/mod.rs:71-75`](../../.source_1765210505/allocator-api2-0.2.21/src/stable/mod.rs#L71-L75)*

### `addr`

```rust
fn addr<T>(x: *const T) -> usize
```

*Defined in [`allocator-api2-0.2.21/src/stable/mod.rs:91-96`](../../.source_1765210505/allocator-api2-0.2.21/src/stable/mod.rs#L91-L96)*

### `invalid_mut`

```rust
fn invalid_mut<T>(addr: usize) -> *mut T
```

*Defined in [`allocator-api2-0.2.21/src/stable/mod.rs:100-105`](../../.source_1765210505/allocator-api2-0.2.21/src/stable/mod.rs#L100-L105)*

## Macros

### `vec!`

*Defined in [`allocator-api2-0.2.21/src/stable/macros.rs:56-83`](../../.source_1765210505/allocator-api2-0.2.21/src/stable/macros.rs#L56-L83)*

Creates a [`Vec`](stable/vec/index.md) containing the arguments.

`vec!` allows `Vec`s to be defined with the same syntax as array expressions.
There are two forms of this macro:

- Create a [`Vec`](stable/vec/index.md) containing a given list of elements:

```rust
use allocator_api2::vec;
let v = vec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
```

```rust
use allocator_api2::{vec, alloc::Global};
let v = vec![in Global; 1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
```

- Create a [`Vec`](stable/vec/index.md) from a given element and size:

```rust
use allocator_api2::vec;
let v = vec![1; 3];
assert_eq!(v, [1, 1, 1]);
```

```rust
use allocator_api2::{vec, alloc::Global};
let v = vec![in Global; 1; 3];
assert_eq!(v, [1, 1, 1]);
```

Note that unlike array expressions this syntax supports all elements
which implement [`Clone`](../fs_err/index.md) and the number of elements doesn't have to be
a constant.

This will use `clone` to duplicate an expression, so one should be careful
using this with types having a nonstandard `Clone` implementation. For
example, `vec![Rc::new(1); 5]` will create a vector of five references
to the same boxed integer value, not five references pointing to independently
boxed integers.

Also, note that `vec![expr; 0]` is allowed, and produces an empty vector.
This will still evaluate `expr`, however, and immediately drop the resulting value, so
be mindful of side effects.

### `unsize_box!`

*Defined in [`allocator-api2-0.2.21/src/stable/mod.rs:47-60`](../../.source_1765210505/allocator-api2-0.2.21/src/stable/mod.rs#L47-L60)*

Allows turning a `Box<T: Sized, A>` into a `Box<U: ?Sized, A>` where `T` can be unsizing-coerced into a `U`.

This is the only way to create an `allocator_api2::boxed::Box` of an unsized type on stable.

With the standard library's `alloc::boxed::Box`, this is done automatically using the unstable unsize traits, but this crate's Box
can't take advantage of that machinery on stable. So, we need to use type inference and the fact that you *can*
still coerce the inner pointer of a box to get the compiler to help us unsize it using this macro.

## Example

```rust
use allocator_api2::unsize_box;
use allocator_api2::boxed::Box;
use core::any::Any;

let sized_box: Box<u64> = Box::new(0);
let unsized_box: Box<dyn Any> = unsize_box!(sized_box);
```

