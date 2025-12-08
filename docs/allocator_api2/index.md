# Crate `allocator_api2`


allocator-api2 crate.


## Modules

- [`alloc`](alloc/index.md) - Memory allocation APIs
- [`boxed`](boxed/index.md) - The `Box<T>` type for heap allocation.
- [`vec`](vec/index.md) - A contiguous growable array type with heap-allocated contents, written
- [`collections`](collections/index.md) - 

## Macros

### `vec!`

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
which implement `Clone` and the number of elements doesn't have to be
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

Allows turning a `Box<T: Sized, A>` into a `Box<U: ?Sized, A>` where `T` can be unsizing-coerced into a `U`.

This is the only way to create an `allocator_api2::boxed::Box` of an unsized type on stable.

With the standard library's `alloc::boxed::Box`, this is done automatically using the unstable unsize traits, but this crate's Box
can't take advantage of that machinery on stable. So, we need to use type inference and the fact that you *can*
still coerce the inner pointer of a box to get the compiler to help us unsize it using this macro.

# Example

```rust
use allocator_api2::unsize_box;
use allocator_api2::boxed::Box;
use core::any::Any;

let sized_box: Box<u64> = Box::new(0);
let unsized_box: Box<dyn Any> = unsize_box!(sized_box);
```

